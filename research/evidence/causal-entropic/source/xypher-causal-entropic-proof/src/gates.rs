//! Gates G01--G12 of boundary section 7. Within each gate the checks run in
//! the listed order, and each check runs over all configurations (and
//! states) in canonical order before the next check begins.

use std::cell::OnceCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::contact::ContactAnalysis;
use crate::model::{
    lumped_hazard, Case, Generator, HazardInputs, Kind, Model, Outcome, HOME, KAPPA, LAMBDA,
    PRIMARY,
};
use crate::ratio::{integer_power, Ratio};
use crate::CheckId;

#[derive(Clone, Debug)]
pub(crate) struct Failure {
    pub(crate) check: CheckId,
    pub(crate) configuration: Option<usize>,
    pub(crate) detail: String,
}

fn fail(check: CheckId, configuration: Option<usize>, detail: impl Into<String>) -> Failure {
    Failure {
        check,
        configuration,
        detail: detail.into(),
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Status {
    Pass(String),
    NotApplicable(String),
}

pub(crate) type GateResult = Result<Status, Failure>;

pub(crate) struct Context<'a> {
    pub(crate) model: &'a Model,
    pub(crate) contact: &'a OnceCell<ContactAnalysis>,
}

impl Context<'_> {
    fn contact(&self) -> &ContactAnalysis {
        self.contact
            .get_or_init(|| ContactAnalysis::compute(self.model.mutation))
    }
}

fn pass() -> GateResult {
    Ok(Status::Pass(String::new()))
}

/// A generator level: explicit complete states (N = 3) or system states.
struct Level<'a> {
    name: &'static str,
    generator: &'a Generator,
    system_of: &'a [usize],
    label_of: Option<&'a [usize]>,
}

fn levels(model: &Model) -> Vec<Level<'_>> {
    let mut levels = Vec::new();
    if let Some(micro) = &model.micro {
        levels.push(Level {
            name: "complete-state",
            generator: &micro.generator,
            system_of: &micro.system_of,
            label_of: Some(&micro.label_of),
        });
    }
    levels.push(Level {
        name: "system-state",
        generator: &model.system,
        system_of: &model.identity,
        label_of: None,
    });
    levels
}

/// `exp[THAIM_+(G -> G')/alpha] = max(1, N_tau(G')/N_tau(G))`.
fn exp_thaim_plus(model: &Model, from: usize, to: usize) -> Ratio {
    Ratio::ONE.max(Ratio::new(model.matrix_count[to], model.matrix_count[from]))
}

/// `exp[Xi(G -> G')/alpha]`, multiplicatively from the THAIM pair and `U`.
fn exp_xi(model: &Model, from: usize, to: usize) -> Ratio {
    let delta_u = LAMBDA * (model.size(to) as i64 - model.size(from) as i64);
    let energy_factor = Ratio::power(model.case.b, -delta_u);
    let forward = exp_thaim_plus(model, from, to);
    if model.mutation == crate::model::Mutation::UnpairedReceipt {
        forward * energy_factor
    } else {
        forward / exp_thaim_plus(model, to, from) * energy_factor
    }
}

/// Unnormalised configuration marginal of the verified law `b^(-U(z))`.
fn configuration_mass(model: &Model) -> Vec<Ratio> {
    (0..model.config_count)
        .map(|config| {
            let weight = model.boltzmann_weight(config);
            weight * Ratio::integer(model.states_of(config).len() as i128)
        })
        .collect()
}

/// Unnormalised equilibrium toggle fluxes `sum pi(z) k(z -> z')` between
/// configurations, read from the constructed system generator.
fn configuration_fluxes(model: &Model) -> HashMap<(usize, usize), Ratio> {
    let mut fluxes: HashMap<(usize, usize), Ratio> = HashMap::new();
    for (state, row) in model.system.rows.iter().enumerate() {
        let from = model.states[state].config;
        let weight = model.boltzmann_weight(from);
        for entry in row {
            let to = model.states[entry.to].config;
            if to != from {
                let flux = fluxes.entry((from, to)).or_insert(Ratio::ZERO);
                *flux = *flux + weight * entry.hazard;
            }
        }
    }
    fluxes
}

fn flux(fluxes: &HashMap<(usize, usize), Ratio>, from: usize, to: usize) -> Ratio {
    fluxes.get(&(from, to)).copied().unwrap_or(Ratio::ZERO)
}

/// `b^energy` by repeated multiplication, written independently of
/// `lumped_hazard`.
fn reservoir_multiplicity(base: i128, energy: usize) -> Ratio {
    Ratio::integer((0..energy).fold(1_i128, |product, _| product * base))
}

/// System-state hazard stated in boundary section 4.1, written without
/// `lumped_hazard`: `b^(E_R(z'))` for a toggle into `to_config`, 1 otherwise.
fn expected_system_hazard(m: &Model, kind: Kind, to_config: usize) -> Ratio {
    match kind {
        Kind::Build | Kind::Dismantle => {
            KAPPA
                * reservoir_multiplicity(
                    m.case.b,
                    m.bridge_count() - to_config.count_ones() as usize,
                )
        }
        Kind::Step | Kind::Replan => KAPPA,
    }
}

/// Total exit rate of a system state counted directly from the local rules of
/// boundary section 3.4, without consulting the constructed moves. The same
/// total holds for every complete state above it: a toggle reaches every
/// destination reservoir label at unit hazard.
fn expected_exit_rate(m: &Model, state: usize) -> Ratio {
    let config = m.states[state].config;
    let cursor = m.states[state].cursor;
    let plan = m.plan_of(state);
    let steps = usize::from(cursor < m.case.tau) + usize::from(cursor > 0);
    let replans = if cursor == 0 {
        (m.matrix_count[config] - 1) as usize
    } else {
        0
    };
    let island = plan[cursor];
    let walks = |u: usize, w: usize| {
        plan.windows(2)
            .any(|pair| (pair[0] == u && pair[1] == w) || (pair[0] == w && pair[1] == u))
    };
    let mut total = KAPPA * Ratio::integer((steps + replans) as i128);
    for (bridge, &(u, w)) in m.bridges.iter().enumerate() {
        if island != u && island != w {
            continue;
        }
        let target = config ^ (1 << bridge);
        if config & (1 << bridge) == 0 {
            total = total + expected_system_hazard(m, Kind::Build, target);
        } else if !walks(u, w) {
            total = total + expected_system_hazard(m, Kind::Dismantle, target);
        }
    }
    total
}

// ---------------------------------------------------------------- G01

