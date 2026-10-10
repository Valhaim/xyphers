//! Gates G01--G10 of boundary section 5.
//!
//! A gate runs its checks in the listed order; each check runs over every
//! world in the row order of section 4.2, LOCAL before GLOBAL within a world,
//! and over configurations and states in canonical order, before the next
//! check begins. A gate reports its first failing check.
//!
//! The steady state is never assumed: it is the exact solution of `p Q = 0`,
//! `sum p = 1`, from fraction-free elimination over the constructed channels
//! (`src/solve.rs`), as integers `n_z` with `p(z) = n_z / D`. Logarithms enter
//! only `sigma_floor`, `eta`, `Y_bound`, and `eta_global`, as certified
//! enclosures (`src/logs.rs`). No floating-point value is used.

use std::cell::OnceCell;

use crate::big::Big;
use crate::logs::{ln_integer, Dyadic, Enclosure};
use crate::model::{
    plan_label, Agent, Channel, Class, Model, Mutation, Packet, Store, SystemState, WorldSpec,
    FORBIDDEN_RATE_READS, LAMBDA, RATE_READS, UNIT_RATE,
};
use crate::rat::Rat;
use crate::solve::stationary_vector;
use crate::CheckId;

// ------------------------------------------------------------ frozen values

/// The temperature pairs `(b_c, b_h)` in the column order of section 4.1.
pub(crate) const PAIRS: [(u64, u64); 3] = [(3, 2), (8, 2), (6, 3)];

/// One row of the section 4.1 table, copied verbatim.
pub(crate) struct FrozenUndriven {
    n: usize,
    tau: usize,
    states: usize,
    /// `<N_2>` under `pi_c`, one entry per pair of `PAIRS`.
    means: [&'static str; 3],
}

/// Boundary section 4.1.
pub(crate) const FROZEN_UNDRIVEN: [FrozenUndriven; 4] = [
    FrozenUndriven {
        n: 3,
        tau: 1,
        states: 32,
        means: ["55/16", "1955/891", "367/147"],
    },
    FrozenUndriven {
        n: 3,
        tau: 2,
        states: 108,
        means: ["117/28", "3497/1233", "99/31"],
    },
    FrozenUndriven {
        n: 3,
        tau: 3,
        states: 344,
        means: ["262/55", "6827/1955", "1411/367"],
    },
    FrozenUndriven {
        n: 4,
        tau: 1,
        states: 320,
        means: ["19/4", "226/81", "797/245"],
    },
];

/// One row of the section 4.2 table, copied verbatim.
pub(crate) struct FrozenGlobal {
    n: usize,
    tau: usize,
    b_c: u64,
    b_h: u64,
    h: &'static str,
    j: &'static str,
    y: &'static str,
}

const fn frozen_global(
    n: usize,
    tau: usize,
    b_c: u64,
    b_h: u64,
    h: &'static str,
    j: &'static str,
    y: &'static str,
) -> FrozenGlobal {
    FrozenGlobal {
        n,
        tau,
        b_c,
        b_h,
        h,
        j,
        y,
    }
}

/// Boundary section 4.2, rows in order.
pub(crate) const FROZEN_GLOBAL: [FrozenGlobal; 12] = [
    frozen_global(3, 1, 3, 2, "2531/8624", "29/77", "0.7792"),
    frozen_global(3, 2, 3, 2, "1115/3948", "323/987", "0.8630"),
    frozen_global(3, 3, 3, 2, "28563/110605", "579/2011", "0.8969"),
    frozen_global(3, 1, 8, 2, "7429/14256", "11/8", "0.3790"),
    frozen_global(3, 2, 8, 2, "54815/91242", "91/74", "0.4885"),
    frozen_global(3, 3, 8, 2, "1143/1955", "429/391", "0.5329"),
    frozen_global(3, 1, 6, 3, "6308/17787", "41/55", "0.4757"),
    frozen_global(3, 2, 6, 3, "3852/9889", "635/957", "0.5870"),
    frozen_global(3, 3, 6, 3, "77916/211025", "1019/1725", "0.6250"),
    frozen_global(4, 1, 3, 2, "1245/2548", "72/91", "0.6176"),
    frozen_global(4, 1, 8, 2, "65/81", "17/6", "0.2832"),
    frozen_global(4, 1, 6, 3, "278676/503965", "288/187", "0.3590"),
];

/// Boundary section 4.2: `eta_global` to four decimals per pair.
pub(crate) const FROZEN_ETA_GLOBAL: [((u64, u64), &str); 3] =
    [((3, 2), "0.4497"), ((8, 2), "0.3390"), ((6, 3), "0.4150")];

fn frozen(text: &str) -> Rat {
    Rat::parse(text).expect("frozen values are rationals")
}

/// Section 4.1, `<N_2>` under `pi_c` for the world's `(N, tau)` and pair.
fn frozen_undriven_mean(spec: &WorldSpec) -> Option<Rat> {
    let row = FROZEN_UNDRIVEN
        .iter()
        .find(|row| row.n == spec.n && row.tau == spec.tau)?;
    let column = PAIRS
        .iter()
        .position(|&pair| pair == (spec.b_c, spec.b_h))?;
    Some(frozen(row.means[column]))
}

fn frozen_state_count(spec: &WorldSpec) -> Option<usize> {
    FROZEN_UNDRIVEN
        .iter()
        .find(|row| row.n == spec.n && row.tau == spec.tau)
        .map(|row| row.states)
}

fn frozen_global_row(spec: &WorldSpec) -> Option<&'static FrozenGlobal> {
    FROZEN_GLOBAL
        .iter()
        .find(|row| (row.n, row.tau, row.b_c, row.b_h) == (spec.n, spec.tau, spec.b_c, spec.b_h))
}

fn frozen_eta_global(spec: &WorldSpec) -> Option<&'static str> {
    FROZEN_ETA_GLOBAL
        .iter()
        .find(|(pair, _)| *pair == (spec.b_c, spec.b_h))
        .map(|(_, decimal)| *decimal)
}

/// `10^-9`, the certified width of G08 checks 4 and 5.
fn width_tolerance() -> Rat {
    Rat::ratio(1, 1_000_000_000)
}

// ------------------------------------------------------------ steady state

/// Exact steady state `p(z) = n_z / D` with `D = sum n_z`.
#[derive(Clone, Debug)]
pub(crate) struct SteadyState {
    pub(crate) weights: Vec<Big>,
    pub(crate) total: Big,
}

/// Section 2 measures, exact.
#[derive(Clone, Debug)]
pub(crate) struct Measures {
    /// `<N_R>_p`.
    pub(crate) open_future_held: Rat,
    /// `<N_R>_(pi_c)`, closed form `b_c^(-|G|)`.
    pub(crate) open_future_cold: Rat,
    /// `H = <N_R>_p - <N_R>_(pi_c)`.
    pub(crate) held: Rat,
    /// `J`: net packets per unit time from the hot store into the bridges.
    pub(crate) fuel_flow: Rat,
    /// `J D`.
    pub(crate) fuel_numerator: Big,
    /// Net packets per unit time into the cold store, times `D`.
    pub(crate) cold_numerator: Big,
}

