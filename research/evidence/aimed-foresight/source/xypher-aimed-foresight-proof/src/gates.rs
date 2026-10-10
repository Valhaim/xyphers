//! Gates G01--G11 of boundary section 5.
//!
//! A gate runs its checks in the listed order; each check runs over every
//! world in the order of section 3, the agents of a world in the order
//! listed there, and over configurations, states, and channels in canonical
//! order, before the next check begins. A gate reports its first failing
//! check.
//!
//! The steady state is never assumed: it is the exact solution of the
//! orbit-lumped `p Q = 0`, `sum p = 1`, from fraction-free elimination
//! (`src/solve.rs`), as integer orbit totals `n_O`, lifted to integer weights
//! `w(z)` on every state with `p(z) = w(z) / D`. Everything measured is read
//! on the full lifted chain. Logarithms enter only `sigma_floor`, `eta`,
//! `Y_bound`, and `eta_global`, as certified enclosures (`src/logs.rs`). No
//! floating-point value is used.

use std::cell::OnceCell;
use std::collections::VecDeque;

use crate::big::Big;
use crate::logs::{ln_integer, Dyadic, Enclosure};
use crate::model::{
    route_label, Agent, Channel, Class, Model, Mutation, Packet, Store, SystemState, WorldSpec,
    FORBIDDEN_RATE_READS, HOME, LAMBDA, RATE_READS, UNIT_RATE,
};
use crate::rat::Rat;
use crate::rival;
use crate::solve::stationary_vector;
use crate::symmetry::{lift, lumped_rates};
use crate::CheckId;

// ------------------------------------------------------------ frozen values

/// One row of the section 4.1 table, copied verbatim.
struct FrozenWorld {
    n: usize,
    tau: usize,
    states: usize,
    orbits: usize,
    /// `<N_2>` under `pi_c` at `b_c = 3`, `8`, `6`.
    means: [&'static str; 3],
}

/// The `b_c` columns of the section 4.1 table.
const FROZEN_COLD_RATIOS: [u64; 3] = [3, 8, 6];

/// Boundary section 4.1.
const FROZEN_WORLDS: [FrozenWorld; 6] = [
    FrozenWorld {
        n: 3,
        tau: 1,
        states: 48,
        orbits: 28,
        means: ["21/8", "137/81", "93/49"],
    },
    FrozenWorld {
        n: 3,
        tau: 2,
        states: 216,
        orbits: 114,
        means: ["21/8", "137/81", "93/49"],
    },
    FrozenWorld {
        n: 3,
        tau: 3,
        states: 864,
        orbits: 440,
        means: ["21/8", "137/81", "93/49"],
    },
    FrozenWorld {
        n: 4,
        tau: 1,
        states: 512,
        orbits: 120,
        means: ["29/8", "56/27", "118/49"],
    },
    FrozenWorld {
        n: 4,
        tau: 2,
        states: 3072,
        orbits: 612,
        means: ["29/8", "56/27", "118/49"],
    },
    FrozenWorld {
        n: 5,
        tau: 1,
        states: 10240,
        orbits: 660,
        means: ["19/4", "67/27", "145/49"],
    },
];

/// One row of the section 4.2 table, copied verbatim (`J_max` is the
/// closed form `rival::j_max`).
struct FrozenGlobal {
    b_c: u64,
    b_h: u64,
    n: usize,
    h: &'static str,
    j: &'static str,
    y: &'static str,
}

const fn frozen_global(
    (b_c, b_h): (u64, u64),
    n: usize,
    h: &'static str,
    j: &'static str,
    y: &'static str,
) -> FrozenGlobal {
    FrozenGlobal {
        b_c,
        b_h,
        n,
        h,
        j,
        y,
    }
}

/// Boundary section 4.2, rows in order.
const FROZEN_GLOBAL: [FrozenGlobal; 9] = [
    frozen_global((3, 2), 3, "99/392", "3/7", "0.5893"),
    frozen_global((3, 2), 4, "171/392", "6/7", "0.5089"),
    frozen_global((3, 2), 5, "129/196", "10/7", "0.4607"),
    frozen_global((8, 2), 3, "59/162", "3/2", "0.2428"),
    frozen_global((8, 2), 4, "16/27", "3", "0.1975"),
    frozen_global((8, 2), 5, "23/27", "5", "0.1704"),
    frozen_global((6, 3), 3, "1536/5929", "9/11", "0.3166"),
    frozen_global((6, 3), 4, "2529/5929", "18/11", "0.2607"),
    frozen_global((6, 3), 5, "3672/5929", "30/11", "0.2271"),
];

/// Boundary section 4.2: `eta_global` to four decimals per pair.
const FROZEN_ETA_GLOBAL: [((u64, u64), &str); 3] =
    [((3, 2), "0.4497"), ((8, 2), "0.3390"), ((6, 3), "0.4150")];

fn frozen(text: &str) -> Rat {
    Rat::parse(text).expect("frozen values are rationals")
}

fn frozen_world(spec: &WorldSpec) -> Option<&'static FrozenWorld> {
    FROZEN_WORLDS
        .iter()
        .find(|row| row.n == spec.n && row.tau == spec.tau)
}