pub(crate) fn gate_01(cx: &Context) -> GateResult {
    let m = cx.model;
    for config in 0..m.config_count {
        if m.matrix_count[config] < 1 {
            return Err(fail(
                CheckId::G01_COUNT_POSITIVE,
                Some(config),
                format!("N_tau(G)={} < 1", m.matrix_count[config]),
            ));
        }
    }
    for config in 0..m.config_count {
        let depth_first = m.plans[config].len() as i128;
        if depth_first != m.matrix_count[config] {
            return Err(fail(
                CheckId::G01_DFS_EQUALS_MATRIX,
                Some(config),
                format!(
                    "depth-first count {depth_first} != e_h^T L^tau 1 = {}",
                    m.matrix_count[config]
                ),
            ));
        }
    }
    for config in 0..m.config_count {
        let resolution = &m.resolutions[config];
        let multiplicity = resolution.reported_multiplicity;
        for state in m.states_of(config) {
            if m.states[state].cursor != 0 {
                continue;
            }
            let uniform = multiplicity > 0
                && resolution.outcomes.len() == multiplicity
                && resolution.probabilities.len() == multiplicity
                && resolution
                    .probabilities
                    .iter()
                    .all(|&p| p == Ratio::new(1, multiplicity as i128));
            if !uniform {
                let law: Vec<String> = resolution
                    .probabilities
                    .iter()
                    .map(Ratio::to_string)
                    .collect();
                return Err(fail(
                    CheckId::G01_REPLAN_UNIFORM,
                    Some(config),
                    format!(
                        "REPLAN at {} has outcome law [{}] with reported multiplicity {multiplicity}",
                        m.describe_state(state),
                        law.join(",")
                    ),
                ));
            }
            let hazards: Vec<Ratio> = m.system.rows[state]
                .iter()
                .filter(|entry| entry.kind == Kind::Replan)
                .map(|entry| entry.hazard)
                .collect();
            if hazards.iter().any(|&hazard| hazard != hazards[0]) {
                return Err(fail(
                    CheckId::G01_REPLAN_UNIFORM,
                    Some(config),
                    format!(
                        "REPLAN hazards at {} are not equal",
                        m.describe_state(state)
                    ),
                ));
            }
        }
    }
    for config in 0..m.config_count {
        let resolution = &m.resolutions[config];
        let mut alphabet = BTreeSet::new();
        let mut non_sequence = None;
        for outcome in &resolution.outcomes {
            match outcome {
                Outcome::Plan(sequence) => {
                    alphabet.insert(sequence.clone());
                }
                Outcome::Endpoint(island) => non_sequence = Some(*island),
            }
        }
        if non_sequence.is_some()
            || alphabet.len() != resolution.outcomes.len()
            || alphabet != m.reference_plans[config]
        {
            return Err(fail(
                CheckId::G01_REPLAN_ALPHABET,
                Some(config),
                format!(
                    "REPLAN alphabet {:?}{} != Pi_tau(G) {:?}",
                    alphabet,
                    non_sequence
                        .map(|island| format!(" with island outcome {island}"))
                        .unwrap_or_default(),
                    m.reference_plans[config]
                ),
            ));
        }
    }
    let tau = m.case.tau;
    for config in 0..m.config_count {
        let states = &m.states[m.states_of(config)];
        let count = states.len() as i128;
        let expected = (tau as i128 + 1) * m.matrix_count[config];
        let reference = &m.reference_plans[config];
        let uncovered = reference
            .iter()
            .find(|plan| match m.plan_index[config].get(*plan) {
                Some(&index) => !(0..=tau).all(|cursor| {
                    states
                        .iter()
                        .any(|state| state.plan == index && state.cursor == cursor)
                }),
                None => true,
            });
        if count != expected || (reference.len() as i128) * (tau as i128 + 1) != expected {
            return Err(fail(
                CheckId::G01_SYSTEM_STATE_COUNT,
                Some(config),
                format!(
                    "{count} system states, {} brute-force plans x {} cursors, (tau+1) N_tau = {expected}",
                    reference.len(),
                    tau + 1
                ),
            ));
        }
        if let Some(plan) = uncovered {
            return Err(fail(
                CheckId::G01_SYSTEM_STATE_COUNT,
                Some(config),
                format!("brute-force plan {plan:?} lacks a system state at some cursor"),
            ));
        }
    }
    pass()
}

// ---------------------------------------------------------------- G02

pub(crate) fn gate_02(cx: &Context) -> GateResult {
    let m = cx.model;
    for config in 0..m.config_count {
        for plan in &m.plans[config] {
            if !m.plan_valid(config, plan) {
                return Err(fail(
                    CheckId::G02_PLAN_VALIDITY,
                    Some(config),
                    format!("enumerated plan {plan:?} is not valid"),
                ));
            }
        }
    }
    for state in 0..m.states.len() {
        let config = m.states[state].config;
        for local in &m.moves[state] {
            let register = m.destination_register(state, local);
            let consistent = local.to.is_some_and(|to| {
                let target = m.states[to];
                target.config == local.to_config
                    && target.cursor == local.to_cursor
                    && m.plan_of(to) == register
            });
            if !m.plan_valid(local.to_config, register) || !consistent {
                return Err(fail(
                    CheckId::G02_PLAN_VALIDITY,
                    Some(config),
                    format!(
                        "{} from {} writes plan {:?} invalid in destination {}",
                        local.kind.code(),
                        m.describe_state(state),
                        register,
                        m.config_label(local.to_config)
                    ),
                ));
            }
        }
    }
    for state in 0..m.states.len() {
        let config = m.states[state].config;
        let cursor = m.states[state].cursor;
        for local in m.moves[state].iter().filter(|l| l.kind == Kind::Step) {
            let plan = m.plan_of(state);
            let target = m.destination_register(state, local);
            let adjacent_cursor = local.to_cursor.abs_diff(cursor) == 1;
            let local_move = adjacent_cursor && {
                let (from, to) = (plan[cursor], target[local.to_cursor]);
                from == to || m.bridge_built(config, from, to)
            };
            if local.to_config != config || !local_move {
                return Err(fail(
                    CheckId::G02_STEP_LOCALITY,
                    Some(config),
                    format!(
                        "STEP from {} is not along a bridge or a stay",
                        m.describe_state(state)
                    ),
                ));
            }
        }
    }
    for state in 0..m.states.len() {
        let config = m.states[state].config;
        let island = m.position(state);
        for local in m.moves[state].iter().filter(|l| l.kind.is_toggle()) {
            let incident = local.bridge.is_some_and(|bridge| {
                let (u, w) = m.bridges[bridge];
                let built = m.has_bridge(config, bridge);
                (island == u || island == w)
                    && local.to_config == config ^ (1 << bridge)
                    && local.to_cursor == m.states[state].cursor
                    && built == (local.kind == Kind::Dismantle)
            });
            if !incident {
                return Err(fail(
                    CheckId::G02_TOGGLE_INCIDENCE,
                    Some(config),
                    format!(
                        "{} from {} is not incident to island {island}",
                        local.kind.code(),
                        m.describe_state(state)
                    ),
                ));
            }
        }
    }
    for state in 0..m.states.len() {
        let config = m.states[state].config;
        for local in m.moves[state].iter().filter(|l| l.kind == Kind::Dismantle) {
            let bridge = local.bridge.expect("checked by G02 check 3");
            if m.uses_bridge(m.plan_of(state), bridge) {
                return Err(fail(
                    CheckId::G02_HELD_PLAN_PROTECTION,
                    Some(config),
                    format!(
                        "DISMANTLE of {:?} at {} removes a bridge of the held plan",
                        m.bridges[bridge],
                        m.describe_state(state)
                    ),
                ));
            }
        }
    }
    if m.case != PRIMARY {
        return pass();
    }
    let toggle_set = |state: usize| -> BTreeSet<(Kind, usize)> {
        m.moves[state]
            .iter()
            .filter(|local| local.kind.is_toggle())
            .filter_map(|local| local.bridge.map(|bridge| (local.kind, bridge)))
            .collect()
    };
    for config in 0..m.config_count {
        for cursor in 0..=m.case.tau {
            let members: Vec<usize> = m
                .states_of(config)
                .filter(|&state| m.states[state].cursor == cursor)
                .collect();
            for (index, &first) in members.iter().enumerate() {
                for &second in &members[index + 1..] {
                    if toggle_set(first) != toggle_set(second) {
                        return Ok(Status::Pass(format!(
                            "plan read at {} vs {}",
                            m.describe_state(first),
                            m.describe_state(second)
                        )));
                    }
                }
            }
        }
    }
    Err(fail(
        CheckId::G02_PLAN_CONTENTS_READ,
        None,
        "no configuration has two equal-cursor system states with different BUILD/DISMANTLE sets",
    ))
}