/// Certified enclosures of the price floor (sections 2 and 6).
#[derive(Clone, Debug)]
pub(crate) struct Floor {
    pub(crate) sigma_floor: Enclosure,
    pub(crate) eta: Enclosure,
    /// `ln(b_c / b_h)`.
    pub(crate) log_ratio: Enclosure,
}

/// One driven world with one agent, with lazily computed exact results.
pub(crate) struct Run {
    pub(crate) model: Model,
    steady: OnceCell<Result<SteadyState, String>>,
    measures: OnceCell<Result<Measures, String>>,
    floor: OnceCell<Result<Floor, String>>,
}

impl Run {
    fn new(model: Model) -> Self {
        Self {
            model,
            steady: OnceCell::new(),
            measures: OnceCell::new(),
            floor: OnceCell::new(),
        }
    }

    pub(crate) fn steady(&self) -> Result<&SteadyState, String> {
        self.steady
            .get_or_init(|| solve_steady_state(&self.model))
            .as_ref()
            .map_err(Clone::clone)
    }

    pub(crate) fn measures(&self) -> Result<&Measures, String> {
        self.measures
            .get_or_init(|| {
                self.steady()
                    .map(|steady| compute_measures(&self.model, steady))
            })
            .as_ref()
            .map_err(Clone::clone)
    }

    /// Divides by `sigma = J ln(b_c / b_h)`; evaluated only when `J > 0`.
    pub(crate) fn floor(&self) -> Result<&Floor, String> {
        self.floor
            .get_or_init(|| {
                let steady = self.steady()?;
                let measures = self.measures()?;
                compute_floor(&self.model, steady, measures)
            })
            .as_ref()
            .map_err(Clone::clone)
    }
}

/// Every world of a list with both agents, LOCAL before GLOBAL.
pub(crate) struct Suite {
    pub(crate) runs: Vec<Run>,
}

impl Suite {
    pub(crate) fn build(worlds: &[WorldSpec], mutation: Mutation) -> Self {
        let runs = worlds
            .iter()
            .flat_map(|&world| {
                Agent::ALL
                    .into_iter()
                    .map(move |agent| Run::new(Model::build(world, agent, mutation)))
            })
            .collect();
        Self { runs }
    }
}

fn solve_steady_state(m: &Model) -> Result<SteadyState, String> {
    let rates: Vec<(usize, usize, u64)> = m
        .channels
        .iter()
        .flatten()
        .filter_map(|channel| channel.to.map(|to| (channel.from, to, channel.rate)))
        .collect();
    let weights = stationary_vector(m.states.len(), &rates)
        .ok_or_else(|| "p Q = 0 with sum p = 1 is singular".to_string())?;
    let total = weights
        .iter()
        .fold(Big::zero(), |sum, weight| &sum + weight);
    if !total.is_positive() {
        return Err(format!("normalisation D = {total} is not positive"));
    }
    Ok(SteadyState { weights, total })
}

/// Integer weights `b_c^(K - |G|)` of `pi_c` per configuration.
fn cold_weights(m: &Model) -> Vec<Big> {
    let base = Big::from_u64(m.spec.b_c);
    (0..m.config_count)
        .map(|config| base.pow((m.bridge_count() - m.size(config)) as u32))
        .collect()
}

/// `<N_R>` under `pi_c(z) ~ b_c^(-|G|)`, summed over system states.
pub(crate) fn undriven_open_future(m: &Model) -> Rat {
    let weights = cold_weights(m);
    let mut mass = Big::zero();
    let mut open = Big::zero();
    for (state, current) in m.states.iter().enumerate() {
        let weight = &weights[current.config];
        mass = &mass + weight;
        open = &open + &weight.mul_small(m.open_future(state));
    }
    Rat::new(open, mass)
}

fn compute_measures(m: &Model, steady: &SteadyState) -> Measures {
    let mut fuel = Big::zero();
    let mut cold = Big::zero();
    let mut open = Big::zero();
    for (state, row) in m.channels.iter().enumerate() {
        let mut fuel_rate = 0_i64;
        let mut cold_rate = 0_i64;
        for channel in row.iter().filter(|channel| channel.to.is_some()) {
            let rate = channel.rate as i64;
            match (channel.class, channel.packet) {
                (Class::Fuel, Packet::FromStore) => fuel_rate += rate,
                (Class::Fuel, Packet::IntoStore) => fuel_rate -= rate,
                (Class::Weather, Packet::IntoStore) => cold_rate += rate,
                (Class::Weather, Packet::FromStore) => cold_rate -= rate,
                _ => {}
            }
        }
        let weight = &steady.weights[state];
        fuel = &fuel + &(weight * &Big::from_i64(fuel_rate));
        cold = &cold + &(weight * &Big::from_i64(cold_rate));
        open = &open + &weight.mul_small(m.open_future(state));
    }
    let open_future_held = Rat::new(open, steady.total.clone());
    let open_future_cold = undriven_open_future(m);
    let held = &open_future_held - &open_future_cold;
    Measures {
        open_future_held,
        open_future_cold,
        held,
        fuel_flow: Rat::new(fuel.clone(), steady.total.clone()),
        fuel_numerator: fuel,
        cold_numerator: cold,
    }
}

/// `sigma_floor = -(1/D) sum_z m_z [ln n_z + |G(z)| ln b_c]` with
/// `m_z = D (p L_undriven)(z)`, the section 6 per-state formula, and
/// `eta = sigma_floor / sigma` with `sigma = J ln(b_c / b_h)`.
fn compute_floor(m: &Model, steady: &SteadyState, measures: &Measures) -> Result<Floor, String> {
    if !measures.fuel_flow.is_positive() {
        return Err(format!(
            "J = {} is not positive; sigma = J ln(b_c/b_h) is not evaluated",
            measures.fuel_flow
        ));
    }
    let mut balance = vec![Big::zero(); m.states.len()];
    for (from, row) in m.channels.iter().enumerate() {
        for channel in row.iter().filter(|channel| channel.class.is_undriven()) {
            let Some(to) = channel.to else { continue };
            let flow = steady.weights[from].mul_small(channel.rate);
            balance[to] = &balance[to] + &flow;
            balance[from] = &balance[from] - &flow;
        }
    }
    let mut sum = Dyadic::zero();
    let mut bridge_weight = Big::zero();
    for (state, weight) in balance.iter().enumerate() {
        if weight.is_zero() {
            continue;
        }
        let n = &steady.weights[state];
        if !n.is_positive() {
            return Err(format!(
                "n_z = {n} is not positive at {}",
                m.describe_state(state)
            ));
        }
        sum.add_scaled(weight, &ln_integer(n));
        bridge_weight = &bridge_weight + &weight.mul_small(m.size(m.states[state].config) as u64);
    }
    let ln_cold = ln_integer(&Big::from_u64(m.spec.b_c));
    let ln_hot = ln_integer(&Big::from_u64(m.spec.b_h));
    sum.add_scaled(&bridge_weight, &ln_cold);
    let sigma_floor = sum.negated_over(&steady.total);
    let log_ratio = ln_cold.enclosure().minus(&ln_hot.enclosure());
    let sigma = Enclosure::point(measures.fuel_flow.clone()).times(&log_ratio);
    let eta = sigma_floor
        .divided_by(&sigma)
        .ok_or_else(|| "the enclosure of sigma contains zero".to_string())?
        .round_outward();
    Ok(Floor {
        sigma_floor,
        eta,
        log_ratio,
    })
}