/// Section 4.1, `<N_2>` under `pi_c` for the world's `(N, tau)` and `b_c`.
fn frozen_undriven_mean(spec: &WorldSpec) -> Option<Rat> {
    let row = frozen_world(spec)?;
    let column = FROZEN_COLD_RATIOS
        .iter()
        .position(|&ratio| ratio == spec.b_c)?;
    Some(frozen(row.means[column]))
}

fn frozen_global_row(spec: &WorldSpec) -> Option<&'static FrozenGlobal> {
    FROZEN_GLOBAL
        .iter()
        .find(|row| (row.b_c, row.b_h, row.n) == (spec.b_c, spec.b_h, spec.n))
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

/// Exact steady state: integer orbit totals `n_O` and the lifted weights
/// `w(z) = n_O (N-1)!/|O|`, with `p(z) = w(z) / D`, `D = sum w`.
#[derive(Clone, Debug)]
pub(crate) struct SteadyState {
    pub(crate) orbit_weights: Vec<Big>,
    pub(crate) weights: Vec<Big>,
    pub(crate) total: Big,
}

/// Section 2 measures, exact, on the full chain.
#[derive(Clone, Debug)]
pub(crate) struct Measures {
    /// `<N_2(G, x)>_p`.
    pub(crate) open_future_held: Rat,
    /// `<N_2(G, x)>_(pi_c)`, from the weights `b_c^(-|G|)`.
    pub(crate) open_future_cold: Rat,
    /// `H = <N_2>_p - <N_2>_(pi_c)`.
    pub(crate) held: Rat,
    /// `J`: net packets per unit time from the hot store into the bridges.
    pub(crate) fuel_flow: Rat,
    /// `J D`.
    pub(crate) fuel_numerator: Big,
    /// Net packets per unit time into the cold store, times `D`.
    pub(crate) cold_numerator: Big,
    /// `<|G|>_p D`.
    pub(crate) bridge_numerator: Big,
    /// `rho = <|G|>_p / K`.
    pub(crate) density: Rat,
}

/// Certified enclosures of the price floor (section 2).
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

    /// `J_max = K (b_c - b_h)/(b_h + 1)` of the constructed world.
    pub(crate) fn j_max(&self) -> Rat {
        let spec = &self.model.spec;
        rival::j_max(spec.bridge_count(), spec.b_c, spec.b_h)
    }

    /// `s*(J)` when `J < J_max`, `Ok(None)` otherwise; `Err` when the
    /// inverse is undefined.
    pub(crate) fn rival_rate(&self) -> Result<Option<Rat>, String> {
        let measures = self.measures()?;
        if measures.fuel_flow >= self.j_max() {
            return Ok(None);
        }
        let m = &self.model;
        rival::rival_rate(
            m.spec.bridge_count(),
            m.spec.b_c,
            m.spec.b_h,
            m.rival_hot_weight(),
            &measures.fuel_flow,
        )
        .map(Some)
        .ok_or_else(|| "s*(J) undefined (zero denominator)".to_string())
    }

    /// `H_warm = Phi_N(rho) - Phi_N(r_0)`.
    pub(crate) fn warm_held(&self) -> Result<Rat, String> {
        let measures = self.measures()?;
        let spec = &self.model.spec;
        Ok(&rival::phi(spec.n, &measures.density)
            - &rival::phi(spec.n, &rival::cold_density(spec.b_c)))
    }

    /// `Y = H / J`, defined when `J != 0`.
    pub(crate) fn fuel_yield(&self) -> Result<Rat, String> {
        let measures = self.measures()?;
        if measures.fuel_flow.is_zero() {
            return Err("undefined (J = 0)".to_string());
        }
        Ok(&measures.held / &measures.fuel_flow)
    }

    /// `E = H / H_warm`, defined when `H_warm != 0`.
    pub(crate) fn edge(&self) -> Result<Rat, String> {
        let measures = self.measures()?;
        let warm = self.warm_held()?;
        if warm.is_zero() {
            return Err("undefined (H_warm = 0)".to_string());
        }
        Ok(&measures.held / &warm)
    }
}

/// Every world of a list with each of its agents, in section 3 order.
pub(crate) struct Suite {
    pub(crate) runs: Vec<Run>,
}

impl Suite {
    pub(crate) fn build(worlds: &[WorldSpec], mutation: Mutation) -> Self {
        let runs = worlds
            .iter()
            .flat_map(|&world| {
                world
                    .agents()
                    .into_iter()
                    .map(move |agent| Run::new(Model::build(world, agent, mutation)))
            })
            .collect();
        Self { runs }
    }

    /// The run of a declared world with an agent.
    pub(crate) fn run(&self, world: &WorldSpec, agent: Agent) -> Option<&Run> {
        self.runs
            .iter()
            .find(|run| run.model.declared == *world && run.model.agent == agent)
    }
}

fn solve_steady_state(m: &Model) -> Result<SteadyState, String> {
    let rates = lumped_rates(m);
    let orbit_weights = stationary_vector(m.orbits.count(), &rates)
        .ok_or_else(|| "the lumped p Q = 0 with sum p = 1 is singular".to_string())?;
    let weights = lift(m, &orbit_weights)?;
    let total = weights
        .iter()
        .fold(Big::zero(), |sum, weight| &sum + weight);
    if !total.is_positive() {
        return Err(format!("normalisation D = {total} is not positive"));
    }
    Ok(SteadyState {
        orbit_weights,
        weights,
        total,
    })
}

