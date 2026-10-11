//! CAL-CEF-5 exact verifier: can a theory predict, in advance and in
//! numbers, how much open future an agent's energy buys?
//!
//! Boundary: `research/physics/derivable/xypher-theory-of-aim-boundary.md`
//! (frozen at commit 665da9b276bb0f3c5b89a21c06500502aa39ae44), which
//! extends `research/physics/derivable/xypher-aimed-foresight-boundary.md`
//! (CAL-CEF-4) with the speed `v`, the measure radius `R`, and the frozen
//! theory table of its section 4.3.
//!
//! `evaluate` runs the twenty worlds of section 3 with each of their agents
//! through gates G01--G12 (section 5), then the controls C01--C09 on the
//! primary witness (section 8), then the hypotheses H1--H5 by exact rational
//! comparison (section 7), and renders the section 6 report.
//! `evaluate_primary` does the same for the primary witness alone, without
//! the hypotheses, which need every world.

mod big;
mod gates;
mod logs;
mod model;
mod rat;
mod rival;
mod sha256;
mod solve;
mod symmetry;
mod theory;

use std::collections::BTreeMap;
use std::fmt::Write;

use gates::{yield_bound, Failure, GateResult, Run, Suite};
use model::{Agent, Mutation, WorldSpec, PAIRS, PRIMARY, WORLDS};
use rat::Rat;
use theory::{theory_key, theory_values};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateId {
    G01,
    G02,
    G03,
    G04,
    G05,
    G06,
    G07,
    G08,
    G09,
    G10,
    G11,
    G12,
}

impl GateId {
    pub const ALL: [Self; 12] = [
        Self::G01,
        Self::G02,
        Self::G03,
        Self::G04,
        Self::G05,
        Self::G06,
        Self::G07,
        Self::G08,
        Self::G09,
        Self::G10,
        Self::G11,
        Self::G12,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::G01 => "G01",
            Self::G02 => "G02",
            Self::G03 => "G03",
            Self::G04 => "G04",
            Self::G05 => "G05",
            Self::G06 => "G06",
            Self::G07 => "G07",
            Self::G08 => "G08",
            Self::G09 => "G09",
            Self::G10 => "G10",
            Self::G11 => "G11",
            Self::G12 => "G12",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::G01 => "grounding",
            Self::G02 => "executable rules",
            Self::G03 => "reciprocity",
            Self::G04 => "accounting",
            Self::G05 => "undriven world",
            Self::G06 => "driven steady state",
            Self::G07 => "GLOBAL calibration",
            Self::G08 => "second law and the floor",
            Self::G09 => "warming rival",
            Self::G10 => "Xypher slot reading",
            Self::G11 => "frozen values",
            Self::G12 => "frozen theory",
        }
    }

    fn evaluate(self, suite: &Suite) -> GateResult {
        match self {
            Self::G01 => gates::gate_01(suite),
            Self::G02 => gates::gate_02(suite),
            Self::G03 => gates::gate_03(suite),
            Self::G04 => gates::gate_04(suite),
            Self::G05 => gates::gate_05(suite),
            Self::G06 => gates::gate_06(suite),
            Self::G07 => gates::gate_07(suite),
            Self::G08 => gates::gate_08(suite),
            Self::G09 => gates::gate_09(suite),
            Self::G10 => gates::gate_10(suite),
            Self::G11 => gates::gate_11(suite),
            Self::G12 => gates::gate_12(suite),
        }
    }
}

macro_rules! check_ids {
    ($($name:ident => $gate:ident $index:literal),* $(,)?) => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum CheckId {
            $($name,)*
        }

        impl CheckId {
            pub const fn code(self) -> &'static str {
                match self {
                    $(Self::$name => stringify!($name),)*
                }
            }

            pub const fn gate(self) -> GateId {
                match self {
                    $(Self::$name => GateId::$gate,)*
                }
            }

            /// Position of the check within its gate (section 5 order).
            pub const fn index(self) -> usize {
                match self {
                    $(Self::$name => $index,)*
                }
            }
        }
    };
}