/// `Y_bound = H ln(b_c / b_h) / sigma_floor`, certified, when the
/// enclosure of `sigma_floor` is strictly positive.
pub(crate) fn yield_bound(run: &Run) -> Result<Enclosure, String> {
    let measures = run.measures()?;
    let floor = run.floor()?;
    if !floor.sigma_floor.lo.is_positive() {
        return Err("the enclosure of sigma_floor is not strictly positive".to_string());
    }
    Enclosure::point(measures.held.clone())
        .times(&floor.log_ratio)
        .divided_by(&floor.sigma_floor)
        .map(|bound| bound.round_outward())
        .ok_or_else(|| "the enclosure of sigma_floor contains zero".to_string())
}

/// Certified `ln[2 b_c/(b_c + b_h)] / ln(b_c/b_h)` (section 4.2).
pub(crate) fn global_efficiency_closed_form(spec: &WorldSpec) -> Option<Enclosure> {
    let numerator = ln_integer(&Big::from_u64(2 * spec.b_c))
        .enclosure()
        .minus(&ln_integer(&Big::from_u64(spec.b_c + spec.b_h)).enclosure());
    let denominator = ln_integer(&Big::from_u64(spec.b_c))
        .enclosure()
        .minus(&ln_integer(&Big::from_u64(spec.b_h)).enclosure());
    numerator
        .divided_by(&denominator)
        .map(|value| value.round_outward())
}

// ------------------------------------------------------------ failures

/// First failure of a gate: check, run (world and agent), configuration of
/// the offending state (`None` for a world-level check), detail, and the
/// observed value of a world-level check where one is recorded.
#[derive(Clone, Debug)]
pub(crate) struct Failure {
    pub(crate) check: CheckId,
    pub(crate) run: usize,
    pub(crate) configuration: Option<usize>,
    pub(crate) detail: String,
    pub(crate) value: Option<Rat>,
}

pub(crate) type GateResult = Result<String, Failure>;

fn fail(
    check: CheckId,
    run: usize,
    configuration: Option<usize>,
    detail: impl Into<String>,
) -> Failure {
    Failure {
        check,
        run,
        configuration,
        detail: detail.into(),
        value: None,
    }
}

/// Runs `test` on every state of every run in order.
fn scan_states(
    suite: &Suite,
    check: CheckId,
    mut test: impl FnMut(&Run, usize) -> Option<String>,
) -> Result<(), Failure> {
    for (index, run) in suite.runs.iter().enumerate() {
        for state in 0..run.model.states.len() {
            if let Some(detail) = test(run, state) {
                return Err(fail(
                    check,
                    index,
                    Some(run.model.states[state].config),
                    detail,
                ));
            }
        }
    }
    Ok(())
}

/// Runs `test` on every channel of every state of every run in order.
fn scan_channels(
    suite: &Suite,
    check: CheckId,
    mut test: impl FnMut(&Model, &Channel) -> Option<String>,
) -> Result<(), Failure> {
    scan_states(suite, check, |run, state| {
        let m = &run.model;
        m.channels[state].iter().find_map(|channel| {
            test(m, channel).map(|detail| format!("{}: {detail}", m.describe_channel(channel)))
        })
    })
}

/// Runs `test` on every configuration of every run in order.
fn scan_configurations(
    suite: &Suite,
    check: CheckId,
    mut test: impl FnMut(&Model, usize) -> Option<String>,
) -> Result<(), Failure> {
    for (index, run) in suite.runs.iter().enumerate() {
        for config in 0..run.model.config_count {
            if let Some(detail) = test(&run.model, config) {
                return Err(fail(check, index, Some(config), detail));
            }
        }
    }
    Ok(())
}

/// A world-level outcome: `Err` carries a detail, the configuration of an
/// offending state if any, and an observed value if any.
type RunCheck = Result<(), (String, Option<usize>, Option<Rat>)>;

/// Runs a world-level `test` on every run in order.
fn scan_runs(
    suite: &Suite,
    check: CheckId,
    mut test: impl FnMut(&Run) -> RunCheck,
) -> Result<(), Failure> {
    for (index, run) in suite.runs.iter().enumerate() {
        if let Err((detail, configuration, value)) = test(run) {
            let mut failure = fail(check, index, configuration, detail);
            failure.value = value;
            return Err(failure);
        }
    }
    Ok(())
}

/// World-level failure without a configuration or value.
fn world(detail: impl Into<String>) -> (String, Option<usize>, Option<Rat>) {
    (detail.into(), None, None)
}

/// The reverse of a channel: the channel of the same class (and bridge)
/// from its destination back to its source.
fn reverse_of<'a>(m: &'a Model, channel: &Channel) -> Option<&'a Channel> {
    let to = channel.to?;
    m.channels[to].iter().find(|candidate| {
        candidate.to == Some(channel.from)
            && candidate.class == channel.class
            && candidate.bridge == channel.bridge
    })
}

/// Breadth-first reachability over admitted resolved channels.
fn reach(m: &Model, root: usize, admit: fn(Class) -> bool, backward: bool) -> Vec<bool> {
    let count = m.states.len();
    let mut rows: Vec<Vec<usize>> = vec![Vec::new(); count];
    for (from, row) in m.channels.iter().enumerate() {
        for channel in row.iter().filter(|channel| admit(channel.class)) {
            if let Some(to) = channel.to {
                if backward {
                    rows[to].push(from);
                } else {
                    rows[from].push(to);
                }
            }
        }
    }
    let mut seen = vec![false; count];
    if count == 0 {
        return seen;
    }
    seen[root] = true;
    let mut queue = std::collections::VecDeque::from([root]);
    while let Some(state) = queue.pop_front() {
        for &next in &rows[state] {
            if !seen[next] {
                seen[next] = true;
                queue.push_back(next);
            }
        }
    }
    seen
}

/// First state (canonical order) not mutually reachable with `z0`.
fn first_unconnected(m: &Model, admit: fn(Class) -> bool) -> Option<usize> {
    let forward = reach(m, 0, admit, false);
    let backward = reach(m, 0, admit, true);
    (0..m.states.len()).find(|&state| !forward[state] || !backward[state])
}

