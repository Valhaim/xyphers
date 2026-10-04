pub mod exact;
pub mod fixtures;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fmt::Write;

use exact::{ExactLog, Ratio, ScaledLog, Temperature, TemperatureSet};
use fixtures::{
    confirmatory_cases, inherited_calibration, integer_power, ConfirmatoryCase, ControlId, Lane,
    CYCLE_EDGES, TOTAL_ENERGY_INDEX,
};

pub const BOUNDARY_COMMIT: &str = "262cb0c076c78c7cc381fd9db00db563bf341b3b";
pub const BOUNDARY_SHA256: &str =
    "780e60cb3f08318edcb269718aea2e1ae7f98252bc143290b60264201cbe8859";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
}

impl GateId {
    pub const ALL: [Self; 13] = [
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
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::G01 => "G01_TYPED_SOURCES",
            Self::G02 => "G02_RESERVOIR_SLOPE",
            Self::G03 => "G03_MICRO_LUMPING",
            Self::G04 => "G04_LDB_CLASSIFICATION",
            Self::G05 => "G05_BODY_INVARIANCE",
            Self::G06 => "G06_ENERGY_GAUGE",
            Self::G07 => "G07_COMPONENT_INTERSECTION",
            Self::G08 => "G08_TAU_GROSS_PATH",
            Self::G09 => "G09_TAU_SIGNED_PAIR",
            Self::G10 => "G10_VARIABLE_INTEGRABILITY",
            Self::G11 => "G11_HEAT_WORK_NONIDENTITY",
            Self::G12 => "G12_PROTOCOL_PRICE_SCOPE",
            Self::G13 => "G13_EOS_SCOPE",
        }
    }

    pub const fn short(self) -> &'static str {
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
        }
    }

    const fn index(self) -> usize {
        match self {
            Self::G01 => 0,
            Self::G02 => 1,
            Self::G03 => 2,
            Self::G04 => 3,
            Self::G05 => 4,
            Self::G06 => 5,
            Self::G07 => 6,
            Self::G08 => 7,
            Self::G09 => 8,
            Self::G10 => 9,
            Self::G11 => 10,
            Self::G12 => 11,
            Self::G13 => 12,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GateOutcome {
    pub gate: GateId,
    pub passed: bool,
    pub detail: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlGateStatus {
    NotApplicable,
    Pass,
    Fail,
}

impl ControlGateStatus {
    const fn label(self) -> &'static str {
        match self {
            Self::NotApplicable => "N/A",
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlOutcome {
    pub control: ControlId,
    pub lane: Lane,
    pub gate_statuses: [ControlGateStatus; 13],
    pub expected: &'static str,
    pub observed: String,
    pub passed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofReport {
    pub gates: Vec<GateOutcome>,
    pub controls: Vec<ControlOutcome>,
    pub rate_verdict: RateDerivedVerdict,
}

impl ProofReport {
    pub fn is_success(&self) -> bool {
        self.gates.len() == GateId::ALL.len()
            && self
                .gates
                .iter()
                .zip(GateId::ALL)
                .all(|(outcome, expected)| outcome.gate == expected)
            && self.gates.iter().all(|outcome| outcome.passed)
            && self.controls.len() == ControlId::ALL.len()
            && self
                .controls
                .iter()
                .zip(ControlId::ALL)
                .all(|(outcome, expected)| outcome.control == expected)
            && self.controls.iter().all(|outcome| outcome.passed)
            && self.rate_verdict == RateDerivedVerdict::Unique
    }

    fn gate_passed(&self, gate: GateId) -> bool {
        self.gates
            .iter()
            .find(|outcome| outcome.gate == gate)
            .is_some_and(|outcome| outcome.passed)
    }

    pub fn render(&self) -> String {
        let calibration = inherited_calibration();
        let mut output = String::new();
        writeln!(output, "CAL_ALPHA_0_EXACT_VERIFIER v1").unwrap();
        writeln!(
            output,
            "BOUNDARY commit={BOUNDARY_COMMIT} sha256={BOUNDARY_SHA256}"
        )
        .unwrap();
        writeln!(
            output,
            "CALIBRATION b={} lambda={} g=1,4,4 n=0,1,2 fibers=4,8,4 rates=8,4,4,8 status={}",
            calibration.reservoir_base,
            calibration.lambda,
            if calibration.evaluation_data {
                "EVALUATION"
            } else {
                "INHERITED_NON_EVALUATION"
            }
        )
        .unwrap();
        for outcome in &self.gates {
            writeln!(
                output,
                "{} {} {}",
                outcome.gate.code(),
                if outcome.passed { "PASS" } else { "FAIL" },
                outcome.detail
            )
            .unwrap();
        }
        for outcome in &self.controls {
            write!(
                output,
                "{} {} {} lane={} expected={} observed={} gates=",
                outcome.control.code(),
                if outcome.passed { "PASS" } else { "FAIL" },
                outcome.control.label(),
                outcome.lane.label(),
                outcome.expected,
                outcome.observed
            )
            .unwrap();
            for (index, gate) in GateId::ALL.iter().enumerate() {
                if index > 0 {
                    output.push(',');
                }
                write!(
                    output,
                    "{}:{}",
                    gate.short(),
                    outcome.gate_statuses[index].label()
                )
                .unwrap();
            }
            output.push('\n');
        }
        let ldb = self.rate_verdict.label();
        let reservoir = if self.gate_passed(GateId::G02) && self.gate_passed(GateId::G04) {
            "OPERATIONAL_TEMPERATURE"
        } else if !self.gate_passed(GateId::G02) {
            "NOT_ADMITTED:G02_RESERVOIR_SLOPE"
        } else {
            "NOT_ADMITTED:G04_LDB_CLASSIFICATION"
        };
        let (gate_statistic, accounting_price) = if self.gate_passed(GateId::G12) {
            ("GATE_STATISTIC", "ACCOUNTING_PRICE_INTERFACE")
        } else {
            ("SCOPE_FAILED", "SCOPE_FAILED")
        };
        let tau = if self.gate_passed(GateId::G08) && self.gate_passed(GateId::G13) {
            "GROSS_PATH_RECEIPT"
        } else {
            "SCOPE_FAILED"
        };
        let scaling = if self.gate_passed(GateId::G01) {
            "SCALING_HYPOTHESIS"
        } else {
            "SCOPE_FAILED"
        };
        let signed_tau = if self.gate_passed(GateId::G09) {
            "SIGNED_ENTROPY_TERM"
        } else {
            "SCOPE_FAILED"
        };
        let (source_form, divided_form) = if self.gate_passed(GateId::G10) {
            ("NONINTEGRABLE_EDGE_FORM", "STATE_POTENTIAL")
        } else {
            ("SCOPE_FAILED", "SCOPE_FAILED")
        };
        let heat_work = if self.gate_passed(GateId::G11) {
            "UNIDENTIFIED"
        } else {
            "SCOPE_FAILED"
        };
        writeln!(
            output,
            "VERDICTS T_LDB={ldb} T_R={reservoir} a_P={gate_statistic} p_o={accounting_price} TAU_PLUS={tau}"
        )
        .unwrap();
        writeln!(
            output,
            "VERDICTS_MORE A_N={scaling} p_N={scaling} TAU_SIGNED={signed_tau} source_p_dS={source_form} divided_p_dS={divided_form} TAU_HEAT_WORK={heat_work}"
        )
        .unwrap();
        writeln!(
            output,
            "OVERALL {}",
            if self.is_success() { "PASS" } else { "FAIL" }
        )
        .unwrap();
        output
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GateFailure(String);

fn require(condition: bool, detail: impl Into<String>) -> Result<(), GateFailure> {
    if condition {
        Ok(())
    } else {
        Err(GateFailure(detail.into()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SourceKind {
    ReservoirFiniteDifference,
    ReciprocalRateRatio,
    ActionHistory,
    StateAccounting,
    NetworkAggregate,
    NetworkPerUnit,
    SymmetricActivity,
    TransitionReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ObservableKind {
    ReservoirEntropy,
    BodyEntropy,
    ActedPhi,
    DeclaredEntropy,
    NetworkSize,
    ChannelOpportunity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LogBase {
    Natural,
    NotApplicable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Unit {
    EnergyPerNat,
    NatPerEnergy,
    DimensionlessGate,
    WorkPerNat,
    Aggregate,
    AggregatePerNode,
    PerTime,
    Work,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Role {
    Temperature,
    InverseTemperature,
    KineticCompatibilityTemperature,
    KineticCompatibilityInverse,
    GateStatistic,
    AccountingPrice,
    AggregateScaling,
    PerUnitScaling,
    KineticClock,
    GrossReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TypedCandidate {
    symbol: &'static str,
    legacy_alias: Option<&'static str>,
    source: SourceKind,
    observable: ObservableKind,
    log_base: LogBase,
    unit: Unit,
    role: Role,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ManifestRow {
    symbol: &'static str,
    legacy_alias: Option<&'static str>,
    source: SourceKind,
    observable: ObservableKind,
    log_base: LogBase,
    unit: Unit,
    role: Role,
}

const FROZEN_TYPED_MANIFEST: [ManifestRow; 10] = [
    ManifestRow {
        symbol: "T_R",
        legacy_alias: Some("alpha"),
        source: SourceKind::ReservoirFiniteDifference,
        observable: ObservableKind::ReservoirEntropy,
        log_base: LogBase::Natural,
        unit: Unit::EnergyPerNat,
        role: Role::Temperature,
    },
    ManifestRow {
        symbol: "beta_R",
        legacy_alias: Some("alpha"),
        source: SourceKind::ReservoirFiniteDifference,
        observable: ObservableKind::ReservoirEntropy,
        log_base: LogBase::Natural,
        unit: Unit::NatPerEnergy,
        role: Role::InverseTemperature,
    },
    ManifestRow {
        symbol: "T_LDB",
        legacy_alias: None,
        source: SourceKind::ReciprocalRateRatio,
        observable: ObservableKind::BodyEntropy,
        log_base: LogBase::Natural,
        unit: Unit::EnergyPerNat,
        role: Role::KineticCompatibilityTemperature,
    },
    ManifestRow {
        symbol: "beta_LDB",
        legacy_alias: None,
        source: SourceKind::ReciprocalRateRatio,
        observable: ObservableKind::BodyEntropy,
        log_base: LogBase::Natural,
        unit: Unit::NatPerEnergy,
        role: Role::KineticCompatibilityInverse,
    },
    ManifestRow {
        symbol: "a_P",
        legacy_alias: Some("alpha"),
        source: SourceKind::ActionHistory,
        observable: ObservableKind::ActedPhi,
        log_base: LogBase::NotApplicable,
        unit: Unit::DimensionlessGate,
        role: Role::GateStatistic,
    },
    ManifestRow {
        symbol: "p_o",
        legacy_alias: Some("alpha"),
        source: SourceKind::StateAccounting,
        observable: ObservableKind::DeclaredEntropy,
        log_base: LogBase::Natural,
        unit: Unit::WorkPerNat,
        role: Role::AccountingPrice,
    },
    ManifestRow {
        symbol: "A(N)",
        legacy_alias: Some("alpha"),
        source: SourceKind::NetworkAggregate,
        observable: ObservableKind::NetworkSize,
        log_base: LogBase::NotApplicable,
        unit: Unit::Aggregate,
        role: Role::AggregateScaling,
    },
    ManifestRow {
        symbol: "p_N",
        legacy_alias: Some("alpha"),
        source: SourceKind::NetworkPerUnit,
        observable: ObservableKind::NetworkSize,
        log_base: LogBase::NotApplicable,
        unit: Unit::AggregatePerNode,
        role: Role::PerUnitScaling,
    },
    ManifestRow {
        symbol: "kappa_a",
        legacy_alias: Some("alpha"),
        source: SourceKind::SymmetricActivity,
        observable: ObservableKind::ChannelOpportunity,
        log_base: LogBase::NotApplicable,
        unit: Unit::PerTime,
        role: Role::KineticClock,
    },
    ManifestRow {
        symbol: "TAU_+",
        legacy_alias: None,
        source: SourceKind::TransitionReceipt,
        observable: ObservableKind::DeclaredEntropy,
        log_base: LogBase::Natural,
        unit: Unit::Work,
        role: Role::GrossReceipt,
    },
];

fn typed_candidates() -> [TypedCandidate; 10] {
    [
        TypedCandidate {
            symbol: "T_R",
            legacy_alias: Some("alpha"),
            source: SourceKind::ReservoirFiniteDifference,
            observable: ObservableKind::ReservoirEntropy,
            log_base: LogBase::Natural,
            unit: Unit::EnergyPerNat,
            role: Role::Temperature,
        },
        TypedCandidate {
            symbol: "beta_R",
            legacy_alias: Some("alpha"),
            source: SourceKind::ReservoirFiniteDifference,
            observable: ObservableKind::ReservoirEntropy,
            log_base: LogBase::Natural,
            unit: Unit::NatPerEnergy,
            role: Role::InverseTemperature,
        },
        TypedCandidate {
            symbol: "T_LDB",
            legacy_alias: None,
            source: SourceKind::ReciprocalRateRatio,
            observable: ObservableKind::BodyEntropy,
            log_base: LogBase::Natural,
            unit: Unit::EnergyPerNat,
            role: Role::KineticCompatibilityTemperature,
        },
        TypedCandidate {
            symbol: "beta_LDB",
            legacy_alias: None,
            source: SourceKind::ReciprocalRateRatio,
            observable: ObservableKind::BodyEntropy,
            log_base: LogBase::Natural,
            unit: Unit::NatPerEnergy,
            role: Role::KineticCompatibilityInverse,
        },
        TypedCandidate {
            symbol: "a_P",
            legacy_alias: Some("alpha"),
            source: SourceKind::ActionHistory,
            observable: ObservableKind::ActedPhi,
            log_base: LogBase::NotApplicable,
            unit: Unit::DimensionlessGate,
            role: Role::GateStatistic,
        },
        TypedCandidate {
            symbol: "p_o",
            legacy_alias: Some("alpha"),
            source: SourceKind::StateAccounting,
            observable: ObservableKind::DeclaredEntropy,
            log_base: LogBase::Natural,
            unit: Unit::WorkPerNat,
            role: Role::AccountingPrice,
        },
        TypedCandidate {
            symbol: "A(N)",
            legacy_alias: Some("alpha"),
            source: SourceKind::NetworkAggregate,
            observable: ObservableKind::NetworkSize,
            log_base: LogBase::NotApplicable,
            unit: Unit::Aggregate,
            role: Role::AggregateScaling,
        },
        TypedCandidate {
            symbol: "p_N",
            legacy_alias: Some("alpha"),
            source: SourceKind::NetworkPerUnit,
            observable: ObservableKind::NetworkSize,
            log_base: LogBase::NotApplicable,
            unit: Unit::AggregatePerNode,
            role: Role::PerUnitScaling,
        },
        TypedCandidate {
            symbol: "kappa_a",
            legacy_alias: Some("alpha"),
            source: SourceKind::SymmetricActivity,
            observable: ObservableKind::ChannelOpportunity,
            log_base: LogBase::NotApplicable,
            unit: Unit::PerTime,
            role: Role::KineticClock,
        },
        TypedCandidate {
            symbol: "TAU_+",
            legacy_alias: None,
            source: SourceKind::TransitionReceipt,
            observable: ObservableKind::DeclaredEntropy,
            log_base: LogBase::Natural,
            unit: Unit::Work,
            role: Role::GrossReceipt,
        },
    ]
}

fn role_accepts_unit(role: Role, unit: Unit) -> bool {
    matches!(
        (role, unit),
        (Role::Temperature, Unit::EnergyPerNat)
            | (Role::InverseTemperature, Unit::NatPerEnergy)
            | (Role::KineticCompatibilityTemperature, Unit::EnergyPerNat)
            | (Role::KineticCompatibilityInverse, Unit::NatPerEnergy)
            | (Role::GateStatistic, Unit::DimensionlessGate)
            | (Role::AccountingPrice, Unit::WorkPerNat)
            | (Role::AggregateScaling, Unit::Aggregate)
            | (Role::PerUnitScaling, Unit::AggregatePerNode)
            | (Role::KineticClock, Unit::PerTime)
            | (Role::GrossReceipt, Unit::Work)
    )
}

fn resolve_candidate<'a>(
    name: &str,
    candidates: &'a [TypedCandidate],
) -> Result<&'a TypedCandidate, GateFailure> {
    let matches: Vec<_> = candidates
        .iter()
        .filter(|candidate| candidate.symbol == name || candidate.legacy_alias == Some(name))
        .collect();
    match matches.as_slice() {
        [candidate] => Ok(*candidate),
        [] => Err(GateFailure(format!("unknown typed symbol {name}"))),
        _ => Err(GateFailure(format!(
            "bare symbol {name} is ambiguous across {} typed candidates",
            matches.len()
        ))),
    }
}

fn check_typed_sources(inverse_label_control: bool) -> Result<(), GateFailure> {
    let mut candidates = typed_candidates();
    if inverse_label_control {
        candidates[1].role = Role::Temperature;
    }
    let mut symbols = BTreeSet::new();
    for (candidate, expected) in candidates.into_iter().zip(FROZEN_TYPED_MANIFEST) {
        require(!candidate.symbol.is_empty(), "empty typed symbol")?;
        require(symbols.insert(candidate.symbol), "duplicate typed symbol")?;
        require(
            role_accepts_unit(candidate.role, candidate.unit),
            format!(
                "{} has role {:?} but unit {:?}",
                candidate.symbol, candidate.role, candidate.unit
            ),
        )?;
        require(candidate.symbol == expected.symbol, "typed symbol changed")?;
        require(
            candidate.legacy_alias == expected.legacy_alias,
            format!("{} legacy alias changed", candidate.symbol),
        )?;
        require(
            candidate.source == expected.source,
            format!("{} source changed", candidate.symbol),
        )?;
        require(
            candidate.observable == expected.observable,
            format!("{} observable changed", candidate.symbol),
        )?;
        require(
            candidate.log_base == expected.log_base,
            format!("{} log base changed", candidate.symbol),
        )?;
        require(
            candidate.unit == expected.unit,
            format!("{} unit changed", candidate.symbol),
        )?;
        require(
            candidate.role == expected.role,
            format!("{} role changed", candidate.symbol),
        )?;
    }
    let alpha_candidates: Vec<_> = typed_candidates()
        .into_iter()
        .filter(|candidate| candidate.legacy_alias == Some("alpha"))
        .collect();
    for (index, left) in alpha_candidates.iter().enumerate() {
        for right in alpha_candidates.iter().skip(index + 1) {
            require(
                left.symbol != right.symbol
                    && (left.source, left.observable, left.unit, left.role)
                        != (right.source, right.observable, right.unit, right.role),
                "bare alpha alias collapsed distinct typed candidates",
            )?;
        }
    }
    require(
        resolve_candidate("alpha", &typed_candidates()).is_err(),
        "bare alpha unexpectedly resolved to one typed quantity",
    )?;
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LdbDatum {
    label: &'static str,
    delta_s: ExactLog,
    delta_u: Ratio,
    rho: Ratio,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum LdbClass {
    Unique(Temperature),
    CompatibleUnidentified,
    Incompatible(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RateDerivedVerdict {
    Unique,
    CompatibleUnidentified,
    Incompatible,
}

impl RateDerivedVerdict {
    const fn label(self) -> &'static str {
        match self {
            Self::Unique => "UNIQUE",
            Self::CompatibleUnidentified => "COMPATIBLE_UNIDENTIFIED",
            Self::Incompatible => "INCOMPATIBLE",
        }
    }
}

impl fmt::Display for LdbClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unique(temperature) => write!(formatter, "UNIQUE:{temperature}"),
            Self::CompatibleUnidentified => write!(formatter, "COMPATIBLE_UNIDENTIFIED"),
            Self::Incompatible(_) => write!(formatter, "INCOMPATIBLE"),
        }
    }
}

fn classify_ldb(channels: &[LdbDatum]) -> LdbClass {
    let mut labels = BTreeSet::new();
    let mut identified: Option<Temperature> = None;
    for channel in channels {
        if !labels.insert(channel.label) {
            return LdbClass::Incompatible(format!("duplicate channel label {}", channel.label));
        }
        if !channel.rho.is_positive() {
            return LdbClass::Incompatible(format!("{} has nonpositive rho", channel.label));
        }
        let ell = ExactLog::of_positive_ratio(channel.rho);
        let d = channel.delta_s.minus(&ell);
        if channel.delta_u.is_zero() {
            if !d.is_zero() {
                return LdbClass::Incompatible(format!(
                    "{} has zero energy but d={d}",
                    channel.label
                ));
            }
            continue;
        }
        if d.is_zero() || d.signum() != channel.delta_u.signum() {
            return LdbClass::Incompatible(format!(
                "{} has no positive finite temperature",
                channel.label
            ));
        }
        let candidate = Temperature::new(channel.delta_u, d);
        if let Some(previous) = &identified {
            if previous != &candidate {
                return LdbClass::Incompatible(format!(
                    "{} requires {candidate}, previous channels require {previous}",
                    channel.label
                ));
            }
        } else {
            identified = Some(candidate);
        }
    }
    identified
        .map(LdbClass::Unique)
        .unwrap_or(LdbClass::CompatibleUnidentified)
}

fn case_channels(
    case: &ConfirmatoryCase,
    evidence: &[CaseHazardEvidence],
) -> Result<Vec<LdbDatum>, GateFailure> {
    const LABELS: [&str; 4] = ["edge-01", "edge-12", "edge-23", "edge-30"];
    let observed = evidence
        .iter()
        .find(|item| {
            item.reservoir_base == case.reservoir_base
                && item.lambda == case.lambda
                && item.body_variant == case.body_variant
        })
        .ok_or_else(|| GateFailure(format!("missing G03 evidence for {}", case.id())))?;
    let mut channels = Vec::with_capacity(4);
    for (&(from, to), label) in CYCLE_EDGES.iter().zip(LABELS) {
        let forward = *observed
            .hazards
            .get(&(from, to))
            .ok_or_else(|| GateFailure(format!("missing {} forward hazard", case.id())))?;
        let reverse = *observed
            .hazards
            .get(&(to, from))
            .ok_or_else(|| GateFailure(format!("missing {} reverse hazard", case.id())))?;
        channels.push(LdbDatum {
            label,
            delta_s: ExactLog::of_positive_ratio(Ratio::new(
                case.degeneracies[to],
                case.degeneracies[from],
            )),
            delta_u: case.energy(to) - case.energy(from),
            rho: forward / reverse,
        });
    }
    Ok(channels)
}

fn gate_01() -> Result<String, GateFailure> {
    check_typed_sources(false)?;
    Ok("candidates=10 bare-alpha-equality=rejected".to_string())
}

fn reservoir_slope_check(
    lambda: Ratio,
    multiplicities: &[i128],
    declared_temperature: &Temperature,
) -> Result<usize, GateFailure> {
    let mut differences = 0_usize;
    for from in 0..multiplicities.len() {
        for to in 0..multiplicities.len() {
            if from == to {
                continue;
            }
            require(
                multiplicities[from] > 0 && multiplicities[to] > 0,
                "reservoir multiplicities must be positive",
            )?;
            let delta_s =
                ExactLog::of_positive_ratio(Ratio::new(multiplicities[to], multiplicities[from]));
            let delta_e = lambda * Ratio::integer(to as i128 - from as i128);
            let predicted = declared_temperature
                .energy_for_log(&delta_s)
                .ok_or_else(|| {
                    GateFailure(format!(
                        "reservoir slope {delta_s} is not proportional to {}",
                        declared_temperature.denominator()
                    ))
                })?;
            require(
                predicted == delta_e,
                format!(
                    "T_R*DeltaS={} but DeltaE={} for reservoir {}->{}",
                    predicted, delta_e, from, to
                ),
            )?;
            differences += 1;
        }
    }
    Ok(differences)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReservoirEvidence {
    reservoir_base: u64,
    lambda: Ratio,
    temperature: Temperature,
    checked_differences: usize,
}

fn derive_primary_reservoir_evidence() -> Result<Vec<ReservoirEvidence>, GateFailure> {
    let reservoir_keys: BTreeSet<_> = confirmatory_cases()
        .into_iter()
        .map(|case| (case.reservoir_base, case.lambda))
        .collect();
    let mut evidence = Vec::with_capacity(reservoir_keys.len());
    for (reservoir_base, lambda) in reservoir_keys {
        let multiplicities: Vec<_> = (0..=TOTAL_ENERGY_INDEX)
            .map(|index| integer_power(reservoir_base as i128, index as u32))
            .collect();
        let temperature = Temperature::from_base(lambda, reservoir_base);
        let checked_differences = reservoir_slope_check(lambda, &multiplicities, &temperature)?;
        evidence.push(ReservoirEvidence {
            reservoir_base,
            lambda,
            temperature,
            checked_differences,
        });
    }
    Ok(evidence)
}

fn gate_02(evidence: &[ReservoirEvidence]) -> Result<String, GateFailure> {
    require(evidence.len() == 4, "reservoir evidence count changed")?;
    let mut differences = 0_usize;
    for item in evidence {
        require(
            item.temperature == Temperature::from_base(item.lambda, item.reservoir_base),
            "reservoir evidence changed after derivation",
        )?;
        differences += item.checked_differences;
    }
    require(
        differences == 48,
        "reservoir finite-difference count changed",
    )?;
    Ok("reservoirs=4 ordered_nonzero_differences=48 beta=ln(b)/lambda".to_string())
}

fn macro_adjacent(left: usize, right: usize) -> bool {
    CYCLE_EDGES
        .iter()
        .any(|&(from, to)| (from == left && to == right) || (from == right && to == left))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MicroAddress {
    macro_state: usize,
    crystal_index: i128,
    reservoir_index: i128,
}

fn valid_micro_address(case: &ConfirmatoryCase, address: MicroAddress) -> bool {
    if address.macro_state >= case.degeneracies.len() {
        return false;
    }
    let reservoir_energy = TOTAL_ENERGY_INDEX - case.energy_indices[address.macro_state];
    let reservoir_count = integer_power(case.reservoir_base as i128, reservoir_energy as u32);
    (0..case.degeneracies[address.macro_state]).contains(&address.crystal_index)
        && (0..reservoir_count).contains(&address.reservoir_index)
}

fn unit_microedge(case: &ConfirmatoryCase, from: MicroAddress, to: MicroAddress) -> bool {
    valid_micro_address(case, from)
        && valid_micro_address(case, to)
        && macro_adjacent(from.macro_state, to.macro_state)
}

fn independently_enumerated_hazard(
    case: &ConfirmatoryCase,
    from: MicroAddress,
    destination_macro: usize,
) -> Result<i128, GateFailure> {
    require(
        valid_micro_address(case, from),
        "invalid microscopic origin",
    )?;
    let reservoir_energy = TOTAL_ENERGY_INDEX - case.energy_indices[destination_macro];
    let reservoir_count = integer_power(case.reservoir_base as i128, reservoir_energy as u32);
    let mut count = 0_i128;
    for crystal_index in 0..case.degeneracies[destination_macro] {
        for reservoir_index in 0..reservoir_count {
            let to = MicroAddress {
                macro_state: destination_macro,
                crystal_index,
                reservoir_index,
            };
            if unit_microedge(case, from, to) {
                require(
                    unit_microedge(case, to, from),
                    "microscopic edge lacks its reciprocal unit edge",
                )?;
                count = count.checked_add(1).expect("microedge count overflow");
            }
        }
    }
    Ok(count)
}

fn independently_enumerated_macro_hazard(
    case: &ConfirmatoryCase,
    from_macro: usize,
    to_macro: usize,
) -> Result<Ratio, GateFailure> {
    require(
        macro_adjacent(from_macro, to_macro),
        "macro hazard requested for a nonadjacent pair",
    )?;
    let from_reservoir_energy = TOTAL_ENERGY_INDEX - case.energy_indices[from_macro];
    let from_reservoir_count =
        integer_power(case.reservoir_base as i128, from_reservoir_energy as u32);
    let mut common = None;
    for crystal_index in 0..case.degeneracies[from_macro] {
        for reservoir_index in 0..from_reservoir_count {
            let from = MicroAddress {
                macro_state: from_macro,
                crystal_index,
                reservoir_index,
            };
            let counted = independently_enumerated_hazard(case, from, to_macro)?;
            if let Some(previous) = common {
                require(
                    counted == previous,
                    format!(
                        "{} {}->{} is not strongly lumpable",
                        case.id(),
                        from_macro,
                        to_macro
                    ),
                )?;
            } else {
                common = Some(counted);
            }
        }
    }
    Ok(Ratio::integer(common.ok_or_else(|| {
        GateFailure("empty microscopic origin fiber".to_string())
    })?))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CaseHazardEvidence {
    reservoir_base: u64,
    lambda: Ratio,
    body_variant: usize,
    hazards: BTreeMap<(usize, usize), Ratio>,
}

fn derive_micro_lumping_evidence() -> Result<Vec<CaseHazardEvidence>, GateFailure> {
    let cases = confirmatory_cases();
    let mut evidence = Vec::with_capacity(cases.len());
    for case in &cases {
        let mut hazards = BTreeMap::new();
        for &(left, right) in &CYCLE_EDGES {
            for (from_macro, to_macro) in [(left, right), (right, left)] {
                let counted = independently_enumerated_macro_hazard(case, from_macro, to_macro)?;
                require(
                    counted == case.macro_hazard(to_macro),
                    format!(
                        "{} {}->{} counted {} but formula predicts {}",
                        case.id(),
                        from_macro,
                        to_macro,
                        counted,
                        case.macro_hazard(to_macro)
                    ),
                )?;
                require(
                    hazards.insert((from_macro, to_macro), counted).is_none(),
                    "duplicate directed macro hazard",
                )?;
            }
        }
        evidence.push(CaseHazardEvidence {
            reservoir_base: case.reservoir_base,
            lambda: case.lambda,
            body_variant: case.body_variant,
            hazards,
        });
    }
    Ok(evidence)
}

fn gate_03(evidence: &[CaseHazardEvidence]) -> Result<String, GateFailure> {
    require(
        evidence.len() == 8
            && evidence
                .iter()
                .map(|case| case.hazards.len())
                .sum::<usize>()
                == 64,
        "directed macro-channel count changed",
    )?;
    Ok("cases=8 directed_channels=64 unit_edges=lazy_complete_bipartite".to_string())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CaseTemperature {
    reservoir_base: u64,
    lambda: Ratio,
    body_variant: usize,
    g_one: i128,
    derived: Temperature,
}

struct LdbEvidenceFailure {
    verdict: RateDerivedVerdict,
    failure: GateFailure,
}

fn derive_primary_case_temperatures(
    micro_evidence: &[CaseHazardEvidence],
) -> Result<Vec<CaseTemperature>, LdbEvidenceFailure> {
    let cases = confirmatory_cases();
    let mut identities = Vec::with_capacity(cases.len());
    for case in &cases {
        let channels =
            case_channels(case, micro_evidence).map_err(|failure| LdbEvidenceFailure {
                verdict: RateDerivedVerdict::Incompatible,
                failure,
            })?;
        if channels.len() != 4 {
            return Err(LdbEvidenceFailure {
                verdict: RateDerivedVerdict::Incompatible,
                failure: GateFailure("channel labels were aggregated".to_string()),
            });
        }
        let derived = match classify_ldb(&channels) {
            LdbClass::Unique(temperature) => temperature,
            classification => {
                let verdict = match &classification {
                    LdbClass::CompatibleUnidentified => RateDerivedVerdict::CompatibleUnidentified,
                    LdbClass::Incompatible(_) => RateDerivedVerdict::Incompatible,
                    LdbClass::Unique(_) => unreachable!(),
                };
                return Err(LdbEvidenceFailure {
                    verdict,
                    failure: GateFailure(format!("{} classified as {classification}", case.id())),
                });
            }
        };
        identities.push(CaseTemperature {
            reservoir_base: case.reservoir_base,
            lambda: case.lambda,
            body_variant: case.body_variant,
            g_one: case.degeneracies[1],
            derived,
        });
    }
    Ok(identities)
}

fn gate_04(
    identities: &[CaseTemperature],
    reservoirs: &[ReservoirEvidence],
) -> Result<String, GateFailure> {
    require(
        identities.len() == 8,
        "confirmatory LDB identity count changed",
    )?;
    for identity in identities {
        let declared_reservoir = reservoirs
            .iter()
            .find(|reservoir| {
                reservoir.reservoir_base == identity.reservoir_base
                    && reservoir.lambda == identity.lambda
            })
            .ok_or_else(|| {
                GateFailure(format!(
                    "no passing G02 reservoir evidence for b={} lambda={}",
                    identity.reservoir_base, identity.lambda
                ))
            })?;
        require(
            identity.derived == declared_reservoir.temperature,
            format!(
                "b={} lambda={} g{} gives T_LDB={} but T_R={}",
                identity.reservoir_base,
                identity.lambda,
                identity.body_variant + 1,
                identity.derived,
                declared_reservoir.temperature
            ),
        )?;
    }
    Ok("cases=8 labelled_pairs=32 A_LDB=UNIQUE T_LDB=T_R".to_string())
}

fn gate_05(identities: &[CaseTemperature]) -> Result<String, GateFailure> {
    let mut by_reservoir: BTreeMap<(u64, Ratio), Temperature> = BTreeMap::new();
    require(
        identities.len() == 8,
        "G05 did not receive all G04 identities",
    )?;
    for identity in identities {
        let key = (identity.reservoir_base, identity.lambda);
        if let Some(previous) = by_reservoir.get(&key) {
            require(
                previous == &identity.derived,
                format!(
                    "body variants at b={} lambda={} yield {} and {}",
                    key.0, key.1, previous, identity.derived
                ),
            )?;
        } else {
            by_reservoir.insert(key, identity.derived.clone());
        }
    }
    require(
        by_reservoir.len() == 4,
        "cross-case reservoir groups changed",
    )?;
    Ok("groups=4 body_variants=2 temperature_invariant".to_string())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SymbolicAffinity {
    entropy_energy: ScaledLog,
    delta_u: Ratio,
}

impl SymbolicAffinity {
    fn scaled_energy(&self, factor: Ratio) -> Self {
        Self {
            entropy_energy: self.entropy_energy.scaled_energy(factor),
            delta_u: self.delta_u * factor,
        }
    }

    fn dimensionless(&self) -> Option<ExactLog> {
        Some(
            self.entropy_energy.logarithm.minus(
                &self
                    .entropy_energy
                    .temperature
                    .dimensionless_energy(self.delta_u)?,
            ),
        )
    }
}

fn tau_plus(temperature: &Temperature, delta_s: &ExactLog) -> ScaledLog {
    if delta_s.signum() > 0 {
        ScaledLog::new(temperature.clone(), delta_s.clone())
    } else {
        ScaledLog::zero(temperature.clone())
    }
}

fn check_energy_gauge(
    primary_reservoirs: &[ReservoirEvidence],
    primary_hazards: &[CaseHazardEvidence],
) -> Result<(), GateFailure> {
    let scale = Ratio::integer(3);
    let offsets = [Ratio::integer(7), Ratio::integer(11)];
    let cases = confirmatory_cases();
    require(cases.len() == 8, "energy-gauge case matrix changed")?;
    for case in cases {
        let base_evidence = primary_hazards
            .iter()
            .find(|item| {
                item.reservoir_base == case.reservoir_base
                    && item.lambda == case.lambda
                    && item.body_variant == case.body_variant
            })
            .ok_or_else(|| {
                GateFailure(format!("missing immutable G03 evidence for {}", case.id()))
            })?;
        let base_reservoir = primary_reservoirs
            .iter()
            .find(|item| item.reservoir_base == case.reservoir_base && item.lambda == case.lambda)
            .ok_or_else(|| {
                GateFailure(format!("missing immutable G02 evidence for {}", case.id()))
            })?;
        let mut transformed_case = case.clone();
        transformed_case.lambda = transformed_case.lambda * scale;
        let base_temperature = base_reservoir.temperature.clone();
        let scaled_temperature = transformed_case.reservoir_temperature();
        require(
            scaled_temperature == Temperature::from_base(case.lambda * scale, case.reservoir_base),
            "temperature did not co-transform under the energy gauge",
        )?;
        for component_offset in offsets {
            for &(from, to) in &CYCLE_EDGES {
                let base_forward = *base_evidence
                    .hazards
                    .get(&(from, to))
                    .ok_or_else(|| GateFailure("missing base forward hazard".to_string()))?;
                let transformed_forward =
                    independently_enumerated_macro_hazard(&transformed_case, from, to)?;
                let base_reverse = *base_evidence
                    .hazards
                    .get(&(to, from))
                    .ok_or_else(|| GateFailure("missing base reverse hazard".to_string()))?;
                let transformed_reverse =
                    independently_enumerated_macro_hazard(&transformed_case, to, from)?;
                require(
                    base_forward == transformed_forward && base_reverse == transformed_reverse,
                    "micro-lumped dynamics changed under a pure energy-unit gauge",
                )?;
                let delta_s = ExactLog::of_positive_ratio(Ratio::new(
                    case.degeneracies[to],
                    case.degeneracies[from],
                ));
                let delta_u = case.energy(to) - case.energy(from);
                let transformed_from = transformed_case.energy(from) + component_offset;
                let transformed_to = transformed_case.energy(to) + component_offset;
                let transformed_delta_u = transformed_to - transformed_from;
                require(
                    transformed_delta_u == scale * delta_u,
                    "component offset changed an energy difference",
                )?;
                let base_tau = tau_plus(&base_temperature, &delta_s);
                let scaled_tau = tau_plus(&scaled_temperature, &delta_s);
                require(
                    scaled_tau == base_tau.scaled_energy(scale),
                    "TAU did not co-transform under the energy gauge",
                )?;
                let base_xi = SymbolicAffinity {
                    entropy_energy: ScaledLog::new(base_temperature.clone(), delta_s.clone()),
                    delta_u,
                };
                let scaled_xi = SymbolicAffinity {
                    entropy_energy: ScaledLog::new(scaled_temperature.clone(), delta_s),
                    delta_u: transformed_delta_u,
                };
                require(
                    scaled_xi == base_xi.scaled_energy(scale),
                    "Xi did not co-transform under the energy gauge",
                )?;
                require(
                    scaled_xi.dimensionless() == base_xi.dimensionless(),
                    "Xi/T changed under the energy gauge",
                )?;
            }
        }
    }
    Ok(())
}

fn gate_06(
    primary_reservoirs: &[ReservoirEvidence],
    primary_hazards: &[CaseHazardEvidence],
) -> Result<String, GateFailure> {
    check_energy_gauge(primary_reservoirs, primary_hazards)?;
    Ok("cases=8 scale=3 offsets=7,11 rates_and_Xi_over_T=invariant".to_string())
}

fn flat_component() -> Vec<LdbDatum> {
    vec![LdbDatum {
        label: "flat",
        delta_s: ExactLog::of_positive_integer(2),
        delta_u: Ratio::ZERO,
        rho: Ratio::integer(2),
    }]
}

fn informative_component(label: &'static str, base: i128, energy_gap: i128) -> Vec<LdbDatum> {
    vec![LdbDatum {
        label,
        delta_s: ExactLog::zero(),
        delta_u: Ratio::integer(energy_gap),
        rho: Ratio::new(1, base),
    }]
}

fn class_as_set(classification: LdbClass) -> TemperatureSet {
    match classification {
        LdbClass::Unique(temperature) => TemperatureSet::Singleton(temperature),
        LdbClass::CompatibleUnidentified => TemperatureSet::AllPositive,
        LdbClass::Incompatible(_) => TemperatureSet::Empty,
    }
}

fn d_all() -> TemperatureSet {
    class_as_set(classify_ldb(&flat_component()))
}

fn d_singleton() -> TemperatureSet {
    class_as_set(classify_ldb(&informative_component("informative-3", 3, 1))).intersect(&d_all())
}

fn d_empty() -> TemperatureSet {
    class_as_set(classify_ldb(&informative_component("informative-3", 3, 1))).intersect(
        &class_as_set(classify_ldb(&informative_component("informative-5", 5, 1))),
    )
}

fn per_component_scale_set() -> TemperatureSet {
    class_as_set(classify_ldb(&informative_component("scaled-2", 3, 2))).intersect(&class_as_set(
        classify_ldb(&informative_component("scaled-3", 3, 3)),
    ))
}

fn gate_07() -> Result<String, GateFailure> {
    require(
        d_all() == TemperatureSet::AllPositive,
        "D_ALL misclassified",
    )?;
    require(
        d_singleton() == TemperatureSet::Singleton(Temperature::from_base(Ratio::ONE, 3)),
        "D_SINGLETON misclassified",
    )?;
    require(d_empty() == TemperatureSet::Empty, "D_EMPTY misclassified")?;
    Ok("D_ALL=ALL_POSITIVE D_SINGLETON={1/ln(3)} D_EMPTY=EMPTY".to_string())
}

fn positive_part(value: Ratio) -> Ratio {
    if value.is_positive() {
        value
    } else {
        Ratio::ZERO
    }
}

fn gross_tau_cycle(assert_state_potential: bool) -> Result<[Ratio; 3], GateFailure> {
    let temperature = Ratio::integer(2);
    let entropy = [Ratio::ZERO, Ratio::ONE, Ratio::ZERO];
    let edges = [(0_usize, 1_usize), (1, 2), (2, 0)];
    let mut receipts = [Ratio::ZERO; 3];
    let mut entropy_period = Ratio::ZERO;
    for (index, &(from, to)) in edges.iter().enumerate() {
        let delta_s = entropy[to] - entropy[from];
        entropy_period = entropy_period + delta_s;
        receipts[index] = temperature * positive_part(delta_s);
    }
    require(
        entropy_period.is_zero(),
        "gross fixture does not close in state",
    )?;
    require(
        receipts == [Ratio::integer(2), Ratio::ZERO, Ratio::ZERO],
        "gross TAU fixture changed",
    )?;
    let receipt_period = receipts
        .into_iter()
        .fold(Ratio::ZERO, |sum, value| sum + value);
    require(
        receipt_period == Ratio::integer(2),
        "gross TAU period is not two",
    )?;
    if assert_state_potential {
        return Err(GateFailure(format!(
            "closed state loop has nonzero gross receipt period {receipt_period}"
        )));
    }
    Ok(receipts)
}

fn gate_08() -> Result<String, GateFailure> {
    gross_tau_cycle(false)?;
    Ok("T=2 S=0,1,0 gross=2,0,0 period=2".to_string())
}

fn signed_tau_cycle(use_unpaired_xi: bool) -> Result<(), GateFailure> {
    let temperature = Ratio::integer(2);
    let entropy = [Ratio::ZERO, Ratio::ONE, Ratio::ZERO];
    let energy = [Ratio::ZERO, Ratio::integer(3), Ratio::ZERO];
    let cycle = [(0_usize, 1_usize), (1, 2), (2, 0)];
    let mut signed_values = [Ratio::ZERO; 3];
    let mut xi_values = [Ratio::ZERO; 3];
    for (index, &(from, to)) in cycle.iter().enumerate() {
        let delta_s = entropy[to] - entropy[from];
        let delta_u = energy[to] - energy[from];
        let forward = temperature * positive_part(delta_s);
        let reverse = temperature * positive_part(-delta_s);
        let signed = forward - reverse;
        require(
            signed == temperature * delta_s,
            "forward-minus-reverse TAU identity failed",
        )?;
        signed_values[index] = signed;
        xi_values[index] = if use_unpaired_xi {
            forward - delta_u
        } else {
            signed - delta_u
        };
    }
    if use_unpaired_xi {
        let forward_bad = temperature * positive_part(Ratio::ONE) - Ratio::integer(3);
        let reverse_bad = temperature * positive_part(-Ratio::ONE) - Ratio::integer(-3);
        require(
            forward_bad == Ratio::integer(-1),
            "C15 forward fixture changed",
        )?;
        require(
            reverse_bad == Ratio::integer(3),
            "C15 reverse fixture changed",
        )?;
        return Err(GateFailure(format!(
            "unpaired Xi reciprocal sum={} instead of zero",
            forward_bad + reverse_bad
        )));
    }
    require(
        signed_values == [Ratio::integer(2), Ratio::integer(-2), Ratio::ZERO],
        "signed TAU fixture changed",
    )?;
    require(
        xi_values == [Ratio::integer(-1), Ratio::ONE, Ratio::ZERO],
        "Xi fixture changed",
    )?;
    require(
        signed_values
            .into_iter()
            .fold(Ratio::ZERO, |sum, value| sum + value)
            .is_zero(),
        "signed TAU cycle did not close",
    )?;
    require(
        xi_values
            .into_iter()
            .fold(Ratio::ZERO, |sum, value| sum + value)
            .is_zero(),
        "Xi cycle did not close",
    )?;
    for &(from, to) in &cycle {
        let delta_s = entropy[to] - entropy[from];
        let delta_u = energy[to] - energy[from];
        let xi = temperature * delta_s - delta_u;
        let reverse_xi = temperature * (-delta_s) - (-delta_u);
        require(xi == -reverse_xi, "Xi is not antisymmetric")?;
    }
    Ok(())
}

fn gate_09() -> Result<String, GateFailure> {
    signed_tau_cycle(false)?;
    Ok("signed=2,-2,0 Xi=-1,1,0 pair_and_cycles=exact".to_string())
}

fn state_price(entropy: Ratio) -> Ratio {
    Ratio::integer(2) * entropy + Ratio::ONE
}

fn state_potential(entropy: Ratio) -> Ratio {
    entropy * entropy + entropy
}

fn variable_price_periods(claim_source_integrable: bool) -> Result<(Ratio, Ratio), GateFailure> {
    let entropy = [Ratio::ZERO, Ratio::ONE, Ratio::integer(2)];
    let cycle = [(0_usize, 1_usize), (1, 2), (2, 0)];
    let mut source_period = Ratio::ZERO;
    let mut divided_period = Ratio::ZERO;
    let mut source_increments = [Ratio::ZERO; 3];
    let mut divided_increments = [Ratio::ZERO; 3];
    for (index, &(from, to)) in cycle.iter().enumerate() {
        let delta_s = entropy[to] - entropy[from];
        source_increments[index] = state_price(entropy[from]) * delta_s;
        source_period = source_period + source_increments[index];
        let delta_a = state_potential(entropy[to]) - state_potential(entropy[from]);
        let divided_price = if delta_s.is_zero() {
            require(delta_a.is_zero(), "zero-entropy edge changes A(S)")?;
            Ratio::ZERO
        } else {
            delta_a / delta_s
        };
        divided_increments[index] = divided_price * delta_s;
        divided_period = divided_period + divided_increments[index];
    }
    require(
        source_increments == [Ratio::ONE, Ratio::integer(3), Ratio::integer(-10)],
        "source-price edge increments changed",
    )?;
    require(
        divided_increments == [Ratio::integer(2), Ratio::integer(4), Ratio::integer(-6)],
        "divided-difference edge increments changed",
    )?;
    require(
        source_period == Ratio::integer(-6),
        "source-price period changed",
    )?;
    require(
        divided_period.is_zero(),
        "divided-difference period is nonzero",
    )?;
    if claim_source_integrable {
        return Err(GateFailure(format!(
            "departure-state price has nonzero fundamental-cycle period {source_period}"
        )));
    }
    Ok((source_period, divided_period))
}

fn gate_10() -> Result<String, GateFailure> {
    variable_price_periods(false)?;
    Ok("source_increments=1,3,-10 period=-6 divided=2,4,-6 period=0".to_string())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HeatWorkLedger {
    heat_into_system: Ratio,
    work_on_system: Ratio,
}

impl HeatWorkLedger {
    fn delta_u(self) -> Ratio {
        self.heat_into_system + self.work_on_system
    }
}

fn heat_work_nonidentity(claim_identical: bool) -> Result<(), GateFailure> {
    let temperature = Ratio::ONE;
    let delta_s = Ratio::ONE;
    let delta_u = Ratio::ONE;
    let signed_tau = temperature * delta_s;
    let xi = signed_tau - delta_u;
    let heat = HeatWorkLedger {
        heat_into_system: Ratio::ONE,
        work_on_system: Ratio::ZERO,
    };
    let work = HeatWorkLedger {
        heat_into_system: Ratio::ZERO,
        work_on_system: Ratio::ONE,
    };
    require(
        heat.delta_u() == delta_u,
        "heat ledger violates the first law",
    )?;
    require(
        work.delta_u() == delta_u,
        "work ledger violates the first law",
    )?;
    require(
        signed_tau == Ratio::ONE && xi.is_zero(),
        "coarse fixture changed",
    )?;
    require(heat != work, "heat/work ledgers collapsed structurally")?;
    for ledger in [heat, work] {
        let sigma = delta_s - ledger.heat_into_system / temperature;
        let ledger_xi = temperature * sigma - ledger.work_on_system;
        require(
            ledger_xi == xi,
            "Xi=T*Sigma-W_on disagrees with a declared ledger",
        )?;
    }
    if claim_identical {
        return Err(GateFailure(
            "equal TAU and Xi do not identify distinct (Q,W_on) ledgers".to_string(),
        ));
    }
    Ok(())
}

fn gate_11() -> Result<String, GateFailure> {
    heat_work_nonidentity(false)?;
    Ok("coarse=(DeltaS=1,DeltaU=1,TAU=1,Xi=0) ledgers=(1,0)!=(0,1)".to_string())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProtocolVerdict {
    OperationalTemperature,
    NotAdmittedTemperature,
    GateStatistic,
    AccountingPriceInterface,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TemperatureEvidence {
    independent_energy_entropy_boundary: bool,
    reservoir_relation: bool,
    unique_ldb_match: bool,
    lawful_contact: bool,
}

fn temperature_admitted(evidence: TemperatureEvidence) -> bool {
    evidence.independent_energy_entropy_boundary
        && ((evidence.reservoir_relation && evidence.unique_ldb_match) || evidence.lawful_contact)
}

fn protocol_verdict(role: Role, evidence: TemperatureEvidence) -> ProtocolVerdict {
    if temperature_admitted(evidence) {
        return ProtocolVerdict::OperationalTemperature;
    }
    match role {
        Role::GateStatistic => ProtocolVerdict::GateStatistic,
        Role::AccountingPrice => ProtocolVerdict::AccountingPriceInterface,
        Role::Temperature | Role::KineticCompatibilityTemperature => {
            ProtocolVerdict::NotAdmittedTemperature
        }
        _ => ProtocolVerdict::NotAdmittedTemperature,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RelabeledGateValue {
    gate_units: Ratio,
    energy_per_nat_per_gate_unit: Ratio,
}

impl RelabeledGateValue {
    fn converted_energy_per_nat(self) -> Ratio {
        self.gate_units * self.energy_per_nat_per_gate_unit
    }
}

fn protocol_scope(relabel_gate: bool) -> Result<(), GateFailure> {
    let absent = TemperatureEvidence {
        independent_energy_entropy_boundary: false,
        reservoir_relation: false,
        unique_ldb_match: false,
        lawful_contact: false,
    };
    let gate = protocol_verdict(Role::GateStatistic, absent);
    let price = protocol_verdict(Role::AccountingPrice, absent);
    require(
        gate == ProtocolVerdict::GateStatistic,
        "a_P was promoted to temperature",
    )?;
    require(
        price == ProtocolVerdict::AccountingPriceInterface,
        "p_o was promoted to temperature",
    )?;
    if relabel_gate {
        let relabeled = RelabeledGateValue {
            gate_units: Ratio::new(1, 2),
            energy_per_nat_per_gate_unit: Ratio::ONE,
        };
        require(
            relabeled.converted_energy_per_nat() == Ratio::new(1, 2),
            "C18 numeric conversion fixture changed",
        )?;
        require(
            temperature_admitted(absent),
            "a_P=1/2 plus a unit conversion supplies no reservoir, LDB, or contact evidence",
        )?;
    }
    Ok(())
}

fn gate_12() -> Result<String, GateFailure> {
    protocol_scope(false)?;
    Ok("a_P=GATE_STATISTIC p_o=ACCOUNTING_PRICE_INTERFACE".to_string())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EosVerdict {
    ConditionalAccounting,
    EquationOfState,
}

fn receipt_value(price: Ratio, delta_s: Ratio) -> Ratio {
    assert!(price >= Ratio::ZERO, "receipt price must be nonnegative");
    price * positive_part(delta_s)
}

fn eos_scope(claim_receipt_is_eos: bool) -> Result<(), GateFailure> {
    let delta_s = Ratio::integer(2);
    let first_price = Ratio::ONE;
    let second_price = Ratio::integer(3);
    let first_receipt = receipt_value(first_price, delta_s);
    let second_receipt = receipt_value(second_price, delta_s);
    require(
        first_receipt == Ratio::integer(2) && second_receipt == Ratio::integer(6),
        "receipt nonselection fixture changed",
    )?;
    require(
        first_receipt != second_receipt,
        "distinct supplied prices did not produce distinct valid receipts",
    )?;
    require(
        first_receipt / delta_s == first_price && second_receipt / delta_s == second_price,
        "receipt rearrangement failed",
    )?;
    let receipt = EosVerdict::ConditionalAccounting;
    let selected_temperature = Temperature::from_base(Ratio::ONE, 3);
    let reservoir = if selected_temperature.energy_for_log(&ExactLog::of_positive_integer(3))
        == Some(Ratio::ONE)
    {
        EosVerdict::EquationOfState
    } else {
        EosVerdict::ConditionalAccounting
    };
    require(
        receipt == EosVerdict::ConditionalAccounting,
        "receipt expression was misclassified",
    )?;
    require(
        reservoir == EosVerdict::EquationOfState,
        "reservoir relation was misclassified",
    )?;
    if claim_receipt_is_eos {
        return Err(GateFailure(
            "the same DeltaS admits p=1 with TAU=2 and p=3 with TAU=6; p=TAU/DeltaS only returns the supplied choice"
                .to_string(),
        ));
    }
    Ok(())
}

fn gate_13() -> Result<String, GateFailure> {
    eos_scope(false)?;
    Ok("receipt=CONDITIONAL_ACCOUNTING reservoir=EQUATION_OF_STATE".to_string())
}

fn run_primary_gate(
    gate: GateId,
    reservoir_evidence: Option<&[ReservoirEvidence]>,
    micro_evidence: Option<&[CaseHazardEvidence]>,
    primary_identities: Option<&[CaseTemperature]>,
) -> Result<String, GateFailure> {
    match gate {
        GateId::G01 => gate_01(),
        GateId::G02 => gate_02(reservoir_evidence.ok_or_else(|| {
            GateFailure("G02 has no finite-difference reservoir evidence".to_string())
        })?),
        GateId::G03 => gate_03(micro_evidence.ok_or_else(|| {
            GateFailure("G03 has no enumerated micro-lumping evidence".to_string())
        })?),
        GateId::G04 => gate_04(
            primary_identities.ok_or_else(|| {
                GateFailure("G04 has no independently enumerated channel identities".to_string())
            })?,
            reservoir_evidence.ok_or_else(|| {
                GateFailure("G04 has no immutable passing G02 reservoir evidence".to_string())
            })?,
        ),
        GateId::G05 => gate_05(primary_identities.ok_or_else(|| {
            GateFailure("G05 has no immutable passing G04 identities".to_string())
        })?),
        GateId::G06 => gate_06(
            reservoir_evidence.ok_or_else(|| {
                GateFailure("G06 has no immutable passing G02 evidence".to_string())
            })?,
            micro_evidence.ok_or_else(|| {
                GateFailure("G06 has no immutable passing G03 evidence".to_string())
            })?,
        ),
        GateId::G07 => gate_07(),
        GateId::G08 => gate_08(),
        GateId::G09 => gate_09(),
        GateId::G10 => gate_10(),
        GateId::G11 => gate_11(),
        GateId::G12 => gate_12(),
        GateId::G13 => gate_13(),
    }
}

fn lane_owner(lane: Lane) -> Option<GateId> {
    match lane {
        Lane::TypedSources => None,
        Lane::Reservoir => Some(GateId::G02),
        Lane::Channel => Some(GateId::G04),
        Lane::CrossCase => Some(GateId::G05),
        Lane::Gauge => Some(GateId::G06),
        Lane::Component => Some(GateId::G07),
        Lane::GrossTau => Some(GateId::G08),
        Lane::SignedTau => Some(GateId::G09),
        Lane::VariablePrice => Some(GateId::G10),
        Lane::HeatWork => Some(GateId::G11),
        Lane::ProtocolPrice => Some(GateId::G12),
        Lane::EquationOfState => Some(GateId::G13),
    }
}

enum ControlSignal {
    Pass,
    Classification(String),
}

fn half_reservoir_control() -> Result<ControlSignal, GateFailure> {
    let multiplicities = [1_i128, 3, 9, 27];
    reservoir_slope_check(
        Ratio::ONE,
        &multiplicities,
        &Temperature::from_base(Ratio::new(1, 2), 3),
    )?;
    Ok(ControlSignal::Pass)
}

fn nonlinear_reservoir_control() -> Result<ControlSignal, GateFailure> {
    reservoir_slope_check(
        Ratio::ONE,
        &[1, 2, 8],
        &Temperature::from_base(Ratio::ONE, 2),
    )?;
    Ok(ControlSignal::Pass)
}

fn body_dependent_map_control(
    primary_identities: &[CaseTemperature],
) -> Result<ControlSignal, GateFailure> {
    let identities: Vec<_> = primary_identities
        .iter()
        .filter(|identity| identity.reservoir_base == 3 && identity.lambda == Ratio::ONE)
        .collect();
    require(identities.len() == 2, "C03 cross-case fixture changed")?;
    let mut proposed = Vec::new();
    for identity in identities {
        proposed.push(
            identity
                .derived
                .scaled_energy(Ratio::integer(identity.g_one)),
        );
    }
    require(
        proposed[0] == proposed[1],
        format!("T_hat(g)=g(1)T_R gives {} and {}", proposed[0], proposed[1]),
    )?;
    Ok(ControlSignal::Pass)
}

fn scale_energy_only_control(
    primary_reservoirs: &[ReservoirEvidence],
    primary_hazards: &[CaseHazardEvidence],
) -> Result<ControlSignal, GateFailure> {
    let case = confirmatory_cases()
        .into_iter()
        .next()
        .expect("confirmatory matrix is nonempty");
    let fixed_temperature = primary_reservoirs
        .iter()
        .find(|item| item.reservoir_base == case.reservoir_base && item.lambda == case.lambda)
        .ok_or_else(|| GateFailure("C04 has no immutable G02 temperature".to_string()))?
        .temperature
        .clone();
    let mut channels = case_channels(&case, primary_hazards)?;
    for channel in &mut channels {
        channel.delta_u = channel.delta_u * Ratio::integer(3);
    }
    match classify_ldb(&channels) {
        LdbClass::Unique(temperature) => require(
            temperature == fixed_temperature,
            format!(
                "scaled U requires T_LDB={} while fixed T_R={}",
                temperature, fixed_temperature
            ),
        )?,
        classification => {
            return Err(GateFailure(format!(
                "scaled-U channel lane classified as {classification}"
            )))
        }
    }
    Ok(ControlSignal::Pass)
}

fn incompatible_control(channels: Vec<LdbDatum>) -> Result<ControlSignal, GateFailure> {
    match classify_ldb(&channels) {
        LdbClass::Incompatible(reason) => Err(GateFailure(reason)),
        _ => Ok(ControlSignal::Pass),
    }
}

fn aggregation_trap_control() -> Result<ControlSignal, GateFailure> {
    let aggregate_rho = Ratio::new(2 + 1, 1 + 2);
    require(
        aggregate_rho == Ratio::ONE,
        "C11 aggregate fixture no longer hides the labelled mismatch",
    )?;
    incompatible_control(vec![
        LdbDatum {
            label: "parallel-a",
            delta_s: ExactLog::zero(),
            delta_u: Ratio::ZERO,
            rho: Ratio::integer(2),
        },
        LdbDatum {
            label: "parallel-b",
            delta_s: ExactLog::zero(),
            delta_u: Ratio::ZERO,
            rho: Ratio::new(1, 2),
        },
    ])
}

fn run_control_owner(
    control: ControlId,
    primary_reservoirs: &[ReservoirEvidence],
    primary_hazards: &[CaseHazardEvidence],
    primary_identities: &[CaseTemperature],
) -> Result<ControlSignal, GateFailure> {
    match control {
        ControlId::C01 => half_reservoir_control(),
        ControlId::C02 => Ok(ControlSignal::Pass),
        ControlId::C03 => body_dependent_map_control(primary_identities),
        ControlId::C04 => scale_energy_only_control(primary_reservoirs, primary_hazards),
        ControlId::C05 => Ok(ControlSignal::Classification(d_all().to_string())),
        ControlId::C06 => Ok(ControlSignal::Classification(d_singleton().to_string())),
        ControlId::C07 => Ok(ControlSignal::Classification(d_empty().to_string())),
        ControlId::C08 => nonlinear_reservoir_control(),
        ControlId::C09 => incompatible_control(vec![LdbDatum {
            label: "zero-energy-ratio",
            delta_s: ExactLog::zero(),
            delta_u: Ratio::ZERO,
            rho: Ratio::integer(2),
        }]),
        ControlId::C10 => incompatible_control(vec![
            LdbDatum {
                label: "slope-3",
                delta_s: ExactLog::zero(),
                delta_u: Ratio::ONE,
                rho: Ratio::new(1, 3),
            },
            LdbDatum {
                label: "slope-5",
                delta_s: ExactLog::zero(),
                delta_u: Ratio::ONE,
                rho: Ratio::new(1, 5),
            },
        ]),
        ControlId::C11 => aggregation_trap_control(),
        ControlId::C12 => {
            check_energy_gauge(primary_reservoirs, primary_hazards)?;
            Ok(ControlSignal::Pass)
        }
        ControlId::C13 => Ok(ControlSignal::Classification(
            per_component_scale_set().to_string(),
        )),
        ControlId::C14 => {
            gross_tau_cycle(true)?;
            Ok(ControlSignal::Pass)
        }
        ControlId::C15 => {
            signed_tau_cycle(true)?;
            Ok(ControlSignal::Pass)
        }
        ControlId::C16 => {
            variable_price_periods(true)?;
            Ok(ControlSignal::Pass)
        }
        ControlId::C17 => {
            heat_work_nonidentity(true)?;
            Ok(ControlSignal::Pass)
        }
        ControlId::C18 => {
            protocol_scope(true)?;
            Ok(ControlSignal::Pass)
        }
        ControlId::C19 => {
            eos_scope(true)?;
            Ok(ControlSignal::Pass)
        }
    }
}

fn expected_control(control: ControlId) -> &'static str {
    match control {
        ControlId::C01 => "FAIL@G02",
        ControlId::C02 => "FAIL@G01",
        ControlId::C03 => "FAIL@G05",
        ControlId::C04 => "FAIL@G04",
        ControlId::C05 => "CLASS@G07:ALL_POSITIVE",
        ControlId::C06 => "CLASS@G07:{1/ln(3)}",
        ControlId::C07 => "CLASS@G07:EMPTY",
        ControlId::C08 => "FAIL@G02",
        ControlId::C09 => "FAIL@G04",
        ControlId::C10 => "FAIL@G04",
        ControlId::C11 => "FAIL@G04",
        ControlId::C12 => "PASS@G06",
        ControlId::C13 => "CLASS@G07:EMPTY",
        ControlId::C14 => "FAIL@G08",
        ControlId::C15 => "FAIL@G09",
        ControlId::C16 => "FAIL@G10",
        ControlId::C17 => "FAIL@G11",
        ControlId::C18 => "FAIL@G12",
        ControlId::C19 => "FAIL@G13",
    }
}

fn evaluate_control(
    control: ControlId,
    primary_reservoirs: &[ReservoirEvidence],
    primary_hazards: &[CaseHazardEvidence],
    primary_identities: &[CaseTemperature],
) -> ControlOutcome {
    let lane = control.lane();
    let expected = expected_control(control);
    let mut gate_statuses = [ControlGateStatus::NotApplicable; 13];
    let typed = check_typed_sources(control == ControlId::C02);
    let typed_passed = typed.is_ok();
    gate_statuses[GateId::G01.index()] = if typed_passed {
        ControlGateStatus::Pass
    } else {
        ControlGateStatus::Fail
    };
    let observed = if !typed_passed {
        "FAIL@G01".to_string()
    } else if let Some(owner) = lane_owner(lane) {
        match run_control_owner(
            control,
            primary_reservoirs,
            primary_hazards,
            primary_identities,
        ) {
            Ok(ControlSignal::Pass) => {
                gate_statuses[owner.index()] = ControlGateStatus::Pass;
                format!("PASS@{}", owner.short())
            }
            Ok(ControlSignal::Classification(classification)) => {
                gate_statuses[owner.index()] = ControlGateStatus::Pass;
                format!("CLASS@{}:{classification}", owner.short())
            }
            Err(_) => {
                gate_statuses[owner.index()] = ControlGateStatus::Fail;
                format!("FAIL@{}", owner.short())
            }
        }
    } else {
        "PASS@G01".to_string()
    };
    ControlOutcome {
        control,
        lane,
        gate_statuses,
        expected,
        passed: observed == expected,
        observed,
    }
}

pub fn evaluate() -> ProofReport {
    let mut gates = Vec::with_capacity(GateId::ALL.len());
    let mut reservoir_evidence = None;
    let mut micro_evidence = None;
    let mut primary_identities = None;
    let mut rate_verdict = RateDerivedVerdict::Incompatible;
    for gate in GateId::ALL {
        let result = if gate == GateId::G02 {
            match derive_primary_reservoir_evidence() {
                Ok(evidence) => {
                    let result = gate_02(&evidence);
                    if result.is_ok() {
                        reservoir_evidence = Some(evidence);
                    }
                    result
                }
                Err(error) => Err(error),
            }
        } else if gate == GateId::G03 {
            match derive_micro_lumping_evidence() {
                Ok(evidence) => {
                    let result = gate_03(&evidence);
                    if result.is_ok() {
                        micro_evidence = Some(evidence);
                    }
                    result
                }
                Err(error) => Err(error),
            }
        } else if gate == GateId::G04 {
            match derive_primary_case_temperatures(micro_evidence.as_deref().unwrap_or(&[])) {
                Ok(identities) => {
                    rate_verdict = RateDerivedVerdict::Unique;
                    let result = gate_04(&identities, reservoir_evidence.as_deref().unwrap_or(&[]));
                    if result.is_ok() {
                        primary_identities = Some(identities);
                    }
                    result
                }
                Err(error) => {
                    rate_verdict = error.verdict;
                    Err(error.failure)
                }
            }
        } else {
            run_primary_gate(
                gate,
                reservoir_evidence.as_deref(),
                micro_evidence.as_deref(),
                primary_identities.as_deref(),
            )
        };
        gates.push(match result {
            Ok(detail) => GateOutcome {
                gate,
                passed: true,
                detail,
            },
            Err(GateFailure(detail)) => GateOutcome {
                gate,
                passed: false,
                detail,
            },
        });
    }
    let identities = primary_identities.as_deref().unwrap_or(&[]);
    let reservoirs = reservoir_evidence.as_deref().unwrap_or(&[]);
    let hazards = micro_evidence.as_deref().unwrap_or(&[]);
    let controls = ControlId::ALL
        .into_iter()
        .map(|control| evaluate_control(control, reservoirs, hazards, identities))
        .collect();
    ProofReport {
        gates,
        controls,
        rate_verdict,
    }
}