// Section 5 checks in order; G10 is three numbered checks and G12 three
// table-level checks.
check_ids! {
    G01_ROUTE_SET => G01 1,
    G01_WALK_COUNTS => G01 2,
    G01_STATE_ORBIT_COUNTS => G01 3,
    G02_DESTINATION_STATES => G02 1,
    G02_STEP_REPLAN => G02 2,
    G02_WEATHER_RULES => G02 3,
    G02_FUEL_RULES => G02 4,
    G02_CLASS_CHANGES => G02 5,
    G03_REVERSE_IN_CLASS => G03 1,
    G03_RATE_RATIOS => G03 2,
    G04_ONE_PACKET_ONE_STORE => G04 1,
    G04_HOT_EQUALS_COLD => G04 2,
    G04_BRIDGE_FLOW_IDENTITY => G04 3,
    G05_UNDRIVEN_CONNECTED => G05 1,
    G05_COLD_DETAILED_BALANCE => G05 2,
    G06_DRIVEN_CONNECTED => G06 1,
    G06_RELABELLING_SYMMETRY => G06 2,
    G06_POSITIVE_ORBIT_SOLUTION => G06 3,
    G06_LIFTED_BALANCE => G06 4,
    G07_PRODUCT_LAW => G07 1,
    G07_CLOSED_FORMS => G07 2,
    G08_POSITIVE_FUEL_FLOW => G08 1,
    G08_STORE_SYMBOLS => G08 2,
    G08_NONNEGATIVE_TERMS => G08 3,
    G08_CERTIFIED_EFFICIENCY => G08 4,
    G08_GLOBAL_EFFICIENCY => G08 5,
    G09_RIVAL_INVERSE => G09 1,
    G09_GLOBAL_RIVAL => G09 2,
    G09_PHI_IDENTITY => G09 3,
    G10_NAMED_SLOTS => G10 1,
    G10_EMPTY_RUBY_ABSENT_OPAL => G10 2,
    G10_RATE_INPUTS => G10 3,
    G11_UNDRIVEN_MEAN => G11 1,
    G11_GLOBAL_TABLE => G11 2,
    G11_GLOBAL_EFFICIENCY => G11 3,
    G12_THEORY_DIGEST => G12 1,
    G12_THEORY_PARSE => G12 2,
    G12_THEORY_KEYS => G12 3,
}

/// A first failure as `(gate, check, world, agent, configuration)`; a
/// table-level failure of G12 has no world, agent, or configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstFailure {
    pub check: CheckId,
    /// `None` for a table-level check.
    pub world: Option<String>,
    /// `None` for a table-level check.
    pub agent: Option<String>,
    /// Configuration bit mask of the offending state; `None` for a
    /// world-level or table-level check.
    pub configuration: Option<usize>,
    pub configuration_label: Option<String>,
    pub value: Option<String>,
    pub detail: String,
}

impl FirstFailure {
    fn from_failure(failure: &Failure, suite: &Suite) -> Self {
        let model = failure.run.map(|run| &suite.runs[run].model);
        Self {
            check: failure.check,
            world: model.map(|model| model.spec.label()),
            agent: model.map(|model| model.agent.code()),
            configuration: failure.configuration,
            configuration_label: model.and_then(|model| {
                failure
                    .configuration
                    .map(|config| model.config_label(config))
            }),
            value: failure.value.as_ref().map(|value| value.to_string()),
            detail: failure.detail.clone(),
        }
    }

