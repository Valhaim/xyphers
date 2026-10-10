//! CAL-CEF-3 exact verifier: how much open future can a unit of energy buy?
//!
//! Boundary: `research/physics/derivable/xypher-open-future-price-boundary.md`
//! (frozen at commit e5474cb773c3da74ce4162d20e07f37d904a945e).
//!
//! `evaluate` runs the twelve worlds of section 4.2 with the LOCAL and the
//! GLOBAL agent through gates G01--G10 (section 5), then the controls
//! C01--C06 on the primary witness (section 8), then the hypotheses H0--H3 by
//! exact rational comparison (section 7), and renders the section 6 report.

mod big;
mod gates;
mod logs;
mod model;
mod rat;
mod solve;

use std::fmt::Write;

use gates::{yield_bound, Failure, GateResult, Run, Suite};
use model::{Agent, Mutation, PRIMARY, WORLDS};
use rat::Rat;

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
}

impl GateId {
    pub const ALL: [Self; 10] = [
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
            Self::G09 => "Xypher slot reading",
            Self::G10 => "frozen values",
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

// Section 5 checks in order. G09 lists its requirements without numbers; they
// are checked in the order written: the named slots, Ruby empty and Opal
// absent, and the rate inputs.
check_ids! {
    G01_PLAN_COUNT_POSITIVE => G01 1,
    G01_DEPTH_FIRST_COUNTS => G01 2,
    G01_STATE_COUNT => G01 3,
    G02_DESTINATION_STATES => G02 1,
    G02_STEP_REPLAN => G02 2,
    G02_WEATHER_RULES => G02 3,
    G02_FUEL_RULES => G02 4,
    G02_PLAN_CURSOR_KEPT => G02 5,
    G03_REVERSE_IN_CLASS => G03 1,
    G03_RATE_RATIOS => G03 2,
    G04_ONE_PACKET_ONE_STORE => G04 1,
    G04_HOT_EQUALS_COLD => G04 2,
    G05_UNDRIVEN_CONNECTED => G05 1,
    G05_COLD_DETAILED_BALANCE => G05 2,
    G06_DRIVEN_CONNECTED => G06 1,
    G06_POSITIVE_SOLUTION => G06 2,
    G06_FLUX_BALANCE => G06 3,
    G07_PRODUCT_LAW => G07 1,
    G07_CLOSED_FORMS => G07 2,
    G08_POSITIVE_FUEL_FLOW => G08 1,
    G08_STORE_SYMBOLS => G08 2,
    G08_NONNEGATIVE_TERMS => G08 3,
    G08_CERTIFIED_EFFICIENCY => G08 4,
    G08_GLOBAL_EFFICIENCY => G08 5,
    G09_NAMED_SLOTS => G09 1,
    G09_EMPTY_RUBY_ABSENT_OPAL => G09 2,
    G09_RATE_INPUTS => G09 3,
    G10_UNDRIVEN_MEAN => G10 1,
    G10_GLOBAL_TABLE => G10 2,
    G10_GLOBAL_EFFICIENCY => G10 3,
}

/// A first failure as `(gate, check, world, agent, configuration)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstFailure {
    pub check: CheckId,
    pub world: String,
    pub agent: &'static str,
    /// Configuration bit mask of the offending state; `None` for a
    /// world-level check.
    pub configuration: Option<usize>,
    pub configuration_label: Option<String>,
    pub value: Option<String>,
    pub detail: String,
}

impl FirstFailure {
    fn from_failure(failure: &Failure, suite: &Suite) -> Self {
        let model = &suite.runs[failure.run].model;
        Self {
            check: failure.check,
            world: model.spec.label(),
            agent: model.agent.code(),
            configuration: failure.configuration,
            configuration_label: failure
                .configuration
                .map(|config| model.config_label(config)),
            value: failure.value.as_ref().map(Rat::to_string),
            detail: failure.detail.clone(),
        }
    }

