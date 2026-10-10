//! CAL-CEF-2 exact verifier: control information thermodynamics of a
//! notebook that records where the traveller's held plan ends.
//!
//! Boundary: `research/physics/derivable/xypher-control-information-thermodynamics-boundary.md`
//! (frozen at commit 79d344024eb0169d4b458213936fc5cec8c476cd).

mod gates;
mod model;
mod primes;
mod ratio;

use std::fmt::Write;

use gates::{Context, Failure, GateResult, Status};
use model::{Case, Model, Mutation, FAMILY, PRIMARY, SLIPPED_STOCKPILE};
use ratio::Ratio;

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
    G13,
    G14,
}

impl GateId {
    pub const ALL: [Self; 14] = [
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
        Self::G13,
        Self::G14,
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
            Self::G13 => "G13",
            Self::G14 => "G14",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::G01 => "Crystal and command grounding",
            Self::G02 => "executable registers",
            Self::G03 => "reciprocal generator",
            Self::G04 => "pathwise accounting",
            Self::G05 => "reservoir lumpability",
            Self::G06 => "one temperature, three traffics",
            Self::G07 => "connectivity and equilibrium",
            Self::G08 => "undistorted world",
            Self::G09 => "control channel",
            Self::G10 => "feedback exchange",
            Self::G11 => "shuffled deck",
            Self::G12 => "Landauer erasure",
            Self::G13 => "no free lunch",
            Self::G14 => "Xypher slot reading",
        }
    }

    fn evaluate(self, context: &Context) -> GateResult {
        match self {
            Self::G01 => gates::gate_01(context),
            Self::G02 => gates::gate_02(context),
            Self::G03 => gates::gate_03(context),
            Self::G04 => gates::gate_04(context),
            Self::G05 => gates::gate_05(context),
            Self::G06 => gates::gate_06(context),
            Self::G07 => gates::gate_07(context),
            Self::G08 => gates::gate_08(context),
            Self::G09 => gates::gate_09(context),
            Self::G10 => gates::gate_10(context),
            Self::G11 => gates::gate_11(context),
            Self::G12 => gates::gate_12(context),
            Self::G13 => gates::gate_13(context),
            Self::G14 => gates::gate_14(context),
        }
    }

    /// Section 7 allows N/A only for G05 at N = 4.
    const fn may_be_not_applicable(self, n: usize) -> bool {
        matches!(self, Self::G05) && n == 4
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

            /// Position of the check within its gate (section 7 order).
            pub const fn index(self) -> usize {
                match self {
                    $(Self::$name => $index,)*
                }
            }
        }
    };
}