    fn describe(&self) -> String {
        let mut text = format!(
            "{} check {} {} ",
            self.check.gate().code(),
            self.check.index(),
            self.check.code()
        );
        match (&self.world, &self.agent) {
            (Some(world), Some(agent)) => {
                write!(text, "world {world} agent {agent} ").unwrap();
                match &self.configuration_label {
                    Some(label) => write!(text, "configuration {label}").unwrap(),
                    None => text.push_str("world-level"),
                }
            }
            _ => text.push_str("table-level"),
        }
        if let Some(value) = &self.value {
            write!(text, " value {value}").unwrap();
        }
        text
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateOutcome {
    pub gate: GateId,
    pub failure: Option<FirstFailure>,
    pub note: String,
}

impl GateOutcome {
    pub fn passed(&self) -> bool {
        self.failure.is_none()
    }
}

/// Where section 8 names the first failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExpectedAt {
    /// At the configuration with this bit mask.
    Configuration(usize),
    /// At a world-level check.
    WorldLevel,
    /// At a world-level check recording `J = 0`.
    ZeroFuelFlow,
    /// At a table-level check (no world or agent).
    TableLevel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlOutcome {
    pub code: &'static str,
    pub name: &'static str,
    pub expected: CheckId,
    /// `None` for a table-level expectation.
    pub expected_agent: Option<String>,
    pub expected_at: String,
    pub observed: Option<FirstFailure>,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HypothesisOutcome {
    pub code: &'static str,
    pub statement: &'static str,
    pub supported: bool,
    /// Every item at which the hypothesis fails.
    pub failing: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofReport {
    /// The declared worlds whose gates were run.
    pub world_count: usize,
    pub gates: Vec<GateOutcome>,
    pub controls: Vec<ControlOutcome>,
    /// One line per world and agent (section 6).
    pub worlds: Vec<String>,
    pub hypotheses: Vec<HypothesisOutcome>,
}

impl ProofReport {
    /// `OVERALL PASS`: every gate passes and every control fails first where
    /// section 8 says. Hypotheses never enter.
    pub fn is_success(&self) -> bool {
        self.gates.len() == GateId::ALL.len()
            && self.gates.iter().all(GateOutcome::passed)
            && self.controls.len() == CONTROLS.len()
            && self.controls.iter().all(|control| control.passed)
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        writeln!(
            output,
            "XYPHER_THEORY_OF_AIM_PROOF CAL-CEF-5 primary={} worlds={} steady_states={} agents=LOCAL,AIM-k,GLOBAL",
            PRIMARY.label(),
            self.world_count,
            self.worlds.len()
        )
        .unwrap();
        for outcome in &self.gates {
            write!(
                output,
                "{} {} {}",
                outcome.gate.code(),
                if outcome.passed() { "PASS" } else { "FAIL" },
                outcome.gate.label()
            )
            .unwrap();
            match &outcome.failure {
                Some(failure) => write!(
                    output,
                    " first failure {} -- {}",
                    failure.describe(),
                    failure.detail
                )
                .unwrap(),
                None => write!(output, " -- {}", outcome.note).unwrap(),
            }
            writeln!(output).unwrap();
        }
        for control in &self.controls {
            let observed = control
                .observed
                .as_ref()
                .map_or_else(|| "NO_FAILURE".to_string(), FirstFailure::describe);
            write!(
                output,
                "{} {} {} observed {} expected {} check {} {} ",
                control.code,
                if control.passed { "PASS" } else { "FAIL" },
                control.name,
                observed,
                control.expected.gate().code(),
                control.expected.index(),
                control.expected.code()
            )
            .unwrap();
            if let Some(agent) = &control.expected_agent {
                write!(output, "agent {agent} ").unwrap();
            }
            output.push_str(&control.expected_at);
            if let Some(failure) = &control.observed {
                write!(output, " -- {}", failure.detail).unwrap();
            }
            writeln!(output).unwrap();
        }
        for line in &self.worlds {
            writeln!(output, "{line}").unwrap();
        }
        for hypothesis in &self.hypotheses {
            writeln!(
                output,
                "{} {} {} failing={}",
                hypothesis.code,
                if hypothesis.supported {
                    "SUPPORTED"
                } else {
                    "REFUTED"
                },
                hypothesis.statement,
                if hypothesis.failing.is_empty() {
                    "none".to_string()
                } else {
                    hypothesis.failing.join(";")
                }
            )
            .unwrap();
        }
        writeln!(
            output,
            "OVERALL {}",
            if self.is_success() { "PASS" } else { "FAIL" }
        )
        .unwrap();
        output
    }
}

struct ControlSpec {
    code: &'static str,
    name: &'static str,
    mutation: Mutation,
    expected: CheckId,
    /// `None` for a table-level expectation.
    agent: Option<Agent>,
    at: ExpectedAt,
}

/// Boundary section 8, controls in table order, on the primary witness.
const CONTROLS: [ControlSpec; 9] = [
    ControlSpec {
        code: "C01",
        name: "one-way-fuel",
        mutation: Mutation::OneWayFuel,
        expected: CheckId::G03_REVERSE_IN_CLASS,
        agent: Some(Agent::Local),
        at: ExpectedAt::Configuration(0b000),
    },
    ControlSpec {
        code: "C02",
        name: "free-protection",
        mutation: Mutation::FreeProtection,
        expected: CheckId::G02_WEATHER_RULES,
        agent: Some(Agent::Local),
        at: ExpectedAt::Configuration(0b001),
    },
    ControlSpec {
        code: "C03",
        name: "equal-stores",
        mutation: Mutation::EqualStores,
        expected: CheckId::G08_POSITIVE_FUEL_FLOW,
        agent: Some(Agent::Local),
        at: ExpectedAt::ZeroFuelFlow,
    },
    ControlSpec {
        code: "C04",
        name: "biased-step",
        mutation: Mutation::BiasedStep,
        expected: CheckId::G03_RATE_RATIOS,
        agent: Some(Agent::Local),
        at: ExpectedAt::Configuration(0b000),
    },
    ControlSpec {
        code: "C05",
        name: "aim-off-by-one",
        mutation: Mutation::AimOffByOne,
        expected: CheckId::G02_FUEL_RULES,
        agent: Some(Agent::Aim(1)),
        at: ExpectedAt::Configuration(0b000),
    },
    ControlSpec {
        code: "C06",
        name: "wrong-calibration",
        mutation: Mutation::WrongCalibration,
        expected: CheckId::G07_PRODUCT_LAW,
        agent: Some(Agent::Global),
        at: ExpectedAt::Configuration(0b000),
    },
    ControlSpec {
        code: "C07",
        name: "wrong-rival",
        mutation: Mutation::WrongRival,
        expected: CheckId::G09_RIVAL_INVERSE,
        agent: Some(Agent::Local),
        at: ExpectedAt::WorldLevel,
    },
    ControlSpec {
        code: "C08",
        name: "slow-replanning",
        mutation: Mutation::SlowReplanning,
        expected: CheckId::G10_RATE_INPUTS,
        agent: Some(Agent::Local),
        at: ExpectedAt::Configuration(0b000),
    },
    ControlSpec {
        code: "C09",
        name: "corrupted-theory",
        mutation: Mutation::CorruptedTheory,
        expected: CheckId::G12_THEORY_DIGEST,
        agent: None,
        at: ExpectedAt::TableLevel,
    },
];

fn gate_outcome(gate: GateId, suite: &Suite) -> GateOutcome {
    match gate.evaluate(suite) {
        Ok(note) => GateOutcome {
            gate,
            failure: None,
            note,
        },
        Err(failure) => GateOutcome {
            gate,
            failure: Some(FirstFailure::from_failure(&failure, suite)),
            note: String::new(),
        },
    }
}

/// Runs the gates on the mutated primary witness, its agents in section 3
/// order, and stops at the first failing check: nothing after it is
/// evaluated, in particular no quantity that divides by `J`, `sigma`, or
/// `H_warm`.
fn evaluate_control(spec: &ControlSpec) -> ControlOutcome {
    let suite = Suite::build(&[PRIMARY], spec.mutation);
    let observed = GateId::ALL
        .iter()
        .find_map(|gate| gate.evaluate(&suite).err())
        .map(|failure| FirstFailure::from_failure(&failure, &suite));
    let bridges = model::candidate_bridges(PRIMARY.n);
    let expected_at = match spec.at {
        ExpectedAt::Configuration(config) => {
            format!("configuration {}", model::config_label(&bridges, config))
        }
        ExpectedAt::WorldLevel => "world-level".to_string(),
        ExpectedAt::ZeroFuelFlow => "world-level value 0 (J = 0)".to_string(),
        ExpectedAt::TableLevel => "table-level".to_string(),
    };
    let expected_world = spec.mutation.constructed(PRIMARY).label();
    let passed = observed.as_ref().is_some_and(|failure| {
        let located = match spec.agent {
            Some(agent) => {
                failure.world.as_deref() == Some(expected_world.as_str())
                    && failure.agent == Some(agent.code())
            }
            None => failure.world.is_none() && failure.agent.is_none(),
        };
        failure.check == spec.expected
            && located
            && match spec.at {
                ExpectedAt::Configuration(config) => failure.configuration == Some(config),
                ExpectedAt::WorldLevel | ExpectedAt::TableLevel => failure.configuration.is_none(),
                ExpectedAt::ZeroFuelFlow => {
                    failure.configuration.is_none()
                        && failure.value.as_deref() == Some(Rat::zero().to_string().as_str())
                }
            }
    });
    ControlOutcome {
        code: spec.code,
        name: spec.name,
        expected: spec.expected,
        expected_agent: spec.agent.map(Agent::code),
        expected_at,
        observed,
        passed,
    }
}

fn exact_and_decimal(value: &Rat) -> String {
    format!("{value} ~{}", value.round_decimal(4))
}

/// The frozen `E_th` of the verbatim section 4.3 table, by key; empty when
/// the table does not parse (G12 then fails and every lookup is undefined).
type TheoryValues = BTreeMap<String, Rat>;

fn world_line(run: &Run, theory: &TheoryValues) -> String {
    let m = &run.model;
    let mut line = format!(
        "WORLD {} {} states={} orbits={}",
        m.spec.label(),
        m.agent.code(),
        m.states.len(),
        m.orbits.count()
    );
    let measures = match run.measures() {
        Ok(measures) => measures,
        Err(reason) => {
            write!(line, " unavailable ({reason})").unwrap();
            return line;
        }
    };
    write!(
        line,
        " NR_p={} NR_pi_c={} H={} J={}",
        exact_and_decimal(&measures.open_future_held),
        exact_and_decimal(&measures.open_future_cold),
        exact_and_decimal(&measures.held),
        exact_and_decimal(&measures.fuel_flow)
    )
    .unwrap();
    match run.fuel_yield() {
        Ok(value) => write!(line, " Y={}", exact_and_decimal(&value)).unwrap(),
        Err(reason) => write!(line, " Y={reason}").unwrap(),
    }
    write!(line, " rho={}", exact_and_decimal(&measures.density)).unwrap();
    match run.rival_rate() {
        Ok(Some(s)) => write!(line, " s*={}", exact_and_decimal(&s)).unwrap(),
        Ok(None) => line.push_str(" s*=none"),
        Err(reason) => write!(line, " s*={reason}").unwrap(),
    }
    let edge = run.edge();
    match &edge {
        Ok(value) => write!(line, " E={}", exact_and_decimal(value)).unwrap(),
        Err(reason) => write!(line, " E={reason}").unwrap(),
    }
    if m.agent == Agent::Global {
        line.push_str(" E_th=none E/E_th=none");
    } else {
        match theory.get(&theory_key(&m.declared, m.agent)) {
            Some(frozen) => {
                write!(line, " E_th={}", frozen.round_decimal(6)).unwrap();
                match &edge {
                    Ok(value) => {
                        let ratio = value / frozen;
                        write!(line, " E/E_th={ratio} ~{}", ratio.round_decimal(6)).unwrap();
                    }
                    Err(_) => line.push_str(" E/E_th=undefined"),
                }
            }
            None => line.push_str(" E_th=unavailable E/E_th=undefined"),
        }
    }
    match run.floor() {
        Ok(floor) => write!(line, " eta={}", floor.eta.render(12)).unwrap(),
        Err(reason) => write!(line, " eta=unavailable ({reason})").unwrap(),
    }
    match yield_bound(run) {
        Ok(bound) => write!(line, " Y_bound={}", bound.render(12)).unwrap(),
        Err(reason) => write!(line, " Y_bound=unavailable ({reason})").unwrap(),
    }
    line
}

/// `E` of a declared world's agent, undefined when the run is missing.
fn edge_of(suite: &Suite, world: &WorldSpec, agent: Agent) -> Result<Rat, String> {
    suite
        .run(world, agent)
        .ok_or_else(|| "missing run".to_string())?
        .edge()
}

/// Every value defined and each strictly below the one before.
fn strictly_decreasing(values: &[Result<Rat, String>]) -> bool {
    values.iter().all(Result::is_ok)
        && values
            .windows(2)
            .all(|window| match (&window[0], &window[1]) {
                (Ok(upper), Ok(lower)) => lower < upper,
                _ => false,
            })
}

/// The four speed ladders of H2: `(N, tau)` at each pair, `R = 2`.
const SPEED_LADDERS: [(usize, usize); 2] = [(3, 2), (4, 2)];
/// The speeds of a ladder, in order.
const SPEEDS: [u64; 3] = [1, 4, 16];

fn hypotheses(suite: &Suite, theory: &TheoryValues) -> Vec<HypothesisOutcome> {
    let one_percent_low = Rat::ratio(99, 100);
    let one_percent_high = Rat::ratio(101, 100);
    let three_percent = Rat::ratio(103, 100);

    // H1: (99/100) E_th <= E <= (101/100) E_th for each LOCAL and AIM state.
    let mut within = Vec::new();
    for world in &WORLDS {
        for agent in world.agents() {
            if agent == Agent::Global {
                continue;
            }
            let key = theory_key(world, agent);
            match (edge_of(suite, world, agent), theory.get(&key)) {
                (Ok(edge), Some(frozen)) => {
                    if edge < &one_percent_low * frozen || edge > &one_percent_high * frozen {
                        within.push(key);
                    }
                }
                _ => within.push(format!("{key} (undefined)")),
            }
        }
    }

    // H2: E strictly decreasing in v = 1, 4, 16 for LOCAL, AIM-1, AIM-2 in
    // each of the four speed ladders.
    let mut faster = Vec::new();
    for (b_c, b_h) in PAIRS {
        for (n, tau) in SPEED_LADDERS {
            for agent in [Agent::Local, Agent::Aim(1), Agent::Aim(2)] {
                let edges: Vec<Result<Rat, String>> = SPEEDS
                    .iter()
                    .map(|&v| edge_of(suite, &WorldSpec::new(n, tau, b_c, b_h, v, 2), agent))
                    .collect();
                if !strictly_decreasing(&edges) {
                    faster.push(format!(
                        "({n},{tau},{b_c},{b_h},R=2) {} v=1,4,16",
                        agent.code()
                    ));
                }
            }
        }
    }

    // H3: in every tau >= 2 world, E_AIM-k strictly decreasing in k and
    // E_AIM-tau < E_LOCAL.
    let mut blind = Vec::new();
    for world in WORLDS.iter().filter(|world| world.tau >= 2) {
        let label = world.label();
        let ladder: Vec<Result<Rat, String>> = (1..=world.tau)
            .map(|k| edge_of(suite, world, Agent::Aim(k)))
            .collect();
        if !strictly_decreasing(&ladder) {
            blind.push(format!("{label} (AIM-k ladder)"));
        }
        match (
            edge_of(suite, world, Agent::Aim(world.tau)),
            edge_of(suite, world, Agent::Local),
        ) {
            (Ok(route), Ok(local)) if route < local => {}
            (Ok(_), Ok(_)) => blind.push(format!("{label} (AIM-{} vs LOCAL)", world.tau)),
            _ => blind.push(format!("{label} (AIM-{} vs LOCAL undefined)", world.tau)),
        }
    }

    // H4: E_AIM-1 > E_LOCAL in all twenty worlds.
    let mut nearest = Vec::new();
    for world in &WORLDS {
        let label = world.label();
        match (
            edge_of(suite, world, Agent::Aim(1)),
            edge_of(suite, world, Agent::Local),
        ) {
            (Ok(aim), Ok(local)) if aim > local => {}
            (Ok(_), Ok(_)) => nearest.push(label),
            _ => nearest.push(format!("{label} (undefined)")),
        }
    }

    // H5: E_LOCAL <= L(N, rho_LOCAL) <= (103/100) E_LOCAL at tau = 1.
    let mut law = Vec::new();
    for world in WORLDS.iter().filter(|world| world.tau == 1) {
        let label = world.label();
        let local = suite.run(world, Agent::Local);
        let edge = edge_of(suite, world, Agent::Local);
        let simple = local
            .and_then(|run| run.measures().ok())
            .and_then(|measures| rival::simple_law(world.n, world.b_c, &measures.density));
        match (edge, simple) {
            (Ok(edge), Some(simple)) => {
                if simple < edge {
                    law.push(format!("{label} (L below E_LOCAL)"));
                } else if simple > &three_percent * &edge {
                    law.push(format!("{label} (L above (103/100) E_LOCAL)"));
                }
            }
            _ => law.push(format!("{label} (undefined)")),
        }
    }

    vec![
        HypothesisOutcome {
            code: "H1",
            statement: "theory-within-one-percent (99/100)E_th<=E<=(101/100)E_th for all 56 LOCAL and AIM steady states",
            supported: within.is_empty(),
            failing: within,
        },
        HypothesisOutcome {
            code: "H2",
            statement: "faster-travellers-gain-less E strictly decreasing v=1,4,16 for LOCAL, AIM-1, AIM-2 in the four speed ladders (twelve sequences)",
            supported: faster.is_empty(),
            failing: faster,
        },
        HypothesisOutcome {
            code: "H3",
            statement: "blind-foresight-never-pays E_AIM-k strictly decreasing in k=1..tau and E_AIM-tau<E_LOCAL in the fourteen tau>=2 worlds",
            supported: blind.is_empty(),
            failing: blind,
        },
        HypothesisOutcome {
            code: "H4",
            statement: "nearest-aim-leads E_AIM-1>E_LOCAL in all 20 worlds",
            supported: nearest.is_empty(),
            failing: nearest,
        },
        HypothesisOutcome {
            code: "H5",
            statement: "simple-law E_LOCAL<=L(N,rho_LOCAL)<=(103/100)E_LOCAL in the six tau=1 worlds",
            supported: law.is_empty(),
            failing: law,
        },
    ]
}

fn gates_and_controls(worlds: &[WorldSpec]) -> (Suite, Vec<GateOutcome>, Vec<ControlOutcome>) {
    let suite = Suite::build(worlds, Mutation::None);
    let gates = GateId::ALL
        .iter()
        .map(|&gate| gate_outcome(gate, &suite))
        .collect();
    let controls = CONTROLS.iter().map(evaluate_control).collect();
    (suite, gates, controls)
}

/// Evaluates the preregistered construction of all twenty worlds, the
/// controls, and the hypotheses, in that order.
pub fn evaluate() -> ProofReport {
    let (suite, gates, controls) = gates_and_controls(&WORLDS);
    let theory = theory_values().unwrap_or_default();
    let worlds = suite
        .runs
        .iter()
        .map(|run| world_line(run, &theory))
        .collect();
    let hypotheses = hypotheses(&suite, &theory);
    ProofReport {
        world_count: WORLDS.len(),
        gates,
        controls,
        worlds,
        hypotheses,
    }
}

/// The gates on the primary witness `(3, 2, 5, 2, 4, 2)` alone, its world
/// lines, and the controls; no hypothesis is evaluated (each needs every
/// world).
pub fn evaluate_primary() -> ProofReport {
    let (suite, gates, controls) = gates_and_controls(&[PRIMARY]);
    let theory = theory_values().unwrap_or_default();
    let worlds = suite
        .runs
        .iter()
        .map(|run| world_line(run, &theory))
        .collect();
    ProofReport {
        world_count: 1,
        gates,
        controls,
        worlds,
        hypotheses: Vec::new(),
    }
}