// ------------------------------------------------------------ G01

pub(crate) fn gate_01(suite: &Suite) -> GateResult {
    scan_configurations(suite, CheckId::G01_PLAN_COUNT_POSITIVE, |m, config| {
        (m.matrix_count_tau[config] < 1).then(|| {
            format!(
                "N_tau({}) = {} < 1",
                m.config_label(config),
                m.matrix_count_tau[config]
            )
        })
    })?;
    scan_configurations(suite, CheckId::G01_DEPTH_FIRST_COUNTS, |m, config| {
        let tau = m.plans[config].len() as u64;
        let reference = m.plans_r[config].len() as u64;
        if tau != m.matrix_count_tau[config] {
            Some(format!(
                "{tau} depth-first {}-step plans, e_h^T L^tau 1 = {}",
                m.spec.tau, m.matrix_count_tau[config]
            ))
        } else if reference != m.matrix_count_r[config] {
            Some(format!(
                "{reference} depth-first {}-step plans, e_h^T L^R 1 = {}",
                m.spec.r, m.matrix_count_r[config]
            ))
        } else {
            None
        }
    })?;
    scan_runs(suite, CheckId::G01_STATE_COUNT, |run| {
        let m = &run.model;
        match frozen_state_count(&m.declared) {
            Some(expected) if expected == m.states.len() => Ok(()),
            Some(expected) => Err(world(format!(
                "{} system states, section 4.1 states {expected}",
                m.states.len()
            ))),
            None => Err(world("no section 4.1 row for this world")),
        }
    })?;
    Ok("N_tau(G) >= 1; depth-first counts equal e_h^T L^tau 1 and e_h^T L^R 1; state counts 32/108/344/320".to_string())
}

// ------------------------------------------------------------ G02

fn destination_violation(m: &Model, channel: &Channel) -> Option<String> {
    if channel.to_config >= m.config_count {
        return Some("destination configuration is not a configuration".to_string());
    }
    if channel.to_cursor > m.spec.tau {
        return Some("destination cursor exceeds tau".to_string());
    }
    if !m.plan_valid(channel.to_config, &channel.to_plan) {
        return Some(format!(
            "plan {} is not valid in {}",
            plan_label(&channel.to_plan),
            m.config_label(channel.to_config)
        ));
    }
    let to = channel.to?;
    let resolved = m.states[to];
    if resolved.config != channel.to_config
        || m.plan_of(to) != channel.to_plan.as_slice()
        || resolved.cursor != channel.to_cursor
    {
        return Some("resolved state differs from the destination description".to_string());
    }
    None
}

fn destination_missing(channel: &Channel) -> Option<String> {
    channel
        .to
        .is_none()
        .then(|| "destination is not a system state of the world".to_string())
}

fn step_replan_violation(m: &Model, state: usize) -> Option<String> {
    let SystemState { config, cursor, .. } = m.states[state];
    let plan = m.plan_of(state);
    let tau = m.spec.tau;
    let mut step_cursors: Vec<usize> = Vec::new();
    let mut replan_plans: Vec<&[usize]> = Vec::new();
    for channel in &m.channels[state] {
        match channel.class {
            Class::Step => {
                if channel.to_config != config || channel.to_plan != plan {
                    return Some("STEP changes the configuration or the plan".to_string());
                }
                if channel.to_cursor + 1 != cursor && channel.to_cursor != cursor + 1 {
                    return Some("STEP does not move the cursor by one".to_string());
                }
                step_cursors.push(channel.to_cursor);
            }
            Class::Replan => {
                if cursor != 0 {
                    return Some(format!("REPLAN at cursor {cursor}"));
                }
                if channel.to_config != config || channel.to_cursor != 0 {
                    return Some("REPLAN changes the configuration or the cursor".to_string());
                }
                if channel.to_plan == plan {
                    return Some("REPLAN to the held plan".to_string());
                }
                replan_plans.push(&channel.to_plan);
            }
            Class::Weather | Class::Fuel => {}
        }
    }
    let mut expected_cursors = Vec::new();
    if cursor < tau {
        expected_cursors.push(cursor + 1);
    }
    if cursor > 0 {
        expected_cursors.push(cursor - 1);
    }
    step_cursors.sort_unstable();
    expected_cursors.sort_unstable();
    if step_cursors != expected_cursors {
        return Some(format!(
            "STEP to cursors {step_cursors:?}, declared {expected_cursors:?}"
        ));
    }
    let mut expected_plans: Vec<&[usize]> = if cursor == 0 {
        m.plans[config]
            .iter()
            .map(Vec::as_slice)
            .filter(|other| *other != plan)
            .collect()
    } else {
        Vec::new()
    };
    replan_plans.sort_unstable();
    expected_plans.sort_unstable();
    if replan_plans != expected_plans {
        return Some(format!(
            "REPLAN to {} plans, declared every other plan of Pi_tau(G) once ({})",
            replan_plans.len(),
            expected_plans.len()
        ));
    }
    None
}

/// Shared rule of G02 checks 3 and 4 for the toggles of one class: each
/// toggles exactly its candidate bridge, is permitted, never removes a bridge
/// the held plan uses, and every permitted build is present.
fn toggle_violation(
    m: &Model,
    state: usize,
    class: Class,
    permitted: impl Fn(usize) -> bool,
) -> Option<String> {
    let SystemState { config, .. } = m.states[state];
    let plan = m.plan_of(state);
    let count = m.bridge_count();
    let mut builds = vec![false; count];
    for channel in m.channels[state]
        .iter()
        .filter(|channel| channel.class == class)
    {
        let Some(bridge) = channel.bridge.filter(|&bridge| bridge < count) else {
            return Some(format!("{} names no candidate bridge", class.code()));
        };
        let bit = 1_usize << bridge;
        if channel.to_config != config ^ bit {
            return Some(format!(
                "{} does not toggle exactly its bridge",
                class.code()
            ));
        }
        if !permitted(bridge) {
            return Some(format!(
                "{} toggles {} outside its permission",
                class.code(),
                m.config_label(bit)
            ));
        }
        if config & bit == 0 {
            builds[bridge] = true;
        } else if m.uses_bridge(plan, bridge) {
            return Some(format!(
                "{} removes {}, which the held plan uses",
                class.code(),
                m.config_label(bit)
            ));
        }
    }
    (0..count)
        .find(|&bridge| config & (1 << bridge) == 0 && permitted(bridge) && !builds[bridge])
        .map(|bridge| {
            format!(
                "permitted {} build of {} at {} is missing",
                class.code(),
                m.config_label(1 << bridge),
                m.describe_state(state)
            )
        })
}

/// FUEL permission written from section 1.3, independently of the
/// constructor: LOCAL toggles bridges incident to `x(z)`, GLOBAL any.
fn fuel_permitted(m: &Model, state: usize, bridge: usize) -> bool {
    match m.agent {
        Agent::Local => {
            let island = m.position(state);
            let (u, w) = m.bridges[bridge];
            island == u || island == w
        }
        Agent::Global => true,
    }
}