// ---------------------------------------------------------------- G03

pub(crate) fn gate_03(cx: &Context) -> GateResult {
    let m = cx.model;
    let levels = levels(m);
    for level in &levels {
        for (from, row) in level.generator.rows.iter().enumerate() {
            for entry in row {
                if !level.generator.hazard(entry.to, from).is_positive() {
                    return Err(fail(
                        CheckId::G03_RECIPROCAL_SUPPORT,
                        Some(m.states[level.system_of[from]].config),
                        format!(
                            "{} {} channel {} -> {} has no positive reverse",
                            level.name,
                            entry.kind.code(),
                            from,
                            entry.to
                        ),
                    ));
                }
            }
        }
    }
    let (level, micro) = match &m.micro {
        Some(_) => (&levels[0], true),
        None => (&levels[levels.len() - 1], false),
    };
    for (from, row) in level.generator.rows.iter().enumerate() {
        for entry in row {
            let forward = entry.hazard;
            let reverse = level.generator.hazard(entry.to, from);
            let (expected_forward, expected_reverse) = if micro {
                (KAPPA, KAPPA)
            } else {
                let from_config = m.states[from].config;
                let to_config = m.states[entry.to].config;
                (
                    expected_system_hazard(m, entry.kind, to_config),
                    expected_system_hazard(m, entry.kind.reverse(), from_config),
                )
            };
            if forward != expected_forward || reverse != expected_reverse {
                return Err(fail(
                    CheckId::G03_EQUAL_REVERSE_HAZARDS,
                    Some(m.states[level.system_of[from]].config),
                    format!(
                        "{} {} channel {from} <-> {}: hazards {forward} / {reverse}, expected {expected_forward} / {expected_reverse}",
                        level.name,
                        entry.kind.code(),
                        entry.to
                    ),
                ));
            }
        }
    }
    for level in &levels {
        for from in 0..level.generator.len() {
            let row = &level.generator.rows[from];
            let nonnegative = row
                .iter()
                .all(|entry| entry.hazard.is_positive() && entry.to != from);
            let escape = level.generator.escape(from);
            let counted = expected_exit_rate(m, level.system_of[from]);
            let closure = level.generator.diagonal(from) + escape;
            if !nonnegative || escape != counted || !closure.is_zero() {
                return Err(fail(
                    CheckId::G03_ROW_CLOSURE,
                    Some(m.states[level.system_of[from]].config),
                    format!(
                        "{} row {from}: exit rate {escape}, counted from local rules {counted}, closure {closure}",
                        level.name
                    ),
                ));
            }
        }
    }
    Ok(Status::Pass(
        "row closure: each diagonal is -(exit rate) and each exit rate equals the rate counted independently from the section 3.4 rules".to_string(),
    ))
}

// ---------------------------------------------------------------- G04

/// Externally supplied work per channel: the model is undriven.
const fn external_work(_kind: Kind) -> i64 {
    0
}