check_ids! {
    G01_COUNT_POSITIVE => G01 1,
    G01_DFS_EQUALS_MATRIX => G01 2,
    G01_REPLAN_UNIFORM => G01 3,
    G01_DESTINATION_SUM => G01 4,
    G01_SLICE_STATE_COUNT => G01 5,
    G02_DESTINATION_VALIDITY => G02 1,
    G02_TRAVEL_AND_TOGGLES => G02 2,
    G02_MEASURE_SEMANTICS => G02 3,
    G02_FEEDBACK_SEMANTICS => G02 4,
    G02_ERASURE_SEMANTICS => G02 5,
    G02_REGISTERS_UNTOUCHED => G02 6,
    G03_RECIPROCAL_SUPPORT => G03 1,
    G03_HAZARD_VALUES => G03 2,
    G03_ROW_CLOSURE => G03 3,
    G04_CONSERVATION => G04 1,
    G04_EXCHANGE_PARTNERS => G04 2,
    G04_ERASE_ONE_PACKET => G04 3,
    G04_NEUTRAL_CHANNELS => G04 4,
    G05_LUMPABILITY => G05 1,
    G06_BRIDGE_TRAFFIC => G06 1,
    G06_FEEDBACK_TRAFFIC => G06 2,
    G06_ERASURE_TRAFFIC => G06 3,
    G06_CONSTANT_ENTROPY => G06 4,
    G07_CONNECTIVITY => G07 1,
    G07_PROPAGATED_LAW => G07 2,
    G07_FLUX_BALANCE => G07 3,
    G08_CONFIGURATION_MARGINAL => G08 1,
    G08_NOTEBOOK_MARGINAL => G08 2,
    G08_STOCKPILE_MARGINAL => G08 3,
    G08_PLAN_NOTEBOOK_INDEPENDENCE => G08 4,
    G08_STATE_TOTALS => G08 5,
    G09_DESTINATION_COUNTS => G09 1,
    G09_CAPACITY_WITNESS => G09 2,
    G09_PLAN_LAW_INFORMATION => G09 3,
    G10_FEEDBACK_STRUCTURE => G10 1,
    G10_MASS_RATIO => G10 2,
    G10_RATE_RATIO => G10 3,
    G10_AVERAGE_WORTH => G10 4,
    G10_FROZEN_EXCHANGE => G10 5,
    G11_CORRELATED_UNIFORM => G11 1,
    G11_SHUFFLED_MARGINALS => G11 2,
    G11_NOTEBOOK_BLIND_ENERGY => G11 3,
    G11_GAP_FROM_LAWS => G11 4,
    G11_GAP_EQUALS_EXCHANGE => G11 5,
    G11_CAPACITY_GAP => G11 6,
    G12_ERASURE_STRUCTURE => G12 1,
    G12_BLANK_RATIO => G12 2,
    G12_FROZEN_ERASURE => G12 3,
    G13_STOCKPILE_CURRENT => G13 1,
    G13_NOTEBOOK_CURRENT => G13 2,
    G14_NAMED_SLOTS => G14 1,
    G14_EMPTY_RUBY => G14 2,
    G14_HAZARD_INPUTS => G14 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateStatus {
    Pass,
    Fail,
    NotApplicable,
}

impl GateStatus {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::NotApplicable => "N/A",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateOutcome {
    pub gate: GateId,
    pub status: GateStatus,
    pub failure: Option<CheckId>,
    pub configuration: Option<String>,
    pub detail: String,
}

impl GateOutcome {
    pub fn passed(&self, n: usize) -> bool {
        match self.status {
            GateStatus::Pass => true,
            GateStatus::NotApplicable => self.gate.may_be_not_applicable(n),
            GateStatus::Fail => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlOutcome {
    pub code: &'static str,
    pub name: &'static str,
    pub expected_failure: CheckId,
    pub expected_configuration: String,
    pub expected_value: Option<String>,
    pub observed_failure: Option<CheckId>,
    pub observed_configuration: Option<String>,
    pub observed_detail: String,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FamilyOutcome {
    pub case: String,
    pub n: usize,
    pub configurations: usize,
    pub system_states: usize,
    pub complete_states: usize,
    pub partition: String,
    pub mean_bridges: String,
    pub erasure: String,
    pub mean_stockpile: String,
    pub blank: String,
    pub k_one: String,
    pub gates: Vec<GateOutcome>,
}

impl FamilyOutcome {
    pub fn passed(&self) -> bool {
        self.gates.len() == GateId::ALL.len()
            && self.gates.iter().all(|outcome| outcome.passed(self.n))
    }

    fn summary(&self) -> String {
        let count = |status: GateStatus| {
            self.gates
                .iter()
                .filter(|outcome| outcome.status == status)
                .count()
        };
        let not_applicable: Vec<&str> = self
            .gates
            .iter()
            .filter(|outcome| outcome.status == GateStatus::NotApplicable)
            .map(|outcome| outcome.gate.code())
            .collect();
        let mut summary = format!(
            "PASS={} FAIL={} N/A={}",
            count(GateStatus::Pass),
            count(GateStatus::Fail),
            count(GateStatus::NotApplicable)
        );
        if !not_applicable.is_empty() {
            write!(summary, "({})", not_applicable.join(",")).unwrap();
        }
        if let Some(failed) = self
            .gates
            .iter()
            .find(|outcome| outcome.status == GateStatus::Fail)
        {
            write!(
                summary,
                " first={}",
                failed.failure.map(CheckId::code).unwrap_or("UNKNOWN")
            )
            .unwrap();
        }
        summary
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofReport {
    pub gates: Vec<GateOutcome>,
    pub controls: Vec<ControlOutcome>,
    /// One line per primary configuration (section 9).
    pub witness: Vec<String>,
    /// Erasure ratio, mean stockpile, blank probability, correct fraction.
    pub instrument: String,
    pub family: Vec<FamilyOutcome>,
}

impl ProofReport {
    pub fn is_success(&self) -> bool {
        self.gates.len() == GateId::ALL.len()
            && self.gates.iter().all(|outcome| outcome.passed(PRIMARY.n))
            && self.controls.len() == CONTROLS.len()
            && self.controls.iter().all(|outcome| outcome.passed)
            && self.family.len() == FAMILY.len()
            && self.family.iter().all(FamilyOutcome::passed)
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        writeln!(
            output,
            "XYPHER_CONTROL_INFORMATION_PROOF CAL-CEF-2 primary={}",
            PRIMARY.label()
        )
        .unwrap();
        for outcome in &self.gates {
            write!(
                output,
                "{} {} {}",
                outcome.gate.code(),
                outcome.status.code(),
                outcome.gate.label()
            )
            .unwrap();
            if let Some(check) = outcome.failure {
                write!(output, " check {} {}", check.index(), check.code()).unwrap();
            }
            if let Some(configuration) = &outcome.configuration {
                write!(output, " at {configuration}").unwrap();
            }
            if !outcome.detail.is_empty() {
                write!(output, " -- {}", outcome.detail).unwrap();
            }
            writeln!(output).unwrap();
        }
        for outcome in &self.controls {
            let observed = outcome
                .observed_failure
                .map(|check| {
                    format!(
                        "{} check {} {} at {}",
                        check.gate().code(),
                        check.index(),
                        check.code(),
                        outcome
                            .observed_configuration
                            .as_deref()
                            .unwrap_or("global")
                    )
                })
                .unwrap_or_else(|| "NO_FAILURE".to_string());
            write!(
                output,
                "{} {} {} observed {} expected {} check {} {} at {}",
                outcome.code,
                if outcome.passed { "PASS" } else { "FAIL" },
                outcome.name,
                observed,
                outcome.expected_failure.gate().code(),
                outcome.expected_failure.index(),
                outcome.expected_failure.code(),
                outcome.expected_configuration
            )
            .unwrap();
            if let Some(value) = &outcome.expected_value {
                write!(output, " value {value}").unwrap();
            }
            if !outcome.observed_detail.is_empty() {
                write!(output, " -- {}", outcome.observed_detail).unwrap();
            }
            writeln!(output).unwrap();
        }
        for line in &self.witness {
            writeln!(output, "{line}").unwrap();
        }
        writeln!(output, "{}", self.instrument).unwrap();
        for family in &self.family {
            writeln!(
                output,
                "FAMILY {} configurations={} system={} complete={} Z={} mean_bridges={} erasure={} mean_stockpile={} blank={} K1=[{}] gates {}",
                family.case,
                family.configurations,
                family.system_states,
                family.complete_states,
                family.partition,
                family.mean_bridges,
                family.erasure,
                family.mean_stockpile,
                family.blank,
                family.k_one,
                family.summary()
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

/// Where section 8 names the first failure.
#[derive(Clone, Copy, Debug)]
enum ExpectedAt {
    Configuration(usize),
    Global,
}

struct ControlSpec {
    code: &'static str,
    name: &'static str,
    mutation: Mutation,
    expected: CheckId,
    at: ExpectedAt,
    /// Observed value the failing global check must report (section 8).
    value: Option<(i128, i128)>,
}

/// Boundary section 8, C09: mean stockpile `11/15` under `W = 3`.
const C09_MEAN_STOCKPILE: (i128, i128) = (11, 15);

/// Boundary section 8, controls in table order.
const CONTROLS: [ControlSpec; 9] = [
    ControlSpec {
        code: "C01",
        name: "one-way-harvest",
        mutation: Mutation::OneWayHarvest,
        expected: CheckId::G03_RECIPROCAL_SUPPORT,
        at: ExpectedAt::Configuration(0),
        value: None,
    },
    ControlSpec {
        code: "C02",
        name: "free-reset",
        mutation: Mutation::FreeReset,
        expected: CheckId::G02_ERASURE_SEMANTICS,
        at: ExpectedAt::Configuration(0),
        value: None,
    },
    ControlSpec {
        code: "C03",
        name: "blind-measurement",
        mutation: Mutation::BlindMeasurement,
        expected: CheckId::G02_MEASURE_SEMANTICS,
        at: ExpectedAt::Configuration(0b001),
        value: None,
    },
    ControlSpec {
        code: "C04",
        name: "cheating-harvest",
        mutation: Mutation::CheatingHarvest,
        expected: CheckId::G02_FEEDBACK_SEMANTICS,
        at: ExpectedAt::Configuration(0),
        value: None,
    },
    ControlSpec {
        code: "C05",
        name: "telepathic-demon",
        mutation: Mutation::TelepathicDemon,
        expected: CheckId::G02_FEEDBACK_SEMANTICS,
        at: ExpectedAt::Configuration(0),
        value: None,
    },
    ControlSpec {
        code: "C06",
        name: "directed-kinetic-mutation",
        mutation: Mutation::DirectedKinetic,
        expected: CheckId::G03_HAZARD_VALUES,
        at: ExpectedAt::Configuration(0),
        value: None,
    },
    ControlSpec {
        code: "C07",
        name: "hoarding-demon",
        mutation: Mutation::HoardingDemon,
        expected: CheckId::G02_FEEDBACK_SEMANTICS,
        at: ExpectedAt::Configuration(0b001),
        value: None,
    },
    ControlSpec {
        code: "C08",
        name: "unequal-shuffle",
        mutation: Mutation::UnequalShuffle,
        expected: CheckId::G11_SHUFFLED_MARGINALS,
        at: ExpectedAt::Configuration(0),
        value: None,
    },
    ControlSpec {
        code: "C09",
        name: "stockpile-slip",
        mutation: Mutation::StockpileSlip,
        expected: CheckId::G08_STOCKPILE_MARGINAL,
        at: ExpectedAt::Global,
        value: Some(C09_MEAN_STOCKPILE),
    },
];

/// Values read from the propagated law for the report lines.
struct CaseSummary {
    configurations: usize,
    system_states: usize,
    complete_states: usize,
    partition: String,
    mean_bridges: String,
    erasure: String,
    mean_stockpile: String,
    blank: String,
    correct_fraction: String,
    k_one: String,
    witness: Vec<String>,
}

struct CaseEvaluation {
    gates: Vec<GateOutcome>,
    first_failure: Option<Failure>,
    summary: Option<CaseSummary>,
}

const UNAVAILABLE: &str = "unavailable";

fn show_ratio(value: Option<Ratio>) -> String {
    value.map_or_else(|| "undefined".to_string(), |value| value.to_string())
}

fn summarise(model: &Model, context: &Context) -> CaseSummary {
    let law = context.stationary_law();
    let read = |f: &dyn Fn(&gates::StationaryLaw) -> String| law.map_or(UNAVAILABLE.to_string(), f);
    let witness = if model.is_primary() {
        (0..model.config_count)
            .map(|config| {
                let counts = gates::destination_counts(model, config);
                let reachable = counts.iter().filter(|&&count| count > 0).count();
                let k = read(&|law| {
                    gates::exchange_table(model, law, config)
                        .iter()
                        .map(|k| k.map_or("-".to_string(), |value| value.to_string()))
                        .collect::<Vec<_>>()
                        .join(",")
                });
                let information = gates::uniform_plan_information(model, config);
                let capacity = gates::capacity_information(model, config);
                format!(
                    "WITNESS {} N_2={} n={} |D|={reachable} K={k} exp[N I]={} exp[|D| I*]={}",
                    model.config_label(config),
                    model.matrix_count[config],
                    counts
                        .iter()
                        .map(i128::to_string)
                        .collect::<Vec<_>>()
                        .join(","),
                    information
                        .exp_value()
                        .map_or_else(|| information.to_string(), |value| value.to_string()),
                    capacity
                        .exp_value()
                        .map_or_else(|| capacity.to_string(), |value| value.to_string())
                )
            })
            .collect()
    } else {
        Vec::new()
    };
    CaseSummary {
        configurations: model.config_count,
        system_states: model.states.len(),
        complete_states: model.complete_state_count(),
        partition: gates::partition_function(model).to_string(),
        mean_bridges: read(&|law| gates::mean_bridges(model, law).to_string()),
        erasure: read(&|law| {
            gates::erasure_ratios(model, law)
                .into_iter()
                .map(show_ratio)
                .collect::<Vec<_>>()
                .join("|")
        }),
        mean_stockpile: read(&|law| gates::mean_stockpile(model, law).to_string()),
        blank: read(&|law| gates::notebook_probability(model, law, None).to_string()),
        correct_fraction: read(&|law| show_ratio(gates::correct_fraction(model, law))),
        k_one: read(&|law| {
            gates::k_one_configurations(model, law)
                .into_iter()
                .map(|config| model.config_label(config))
                .collect::<Vec<_>>()
                .join(";")
        }),
        witness,
    }
}

fn evaluate_case(case: Case, mutation: Mutation) -> CaseEvaluation {
    let model = Model::build(case, mutation);
    let context = Context::new(&model);
    let mut gates = Vec::with_capacity(GateId::ALL.len());
    let mut first_failure = None;
    for gate in GateId::ALL {
        match gate.evaluate(&context) {
            Ok(Status::Pass(note)) => gates.push(GateOutcome {
                gate,
                status: GateStatus::Pass,
                failure: None,
                configuration: None,
                detail: note,
            }),
            Ok(Status::NotApplicable(reason)) => gates.push(GateOutcome {
                gate,
                status: GateStatus::NotApplicable,
                failure: None,
                configuration: None,
                detail: reason,
            }),
            Err(found) => {
                gates.push(GateOutcome {
                    gate,
                    status: GateStatus::Fail,
                    failure: Some(found.check),
                    configuration: Some(
                        found
                            .configuration
                            .map_or("global".to_string(), |config| model.config_label(config)),
                    ),
                    detail: found.detail.clone(),
                });
                if first_failure.is_none() {
                    first_failure = Some(found);
                }
                // Controls record only their first failure; the preregistered
                // construction reports every gate (boundary section 9).
                if mutation != Mutation::None {
                    break;
                }
            }
        }
    }
    let summary = (mutation == Mutation::None).then(|| summarise(&model, &context));
    CaseEvaluation {
        gates,
        first_failure,
        summary,
    }
}

fn evaluate_control(spec: &ControlSpec) -> ControlOutcome {
    let case = match spec.mutation {
        Mutation::StockpileSlip => Case::new(PRIMARY.n, PRIMARY.tau, PRIMARY.b, SLIPPED_STOCKPILE),
        _ => PRIMARY,
    };
    let evaluation = evaluate_case(case, spec.mutation);
    let bridges: Vec<(usize, usize)> = (0..case.n)
        .flat_map(|u| ((u + 1)..case.n).map(move |w| (u, w)))
        .collect();
    let label = |config: usize| model::config_label(&bridges, config);
    let observed = evaluation.first_failure.as_ref();
    let observed_failure = observed.map(|found| found.check);
    let observed_config = observed.and_then(|found| found.configuration);
    let expected_config = match spec.at {
        ExpectedAt::Configuration(config) => Some(config),
        ExpectedAt::Global => None,
    };
    let expected_value = spec
        .value
        .map(|(numerator, denominator)| Ratio::new(numerator, denominator));
    let value_matches =
        expected_value.is_none_or(|value| observed.and_then(|found| found.value) == Some(value));
    let passed = observed.is_some()
        && observed_failure == Some(spec.expected)
        && observed_config == expected_config
        && value_matches;
    ControlOutcome {
        code: spec.code,
        name: spec.name,
        expected_failure: spec.expected,
        expected_configuration: expected_config.map_or("global".to_string(), label),
        expected_value: expected_value.map(|value| value.to_string()),
        observed_failure,
        observed_configuration: observed
            .map(|found| found.configuration.map_or("global".to_string(), label)),
        observed_detail: observed
            .map(|found| found.detail.clone())
            .unwrap_or_default(),
        passed,
    }
}

pub fn evaluate() -> ProofReport {
    let primary = evaluate_case(PRIMARY, Mutation::None);
    let controls = CONTROLS.iter().map(evaluate_control).collect();

    let (witness, instrument) = match &primary.summary {
        Some(summary) => (
            summary.witness.clone(),
            format!(
                "INSTRUMENT {} erasure_ratio={} mean_stockpile={} blank={} correct_fraction={}",
                PRIMARY.label(),
                summary.erasure,
                summary.mean_stockpile,
                summary.blank,
                summary.correct_fraction
            ),
        ),
        None => (
            Vec::new(),
            format!("INSTRUMENT {} {UNAVAILABLE}", PRIMARY.label()),
        ),
    };

    let family = FAMILY
        .iter()
        .map(|&case| {
            let evaluation = evaluate_case(case, Mutation::None);
            let summary = evaluation
                .summary
                .expect("preregistered cases carry a summary");
            FamilyOutcome {
                case: case.label(),
                n: case.n,
                configurations: summary.configurations,
                system_states: summary.system_states,
                complete_states: summary.complete_states,
                partition: summary.partition,
                mean_bridges: summary.mean_bridges,
                erasure: summary.erasure,
                mean_stockpile: summary.mean_stockpile,
                blank: summary.blank,
                k_one: summary.k_one,
                gates: evaluation.gates,
            }
        })
        .collect();

    ProofReport {
        gates: primary.gates,
        controls,
        witness,
        instrument,
        family,
    }
}