pub(crate) fn gate_02(suite: &Suite) -> GateResult {
    scan_channels(suite, CheckId::G02_DESTINATION_STATES, |m, channel| {
        destination_violation(m, channel).or_else(|| destination_missing(channel))
    })?;
    scan_states(suite, CheckId::G02_STEP_REPLAN, |run, state| {
        step_replan_violation(&run.model, state)
    })?;
    scan_states(suite, CheckId::G02_WEATHER_RULES, |run, state| {
        toggle_violation(&run.model, state, Class::Weather, |_| true)
    })?;
    scan_states(suite, CheckId::G02_FUEL_RULES, |run, state| {
        let m = &run.model;
        toggle_violation(m, state, Class::Fuel, |bridge| {
            fuel_permitted(m, state, bridge)
        })
    })?;
    scan_channels(suite, CheckId::G02_PLAN_CURSOR_KEPT, |m, channel| {
        let SystemState { cursor, .. } = m.states[channel.from];
        let changes =
            channel.to_plan.as_slice() != m.plan_of(channel.from) || channel.to_cursor != cursor;
        (changes && !matches!(channel.class, Class::Step | Class::Replan))
            .then(|| "a toggle changes the plan or the cursor".to_string())
    })?;
    Ok("destinations are system states; STEP/REPLAN/WEATHER/FUEL follow section 1.3".to_string())
}

// ------------------------------------------------------------ G03

/// Rates `(k, k')` of a channel and its reverse declared in section 1.3,
/// written without `channel_rate`.
fn declared_rates(m: &Model, channel: &Channel) -> (u64, u64) {
    let ratio = match channel.class {
        Class::Step | Class::Replan => return (UNIT_RATE, UNIT_RATE),
        Class::Weather => m.spec.b_c,
        Class::Fuel => m.spec.b_h,
    };
    let config = m.states[channel.from].config;
    let builds = channel
        .bridge
        .is_some_and(|bridge| !m.has_bridge(config, bridge));
    if builds {
        (UNIT_RATE, ratio)
    } else {
        (ratio, UNIT_RATE)
    }
}

pub(crate) fn gate_03(suite: &Suite) -> GateResult {
    scan_channels(suite, CheckId::G03_REVERSE_IN_CLASS, |m, channel| {
        reverse_of(m, channel)
            .is_none()
            .then(|| format!("no reverse {} channel", channel.class.code()))
    })?;
    scan_channels(suite, CheckId::G03_RATE_RATIOS, |m, channel| {
        let Some(reverse) = reverse_of(m, channel) else {
            return Some("no reverse channel".to_string());
        };
        let declared = declared_rates(m, channel);
        ((channel.rate, reverse.rate) != declared).then(|| {
            format!(
                "rates {} : {} (forward : reverse), declared {} : {}",
                channel.rate, reverse.rate, declared.0, declared.1
            )
        })
    })?;
    Ok(
        "every channel has a reverse in its class; STEP/REPLAN 1:1, WEATHER 1:b_c, FUEL 1:b_h"
            .to_string(),
    )
}

// ------------------------------------------------------------ G04

fn accounting_violation(m: &Model, channel: &Channel) -> Option<String> {
    let from_config = m.states[channel.from].config;
    let change = m.size(channel.to_config) as i64 - m.size(from_config) as i64;
    match channel.class {
        Class::Step | Class::Replan => {
            (change != 0 || channel.store.is_some() || channel.packet != Packet::Neutral)
                .then(|| "STEP/REPLAN moves a packet or names a store".to_string())
        }
        Class::Weather | Class::Fuel => {
            let store = if channel.class == Class::Weather {
                Store::Cold
            } else {
                Store::Hot
            };
            if channel.store != Some(store) {
                return Some(format!(
                    "{} names store {:?}, declared {}",
                    channel.class.code(),
                    channel.store.map(Store::code),
                    store.code()
                ));
            }
            let toggled = (channel.to_config ^ from_config).count_ones();
            let consistent = toggled == 1
                && matches!(
                    (change, channel.packet),
                    (1, Packet::FromStore) | (-1, Packet::IntoStore)
                );
            (!consistent).then(|| {
                format!(
                    "bridge change {change} with packet {:?} moves other than one packet",
                    channel.packet
                )
            })
        }
    }
}

pub(crate) fn gate_04(suite: &Suite) -> GateResult {
    scan_channels(
        suite,
        CheckId::G04_ONE_PACKET_ONE_STORE,
        accounting_violation,
    )?;
    scan_runs(suite, CheckId::G04_HOT_EQUALS_COLD, |run| {
        let measures = run.measures().map_err(world)?;
        if measures.fuel_numerator == measures.cold_numerator {
            Ok(())
        } else {
            Err(world(format!(
                "net flow from the hot store {} differs from net flow into the cold store {}",
                measures.fuel_flow,
                Rat::new(
                    measures.cold_numerator.clone(),
                    run.steady().map_err(world)?.total.clone()
                )
            )))
        }
    })?;
    Ok(
        "one packet per bridge change with its named store; hot outflow equals cold inflow"
            .to_string(),
    )
}

// ------------------------------------------------------------ G05

pub(crate) fn gate_05(suite: &Suite) -> GateResult {
    scan_runs(suite, CheckId::G05_UNDRIVEN_CONNECTED, |run| {
        let m = &run.model;
        match first_unconnected(m, Class::is_undriven) {
            None => Ok(()),
            Some(state) => Err((
                format!(
                    "{} is not mutually reachable with z0 under STEP, REPLAN, WEATHER",
                    m.describe_state(state)
                ),
                Some(m.states[state].config),
                None,
            )),
        }
    })?;
    for (index, run) in suite.runs.iter().enumerate() {
        let m = &run.model;
        let weights = cold_weights(m);
        let weight = |state: usize| &weights[m.states[state].config];
        for state in 0..m.states.len() {
            let config = Some(m.states[state].config);
            let mut outflow = Big::zero();
            for channel in m.channels[state]
                .iter()
                .filter(|channel| channel.class.is_undriven())
            {
                let Some(to) = channel.to else { continue };
                let Some(reverse) = reverse_of(m, channel) else {
                    return Err(fail(
                        CheckId::G05_COLD_DETAILED_BALANCE,
                        index,
                        config,
                        format!("{}: no reverse channel", m.describe_channel(channel)),
                    ));
                };
                let forward = weight(state).mul_small(channel.rate);
                let backward = weight(to).mul_small(reverse.rate);
                if forward != backward {
                    return Err(fail(
                        CheckId::G05_COLD_DETAILED_BALANCE,
                        index,
                        config,
                        format!(
                            "{}: b_c^(-|G|) fluxes {forward} and {backward} differ",
                            m.describe_channel(channel)
                        ),
                    ));
                }
                outflow = &outflow + &forward;
            }
            let mut inflow = Big::zero();
            for &(from, position) in &m.incoming[state] {
                let channel = &m.channels[from][position];
                if channel.class.is_undriven() {
                    inflow = &inflow + &weight(from).mul_small(channel.rate);
                }
            }
            if inflow != outflow {
                return Err(fail(
                    CheckId::G05_COLD_DETAILED_BALANCE,
                    index,
                    config,
                    format!(
                        "{}: b_c^(-|G|) is not stationary (inflow {inflow}, outflow {outflow})",
                        m.describe_state(state)
                    ),
                ));
            }
        }
    }
    Ok(
        "undriven graph connected; b_c^(-|G|) stationary with detailed balance on every channel"
            .to_string(),
    )
}