pub(crate) fn gate_04(cx: &Context) -> GateResult {
    let m = cx.model;
    let levels = levels(m);
    let total = LAMBDA * m.bridge_count() as i64;
    let energies = |level: &Level, from: usize, to: usize| {
        let from_config = m.states[level.system_of[from]].config;
        let to_config = m.states[level.system_of[to]].config;
        let delta_u = LAMBDA * (m.size(to_config) as i64 - m.size(from_config) as i64);
        let delta_reservoir = LAMBDA
            * (m.reservoir_energy(to_config) as i64 - m.reservoir_energy(from_config) as i64);
        (from_config, to_config, delta_u, delta_reservoir)
    };

    for level in &levels {
        for (from, row) in level.generator.rows.iter().enumerate() {
            for entry in row.iter().filter(|entry| entry.kind.is_toggle()) {
                let (from_config, _, delta_u, delta_reservoir) = energies(level, from, entry.to);
                let sign = if entry.kind == Kind::Build { 1 } else { -1 };
                if delta_u != sign * LAMBDA || delta_reservoir != -sign * LAMBDA {
                    return Err(fail(
                        CheckId::G04_TOGGLE_ENERGY,
                        Some(from_config),
                        format!(
                            "{} {} {from}->{}: dU={delta_u}, dE_R={delta_reservoir}",
                            level.name,
                            entry.kind.code(),
                            entry.to
                        ),
                    ));
                }
            }
        }
    }
    for level in &levels {
        for (from, row) in level.generator.rows.iter().enumerate() {
            for entry in row {
                let (from_config, _, delta_u, delta_reservoir) = energies(level, from, entry.to);
                let heat_into_system = -delta_reservoir;
                if heat_into_system != delta_u {
                    return Err(fail(
                        CheckId::G04_HEAT_EQUALS_DELTA_U,
                        Some(from_config),
                        format!(
                            "{} {} {from}->{}: heat {heat_into_system} != dU {delta_u}",
                            level.name,
                            entry.kind.code(),
                            entry.to
                        ),
                    ));
                }
            }
        }
    }
    for level in &levels {
        for (from, row) in level.generator.rows.iter().enumerate() {
            for entry in row {
                let (from_config, to_config, delta_u, delta_reservoir) =
                    energies(level, from, entry.to);
                let work = external_work(entry.kind);
                let before = LAMBDA * m.size(from_config) as i64
                    + LAMBDA * m.reservoir_energy(from_config) as i64;
                let after = LAMBDA * m.size(to_config) as i64
                    + LAMBDA * m.reservoir_energy(to_config) as i64;
                if work != 0
                    || delta_u != -delta_reservoir + work
                    || before != total
                    || after != total
                {
                    return Err(fail(
                        CheckId::G04_ZERO_WORK,
                        Some(from_config),
                        format!(
                            "{} {} {from}->{}: work {work}, totals {before}->{after}",
                            level.name,
                            entry.kind.code(),
                            entry.to
                        ),
                    ));
                }
            }
        }
    }
    for level in &levels {
        for (from, row) in level.generator.rows.iter().enumerate() {
            for entry in row.iter().filter(|entry| !entry.kind.is_toggle()) {
                let (from_config, to_config, delta_u, delta_reservoir) =
                    energies(level, from, entry.to);
                let same_label = level
                    .label_of
                    .is_none_or(|labels| labels[from] == labels[entry.to]);
                if delta_u != 0 || delta_reservoir != 0 || from_config != to_config || !same_label {
                    return Err(fail(
                        CheckId::G04_NEUTRAL_CHANNELS,
                        Some(from_config),
                        format!(
                            "{} {} {from}->{} changes energy, configuration, or reservoir label",
                            level.name,
                            entry.kind.code(),
                            entry.to
                        ),
                    ));
                }
            }
        }
    }
    for level in &levels {
        for (from, row) in level.generator.rows.iter().enumerate() {
            for entry in row.iter().filter(|entry| !entry.kind.is_toggle()) {
                let reverse_row = &level.generator.rows[entry.to];
                let reversible = reverse_row
                    .binary_search_by_key(&from, |reverse| reverse.to)
                    .is_ok_and(|position| reverse_row[position].kind == entry.kind);
                if !reversible {
                    return Err(fail(
                        CheckId::G04_REGISTER_REVERSIBILITY,
                        Some(m.states[level.system_of[from]].config),
                        format!(
                            "{} {} register write {from}->{} has no reverse write",
                            level.name,
                            entry.kind.code(),
                            entry.to
                        ),
                    ));
                }
            }
        }
    }
    pass()
}

// ---------------------------------------------------------------- G05

pub(crate) fn gate_05(cx: &Context) -> GateResult {
    let m = cx.model;
    let Some(micro) = &m.micro else {
        return Ok(Status::NotApplicable(
            "reservoir lift not enumerated for N = 4".to_string(),
        ));
    };
    let lumped_row = |complete: usize| -> BTreeMap<usize, Ratio> {
        let mut row = BTreeMap::new();
        for entry in &micro.generator.rows[complete] {
            let target = micro.system_of[entry.to];
            let rate = row.entry(target).or_insert(Ratio::ZERO);
            *rate = *rate + entry.hazard;
        }
        row
    };
    for state in 0..m.states.len() {
        let first = micro.offsets[state];
        let reference = lumped_row(first);
        for complete in first + 1..micro.offsets[state + 1] {
            if lumped_row(complete) != reference {
                return Err(fail(
                    CheckId::G05_STRONG_LUMPABILITY,
                    Some(m.states[state].config),
                    format!(
                        "complete states {first} and {complete} of {} send different rates",
                        m.describe_state(state)
                    ),
                ));
            }
        }
    }
    for state in 0..m.states.len() {
        let observed = lumped_row(micro.offsets[state]);
        let expected: BTreeMap<usize, Ratio> = m.system.rows[state]
            .iter()
            .map(|entry| {
                let inputs = HazardInputs {
                    kind: entry.kind,
                    destination_reservoir_energy: m.reservoir_energy(m.states[entry.to].config),
                    base: m.case.b,
                };
                (entry.to, lumped_hazard(inputs))
            })
            .collect();
        if observed != expected {
            return Err(fail(
                CheckId::G05_LUMPED_HAZARDS,
                Some(m.states[state].config),
                format!(
                    "lumped rates of {} differ from b^(E_R(z')) / 1",
                    m.describe_state(state)
                ),
            ));
        }
    }
    pass()
}

// ---------------------------------------------------------------- G06

pub(crate) fn gate_06(cx: &Context) -> GateResult {
    let m = cx.model;
    let reservoir = |energy: usize| integer_power(m.case.b, energy as u64);
    for energy in 0..m.bridge_count() {
        let ratio = Ratio::new(reservoir(energy + 1), reservoir(energy));
        if ratio != Ratio::integer(m.case.b) {
            return Err(fail(
                CheckId::G06_RESERVOIR_RELATION,
                None,
                format!(
                    "g_R({})/g_R({energy}) = {ratio} != b = {}",
                    energy + 1,
                    m.case.b
                ),
            ));
        }
    }
    // S(z) = ln g(z) with g(z) = 1 for every system state.
    let system_multiplicity = |_state: usize| Ratio::ONE;
    for (from, row) in m.system.rows.iter().enumerate() {
        let from_config = m.states[from].config;
        for entry in row {
            let to_config = m.states[entry.to].config;
            let reverse = m.system.hazard(entry.to, from);
            let delta_u = LAMBDA * (m.size(to_config) as i64 - m.size(from_config) as i64);
            let predicted = system_multiplicity(entry.to) / system_multiplicity(from)
                * Ratio::power(m.case.b, -delta_u);
            if !reverse.is_positive() || entry.hazard != predicted * reverse {
                return Err(fail(
                    CheckId::G06_LOCAL_DETAILED_BALANCE,
                    Some(from_config),
                    format!(
                        "{} {} -> {}: k={} k_rev={reverse}, predicted ratio {predicted}",
                        entry.kind.code(),
                        m.describe_state(from),
                        m.describe_state(entry.to),
                        entry.hazard
                    ),
                ));
            }
        }
    }
    pass()
}