/// Integer weights `b_c^(K - |G|)` of `pi_c` per configuration.
fn cold_weights(m: &Model) -> Vec<Big> {
    let base = Big::from_u64(m.spec.b_c);
    (0..m.config_count)
        .map(|config| base.pow((m.bridge_count() - m.size(config)) as u32))
        .collect()
}

/// `<N_2(G, x)>` under `pi_c(z) ~ b_c^(-|G|)`, summed over system states.
pub(crate) fn undriven_open_future(m: &Model) -> Rat {
    let weights = cold_weights(m);
    let mut mass = Big::zero();
    let mut open = Big::zero();
    for (state, current) in m.states.iter().enumerate() {
        let weight = &weights[current.config];
        mass = &mass + weight;
        open = &open + &weight.mul_small(m.open_future_at(state));
    }
    Rat::new(open, mass)
}

fn compute_measures(m: &Model, steady: &SteadyState) -> Measures {
    let mut fuel = Big::zero();
    let mut cold = Big::zero();
    let mut open = Big::zero();
    let mut bridges = Big::zero();
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
        open = &open + &weight.mul_small(m.open_future_at(state));
        bridges = &bridges + &weight.mul_small(m.size(m.states[state].config) as u64);
    }
    let open_future_held = Rat::new(open, steady.total.clone());
    let open_future_cold = undriven_open_future(m);
    let held = &open_future_held - &open_future_cold;
    let density = Rat::new(
        bridges.clone(),
        steady.total.mul_small(m.bridge_count() as u64),
    );
    Measures {
        open_future_held,
        open_future_cold,
        held,
        fuel_flow: Rat::new(fuel.clone(), steady.total.clone()),
        fuel_numerator: fuel,
        cold_numerator: cold,
        bridge_numerator: bridges,
        density,
    }
}