// ------------------------------------------------------------ G06

pub(crate) fn gate_06(suite: &Suite) -> GateResult {
    scan_runs(suite, CheckId::G06_DRIVEN_CONNECTED, |run| {
        let m = &run.model;
        match first_unconnected(m, |_| true) {
            None => Ok(()),
            Some(state) => Err((
                format!(
                    "{} is not mutually reachable with z0",
                    m.describe_state(state)
                ),
                Some(m.states[state].config),
                None,
            )),
        }
    })?;
    scan_runs(suite, CheckId::G06_POSITIVE_SOLUTION, |run| {
        let m = &run.model;
        let steady = run.steady().map_err(world)?;
        match steady
            .weights
            .iter()
            .position(|weight| !weight.is_positive())
        {
            None => Ok(()),
            Some(state) => Err((
                format!(
                    "{}: n_z = {} is not positive",
                    m.describe_state(state),
                    steady.weights[state]
                ),
                Some(m.states[state].config),
                None,
            )),
        }
    })?;
    scan_runs(suite, CheckId::G06_FLUX_BALANCE, |run| {
        let m = &run.model;
        let steady = run.steady().map_err(world)?;
        for state in 0..m.states.len() {
            let exit: u64 = m.channels[state]
                .iter()
                .filter(|channel| channel.to.is_some())
                .map(|channel| channel.rate)
                .sum();
            let outflow = steady.weights[state].mul_small(exit);
            let mut inflow = Big::zero();
            for &(from, position) in &m.incoming[state] {
                inflow = &inflow + &steady.weights[from].mul_small(m.channels[from][position].rate);
            }
            if inflow != outflow {
                return Err((
                    format!(
                        "{}: inflow {inflow} differs from outflow {outflow} (units 1/D)",
                        m.describe_state(state)
                    ),
                    Some(m.states[state].config),
                    None,
                ));
            }
        }
        Ok(())
    })?;
    Ok("driven graph connected; exact p strictly positive; every state balanced".to_string())
}

// ------------------------------------------------------------ G07

/// Integer weights `a^|G| b^(K - |G|)` of the product law `q^|G|`,
/// `q = a/b`, per configuration.
fn product_weights(m: &Model, (a, b): (u64, u64)) -> Vec<Big> {
    let k = m.bridge_count();
    (0..m.config_count)
        .map(|config| {
            let size = m.size(config);
            &Big::from_u64(a).pow(size as u32) * &Big::from_u64(b).pow((k - size) as u32)
        })
        .collect()
}

pub(crate) fn gate_07(suite: &Suite) -> GateResult {
    scan_runs(suite, CheckId::G07_PRODUCT_LAW, |run| {
        let m = &run.model;
        if m.agent != Agent::Global {
            return Ok(());
        }
        let steady = run.steady().map_err(world)?;
        let q = m.reference_q();
        let weights = product_weights(m, q);
        let mass = m
            .states
            .iter()
            .fold(Big::zero(), |sum, state| &sum + &weights[state.config]);
        for (state, current) in m.states.iter().enumerate() {
            let measured = &steady.weights[state] * &mass;
            let reference = &weights[current.config] * &steady.total;
            if measured != reference {
                return Err((
                    format!(
                        "{}: p = {} differs from q^|G| / Z with q = {}/{}",
                        m.describe_state(state),
                        Rat::new(steady.weights[state].clone(), steady.total.clone()),
                        q.0,
                        q.1
                    ),
                    Some(current.config),
                    None,
                ));
            }
        }
        Ok(())
    })?;
    scan_runs(suite, CheckId::G07_CLOSED_FORMS, |run| {
        let m = &run.model;
        if m.agent != Agent::Global {
            return Ok(());
        }
        let measures = run.measures().map_err(world)?;
        let weights = product_weights(m, m.reference_q());
        let mut mass = Big::zero();
        let mut open = Big::zero();
        let mut bridges = Big::zero();
        for (state, current) in m.states.iter().enumerate() {
            let weight = &weights[current.config];
            mass = &mass + weight;
            open = &open + &weight.mul_small(m.open_future(state));
            bridges = &bridges + &weight.mul_small(m.size(current.config) as u64);
        }
        let closed_held = &Rat::new(open, mass.clone()) - &undriven_open_future(m);
        let spread = Rat::new(
            Big::from_i64(m.spec.b_c as i64 - m.spec.b_h as i64),
            Big::from_u64(m.spec.b_c + m.spec.b_h),
        );
        let free = &Rat::from_i64(m.bridge_count() as i64) - &Rat::new(bridges, mass);
        let closed_flow = &spread * &free;
        if closed_held != measures.held {
            Err(world(format!(
                "H_global = {} differs from the closed form {closed_held}",
                measures.held
            )))
        } else if closed_flow != measures.fuel_flow {
            Err(world(format!(
                "J_global = {} differs from the closed form {closed_flow}",
                measures.fuel_flow
            )))
        } else {
            Ok(())
        }
    })?;
    Ok("GLOBAL steady states are q^|G| with q = 2/(b_c + b_h); H_global and J_global equal their closed forms".to_string())
}

// ------------------------------------------------------------ G08

/// A channel pair `c: z -> z'`, `c': z' -> z`, visited once from `z < z'`.
struct Pair<'a> {
    from: usize,
    to: usize,
    forward: &'a Channel,
    reverse: &'a Channel,
}

/// Every channel pair of a model in canonical order of the lower state;
/// `Err` names the first channel without a reverse.
fn channel_pairs(m: &Model) -> Result<Vec<Pair<'_>>, usize> {
    let mut pairs = Vec::new();
    for (from, row) in m.channels.iter().enumerate() {
        for channel in row {
            let Some(to) = channel.to else {
                return Err(from);
            };
            if to < from {
                continue;
            }
            let reverse = reverse_of(m, channel).ok_or(from)?;
            pairs.push(Pair {
                from,
                to,
                forward: channel,
                reverse,
            });
        }
    }
    Ok(pairs)
}