// ---------------------------------------------------------------- G07

fn reach(rows: &[Vec<usize>], root: usize) -> Vec<bool> {
    let mut seen = vec![false; rows.len()];
    if rows.is_empty() {
        return seen;
    }
    seen[root] = true;
    let mut queue = vec![root];
    while let Some(state) = queue.pop() {
        for &next in &rows[state] {
            if !seen[next] {
                seen[next] = true;
                queue.push(next);
            }
        }
    }
    seen
}

pub(crate) fn gate_07(cx: &Context) -> GateResult {
    let m = cx.model;
    let forward: Vec<Vec<usize>> = m
        .system
        .rows
        .iter()
        .map(|row| row.iter().map(|entry| entry.to).collect())
        .collect();
    let mut backward: Vec<Vec<usize>> = vec![Vec::new(); forward.len()];
    for (from, row) in forward.iter().enumerate() {
        for &to in row {
            backward[to].push(from);
        }
    }
    let reached_forward = reach(&forward, 0);
    let reached_backward = reach(&backward, 0);
    if let Some(state) =
        (0..forward.len()).find(|&state| !reached_forward[state] || !reached_backward[state])
    {
        return Err(fail(
            CheckId::G07_CONNECTIVITY,
            Some(m.states[state].config),
            format!(
                "{} is not mutually reachable with z0",
                m.describe_state(state)
            ),
        ));
    }

    let weight = |state: usize| m.boltzmann_weight(m.states[state].config);
    let mut inflow = vec![Ratio::ZERO; m.states.len()];
    for (from, row) in m.system.rows.iter().enumerate() {
        for entry in row {
            inflow[entry.to] = inflow[entry.to] + weight(from) * entry.hazard;
        }
    }
    for (state, &incoming) in inflow.iter().enumerate() {
        let outgoing = weight(state) * m.system.escape(state);
        if incoming != outgoing {
            return Err(fail(
                CheckId::G07_STATIONARITY,
                Some(m.states[state].config),
                format!(
                    "{}: inflow {incoming} != outflow {outgoing}",
                    m.describe_state(state)
                ),
            ));
        }
    }
    for (from, row) in m.system.rows.iter().enumerate() {
        for entry in row {
            let forward_flux = weight(from) * entry.hazard;
            let reverse_flux = weight(entry.to) * m.system.hazard(entry.to, from);
            if forward_flux != reverse_flux {
                return Err(fail(
                    CheckId::G07_DETAILED_BALANCE,
                    Some(m.states[from].config),
                    format!(
                        "{} <-> {}: fluxes {forward_flux} and {reverse_flux}",
                        m.describe_state(from),
                        m.describe_state(entry.to)
                    ),
                ));
            }
        }
    }
    pass()
}

// ---------------------------------------------------------------- G08

/// Frozen values of boundary section 5.2 (copied, not computed).
pub(crate) struct FrozenFamilyRow {
    pub(crate) case: Case,
    pub(crate) configurations: usize,
    pub(crate) system_states: usize,
    pub(crate) complete_states: usize,
    pub(crate) count_range: (i128, i128),
    pub(crate) partition: (i128, i128),
    pub(crate) mean_bridges: (i128, i128),
}

const fn row(
    case: (usize, usize, i128),
    configurations: usize,
    system_states: usize,
    complete_states: usize,
    count_range: (i128, i128),
    partition: (i128, i128),
    mean_bridges: (i128, i128),
) -> FrozenFamilyRow {
    FrozenFamilyRow {
        case: Case::new(case.0, case.1, case.2),
        configurations,
        system_states,
        complete_states,
        count_range,
        partition,
        mean_bridges,
    }
}

pub(crate) const FROZEN_FAMILY: [FrozenFamilyRow; 8] = [
    row((3, 1, 2), 8, 32, 90, (1, 3), (45, 8), (19, 15)),
    row((3, 2, 2), 8, 108, 261, (1, 9), (87, 8), (131, 87)),
    row((3, 3, 2), 8, 344, 740, (1, 27), (185, 8), (313, 185)),
    row((3, 2, 3), 8, 108, 504, (1, 9), (56, 9), (5, 4)),
    row((4, 1, 2), 64, 320, 2_916, (1, 4), (729, 32), (7, 3)),
    row((4, 2, 2), 64, 1_344, 10_206, (1, 16), (1701, 32), (55, 21)),
    row(
        (4, 2, 3),
        64,
        1_344,
        44_544,
        (1, 16),
        (14848, 729),
        (123, 58),
    ),
    row((4, 3, 2), 64, 5_248, 34_344, (1, 64), (4293, 32), (151, 53)),
];

/// Frozen per-configuration table of boundary section 5.1.
pub(crate) const FROZEN_PRIMARY_SIZES: [usize; 8] = [0, 1, 1, 2, 1, 2, 2, 3];
pub(crate) const FROZEN_PRIMARY_COUNTS: [i128; 8] = [1, 4, 4, 7, 1, 5, 5, 9];
pub(crate) const FROZEN_PRIMARY_SYSTEM: [usize; 8] = [3, 12, 12, 21, 3, 15, 15, 27];
pub(crate) const FROZEN_PRIMARY_COMPLETE: [usize; 8] = [24, 48, 48, 42, 12, 30, 30, 27];
pub(crate) const FROZEN_PRIMARY_WEIGHTS: [(i128, i128); 8] = [
    (1, 1),
    (2, 1),
    (2, 1),
    (7, 4),
    (1, 2),
    (5, 4),
    (5, 4),
    (9, 8),
];
pub(crate) const FROZEN_PRIMARY_TOTALS: (usize, usize) = (108, 261);
/// Section 5.1 example: `kbar(none -> {0,1}) / kbar({0,1} -> none) = 2`.
pub(crate) const FROZEN_PRIMARY_EXAMPLE_ODDS: (usize, usize, i128) = (0, 1, 2);
/// Section 7 G09: the primary witness's zero-flux pairs are exactly `none <-> {1,2}`.
pub(crate) const FROZEN_PRIMARY_ZERO_FLUX: [(usize, usize); 1] = [(0, 4)];

fn frozen_row(case: Case) -> Option<&'static FrozenFamilyRow> {
    FROZEN_FAMILY.iter().find(|row| row.case == case)
}

/// `Z = sum_G N_tau(G) b^(-|G|)` from the matrix-power counts.
pub(crate) fn partition_function(m: &Model) -> Ratio {
    (0..m.config_count).fold(Ratio::ZERO, |sum, config| {
        sum + Ratio::integer(m.matrix_count[config]) * m.boltzmann_weight(config)
    })
}

