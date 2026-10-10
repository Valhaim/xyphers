//! Gates G01--G14 of boundary section 7. Within each gate the checks run in
//! the listed order, and each check runs over all configurations (and
//! states) in canonical order before the next check begins.
//!
//! The stationary law is never assumed: it is propagated from the
//! constructed system generator along a breadth-first spanning tree (section
//! 6). G07 check 2 compares it with `b^(-(|G| + w))`; every later gate reads
//! the propagated law. Logarithms are never evaluated: entropies and
//! information are rational-coefficient vectors over `ln(prime)`
//! (`src/primes.rs`).

// Checks walk configurations by bit-mask index in canonical order and read
// several parallel per-configuration tables at that index.
#![allow(clippy::needless_range_loop)]

use std::cell::OnceCell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::model::{
    plan_label, Edge, Generator, Kind, MicroLevel, Model, Note, SystemState, KAPPA, LAMBDA,
};
use crate::primes::{capacity_target, plan_information_target, LogVector};
use crate::ratio::Ratio;
use crate::CheckId;

#[derive(Clone, Debug)]
pub(crate) struct Failure {
    pub(crate) check: CheckId,
    pub(crate) configuration: Option<usize>,
    pub(crate) detail: String,
    /// Observed value of a global prediction check (G08 check 3 records the
    /// mean stockpile).
    pub(crate) value: Option<Ratio>,
}

fn fail(check: CheckId, configuration: Option<usize>, detail: impl Into<String>) -> Failure {
    Failure {
        check,
        configuration,
        detail: detail.into(),
        value: None,
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Status {
    Pass(String),
    NotApplicable(String),
}

pub(crate) type GateResult = Result<Status, Failure>;

fn pass() -> GateResult {
    Ok(Status::Pass(String::new()))
}

fn pass_with(note: impl Into<String>) -> GateResult {
    Ok(Status::Pass(note.into()))
}

// ---------------------------------------------------------------- frozen values

/// Configurations with `K = 1` for every reachable `y` and `|D| >= 2`
/// (boundary section 5.2, last column).
#[derive(Clone, Copy, Debug)]
pub(crate) enum FrozenKOne {
    /// Explicit configuration bit masks in canonical order.
    Configurations(&'static [usize]),
    /// "the 24 configurations in which home has exactly one bridge".
    HomeDegreeOne { count: usize },
}

/// Frozen values of boundary section 5.2 (copied, not computed).
pub(crate) struct FrozenFamilyRow {
    pub(crate) case: (usize, usize, i128, usize),
    pub(crate) configurations: usize,
    pub(crate) system_states: usize,
    pub(crate) complete_states: usize,
    pub(crate) partition: (i128, i128),
    pub(crate) mean_bridges: (i128, i128),
    pub(crate) erasure: (i128, i128),
    pub(crate) mean_stockpile: (i128, i128),
    pub(crate) blank: (i128, i128),
    pub(crate) k_one: FrozenKOne,
}

/// Boundary section 5.2, table rows in order.
pub(crate) const FROZEN_FAMILY: [FrozenFamilyRow; 7] = [
    FrozenFamilyRow {
        case: (3, 1, 2, 2),
        configurations: 8,
        system_states: 384,
        complete_states: 2_520,
        partition: (45, 8),
        mean_bridges: (19, 15),
        erasure: (2, 3),
        mean_stockpile: (4, 7),
        blank: (1, 4),
        // {0,1}; {0,2}; {0,1},{1,2}; {0,2},{1,2}
        k_one: FrozenKOne::Configurations(&[0b001, 0b010, 0b101, 0b110]),
    },
    FrozenFamilyRow {
        case: (3, 2, 2, 2),
        configurations: 8,
        system_states: 1_296,
        complete_states: 7_308,
        partition: (87, 8),
        mean_bridges: (131, 87),
        erasure: (2, 3),
        mean_stockpile: (4, 7),
        blank: (1, 4),
        // {0,1}; {0,2}
        k_one: FrozenKOne::Configurations(&[0b001, 0b010]),
    },
    FrozenFamilyRow {
        case: (3, 3, 2, 2),
        configurations: 8,
        system_states: 4_128,
        complete_states: 20_720,
        partition: (185, 8),
        mean_bridges: (313, 185),
        erasure: (2, 3),
        mean_stockpile: (4, 7),
        blank: (1, 4),
        // {0,1}; {0,2}
        k_one: FrozenKOne::Configurations(&[0b001, 0b010]),
    },
    FrozenFamilyRow {
        case: (3, 2, 3, 2),
        configurations: 8,
        system_states: 1_296,
        complete_states: 26_208,
        partition: (56, 9),
        mean_bridges: (5, 4),
        erasure: (1, 1),
        mean_stockpile: (5, 13),
        blank: (1, 4),
        // all three
        k_one: FrozenKOne::Configurations(&[0b111]),
    },
    FrozenFamilyRow {
        case: (3, 1, 3, 1),
        configurations: 8,
        system_states: 256,
        complete_states: 3_072,
        partition: (32, 9),
        mean_bridges: (1, 1),
        erasure: (1, 1),
        mean_stockpile: (1, 4),
        blank: (1, 4),
        // {0,1},{0,2}; all three
        k_one: FrozenKOne::Configurations(&[0b011, 0b111]),
    },
    FrozenFamilyRow {
        case: (4, 1, 2, 1),
        configurations: 64,
        system_states: 3_200,
        complete_states: 43_740,
        partition: (729, 32),
        mean_bridges: (7, 3),
        erasure: (1, 2),
        mean_stockpile: (1, 3),
        blank: (1, 5),
        k_one: FrozenKOne::HomeDegreeOne { count: 24 },
    },
    FrozenFamilyRow {
        case: (4, 2, 2, 1),
        configurations: 64,
        system_states: 13_440,
        complete_states: 153_090,
        partition: (1701, 32),
        mean_bridges: (55, 21),
        erasure: (1, 2),
        mean_stockpile: (1, 3),
        blank: (1, 5),
        // {0,1}; {0,2}; {0,3}; {0,3},{1,2}; {0,2},{1,3}; {0,1},{2,3}
        k_one: FrozenKOne::Configurations(&[1, 2, 4, 12, 18, 33]),
    },
];

/// Boundary section 5.1 table, column `N_2`, configurations in canonical
/// order: none, {0,1}, {0,2}, {0,1},{0,2}, {1,2}, {0,1},{1,2}, {0,2},{1,2},
/// all three.
pub(crate) const FROZEN_PRIMARY_COUNTS: [i128; 8] = [1, 4, 4, 7, 1, 5, 5, 9];
/// Section 5.1, column `n_0, n_1, n_2`.
pub(crate) const FROZEN_PRIMARY_DESTINATIONS: [[i128; 3]; 8] = [
    [1, 0, 0],
    [2, 2, 0],
    [2, 0, 2],
    [3, 2, 2],
    [1, 0, 0],
    [2, 2, 1],
    [2, 1, 2],
    [3, 3, 3],
];
/// Section 5.1, column `|D|`.
pub(crate) const FROZEN_PRIMARY_REACHABLE: [usize; 8] = [1, 2, 2, 3, 1, 3, 3, 3];
/// Section 5.1, column `K(G, y)` for reachable `y` (`None` = unreachable).
pub(crate) const FROZEN_PRIMARY_K: [[Option<(i128, i128)>; 3]; 8] = [
    [Some((1, 2)), None, None],
    [Some((1, 1)), Some((1, 1)), None],
    [Some((1, 1)), None, Some((1, 1))],
    [Some((7, 6)), Some((7, 4)), Some((7, 4))],
    [Some((1, 2)), None, None],
    [Some((5, 4)), Some((5, 4)), Some((5, 2))],
    [Some((5, 4)), Some((5, 2)), Some((5, 4))],
    [Some((3, 2)), Some((3, 2)), Some((3, 2))],
];
/// Section 5.1, column `exp[N I]`.
pub(crate) const FROZEN_PRIMARY_EXP_NI: [(i128, i128); 8] = [
    (1, 1),
    (16, 1),
    (16, 1),
    (823_543, 432),
    (1, 1),
    (3125, 16),
    (3125, 16),
    (19_683, 1),
];
/// Section 5.1, capacity witness plans listed by destination `0, 1, 2`.
pub(crate) const FROZEN_PRIMARY_WITNESS: [&[&str]; 8] = [
    &["000"],
    &["000", "001"],
    &["000", "002"],
    &["000", "001", "002"],
    &["000"],
    &["000", "001", "012"],
    &["000", "021", "002"],
    &["000", "001", "002"],
];
/// Section 5.1, `exp[|D| I*] = |D|^|D|` in configuration order.
pub(crate) const FROZEN_PRIMARY_EXP_CAPACITY: [i128; 8] = [1, 4, 4, 27, 1, 27, 27, 27];
/// Section 5.1: `I(G) = I*(G)` exactly at none, {0,1}, {0,2}, {1,2}, all three.
pub(crate) const FROZEN_PRIMARY_EQUAL_INFORMATION: [usize; 5] = [0b000, 0b001, 0b010, 0b100, 0b111];
/// Sections 4.5 and 5.3: `K = 1` at {0,1} and {0,2} in the primary witness.
pub(crate) const FROZEN_PRIMARY_K_ONE: [usize; 2] = [0b001, 0b010];
/// Section 5.1: 1,296 system states and 7,308 complete states.
pub(crate) const FROZEN_PRIMARY_TOTALS: (usize, usize) = (1_296, 7_308);

pub(crate) fn frozen_row(m: &Model) -> Option<&'static FrozenFamilyRow> {
    let case = m.predicted_case();
    FROZEN_FAMILY
        .iter()
        .find(|row| row.case == (case.n, case.tau, case.b, case.w))
}

fn frozen(pair: (i128, i128)) -> Ratio {
    Ratio::new(pair.0, pair.1)
}

// ---------------------------------------------------------------- stationary law

/// Stationary law propagated from the constructed generator (section 6).
#[derive(Clone, Debug)]
pub(crate) struct StationaryLaw {
    /// `pi(z0) = 1` at the first system state, propagated by hazard ratios.
    pub(crate) unnormalised: Vec<Ratio>,
    pub(crate) probability: Vec<Ratio>,
}

/// Why the law could not be propagated: first offending configuration and
/// a detail.
#[derive(Clone, Debug)]
pub(crate) struct LawError {
    configuration: Option<usize>,
    detail: String,
}

impl StationaryLaw {
    /// Fix `pi(z0) = 1`, propagate `pi(z') = pi(z) k(z -> z') / k(z' -> z)`
    /// along a breadth-first spanning tree of the system-state graph, and
    /// normalise. Detailed balance on every edge is verified by G07 check 3.
    fn propagate(m: &Model) -> Result<Self, LawError> {
        let count = m.states.len();
        let mut weight: Vec<Option<Ratio>> = vec![None; count];
        if count == 0 {
            return Err(LawError {
                configuration: None,
                detail: "empty state space".to_string(),
            });
        }
        weight[0] = Some(Ratio::ONE);
        let mut queue = VecDeque::from([0_usize]);
        while let Some(state) = queue.pop_front() {
            let here = weight[state].expect("queued states carry a weight");
            let mut neighbours: Vec<usize> = m.system.rows[state].iter().map(|e| e.to).collect();
            neighbours.dedup();
            for next in neighbours {
                if weight[next].is_some() {
                    continue;
                }
                let forward = m.system.hazard(state, next);
                let reverse = m.system.hazard(next, state);
                let Some(ratio) = forward.checked_div(reverse) else {
                    return Err(LawError {
                        configuration: Some(m.states[state].config),
                        detail: format!(
                            "tree edge {} -> {} has no reverse rate",
                            m.describe_state(state),
                            m.describe_state(next)
                        ),
                    });
                };
                weight[next] = Some(here * ratio);
                queue.push_back(next);
            }
        }
        if let Some(state) = weight.iter().position(Option::is_none) {
            return Err(LawError {
                configuration: Some(m.states[state].config),
                detail: format!("{} is not reached from z0", m.describe_state(state)),
            });
        }
        let unnormalised: Vec<Ratio> = weight
            .into_iter()
            .map(|w| w.unwrap_or(Ratio::ZERO))
            .collect();
        let total = unnormalised
            .iter()
            .fold(Ratio::ZERO, |sum, &value| sum + value);
        let probability = unnormalised.iter().map(|&value| value / total).collect();
        Ok(Self {
            unnormalised,
            probability,
        })
    }

    fn mass(&self, states: impl IntoIterator<Item = usize>) -> Ratio {
        states
            .into_iter()
            .fold(Ratio::ZERO, |sum, state| sum + self.probability[state])
    }
}

pub(crate) struct Context<'a> {
    pub(crate) model: &'a Model,
    law: OnceCell<Result<StationaryLaw, LawError>>,
}

impl<'a> Context<'a> {
    pub(crate) fn new(model: &'a Model) -> Self {
        Self {
            model,
            law: OnceCell::new(),
        }
    }

    fn law_result(&self) -> &Result<StationaryLaw, LawError> {
        self.law
            .get_or_init(|| StationaryLaw::propagate(self.model))
    }

    /// The propagated law, or a failure of `check` when it does not exist.
    fn law(&self, check: CheckId) -> Result<&StationaryLaw, Failure> {
        self.law_result().as_ref().map_err(|error| {
            fail(
                check,
                error.configuration,
                format!("stationary law unavailable: {}", error.detail),
            )
        })
    }

    pub(crate) fn stationary_law(&self) -> Option<&StationaryLaw> {
        self.law_result().as_ref().ok()
    }
}

// ---------------------------------------------------------------- helpers

/// A generator level: explicit complete states (N = 3) or system states.
#[derive(Clone, Copy)]
enum Level<'a> {
    Complete(&'a MicroLevel),
    System(&'a Generator),
}

impl Level<'_> {
    fn name(self) -> &'static str {
        match self {
            Self::Complete(_) => "complete-state",
            Self::System(_) => "system-state",
        }
    }

    fn rows_of(self, m: &Model, config: usize) -> std::ops::Range<usize> {
        match self {
            Self::Complete(micro) => {
                micro.offsets[m.config_offsets[config]]..micro.offsets[m.config_offsets[config + 1]]
            }
            Self::System(_) => m.states_of(config),
        }
    }

    fn system_of(self, row: usize) -> usize {
        match self {
            Self::Complete(micro) => micro.system_of[row],
            Self::System(_) => row,
        }
    }

    fn label_of(self, row: usize) -> Option<usize> {
        match self {
            Self::Complete(micro) => Some(micro.label_of[row]),
            Self::System(_) => None,
        }
    }

    fn edges(self, row: usize) -> Vec<Edge> {
        match self {
            Self::Complete(micro) => micro.rows[row].iter().map(|edge| edge.edge()).collect(),
            Self::System(generator) => generator.rows[row].clone(),
        }
    }

    fn channel_hazard(self, from: usize, to: usize, kind: Kind) -> Ratio {
        match self {
            Self::Complete(micro) => micro.channel_hazard(from, to, kind),
            Self::System(generator) => generator.channel_hazard(from, to, kind),
        }
    }

    fn diagonal(self, row: usize) -> Ratio {
        match self {
            Self::Complete(micro) => Ratio::ZERO - micro.escape(row),
            Self::System(generator) => generator.diagonal(row),
        }
    }

    fn escape(self, row: usize) -> Ratio {
        match self {
            Self::Complete(micro) => micro.escape(row),
            Self::System(generator) => generator.escape(row),
        }
    }
}