/// `ln(k / k')` as an integer multiple of the formal symbol of the pair's
/// store (`L_c` for WEATHER, `L_h` for FUEL); `None` when it is not one.
fn store_symbol_multiple(m: &Model, pair: &Pair) -> Option<i64> {
    let rates = (pair.forward.rate, pair.reverse.rate);
    if rates.0 == rates.1 {
        return Some(0);
    }
    let ratio = match pair.forward.class {
        Class::Step | Class::Replan => return None,
        Class::Weather => m.spec.b_c,
        Class::Fuel => m.spec.b_h,
    };
    if rates == (UNIT_RATE, ratio) {
        Some(-1)
    } else if rates == (ratio, UNIT_RATE) {
        Some(1)
    } else {
        None
    }
}

pub(crate) fn gate_08(suite: &Suite) -> GateResult {
    scan_runs(suite, CheckId::G08_POSITIVE_FUEL_FLOW, |run| {
        let measures = run.measures().map_err(world)?;
        if measures.fuel_flow.is_positive() {
            Ok(())
        } else {
            Err((
                format!("J = {} is not positive", measures.fuel_flow),
                None,
                Some(measures.fuel_flow.clone()),
            ))
        }
    })?;
    scan_runs(suite, CheckId::G08_STORE_SYMBOLS, |run| {
        let m = &run.model;
        let steady = run.steady().map_err(world)?;
        let measures = run.measures().map_err(world)?;
        let pairs = channel_pairs(m).map_err(|state| {
            (
                format!(
                    "{} has a channel without a reverse",
                    m.describe_state(state)
                ),
                Some(m.states[state].config),
                None,
            )
        })?;
        let mut cold = Big::zero();
        let mut hot = Big::zero();
        for pair in &pairs {
            let Some(multiple) = store_symbol_multiple(m, pair) else {
                return Err((
                    format!(
                        "{}: ln({}/{}) is not a multiple of its store symbol",
                        m.describe_channel(pair.forward),
                        pair.forward.rate,
                        pair.reverse.rate
                    ),
                    Some(m.states[pair.from].config),
                    None,
                ));
            };
            if multiple == 0 {
                continue;
            }
            let flow = &steady.weights[pair.from].mul_small(pair.forward.rate)
                - &steady.weights[pair.to].mul_small(pair.reverse.rate);
            let term = &flow * &Big::from_i64(multiple);
            match pair.forward.class {
                Class::Weather => cold = &cold + &term,
                Class::Fuel => hot = &hot + &term,
                Class::Step | Class::Replan => {}
            }
        }
        let expected_hot = -&measures.fuel_numerator;
        if cold == measures.fuel_numerator && hot == expected_hot {
            Ok(())
        } else {
            Err(world(format!(
                "sum [p k - p' k'] ln(k/k') = ({}) L_c + ({}) L_h, J ln(b_c/b_h) = ({}) L_c + ({}) L_h",
                Rat::new(cold, steady.total.clone()),
                Rat::new(hot, steady.total.clone()),
                measures.fuel_flow,
                -&measures.fuel_flow
            )))
        }
    })?;
    scan_runs(suite, CheckId::G08_NONNEGATIVE_TERMS, |run| {
        let m = &run.model;
        let steady = run.steady().map_err(world)?;
        let pairs = channel_pairs(m).map_err(|state| {
            (
                format!(
                    "{} has a channel without a reverse",
                    m.describe_state(state)
                ),
                Some(m.states[state].config),
                None,
            )
        })?;
        let mut net = vec![Big::zero(); m.states.len()];
        for pair in &pairs {
            let forward = steady.weights[pair.from].mul_small(pair.forward.rate);
            let backward = steady.weights[pair.to].mul_small(pair.reverse.rate);
            // (a - b) ln(a/b) >= 0 needs a > 0 and b > 0; then both factors
            // carry the sign of a.cmp(b).
            if !forward.is_positive() || !backward.is_positive() {
                return Err((
                    format!(
                        "{}: p k = {forward}/D or p' k' = {backward}/D is not positive",
                        m.describe_channel(pair.forward)
                    ),
                    Some(m.states[pair.from].config),
                    None,
                ));
            }
            let difference = &forward - &backward;
            net[pair.from] = &net[pair.from] + &difference;
            net[pair.to] = &net[pair.to] - &difference;
        }
        // sum_pairs (a - b)(ln n_z - ln n_z') = sum_z (net outflow) ln n_z
        // vanishes, so sigma = sigma_floor + (fuel terms).
        match net.iter().position(|value| !value.is_zero()) {
            None => Ok(()),
            Some(state) => Err((
                format!(
                    "{}: pair flows leave net {} (units 1/D)",
                    m.describe_state(state),
                    net[state]
                ),
                Some(m.states[state].config),
                None,
            )),
        }
    })?;
    scan_runs(suite, CheckId::G08_CERTIFIED_EFFICIENCY, |run| {
        let floor = run.floor().map_err(world)?;
        let tolerance = width_tolerance();
        if floor.sigma_floor.width() > tolerance {
            Err(world(format!(
                "sigma_floor enclosure {} wider than 10^-9",
                floor.sigma_floor.render(15)
            )))
        } else if floor.eta.width() > tolerance {
            Err(world(format!(
                "eta enclosure {} wider than 10^-9",
                floor.eta.render(15)
            )))
        } else if !floor.eta.strictly_inside(&Rat::zero(), &Rat::one()) {
            Err(world(format!(
                "eta enclosure {} is not strictly inside (0, 1)",
                floor.eta.render(15)
            )))
        } else {
            Ok(())
        }
    })?;
    scan_runs(suite, CheckId::G08_GLOBAL_EFFICIENCY, |run| {
        let m = &run.model;
        if m.agent != Agent::Global {
            return Ok(());
        }
        let floor = run.floor().map_err(world)?;
        let closed = global_efficiency_closed_form(&m.spec)
            .ok_or_else(|| world("ln(b_c/b_h) enclosure contains zero"))?;
        if closed.width() > width_tolerance() {
            Err(world(format!(
                "closed-form eta_global enclosure {} wider than 10^-9",
                closed.render(15)
            )))
        } else if !closed.intersects(&floor.eta) {
            Err(world(format!(
                "closed-form eta_global {} misses eta {}",
                closed.render(15),
                floor.eta.render(15)
            )))
        } else {
            Ok(())
        }
    })?;
    Ok("J > 0; second law in store symbols; nonnegative pair terms; certified eta in (0,1); GLOBAL eta meets its closed form".to_string())
}