/// Mean bridge count under the enumerated configuration marginal.
pub(crate) fn mean_bridges(m: &Model) -> Ratio {
    let mass = configuration_mass(m);
    let total = mass.iter().fold(Ratio::ZERO, |sum, &value| sum + value);
    let weighted = mass
        .iter()
        .enumerate()
        .fold(Ratio::ZERO, |sum, (config, &value)| {
            sum + value * Ratio::integer(m.size(config) as i128)
        });
    weighted / total
}

pub(crate) fn gate_08(cx: &Context) -> GateResult {
    let m = cx.model;
    let mass = configuration_mass(m);
    let total = mass.iter().fold(Ratio::ZERO, |sum, &value| sum + value);
    let partition = partition_function(m);
    for (config, &config_mass) in mass.iter().enumerate() {
        let marginal = config_mass / total;
        let predicted =
            Ratio::integer(m.matrix_count[config]) * m.boltzmann_weight(config) / partition;
        if marginal != predicted {
            return Err(fail(
                CheckId::G08_CONFIGURATION_MARGINAL,
                Some(config),
                format!("marginal {marginal} != N_tau b^(-|G|)/Z = {predicted}"),
            ));
        }
    }
    let Some(frozen) = frozen_row(m.case) else {
        return Err(fail(
            CheckId::G08_PARTITION_AND_MEAN,
            None,
            format!("case {} has no section 5 row", m.case.label()),
        ));
    };
    let mean = mean_bridges(m);
    let frozen_partition = Ratio::new(frozen.partition.0, frozen.partition.1);
    let frozen_mean = Ratio::new(frozen.mean_bridges.0, frozen.mean_bridges.1);
    if partition != frozen_partition || mean != frozen_mean {
        return Err(fail(
            CheckId::G08_PARTITION_AND_MEAN,
            None,
            format!(
                "Z={partition} mean={mean}, section 5: Z={frozen_partition} mean={frozen_mean}"
            ),
        ));
    }
    let counts = &m.matrix_count;
    let range = (
        counts.iter().copied().min().unwrap_or(0),
        counts.iter().copied().max().unwrap_or(0),
    );
    let complete = m.complete_state_count();
    let lift_identity = Ratio::integer(complete as i128)
        == partition
            * Ratio::integer(
                (m.case.tau as i128 + 1) * integer_power(m.case.b, m.bridge_count() as u64),
            );
    if m.config_count != frozen.configurations
        || m.states.len() != frozen.system_states
        || complete != frozen.complete_states
        || range != frozen.count_range
        || !lift_identity
    {
        return Err(fail(
            CheckId::G08_FROZEN_TABLE,
            None,
            format!(
                "configurations={} system={} complete={complete} N_tau range={}..{}; section 5.2: {} {} {} {}..{}",
                m.config_count,
                m.states.len(),
                range.0,
                range.1,
                frozen.configurations,
                frozen.system_states,
                frozen.complete_states,
                frozen.count_range.0,
                frozen.count_range.1
            ),
        ));
    }
    if m.case == PRIMARY {
        let micro = m.micro.as_ref().expect("primary witness has N = 3");
        for config in 0..m.config_count {
            let system = m.states_of(config).len();
            let complete = micro.offsets[m.config_offsets[config + 1]]
                - micro.offsets[m.config_offsets[config]];
            let weight = Ratio::integer(m.matrix_count[config]) * m.boltzmann_weight(config);
            let (numerator, denominator) = FROZEN_PRIMARY_WEIGHTS[config];
            if m.size(config) != FROZEN_PRIMARY_SIZES[config]
                || m.matrix_count[config] != FROZEN_PRIMARY_COUNTS[config]
                || system != FROZEN_PRIMARY_SYSTEM[config]
                || complete != FROZEN_PRIMARY_COMPLETE[config]
                || weight != Ratio::new(numerator, denominator)
            {
                return Err(fail(
                    CheckId::G08_FROZEN_TABLE,
                    Some(config),
                    format!(
                        "|G|={} N_2={} system={system} complete={complete} weight={weight} differ from section 5.1",
                        m.size(config),
                        m.matrix_count[config]
                    ),
                ));
            }
        }
        if (m.states.len(), m.complete_state_count()) != FROZEN_PRIMARY_TOTALS {
            return Err(fail(
                CheckId::G08_FROZEN_TABLE,
                None,
                "primary totals differ from 108 system / 261 complete states",
            ));
        }
    }
    pass()
}

// ---------------------------------------------------------------- G09