    fn describe(&self) -> String {
        let mut text = format!(
            "{} check {} {} world {} agent {} ",
            self.check.gate().code(),
            self.check.index(),
            self.check.code(),
            self.world,
            self.agent
        );
        match &self.configuration_label {
            Some(label) => write!(text, "configuration {label}").unwrap(),
            None => text.push_str("world-level"),
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
pub enum ExpectedAt {
    /// At the configuration with this bit mask.
    Configuration(usize),
    /// At a world-level check recording `J = 0`.
    ZeroFuelFlow,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlOutcome {
    pub code: &'static str,
    pub name: &'static str,
    pub expected: CheckId,
    pub expected_agent: &'static str,
    pub expected_at: String,
    pub observed: Option<FirstFailure>,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HypothesisOutcome {
    pub code: &'static str,
    pub statement: &'static str,
    pub supported: bool,
    /// Every world or temperature pair at which the hypothesis fails.
    pub failing: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofReport {
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
            "XYPHER_OPEN_FUTURE_PRICE_PROOF CAL-CEF-3 primary={} agent=LOCAL worlds={} agents=LOCAL,GLOBAL",
            PRIMARY.label(),
            WORLDS.len()
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
                "{} {} {} observed {} expected {} check {} {} agent {} {}",
                control.code,
                if control.passed { "PASS" } else { "FAIL" },
                control.name,
                observed,
                control.expected.gate().code(),
                control.expected.index(),
                control.expected.code(),
                control.expected_agent,
                control.expected_at
            )
            .unwrap();
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
    agent: Agent,
    at: ExpectedAt,
}

/// Boundary section 8, controls in table order, on the primary witness.
const CONTROLS: [ControlSpec; 6] = [
    ControlSpec {
        code: "C01",
        name: "one-way-fuel",
        mutation: Mutation::OneWayFuel,
        expected: CheckId::G03_REVERSE_IN_CLASS,
        agent: Agent::Local,
        at: ExpectedAt::Configuration(0b000),
    },
    ControlSpec {
        code: "C02",
        name: "plan-breaking-weather",
        mutation: Mutation::PlanBreakingWeather,
        expected: CheckId::G02_DESTINATION_STATES,
        agent: Agent::Local,
        at: ExpectedAt::Configuration(0b001),
    },
    ControlSpec {
        code: "C03",
        name: "equal-stores",
        mutation: Mutation::EqualStores,
        expected: CheckId::G08_POSITIVE_FUEL_FLOW,
        agent: Agent::Local,
        at: ExpectedAt::ZeroFuelFlow,
    },
    ControlSpec {
        code: "C04",
        name: "biased-weather",
        mutation: Mutation::BiasedWeather,
        expected: CheckId::G03_RATE_RATIOS,
        agent: Agent::Local,
        at: ExpectedAt::Configuration(0b000),
    },
    ControlSpec {
        code: "C05",
        name: "wrong-store",
        mutation: Mutation::WrongStore,
        expected: CheckId::G03_RATE_RATIOS,
        agent: Agent::Local,
        at: ExpectedAt::Configuration(0b000),
    },
    ControlSpec {
        code: "C06",
        name: "wrong-calibration",
        mutation: Mutation::WrongCalibration,
        expected: CheckId::G07_PRODUCT_LAW,
        agent: Agent::Global,
        at: ExpectedAt::Configuration(0b000),
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

/// Runs the gates on the mutated primary witness and stops at the first
/// failing check: nothing after it is evaluated, in particular no quantity
/// that divides by `J` or `sigma`.
fn evaluate_control(spec: &ControlSpec) -> ControlOutcome {
    let suite = Suite::build(&[PRIMARY], spec.mutation);
    let observed = GateId::ALL
        .iter()
        .find_map(|gate| gate.evaluate(&suite).err())
        .map(|failure| FirstFailure::from_failure(&failure, &suite));
    let bridges = &suite.runs[0].model.bridges;
    let expected_at = match spec.at {
        ExpectedAt::Configuration(config) => {
            format!("configuration {}", model::config_label(bridges, config))
        }
        ExpectedAt::ZeroFuelFlow => "world-level value 0 (J = 0)".to_string(),
    };
    let passed = observed.as_ref().is_some_and(|failure| {
        failure.check == spec.expected
            && failure.agent == spec.agent.code()
            && failure.world == spec.mutation.constructed(PRIMARY).label()
            && match spec.at {
                ExpectedAt::Configuration(config) => failure.configuration == Some(config),
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
        expected_agent: spec.agent.code(),
        expected_at,
        observed,
        passed,
    }
}

fn exact_and_decimal(value: &Rat) -> String {
    format!("{value} ~{}", value.round_decimal(4))
}

/// `Y = H / J`, defined when `J != 0`.
fn yield_of(run: &Run) -> Result<Rat, String> {
    let measures = run.measures()?;
    if measures.fuel_flow.is_zero() {
        return Err("undefined (J = 0)".to_string());
    }
    Ok(&measures.held / &measures.fuel_flow)
}

fn world_line(run: &Run) -> String {
    let m = &run.model;
    let mut line = format!(
        "WORLD {} {} states={}",
        m.spec.label(),
        m.agent.code(),
        m.states.len()
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
        " N2_p={} N2_pi_c={} H={} J={}",
        exact_and_decimal(&measures.open_future_held),
        exact_and_decimal(&measures.open_future_cold),
        exact_and_decimal(&measures.held),
        exact_and_decimal(&measures.fuel_flow)
    )
    .unwrap();
    match yield_of(run) {
        Ok(value) => write!(line, " Y={}", exact_and_decimal(&value)).unwrap(),
        Err(reason) => write!(line, " Y={reason}").unwrap(),
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

/// LOCAL and GLOBAL runs of world `index` (section 4.2 row order).
fn agents_of(suite: &Suite, index: usize) -> (&Run, &Run) {
    (&suite.runs[2 * index], &suite.runs[2 * index + 1])
}

/// World indices `(N = 3, tau = 1, 2, 3)` of each temperature pair.
fn horizon_ladders() -> Vec<((u64, u64), Vec<usize>)> {
    gates::PAIRS
        .iter()
        .map(|&pair| {
            let mut ladder: Vec<usize> = (0..WORLDS.len())
                .filter(|&index| {
                    let world = WORLDS[index];
                    world.n == 3 && (world.b_c, world.b_h) == pair
                })
                .collect();
            ladder.sort_by_key(|&index| WORLDS[index].tau);
            (pair, ladder)
        })
        .collect()
}

fn strictly_increasing(values: &[Result<Rat, String>]) -> bool {
    values
        .windows(2)
        .all(|window| match (&window[0], &window[1]) {
            (Ok(lower), Ok(upper)) => lower < upper,
            _ => false,
        })
}

fn hypotheses(suite: &Suite) -> Vec<HypothesisOutcome> {
    let world_label = |index: usize| WORLDS[index].label();
    let mut fuel_buys = Vec::new();
    let mut targeting = Vec::new();
    for index in 0..WORLDS.len() {
        let (local, global) = agents_of(suite, index);
        match local.measures() {
            Ok(measures) if measures.held.is_positive() => {}
            Ok(_) => fuel_buys.push(world_label(index)),
            Err(_) => fuel_buys.push(format!("{} (unavailable)", world_label(index))),
        }
        match (yield_of(local), yield_of(global)) {
            (Ok(y_local), Ok(y_global)) if y_local > y_global => {}
            (Ok(_), Ok(_)) => targeting.push(world_label(index)),
            _ => targeting.push(format!("{} (Y undefined)", world_label(index))),
        }
    }
    let mut horizon = Vec::new();
    let mut foresight = Vec::new();
    for (pair, ladder) in horizon_ladders() {
        let label = format!("({},{})", pair.0, pair.1);
        let local: Vec<Result<Rat, String>> = ladder
            .iter()
            .map(|&index| yield_of(agents_of(suite, index).0))
            .collect();
        let advantage: Vec<Result<Rat, String>> = ladder
            .iter()
            .map(|&index| {
                let (local, global) = agents_of(suite, index);
                let y_global = yield_of(global)?;
                if y_global.is_zero() {
                    return Err("Y_global = 0".to_string());
                }
                Ok(&yield_of(local)? / &y_global)
            })
            .collect();
        if ladder.len() != 3 || !strictly_increasing(&local) {
            horizon.push(label.clone());
        }
        if ladder.len() != 3 || !strictly_increasing(&advantage) {
            foresight.push(label);
        }
    }
    vec![
        HypothesisOutcome {
            code: "H0",
            statement: "fuel-buys-open-future H_local>0 in every world",
            supported: fuel_buys.is_empty(),
            failing: fuel_buys,
        },
        HypothesisOutcome {
            code: "H1",
            statement: "targeting-beats-heating Y_local>Y_global in every world",
            supported: targeting.is_empty(),
            failing: targeting,
        },
        HypothesisOutcome {
            code: "H2",
            statement: "yield-rises-with-horizon Y_local strictly increasing tau=1,2,3 (N=3) for each pair",
            supported: horizon.is_empty(),
            failing: horizon,
        },
        HypothesisOutcome {
            code: "H3",
            statement: "foresight-sharpens-targeting Y_local/Y_global strictly increasing tau=1,2,3 (N=3) for each pair",
            supported: foresight.is_empty(),
            failing: foresight,
        },
    ]
}

/// Evaluates the preregistered construction, the controls, and the
/// hypotheses, in that order.
pub fn evaluate() -> ProofReport {
    let suite = Suite::build(&WORLDS, Mutation::None);
    let gates = GateId::ALL
        .iter()
        .map(|&gate| gate_outcome(gate, &suite))
        .collect();
    let controls = CONTROLS.iter().map(evaluate_control).collect();
    let worlds = suite.runs.iter().map(world_line).collect();
    let hypotheses = hypotheses(&suite);
    ProofReport {
        gates,
        controls,
        worlds,
        hypotheses,
    }
}