/// `sigma_floor = -(1/D) sum_z m_z [ln w(z) + |G(z)| ln b_c]` with
/// `m_z = D (p L_undriven)(z)` on the full chain, and
/// `eta = sigma_floor / sigma` with `sigma = J ln(b_c / b_h)`. `ln w(z)` is
/// enclosed once per orbit, where `w` is constant.
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
    let mut logs: Vec<Option<Dyadic>> = vec![None; m.orbits.count()];
    let mut sum = Dyadic::zero();
    let mut bridge_weight = Big::zero();
    for (state, weight) in balance.iter().enumerate() {
        if weight.is_zero() {
            continue;
        }
        let n = &steady.weights[state];
        if !n.is_positive() {
            return Err(format!(
                "w(z) = {n} is not positive at {}",
                m.describe_state(state)
            ));
        }
        let orbit = m.orbits.of_state[state];
        let log = logs[orbit].get_or_insert_with(|| ln_integer(n));
        sum.add_scaled(weight, log);
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
    mut test: impl FnMut(&Model, usize) -> Option<String>,
) -> Result<(), Failure> {
    for (index, run) in suite.runs.iter().enumerate() {
        let m = &run.model;
        for state in 0..m.states.len() {
            if let Some(detail) = test(m, state) {
                return Err(fail(check, index, Some(m.states[state].config), detail));
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
    scan_states(suite, check, |m, state| {
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

/// Failure at the configuration of a state.
fn at_state(m: &Model, state: usize, detail: String) -> (String, Option<usize>, Option<Rat>) {
    (detail, Some(m.states[state].config), None)
}

/// The reverse of a channel: the channel of the same class, on the same
/// bridge, with the same store, from its destination back to its source.
fn reverse_of<'a>(m: &'a Model, channel: &Channel) -> Option<&'a Channel> {
    let to = channel.to?;
    m.channels[to].iter().find(|candidate| {
        candidate.to == Some(channel.from)
            && candidate.class == channel.class
            && candidate.bridge == channel.bridge
            && candidate.store == channel.store
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
    let mut queue = VecDeque::from([root]);
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

/// `V^tau` in lexicographic order by depth-first enumeration, as walks from
/// home, written independently of the constructor.
fn depth_first_routes(n: usize, tau: usize) -> Vec<Vec<usize>> {
    fn extend(n: usize, tau: usize, prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if prefix.len() == tau + 1 {
            out.push(prefix.clone());
            return;
        }
        for next in 0..n {
            prefix.push(next);
            extend(n, tau, prefix, out);
            prefix.pop();
        }
    }
    let mut out = Vec::new();
    extend(n, tau, &mut vec![HOME], &mut out);
    out
}

/// Two-step walks from `x`, each step a stay or a crossing of a bridge in
/// `G`, by depth-first enumeration.
fn walks_by_enumeration(m: &Model, config: usize, x: usize, steps: usize) -> u64 {
    if steps == 0 {
        return 1;
    }
    (0..m.spec.n)
        .filter(|&next| {
            next == x
                || m.bridge_between(x, next)
                    .is_some_and(|bridge| m.has_bridge(config, bridge))
        })
        .map(|next| walks_by_enumeration(m, config, next, steps - 1))
        .sum()
}

pub(crate) fn gate_01(suite: &Suite) -> GateResult {
    scan_runs(suite, CheckId::G01_ROUTE_SET, |run| {
        let m = &run.model;
        let (n, tau) = (m.spec.n, m.spec.tau);
        let expected = depth_first_routes(n, tau);
        if m.routes.len() != n.pow(tau as u32) {
            return Err(world(format!(
                "{} routes, N^tau = {}",
                m.routes.len(),
                n.pow(tau as u32)
            )));
        }
        if !m.routes.windows(2).all(|pair| pair[0] < pair[1]) {
            return Err(world("routes are not in strictly lexicographic order"));
        }
        match m
            .routes
            .iter()
            .zip(&expected)
            .position(|(built, enumerated)| built != enumerated)
        {
            None if m.routes.len() == expected.len() => Ok(()),
            None => Err(world("route count differs from V^tau")),
            Some(index) => Err(world(format!(
                "route {index} is {}, V^tau in lexicographic order has {}",
                route_label(&m.routes[index]),
                route_label(&expected[index])
            ))),
        }
    })?;
    scan_configurations(suite, CheckId::G01_WALK_COUNTS, |m, config| {
        (0..m.spec.n).find_map(|x| {
            let enumerated = walks_by_enumeration(m, config, x, 2);
            let matrix = m.open_future[config][x];
            (enumerated != matrix).then(|| {
                format!(
                    "island {x}: {enumerated} two-step walks by enumeration, e_x^T (I + A_G)^2 1 = {matrix}"
                )
            })
        })
    })?;
    scan_runs(suite, CheckId::G01_STATE_ORBIT_COUNTS, |run| {
        let m = &run.model;
        let Some(row) = frozen_world(&m.declared) else {
            return Err(world("no section 4.1 row for this world"));
        };
        if m.states.len() != row.states {
            Err(world(format!(
                "{} system states, section 4.1 states {}",
                m.states.len(),
                row.states
            )))
        } else if m.orbits.count() != row.orbits {
            Err(world(format!(
                "{} orbits, section 4.1 states {}",
                m.orbits.count(),
                row.orbits
            )))
        } else {
            Ok(())
        }
    })?;
    Ok("routes are V^tau in lexicographic order; N_2 by enumeration equals e_x^T (I+A_G)^2 1; state and orbit counts match section 4.1".to_string())
}

// ------------------------------------------------------------ G02

/// Step `step` of the held route at `state` is open in `config`: it stays,
/// or it crosses a bridge of `config` (section 1.3, written independently of
/// the constructor).
fn step_permitted(m: &Model, state: usize, config: usize, step: usize) -> bool {
    let walk = m.walk_of(state);
    let (a, b) = (walk[step], walk[step + 1]);
    a == b
        || m.bridge_between(a, b)
            .is_some_and(|bridge| m.has_bridge(config, bridge))
}

/// The agent's declared `A(r, t)` (section 1.4), written independently of
/// the constructor and of its control hooks.
pub(crate) fn declared_permission(m: &Model, state: usize) -> usize {
    let walk = m.walk_of(state);
    let t = m.states[state].cursor;
    let tau = m.spec.tau;
    let mut mask = 0_usize;
    match m.agent {
        Agent::Local => {
            let x = walk[t];
            for (bridge, &(u, w)) in m.bridges.iter().enumerate() {
                if u == x || w == x {
                    mask |= 1 << bridge;
                }
            }
        }
        Agent::Aim(k) => {
            // Moving steps max(0, t - k), ..., min(tau, t + k) - 1.
            for step in t.saturating_sub(k)..(t + k).min(tau) {
                if walk[step] != walk[step + 1] {
                    if let Some(bridge) = m.bridge_between(walk[step], walk[step + 1]) {
                        mask |= 1 << bridge;
                    }
                }
            }
        }
        Agent::Global => {
            for bridge in 0..m.bridge_count() {
                mask |= 1 << bridge;
            }
        }
    }
    mask
}

fn destination_violation(m: &Model, channel: &Channel) -> Option<String> {
    let Some(to) = channel.to else {
        return Some("destination is not a system state of the world".to_string());
    };
    let resolved = m.states.get(to)?;
    (*resolved
        != SystemState {
            config: channel.to_config,
            route: channel.to_route,
            cursor: channel.to_cursor,
        })
    .then(|| "resolved state differs from the destination description".to_string())
}

fn step_replan_violation(m: &Model, state: usize) -> Option<String> {
    let SystemState {
        config,
        route,
        cursor,
    } = m.states[state];
    let tau = m.spec.tau;
    let mut step_cursors: Vec<usize> = Vec::new();
    let mut replan_routes: Vec<usize> = Vec::new();
    for channel in &m.channels[state] {
        match channel.class {
            Class::Step => {
                if channel.to_config != config || channel.to_route != route {
                    return Some("STEP changes the configuration or the route".to_string());
                }
                step_cursors.push(channel.to_cursor);
            }
            Class::Replan => {
                if channel.to_config != config || channel.to_cursor != cursor {
                    return Some("REPLAN changes the configuration or the cursor".to_string());
                }
                replan_routes.push(channel.to_route);
            }
            Class::Weather | Class::Fuel => {}
        }
    }
    let mut expected_cursors = Vec::new();
    if cursor < tau && step_permitted(m, state, config, cursor) {
        expected_cursors.push(cursor + 1);
    }
    if cursor > 0 && step_permitted(m, state, config, cursor - 1) {
        expected_cursors.push(cursor - 1);
    }
    step_cursors.sort_unstable();
    expected_cursors.sort_unstable();
    if step_cursors != expected_cursors {
        return Some(format!(
            "STEP to cursors {step_cursors:?}, section 1.3 permits {expected_cursors:?}"
        ));
    }
    let expected_routes: Vec<usize> = if cursor == 0 {
        (0..m.routes.len())
            .filter(|&other| other != route)
            .collect()
    } else {
        Vec::new()
    };
    replan_routes.sort_unstable();
    if replan_routes != expected_routes {
        return Some(format!(
            "REPLAN to {} routes at cursor {cursor}, declared {} (every other route at t = 0 only)",
            replan_routes.len(),
            expected_routes.len()
        ));
    }
    None
}

fn weather_violation(m: &Model, state: usize) -> Option<String> {
    let config = m.states[state].config;
    let count = m.bridge_count();
    let mut seen = vec![0_usize; count];
    for channel in m.channels[state]
        .iter()
        .filter(|channel| channel.class == Class::Weather)
    {
        let Some(bridge) = channel.bridge.filter(|&bridge| bridge < count) else {
            return Some("WEATHER names no candidate bridge".to_string());
        };
        if channel.to_config != config ^ (1 << bridge) {
            return Some(format!(
                "WEATHER on {} does not toggle exactly its bridge",
                m.config_label(1 << bridge)
            ));
        }
        seen[bridge] += 1;
    }
    (0..count).find_map(|bridge| {
        let kind = if m.has_bridge(config, bridge) {
            "wear"
        } else {
            "build"
        };
        match seen[bridge] {
            1 => None,
            0 => Some(format!(
                "WEATHER {kind} of {} at {} is missing",
                m.config_label(1 << bridge),
                m.describe_state(state)
            )),
            more => Some(format!(
                "{more} WEATHER channels on {} at {}",
                m.config_label(1 << bridge),
                m.describe_state(state)
            )),
        }
    })
}

fn fuel_violation(m: &Model, state: usize) -> Option<String> {
    let config = m.states[state].config;
    let count = m.bridge_count();
    let declared = declared_permission(m, state);
    let mut builds = vec![false; count];
    for channel in m.channels[state]
        .iter()
        .filter(|channel| channel.class == Class::Fuel)
    {
        let Some(bridge) = channel.bridge.filter(|&bridge| bridge < count) else {
            return Some("FUEL names no candidate bridge".to_string());
        };
        let bit = 1_usize << bridge;
        if channel.to_config != config ^ bit {
            return Some(format!(
                "FUEL on {} does not toggle exactly its bridge",
                m.config_label(bit)
            ));
        }
        if declared & bit == 0 {
            return Some(format!(
                "FUEL toggles {} at {}, outside the declared A(r, t) = {}",
                m.config_label(bit),
                m.describe_state(state),
                m.config_label(declared)
            ));
        }
        if config & bit == 0 {
            builds[bridge] = true;
        }
    }
    (0..count)
        .find(|&bridge| {
            declared & (1 << bridge) != 0 && !m.has_bridge(config, bridge) && !builds[bridge]
        })
        .map(|bridge| {
            format!(
                "FUEL build of {} in A(r, t) at {} is missing",
                m.config_label(1 << bridge),
                m.describe_state(state)
            )
        })
}

pub(crate) fn gate_02(suite: &Suite) -> GateResult {
    scan_channels(
        suite,
        CheckId::G02_DESTINATION_STATES,
        destination_violation,
    )?;
    scan_states(suite, CheckId::G02_STEP_REPLAN, step_replan_violation)?;
    scan_states(suite, CheckId::G02_WEATHER_RULES, weather_violation)?;
    scan_states(suite, CheckId::G02_FUEL_RULES, fuel_violation)?;
    scan_channels(suite, CheckId::G02_CLASS_CHANGES, |m, channel| {
        let SystemState {
            config,
            route,
            cursor,
        } = m.states[channel.from];
        if channel.to_cursor != cursor && channel.class != Class::Step {
            Some("only STEP may change the cursor".to_string())
        } else if channel.to_route != route && channel.class != Class::Replan {
            Some("only REPLAN may change the route".to_string())
        } else if channel.to_config != config
            && !matches!(channel.class, Class::Weather | Class::Fuel)
        {
            Some("only WEATHER and FUEL may change the configuration".to_string())
        } else {
            None
        }
    })?;
    Ok(
        "destinations are states; STEP, REPLAN, WEATHER, FUEL follow sections 1.3 and 1.4"
            .to_string(),
    )
}

// ------------------------------------------------------------ G03

/// Whether a toggle channel builds its bridge at its source state.
fn builds(m: &Model, channel: &Channel) -> bool {
    channel
        .bridge
        .is_some_and(|bridge| !m.has_bridge(m.states[channel.from].config, bridge))
}

/// Rates `(k, k')` of a channel and its reverse declared in section 1.3,
/// written without `channel_rate`.
fn declared_rates(m: &Model, channel: &Channel) -> (u64, u64) {
    let ratio = match channel.class {
        Class::Step | Class::Replan => return (UNIT_RATE, UNIT_RATE),
        Class::Weather => m.spec.b_c,
        Class::Fuel => m.spec.b_h,
    };
    if builds(m, channel) {
        (UNIT_RATE, ratio)
    } else {
        (ratio, UNIT_RATE)
    }
}

pub(crate) fn gate_03(suite: &Suite) -> GateResult {
    scan_channels(suite, CheckId::G03_REVERSE_IN_CLASS, |m, channel| {
        reverse_of(m, channel).is_none().then(|| {
            format!(
                "no reverse {} channel on the same bridge with the same store",
                channel.class.code()
            )
        })
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
    Ok("every channel has a reverse in its class, bridge, and store; STEP/REPLAN 1:1, WEATHER 1:b_c, FUEL 1:b_h".to_string())
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
            let total = &run.steady().map_err(world)?.total;
            Err(world(format!(
                "net flow from the hot store {} differs from net flow into the cold store {}",
                measures.fuel_flow,
                Rat::new(measures.cold_numerator.clone(), total.clone())
            )))
        }
    })?;
    scan_runs(suite, CheckId::G04_BRIDGE_FLOW_IDENTITY, |run| {
        let m = &run.model;
        let measures = run.measures().map_err(world)?;
        let total = &run.steady().map_err(world)?.total;
        // J D = (b_c + 1) <|G|>_p D - K D, in integers.
        let identity = &measures.bridge_numerator.mul_small(m.spec.b_c + 1)
            - &total.mul_small(m.bridge_count() as u64);
        if identity == measures.fuel_numerator {
            Ok(())
        } else {
            Err(world(format!(
                "J = {}, (b_c + 1)<|G|>_p - K = {}",
                measures.fuel_flow,
                Rat::new(identity, total.clone())
            )))
        }
    })?;
    Ok("one packet per bridge change with its named store; hot outflow equals cold inflow; J = (b_c+1)<|G|>_p - K".to_string())
}

// ------------------------------------------------------------ G05

pub(crate) fn gate_05(suite: &Suite) -> GateResult {
    scan_runs(suite, CheckId::G05_UNDRIVEN_CONNECTED, |run| {
        let m = &run.model;
        match first_unconnected(m, Class::is_undriven) {
            None => Ok(()),
            Some(state) => Err(at_state(
                m,
                state,
                format!(
                    "{} is not mutually reachable with z0 under STEP, REPLAN, WEATHER",
                    m.describe_state(state)
                ),
            )),
        }
    })?;
    for (index, run) in suite.runs.iter().enumerate() {
        let m = &run.model;
        let weights = cold_weights(m);
        let weight = |state: usize| &weights[m.states[state].config];
        for state in 0..m.states.len() {
            for channel in m.channels[state]
                .iter()
                .filter(|channel| channel.class.is_undriven())
            {
                let Some(to) = channel.to else { continue };
                let detail = match reverse_of(m, channel) {
                    None => Some("no reverse channel".to_string()),
                    Some(reverse) => {
                        let forward = weight(state).mul_small(channel.rate);
                        let backward = weight(to).mul_small(reverse.rate);
                        (forward != backward)
                            .then(|| format!("b_c^(-|G|) fluxes {forward} and {backward} differ"))
                    }
                };
                if let Some(detail) = detail {
                    return Err(fail(
                        CheckId::G05_COLD_DETAILED_BALANCE,
                        index,
                        Some(m.states[state].config),
                        format!("{}: {detail}", m.describe_channel(channel)),
                    ));
                }
            }
        }
    }
    Ok("undriven graph strongly connected; b_c^(-|G|) in detailed balance on every undriven channel".to_string())
}

// ------------------------------------------------------------ G06

/// Whether the image of `channel` under relabelling `index` is a channel of
/// the same class, store, and rate on the image bridge.
fn image_violation(m: &Model, channel: &Channel, index: usize) -> Option<String> {
    let relabelling = &m.relabellings[index];
    let source = m.image_state(relabelling, channel.from);
    let config = relabelling.configs.get(channel.to_config)?;
    let route = relabelling.routes.get(channel.to_route)?;
    let bridge = match channel.bridge {
        Some(bridge) => Some(*relabelling.bridges.get(bridge)?),
        None => None,
    };
    let present = m.channels[source].iter().any(|candidate| {
        candidate.to_config == *config
            && candidate.to_route == *route
            && candidate.to_cursor == channel.to_cursor
            && candidate.class == channel.class
            && candidate.store == channel.store
            && candidate.rate == channel.rate
            && candidate.bridge == bridge
    });
    (!present).then(|| {
        format!(
            "relabelling {:?} has no image channel of the same class, store, and rate at {}",
            relabelling.islands,
            m.describe_state(source)
        )
    })
}

pub(crate) fn gate_06(suite: &Suite) -> GateResult {
    scan_runs(suite, CheckId::G06_DRIVEN_CONNECTED, |run| {
        let m = &run.model;
        match first_unconnected(m, |_| true) {
            None => Ok(()),
            Some(state) => Err(at_state(
                m,
                state,
                format!(
                    "{} is not mutually reachable with z0",
                    m.describe_state(state)
                ),
            )),
        }
    })?;
    scan_channels(suite, CheckId::G06_RELABELLING_SYMMETRY, |m, channel| {
        (0..m.relabellings.len()).find_map(|index| image_violation(m, channel, index))
    })?;
    scan_runs(suite, CheckId::G06_POSITIVE_ORBIT_SOLUTION, |run| {
        let m = &run.model;
        let steady = run.steady().map_err(world)?;
        match steady
            .orbit_weights
            .iter()
            .position(|weight| !weight.is_positive())
        {
            None => Ok(()),
            Some(orbit) => {
                let representative = m.orbits.representatives[orbit];
                Err(at_state(
                    m,
                    representative,
                    format!(
                        "orbit of {}: n_O = {} is not positive",
                        m.describe_state(representative),
                        steady.orbit_weights[orbit]
                    ),
                ))
            }
        }
    })?;
    scan_runs(suite, CheckId::G06_LIFTED_BALANCE, |run| {
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
                return Err(at_state(
                    m,
                    state,
                    format!(
                        "{}: inflow {inflow} differs from outflow {outflow} (units 1/D)",
                        m.describe_state(state)
                    ),
                ));
            }
        }
        Ok(())
    })?;
    Ok("driven graph strongly connected; every channel symmetric under relabelling; orbit solution positive; lifted law balances every state".to_string())
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
                return Err(at_state(
                    m,
                    state,
                    format!(
                        "{}: p = {} differs from q^|G| / Z with q = {}/{}",
                        m.describe_state(state),
                        Rat::new(steady.weights[state].clone(), steady.total.clone()),
                        q.0,
                        q.1
                    ),
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
        let spec = &m.spec;
        let held = rival::global_held(spec.n, spec.b_c, spec.b_h);
        let flow = rival::global_flow(spec.bridge_count(), spec.b_c, spec.b_h);
        if measures.held != held {
            Err(world(format!(
                "H_global = {} differs from the closed form {held}",
                measures.held
            )))
        } else if measures.fuel_flow != flow {
            Err(world(format!(
                "J_global = {} differs from the closed form {flow}",
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
/// `Err` names the first state with a channel without a reverse.
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

fn pairs_or_failure(m: &Model) -> Result<Vec<Pair<'_>>, (String, Option<usize>, Option<Rat>)> {
    channel_pairs(m).map_err(|state| {
        at_state(
            m,
            state,
            format!(
                "{} has a channel without a reverse",
                m.describe_state(state)
            ),
        )
    })
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
        let pairs = pairs_or_failure(m)?;
        let mut cold = Big::zero();
        let mut hot = Big::zero();
        for pair in &pairs {
            let Some(multiple) = store_symbol_multiple(m, pair) else {
                return Err(at_state(
                    m,
                    pair.from,
                    format!(
                        "{}: ln({}/{}) is not a multiple of its store symbol",
                        m.describe_channel(pair.forward),
                        pair.forward.rate,
                        pair.reverse.rate
                    ),
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
        let pairs = pairs_or_failure(m)?;
        let mut net = vec![Big::zero(); m.states.len()];
        for pair in &pairs {
            let forward = steady.weights[pair.from].mul_small(pair.forward.rate);
            let backward = steady.weights[pair.to].mul_small(pair.reverse.rate);
            // (a - b) ln(a/b) >= 0 needs a > 0 and b > 0; then both factors
            // carry the sign of a.cmp(b).
            if !forward.is_positive() || !backward.is_positive() {
                return Err(at_state(
                    m,
                    pair.from,
                    format!(
                        "{}: p k = {forward}/D or p' k' = {backward}/D is not positive",
                        m.describe_channel(pair.forward)
                    ),
                ));
            }
            let difference = &forward - &backward;
            net[pair.from] = &net[pair.from] + &difference;
            net[pair.to] = &net[pair.to] - &difference;
        }
        // sum_pairs (a - b)(ln w(z) - ln w(z')) = sum_z (net outflow) ln w(z)
        // vanishes, so sigma = sigma_floor + (FUEL terms), each term a
        // nonnegative (a - b) ln(a/b).
        match net.iter().position(|value| !value.is_zero()) {
            None => Ok(()),
            Some(state) => Err(at_state(
                m,
                state,
                format!(
                    "{}: pair flows leave net {} (units 1/D)",
                    m.describe_state(state),
                    net[state]
                ),
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

pub(crate) fn gate_09(suite: &Suite) -> GateResult {
    scan_runs(suite, CheckId::G09_RIVAL_INVERSE, |run| {
        let spec = &run.model.spec;
        let measures = run.measures().map_err(world)?;
        let Some(s) = run.rival_rate().map_err(world)? else {
            return Ok(());
        };
        if !s.is_positive() {
            return Err(world(format!("s*(J) = {s} is not positive")));
        }
        let flow = rival::warming_flow(spec.bridge_count(), spec.b_c, spec.b_h, &s)
            .ok_or_else(|| world(format!("J_warm(s*) undefined at s* = {s}")))?;
        if flow != measures.fuel_flow {
            return Err(world(format!(
                "J_warm(s*) = {flow} differs from J = {} at s* = {s}",
                measures.fuel_flow
            )));
        }
        let density = rival::warming_density(spec.b_c, spec.b_h, &s)
            .ok_or_else(|| world(format!("r_s undefined at s* = {s}")))?;
        if density != measures.density {
            return Err(world(format!(
                "r_(s*) = {density} differs from rho = {}",
                measures.density
            )));
        }
        Ok(())
    })?;
    scan_runs(suite, CheckId::G09_GLOBAL_RIVAL, |run| {
        let m = &run.model;
        if m.agent != Agent::Global {
            return Ok(());
        }
        let measures = run.measures().map_err(world)?;
        let expected = rival::global_density(m.spec.b_c, m.spec.b_h);
        if measures.density != expected {
            return Err(world(format!(
                "rho_global = {}, declared 2/(b_c + b_h + 2) = {expected}",
                measures.density
            )));
        }
        match run.rival_rate().map_err(world)? {
            Some(s) if s == Rat::one() => {}
            Some(s) => return Err(world(format!("s*_global = {s}, declared 1"))),
            None => return Err(world("s*_global is none (J_global >= J_max)")),
        }
        let edge = run.edge().map_err(world)?;
        if edge != Rat::one() {
            return Err(world(format!("E_global = {edge}, declared 1")));
        }
        Ok(())
    })?;
    Ok("s*(J) > 0 inverts J_warm with r_(s*) = rho wherever J < J_max; GLOBAL has rho = 2/(b_c+b_h+2), s* = 1, E = 1".to_string())
}

// ------------------------------------------------------------ G10

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GraphSubstrateBinding {
    ConfigurationRouteCursor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CrystalBinding {
    ReplanOverAllRoutes,
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

/// Boundary section 5, G10, and section 11.
const SLOTS: XypherSlotReading = XypherSlotReading {
    graph_substrate: GraphSubstrateBinding::ConfigurationRouteCursor,
    crystal: CrystalBinding::ReplanOverAllRoutes,
    thermo: ThermoBinding::TwoDeclaredStoresEnergyLambdaBridges,
    praxion: PraxionBinding::BaseActorWithFuelMoves,
    environment: EnvironmentBinding::Weather,
    ruby: None,
    opal: None,
};

/// The rate of a channel recomputed from the section 1.3 rule with the
/// declared store ratios and the declared `A(r, t)` only, independently of
/// `channel_rate`; `None` when the rule grants no such channel.
fn rule_rate(m: &Model, channel: &Channel) -> Option<u64> {
    let ratio = match channel.class {
        Class::Step | Class::Replan => return Some(UNIT_RATE),
        Class::Weather => m.spec.b_c,
        Class::Fuel => m.spec.b_h,
    };
    let bridge = channel.bridge?;
    if channel.class == Class::Fuel && declared_permission(m, channel.from) & (1 << bridge) == 0 {
        return None;
    }
    Some(if builds(m, channel) { UNIT_RATE } else { ratio })
}

pub(crate) fn gate_10(suite: &Suite) -> GateResult {
    let named = matches!(
        SLOTS,
        XypherSlotReading {
            graph_substrate: GraphSubstrateBinding::ConfigurationRouteCursor,
            crystal: CrystalBinding::ReplanOverAllRoutes,
            thermo: ThermoBinding::TwoDeclaredStoresEnergyLambdaBridges,
            praxion: PraxionBinding::BaseActorWithFuelMoves,
            environment: EnvironmentBinding::Weather,
            ..
        }
    );
    if !named {
        return Err(fail(
            CheckId::G10_NAMED_SLOTS,
            0,
            None,
            "a G10 slot binding is unbound",
        ));
    }
    scan_states(suite, CheckId::G10_NAMED_SLOTS, |m, state| {
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
                    "{}: the environment must use the cold store and the Praxion the hot store",
                    m.describe_channel(channel)
                )
            })
        })
    })?;
    if SLOTS.ruby.is_some() || SLOTS.opal.is_some() {
        return Err(fail(
            CheckId::G10_EMPTY_RUBY_ABSENT_OPAL,
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
            CheckId::G10_RATE_INPUTS,
            0,
            None,
            format!("rate reads {RATE_READS:?} include a forbidden quantity"),
        ));
    }
    scan_channels(
        suite,
        CheckId::G10_RATE_INPUTS,
        |m, channel| match rule_rate(m, channel) {
            Some(expected) if expected == channel.rate => None,
            Some(expected) => Some(format!(
            "rate {} differs from the (state, bridge, class, store ratio, A(r, t)) rule {expected}",
            channel.rate
        )),
            None => Some("the section 1.3 rule grants no such channel".to_string()),
        },
    )?;
    Ok(format!(
        "slots bound; Ruby empty, Opal absent; rates read only {RATE_READS:?} (RateInputs) and equal the section 1.3 rule"
    ))
}

// ------------------------------------------------------------ G11

pub(crate) fn gate_11(suite: &Suite) -> GateResult {
    scan_runs(suite, CheckId::G11_UNDRIVEN_MEAN, |run| {
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
    scan_runs(suite, CheckId::G11_GLOBAL_TABLE, |run| {
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
        if flow.is_zero() {
            return Err(world("J_global = 0; Y_global undefined"));
        }
        let decimal = (&held / &flow).round_decimal(4);
        if decimal != row.y {
            return Err(world(format!(
                "H_global / J_global rounds to {decimal}, section 4.2 states {}",
                row.y
            )));
        }
        Ok(())
    })?;
    scan_runs(suite, CheckId::G11_GLOBAL_EFFICIENCY, |run| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::lexicographic_routes;

    #[test]
    fn depth_first_routes_equal_the_lexicographic_construction() {
        for (n, tau) in [(3, 1), (3, 2), (3, 3), (4, 2), (5, 1)] {
            assert_eq!(depth_first_routes(n, tau), lexicographic_routes(n, tau));
        }
    }
}