pub(crate) fn gate_09(cx: &Context) -> GateResult {
    let m = cx.model;
    let neighbours =
        |config: usize| (0..m.bridge_count()).map(move |bridge| config ^ (1 << bridge));
    for config in 0..m.config_count {
        for other in neighbours(config) {
            let product = exp_xi(m, config, other) * exp_xi(m, other, config);
            if product != Ratio::ONE {
                return Err(fail(
                    CheckId::G09_XI_ANTISYMMETRY,
                    Some(config),
                    format!(
                        "exp(Xi/alpha) forward*reverse = {product} for {} <-> {}",
                        m.config_label(config),
                        m.config_label(other)
                    ),
                ));
            }
        }
    }

    let mass = configuration_mass(m);
    let fluxes = configuration_fluxes(m);
    let mut zero_flux = Vec::new();
    let mut positive = 0;
    for config in 0..m.config_count {
        for other in neighbours(config) {
            let forward = flux(&fluxes, config, other);
            let reverse = flux(&fluxes, other, config);
            match (forward.is_positive(), reverse.is_positive()) {
                (false, false) => {
                    if config < other {
                        zero_flux.push((config, other));
                    }
                }
                (true, true) => {
                    positive += 1;
                    // kbar(G->G')/kbar(G'->G) = exp(Xi/alpha), cross-multiplied.
                    let left = forward * mass[other];
                    let right = exp_xi(m, config, other) * mass[config] * reverse;
                    if left != right {
                        return Err(fail(
                            CheckId::G09_EMERGENT_ODDS,
                            Some(config),
                            format!(
                                "{} -> {}: F pi(G') = {left} != exp(Xi/alpha) pi(G) F_rev = {right}",
                                m.config_label(config),
                                m.config_label(other)
                            ),
                        ));
                    }
                }
                _ => {
                    return Err(fail(
                        CheckId::G09_EMERGENT_ODDS,
                        Some(config),
                        format!(
                            "one-sided toggle flux {} -> {}: {forward} vs {reverse}",
                            m.config_label(config),
                            m.config_label(other)
                        ),
                    ));
                }
            }
        }
    }
    let zero_labels: Vec<String> = zero_flux
        .iter()
        .map(|&(left, right)| format!("{}<->{}", m.config_label(left), m.config_label(right)))
        .collect();
    if m.case == PRIMARY {
        let (from, to, odds) = FROZEN_PRIMARY_EXAMPLE_ODDS;
        let left = flux(&fluxes, from, to) * mass[to];
        let right = Ratio::integer(odds) * mass[from] * flux(&fluxes, to, from);
        if zero_flux != FROZEN_PRIMARY_ZERO_FLUX || left.is_zero() || left != right {
            return Err(fail(
                CheckId::G09_EMERGENT_ODDS,
                None,
                format!(
                    "zero-flux pairs [{}] (expected none<->{{1,2}}); example odds check {left} vs {right}",
                    zero_labels.join(" ")
                ),
            ));
        }
    }

    let rates_out = |state: usize| -> BTreeMap<usize, Ratio> {
        let from = m.states[state].config;
        let mut rates = BTreeMap::new();
        for entry in &m.system.rows[state] {
            let to = m.states[entry.to].config;
            if to != from {
                let rate = rates.entry(to).or_insert(Ratio::ZERO);
                *rate = *rate + entry.hazard;
            }
        }
        rates
    };
    let mut general = None;
    'search: for config in 0..m.config_count {
        let members: Vec<usize> = m.states_of(config).collect();
        let Some(&first) = members.first() else {
            continue;
        };
        let reference = rates_out(first);
        for &state in &members[1..] {
            if rates_out(state) != reference {
                general = Some((first, state));
                break 'search;
            }
        }
    }
    let Some((first, second)) = general else {
        return Err(fail(
            CheckId::G09_NON_LUMPABILITY,
            None,
            "configuration projection is strongly lumpable",
        ));
    };
    let mut note = format!(
        "positive-flux pairs={} zero-flux=[{}] non-lumpable at {} vs {}",
        positive,
        zero_labels.join(" "),
        m.describe_state(first),
        m.describe_state(second)
    );
    if m.case == PRIMARY {
        // Section 4.3: at {0,1}, home can build {0,2} not {1,2}; island 1 the reverse.
        let (at, via_02, via_12) = (0b001, 0b011, 0b101);
        let rate = |state: usize, target: usize| {
            rates_out(state)
                .get(&target)
                .copied()
                .unwrap_or(Ratio::ZERO)
        };
        let home = m.states_of(at).find(|&state| {
            m.position(state) == HOME
                && rate(state, via_02).is_positive()
                && rate(state, via_12).is_zero()
        });
        let island_one = m.states_of(at).find(|&state| {
            m.position(state) == 1
                && rate(state, via_12).is_positive()
                && rate(state, via_02).is_zero()
        });
        match (home, island_one) {
            (Some(home), Some(island_one)) => {
                note = format!(
                    "{note}; section 4.3 witness {} vs {}",
                    m.describe_state(home),
                    m.describe_state(island_one)
                );
            }
            _ => {
                return Err(fail(
                    CheckId::G09_NON_LUMPABILITY,
                    Some(at),
                    "section 4.3 witness pair not found",
                ));
            }
        }
    }
    Ok(Status::Pass(note))
}

