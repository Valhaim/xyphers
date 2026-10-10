//! CAL-CEF-1 exact verifier: causal entropic thermodynamics of a traveller
//! holding committed tau-step plans on a fixed archipelago.
//!
//! Boundary: `research/physics/derivable/xypher-causal-entropic-thermodynamics-boundary.md`.

mod contact;
mod gates;
mod model;
mod ratio;

use std::cell::OnceCell;
use std::fmt::Write;

use contact::ContactAnalysis;
use gates::{Context, Failure, GateResult, Status};
use model::{Case, Model, Mutation, FAMILY, PRIMARY};
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
            Self::G01 => "Crystal grounding",
            Self::G02 => "executable plans",
            Self::G03 => "reciprocal generator",
            Self::G04 => "pathwise accounting",
            Self::G05 => "reservoir lumpability",
            Self::G06 => "system-state local detailed balance",
            Self::G07 => "connectivity and equilibrium",
            Self::G08 => "configuration law",
            Self::G09 => "emergent causal entropic odds",
            Self::G10 => "Xypher slot reading",
            Self::G11 => "same-temperature contact",
            Self::G12 => "home-bridge law",
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
        }
    }

    /// Section 7 allows N/A only for G05 at N = 4 and G12 at tau != 2.
    const fn may_be_not_applicable(self, n: usize, tau: usize) -> bool {
        matches!(self, Self::G05) && n == 4 || matches!(self, Self::G12) && tau != 2
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
    G01_REPLAN_ALPHABET => G01 4,
    G01_SYSTEM_STATE_COUNT => G01 5,
    G02_PLAN_VALIDITY => G02 1,
    G02_STEP_LOCALITY => G02 2,
    G02_TOGGLE_INCIDENCE => G02 3,
    G02_HELD_PLAN_PROTECTION => G02 4,
    G02_PLAN_CONTENTS_READ => G02 5,
    G03_RECIPROCAL_SUPPORT => G03 1,
    G03_EQUAL_REVERSE_HAZARDS => G03 2,
    G03_ROW_CLOSURE => G03 3,
    G04_TOGGLE_ENERGY => G04 1,
    G04_HEAT_EQUALS_DELTA_U => G04 2,
    G04_ZERO_WORK => G04 3,
    G04_NEUTRAL_CHANNELS => G04 4,
    G04_REGISTER_REVERSIBILITY => G04 5,
    G05_STRONG_LUMPABILITY => G05 1,
    G05_LUMPED_HAZARDS => G05 2,
    G06_RESERVOIR_RELATION => G06 1,
    G06_LOCAL_DETAILED_BALANCE => G06 2,
    G07_CONNECTIVITY => G07 1,
    G07_STATIONARITY => G07 2,
    G07_DETAILED_BALANCE => G07 3,
    G08_CONFIGURATION_MARGINAL => G08 1,
    G08_PARTITION_AND_MEAN => G08 2,
    G08_FROZEN_TABLE => G08 3,
    G09_XI_ANTISYMMETRY => G09 1,
    G09_EMERGENT_ODDS => G09 2,
    G09_NON_LUMPABILITY => G09 3,
    G10_NAMED_SLOTS => G10 1,
    G10_EMPTY_RUBY => G10 2,
    G10_HAZARD_INPUTS => G10 3,
    G11_PREPARED_STATIONARY => G11 1,
    G11_COMPONENT_UNIFORM => G11 2,
    G11_ZERO_CURRENT => G11 3,
    G12_COUNT_FORMULA => G12 1,
    G12_DELTA_FORMULA => G12 2,
    G12_HOME_BRIDGE_ODDS => G12 3,
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
    pub fn passed(&self, n: usize, tau: usize) -> bool {
        match self.status {
            GateStatus::Pass => true,
            GateStatus::NotApplicable => self.gate.may_be_not_applicable(n, tau),
            GateStatus::Fail => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlOutcome {
    pub code: &'static str,
    pub name: &'static str,
    pub expected_failure: CheckId,
    pub expected_configuration: Option<String>,
    pub observed_failure: Option<CheckId>,
    pub observed_configuration: Option<String>,
    pub observed_detail: String,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FamilyOutcome {
    pub case: String,
    pub n: usize,
    pub tau: usize,
    pub configurations: usize,
    pub system_states: usize,
    pub complete_states: usize,
    pub partition: String,
    pub mean_bridges: String,
    pub gates: Vec<GateOutcome>,
}

impl FamilyOutcome {
    pub fn passed(&self) -> bool {
        self.gates.len() == GateId::ALL.len()
            && self
                .gates
                .iter()
                .all(|outcome| outcome.passed(self.n, self.tau))
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
    pub witness: String,
    pub family: Vec<FamilyOutcome>,
}

impl ProofReport {
    pub fn is_success(&self) -> bool {
        self.gates.len() == GateId::ALL.len()
            && self
                .gates
                .iter()
                .all(|outcome| outcome.passed(PRIMARY.n, PRIMARY.tau))
            && self.controls.len() == CONTROLS.len()
            && self.controls.iter().all(|outcome| outcome.passed)
            && self.family.len() == FAMILY.len()
            && self.family.iter().all(FamilyOutcome::passed)
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        writeln!(
            output,
            "XYPHER_CAUSAL_ENTROPIC_PROOF CAL-CEF-1 primary=(3,2,2)"
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
                write!(output, " {}", check.code()).unwrap();
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
                        "{} check {} {}{}",
                        check.gate().code(),
                        check.index(),
                        check.code(),
                        configuration_suffix(&outcome.observed_configuration)
                    )
                })
                .unwrap_or_else(|| "NO_FAILURE".to_string());
            write!(
                output,
                "{} {} {} -> {} (expected {} check {} {}{})",
                outcome.code,
                if outcome.passed { "PASS" } else { "FAIL" },
                outcome.name,
                observed,
                outcome.expected_failure.gate().code(),
                outcome.expected_failure.index(),
                outcome.expected_failure.code(),
                configuration_suffix(&outcome.expected_configuration)
            )
            .unwrap();
            if !outcome.observed_detail.is_empty() {
                write!(output, " -- {}", outcome.observed_detail).unwrap();
            }
            writeln!(output).unwrap();
        }
        writeln!(output, "{}", self.witness).unwrap();
        for family in &self.family {
            writeln!(
                output,
                "FAMILY {} configurations={} system={} complete={} Z={} mean_bridges={} gates {}",
                family.case,
                family.configurations,
                family.system_states,
                family.complete_states,
                family.partition,
                family.mean_bridges,
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

fn configuration_suffix(configuration: &Option<String>) -> String {
    configuration
        .as_ref()
        .map(|label| format!(" at {label}"))
        .unwrap_or_default()
}

struct ControlSpec {
    code: &'static str,
    name: &'static str,
    mutation: Mutation,
    expected: CheckId,
    /// Configuration (bit mask) where section 8 names the first failure.
    configuration: Option<usize>,
}

const CONTROLS: [ControlSpec; 8] = [
    ControlSpec {
        code: "C01",
        name: "one-way-builder",
        mutation: Mutation::OneWayBuilder,
        expected: CheckId::G03_RECIPROCAL_SUPPORT,
        configuration: None,
    },
    ControlSpec {
        code: "C02",
        name: "destination-readout",
        mutation: Mutation::DestinationReadout,
        expected: CheckId::G01_REPLAN_UNIFORM,
        configuration: Some(0b011),
    },
    ControlSpec {
        code: "C03",
        name: "directed-kinetic-mutation",
        mutation: Mutation::DirectedKinetic,
        expected: CheckId::G03_EQUAL_REVERSE_HAZARDS,
        configuration: None,
    },
    ControlSpec {
        code: "C04",
        name: "unpaired-receipt",
        mutation: Mutation::UnpairedReceipt,
        expected: CheckId::G09_XI_ANTISYMMETRY,
        configuration: None,
    },
    ControlSpec {
        code: "C05",
        name: "no-staying",
        mutation: Mutation::NoStaying,
        expected: CheckId::G01_COUNT_POSITIVE,
        configuration: Some(0),
    },
    ControlSpec {
        code: "C06",
        name: "horizon-slip",
        mutation: Mutation::HorizonSlip,
        expected: CheckId::G01_REPLAN_ALPHABET,
        configuration: Some(0),
    },
    ControlSpec {
        code: "C07",
        name: "plan-breaker",
        mutation: Mutation::PlanBreaker,
        expected: CheckId::G02_PLAN_VALIDITY,
        configuration: None,
    },
    ControlSpec {
        code: "C08",
        name: "unequal-reservoirs",
        mutation: Mutation::UnequalReservoirs,
        expected: CheckId::G11_PREPARED_STATIONARY,
        configuration: None,
    },
];

struct CaseEvaluation {
    model: Model,
    gates: Vec<GateOutcome>,
    first_failure: Option<Failure>,
}

fn evaluate_case(
    case: Case,
    mutation: Mutation,
    contact: &OnceCell<ContactAnalysis>,
) -> CaseEvaluation {
    let model = Model::build(case, mutation);
    let context = Context {
        model: &model,
        contact,
    };
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
                    configuration: found.configuration.map(|config| model.config_label(config)),
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
    CaseEvaluation {
        model,
        gates,
        first_failure,
    }
}

fn witness_line(model: &Model, contact: &ContactAnalysis) -> String {
    let join = |values: Vec<String>| values.join(",");
    let configs = 0..model.config_count;
    let counts = join(
        configs
            .clone()
            .map(|config| model.matrix_count[config].to_string())
            .collect(),
    );
    let system = join(
        configs
            .clone()
            .map(|config| model.states_of(config).len().to_string())
            .collect(),
    );
    let complete = join(
        configs
            .clone()
            .map(|config| match &model.micro {
                Some(micro) => (micro.offsets[model.config_offsets[config + 1]]
                    - micro.offsets[model.config_offsets[config]])
                    .to_string(),
                None => "-".to_string(),
            })
            .collect(),
    );
    let weights = join(
        configs
            .map(|config| {
                (Ratio::integer(model.matrix_count[config]) * model.boltzmann_weight(config))
                    .to_string()
            })
            .collect(),
    );
    let currents = join(
        contact
            .pairs
            .iter()
            .map(|pair| {
                let shells: Vec<String> = pair
                    .shells
                    .iter()
                    .map(|shell| format!("M{}:{}", shell.shell, shell.current))
                    .collect();
                format!("{}=[{}]", pair.name, shells.join(" "))
            })
            .collect(),
    );
    format!(
        "WITNESS {} N_2={counts} system={system} complete={complete} weights={weights} contact_current={currents}",
        model.case.label()
    )
}

pub fn evaluate() -> ProofReport {
    let baseline_contact = OnceCell::new();
    let primary = evaluate_case(PRIMARY, Mutation::None, &baseline_contact);

    let controls = CONTROLS
        .iter()
        .map(|spec| {
            let contact = OnceCell::new();
            let evaluation = evaluate_case(PRIMARY, spec.mutation, &contact);
            let observed = evaluation.first_failure.as_ref();
            let observed_failure = observed.map(|found| found.check);
            let observed_config = observed.and_then(|found| found.configuration);
            let passed = observed_failure == Some(spec.expected)
                && (spec.configuration.is_none() || observed_config == spec.configuration);
            ControlOutcome {
                code: spec.code,
                name: spec.name,
                expected_failure: spec.expected,
                expected_configuration: spec
                    .configuration
                    .map(|config| evaluation.model.config_label(config)),
                observed_failure,
                observed_configuration: observed_config
                    .map(|config| evaluation.model.config_label(config)),
                observed_detail: observed
                    .map(|found| found.detail.clone())
                    .unwrap_or_default(),
                passed,
            }
        })
        .collect();

    let witness = witness_line(
        &primary.model,
        baseline_contact.get_or_init(|| ContactAnalysis::compute(Mutation::None)),
    );

    let family = FAMILY
        .iter()
        .map(|&case| {
            let evaluation = evaluate_case(case, Mutation::None, &baseline_contact);
            let model = &evaluation.model;
            FamilyOutcome {
                case: case.label(),
                n: case.n,
                tau: case.tau,
                configurations: model.config_count,
                system_states: model.states.len(),
                complete_states: model.complete_state_count(),
                partition: gates::partition_function(model).to_string(),
                mean_bridges: gates::mean_bridges(model).to_string(),
                gates: evaluation.gates,
            }
        })
        .collect();

    ProofReport {
        gates: primary.gates,
        controls,
        witness,
        family,
    }
}