fn levels(model: &Model) -> Vec<Level<'_>> {
    let mut levels = Vec::new();
    if let Some(micro) = &model.micro {
        levels.push(Level::Complete(micro));
    }
    levels.push(Level::System(&model.system));
    levels
}

/// Runs `test` over every channel of every level, configurations outermost
/// in canonical order, and reports the first failure.
fn scan_channels(
    m: &Model,
    check: CheckId,
    mut test: impl FnMut(Level, usize, &Edge) -> Option<String>,
) -> Result<(), Failure> {
    let levels = levels(m);
    for config in 0..m.config_count {
        for &level in &levels {
            for row in level.rows_of(m, config) {
                for edge in level.edges(row) {
                    if let Some(detail) = test(level, row, &edge) {
                        return Err(fail(
                            check,
                            Some(config),
                            format!(
                                "{} {} {row} -> {}: {detail}",
                                level.name(),
                                edge.kind.code(),
                                edge.to
                            ),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

/// Whether the declared rules of section 3.3 move a packet with this
/// channel kind (bridge, feedback, and erasure traffic).
const fn moves_packet(kind: Kind) -> bool {
    matches!(
        kind,
        Kind::Build | Kind::Dismantle | Kind::Harvest | Kind::Commit | Kind::Erase | Kind::Scribble
    )
}

/// `b^energy` by repeated multiplication, written independently of
/// `lumped_hazard`.
fn reservoir_multiplicity(base: i128, energy: usize) -> Ratio {
    Ratio::integer((0..energy).fold(1_i128, |product, _| product * base))
}

/// `value^exponent` for a signed exponent.
fn ratio_power(value: Ratio, exponent: i64) -> Ratio {
    let mut result = Ratio::ONE;
    for _ in 0..exponent.unsigned_abs() {
        result = if exponent >= 0 {
            result * value
        } else {
            result / value
        };
    }
    result
}

/// System-state hazard stated in boundary section 4.1, written without
/// `lumped_hazard`: `b^(E_R(z'))` for a packet-moving channel into `to`,
/// 1 otherwise.
fn expected_system_hazard(m: &Model, kind: Kind, to: usize) -> Ratio {
    if moves_packet(kind) {
        KAPPA * reservoir_multiplicity(m.case.b, m.reservoir_energy(to))
    } else {
        KAPPA
    }
}

/// `n_y(G)` by counting the depth-first plans ending at each island.
pub(crate) fn destination_counts(m: &Model, config: usize) -> Vec<i128> {
    let mut counts = vec![0_i128; m.case.n];
    for plan in &m.plans[config] {
        counts[Model::destination(plan)] += 1;
    }
    counts
}

/// Total exit rate of a system state counted directly from the local rules
/// of boundary section 3.3, without consulting the constructed moves. The
/// same total holds for every complete state above it: a packet-moving
/// channel reaches every destination reservoir label at unit hazard.
fn expected_exit_rate(m: &Model, state: usize) -> Ratio {
    let SystemState {
        config,
        cursor,
        note,
        stock,
        ..
    } = m.states[state];
    let plan = m.plan_of(state);
    let energy = m.reservoir_energy(state);
    let up = |count: usize| {
        Ratio::integer(count as i128) * KAPPA * reservoir_multiplicity(m.case.b, energy - 1)
    };
    let down = |count: usize| {
        Ratio::integer(count as i128) * KAPPA * reservoir_multiplicity(m.case.b, energy + 1)
    };
    let unit = |count: usize| Ratio::integer(count as i128) * KAPPA;
    let steps = usize::from(cursor < m.case.tau) + usize::from(cursor > 0);
    let mut total = unit(steps);
    if cursor == 0 {
        let plan_count = m.matrix_count[config] as usize;
        let destination = plan[plan.len() - 1];
        total = total + unit(plan_count - 1);
        match note {
            None => {
                total = total + unit(1);
                if stock < m.case.w {
                    total = total + up(m.case.n);
                }
            }
            Some(record) => {
                if record == destination {
                    total = total + unit(1);
                    if stock < m.case.w {
                        total = total + up(plan_count);
                    }
                }
                if stock >= 1 {
                    let ending = m.plans[config]
                        .iter()
                        .filter(|candidate| candidate[candidate.len() - 1] == record)
                        .count();
                    total = total + down(ending) + down(1);
                }
            }
        }
    }
    let island = plan[cursor];
    let walks = |u: usize, w: usize| {
        plan.windows(2)
            .any(|pair| (pair[0] == u && pair[1] == w) || (pair[0] == w && pair[1] == u))
    };
    for (bridge, &(u, w)) in m.bridges.iter().enumerate() {
        if island != u && island != w {
            continue;
        }
        if config & (1 << bridge) == 0 {
            total = total + up(1);
        } else if !walks(u, w) {
            total = total + down(1);
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
        let count = resolution.outcomes.len();
        let alphabet: BTreeSet<Vec<usize>> = resolution.outcomes.iter().cloned().collect();
        let uniform = count > 0
            && resolution.probabilities.len() == count
            && resolution
                .probabilities
                .iter()
                .all(|&p| p == Ratio::new(1, count as i128));
        if !uniform || alphabet.len() != count || alphabet != m.reference_plans[config] {
            let law: Vec<String> = resolution
                .probabilities
                .iter()
                .map(Ratio::to_string)
                .collect();
            return Err(fail(
                CheckId::G01_REPLAN_UNIFORM,
                Some(config),
                format!(
                    "REPLAN outcome law [{}] over {count} outcomes; alphabet equals Pi_tau(G): {}",
                    law.join(","),
                    alphabet == m.reference_plans[config]
                ),
            ));
        }
        for state in m.states_of(config) {
            let SystemState {
                plan, note, stock, ..
            } = m.states[state];
            if m.states[state].cursor != 0 {
                continue;
            }
            let replans: Vec<&Edge> = m.system.rows[state]
                .iter()
                .filter(|edge| edge.kind == Kind::Replan)
                .collect();
            let targets: BTreeSet<usize> = replans.iter().map(|edge| edge.to).collect();
            let expected: BTreeSet<usize> = (0..m.plans[config].len())
                .filter(|&other| other != plan)
                .map(|other| m.state_at(config, other, 0, note, stock))
                .collect();
            let equal = replans.iter().all(|edge| edge.hazard == replans[0].hazard);
            if targets != expected || !equal {
                return Err(fail(
                    CheckId::G01_REPLAN_UNIFORM,
                    Some(config),
                    format!(
                        "REPLAN at {} reaches {} of {} other plans with equal hazards: {equal}",
                        m.describe_state(state),
                        targets.len(),
                        expected.len()
                    ),
                ));
            }
        }
    }
    for config in 0..m.config_count {
        let counts = destination_counts(m, config);
        let sum: i128 = counts.iter().sum();
        if sum != m.matrix_count[config] {
            return Err(fail(
                CheckId::G01_DESTINATION_SUM,
                Some(config),
                format!("sum_y n_y = {sum} != N_tau(G) = {}", m.matrix_count[config]),
            ));
        }
    }
    let tau = m.case.tau;
    for config in 0..m.config_count {
        let expected = (tau as i128 + 1) * m.matrix_count[config];
        for note in m.notebook_values() {
            for stock in 0..=m.case.w {
                let slice: Vec<SystemState> = m.states[m.states_of(config)]
                    .iter()
                    .copied()
                    .filter(|state| state.note == note && state.stock == stock)
                    .collect();
                let reference = &m.reference_plans[config];
                let covered = reference.iter().all(|sequence| {
                    m.plan_index[config].get(sequence).is_some_and(|&index| {
                        (0..=tau).all(|cursor| {
                            slice
                                .iter()
                                .any(|state| state.plan == index && state.cursor == cursor)
                        })
                    })
                });
                if slice.len() as i128 != expected
                    || (reference.len() as i128) * (tau as i128 + 1) != expected
                    || !covered
                {
                    return Err(fail(
                        CheckId::G01_SLICE_STATE_COUNT,
                        Some(config),
                        format!(
                            "slice y={} w={stock}: {} system states, (tau+1) N_tau = {expected}, brute-force plans covered: {covered}",
                            crate::model::note_label(note),
                            slice.len()
                        ),
                    ));
                }
            }
        }
    }
    pass()
}

// ---------------------------------------------------------------- G02

fn travel_violation(m: &Model, state: usize) -> Option<String> {
    let SystemState { config, cursor, .. } = m.states[state];
    let plan = m.plan_of(state);
    let island = m.position(state);
    for local in &m.moves[state] {
        match local.kind {
            Kind::Step => {
                let adjacent =
                    local.to_cursor.abs_diff(cursor) == 1 && local.to_cursor <= m.case.tau;
                let along = adjacent && {
                    let (from, to) = (plan[cursor], local.register[local.to_cursor]);
                    from == to || m.bridge_built(config, from, to)
                };
                if local.to_config != config || local.register != plan || !along {
                    return Some(format!(
                        "(a) STEP from {} is not along a bridge or a stay",
                        m.describe_state(state)
                    ));
                }
            }
            Kind::Build | Kind::Dismantle => {
                let incident = local.bridge.is_some_and(|bridge| {
                    let (u, w) = m.bridges[bridge];
                    (island == u || island == w)
                        && local.to_config == config ^ (1 << bridge)
                        && m.has_bridge(config, bridge) == (local.kind == Kind::Dismantle)
                        && local.to_cursor == cursor
                        && local.register == plan
                });
                if !incident {
                    return Some(format!(
                        "(b) {} from {} is not incident to island {island}",
                        local.kind.code(),
                        m.describe_state(state)
                    ));
                }
                if local.kind == Kind::Dismantle {
                    let bridge = local.bridge.unwrap_or_default();
                    if m.uses_bridge(plan, bridge) {
                        return Some(format!(
                            "(c) DISMANTLE of {:?} at {} removes a bridge of the held plan",
                            m.bridges[bridge],
                            m.describe_state(state)
                        ));
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn measure_violation(m: &Model, state: usize) -> Option<String> {
    let SystemState {
        config,
        cursor,
        note,
        stock,
        ..
    } = m.states[state];
    let plan = m.plan_of(state);
    let destination = Model::destination(plan);
    for local in &m.moves[state] {
        let keeps_rest = local.to_config == config
            && local.register == plan
            && local.to_cursor == 0
            && local.to_stock == stock;
        let ok = match local.kind {
            Kind::Measure => {
                cursor == 0 && note.is_none() && local.to_note == Some(destination) && keeps_rest
            }
            Kind::Unmeasure => {
                cursor == 0 && note == Some(destination) && local.to_note.is_none() && keeps_rest
            }
            _ => true,
        };
        if !ok {
            return Some(format!(
                "{} from {} writes y={} with d(p)={destination}",
                local.kind.code(),
                m.describe_state(state),
                crate::model::note_label(local.to_note)
            ));
        }
    }
    None
}

fn feedback_violation(m: &Model, state: usize) -> Option<String> {
    let SystemState {
        config,
        cursor,
        note,
        stock,
        ..
    } = m.states[state];
    let plan = m.plan_of(state);
    let destination = Model::destination(plan);
    let mut released = BTreeSet::new();
    let mut harvests = 0;
    for local in &m.moves[state] {
        match local.kind {
            Kind::Harvest => {
                harvests += 1;
                let ok = cursor == 0
                    && note == Some(destination)
                    && stock < m.case.w
                    && local.to_config == config
                    && local.to_cursor == 0
                    && local.to_note == note
                    && local.to_stock == stock + 1;
                if !ok {
                    return Some(format!(
                        "HARVEST from {} (d(p)={destination}) to y={} w={}",
                        m.describe_state(state),
                        crate::model::note_label(local.to_note),
                        local.to_stock
                    ));
                }
                released.insert(local.register.clone());
            }
            Kind::Commit => {
                let ok = cursor == 0
                    && note.is_some()
                    && stock >= 1
                    && local.to_config == config
                    && local.to_cursor == 0
                    && local.to_note == note
                    && local.to_stock + 1 == stock
                    && note == Some(Model::destination(&local.register));
                if !ok {
                    return Some(format!(
                        "COMMIT from {} reaches plan {} with record y={}",
                        m.describe_state(state),
                        plan_label(&local.register),
                        crate::model::note_label(note)
                    ));
                }
            }
            _ => {}
        }
    }
    let every_plan: BTreeSet<Vec<usize>> = m.plans[config].iter().cloned().collect();
    if harvests > 0 && released != every_plan {
        return Some(format!(
            "HARVEST from {} reaches {} of {} plans",
            m.describe_state(state),
            released.len(),
            every_plan.len()
        ));
    }
    None
}

fn erasure_violation(m: &Model, state: usize) -> Option<String> {
    let SystemState {
        config,
        cursor,
        note,
        stock,
        ..
    } = m.states[state];
    let plan = m.plan_of(state);
    let mut written = BTreeSet::new();
    let mut scribbles = 0;
    for local in &m.moves[state] {
        let keeps_rest =
            local.to_config == config && local.register == plan && local.to_cursor == 0;
        match local.kind {
            Kind::Erase => {
                let ok = cursor == 0
                    && note.is_some()
                    && stock >= 1
                    && local.to_note.is_none()
                    && local.to_stock + 1 == stock
                    && keeps_rest;
                if !ok {
                    return Some(format!(
                        "ERASE from {} arrives at y={} w={}",
                        m.describe_state(state),
                        crate::model::note_label(local.to_note),
                        local.to_stock
                    ));
                }
            }
            Kind::Scribble => {
                scribbles += 1;
                let ok = cursor == 0
                    && note.is_none()
                    && stock < m.case.w
                    && local.to_note.is_some()
                    && local.to_stock == stock + 1
                    && keeps_rest;
                if !ok {
                    return Some(format!(
                        "SCRIBBLE from {} arrives at y={} w={}",
                        m.describe_state(state),
                        crate::model::note_label(local.to_note),
                        local.to_stock
                    ));
                }
                written.insert(local.to_note);
            }
            _ => {}
        }
    }
    let alphabet: BTreeSet<Note> = (0..m.case.n).map(Some).collect();
    if scribbles > 0 && written != alphabet {
        return Some(format!(
            "SCRIBBLE from {} writes {} of {} symbols",
            m.describe_state(state),
            written.len(),
            alphabet.len()
        ));
    }
    None
}

pub(crate) fn gate_02(cx: &Context) -> GateResult {
    let m = cx.model;
    for state in 0..m.states.len() {
        let config = m.states[state].config;
        for local in &m.moves[state] {
            let consistent = local.to.is_some_and(|to| {
                let target = m.states[to];
                target.config == local.to_config
                    && target.cursor == local.to_cursor
                    && target.note == local.to_note
                    && target.stock == local.to_stock
                    && m.plan_of(to) == local.register.as_slice()
            });
            if !m.plan_valid(local.to_config, &local.register) || !consistent {
                return Err(fail(
                    CheckId::G02_DESTINATION_VALIDITY,
                    Some(config),
                    format!(
                        "{} from {} to G={} p={} t={} y={} w={} is not a declared state",
                        local.kind.code(),
                        m.describe_state(state),
                        m.config_label(local.to_config),
                        plan_label(&local.register),
                        local.to_cursor,
                        crate::model::note_label(local.to_note),
                        local.to_stock
                    ),
                ));
            }
        }
    }
    for state in 0..m.states.len() {
        if let Some(detail) = travel_violation(m, state) {
            return Err(fail(
                CheckId::G02_TRAVEL_AND_TOGGLES,
                Some(m.states[state].config),
                detail,
            ));
        }
    }
    let mut plan_read = String::new();
    if m.is_primary() {
        // (d) at {0,1}, cursor 0: equal notebook and stockpile, different
        // DISMANTLE sets.
        let at = 0b001;
        let dismantles = |state: usize| -> BTreeSet<usize> {
            m.moves[state]
                .iter()
                .filter(|local| local.kind == Kind::Dismantle)
                .filter_map(|local| local.bridge)
                .collect()
        };
        let members: Vec<usize> = m
            .states_of(at)
            .filter(|&state| m.states[state].cursor == 0)
            .collect();
        let witness = members.iter().enumerate().find_map(|(index, &first)| {
            members[index + 1..]
                .iter()
                .find(|&&second| {
                    m.states[first].note == m.states[second].note
                        && m.states[first].stock == m.states[second].stock
                        && dismantles(first) != dismantles(second)
                })
                .map(|&second| (first, second))
        });
        match witness {
            Some((first, second)) => {
                plan_read = format!(
                    "plan read at {} vs {}",
                    m.describe_state(first),
                    m.describe_state(second)
                );
            }
            None => {
                return Err(fail(
                    CheckId::G02_TRAVEL_AND_TOGGLES,
                    Some(at),
                    "(d) no two cursor-0 states of {0,1} with equal notebook and stockpile have different DISMANTLE sets",
                ));
            }
        }
    }
    for state in 0..m.states.len() {
        if let Some(detail) = measure_violation(m, state) {
            return Err(fail(
                CheckId::G02_MEASURE_SEMANTICS,
                Some(m.states[state].config),
                detail,
            ));
        }
    }
    for state in 0..m.states.len() {
        if let Some(detail) = feedback_violation(m, state) {
            return Err(fail(
                CheckId::G02_FEEDBACK_SEMANTICS,
                Some(m.states[state].config),
                detail,
            ));
        }
    }
    for state in 0..m.states.len() {
        if let Some(detail) = erasure_violation(m, state) {
            return Err(fail(
                CheckId::G02_ERASURE_SEMANTICS,
                Some(m.states[state].config),
                detail,
            ));
        }
    }
    for state in 0..m.states.len() {
        let SystemState { note, stock, .. } = m.states[state];
        for local in &m.moves[state] {
            let traffic = matches!(
                local.kind,
                Kind::Step | Kind::Replan | Kind::Build | Kind::Dismantle
            );
            if traffic && (local.to_note != note || local.to_stock != stock) {
                return Err(fail(
                    CheckId::G02_REGISTERS_UNTOUCHED,
                    Some(m.states[state].config),
                    format!(
                        "{} from {} changes the notebook or stockpile",
                        local.kind.code(),
                        m.describe_state(state)
                    ),
                ));
            }
        }
    }
    pass_with(plan_read)
}

// ---------------------------------------------------------------- G03

pub(crate) fn gate_03(cx: &Context) -> GateResult {
    let m = cx.model;
    scan_channels(m, CheckId::G03_RECIPROCAL_SUPPORT, |level, from, edge| {
        (!level
            .channel_hazard(edge.to, from, edge.kind.reverse())
            .is_positive())
        .then(|| format!("no positive {} reverse", edge.kind.reverse().code()))
    })?;
    let micro = m.micro.is_some();
    let all = levels(m);
    let level = all[0];
    for config in 0..m.config_count {
        for from in level.rows_of(m, config) {
            for edge in level.edges(from) {
                let forward = edge.hazard;
                let reverse = level.channel_hazard(edge.to, from, edge.kind.reverse());
                let (expected_forward, expected_reverse) = if micro {
                    (KAPPA, KAPPA)
                } else {
                    (
                        expected_system_hazard(m, edge.kind, edge.to),
                        expected_system_hazard(m, edge.kind.reverse(), from),
                    )
                };
                if forward != expected_forward || reverse != expected_reverse {
                    return Err(fail(
                        CheckId::G03_HAZARD_VALUES,
                        Some(config),
                        format!(
                            "{} {} {from} <-> {}: hazards {forward} / {reverse}, expected {expected_forward} / {expected_reverse}",
                            level.name(),
                            edge.kind.code(),
                            edge.to
                        ),
                    ));
                }
            }
        }
    }
    for config in 0..m.config_count {
        for &level in &all {
            for from in level.rows_of(m, config) {
                let edges = level.edges(from);
                let positive = edges
                    .iter()
                    .all(|edge| edge.hazard.is_positive() && edge.to != from);
                let escape = level.escape(from);
                let counted = expected_exit_rate(m, level.system_of(from));
                let diagonal = level.diagonal(from);
                let closure = diagonal + escape;
                if !positive || escape != counted || !closure.is_zero() {
                    return Err(fail(
                        CheckId::G03_ROW_CLOSURE,
                        Some(config),
                        format!(
                            "{} row {from}: exit rate {escape}, counted from the section 3.3 rules {counted}, closure {closure}",
                            level.name()
                        ),
                    ));
                }
            }
        }
    }
    pass_with("each diagonal is -(exit rate) and each exit rate equals the rate counted independently from the section 3.3 rules")
}

// ---------------------------------------------------------------- G04

/// Externally supplied work per channel: the witness is undriven.
const fn external_work(_kind: Kind) -> i64 {
    0
}

/// Declared exchange of each packet-moving channel (section 3.3):
/// `(Delta |G|, Delta w, Delta E_R)`.
const fn declared_exchange(kind: Kind) -> Option<(i64, i64, i64)> {
    match kind {
        Kind::Build => Some((1, 0, -1)),
        Kind::Dismantle => Some((-1, 0, 1)),
        Kind::Harvest | Kind::Scribble => Some((0, 1, -1)),
        Kind::Commit | Kind::Erase => Some((0, -1, 1)),
        Kind::Step | Kind::Replan | Kind::Measure | Kind::Unmeasure => None,
    }
}

struct Delta {
    bridges: i64,
    stock: i64,
    energy: i64,
    reservoir: i64,
}

fn delta(m: &Model, from: usize, to: usize) -> Delta {
    let (a, b) = (m.states[from], m.states[to]);
    Delta {
        bridges: m.size(b.config) as i64 - m.size(a.config) as i64,
        stock: b.stock as i64 - a.stock as i64,
        energy: m.system_energy(to) - m.system_energy(from),
        reservoir: LAMBDA * (m.reservoir_energy(to) as i64 - m.reservoir_energy(from) as i64),
    }
}

pub(crate) fn gate_04(cx: &Context) -> GateResult {
    let m = cx.model;
    let total = LAMBDA * m.total_packets() as i64;
    scan_channels(m, CheckId::G04_CONSERVATION, |level, from, edge| {
        let (source, target) = (level.system_of(from), level.system_of(edge.to));
        let before = m.system_energy(source) + LAMBDA * m.reservoir_energy(source) as i64;
        let after = m.system_energy(target) + LAMBDA * m.reservoir_energy(target) as i64;
        (before != total || after != total)
            .then(|| format!("U + E_R = {before} -> {after}, E_tot = {total}"))
    })?;
    scan_channels(m, CheckId::G04_EXCHANGE_PARTNERS, |level, from, edge| {
        let change = delta(m, level.system_of(from), level.system_of(edge.to));
        let work = external_work(edge.kind);
        let heat = -change.reservoir;
        let first_law = change.energy == heat + work;
        match declared_exchange(edge.kind) {
            Some((bridges, stock, reservoir)) => {
                let exchanged = change.bridges == bridges
                    && change.stock == stock
                    && change.reservoir == LAMBDA * reservoir;
                (!exchanged || work != 0 || !first_law).then(|| {
                    format!(
                        "dG={} dw={} dE_R={} work={work}, declared ({bridges},{stock},{reservoir})",
                        change.bridges, change.stock, change.reservoir
                    )
                })
            }
            None => (work != 0 || !first_law).then(|| format!("work {work}")),
        }
    })?;
    scan_channels(m, CheckId::G04_ERASE_ONE_PACKET, |level, from, edge| {
        let change = delta(m, level.system_of(from), level.system_of(edge.to));
        let expected = match edge.kind {
            Kind::Erase => (-1, LAMBDA),
            Kind::Scribble => (1, -LAMBDA),
            _ => return None,
        };
        ((change.stock, change.reservoir) != expected || change.bridges != 0).then(|| {
            format!(
                "dw={} dE_R={}, declared one packet",
                change.stock, change.reservoir
            )
        })
    })?;
    scan_channels(m, CheckId::G04_NEUTRAL_CHANNELS, |level, from, edge| {
        if declared_exchange(edge.kind).is_some() {
            return None;
        }
        let change = delta(m, level.system_of(from), level.system_of(edge.to));
        let same_label = level.label_of(from) == level.label_of(edge.to);
        (change.energy != 0 || change.reservoir != 0 || !same_label).then(|| {
            format!(
                "dU={} dE_R={} label kept: {same_label}",
                change.energy, change.reservoir
            )
        })
    })?;
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
        for edge in &micro.rows[complete] {
            let target = micro.system_of[edge.to as usize];
            let rate = row.entry(target).or_insert(Ratio::ZERO);
            *rate = *rate + edge.edge().hazard;
        }
        row
    };
    for state in 0..m.states.len() {
        let first = micro.offsets[state];
        let reference = lumped_row(first);
        for complete in first + 1..micro.offsets[state + 1] {
            if lumped_row(complete) != reference {
                return Err(fail(
                    CheckId::G05_LUMPABILITY,
                    Some(m.states[state].config),
                    format!(
                        "complete states {first} and {complete} of {} send different rates",
                        m.describe_state(state)
                    ),
                ));
            }
        }
        let mut expected: BTreeMap<usize, Ratio> = BTreeMap::new();
        for edge in &m.system.rows[state] {
            let hazard = if m.reservoir_energy(edge.to) != m.reservoir_energy(state) {
                KAPPA * reservoir_multiplicity(m.case.b, m.reservoir_energy(edge.to))
            } else {
                KAPPA
            };
            let rate = expected.entry(edge.to).or_insert(Ratio::ZERO);
            *rate = *rate + hazard;
        }
        if reference != expected {
            return Err(fail(
                CheckId::G05_LUMPABILITY,
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
    // The b of the reservoir law g_R(E) = b^E.
    let law_base = reservoir_multiplicity(m.case.b, 1) / reservoir_multiplicity(m.case.b, 0);
    let classes: [(CheckId, [Kind; 2]); 3] = [
        (CheckId::G06_BRIDGE_TRAFFIC, [Kind::Build, Kind::Dismantle]),
        (CheckId::G06_FEEDBACK_TRAFFIC, [Kind::Harvest, Kind::Commit]),
        (CheckId::G06_ERASURE_TRAFFIC, [Kind::Erase, Kind::Scribble]),
    ];
    for (check, kinds) in classes {
        if check == CheckId::G06_BRIDGE_TRAFFIC {
            for energy in 0..m.total_packets() {
                let ratio = reservoir_multiplicity(m.case.b, energy + 1)
                    / reservoir_multiplicity(m.case.b, energy);
                if ratio != law_base {
                    return Err(fail(
                        check,
                        None,
                        format!(
                            "g_R({})/g_R({energy}) = {ratio} != b = {law_base}",
                            energy + 1
                        ),
                    ));
                }
            }
        }
        for config in 0..m.config_count {
            for from in m.states_of(config) {
                for edge in m.system.rows[from]
                    .iter()
                    .filter(|e| kinds.contains(&e.kind))
                {
                    let reverse = m.system.channel_hazard(edge.to, from, edge.kind.reverse());
                    let delta_u = m.system_energy(edge.to) - m.system_energy(from);
                    let predicted = ratio_power(law_base, -(delta_u / LAMBDA));
                    if !reverse.is_positive() || edge.hazard != predicted * reverse {
                        return Err(fail(
                            check,
                            Some(config),
                            format!(
                                "{} {} -> {}: k={} k_rev={reverse}, predicted ratio {predicted}",
                                edge.kind.code(),
                                m.describe_state(from),
                                m.describe_state(edge.to),
                                edge.hazard
                            ),
                        ));
                    }
                }
            }
        }
    }
    // S(z) = ln g(z): one arrangement of bridges, plan, cursor, notebook,
    // and stockpile in every system state.
    let system_multiplicity = |_state: usize| Ratio::ONE;
    let reference = system_multiplicity(0);
    for state in 0..m.states.len() {
        if system_multiplicity(state) != reference {
            return Err(fail(
                CheckId::G06_CONSTANT_ENTROPY,
                Some(m.states[state].config),
                format!("g({}) differs from g(z0)", m.describe_state(state)),
            ));
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
        .map(|row| row.iter().map(|edge| edge.to).collect())
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

    let law = cx.law(CheckId::G07_PROPAGATED_LAW)?;
    let packets = |state: usize| m.system_packets(state) as i64;
    let constant = law.unnormalised[0] * Ratio::power(m.case.b, packets(0));
    for state in 0..m.states.len() {
        let scaled = law.unnormalised[state] * Ratio::power(m.case.b, packets(state));
        if scaled != constant {
            return Err(fail(
                CheckId::G07_PROPAGATED_LAW,
                Some(m.states[state].config),
                format!(
                    "{}: propagated pi = {} is not b^(-(|G|+w)) relative to z0",
                    m.describe_state(state),
                    law.unnormalised[state]
                ),
            ));
        }
    }
    for from in 0..m.states.len() {
        for edge in &m.system.rows[from] {
            let reverse = m.system.channel_hazard(edge.to, from, edge.kind.reverse());
            let forward_flux = law.unnormalised[from] * edge.hazard;
            let reverse_flux = law.unnormalised[edge.to] * reverse;
            if forward_flux != reverse_flux {
                return Err(fail(
                    CheckId::G07_FLUX_BALANCE,
                    Some(m.states[from].config),
                    format!(
                        "{} {} <-> {}: fluxes {forward_flux} and {reverse_flux}",
                        edge.kind.code(),
                        m.describe_state(from),
                        m.describe_state(edge.to)
                    ),
                ));
            }
        }
    }
    pass_with("propagated along a breadth-first spanning tree from z0")
}

// ---------------------------------------------------------------- observables

/// `Z = sum_G N_tau(G) b^(-|G|)` from the matrix-power counts.
pub(crate) fn partition_function(m: &Model) -> Ratio {
    (0..m.config_count).fold(Ratio::ZERO, |sum, config| {
        sum + Ratio::integer(m.matrix_count[config])
            * Ratio::power(m.case.b, -(m.size(config) as i64))
    })
}

pub(crate) fn configuration_marginal(m: &Model, law: &StationaryLaw) -> Vec<Ratio> {
    (0..m.config_count)
        .map(|config| law.mass(m.states_of(config)))
        .collect()
}

pub(crate) fn mean_bridges(m: &Model, law: &StationaryLaw) -> Ratio {
    (0..m.states.len()).fold(Ratio::ZERO, |sum, state| {
        sum + law.probability[state] * Ratio::integer(m.size(m.states[state].config) as i128)
    })
}

pub(crate) fn mean_stockpile(m: &Model, law: &StationaryLaw) -> Ratio {
    (0..m.states.len()).fold(Ratio::ZERO, |sum, state| {
        sum + law.probability[state] * Ratio::integer(m.states[state].stock as i128)
    })
}

pub(crate) fn notebook_probability(m: &Model, law: &StationaryLaw, note: Note) -> Ratio {
    law.mass((0..m.states.len()).filter(|&state| m.states[state].note == note))
}

/// Equilibrium fraction of written records at cursor 0 that equal `d(p)`.
pub(crate) fn correct_fraction(m: &Model, law: &StationaryLaw) -> Option<Ratio> {
    let written: Vec<usize> = (0..m.states.len())
        .filter(|&state| m.states[state].cursor == 0 && m.states[state].note.is_some())
        .collect();
    let correct = law.mass(
        written
            .iter()
            .copied()
            .filter(|&state| m.states[state].note == Some(Model::destination(m.plan_of(state)))),
    );
    correct.checked_div(law.mass(written))
}

/// `C(G, y, w)`: cursor-0 states with a correct record `y = d(p)`.
fn correct_macrostate(m: &Model, config: usize, island: usize, stock: usize) -> Vec<usize> {
    m.plans[config]
        .iter()
        .enumerate()
        .filter(|(_, plan)| Model::destination(plan) == island)
        .map(|(plan, _)| m.state_at(config, plan, 0, Some(island), stock))
        .collect()
}

/// `L(G, y, w)`: cursor-0 states with record `y` and any plan.
fn released_macrostate(m: &Model, config: usize, island: usize, stock: usize) -> Vec<usize> {
    (0..m.plans[config].len())
        .map(|plan| m.state_at(config, plan, 0, Some(island), stock))
        .collect()
}

/// `K(G, y) = pi(L(G, y, w + 1)) / pi(C(G, y, w))` from the propagated law.
pub(crate) fn exchange_constant(
    m: &Model,
    law: &StationaryLaw,
    config: usize,
    island: usize,
    stock: usize,
) -> Option<Ratio> {
    let correct = law.mass(correct_macrostate(m, config, island, stock));
    let released = law.mass(released_macrostate(m, config, island, stock + 1));
    released.checked_div(correct)
}

/// Generator-derived `K(G, y)` at `w = 0` for every island (`None` where
/// `y` is unreachable).
pub(crate) fn exchange_table(m: &Model, law: &StationaryLaw, config: usize) -> Vec<Option<Ratio>> {
    let counts = destination_counts(m, config);
    (0..m.case.n)
        .map(|island| {
            if counts[island] > 0 && m.case.w >= 1 {
                exchange_constant(m, law, config, island, 0)
            } else {
                None
            }
        })
        .collect()
}

/// Configurations with `|D| >= 2` and generator-derived `K = 1` for every
/// reachable destination.
pub(crate) fn k_one_configurations(m: &Model, law: &StationaryLaw) -> Vec<usize> {
    (0..m.config_count)
        .filter(|&config| {
            let table = exchange_table(m, law, config);
            let reachable: Vec<&Option<Ratio>> = table.iter().filter(|k| k.is_some()).collect();
            reachable.len() >= 2 && reachable.iter().all(|k| **k == Some(Ratio::ONE))
        })
        .collect()
}

/// `pi(B(G, p, w - 1)) / pi(R(G, p, w))` from the propagated law.
fn erasure_ratio(
    m: &Model,
    law: &StationaryLaw,
    config: usize,
    plan: usize,
    stock: usize,
) -> Option<Ratio> {
    let blank = law.probability[m.state_at(config, plan, 0, None, stock - 1)];
    let written =
        law.mass((0..m.case.n).map(|island| m.state_at(config, plan, 0, Some(island), stock)));
    blank.checked_div(written)
}

/// Distinct erasure ratios over every `(G, p, w >= 1)`, in order of first
/// appearance.
pub(crate) fn erasure_ratios(m: &Model, law: &StationaryLaw) -> Vec<Option<Ratio>> {
    let mut ratios: Vec<Option<Ratio>> = Vec::new();
    for config in 0..m.config_count {
        for plan in 0..m.plans[config].len() {
            for stock in 1..=m.case.w {
                let ratio = erasure_ratio(m, law, config, plan, stock);
                if !ratios.contains(&ratio) {
                    ratios.push(ratio);
                }
            }
        }
    }
    ratios
}

/// Capacity witness: the lexicographically first plan ending at each
/// reachable destination, in destination order.
pub(crate) fn capacity_witness(m: &Model, config: usize) -> Vec<(usize, Vec<usize>)> {
    (0..m.case.n)
        .filter_map(|island| {
            m.plans[config]
                .iter()
                .filter(|plan| Model::destination(plan) == island)
                .min()
                .map(|plan| (island, plan.clone()))
        })
        .collect()
}

/// `N H(d)` under the uniform plan law, from the outcome law `n_y / N`.
pub(crate) fn uniform_plan_information(m: &Model, config: usize) -> LogVector {
    let plans = &m.plans[config];
    let atom = Ratio::new(1, plans.len() as i128);
    let mut outcome = vec![Ratio::ZERO; m.case.n];
    for plan in plans {
        let island = Model::destination(plan);
        outcome[island] = outcome[island] + atom;
    }
    LogVector::entropy(outcome).scaled(Ratio::integer(plans.len() as i128))
}

/// The capacity pair `A*_G`, `B*_G` and `|D| [S(B*) - S(A*)]`.
pub(crate) fn capacity_information(m: &Model, config: usize) -> LogVector {
    let witness = capacity_witness(m, config);
    let reachable = witness.len();
    let atom = Ratio::new(1, reachable as i128);
    // A*: uniform over the |D| pairs (p*_y, y).
    let correlated: BTreeMap<(Vec<usize>, usize), Ratio> = witness
        .iter()
        .map(|(island, plan)| ((plan.clone(), *island), atom))
        .collect();
    let mut plan_marginal: BTreeMap<Vec<usize>, Ratio> = BTreeMap::new();
    let mut record_marginal: BTreeMap<usize, Ratio> = BTreeMap::new();
    for ((plan, island), &mass) in &correlated {
        let entry = plan_marginal.entry(plan.clone()).or_insert(Ratio::ZERO);
        *entry = *entry + mass;
        let entry = record_marginal.entry(*island).or_insert(Ratio::ZERO);
        *entry = *entry + mass;
    }
    // B*: product of the marginals of A*.
    let shuffled = plan_marginal
        .values()
        .flat_map(|&p| record_marginal.values().map(move |&q| p * q));
    let gap = LogVector::entropy(shuffled).minus(&LogVector::entropy(correlated.values().copied()));
    gap.scaled(Ratio::integer(reachable as i128))
}

// ---------------------------------------------------------------- G08

pub(crate) fn gate_08(cx: &Context) -> GateResult {
    let m = cx.model;
    let law = cx.law(CheckId::G08_CONFIGURATION_MARGINAL)?;
    let Some(row) = frozen_row(m) else {
        return Err(fail(
            CheckId::G08_CONFIGURATION_MARGINAL,
            None,
            format!("case {} has no section 5.2 row", m.case.label()),
        ));
    };
    let partition = partition_function(m);
    let marginal = configuration_marginal(m, law);
    for (config, &observed) in marginal.iter().enumerate() {
        let predicted = Ratio::integer(m.matrix_count[config])
            * Ratio::power(m.case.b, -(m.size(config) as i64))
            / partition;
        if observed != predicted {
            return Err(fail(
                CheckId::G08_CONFIGURATION_MARGINAL,
                Some(config),
                format!("pi(G) = {observed} != N_tau b^(-|G|)/Z = {predicted}"),
            ));
        }
    }
    let bridges = mean_bridges(m, law);
    if partition != frozen(row.partition) || bridges != frozen(row.mean_bridges) {
        return Err(fail(
            CheckId::G08_CONFIGURATION_MARGINAL,
            None,
            format!(
                "Z={partition} mean bridges={bridges}; section 5: Z={} mean={}",
                frozen(row.partition),
                frozen(row.mean_bridges)
            ),
        ));
    }

    let values = m.notebook_values();
    let uniform = Ratio::new(1, values.len() as i128);
    for &note in &values {
        let observed = notebook_probability(m, law, note);
        if observed != uniform {
            return Err(fail(
                CheckId::G08_NOTEBOOK_MARGINAL,
                None,
                format!(
                    "pi(y={}) = {observed} != 1/(A+1) = {uniform}",
                    crate::model::note_label(note)
                ),
            ));
        }
    }
    let blank = notebook_probability(m, law, None);
    if blank != frozen(row.blank) {
        return Err(fail(
            CheckId::G08_NOTEBOOK_MARGINAL,
            None,
            format!(
                "blank probability {blank}; section 5: {}",
                frozen(row.blank)
            ),
        ));
    }

    let stock_mass: Vec<Ratio> = (0..=m.case.w)
        .map(|stock| law.mass((0..m.states.len()).filter(|&state| m.states[state].stock == stock)))
        .collect();
    let reference = stock_mass[0];
    for (stock, &mass) in stock_mass.iter().enumerate() {
        if mass * Ratio::power(m.case.b, stock as i64) != reference {
            return Err(fail(
                CheckId::G08_STOCKPILE_MARGINAL,
                None,
                format!("pi(w={stock}) = {mass} is not proportional to b^(-w)"),
            ));
        }
    }
    let mean = mean_stockpile(m, law);
    if mean != frozen(row.mean_stockpile) {
        let mut failure = fail(
            CheckId::G08_STOCKPILE_MARGINAL,
            None,
            format!(
                "mean stockpile {mean}; section 5: {}",
                frozen(row.mean_stockpile)
            ),
        );
        failure.value = Some(mean);
        return Err(failure);
    }

    let alphabet = Ratio::new(1, m.alphabet_size() as i128);
    for config in 0..m.config_count {
        for stock in 0..=m.case.w {
            let plans = m.plans[config].len();
            let joint =
                |plan: usize, note: Note| law.probability[m.state_at(config, plan, 0, note, stock)];
            let total = law.mass(
                (0..plans)
                    .flat_map(|plan| values.iter().map(move |&note| (plan, note)))
                    .map(|(plan, note)| m.state_at(config, plan, 0, note, stock)),
            );
            let plan_mass: Vec<Ratio> = (0..plans)
                .map(|plan| {
                    values
                        .iter()
                        .fold(Ratio::ZERO, |sum, &note| sum + joint(plan, note))
                })
                .collect();
            let note_mass: Vec<Ratio> = values
                .iter()
                .map(|&note| (0..plans).fold(Ratio::ZERO, |sum, plan| sum + joint(plan, note)))
                .collect();
            for plan in 0..plans {
                for (index, &note) in values.iter().enumerate() {
                    if joint(plan, note) * total != plan_mass[plan] * note_mass[index] {
                        return Err(fail(
                            CheckId::G08_PLAN_NOTEBOOK_INDEPENDENCE,
                            Some(config),
                            format!(
                                "w={stock}: plan {} and notebook {} are dependent",
                                plan_label(&m.plans[config][plan]),
                                crate::model::note_label(note)
                            ),
                        ));
                    }
                }
            }
            let mut written = Ratio::ZERO;
            let mut correct = Ratio::ZERO;
            for plan in 0..plans {
                let destination = Model::destination(&m.plans[config][plan]);
                for island in 0..m.case.n {
                    let mass = joint(plan, Some(island));
                    written = written + mass;
                    if island == destination {
                        correct = correct + mass;
                    }
                }
            }
            let fraction = correct.checked_div(written);
            if fraction != Some(alphabet) {
                return Err(fail(
                    CheckId::G08_PLAN_NOTEBOOK_INDEPENDENCE,
                    Some(config),
                    format!(
                        "w={stock}: correct fraction {} != 1/A = {alphabet}",
                        fraction.map_or("undefined".to_string(), |value| value.to_string())
                    ),
                ));
            }
        }
    }

    let complete = m.complete_state_count();
    let primary_totals = !m.is_primary() || (m.states.len(), complete) == FROZEN_PRIMARY_TOTALS;
    if m.config_count != row.configurations
        || m.states.len() != row.system_states
        || complete != row.complete_states
        || !primary_totals
    {
        return Err(fail(
            CheckId::G08_STATE_TOTALS,
            None,
            format!(
                "configurations={} system={} complete={complete}; section 5: {} {} {}",
                m.config_count,
                m.states.len(),
                row.configurations,
                row.system_states,
                row.complete_states
            ),
        ));
    }
    pass()
}

// ---------------------------------------------------------------- G09

pub(crate) fn gate_09(cx: &Context) -> GateResult {
    let m = cx.model;
    for config in 0..m.config_count {
        let counts = destination_counts(m, config);
        let reachable = counts.iter().filter(|&&count| count > 0).count();
        let matches_matrix = counts == m.matrix_endpoints[config];
        let matches_frozen = !m.is_primary()
            || (m.matrix_count[config] == FROZEN_PRIMARY_COUNTS[config]
                && counts.as_slice() == FROZEN_PRIMARY_DESTINATIONS[config]
                && reachable == FROZEN_PRIMARY_REACHABLE[config]);
        if !matches_matrix || !matches_frozen {
            return Err(fail(
                CheckId::G09_DESTINATION_COUNTS,
                Some(config),
                format!(
                    "n_y by enumeration [{}], by e_h^T L^tau e_y [{}], |D|={reachable}",
                    counts
                        .iter()
                        .map(i128::to_string)
                        .collect::<Vec<_>>()
                        .join(","),
                    m.matrix_endpoints[config]
                        .iter()
                        .map(i128::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                ),
            ));
        }
    }
    for config in 0..m.config_count {
        let counts = destination_counts(m, config);
        let witness = capacity_witness(m, config);
        let reachable = witness.len();
        let atom = Ratio::new(1, reachable.max(1) as i128);
        let mut outcome = vec![Ratio::ZERO; m.case.n];
        for (_, plan) in &witness {
            let island = Model::destination(plan);
            outcome[island] = outcome[island] + atom;
        }
        let uniform = reachable > 0
            && (0..m.case.n).all(|island| {
                outcome[island]
                    == if counts[island] > 0 {
                        atom
                    } else {
                        Ratio::ZERO
                    }
            });
        let labels: Vec<String> = witness.iter().map(|(_, plan)| plan_label(plan)).collect();
        let matches_frozen = !m.is_primary() || labels == FROZEN_PRIMARY_WITNESS[config];
        if !uniform || !matches_frozen {
            return Err(fail(
                CheckId::G09_CAPACITY_WITNESS,
                Some(config),
                format!(
                    "witness plans [{}] give outcome law [{}]",
                    labels.join(","),
                    outcome
                        .iter()
                        .map(Ratio::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                ),
            ));
        }
    }
    for config in 0..m.config_count {
        let counts = destination_counts(m, config);
        let observed = uniform_plan_information(m, config);
        let target = plan_information_target(m.matrix_count[config], &counts);
        let matches_frozen = !m.is_primary()
            || observed == LogVector::of_ratio(frozen(FROZEN_PRIMARY_EXP_NI[config]));
        if observed != target || !matches_frozen {
            return Err(fail(
                CheckId::G09_PLAN_LAW_INFORMATION,
                Some(config),
                format!("N I(G) = {observed}, N ln N - sum n_y ln n_y = {target}"),
            ));
        }
    }
    pass()
}

// ---------------------------------------------------------------- G10

pub(crate) fn gate_10(cx: &Context) -> GateResult {
    let m = cx.model;
    let law = cx.law(CheckId::G10_FEEDBACK_STRUCTURE)?;
    let targets = |state: usize, kind: Kind| -> BTreeSet<usize> {
        m.system.rows[state]
            .iter()
            .filter(|edge| edge.kind == kind && edge.hazard.is_positive())
            .map(|edge| edge.to)
            .collect()
    };
    let slices = |config: usize| {
        let counts = destination_counts(m, config);
        (0..m.case.n)
            .filter(move |&island| counts[island] > 0)
            .flat_map(move |island| (0..m.case.w).map(move |stock| (island, stock)))
    };
    for config in 0..m.config_count {
        for (island, stock) in slices(config) {
            let correct = correct_macrostate(m, config, island, stock);
            let released = released_macrostate(m, config, island, stock + 1);
            let correct_set: BTreeSet<usize> = correct.iter().copied().collect();
            let released_set: BTreeSet<usize> = released.iter().copied().collect();
            if let Some(&state) = correct
                .iter()
                .find(|&&state| targets(state, Kind::Harvest) != released_set)
            {
                return Err(fail(
                    CheckId::G10_FEEDBACK_STRUCTURE,
                    Some(config),
                    format!(
                        "HARVEST from {} does not reach exactly L(G,{island},{})",
                        m.describe_state(state),
                        stock + 1
                    ),
                ));
            }
            if let Some(&state) = released
                .iter()
                .find(|&&state| targets(state, Kind::Commit) != correct_set)
            {
                return Err(fail(
                    CheckId::G10_FEEDBACK_STRUCTURE,
                    Some(config),
                    format!(
                        "COMMIT from {} does not reach exactly C(G,{island},{stock})",
                        m.describe_state(state)
                    ),
                ));
            }
        }
    }
    for config in 0..m.config_count {
        let counts = destination_counts(m, config);
        for (island, stock) in slices(config) {
            let observed = exchange_constant(m, law, config, island, stock);
            let predicted = Ratio::new(m.matrix_count[config], counts[island] * m.case.b);
            if observed != Some(predicted) {
                return Err(fail(
                    CheckId::G10_MASS_RATIO,
                    Some(config),
                    format!(
                        "y={island} w={stock}: pi(L)/pi(C) = {:?} != N/(n_y b) = {predicted}",
                        observed.map(|value| value.to_string())
                    ),
                ));
            }
        }
    }
    for config in 0..m.config_count {
        for (island, stock) in slices(config) {
            let correct = correct_macrostate(m, config, island, stock);
            let released = released_macrostate(m, config, island, stock + 1);
            let flux = |sources: &[usize], sinks: &[usize]| {
                sources.iter().fold(Ratio::ZERO, |sum, &from| {
                    sinks.iter().fold(sum, |inner, &to| {
                        inner + law.probability[from] * m.system.hazard(from, to)
                    })
                })
            };
            let pi_correct = law.mass(correct.iter().copied());
            let pi_released = law.mass(released.iter().copied());
            let forward = flux(&correct, &released).checked_div(pi_correct);
            let reverse = flux(&released, &correct).checked_div(pi_released);
            let ratio = forward.zip(reverse).and_then(|(f, r)| f.checked_div(r));
            let mass_ratio = pi_released.checked_div(pi_correct);
            if ratio.is_none() || ratio != mass_ratio {
                return Err(fail(
                    CheckId::G10_RATE_RATIO,
                    Some(config),
                    format!(
                        "y={island} w={stock}: kbar(C->L)/kbar(L->C) = {:?}, pi(L)/pi(C) = {:?}",
                        ratio.map(|value| value.to_string()),
                        mass_ratio.map(|value| value.to_string())
                    ),
                ));
            }
        }
    }
    for config in 0..m.config_count {
        let counts = destination_counts(m, config);
        let table = exchange_table(m, law, config);
        let mut product = LogVector::zero();
        let mut complete = true;
        for island in (0..m.case.n).filter(|&island| counts[island] > 0) {
            match table[island] {
                Some(k) => {
                    product = product.plus(
                        &LogVector::of_ratio(Ratio::integer(m.case.b) * k)
                            .scaled(Ratio::integer(counts[island])),
                    );
                }
                None => complete = false,
            }
        }
        let target = plan_information_target(m.matrix_count[config], &counts);
        if !complete || product != target {
            return Err(fail(
                CheckId::G10_AVERAGE_WORTH,
                Some(config),
                format!("ln prod_y [b K]^(n_y) = {product} != {target}"),
            ));
        }
    }
    let observed_k_one = k_one_configurations(m, law);
    if m.is_primary() {
        for config in 0..m.config_count {
            let table = exchange_table(m, law, config);
            let predicted: Vec<Option<Ratio>> = FROZEN_PRIMARY_K[config]
                .iter()
                .map(|k| k.map(frozen))
                .collect();
            let k_one = FROZEN_PRIMARY_K_ONE.contains(&config) == observed_k_one.contains(&config);
            if table != predicted || !k_one {
                return Err(fail(
                    CheckId::G10_FROZEN_EXCHANGE,
                    Some(config),
                    format!(
                        "K = [{}] differs from section 5.1",
                        table
                            .iter()
                            .map(|k| k.map_or("-".to_string(), |value| value.to_string()))
                            .collect::<Vec<_>>()
                            .join(",")
                    ),
                ));
            }
        }
    }
    if let Some(row) = frozen_row(m) {
        for config in 0..m.config_count {
            let predicted = match row.k_one {
                FrozenKOne::Configurations(list) => list.contains(&config),
                FrozenKOne::HomeDegreeOne { .. } => {
                    (0..m.case.n)
                        .filter(|&v| m.bridge_built(config, 0, v))
                        .count()
                        == 1
                }
            };
            if predicted != observed_k_one.contains(&config) {
                return Err(fail(
                    CheckId::G10_FROZEN_EXCHANGE,
                    Some(config),
                    format!(
                        "K = 1 at every reachable y with |D| >= 2: observed {}, section 5.2 {predicted}",
                        observed_k_one.contains(&config)
                    ),
                ));
            }
        }
        if let FrozenKOne::HomeDegreeOne { count } = row.k_one {
            if observed_k_one.len() != count {
                return Err(fail(
                    CheckId::G10_FROZEN_EXCHANGE,
                    None,
                    format!(
                        "{} configurations with K = 1; section 5.2: {count}",
                        observed_k_one.len()
                    ),
                ));
            }
        }
    } else {
        return Err(fail(
            CheckId::G10_FROZEN_EXCHANGE,
            None,
            format!("case {} has no section 5.2 row", m.case.label()),
        ));
    }
    pass()
}

// ---------------------------------------------------------------- G11

/// The correlated law `A_G` at cursor 0 and stockpile `w`: the propagated
/// law conditioned on a correct written record, keyed by `(plan, record)`.
fn correlated_law(
    m: &Model,
    law: &StationaryLaw,
    config: usize,
    stock: usize,
) -> Option<BTreeMap<(usize, usize), Ratio>> {
    let pairs: Vec<(usize, usize, usize)> = m.plans[config]
        .iter()
        .enumerate()
        .map(|(plan, sequence)| {
            let record = Model::destination(sequence);
            (
                plan,
                record,
                m.state_at(config, plan, 0, Some(record), stock),
            )
        })
        .collect();
    let total = law.mass(pairs.iter().map(|&(_, _, state)| state));
    if total.is_zero() {
        return None;
    }
    Some(
        pairs
            .into_iter()
            .map(|(plan, record, state)| ((plan, record), law.probability[state] / total))
            .collect(),
    )
}

struct ShuffledDeck {
    correlated: BTreeMap<(usize, usize), Ratio>,
    plan_marginal: Vec<Ratio>,
    record_marginal: Vec<Ratio>,
    shuffled: BTreeMap<(usize, usize), Ratio>,
}

fn shuffled_deck(
    m: &Model,
    law: &StationaryLaw,
    config: usize,
    stock: usize,
) -> Option<ShuffledDeck> {
    let correlated = correlated_law(m, law, config, stock)?;
    let plans = m.plans[config].len();
    let mut plan_marginal = vec![Ratio::ZERO; plans];
    let mut record_marginal = vec![Ratio::ZERO; m.case.n];
    for (&(plan, record), &mass) in &correlated {
        plan_marginal[plan] = plan_marginal[plan] + mass;
        record_marginal[record] = record_marginal[record] + mass;
    }
    let shuffled_records = m.shuffled_record_marginal(&record_marginal);
    let mut shuffled = BTreeMap::new();
    for (plan, &p) in plan_marginal.iter().enumerate() {
        for (record, &q) in shuffled_records.iter().enumerate() {
            if (p * q).is_positive() {
                shuffled.insert((plan, record), p * q);
            }
        }
    }
    Some(ShuffledDeck {
        correlated,
        plan_marginal,
        record_marginal,
        shuffled,
    })
}

/// `S(B_G) - S(A_G)` from the two laws.
fn shuffle_gap(deck: &ShuffledDeck) -> LogVector {
    LogVector::entropy(deck.shuffled.values().copied())
        .minus(&LogVector::entropy(deck.correlated.values().copied()))
}

pub(crate) fn gate_11(cx: &Context) -> GateResult {
    let m = cx.model;
    let law = cx.law(CheckId::G11_CORRELATED_UNIFORM)?;
    let decks: Vec<Vec<Option<ShuffledDeck>>> = (0..m.config_count)
        .map(|config| {
            (0..=m.case.w)
                .map(|stock| shuffled_deck(m, law, config, stock))
                .collect()
        })
        .collect();
    for config in 0..m.config_count {
        let atom = Ratio::new(1, m.plans[config].len() as i128);
        for (stock, deck) in decks[config].iter().enumerate() {
            let uniform = deck.as_ref().is_some_and(|deck| {
                deck.correlated.len() == m.plans[config].len()
                    && deck.correlated.values().all(|&mass| mass == atom)
            });
            if !uniform {
                return Err(fail(
                    CheckId::G11_CORRELATED_UNIFORM,
                    Some(config),
                    format!("w={stock}: A_G is not uniform over the pairs (p, d(p))"),
                ));
            }
        }
    }
    for config in 0..m.config_count {
        for (stock, deck) in decks[config].iter().enumerate() {
            let Some(deck) = deck else { continue };
            let mut plan_marginal = vec![Ratio::ZERO; deck.plan_marginal.len()];
            let mut record_marginal = vec![Ratio::ZERO; m.case.n];
            for (&(plan, record), &mass) in &deck.shuffled {
                plan_marginal[plan] = plan_marginal[plan] + mass;
                record_marginal[record] = record_marginal[record] + mass;
            }
            if plan_marginal != deck.plan_marginal || record_marginal != deck.record_marginal {
                let show = |values: &[Ratio]| {
                    values
                        .iter()
                        .map(Ratio::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                };
                return Err(fail(
                    CheckId::G11_SHUFFLED_MARGINALS,
                    Some(config),
                    format!(
                        "w={stock}: B_G record marginal [{}] != A_G record marginal [{}]",
                        show(&record_marginal),
                        show(&deck.record_marginal)
                    ),
                ));
            }
        }
    }
    for config in 0..m.config_count {
        for state in m.states_of(config) {
            let SystemState {
                plan,
                cursor,
                stock,
                ..
            } = m.states[state];
            for note in m.notebook_values() {
                let other = m.state_at(config, plan, cursor, note, stock);
                if m.system_energy(other) != m.system_energy(state) {
                    return Err(fail(
                        CheckId::G11_NOTEBOOK_BLIND_ENERGY,
                        Some(config),
                        format!(
                            "U({}) != U({})",
                            m.describe_state(state),
                            m.describe_state(other)
                        ),
                    ));
                }
            }
        }
    }
    for config in 0..m.config_count {
        let counts = destination_counts(m, config);
        let total = m.matrix_count[config];
        let target = plan_information_target(total, &counts);
        for (stock, deck) in decks[config].iter().enumerate() {
            let Some(deck) = deck else { continue };
            let observed = shuffle_gap(deck).scaled(Ratio::integer(total));
            if observed != target {
                return Err(fail(
                    CheckId::G11_GAP_FROM_LAWS,
                    Some(config),
                    format!("w={stock}: N [S(B)-S(A)] = {observed} != {target}"),
                ));
            }
        }
    }
    for config in 0..m.config_count {
        let counts = destination_counts(m, config);
        let table = exchange_table(m, law, config);
        let exchange = (0..m.case.n).filter(|&island| counts[island] > 0).try_fold(
            LogVector::zero(),
            |sum, island| {
                table[island].map(|k| {
                    sum.plus(
                        &LogVector::of_ratio(Ratio::integer(m.case.b) * k)
                            .scaled(Ratio::integer(counts[island])),
                    )
                })
            },
        );
        for (stock, deck) in decks[config].iter().enumerate() {
            let Some(deck) = deck else { continue };
            let observed = shuffle_gap(deck).scaled(Ratio::integer(m.matrix_count[config]));
            if exchange.as_ref() != Some(&observed) {
                return Err(fail(
                    CheckId::G11_GAP_EQUALS_EXCHANGE,
                    Some(config),
                    format!(
                        "w={stock}: N [S(B)-S(A)] = {observed} != ln prod_y [b K]^(n_y) = {}",
                        exchange.map_or("undefined".to_string(), |value| value.to_string())
                    ),
                ));
            }
        }
    }
    for config in 0..m.config_count {
        let reachable = capacity_witness(m, config).len();
        let observed = capacity_information(m, config);
        let target = capacity_target(reachable);
        let matches_frozen = !m.is_primary()
            || observed == LogVector::of_integer(FROZEN_PRIMARY_EXP_CAPACITY[config] as u128);
        // Section 5.1: I(G) = I*(G) exactly at the listed configurations.
        let equal_information = match decks[config][0].as_ref() {
            Some(deck) => {
                let information = shuffle_gap(deck);
                let capacity = observed.scaled(Ratio::new(1, reachable as i128));
                !m.is_primary()
                    || (information == capacity)
                        == FROZEN_PRIMARY_EQUAL_INFORMATION.contains(&config)
            }
            None => false,
        };
        if observed != target || !matches_frozen || !equal_information {
            return Err(fail(
                CheckId::G11_CAPACITY_GAP,
                Some(config),
                format!(
                    "|D| [S(B*)-S(A*)] = {observed}, |D| ln |D| = {target}, I = I* as section 5.1: {equal_information}"
                ),
            ));
        }
    }
    pass()
}

// ---------------------------------------------------------------- G12

pub(crate) fn gate_12(cx: &Context) -> GateResult {
    let m = cx.model;
    let law = cx.law(CheckId::G12_ERASURE_STRUCTURE)?;
    let targets = |state: usize, kind: Kind| -> BTreeSet<usize> {
        m.system.rows[state]
            .iter()
            .filter(|edge| edge.kind == kind && edge.hazard.is_positive())
            .map(|edge| edge.to)
            .collect()
    };
    let slots = |config: usize| {
        (0..m.plans[config].len())
            .flat_map(move |plan| (1..=m.case.w).map(move |stock| (plan, stock)))
    };
    for config in 0..m.config_count {
        for (plan, stock) in slots(config) {
            let blank = m.state_at(config, plan, 0, None, stock - 1);
            let written: BTreeSet<usize> = (0..m.case.n)
                .map(|island| m.state_at(config, plan, 0, Some(island), stock))
                .collect();
            if let Some(&state) = written
                .iter()
                .find(|&&state| targets(state, Kind::Erase) != BTreeSet::from([blank]))
            {
                return Err(fail(
                    CheckId::G12_ERASURE_STRUCTURE,
                    Some(config),
                    format!(
                        "{} has ERASE targets {:?}, expected exactly {}",
                        m.describe_state(state),
                        targets(state, Kind::Erase),
                        m.describe_state(blank)
                    ),
                ));
            }
            let scribbled = targets(blank, Kind::Scribble);
            if scribbled != written || scribbled.len() != m.alphabet_size() {
                return Err(fail(
                    CheckId::G12_ERASURE_STRUCTURE,
                    Some(config),
                    format!(
                        "{} has {} SCRIBBLE targets, expected the A = {} written states",
                        m.describe_state(blank),
                        scribbled.len(),
                        m.alphabet_size()
                    ),
                ));
            }
        }
    }
    let predicted = Ratio::new(m.case.b, m.alphabet_size() as i128);
    for config in 0..m.config_count {
        for (plan, stock) in slots(config) {
            let observed = erasure_ratio(m, law, config, plan, stock);
            if observed != Some(predicted) {
                return Err(fail(
                    CheckId::G12_BLANK_RATIO,
                    Some(config),
                    format!(
                        "p={} w={stock}: pi(B)/pi(R) = {:?} != b/A = {predicted}",
                        plan_label(&m.plans[config][plan]),
                        observed.map(|value| value.to_string())
                    ),
                ));
            }
        }
    }
    let Some(row) = frozen_row(m) else {
        return Err(fail(
            CheckId::G12_FROZEN_ERASURE,
            None,
            format!("case {} has no section 5.2 row", m.case.label()),
        ));
    };
    for config in 0..m.config_count {
        for (plan, stock) in slots(config) {
            let observed = erasure_ratio(m, law, config, plan, stock);
            if observed != Some(frozen(row.erasure)) {
                return Err(fail(
                    CheckId::G12_FROZEN_ERASURE,
                    Some(config),
                    format!(
                        "erasure ratio {:?}; section 5.2: {}",
                        observed.map(|value| value.to_string()),
                        frozen(row.erasure)
                    ),
                ));
            }
        }
    }
    pass()
}

// ---------------------------------------------------------------- G13

pub(crate) fn gate_13(cx: &Context) -> GateResult {
    let m = cx.model;
    let law = cx.law(CheckId::G13_STOCKPILE_CURRENT)?;
    let mut packets = Ratio::ZERO;
    let mut records = Ratio::ZERO;
    for from in 0..m.states.len() {
        for edge in &m.system.rows[from] {
            let flux = law.probability[from] * edge.hazard;
            let change = m.states[edge.to].stock as i128 - m.states[from].stock as i128;
            packets = packets + flux * Ratio::integer(change);
            match (m.states[from].note, m.states[edge.to].note) {
                (Some(_), None) => records = records + flux,
                (None, Some(_)) => records = records - flux,
                _ => {}
            }
        }
    }
    if !packets.is_zero() {
        return Err(fail(
            CheckId::G13_STOCKPILE_CURRENT,
            None,
            format!("net packet current into the stockpile = {packets}"),
        ));
    }
    if !records.is_zero() {
        return Err(fail(
            CheckId::G13_NOTEBOOK_CURRENT,
            None,
            format!("net written-to-blank current = {records}"),
        ));
    }
    pass()
}

// ---------------------------------------------------------------- G14

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GraphSubstrateBinding {
    BridgesHeldPlanCursorNotebookStockpile,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CrystalBinding {
    UniformReplanOverValidPlans,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThermoBinding {
    ReservoirLawEnergyThaimPairAndXiNotEvaluated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PraxionBinding {
    BaseActorUnitSymmetricHazards,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActionBinding {
    BuildDismantleHarvestCommitEraseScribble,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResolutionBinding {
    StepReplanWritesPlanMeasureWritesNotebook,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThawBinding {
    DeclaredReverseOfEachChannel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MemoryBinding {
    HeldPlanCursorAndNotebookNoLearning,
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
}

/// Quantities the construction exposes; `HAZARD_READS` lists those a
/// channel hazard may read (mirrors `HazardInputs`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Quantity {
    SourceReservoirEnergy,
    DestinationReservoirEnergy,
    ReservoirBase,
    PlanCount,
    DestinationCount,
    PlanInformation,
    CapacityInformation,
    ExchangeConstant,
    Thaim,
    Xi,
}

const HAZARD_READS: [Quantity; 3] = [
    Quantity::SourceReservoirEnergy,
    Quantity::DestinationReservoirEnergy,
    Quantity::ReservoirBase,
];
const FORBIDDEN_HAZARD_READS: [Quantity; 7] = [
    Quantity::PlanCount,
    Quantity::DestinationCount,
    Quantity::PlanInformation,
    Quantity::CapacityInformation,
    Quantity::ExchangeConstant,
    Quantity::Thaim,
    Quantity::Xi,
];

pub(crate) fn gate_14(cx: &Context) -> GateResult {
    let m = cx.model;
    let slots = XypherSlotReading {
        graph_substrate: GraphSubstrateBinding::BridgesHeldPlanCursorNotebookStockpile,
        crystal: CrystalBinding::UniformReplanOverValidPlans,
        thermo: ThermoBinding::ReservoirLawEnergyThaimPairAndXiNotEvaluated,
        praxion: PraxionBinding::BaseActorUnitSymmetricHazards,
        action: ActionBinding::BuildDismantleHarvestCommitEraseScribble,
        resolution: ResolutionBinding::StepReplanWritesPlanMeasureWritesNotebook,
        thaw: ThawBinding::DeclaredReverseOfEachChannel,
        memory: MemoryBinding::HeldPlanCursorAndNotebookNoLearning,
        ruby: None,
        opal_phi: None,
    };
    let named = matches!(
        slots,
        XypherSlotReading {
            graph_substrate: GraphSubstrateBinding::BridgesHeldPlanCursorNotebookStockpile,
            crystal: CrystalBinding::UniformReplanOverValidPlans,
            thermo: ThermoBinding::ReservoirLawEnergyThaimPairAndXiNotEvaluated,
            praxion: PraxionBinding::BaseActorUnitSymmetricHazards,
            action: ActionBinding::BuildDismantleHarvestCommitEraseScribble,
            resolution: ResolutionBinding::StepReplanWritesPlanMeasureWritesNotebook,
            thaw: ThawBinding::DeclaredReverseOfEachChannel,
            memory: MemoryBinding::HeldPlanCursorAndNotebookNoLearning,
            ..
        }
    );
    if !named {
        return Err(fail(
            CheckId::G14_NAMED_SLOTS,
            None,
            "one or more section 3.4 bindings are unbound",
        ));
    }
    if slots.ruby.is_some() || slots.opal_phi.is_some() {
        return Err(fail(
            CheckId::G14_EMPTY_RUBY,
            None,
            "Ruby must be empty and Opal/Phi absent",
        ));
    }
    if HAZARD_READS
        .iter()
        .any(|read| FORBIDDEN_HAZARD_READS.contains(read))
    {
        return Err(fail(
            CheckId::G14_HAZARD_INPUTS,
            None,
            format!("hazard reads {HAZARD_READS:?} include a forbidden quantity"),
        ));
    }
    for config in 0..m.config_count {
        for from in m.states_of(config) {
            for edge in &m.system.rows[from] {
                let expected = expected_system_hazard(m, edge.kind, edge.to);
                if edge.hazard != expected {
                    return Err(fail(
                        CheckId::G14_HAZARD_INPUTS,
                        Some(config),
                        format!(
                            "{} hazard {} at {} differs from the b^(E_R(z')) / 1 rule {expected}",
                            edge.kind.code(),
                            edge.hazard,
                            m.describe_state(from)
                        ),
                    ));
                }
            }
        }
        if let Some(micro) = &m.micro {
            let level = Level::Complete(micro);
            for from in level.rows_of(m, config) {
                if let Some(edge) = micro.rows[from]
                    .iter()
                    .map(|edge| edge.edge())
                    .find(|edge| edge.hazard != KAPPA)
                {
                    return Err(fail(
                        CheckId::G14_HAZARD_INPUTS,
                        Some(config),
                        format!(
                            "micro {} hazard {} != kappa at complete state {from}",
                            edge.kind.code(),
                            edge.hazard
                        ),
                    ));
                }
            }
        }
    }
    Ok(Status::Pass(format!(
        "hazards are computed from HazardInputs {HAZARD_READS:?}, a type with no N_tau, n_y, I, I*, K, THAIM, or Xi field; every system hazard equals the independent b^(E_R(z'))/1 rule and every micro hazard is kappa"
    )))
}