// ---------------------------------------------------------------- G10

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GraphSubstrateBinding {
    BridgeConfigurationsWithHeldPlanAndCursor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CrystalBinding {
    UniformReplanOverValidPlans,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThermoBinding {
    ReservoirLawEnergyThaimPairAndXi,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PraxionBinding {
    BaseActorUnitSymmetricHazards,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActionBinding {
    BuildOrDismantleAtTravellerIsland,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResolutionBinding {
    StepArrivalAndReplanWrite,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThawBinding {
    DeclaredReverseOfEachChannel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MemoryBinding {
    HeldPlanAndCursorNoLearning,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct XypherSlotReading {
    graph_substrate: GraphSubstrateBinding,
    crystal: CrystalBinding,
    thermo: ThermoBinding,
    praxion: PraxionBinding,
    action: ActionBinding,
    resolution: ResolutionBinding,
    thaw: ThawBinding,
    memory: MemoryBinding,
    ruby: Option<()>,
    opal_phi: Option<()>,
    thaim_ledger: Option<()>,
}

/// Quantities the construction exposes; `HAZARD_READS` lists those a
/// channel hazard may read (mirrors `HazardInputs`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Quantity {
    ChannelKind,
    DestinationReservoirEnergy,
    ReservoirBase,
    PlanCount,
    Thaim,
    Xi,
}

const HAZARD_READS: [Quantity; 3] = [
    Quantity::ChannelKind,
    Quantity::DestinationReservoirEnergy,
    Quantity::ReservoirBase,
];
const FORBIDDEN_HAZARD_READS: [Quantity; 3] = [Quantity::PlanCount, Quantity::Thaim, Quantity::Xi];

pub(crate) fn gate_10(cx: &Context) -> GateResult {
    let m = cx.model;
    let slots = XypherSlotReading {
        graph_substrate: GraphSubstrateBinding::BridgeConfigurationsWithHeldPlanAndCursor,
        crystal: CrystalBinding::UniformReplanOverValidPlans,
        thermo: ThermoBinding::ReservoirLawEnergyThaimPairAndXi,
        praxion: PraxionBinding::BaseActorUnitSymmetricHazards,
        action: ActionBinding::BuildOrDismantleAtTravellerIsland,
        resolution: ResolutionBinding::StepArrivalAndReplanWrite,
        thaw: ThawBinding::DeclaredReverseOfEachChannel,
        memory: MemoryBinding::HeldPlanAndCursorNoLearning,
        ruby: None,
        opal_phi: None,
        thaim_ledger: None,
    };
    let named = matches!(
        slots,
        XypherSlotReading {
            graph_substrate: GraphSubstrateBinding::BridgeConfigurationsWithHeldPlanAndCursor,
            crystal: CrystalBinding::UniformReplanOverValidPlans,
            thermo: ThermoBinding::ReservoirLawEnergyThaimPairAndXi,
            praxion: PraxionBinding::BaseActorUnitSymmetricHazards,
            action: ActionBinding::BuildOrDismantleAtTravellerIsland,
            resolution: ResolutionBinding::StepArrivalAndReplanWrite,
            thaw: ThawBinding::DeclaredReverseOfEachChannel,
            memory: MemoryBinding::HeldPlanAndCursorNoLearning,
            ..
        }
    );
    if !named {
        return Err(fail(
            CheckId::G10_NAMED_SLOTS,
            None,
            "one or more section 3.5 bindings are unbound",
        ));
    }
    if slots.ruby.is_some() || slots.opal_phi.is_some() || slots.thaim_ledger.is_some() {
        return Err(fail(
            CheckId::G10_EMPTY_RUBY,
            None,
            "Ruby, Opal/Phi, and a cumulative THAIM ledger must be empty",
        ));
    }
    if HAZARD_READS
        .iter()
        .any(|read| FORBIDDEN_HAZARD_READS.contains(read))
    {
        return Err(fail(
            CheckId::G10_HAZARD_INPUTS,
            None,
            format!("hazard reads {HAZARD_READS:?} include a forbidden quantity"),
        ));
    }
    for (from, row) in m.system.rows.iter().enumerate() {
        for entry in row {
            let expected = expected_system_hazard(m, entry.kind, m.states[entry.to].config);
            if entry.hazard != expected {
                return Err(fail(
                    CheckId::G10_HAZARD_INPUTS,
                    Some(m.states[from].config),
                    format!(
                        "{} hazard {} at {} differs from the b^(E_R(z')) / 1 rule {expected}",
                        entry.kind.code(),
                        entry.hazard,
                        m.describe_state(from)
                    ),
                ));
            }
        }
    }
    if let Some(micro) = &m.micro {
        for (from, row) in micro.generator.rows.iter().enumerate() {
            if let Some(entry) = row.iter().find(|entry| entry.hazard != KAPPA) {
                return Err(fail(
                    CheckId::G10_HAZARD_INPUTS,
                    Some(m.states[micro.system_of[from]].config),
                    format!(
                        "micro {} hazard {} != kappa at complete state {from}",
                        entry.kind.code(),
                        entry.hazard
                    ),
                ));
            }
        }
    }
    Ok(Status::Pass(format!(
        "hazards are computed from HazardInputs {HAZARD_READS:?}, a type with no N_tau, THAIM, or Xi field; every system hazard equals the independent b^(E_R(z'))/1 rule and every micro hazard is kappa"
    )))
}

// ---------------------------------------------------------------- G11

pub(crate) fn gate_11(cx: &Context) -> GateResult {
    let contact = cx.contact();
    for pair in &contact.pairs {
        for shell in &pair.shells {
            if let Some(detail) = &shell.balance_failure {
                return Err(fail(
                    CheckId::G11_PREPARED_STATIONARY,
                    None,
                    format!("{} pair {}: {detail}", pair.name, pair.label()),
                ));
            }
        }
    }
    for pair in &contact.pairs {
        for shell in &pair.shells {
            if let Some(detail) = &shell.component_failure {
                return Err(fail(
                    CheckId::G11_COMPONENT_UNIFORM,
                    None,
                    format!("{} pair {}: {detail}", pair.name, pair.label()),
                ));
            }
        }
    }
    for pair in &contact.pairs {
        for shell in &pair.shells {
            if !shell.current.is_zero() {
                return Err(fail(
                    CheckId::G11_ZERO_CURRENT,
                    None,
                    format!(
                        "{} pair {} shell M={}: current into A = {}",
                        pair.name,
                        pair.label(),
                        shell.shell,
                        shell.current
                    ),
                ));
            }
        }
    }
    let note: Vec<String> = contact
        .pairs
        .iter()
        .map(|pair| {
            let shells: Vec<String> = pair
                .shells
                .iter()
                .map(|shell| {
                    let sizes: Vec<String> =
                        shell.components.iter().map(usize::to_string).collect();
                    format!("M{}:{}[{}]", shell.shell, shell.states, sizes.join("+"))
                })
                .collect();
            format!("{} components {}", pair.name, shells.join(" "))
        })
        .collect();
    Ok(Status::Pass(note.join("; ")))
}

// ---------------------------------------------------------------- G12

pub(crate) fn gate_12(cx: &Context) -> GateResult {
    let m = cx.model;
    if m.case.tau != 2 {
        return Ok(Status::NotApplicable("tau != 2".to_string()));
    }
    let adjacent_home = |config: usize, v: usize| m.bridge_built(config, HOME, v);
    let closed_count = |config: usize| -> i128 {
        (0..m.case.n)
            .filter(|&v| v == HOME || adjacent_home(config, v))
            .map(|v| m.degree(config, v) as i128 + 1)
            .sum()
    };
    for config in 0..m.config_count {
        let closed = closed_count(config);
        if closed != m.matrix_count[config] {
            return Err(fail(
                CheckId::G12_COUNT_FORMULA,
                Some(config),
                format!(
                    "sum over N[h] of (d_v+1) = {closed} != N_2 = {}",
                    m.matrix_count[config]
                ),
            ));
        }
    }
    for config in 0..m.config_count {
        for (bridge, &(u, w)) in m.bridges.iter().enumerate() {
            if m.has_bridge(config, bridge) {
                continue;
            }
            let delta = m.matrix_count[config | (1 << bridge)] - m.matrix_count[config];
            let predicted = if u == HOME {
                m.degree(config, w) as i128 + 3
            } else {
                i128::from(adjacent_home(config, u)) + i128::from(adjacent_home(config, w))
            };
            if delta != predicted {
                return Err(fail(
                    CheckId::G12_DELTA_FORMULA,
                    Some(config),
                    format!("building {{{u},{w}}}: Delta N_2 = {delta} != closed form {predicted}"),
                ));
            }
        }
    }
    let mass = configuration_mass(m);
    let fluxes = configuration_fluxes(m);
    let mut pairs = 0;
    for config in 0..m.config_count {
        for (bridge, &(u, w)) in m.bridges.iter().enumerate() {
            if m.has_bridge(config, bridge) || u != HOME {
                continue;
            }
            let other = config | (1 << bridge);
            let n2 = closed_count(config);
            let odds = Ratio::new(n2 + m.degree(config, w) as i128 + 3, m.case.b * n2);
            let forward = flux(&fluxes, config, other);
            let reverse = flux(&fluxes, other, config);
            let left = forward * mass[other];
            let right = odds * mass[config] * reverse;
            if !forward.is_positive() || !reverse.is_positive() || left != right {
                return Err(fail(
                    CheckId::G12_HOME_BRIDGE_ODDS,
                    Some(config),
                    format!(
                        "home bridge {{0,{w}}}: F pi(G') = {left} != [N_2+d_u+3]/[b N_2] pi(G) F_rev = {right}"
                    ),
                ));
            }
            pairs += 1;
        }
    }
    Ok(Status::Pass(format!("home-bridge pairs={pairs}")))
}