// ------------------------------------------------------------ G09

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GraphSubstrateBinding {
    ConfigurationsHeldPlanCursor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CrystalBinding {
    ReplanOverValidPlans,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThermoBinding {
    TwoDeclaredStoresEnergyLambdaBridges,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PraxionBinding {
    BaseActorWithFuelMoves,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EnvironmentBinding {
    Weather,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct XypherSlotReading {
    graph_substrate: GraphSubstrateBinding,
    crystal: CrystalBinding,
    thermo: ThermoBinding,
    praxion: PraxionBinding,
    environment: EnvironmentBinding,
    ruby: Option<()>,
    opal: Option<()>,
}

/// Boundary section 5, G09, and section 11.
const SLOTS: XypherSlotReading = XypherSlotReading {
    graph_substrate: GraphSubstrateBinding::ConfigurationsHeldPlanCursor,
    crystal: CrystalBinding::ReplanOverValidPlans,
    thermo: ThermoBinding::TwoDeclaredStoresEnergyLambdaBridges,
    praxion: PraxionBinding::BaseActorWithFuelMoves,
    environment: EnvironmentBinding::Weather,
    ruby: None,
    opal: None,
};

/// The rate of a channel recomputed from the section 1.3 rule with the
/// declared store ratios only, independently of `channel_rate`.
fn rule_rate(m: &Model, channel: &Channel) -> u64 {
    let ratio = match channel.class {
        Class::Step | Class::Replan => return UNIT_RATE,
        Class::Weather => m.spec.b_c,
        Class::Fuel => m.spec.b_h,
    };
    match channel.bridge {
        Some(bridge) if m.has_bridge(m.states[channel.from].config, bridge) => ratio,
        _ => UNIT_RATE,
    }
}

pub(crate) fn gate_09(suite: &Suite) -> GateResult {
    let named = matches!(
        SLOTS,
        XypherSlotReading {
            graph_substrate: GraphSubstrateBinding::ConfigurationsHeldPlanCursor,
            crystal: CrystalBinding::ReplanOverValidPlans,
            thermo: ThermoBinding::TwoDeclaredStoresEnergyLambdaBridges,
            praxion: PraxionBinding::BaseActorWithFuelMoves,
            environment: EnvironmentBinding::Weather,
            ..
        }
    );
    if !named {
        return Err(fail(
            CheckId::G09_NAMED_SLOTS,
            0,
            None,
            "a G09 slot binding is unbound",
        ));
    }
    scan_states(suite, CheckId::G09_NAMED_SLOTS, |run, state| {
        let m = &run.model;
        if m.stores() != [(Store::Cold, m.spec.b_c), (Store::Hot, m.spec.b_h)] {
            return Some("the world does not declare exactly the cold and hot stores".to_string());
        }
        if m.energy(state) != LAMBDA * m.size(m.states[state].config) as u64 {
            return Some("U(z) differs from lambda |G|".to_string());
        }
        m.channels[state].iter().find_map(|channel| {
            let expected = match channel.class {
                Class::Step | Class::Replan => None,
                Class::Weather => Some(Store::Cold),
                Class::Fuel => Some(Store::Hot),
            };
            (channel.store != expected).then(|| {
                format!(
                    "{}: environment must use the cold store and the Praxion the hot store",
                    m.describe_channel(channel)
                )
            })
        })
    })?;
    if SLOTS.ruby.is_some() || SLOTS.opal.is_some() {
        return Err(fail(
            CheckId::G09_EMPTY_RUBY_ABSENT_OPAL,
            0,
            None,
            "Ruby must be empty and Opal absent",
        ));
    }
    if RATE_READS
        .iter()
        .any(|read| FORBIDDEN_RATE_READS.contains(read))
    {
        return Err(fail(
            CheckId::G09_RATE_INPUTS,
            0,
            None,
            format!("rate reads {RATE_READS:?} include a forbidden quantity"),
        ));
    }
    scan_channels(suite, CheckId::G09_RATE_INPUTS, |m, channel| {
        let expected = rule_rate(m, channel);
        (channel.rate != expected).then(|| {
            format!(
                "rate {} differs from the (state, bridge, class, store ratio) rule {expected}",
                channel.rate
            )
        })
    })?;
    Ok(format!(
        "slots bound; Ruby empty, Opal absent; rates read only {RATE_READS:?} (RateInputs) and equal the section 1.3 rule"
    ))
}

// ------------------------------------------------------------ G10

pub(crate) fn gate_10(suite: &Suite) -> GateResult {
    scan_runs(suite, CheckId::G10_UNDRIVEN_MEAN, |run| {
        let m = &run.model;
        let expected = frozen_undriven_mean(&m.declared)
            .ok_or_else(|| world("no section 4.1 entry for this world"))?;
        let observed = undriven_open_future(m);
        if observed == expected {
            Ok(())
        } else {
            Err(world(format!(
                "<N_2>_(pi_c) = {observed}, section 4.1 states {expected}"
            )))
        }
    })?;
    scan_runs(suite, CheckId::G10_GLOBAL_TABLE, |run| {
        let m = &run.model;
        if m.agent != Agent::Global {
            return Ok(());
        }
        let row = frozen_global_row(&m.declared)
            .ok_or_else(|| world("no section 4.2 row for this world"))?;
        let measures = run.measures().map_err(world)?;
        let (held, flow) = (frozen(row.h), frozen(row.j));
        if measures.held != held {
            return Err(world(format!(
                "H_global = {}, section 4.2 states {held}",
                measures.held
            )));
        }
        if measures.fuel_flow != flow {
            return Err(world(format!(
                "J_global = {}, section 4.2 states {flow}",
                measures.fuel_flow
            )));
        }
        if !measures.fuel_flow.is_positive() {
            return Err(world("J_global is not positive"));
        }
        let observed = &measures.held / &measures.fuel_flow;
        let expected = &held / &flow;
        if observed != expected {
            return Err(world(format!(
                "Y_global = {observed}, H_global / J_global = {expected}"
            )));
        }
        let decimal = observed.round_decimal(4);
        if decimal != row.y {
            return Err(world(format!(
                "Y_global rounds to {decimal}, section 4.2 states {}",
                row.y
            )));
        }
        Ok(())
    })?;
    scan_runs(suite, CheckId::G10_GLOBAL_EFFICIENCY, |run| {
        let m = &run.model;
        if m.agent != Agent::Global {
            return Ok(());
        }
        let expected = frozen_eta_global(&m.declared)
            .ok_or_else(|| world("no section 4.2 eta_global for this pair"))?;
        let floor = run.floor().map_err(world)?;
        let measured = floor.eta.certified_rounding(4);
        if measured.as_deref() != Some(expected) {
            return Err(world(format!(
                "GLOBAL eta {} rounds to {:?}, section 4.2 states {expected}",
                floor.eta.render(15),
                measured
            )));
        }
        let closed = global_efficiency_closed_form(&m.spec)
            .ok_or_else(|| world("ln(b_c/b_h) enclosure contains zero"))?;
        let closed_rounding = closed.certified_rounding(4);
        if closed_rounding.as_deref() != Some(expected) {
            return Err(world(format!(
                "closed-form eta_global {} rounds to {:?}, section 4.2 states {expected}",
                closed.render(15),
                closed_rounding
            )));
        }
        Ok(())
    })?;
    Ok(
        "section 4.1 and 4.2 values reproduced exactly; Y_global and eta_global decimals match"
            .to_string(),
    )
}
