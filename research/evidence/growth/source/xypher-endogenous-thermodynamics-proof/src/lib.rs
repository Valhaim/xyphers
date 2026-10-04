pub mod adaptive;
pub mod exact;
pub mod model;
pub mod sha256;
pub mod thermal;

use std::collections::BTreeMap;
use std::io::{self, Write};

use adaptive::{
    adaptive_loop_report, blank_live_bucket_clamp_report, constant_r_nonadaptive_report,
    construction_frontier_information_reports, creation_rank_report,
    exact_outcome_copy_information, ideal_counter_downgrade_report, initial_prefix_report,
    legacy_ap_no_go_report, maintenance_report, memory_causality_report, off_shell_report,
    prefix_count_report, prefix_fixture, resource_report, OutcomeCopyInformation, ALPHABET_BASES,
};
use exact::Ratio;
use model::{
    admit_builder_request, attempt_bit_capacity_certificate, descriptor_order_audit,
    driven_protocol_audit, instantiate_alias_before_fresh_support_trace,
    instantiate_one_support_trace, memory_intervention_audit, physical_branch_record_audit,
    primary_fixture, primary_fixture_with_quantum, BuilderAdmissionError, BuilderRequest,
    CompleteState, DescriptorOrderAudit, DrivenChannel, EnabledEvent, HeldOutTrafficRequest,
    KernelSupport, ModelEvent, Phase, PoisonFields, ProtocolTrace, TargetCountRequest,
};
use thermal::{
    classify_prefix_reservoir, verify_external_thermostat_control,
    verify_frozen_contact_activity_control, verify_frozen_contact_fixture,
    verify_gibbs_installed_control, verify_held_out_probe, verify_port_law_mutation_control,
    verify_probe_controls, verify_static_perfect_tree_control, ExactLog, FrozenContactFixture,
    PrefixReservoir, ProbeThermalResult, ReservoirAlternative, StateTemperatureClass,
};

pub const BOUNDARY_COMMIT: &str = "7ee278ed14ade12689540481903e4832f53442d7";
pub const BOUNDARY_SHA256: &str =
    "a3aafdc4d29dc3b6c8a2c5bf8deb554745a311b2f3485e1577f3697b1aa14c80";

pub const SOURCE_MANIFEST: &str = include_str!("../FROZEN-SOURCE.sha256");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GateId {
    E01,
    E02,
    E03,
    E04,
    E05,
    E06,
    E07,
    E08,
    E09,
    E10,
    E11,
    E12,
    E13,
    E14,
    E15,
}

impl GateId {
    pub const ALL: [Self; 15] = [
        Self::E01,
        Self::E02,
        Self::E03,
        Self::E04,
        Self::E05,
        Self::E06,
        Self::E07,
        Self::E08,
        Self::E09,
        Self::E10,
        Self::E11,
        Self::E12,
        Self::E13,
        Self::E14,
        Self::E15,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::E01 => "E01_COMPLETE_STATE",
            Self::E02 => "E02_INITIALLY_NONTHERMAL",
            Self::E03 => "E03_FORBIDDEN_INPUT_NON_USE",
            Self::E04 => "E04_XYPHER_CAUSAL_LOOP",
            Self::E05 => "E05_MEMORY_CAUSAL",
            Self::E06 => "E06_CREATION",
            Self::E07 => "E07_ACTUAL_MULTIPLICITY",
            Self::E08 => "E08_UNIQUE_STATE_TEMPERATURE",
            Self::E09 => "E09_INDEPENDENT_THERMAL_SECTOR",
            Self::E10 => "E10_TEMPERATURE_EQUALITY",
            Self::E11 => "E11_PHYSICAL_LEDGERS",
            Self::E12 => "E12_MAINTENANCE",
            Self::E13 => "E13_CONTACT",
            Self::E14 => "E14_CONTROLS",
            Self::E15 => "E15_NO_HIDDEN_FIT",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlId {
    StaticPerfectTree,
    NonadaptiveBuilder,
    LegacyAPGate,
    ExternalThermostat,
    GibbsInstalled,
    TargetCountBuilder,
    TrafficTrainedBuilder,
    PoisonInput,
    OffShellInventory,
    DescriptorPermutation,
    EnergyShuffle,
    LeafDeletion,
    RepairDisabled,
    DescriptorWithheld,
    PortLawMutation,
    MemoryClamp,
    MemoryPermutation,
    HiddenMemory,
    IdealCounters,
    LedgerLeak,
    ActivityOnly,
    ReverseMismatch,
}

impl ControlId {
    pub const ALL: [Self; 22] = [
        Self::StaticPerfectTree,
        Self::NonadaptiveBuilder,
        Self::LegacyAPGate,
        Self::ExternalThermostat,
        Self::GibbsInstalled,
        Self::TargetCountBuilder,
        Self::TrafficTrainedBuilder,
        Self::PoisonInput,
        Self::OffShellInventory,
        Self::DescriptorPermutation,
        Self::EnergyShuffle,
        Self::LeafDeletion,
        Self::RepairDisabled,
        Self::DescriptorWithheld,
        Self::PortLawMutation,
        Self::MemoryClamp,
        Self::MemoryPermutation,
        Self::HiddenMemory,
        Self::IdealCounters,
        Self::LedgerLeak,
        Self::ActivityOnly,
        Self::ReverseMismatch,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::StaticPerfectTree => "C01_STATIC_PERFECT_TREE",
            Self::NonadaptiveBuilder => "C02_NONADAPTIVE_BUILDER",
            Self::LegacyAPGate => "C03_LEGACY_A_P_GATE",
            Self::ExternalThermostat => "C04_EXTERNAL_THERMOSTAT",
            Self::GibbsInstalled => "C05_GIBBS_INSTALLED",
            Self::TargetCountBuilder => "C06_TARGET_COUNT_BUILDER",
            Self::TrafficTrainedBuilder => "C07_TRAFFIC_TRAINED_BUILDER",
            Self::PoisonInput => "C08_POISON_INPUT",
            Self::OffShellInventory => "C09_OFF_SHELL_INVENTORY",
            Self::DescriptorPermutation => "C10_DESCRIPTOR_PERMUTATION",
            Self::EnergyShuffle => "C11_ENERGY_SHUFFLE",
            Self::LeafDeletion => "C12_LEAF_DELETION",
            Self::RepairDisabled => "C13_REPAIR_DISABLED",
            Self::DescriptorWithheld => "C14_DESCRIPTOR_WITHHELD",
            Self::PortLawMutation => "C15_PORT_LAW_MUTATION",
            Self::MemoryClamp => "C16_MEMORY_CLAMP",
            Self::MemoryPermutation => "C17_MEMORY_PERMUTATION",
            Self::HiddenMemory => "C18_HIDDEN_MEMORY",
            Self::IdealCounters => "C19_IDEAL_COUNTERS",
            Self::LedgerLeak => "C20_LEDGER_LEAK",
            Self::ActivityOnly => "C21_ACTIVITY_ONLY",
            Self::ReverseMismatch => "C22_REVERSE_MISMATCH",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GateOutcome {
    pub gate: GateId,
    pub passed: bool,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlOutcome {
    pub control: ControlId,
    pub passed: bool,
    pub expected: &'static str,
    pub observed: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpecimenSummary {
    pub base: u8,
    pub initial_classification: String,
    pub operations: usize,
    pub shell_counts: Vec<u128>,
    pub state_classification: String,
    pub traffic_classification: String,
    pub state_temperature: String,
    pub ldb_temperature: String,
    pub probe_fibers: [u128; 3],
    pub maintenance_leaf_arms: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InformationReport {
    pub scope: String,
    pub record_variable: String,
    pub branch_evidence: String,
    pub outcome_probabilities: [Ratio; 2],
    pub record_probabilities: [Ratio; 2],
    pub joint_distribution: [[Ratio; 2]; 2],
    pub outcome_entropy: ExactLog,
    pub record_outcome_mutual_information: ExactLog,
    pub record_is_exact_copy: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FinalClassification {
    StaticStructuralTheorem,
    OperationalThermodynamicGraph,
    EndogenousNonadaptiveConstructor,
    AdaptiveThermodynamicConstruction,
    EndogenousStateDependentTemperature,
    EndogenouslyThermodynamicAdaptiveXypher,
    NotAdmissible,
    NoResult,
}

impl FinalClassification {
    const fn label(self) -> &'static str {
        match self {
            Self::StaticStructuralTheorem => "STATIC STRUCTURAL THEOREM",
            Self::OperationalThermodynamicGraph => "OPERATIONAL THERMODYNAMIC GRAPH",
            Self::EndogenousNonadaptiveConstructor => "ENDOGENOUS NONADAPTIVE CONSTRUCTOR",
            Self::AdaptiveThermodynamicConstruction => "ADAPTIVE THERMODYNAMIC CONSTRUCTION",
            Self::EndogenousStateDependentTemperature => "ENDOGENOUS STATE-DEPENDENT TEMPERATURE",
            Self::EndogenouslyThermodynamicAdaptiveXypher => {
                "ENDOGENOUSLY THERMODYNAMIC ADAPTIVE XYPHER"
            }
            Self::NotAdmissible => "NOT ADMISSIBLE",
            Self::NoResult => "NO_RESULT",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofReport {
    pub gates: Vec<GateOutcome>,
    pub controls: Vec<ControlOutcome>,
    pub specimens: Vec<SpecimenSummary>,
    pub information: Vec<InformationReport>,
    pub protocols: Vec<ProtocolTranscript>,
    pub observations: Vec<String>,
    pub thermal_observations: Vec<String>,
    pub contact_observations: Vec<String>,
    pub classification: FinalClassification,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolTranscript {
    pub scope: String,
    pub initial_complete_state: String,
    pub terminal_complete_state: String,
    pub forward: Vec<String>,
    pub inverse: Vec<String>,
    pub round_trip_exact: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct EvidenceJournal {
    controls: Vec<ControlOutcome>,
    specimens: Vec<SpecimenSummary>,
    information: Vec<InformationReport>,
    protocols: Vec<ProtocolTranscript>,
    observations: Vec<String>,
    thermal_observations: Vec<String>,
    contact_observations: Vec<String>,
}

impl ProofReport {
    pub fn is_success(&self) -> bool {
        self.gates.len() == GateId::ALL.len()
            && self
                .gates
                .iter()
                .zip(GateId::ALL)
                .all(|(outcome, gate)| outcome.gate == gate && outcome.passed)
            && self.controls.len() == ControlId::ALL.len()
            && self
                .controls
                .iter()
                .zip(ControlId::ALL)
                .all(|(outcome, control)| outcome.control == control && outcome.passed)
            && self.classification == FinalClassification::EndogenouslyThermodynamicAdaptiveXypher
    }

    pub fn write_to<W: Write>(&self, output: &mut W) -> io::Result<()> {
        writeln!(output, "CAL_ENDO_1_EXACT_VERIFIER v1")?;
        writeln!(
            output,
            "BOUNDARY commit={BOUNDARY_COMMIT} sha256={BOUNDARY_SHA256}"
        )?;
        for observation in &self.observations {
            writeln!(output, "OBSERVATION {observation}")?;
        }
        for specimen in &self.specimens {
            writeln!(
                output,
                "SPECIMEN b={} initial={} operations={} counts={} state={} traffic={} T_state={} T_LDB={} fibers={},{},{} repair_arms={}",
                specimen.base,
                specimen.initial_classification,
                specimen.operations,
                join_counts(&specimen.shell_counts),
                specimen.state_classification,
                specimen.traffic_classification,
                specimen.state_temperature,
                specimen.ldb_temperature,
                specimen.probe_fibers[0],
                specimen.probe_fibers[1],
                specimen.probe_fibers[2],
                specimen.maintenance_leaf_arms,
            )?;
        }
        for outcome in &self.gates {
            writeln!(
                output,
                "{} {} {}",
                outcome.gate.code(),
                if outcome.passed { "PASS" } else { "FAIL" },
                outcome.detail
            )?;
        }
        for outcome in &self.controls {
            writeln!(
                output,
                "{} {} expected={} observed={}",
                outcome.control.code(),
                if outcome.passed { "PASS" } else { "FAIL" },
                outcome.expected,
                outcome.observed
            )?;
        }
        for information in &self.information {
            writeln!(
                output,
                "INFORMATION scope={} record_variable={} p={},{} record_p={},{} joint={},{},{},{} H={} I={} exact_copy={} evidence={}",
                information.scope,
                information.record_variable,
                information.outcome_probabilities[0],
                information.outcome_probabilities[1],
                information.record_probabilities[0],
                information.record_probabilities[1],
                information.joint_distribution[0][0],
                information.joint_distribution[0][1],
                information.joint_distribution[1][0],
                information.joint_distribution[1][1],
                information.outcome_entropy,
                information.record_outcome_mutual_information,
                information.record_is_exact_copy,
                information.branch_evidence,
            )?;
        }
        for protocol in &self.protocols {
            writeln!(
                output,
                "PROTOCOL scope={} round_trip_exact={}",
                protocol.scope, protocol.round_trip_exact
            )?;
            writeln!(
                output,
                "INITIAL_COMPLETE_STATE {}",
                protocol.initial_complete_state
            )?;
            for step in &protocol.forward {
                writeln!(output, "FORWARD {step}")?;
            }
            writeln!(
                output,
                "TERMINAL_COMPLETE_STATE {}",
                protocol.terminal_complete_state
            )?;
            for step in &protocol.inverse {
                writeln!(output, "INVERSE {step}")?;
            }
        }
        for observation in &self.thermal_observations {
            writeln!(output, "THERMAL {observation}")?;
        }
        for observation in &self.contact_observations {
            writeln!(output, "CONTACT {observation}")?;
        }
        writeln!(output, "TAU=NOT_INSTANTIATED Xi=NOT_INSTANTIATED")?;
        writeln!(output, "CLASSIFICATION {}", self.classification.label())?;
        writeln!(
            output,
            "OVERALL {}",
            if self.is_success() { "PASS" } else { "FAIL" }
        )?;
        Ok(())
    }

    pub fn render(&self) -> String {
        let mut output = Vec::new();
        self.write_to(&mut output)
            .expect("writing a report to memory cannot fail");
        String::from_utf8(output).expect("the report renderer emits UTF-8")
    }

    fn fatal(detail: String, evidence: EvidenceJournal) -> Self {
        let controls = ControlId::ALL
            .into_iter()
            .map(|control| {
                evidence
                    .controls
                    .iter()
                    .find(|outcome| outcome.control == control)
                    .cloned()
                    .unwrap_or_else(|| ControlOutcome {
                        control,
                        passed: false,
                        expected: "FROZEN_CLASSIFICATION",
                        observed: format!("NO_RESULT:{detail}"),
                    })
            })
            .collect();
        Self {
            gates: GateId::ALL
                .into_iter()
                .map(|gate| GateOutcome {
                    gate,
                    passed: false,
                    detail: format!("NO_RESULT:{detail}"),
                })
                .collect(),
            controls,
            specimens: evidence.specimens,
            information: evidence.information,
            protocols: evidence.protocols,
            observations: evidence.observations,
            thermal_observations: evidence.thermal_observations,
            contact_observations: evidence.contact_observations,
            classification: FinalClassification::NoResult,
        }
    }
}

fn join_counts(counts: &[u128]) -> String {
    counts
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MaintenanceModelReport {
    leaf_arms: usize,
    every_damaged_count_matches: bool,
    every_damaged_temperature_rejected: bool,
    every_repair_restores: bool,
    every_repair_inverse_and_ledger_closes: bool,
    every_repair_event_surface_replays: bool,
    every_repair_uses_existing_reserve: bool,
    repair_disabled_stays_damaged: bool,
    repair_disabled_rejects_primary_event: bool,
    descriptor_withheld_blocks_repair: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProbeKernelEvidence {
    protocol: thermal::SealedProbeProtocol,
    complete_coordinate_count: usize,
    event_row_count: usize,
    passes: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SpecimenEvidence {
    base: u8,
    initial: thermal::PrefixThermalResult,
    trace: ProtocolTrace,
    state: thermal::PrefixThermalResult,
    probe: ProbeThermalResult,
    probe_kernel: ProbeKernelEvidence,
    maintenance: MaintenanceModelReport,
}

fn specimen_summary(specimen: &SpecimenEvidence) -> SpecimenSummary {
    SpecimenSummary {
        base: specimen.base,
        initial_classification: state_classification_label(specimen.initial.classification)
            .to_string(),
        operations: specimen.trace.receipts.len(),
        shell_counts: specimen.state.shell_counts.clone(),
        state_classification: state_classification_label(specimen.state.classification).to_string(),
        traffic_classification: traffic_classification_label(&specimen.probe).to_string(),
        state_temperature: specimen
            .state
            .temperature
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "UNDEFINED".to_string()),
        ldb_temperature: specimen
            .probe
            .ldb_temperature
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "UNDEFINED".to_string()),
        probe_fibers: specimen.probe.fiber_sizes,
        maintenance_leaf_arms: specimen.maintenance.leaf_arms,
    }
}

fn retain_protocol(evidence: &mut EvidenceJournal, transcript: ProtocolTranscript) {
    evidence.protocols.push(transcript);
}

fn ledger_text(ledger: &model::LedgerRow) -> String {
    format!(
        "channel={:?} d_working={} d_store={} W={} Q={} d_augmented={} W_ext={} Q_ext={} d_external_store={} d_complete={} W_complete={} Q_complete={} resources=[free_descriptors:{} blank_semantic_records:{} attempted_candidates:{} charged_build_cells:{} charged_repair_cells:{} charged_observation_cells:{} charged_external_perturbation_cells:{} written_operation_records:{}]",
        ledger.channel,
        ledger.delta_working,
        ledger.delta_work_store,
        ledger.work_on_working,
        ledger.heat_to_working,
        ledger.delta_augmented,
        ledger.external_work,
        ledger.external_heat,
        ledger.delta_external_work_store,
        ledger.delta_complete,
        ledger.complete_external_work,
        ledger.complete_external_heat,
        ledger.resources.free_descriptors,
        ledger.resources.blank_semantic_records,
        ledger.resources.attempted_candidates,
        ledger.resources.charged_build_cells,
        ledger.resources.charged_repair_cells,
        ledger.resources.charged_observation_cells,
        ledger.resources.charged_external_perturbation_cells,
        ledger.resources.written_operation_records,
    )
}

fn inverse_protocol_name(channel: DrivenChannel) -> &'static str {
    match channel {
        DrivenChannel::Fresh => "UNFRESH",
        DrivenChannel::AliasQuarantine => "UNFAIL",
        DrivenChannel::Credit => "UNCREDIT",
        DrivenChannel::Promote => "DEMOTE",
        DrivenChannel::Perturb => "UNPERTURB",
        DrivenChannel::Absorb => "UNABSORB",
    }
}

fn transition_replays_from_complete_state(
    source: &CompleteState,
    enabled: &EnabledEvent,
) -> Result<bool, String> {
    let transition = source
        .event_transition(enabled, KernelSupport::PRIMARY)
        .map_err(|error| format!("event transition {:?}: {error:?}", enabled.event()))?;
    let working_difference =
        transition.successor.working_energy() as i128 - source.working_energy() as i128;
    let augmented_difference = transition.successor.augmented_internal_energy() as i128
        - source.augmented_internal_energy() as i128;
    let complete_difference = transition
        .successor
        .augmented_energy_with_external_perturbation_cell() as i128
        - source.augmented_energy_with_external_perturbation_cell() as i128;
    let resource_difference = transition
        .successor
        .resource_snapshot()
        .difference_from(&source.resource_snapshot());
    let authoritative_source = thermal::AuthoritativeState::Driven(source.clone());
    let authoritative_event = thermal::AuthoritativeEvent::Driven(enabled.clone());
    let authoritative =
        thermal::authoritative_transition(&authoritative_source, &authoritative_event).ok_or_else(
            || {
                format!(
                    "authoritative driven transition {:?} absent",
                    enabled.event()
                )
            },
        )?;
    let authoritative_replays = authoritative.source == authoritative_source
        && authoritative.event == authoritative_event
        && authoritative.inverse_restores_source
        && matches!(
            (&authoritative.successor, &authoritative.ledger, &authoritative.inverse),
            (
                thermal::AuthoritativeState::Driven(successor),
                thermal::AuthoritativeLedger::Driven(ledger),
                thermal::AuthoritativeInverse::Driven(inverse),
            ) if successor == &transition.successor
                && ledger == &transition.ledger
                && inverse == &transition.inverse
        );
    Ok(authoritative_replays
        && transition.enabled == *enabled
        && transition.hazard == enabled.hazard()
        && transition.channel == enabled.channel()
        && transition.forward_receipt.event == enabled.event()
        && transition.forward_receipt.hazard == enabled.hazard()
        && transition.forward_receipt.ledger == transition.ledger
        && transition.inverse_restores_source
        && transition.ledger.working_first_law_closes()
        && transition.ledger.internal_augmented_first_law_closes()
        && transition.ledger.complete_first_law_closes()
        && transition.inverse.ledger.working_first_law_closes()
        && transition
            .inverse
            .ledger
            .internal_augmented_first_law_closes()
        && transition.inverse.ledger.complete_first_law_closes()
        && working_difference == transition.ledger.delta_working
        && augmented_difference == transition.ledger.delta_augmented
        && complete_difference == transition.ledger.delta_complete
        && resource_difference == transition.ledger.resources)
}

fn protocol_event_surface_replays(trace: &ProtocolTrace) -> Result<bool, String> {
    let mut state = trace.initial_state.clone();
    let mut all_events_replay = true;
    for step in &trace.forward_steps {
        let enabled = state.enabled_events();
        all_events_replay &= state.attempt_bit_capacity_nontruncating_now()
            && state.P == step.phase_before
            && enabled == step.enabled_before
            && step.enabled_before.contains(&step.selected)
            && step.selected.hazard() == step.receipt.hazard
            && step.selected.event() == step.receipt.event;
        for event in &enabled {
            all_events_replay &= transition_replays_from_complete_state(&state, event)?;
        }
        let selected = state
            .event_transition(&step.selected, KernelSupport::PRIMARY)
            .map_err(|error| format!("selected event transition: {error:?}"))?;
        all_events_replay &= selected.forward_receipt == step.receipt
            && selected.successor.P == step.phase_after
            && selected
                .successor
                .Gamma
                .reservoir
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                == step.reservoir_words_after
            && selected
                .successor
                .Gamma
                .workspace
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                == step.workspace_words_after
            && selected.successor.F.len() == step.free_descriptor_count_after;
        state = selected.successor;
    }
    Ok(all_events_replay
        && state == trace.terminal_state
        && state.attempt_bit_capacity_nontruncating_now()
        && trace.receipts.len() == trace.forward_steps.len()
        && trace
            .receipts
            .iter()
            .eq(trace.forward_steps.iter().map(|step| &step.receipt))
        && trace.reverse_lifo_receipts.len() == trace.forward_steps.len())
}

fn protocol_transcript(
    scope: impl Into<String>,
    trace: &ProtocolTrace,
) -> Result<ProtocolTranscript, String> {
    let mut forward = Vec::new();
    let mut forward_state = trace.initial_state.clone();
    for (index, step) in trace.forward_steps.iter().enumerate() {
        let source = forward_state.clone();
        let transition = source
            .event_transition(&step.selected, KernelSupport::PRIMARY)
            .map_err(|error| format!("transcript forward step {index}: {error:?}"))?;
        forward.push(format!(
            "step={} phase={:?}->{:?} enabled={:?} selected={:?} hazard={} operation_record={} {} source_complete_state={:?} successor_complete_state={:?}",
            index,
            step.phase_before,
            step.phase_after,
            step.enabled_before,
            step.selected.event(),
            step.selected.hazard(),
            step.receipt.operation_record,
            ledger_text(&step.receipt.ledger),
            source,
            transition.successor,
        ));
        forward_state = transition.successor;
    }
    let forward_reaches_reported_terminal = forward_state == trace.terminal_state;

    let mut inverse = Vec::new();
    let mut inverse_state = trace.terminal_state.clone();
    let mut inverse_receipts_match = true;
    for (index, expected_receipt) in trace.reverse_lifo_receipts.iter().enumerate() {
        let source = inverse_state.clone();
        let actual_receipt = inverse_state
            .reverse_last()
            .map_err(|error| format!("transcript inverse step {index}: {error:?}"))?;
        inverse_receipts_match &= actual_receipt == *expected_receipt;
        inverse.push(format!(
            "lifo_step={} inverse={} forward_event={:?} hazard={} operation_record={} receipt_matches_trace={} {} source_complete_state={:?} successor_complete_state={:?}",
            index,
            inverse_protocol_name(actual_receipt.ledger.channel),
            actual_receipt.event,
            actual_receipt.hazard,
            actual_receipt.operation_record,
            actual_receipt == *expected_receipt,
            ledger_text(&actual_receipt.ledger),
            source,
            inverse_state,
        ));
    }
    let inverse_restores_reported_initial = inverse_state == trace.initial_state;
    let event_surface_replays = protocol_event_surface_replays(trace)?;
    Ok(ProtocolTranscript {
        scope: scope.into(),
        initial_complete_state: format!("{:?}", trace.initial_state),
        terminal_complete_state: format!("{:?}", trace.terminal_state),
        forward,
        inverse,
        round_trip_exact: forward_reaches_reported_terminal
            && inverse_receipts_match
            && inverse_restores_reported_initial
            && event_surface_replays
            && trace.every_forward_map_has_exact_local_inverse
            && trace.full_reverse_lifo_restores_initial
            && trace.every_working_ledger_closes
            && trace.every_augmented_ledger_closes
            && trace.every_complete_ledger_closes
            && trace.every_forward_energy_difference_matches_ledger
            && trace.every_forward_resource_difference_matches_ledger
            && trace.every_state_has_nontruncating_attempt_bit_capacity,
    })
}

fn single_transition_transcript(
    scope: impl Into<String>,
    source: &CompleteState,
    enabled: &EnabledEvent,
) -> Result<(ProtocolTranscript, CompleteState), String> {
    let transition = source
        .event_transition(enabled, KernelSupport::PRIMARY)
        .map_err(|error| format!("single transition {:?}: {error:?}", enabled.event()))?;
    let round_trip_exact = transition_replays_from_complete_state(source, enabled)?;
    let mut inverse_successor = transition.successor.clone();
    let actual_inverse = inverse_successor
        .reverse_last()
        .map_err(|error| format!("single transition inverse {:?}: {error:?}", enabled.event()))?;
    let forward = vec![format!(
        "phase={:?}->{:?} event={:?} hazard={} operation_record={} {} source_complete_state={:?} successor_complete_state={:?}",
        source.P,
        transition.successor.P,
        transition.enabled.event(),
        transition.hazard,
        transition.forward_receipt.operation_record,
        ledger_text(&transition.ledger),
        source,
        transition.successor,
    )];
    let inverse = vec![format!(
        "inverse={} forward_event={:?} hazard={} operation_record={} receipt_matches_transition={} {} source_complete_state={:?} successor_complete_state={:?}",
        inverse_protocol_name(actual_inverse.ledger.channel),
        actual_inverse.event,
        actual_inverse.hazard,
        actual_inverse.operation_record,
        actual_inverse == transition.inverse,
        ledger_text(&actual_inverse.ledger),
        transition.successor,
        inverse_successor,
    )];
    Ok((
        ProtocolTranscript {
            scope: scope.into(),
            initial_complete_state: format!("{source:?}"),
            terminal_complete_state: format!("{:?}", transition.successor),
            forward,
            inverse,
            round_trip_exact: round_trip_exact
                && actual_inverse == transition.inverse
                && inverse_successor == *source,
        },
        transition.successor,
    ))
}

fn verify_model_maintenance(
    completed: &CompleteState,
    base: u8,
    evidence: &mut EvidenceJournal,
) -> Result<MaintenanceModelReport, String> {
    let leaves = completed.Gamma.shell_words(3);
    let expected_complete = usize::from(base).pow(3);
    let completed_state =
        classify_prefix_reservoir(&PrefixReservoir::from_complete_state(completed));
    let mut every_damaged_count_matches = true;
    let mut every_damaged_temperature_rejected = true;
    let mut every_repair_restores = true;
    let mut every_repair_inverse_and_ledger_closes = true;
    let mut every_repair_event_surface_replays = true;
    let mut every_repair_uses_existing_reserve = true;
    let mut repair_disabled_stays_damaged = true;
    let mut repair_disabled_rejects_primary_event = true;
    let mut descriptor_withheld_blocks_repair = true;
    for leaf in &leaves {
        let descriptor = completed
            .Gamma
            .reservoir
            .get(leaf)
            .ok_or_else(|| format!("b{base} missing selected maintenance leaf"))?
            .descriptor;
        let mut damaged = completed.clone();
        let repair_cells_before = damaged.B_W.B_repair.charged_count();
        damaged
            .schedule_leaf_perturbation(leaf.clone())
            .map_err(|error| format!("b{base} schedule perturb {leaf:?}: {error:?}"))?;
        let perturb = damaged
            .enabled_events()
            .into_iter()
            .find(|event| {
                matches!(event, EnabledEvent::Perturb { leaf: scheduled, .. } if scheduled == leaf)
            })
            .ok_or_else(|| format!("b{base} scheduled perturb {leaf:?} was not enabled"))?;
        let (perturb_protocol, perturbed_state) = single_transition_transcript(
            format!("perturb-b{base}-leaf{:?}", leaf.0),
            &damaged,
            &perturb,
        )?;
        evidence.protocols.push(perturb_protocol);
        damaged = perturbed_state;
        let damaged_state =
            classify_prefix_reservoir(&PrefixReservoir::from_complete_state(&damaged));
        every_damaged_count_matches &= damaged.Gamma.shell_count(3) + 1 == expected_complete;
        every_damaged_temperature_rejected &=
            damaged_state.classification != StateTemperatureClass::UniqueConstant;
        evidence.observations.push(format!(
            "stage=MAINTENANCE_DAMAGED base={} leaf={:?} classification={:?} thermal_result={:?} complete_state={:?}",
            base, leaf, damaged_state.classification, damaged_state, damaged,
        ));
        let primary_repair_event = damaged.enabled_events().first().cloned();
        let mut repair_disabled_state = damaged.clone();
        let disabled_rejects_event = primary_repair_event.as_ref().is_some_and(|event| {
            matches!(
                repair_disabled_state
                    .apply_event_with_support(&event.event(), KernelSupport::REPAIR_DISABLED),
                Err(model::ActionError::EventNotEnabled)
            )
        });
        repair_disabled_stays_damaged &= damaged.Gamma.shell_count(3) + 1 == expected_complete
            && !damaged.Gamma.contains_executable(leaf)
            && damaged.repair_disabled_kernel_is_closed()
            && repair_disabled_state.Gamma == damaged.Gamma;
        repair_disabled_rejects_primary_event &= disabled_rejects_event;

        let mut withheld = damaged.clone();
        withheld.F.remove(&descriptor);
        descriptor_withheld_blocks_repair &=
            withheld.enabled_events().is_empty() && !withheld.Gamma.contains_executable(leaf);

        let repair = instantiate_one_support_trace(damaged)
            .map_err(|error| format!("b{base} repair {leaf:?}: {error:?}"))?;
        let repaired_state = classify_prefix_reservoir(&PrefixReservoir::from_complete_state(
            &repair.terminal_state,
        ));
        evidence.observations.push(format!(
            "stage=MAINTENANCE_REPAIRED base={} leaf={:?} classification={:?} thermal_result={:?} complete_state={:?}",
            base,
            leaf,
            repaired_state.classification,
            repaired_state,
            repair.terminal_state,
        ));
        every_repair_restores &= repair.terminal_state.Gamma.contains_executable(leaf)
            && repair.terminal_state.Gamma.shell_count(3) == expected_complete
            && repaired_state == completed_state;
        every_repair_inverse_and_ledger_closes &= repair.every_forward_map_has_exact_local_inverse
            && repair.every_working_ledger_closes
            && repair.every_augmented_ledger_closes
            && repair.every_complete_ledger_closes
            && repair.every_forward_energy_difference_matches_ledger
            && repair.every_forward_resource_difference_matches_ledger
            && repair.every_state_has_nontruncating_attempt_bit_capacity;
        every_repair_event_surface_replays &= protocol_event_surface_replays(&repair)?;
        every_repair_uses_existing_reserve &= repair_cells_before == model::REPAIR_RESERVE_CELLS
            && repair.initial_state.B_W.B_repair.charged_count() == repair_cells_before
            && repair.terminal_state.B_W.B_repair.charged_count() < repair_cells_before;
        let repair_protocol =
            protocol_transcript(format!("maintenance-b{base}-leaf{:?}", leaf.0), &repair)?;
        evidence.protocols.push(repair_protocol);
    }
    Ok(MaintenanceModelReport {
        leaf_arms: leaves.len(),
        every_damaged_count_matches,
        every_damaged_temperature_rejected,
        every_repair_restores,
        every_repair_inverse_and_ledger_closes,
        every_repair_event_surface_replays,
        every_repair_uses_existing_reserve,
        repair_disabled_stays_damaged,
        repair_disabled_rejects_primary_event,
        descriptor_withheld_blocks_repair,
    })
}

fn information_report(
    scope: impl Into<String>,
    record_variable: impl Into<String>,
    branch_evidence: impl Into<String>,
    information: &OutcomeCopyInformation,
    physical_copy_verified: bool,
) -> InformationReport {
    InformationReport {
        scope: scope.into(),
        record_variable: record_variable.into(),
        branch_evidence: branch_evidence.into(),
        outcome_probabilities: information.outcome_probabilities,
        record_probabilities: information.record_probabilities,
        joint_distribution: information.joint_distribution,
        outcome_entropy: information.outcome_entropy.clone(),
        record_outcome_mutual_information: information.record_outcome_mutual_information.clone(),
        record_is_exact_copy: information.terminal_record_is_exact_copy && physical_copy_verified,
    }
}

fn energy_shuffle(reservoir: &PrefixReservoir) -> PrefixReservoir {
    PrefixReservoir::new(
        reservoir.alphabet_size,
        reservoir.energy_quantum,
        reservoir
            .promoted
            .iter()
            .map(|state| ReservoirAlternative {
                descriptor_id: state.descriptor_id,
                word: state.word.clone(),
                energy_index: match state.energy_index {
                    1 => 2,
                    2 => 1,
                    other => other,
                },
                executable: state.executable,
            })
            .collect(),
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PoisonHarness {
    authoritative_state: CompleteState,
    forbidden_fields: PoisonFields,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PoisonInputAudit {
    sentinels_are_distinct: bool,
    construction_trace_identical: bool,
    off_shell_trace_identical: bool,
    repair_trace_identical: bool,
    every_projected_event_surface_replays: bool,
}

impl PoisonInputAudit {
    fn passes(&self) -> bool {
        self.sentinels_are_distinct
            && self.construction_trace_identical
            && self.off_shell_trace_identical
            && self.repair_trace_identical
            && self.every_projected_event_surface_replays
    }
}

fn trace_projected_from_poison_harness(harness: &PoisonHarness) -> Result<ProtocolTrace, String> {
    instantiate_one_support_trace(harness.authoritative_state.clone())
        .map_err(|error| format!("poison projected trace: {error:?}"))
}

fn poison_pair_trace_is_identical(
    authoritative_state: CompleteState,
    first: &PoisonFields,
    second: &PoisonFields,
) -> Result<(bool, bool), String> {
    let first_harness = PoisonHarness {
        authoritative_state: authoritative_state.clone(),
        forbidden_fields: first.clone(),
    };
    let second_harness = PoisonHarness {
        authoritative_state,
        forbidden_fields: second.clone(),
    };
    let first_trace = trace_projected_from_poison_harness(&first_harness)?;
    let second_trace = trace_projected_from_poison_harness(&second_harness)?;
    let trace_identical = first_harness.forbidden_fields != second_harness.forbidden_fields
        && first_trace.initial_state.forbidden_input_projection()
            == second_trace.initial_state.forbidden_input_projection()
        && first_trace.initial_state.enabled_events()
            == second_trace.initial_state.enabled_events()
        && first_trace.path_law_projection() == second_trace.path_law_projection()
        && first_trace.terminal_state == second_trace.terminal_state
        && first_trace == second_trace;
    let surfaces_replay = protocol_event_surface_replays(&first_trace)?
        && protocol_event_surface_replays(&second_trace)?;
    Ok((trace_identical, surfaces_replay))
}

fn poison_input_audit() -> Result<PoisonInputAudit, String> {
    let first = PoisonFields {
        beta: -1_000_003,
        target_count: 1_000_033,
        held_out_rate: -1_000_037,
    };
    let second = PoisonFields {
        beta: 1_000_081,
        target_count: -1_000_099,
        held_out_rate: 1_000_117,
    };
    let (construction_trace_identical, construction_replays) =
        poison_pair_trace_is_identical(primary_fixture(2, false), &first, &second)?;
    let (off_shell_trace_identical, off_shell_replays) =
        poison_pair_trace_is_identical(primary_fixture(2, true), &first, &second)?;

    let completed = instantiate_one_support_trace(primary_fixture(2, false))
        .map_err(|error| format!("poison repair setup construction: {error:?}"))?
        .terminal_state;
    let leaf = completed
        .Gamma
        .shell_words(3)
        .into_iter()
        .next()
        .ok_or_else(|| "poison repair setup has no depth-three leaf".to_string())?;
    let mut scheduled = completed;
    scheduled
        .schedule_leaf_perturbation(leaf.clone())
        .map_err(|error| format!("poison repair schedule {leaf:?}: {error:?}"))?;
    let perturb = scheduled
        .enabled_events()
        .into_iter()
        .find(|event| matches!(event, EnabledEvent::Perturb { .. }))
        .ok_or_else(|| "poison repair perturb event absent".to_string())?;
    let damaged = scheduled
        .event_transition(&perturb, KernelSupport::PRIMARY)
        .map_err(|error| format!("poison repair perturb transition: {error:?}"))?
        .successor;
    let (repair_trace_identical, repair_replays) =
        poison_pair_trace_is_identical(damaged, &first, &second)?;

    Ok(PoisonInputAudit {
        sentinels_are_distinct: first != second,
        construction_trace_identical,
        off_shell_trace_identical,
        repair_trace_identical,
        every_projected_event_surface_replays: construction_replays
            && off_shell_replays
            && repair_replays,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BuilderAdmissionAudit {
    clean_authoritative_state_admitted: bool,
    target_payload_is_concrete: bool,
    target_payload_rejected_before_policy: bool,
    target_payload_admitted_to_policy: bool,
    traffic_payload_is_concrete: bool,
    traffic_payload_rejected_before_policy: bool,
    traffic_payload_admitted_to_policy: bool,
}

impl BuilderAdmissionAudit {
    fn target_control_passes(&self) -> bool {
        self.clean_authoritative_state_admitted
            && self.target_payload_is_concrete
            && self.target_payload_rejected_before_policy
    }

    fn traffic_control_passes(&self) -> bool {
        self.clean_authoritative_state_admitted
            && self.traffic_payload_is_concrete
            && self.traffic_payload_rejected_before_policy
    }

    fn demonstrates_primary_contamination(&self, poison: &PoisonInputAudit) -> bool {
        self.target_payload_admitted_to_policy
            || self.traffic_payload_admitted_to_policy
            || (poison.sentinels_are_distinct
                && (!poison.construction_trace_identical
                    || !poison.off_shell_trace_identical
                    || !poison.repair_trace_identical))
    }
}

fn builder_admission_audit(binary: &SpecimenEvidence) -> BuilderAdmissionAudit {
    let authoritative_state = &binary.trace.initial_state;
    let initial_counts =
        classify_prefix_reservoir(&PrefixReservoir::from_complete_state(authoritative_state))
            .shell_counts;
    let global_deficit = binary
        .state
        .shell_counts
        .iter()
        .enumerate()
        .map(|(depth, &target)| {
            target as i128 - initial_counts.get(depth).copied().unwrap_or(0) as i128
        })
        .collect::<Vec<_>>();
    let target_payload = TargetCountRequest {
        shell_counts: binary.state.shell_counts.clone(),
        target_multiplicity_ratios: binary
            .state
            .adjacent_ratios
            .iter()
            .filter_map(|ratio| *ratio)
            .collect(),
        global_deficit,
    };
    let traffic_payload = HeldOutTrafficRequest {
        labelled_rate_pairs: [(0_usize, 1_usize), (0, 2), (1, 2)]
            .into_iter()
            .map(|(left, right)| {
                (
                    binary.probe.macro_generator[left][right],
                    binary.probe.macro_generator[right][left],
                )
            })
            .collect(),
        agreement_reward: Ratio::ONE,
    };
    let clean_authoritative_state_admitted =
        admit_builder_request(BuilderRequest::Local(authoritative_state)).is_ok_and(|view| {
            view.Gamma == &authoritative_state.Gamma
                && view.M == &authoritative_state.M
                && view.B_W == &authoritative_state.B_W
                && view.enabled_candidate_events(KernelSupport::PRIMARY)
                    == authoritative_state
                        .enabled_events()
                        .into_iter()
                        .filter(|event| matches!(event, EnabledEvent::Candidate { .. }))
                        .collect::<Vec<_>>()
        });
    let target_admission = admit_builder_request(BuilderRequest::TargetCount(
        authoritative_state,
        &target_payload,
    ));
    let target_payload_admitted_to_policy = target_admission.is_ok();
    let target_payload_rejected_before_policy = target_admission
        .as_ref()
        .is_err_and(|error| *error == BuilderAdmissionError::TargetCountInput);
    let traffic_admission = admit_builder_request(BuilderRequest::HeldOutTraffic(
        authoritative_state,
        &traffic_payload,
    ));
    let traffic_payload_admitted_to_policy = traffic_admission.is_ok();
    let traffic_payload_rejected_before_policy = traffic_admission
        .as_ref()
        .is_err_and(|error| *error == BuilderAdmissionError::HeldOutTrafficInput);
    BuilderAdmissionAudit {
        clean_authoritative_state_admitted,
        target_payload_is_concrete: target_payload.shell_counts == binary.state.shell_counts
            && !target_payload.target_multiplicity_ratios.is_empty()
            && target_payload
                .global_deficit
                .iter()
                .any(|&deficit| deficit > 0),
        target_payload_rejected_before_policy,
        target_payload_admitted_to_policy,
        traffic_payload_is_concrete: traffic_payload.labelled_rate_pairs.len() == 3
            && traffic_payload
                .labelled_rate_pairs
                .iter()
                .all(|(forward, reverse)| forward.is_positive() && reverse.is_positive())
            && traffic_payload.agreement_reward.is_positive(),
        traffic_payload_rejected_before_policy,
        traffic_payload_admitted_to_policy,
    }
}

fn outcome(
    control: ControlId,
    passed: bool,
    expected: &'static str,
    observed: impl Into<String>,
) -> ControlOutcome {
    ControlOutcome {
        control,
        passed,
        expected,
        observed: observed.into(),
    }
}

fn control_passed(controls: &[ControlOutcome], control: ControlId) -> bool {
    controls
        .iter()
        .find(|outcome| outcome.control == control)
        .is_some_and(|outcome| outcome.passed)
}

fn evaluate_controls(
    specimens: &[SpecimenEvidence],
    protocol: &model::DrivenProtocolAudit,
    memory: &model::MemoryInterventionAudit,
    poison: &PoisonInputAudit,
    builder_admission: &BuilderAdmissionAudit,
    descriptor_orders: &[DescriptorOrderAudit],
    equal_contact: &thermal::FrozenContactFixtureResult,
    unequal_contact: &thermal::FrozenContactFixtureResult,
    contact_activity: &thermal::FrozenContactActivityControl,
) -> Result<Vec<ControlOutcome>, String> {
    let binary = specimens
        .iter()
        .find(|specimen| specimen.base == 2)
        .ok_or_else(|| "binary specimen absent".to_string())?;
    let binary_reservoir = PrefixReservoir::from_complete_state(&binary.trace.terminal_state);
    let memory_report = memory_causality_report();
    let legacy = legacy_ap_no_go_report();
    let static_tree = verify_static_perfect_tree_control(2, Ratio::ONE);
    let nonadaptive = constant_r_nonadaptive_report();
    let external_thermostat = verify_external_thermostat_control();
    let gibbs_installed = verify_gibbs_installed_control();
    let port_mutation = verify_port_law_mutation_control(2, Ratio::ONE);
    let blank_clamp = blank_live_bucket_clamp_report();
    let ideal_counters = ideal_counter_downgrade_report();
    let shuffled = classify_prefix_reservoir(&energy_shuffle(&binary_reservoir));
    let probe_controls = verify_probe_controls(&binary_reservoir);

    let mut off_shell_passes = true;
    let mut off_shell_details = Vec::new();
    for base in ALPHABET_BASES {
        let trace = instantiate_one_support_trace(primary_fixture(usize::from(base), true))
            .map_err(|error| format!("b{base} off-shell: {error:?}"))?;
        let state =
            classify_prefix_reservoir(&PrefixReservoir::from_complete_state(&trace.terminal_state));
        let probe =
            verify_held_out_probe(&PrefixReservoir::from_complete_state(&trace.terminal_state));
        let reference = specimens
            .iter()
            .find(|specimen| specimen.base == base)
            .ok_or_else(|| format!("b{base} primary absent"))?;
        let workspace_depth_four = trace
            .terminal_state
            .Gamma
            .workspace
            .keys()
            .filter(|word| word.depth() == 4)
            .count();
        off_shell_passes &= workspace_depth_four == 1
            && state.shell_counts == reference.state.shell_counts
            && state.temperature == reference.state.temperature
            && probe.operational_closure_passes()
            && probe.fiber_sizes == reference.probe.fiber_sizes
            && probe.ldb_temperature == reference.probe.ldb_temperature
            && state.temperature == probe.ldb_temperature
            && trace.terminal_state.F.is_empty()
            && matches!(trace.terminal_state.P, Phase::Build { depth: 4 })
            && protocol_event_surface_replays(&trace)?
            && trace.every_forward_map_has_exact_local_inverse
            && trace.full_reverse_lifo_restores_initial
            && trace.every_working_ledger_closes
            && trace.every_augmented_ledger_closes
            && trace.every_complete_ledger_closes
            && trace.every_forward_resource_difference_matches_ledger
            && trace.every_state_has_nontruncating_attempt_bit_capacity;
        off_shell_details.push(format!("b{base}:workspace4={workspace_depth_four}"));
    }

    let maintenance_controls = specimens.iter().all(|specimen| {
        specimen.maintenance.every_damaged_count_matches
            && specimen.maintenance.every_damaged_temperature_rejected
            && specimen.maintenance.every_repair_restores
            && specimen.maintenance.every_repair_event_surface_replays
    });
    let repair_disabled = specimens.iter().all(|specimen| {
        specimen.maintenance.repair_disabled_stays_damaged
            && specimen.maintenance.repair_disabled_rejects_primary_event
    }) && protocol.repair_disabled_kernel_is_closed;
    let descriptor_withheld = specimens
        .iter()
        .all(|specimen| specimen.maintenance.descriptor_withheld_blocks_repair);
    let descriptor_replays = descriptor_orders
        .iter()
        .map(|audit| {
            Ok(protocol_event_surface_replays(&audit.forward)?
                && protocol_event_surface_replays(&audit.reversed)?)
        })
        .collect::<Result<Vec<bool>, String>>()?;
    let descriptor_permutation_passes = descriptor_orders.len() == ALPHABET_BASES.len()
        && descriptor_orders
            .iter()
            .zip(&descriptor_replays)
            .all(|(audit, replay)| {
                audit.path_law_quotient_identical
                    && audit.promoted_words_identical
                    && audit.shell_counts_identical
                    && audit.forward.terminal_state.Gamma.reservoir.keys().eq(audit
                        .reversed
                        .terminal_state
                        .Gamma
                        .reservoir
                        .keys())
                    && *replay
            })
        && protocol.descriptor_order_path_law_invariant;
    let memory_permutation_passes = memory_report.observed_law.fresh_before_alias_probability
        == Ratio::new(280, 523)
        && memory_report.intervened_law.fresh_before_alias_probability == Ratio::new(1, 2)
        && memory_report.observed_law.expected_work_cells == Ratio::new(766, 523)
        && memory_report.intervened_law.expected_work_cells == Ratio::new(3, 2);
    let hidden_memory_rejected = memory.full_non_memory_coordinates_identical
        && memory.candidate_support_identical
        && memory.observed_live_bucket_counts != memory.intervened_live_bucket_counts
        && memory_report.next_action_law_changes;
    let activity_invariant = probe_controls.activity_only_invariant
        && contact_activity.invariant
        && equal_contact.passes()
        && unequal_contact.passes();

    Ok(vec![
        outcome(
            ControlId::StaticPerfectTree,
            static_tree.matches_frozen_classification(),
            "OPERATIONAL_THERMAL_ONLY@E02/E04/E06",
            format!(
                "observer_supplied={} state={:?} probe_closure={} creation={} adaptation={}",
                static_tree.observer_supplied_architecture,
                static_tree.state.classification,
                static_tree.probe.operational_closure_passes(),
                static_tree.endogenous_creation_admitted,
                static_tree.adaptation_admitted,
            ),
        ),
        outcome(
            ControlId::NonadaptiveBuilder,
            nonadaptive.matches_frozen_classification(),
            "NOT_ADAPTIVE@E05",
            format!(
                "constant_hazard={} observed_P={} intervened_P={} constant_P={}",
                nonadaptive.installed_hazard,
                nonadaptive.observed_memory_law.fresh_before_alias_probability,
                nonadaptive.intervened_memory_law.fresh_before_alias_probability,
                nonadaptive.constant_policy_observed_law.fresh_before_alias_probability,
            ),
        ),
        outcome(
            ControlId::LegacyAPGate,
            legacy.strict_legacy_gate_deadlocks,
            "DEADLOCK@E06",
            "EXACT_DEADLOCK",
        ),
        outcome(
            ControlId::ExternalThermostat,
            external_thermostat.matches_frozen_classification(),
            "RATE_COMPATIBLE_ONLY@E03/E08",
            format!(
                "counts={} state={:?} installed_beta={} rate_compatible_only={}",
                join_counts(&external_thermostat.actual_shell_counts),
                external_thermostat.state_classification,
                external_thermostat.installed_beta,
                external_thermostat.rate_compatible_only,
            ),
        ),
        outcome(
            ControlId::GibbsInstalled,
            gibbs_installed.matches_frozen_classification(),
            "FITTED_REPRESENTATION_ONLY@E03/E08",
            format!(
                "counts={} installed_weights={:?} actual_projection={:?} fitted_only={}",
                join_counts(&gibbs_installed.actual_shell_counts),
                gibbs_installed.installed_gibbs_weights,
                gibbs_installed.actual_microcanonical_projection,
                gibbs_installed.fitted_representation_only,
            ),
        ),
        outcome(
            ControlId::TargetCountBuilder,
            builder_admission.target_control_passes(),
            "CAUSALLY_CONTAMINATED@E03",
            format!(
                "payload_concrete={} rejected_before_policy={} admitted_to_policy={}",
                builder_admission.target_payload_is_concrete,
                builder_admission.target_payload_rejected_before_policy,
                builder_admission.target_payload_admitted_to_policy,
            ),
        ),
        outcome(
            ControlId::TrafficTrainedBuilder,
            builder_admission.traffic_control_passes(),
            "CAUSALLY_CONTAMINATED@E03",
            format!(
                "payload_concrete={} rejected_before_policy={} admitted_to_policy={}",
                builder_admission.traffic_payload_is_concrete,
                builder_admission.traffic_payload_rejected_before_policy,
                builder_admission.traffic_payload_admitted_to_policy,
            ),
        ),
        outcome(
            ControlId::PoisonInput,
            poison.passes(),
            "PROJECTED_TRACE_UNCHANGED@E03",
            format!(
                "external_sentinels_distinct={} construction={} off_shell={} repair={} event_replay={}",
                poison.sentinels_are_distinct,
                poison.construction_trace_identical,
                poison.off_shell_trace_identical,
                poison.repair_trace_identical,
                poison.every_projected_event_surface_replays,
            ),
        ),
        outcome(
            ControlId::OffShellInventory,
            off_shell_passes,
            "ONE_INCOMPLETE_WORKSPACE_NODE;CORE_INVARIANT",
            off_shell_details.join(","),
        ),
        outcome(
            ControlId::DescriptorPermutation,
            descriptor_permutation_passes,
            "COUNTS_CLASSIFICATION_AND_PATH_LAW_QUOTIENT_INVARIANT",
            descriptor_orders
                .iter()
                .zip(ALPHABET_BASES)
                .map(|(audit, base)| {
                    format!(
                        "b{base}:path_law={}:promoted_words={}:shell_counts={}",
                        audit.path_law_quotient_identical,
                        audit.promoted_words_identical,
                        audit.shell_counts_identical,
                    )
                })
                .collect::<Vec<_>>()
                .join(","),
        ),
        outcome(
            ControlId::EnergyShuffle,
            shuffled.classification == StateTemperatureClass::NonPositive
                && shuffled.temperature.is_none(),
            "OLD_TEMPERATURE_REJECTED@E08",
            format!("classification={:?}", shuffled.classification),
        ),
        outcome(
            ControlId::LeafDeletion,
            maintenance_controls,
            "DAMAGED_THEN_RESTORED@E08/E12",
            "ALL_FROZEN_LEAF_ARMS_CHECKED",
        ),
        outcome(
            ControlId::RepairDisabled,
            repair_disabled,
            "DEFECT_REMAINS@E12",
            "NO_REPAIR_SUPPORT_NO_RETURN",
        ),
        outcome(
            ControlId::DescriptorWithheld,
            descriptor_withheld,
            "RESOURCE_OBSTRUCTION@E12",
            "RETURNED_DESCRIPTOR_IS_NECESSARY_RESOURCE",
        ),
        outcome(
            ControlId::PortLawMutation,
            port_mutation.matches_frozen_classification(),
            "OLD_TEMPERATURE_REJECTED@E08",
            format!(
                "parent_vertex={} port={} child={:?} multiplicity={} counts={} classification={:?}",
                port_mutation.duplicated_parent_vertex_id,
                port_mutation.duplicated_port,
                port_mutation.duplicated_canonical_child,
                port_mutation.duplicate_binding_multiplicity,
                join_counts(&port_mutation.actual_shell_counts),
                port_mutation.state_classification,
            ),
        ),
        outcome(
            ControlId::MemoryClamp,
            blank_clamp.matches_frozen_classification(),
            "P_FRESH_FIRST=1/2@E05",
            format!(
                "underlying_P={} clamped_P={} clamped_E_attempts={}",
                blank_clamp.underlying_law.fresh_before_alias_probability,
                blank_clamp.clamped_law.fresh_before_alias_probability,
                blank_clamp.clamped_law.expected_candidate_attempts,
            ),
        ),
        outcome(
            ControlId::MemoryPermutation,
            memory_permutation_passes,
            "280/523->1/2;766/523->3/2@E05",
            "FROZEN_DO_M_EFFECT_MATCHES",
        ),
        outcome(
            ControlId::HiddenMemory,
            hidden_memory_rejected,
            "NON_MARKOV_PROJECTION@E01",
            "SAME_NON_MEMORY_STATE_HAS_TWO_HAZARD_LAWS",
        ),
        outcome(
            ControlId::IdealCounters,
            ideal_counters.matches_frozen_classification(),
            "LOGICAL_ONLY;NO_FULL_LABEL@E11",
            format!(
                "logical_match={} finite_record_bank={} work_store={} inverse={} full_label_rejected={}",
                ideal_counters.logical_law_matches_physical_fixture,
                ideal_counters.has_finite_record_bank,
                ideal_counters.has_finite_work_store,
                ideal_counters.has_named_inverse_protocol,
                ideal_counters.full_physical_label_rejected,
            ),
        ),
        outcome(
            ControlId::LedgerLeak,
            protocol.ledger_leak_is_rejected,
            "FIRST_LAW_FAIL@E11",
            "ONE_OMITTED_CELL_BREAKS_AUGMENTED_CLOSURE",
        ),
        outcome(
            ControlId::ActivityOnly,
            activity_invariant,
            "TEMPERATURE_AND_STATIONARY_LAW_INVARIANT",
            "COMMON_CLOCK_DOUBLES_GENERATORS_AND_TRANSIENT_CURRENTS",
        ),
        outcome(
            ControlId::ReverseMismatch,
            probe_controls.reverse_mismatch_rejected,
            "REVERSE_SUPPORT/LUMPING_FAIL@E09",
            "ROW_CLOSED_MUTANT_REJECTED",
        ),
    ])
}

fn gate(gate: GateId, passed: bool, detail: impl Into<String>) -> GateOutcome {
    GateOutcome {
        gate,
        passed,
        detail: detail.into(),
    }
}

fn state_classification_label(classification: StateTemperatureClass) -> &'static str {
    match classification {
        StateTemperatureClass::Undefined => "UNDEFINED",
        StateTemperatureClass::NonPositive => "NONPOSITIVE",
        StateTemperatureClass::StateDependent => "STATE_DEPENDENT",
        StateTemperatureClass::UniqueConstant => "UNIQUE_CONSTANT",
    }
}

fn traffic_classification_label(probe: &ProbeThermalResult) -> &'static str {
    if probe.operational_closure_passes() {
        "UNIQUE_LDB_EQUILIBRIUM"
    } else if probe.unique_ldb_temperature {
        "RATE_COMPATIBLE_ONLY"
    } else {
        "NOT_ADMITTED"
    }
}

fn probe_kernel_observations(
    base: u8,
    audit: thermal::ProbeAuthoritativeKernelAudit,
) -> (ProbeKernelEvidence, Vec<String>) {
    let kernel = ProbeKernelEvidence {
        protocol: audit.protocol.clone(),
        complete_coordinate_count: audit.complete_coordinates.len(),
        event_row_count: audit.event_rows.len(),
        passes: audit.passes(),
    };
    let mut observations = vec![format!(
        "scope=b{} reference_complete_state={:?} protocol={:?} complete_coordinates={:?} state_reconstruction={} event_replay={} diagonals={} reciprocal_ledgers={}",
        base,
        audit.reference_xypher_state,
        audit.protocol,
        audit.complete_coordinates,
        audit.every_state_reconstructs_from_reference,
        audit.every_enabled_event_replays_from_one_authoritative_kernel,
        audit.every_diagonal_is_negative_outgoing_sum,
        audit.every_event_has_exact_reciprocal_and_heat_ledger,
    )];
    observations.extend(
        audit
            .event_rows
            .into_iter()
            .enumerate()
            .map(|(index, row)| format!("scope=b{base} event_index={index} exact_row={row:?}")),
    );
    (kernel, observations)
}

fn contact_summary_observations(
    equal_contact: &thermal::FrozenContactFixtureResult,
    unequal_contact: &thermal::FrozenContactFixtureResult,
    equal_kernel: &thermal::ContactAuthoritativeKernelAudit,
    unequal_kernel: &thermal::ContactAuthoritativeKernelAudit,
) -> Vec<String> {
    let mut observations = vec![
        format!(
            "fixture=EQUAL exact_result={:?} reference_left_complete_state={:?} reference_right_complete_state={:?} shared_energy_gauge={} protocol={:?} complete_coordinates={:?} both_full_states={} state_reconstruction={} event_replay={} diagonals={} reciprocal_ledgers={}",
            equal_contact,
            equal_kernel.reference_left_state,
            equal_kernel.reference_right_state,
            equal_kernel.shared_energy_gauge,
            equal_kernel.protocol,
            equal_kernel.complete_coordinates,
            equal_kernel.both_full_xypher_states_are_authoritative,
            equal_kernel.every_state_reconstructs_from_references,
            equal_kernel.every_enabled_event_replays_from_one_authoritative_kernel,
            equal_kernel.every_diagonal_is_negative_outgoing_sum,
            equal_kernel.every_event_has_exact_reciprocal_and_heat_ledger,
        ),
        format!(
            "fixture=UNEQUAL exact_result={:?} reference_left_complete_state={:?} reference_right_complete_state={:?} shared_energy_gauge={} protocol={:?} complete_coordinates={:?} both_full_states={} state_reconstruction={} event_replay={} diagonals={} reciprocal_ledgers={}",
            unequal_contact,
            unequal_kernel.reference_left_state,
            unequal_kernel.reference_right_state,
            unequal_kernel.shared_energy_gauge,
            unequal_kernel.protocol,
            unequal_kernel.complete_coordinates,
            unequal_kernel.both_full_xypher_states_are_authoritative,
            unequal_kernel.every_state_reconstructs_from_references,
            unequal_kernel.every_enabled_event_replays_from_one_authoritative_kernel,
            unequal_kernel.every_diagonal_is_negative_outgoing_sum,
            unequal_kernel.every_event_has_exact_reciprocal_and_heat_ledger,
        ),
    ];
    observations.extend(
        equal_kernel
            .event_rows
            .iter()
            .enumerate()
            .map(|(index, row)| format!("fixture=EQUAL event_index={index} exact_row={row:?}")),
    );
    observations.extend(
        unequal_kernel
            .event_rows
            .iter()
            .enumerate()
            .map(|(index, row)| format!("fixture=UNEQUAL event_index={index} exact_row={row:?}")),
    );
    observations
}

fn enabled_candidate_hazards(state: &CompleteState) -> BTreeMap<ModelEvent, Ratio> {
    state
        .enabled_events()
        .into_iter()
        .filter(|event| matches!(event, EnabledEvent::Candidate { .. }))
        .map(|event| (event.event(), event.hazard()))
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MemoryKernelAudit {
    initial_state_is_well_formed: bool,
    observed_state_is_reachable_and_well_formed: bool,
    absorb_event_surface_replays: bool,
    intervention_holds_non_memory_coordinates_fixed: bool,
    intervention_holds_candidate_support_fixed: bool,
    observed_event_surface_replays: bool,
    intervened_event_surface_is_defined: bool,
    intervention_is_not_an_autonomous_transition: bool,
    intervened_state_is_off_manifold_scm_surgery: bool,
    exact_hazard_change_matches_frozen_values: bool,
}

fn complete_state_hazard_replay(
    memory: &model::MemoryInterventionAudit,
) -> Result<MemoryKernelAudit, String> {
    let initial = model::causal_memory_fixture();
    let absorb = initial
        .enabled_events()
        .into_iter()
        .find(|event| matches!(event, EnabledEvent::Absorb { .. }))
        .ok_or_else(|| "hazard replay absorption event absent".to_string())?;
    let absorb_event_surface_replays = transition_replays_from_complete_state(&initial, &absorb)?;
    let observed_from_absorb = initial
        .event_transition(&absorb, KernelSupport::PRIMARY)
        .map_err(|error| format!("hazard replay absorption: {error:?}"))?
        .successor;
    let observed = &memory.intervention.observed_state;
    let intervened = &memory.intervention.intervened_state;
    let observed_hazards = enabled_candidate_hazards(observed);
    let intervened_hazards = enabled_candidate_hazards(intervened);
    let observed_fresh = observed_hazards
        .iter()
        .find(|(event, _)| {
            matches!(event, ModelEvent::Candidate(candidate) if candidate.kind == model::CandidateKind::Fresh)
        })
        .map(|(_, hazard)| *hazard);
    let observed_alias = observed_hazards
        .iter()
        .find(|(event, _)| {
            matches!(event, ModelEvent::Candidate(candidate) if candidate.kind == model::CandidateKind::Alias)
        })
        .map(|(_, hazard)| *hazard);
    let intervened_fresh = intervened_hazards
        .iter()
        .find(|(event, _)| {
            matches!(event, ModelEvent::Candidate(candidate) if candidate.kind == model::CandidateKind::Fresh)
        })
        .map(|(_, hazard)| *hazard);
    let intervened_alias = intervened_hazards
        .iter()
        .find(|(event, _)| {
            matches!(event, ModelEvent::Candidate(candidate) if candidate.kind == model::CandidateKind::Alias)
        })
        .map(|(_, hazard)| *hazard);
    let observed_event_surface_replays = observed
        .enabled_events()
        .iter()
        .map(|event| transition_replays_from_complete_state(observed, event))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .all(|passes| passes);
    let intervened_event_surface_is_defined = intervened
        .enabled_events()
        .iter()
        .map(|event| transition_replays_from_complete_state(intervened, event))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .all(|passes| passes);
    let intervention_is_not_an_autonomous_transition = observed
        .enabled_events()
        .iter()
        .map(|event| observed.event_transition(event, KernelSupport::PRIMARY))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("observed successor audit: {error:?}"))?
        .into_iter()
        .all(|transition| !transition.successor.eq(intervened));
    Ok(MemoryKernelAudit {
        initial_state_is_well_formed: initial.complete_state_is_well_formed(),
        observed_state_is_reachable_and_well_formed: memory.observed_state_is_well_formed
            && observed_from_absorb.eq(observed),
        absorb_event_surface_replays,
        intervention_holds_non_memory_coordinates_fixed: memory
            .full_non_memory_coordinates_identical,
        intervention_holds_candidate_support_fixed: observed_hazards
            .keys()
            .eq(intervened_hazards.keys()),
        observed_event_surface_replays,
        intervened_event_surface_is_defined: intervened_event_surface_is_defined
            && memory.post_intervention_kernel_defined
            && memory.every_post_intervention_transition_replays,
        intervention_is_not_an_autonomous_transition,
        intervened_state_is_off_manifold_scm_surgery: memory.intervention_is_off_manifold_scm
            && memory.exactly_one_memory_coordinate_replaced
            && memory.historical_absorb_record_is_unchanged,
        exact_hazard_change_matches_frozen_values: observed_fresh == Some(Ratio::new(175, 216))
            && observed_alias == Some(Ratio::new(45, 64))
            && intervened_fresh == Some(Ratio::new(45, 64))
            && intervened_alias == Some(Ratio::new(45, 64)),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExactInformationAudit {
    reports: Vec<InformationReport>,
    causal_physical_records_are_injective: bool,
    construction_anchor_matches_physical_branch: bool,
    every_construction_quotient_reported: bool,
    every_information_identity_passes: bool,
}

impl ExactInformationAudit {
    fn passes(&self) -> bool {
        self.causal_physical_records_are_injective
            && self.construction_anchor_matches_physical_branch
            && self.every_construction_quotient_reported
            && self.every_information_identity_passes
            && !self.reports.is_empty()
            && self
                .reports
                .iter()
                .all(|report| report.record_is_exact_copy)
    }
}

fn exact_information_audit() -> Result<ExactInformationAudit, String> {
    let initial = model::causal_memory_fixture();
    let absorb = initial
        .enabled_events()
        .into_iter()
        .find(|event| matches!(event, EnabledEvent::Absorb { .. }))
        .ok_or_else(|| "information audit absorption event absent".to_string())?;
    let observed = initial
        .event_transition(&absorb, KernelSupport::PRIMARY)
        .map_err(|error| format!("information audit absorption: {error:?}"))?
        .successor;
    let physical = physical_branch_record_audit(&observed)
        .map_err(|error| format!("physical branch record audit: {error:?}"))?;
    let causal_probability = physical.fresh.probability;
    let causal_information = exact_outcome_copy_information(causal_probability);
    let causal_law = memory_causality_report().observed_law;
    let causal_physical_records_are_injective = physical.records_injectively_identify_first_event
        && physical.records_are_distinct
        && physical.fresh.record_identifies_outcome
        && physical.alias.record_identifies_outcome
        && matches!(
            physical.fresh.operation_record.entry.as_ref(),
            Some(model::OperationEntry::Fresh { .. })
        )
        && matches!(
            physical.alias.operation_record.entry.as_ref(),
            Some(model::OperationEntry::AliasQuarantine { .. })
        )
        && physical.fresh_hazard == causal_law.fresh_hazard
        && physical.alias_hazard == causal_law.alias_hazard
        && physical.fresh.probability == causal_law.fresh_before_alias_probability
        && physical.alias.probability == Ratio::ONE - physical.fresh.probability
        && causal_information.exact_copy_identity_passes();
    let mut reports = vec![information_report(
        "causal-live-port-a1",
        "first_operation_record_entry",
        format!(
            "reported_record=immutable_first_action_operation_entry fresh_immediate_semantic_record={} fresh_immediate_semantic_state={:?} fresh_operation_record={:?} alias_immediate_semantic_record={} alias_immediate_semantic_state={:?} alias_operation_record={:?} hazards={},{}",
            physical.fresh.semantic_record.id,
            physical.fresh.semantic_record.state,
            physical.fresh.operation_record,
            physical.alias.semantic_record.id,
            physical.alias.semantic_record.state,
            physical.alias.operation_record,
            physical.fresh_hazard,
            physical.alias_hazard,
        ),
        &causal_information,
        causal_physical_records_are_injective,
    )];

    let mut construction_anchor_matches_physical_branch = false;
    let mut every_construction_quotient_reported = true;
    let mut every_information_identity_passes = causal_information.exact_copy_identity_passes();
    for base in ALPHABET_BASES {
        let frontier_reports = construction_frontier_information_reports(base);
        every_construction_quotient_reported &= frontier_reports.len() == 3
            && frontier_reports
                .iter()
                .all(|frontier| frontier.exact_information_account_passes());
        for frontier in frontier_reports {
            for quotient in frontier.quotients {
                let is_physical_anchor = quotient.fresh_hazard == physical.fresh_hazard
                    && quotient.alias_hazard == physical.alias_hazard
                    && quotient.fresh_first_probability == physical.fresh.probability;
                if is_physical_anchor {
                    construction_anchor_matches_physical_branch = quotient
                        .physical_branch_records_copy_first_outcome
                        && physical.records_injectively_identify_first_event;
                }
                let quotient_identity = quotient.information.exact_copy_identity_passes()
                    && quotient.physical_branch_records_copy_first_outcome
                    && quotient.physical_hazards_match_posterior
                    && quotient.physical_probabilities_match_hazards
                    && quotient.physical_audit.branch_probabilities_are_normalized
                    && quotient.physical_audit.representative_is_reachable_quotient
                    && quotient.physical_audit.branch_hazards_match_representative
                    && quotient
                        .physical_audit
                        .records_injectively_copy_first_outcome
                    && quotient
                        .physical_audit
                        .both_branches_complete_required_record_protocol
                    && quotient.branch_expectation_matches_formula
                    && quotient.fresh_first_immediate_record == adaptive::RecordState::Pending
                    && quotient.fresh_first_terminal_record == adaptive::RecordState::Success
                    && quotient.alias_first_immediate_record == adaptive::RecordState::Failure
                    && quotient.alias_first_terminal_record == adaptive::RecordState::Failure
                    && quotient.alias_then_fresh_immediate_record == adaptive::RecordState::Pending
                    && quotient.alias_then_fresh_terminal_record == adaptive::RecordState::Success
                    && quotient.alias_then_fresh_is_forced;
                every_information_identity_passes &= quotient_identity;
                reports.push(information_report(
                    format!(
                        "construction-b{}-depth{}-prior_alias_failures{}",
                        quotient.base,
                        quotient.frontier_depth,
                        quotient.prior_alias_failures,
                    ),
                    "terminal_semantic_record_state",
                    format!(
                        "prior_fresh_successes={} hazards={},{} attempts_if_fresh={} attempts_if_alias={} E_attempts={} pending_to_success={} alias_failure={} forced_followup={} reachable_steps={} fresh_operation_record={} alias_operation_record={} physical_anchor={}",
                        quotient.prior_fresh_successes,
                        quotient.fresh_hazard,
                        quotient.alias_hazard,
                        quotient.fresh_first_branch_attempts,
                        quotient.alias_first_branch_attempts,
                        quotient.branch_weighted_attempt_expectation,
                        quotient.fresh_first_credit_required,
                        quotient.alias_first_terminal_record == adaptive::RecordState::Failure,
                        quotient.alias_then_fresh_is_forced,
                        quotient.physical_audit.representative.reachability_steps.len(),
                        quotient
                            .physical_audit
                            .first_event_records
                            .fresh
                            .operation_record
                            .id,
                        quotient
                            .physical_audit
                            .first_event_records
                            .alias
                            .operation_record
                            .id,
                        is_physical_anchor,
                    ),
                    &quotient.information,
                    quotient_identity,
                ));
            }
        }
    }

    Ok(ExactInformationAudit {
        reports,
        causal_physical_records_are_injective,
        construction_anchor_matches_physical_branch,
        every_construction_quotient_reported,
        every_information_identity_passes,
    })
}

fn source_manifest_is_frozen() -> bool {
    const SOURCES: [(&str, &[u8]); 13] = [
        ("Cargo.toml", include_bytes!("../Cargo.toml")),
        ("Cargo.lock", include_bytes!("../Cargo.lock")),
        ("README.md", include_bytes!("../README.md")),
        ("xypher.runa", include_bytes!("../xypher.runa")),
        ("src/exact.rs", include_bytes!("exact.rs")),
        ("src/model.rs", include_bytes!("model.rs")),
        ("src/adaptive.rs", include_bytes!("adaptive.rs")),
        ("src/thermal.rs", include_bytes!("thermal.rs")),
        ("src/sha256.rs", include_bytes!("sha256.rs")),
        ("src/lib.rs", include_bytes!("lib.rs")),
        ("src/main.rs", include_bytes!("main.rs")),
        (
            "tests/source_contract.rs",
            include_bytes!("../tests/source_contract.rs"),
        ),
        (
            "tests/model_local.rs",
            include_bytes!("../tests/model_local.rs"),
        ),
    ];
    let mut lines = SOURCE_MANIFEST.lines();
    if lines.next() != Some("CAL-ENDO-1 authoritative source manifest v1") {
        return false;
    }
    for (path, bytes) in SOURCES {
        let Some(line) = lines.next() else {
            return false;
        };
        let Some(rest) = line.strip_prefix("sha256:") else {
            return false;
        };
        let Some((recorded, recorded_path)) = rest.split_once("  ") else {
            return false;
        };
        if recorded_path != path
            || recorded.len() != 64
            || !recorded
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            || recorded != sha256::hex(&sha256::digest(bytes))
        {
            return false;
        }
    }
    lines.next().is_none()
}

pub fn evaluate() -> ProofReport {
    let mut evidence = EvidenceJournal::default();
    match evaluate_inner(&mut evidence) {
        Ok(report) => report,
        Err(detail) => ProofReport::fatal(detail, evidence),
    }
}

fn evaluate_inner(evidence: &mut EvidenceJournal) -> Result<ProofReport, String> {
    let initial_states = ALPHABET_BASES
        .into_iter()
        .map(|base| (base, primary_fixture(usize::from(base), false)))
        .collect::<Vec<_>>();
    let initial_classifications = initial_states
        .iter()
        .map(|(base, state)| {
            (
                *base,
                classify_prefix_reservoir(&PrefixReservoir::from_complete_state(state)),
            )
        })
        .collect::<Vec<_>>();
    let initial_nonthermal = initial_classifications.iter().all(|(base, result)| {
        result.classification == StateTemperatureClass::Undefined
            && initial_prefix_report(*base).claimed_interval_has_too_few_nonempty_shells
    });
    for ((base, state), (classification_base, classification)) in
        initial_states.iter().zip(&initial_classifications)
    {
        evidence.observations.push(format!(
            "stage=INITIAL base={} base_alignment={} classification={:?} thermal_result={:?} complete_state={:?}",
            base,
            base == classification_base,
            classification.classification,
            classification,
            state,
        ));
    }

    let mut construction_traces = Vec::new();
    for (base, state) in initial_states {
        let trace = instantiate_one_support_trace(state)
            .map_err(|error| format!("b{base} construction: {error:?}"))?;
        evidence.observations.push(format!(
            "stage=CONSTRUCTION base={} operations={} initial_complete_state={:?} terminal_complete_state={:?}",
            base,
            trace.receipts.len(),
            trace.initial_state,
            trace.terminal_state,
        ));
        construction_traces.push((base, trace));
    }
    let state_classifications = construction_traces
        .iter()
        .map(|(base, trace)| {
            (
                *base,
                classify_prefix_reservoir(&PrefixReservoir::from_complete_state(
                    &trace.terminal_state,
                )),
            )
        })
        .collect::<Vec<_>>();
    let probe_classifications = construction_traces
        .iter()
        .map(|(base, trace)| {
            let reservoir = PrefixReservoir::from_complete_state(&trace.terminal_state);
            (*base, verify_held_out_probe(&reservoir))
        })
        .collect::<Vec<_>>();
    let temperatures_match_before_controls = state_classifications
        .iter()
        .zip(&probe_classifications)
        .all(|((state_base, state), (probe_base, probe))| {
            state_base == probe_base
                && state.temperature.is_some()
                && state.temperature == probe.ldb_temperature
        });
    let mut probe_kernels = Vec::new();
    for (((base, trace), (state_base, state)), (probe_base, probe)) in construction_traces
        .iter()
        .zip(&state_classifications)
        .zip(&probe_classifications)
    {
        evidence.observations.push(format!(
            "stage=STATE base={} base_alignment={} classification={:?} T_state={} thermal_result={:?} terminal_complete_state={:?}",
            base,
            base == state_base,
            state.classification,
            state
                .temperature
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| "UNDEFINED".to_string()),
            state,
            trace.terminal_state,
        ));
        evidence.observations.push(format!(
            "stage=TRAFFIC base={} base_alignment={} classification={} T_LDB={} probe_result={:?}",
            base,
            base == probe_base,
            traffic_classification_label(probe),
            probe
                .ldb_temperature
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| "UNDEFINED".to_string()),
            probe,
        ));
        let audit =
            thermal::audit_authoritative_probe_kernel(&trace.terminal_state, Ratio::ONE, false);
        let (kernel, observations) = probe_kernel_observations(*base, audit);
        probe_kernels.push((*base, kernel));
        evidence.thermal_observations.extend(observations);
    }

    let mut specimens = Vec::new();
    let mut probe_kernels = probe_kernels.into_iter();
    for (((base, trace), (initial_base, initial)), ((state_base, state), (probe_base, probe))) in
        construction_traces
            .into_iter()
            .zip(initial_classifications.into_iter())
            .zip(
                state_classifications
                    .into_iter()
                    .zip(probe_classifications.into_iter()),
            )
    {
        if base != initial_base || base != state_base || base != probe_base {
            return Err("specimen observation order lost base alignment".to_string());
        }
        let (kernel_base, probe_kernel) = probe_kernels
            .next()
            .ok_or_else(|| format!("b{base} authoritative probe kernel absent"))?;
        if kernel_base != base {
            return Err(format!(
                "b{base} authoritative probe kernel lost base alignment to b{kernel_base}"
            ));
        }
        let maintenance = verify_model_maintenance(&trace.terminal_state, base, evidence)?;
        specimens.push(SpecimenEvidence {
            base,
            initial,
            trace,
            state,
            probe,
            probe_kernel,
            maintenance,
        });
        let specimen = specimens.last().expect("a specimen was just appended");
        evidence.specimens.push(specimen_summary(specimen));
    }
    if probe_kernels.next().is_some() {
        return Err("authoritative probe kernel count exceeds specimen count".to_string());
    }

    let protocol =
        driven_protocol_audit().map_err(|error| format!("driven protocol audit: {error:?}"))?;
    let model_memory = memory_intervention_audit()
        .map_err(|error| format!("memory intervention audit: {error:?}"))?;
    let memory_kernel = complete_state_hazard_replay(&model_memory)?;
    let adaptive_loop = adaptive_loop_report();
    let memory = memory_causality_report();
    let information = exact_information_audit()?;
    evidence.information = information.reports.clone();
    let poison = poison_input_audit()?;
    let descriptor_orders = ALPHABET_BASES
        .into_iter()
        .map(|base| {
            descriptor_order_audit(usize::from(base), false)
                .map_err(|error| format!("b{base} descriptor-order audit: {error:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let binary = specimens
        .iter()
        .find(|specimen| specimen.base == 2)
        .ok_or_else(|| "binary specimen absent before controls".to_string())?;
    let builder_admission = builder_admission_audit(binary);
    let alias_before_fresh =
        instantiate_alias_before_fresh_support_trace(primary_fixture(2, false))
            .map_err(|error| format!("alias-before-fresh construction: {error:?}"))?;

    let left = PrefixReservoir::from_complete_state(&binary.trace.terminal_state);
    let equal_right_trace =
        instantiate_one_support_trace(primary_fixture_with_quantum(4, 2, false))
            .map_err(|error| format!("equal-contact right construction: {error:?}"))?;
    let unequal_right_trace =
        instantiate_one_support_trace(primary_fixture_with_quantum(2, 2, false))
            .map_err(|error| format!("unequal-contact right construction: {error:?}"))?;
    let equal_right = PrefixReservoir::from_complete_state(&equal_right_trace.terminal_state);
    let unequal_right = PrefixReservoir::from_complete_state(&unequal_right_trace.terminal_state);
    let equal_contact = verify_frozen_contact_fixture(
        FrozenContactFixture::EqualTemperature,
        &left,
        &equal_right,
        Ratio::ONE,
    );
    let unequal_contact = verify_frozen_contact_fixture(
        FrozenContactFixture::UnequalTemperature,
        &left,
        &unequal_right,
        Ratio::ONE,
    );
    let contact_activity = verify_frozen_contact_activity_control(
        FrozenContactFixture::UnequalTemperature,
        &left,
        &unequal_right,
    );
    let contact_protocol = thermal::frozen_contact_protocol(Ratio::ONE);
    let equal_contact_kernel = thermal::audit_authoritative_contact_kernel(
        &binary.trace.terminal_state,
        &equal_right_trace.terminal_state,
        &contact_protocol,
    );
    let unequal_contact_kernel = thermal::audit_authoritative_contact_kernel(
        &binary.trace.terminal_state,
        &unequal_right_trace.terminal_state,
        &contact_protocol,
    );
    evidence.observations.push(format!(
        "stage=CONTACT fixture=EQUAL classification={:?} result={:?}",
        equal_contact.contact.temperature_order, equal_contact,
    ));
    evidence.observations.push(format!(
        "stage=CONTACT fixture=UNEQUAL classification={:?} result={:?}",
        unequal_contact.contact.temperature_order, unequal_contact,
    ));
    evidence.contact_observations = contact_summary_observations(
        &equal_contact,
        &unequal_contact,
        &equal_contact_kernel,
        &unequal_contact_kernel,
    );

    for specimen in &specimens {
        retain_protocol(
            evidence,
            protocol_transcript(
                format!("primary-construction-b{}", specimen.base),
                &specimen.trace,
            )?,
        );
    }
    retain_protocol(
        evidence,
        protocol_transcript("alias-before-fresh-construction-b2", &alias_before_fresh)?,
    );
    for (audit, base) in descriptor_orders.iter().zip(ALPHABET_BASES) {
        retain_protocol(
            evidence,
            protocol_transcript(format!("descriptor-order-forward-b{base}"), &audit.forward)?,
        );
        retain_protocol(
            evidence,
            protocol_transcript(
                format!("descriptor-order-reversed-b{base}"),
                &audit.reversed,
            )?,
        );
    }
    retain_protocol(
        evidence,
        protocol_transcript(
            "equal-contact-right-construction-b4-lambda2",
            &equal_right_trace,
        )?,
    );
    retain_protocol(
        evidence,
        protocol_transcript(
            "unequal-contact-right-construction-b2-lambda2",
            &unequal_right_trace,
        )?,
    );
    let causal_initial = model::causal_memory_fixture();
    let absorb = causal_initial
        .enabled_events()
        .into_iter()
        .find(|event| matches!(event, EnabledEvent::Absorb { .. }))
        .ok_or_else(|| "causal transcript absorption absent".to_string())?;
    let (absorb_transcript, causal_observed) =
        single_transition_transcript("causal-memory-absorb", &causal_initial, &absorb)?;
    retain_protocol(evidence, absorb_transcript);
    for event in causal_observed
        .enabled_events()
        .into_iter()
        .filter(|event| matches!(event, EnabledEvent::Candidate { .. }))
    {
        let scope = format!("causal-physical-branch-{:?}", event.channel());
        let transcript = single_transition_transcript(scope, &causal_observed, &event)?.0;
        retain_protocol(evidence, transcript);
    }
    let all_protocol_transcripts_close = !evidence.protocols.is_empty()
        && evidence
            .protocols
            .iter()
            .all(|transcript| transcript.round_trip_exact)
        && evidence
            .protocols
            .iter()
            .all(|transcript| !transcript.forward.is_empty() && !transcript.inverse.is_empty());

    let controls = evaluate_controls(
        &specimens,
        &protocol,
        &model_memory,
        &poison,
        &builder_admission,
        &descriptor_orders,
        &equal_contact,
        &unequal_contact,
        &contact_activity,
    )?;
    evidence.controls = controls.clone();

    let primary_event_surfaces_replay = specimens
        .iter()
        .map(|specimen| protocol_event_surface_replays(&specimen.trace))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .all(|passes| passes);
    let authoritative_probe_replay = specimens.iter().all(|specimen| {
        specimen.probe_kernel.passes
            && specimen.probe_kernel.protocol.microscopic_clock == specimen.probe.microscopic_clock
            && specimen
                .probe_kernel
                .protocol
                .deleted_directed_edge
                .is_none()
            && specimen.probe_kernel.complete_coordinate_count as u128
                == specimen.probe.event_replay.complete_microstates_checked
            && specimen.probe_kernel.event_row_count as u128
                == specimen.probe.event_replay.directed_events_checked
    });
    let authoritative_contact_replay = equal_contact_kernel.passes()
        && unequal_contact_kernel.passes()
        && equal_contact_kernel.shared_energy_gauge == Ratio::ONE
        && unequal_contact_kernel.shared_energy_gauge == Ratio::ONE
        && equal_contact_kernel.protocol == contact_protocol
        && unequal_contact_kernel.protocol == contact_protocol
        && equal_contact_kernel.complete_coordinates.len() as u128
            == equal_contact
                .contact
                .event_replay
                .complete_product_microstates_checked
        && equal_contact_kernel.event_rows.len() as u128
            == equal_contact
                .contact
                .event_replay
                .directed_contact_events_checked
        && unequal_contact_kernel.complete_coordinates.len() as u128
            == unequal_contact
                .contact
                .event_replay
                .complete_product_microstates_checked
        && unequal_contact_kernel.event_rows.len() as u128
            == unequal_contact
                .contact
                .event_replay
                .directed_contact_events_checked;
    let authoritative_thermal_contact_replay =
        authoritative_probe_replay && authoritative_contact_replay;
    let complete_state = protocol.primary_fixture_is_markov_complete
        && specimens.iter().all(|specimen| {
            specimen.trace.initial_state.complete_state_is_well_formed()
                && specimen
                    .trace
                    .terminal_state
                    .complete_state_is_well_formed()
        })
        && primary_event_surfaces_replay
        && authoritative_thermal_contact_replay
        && protocol_event_surface_replays(&alias_before_fresh)?
        && all_protocol_transcripts_close
        && specimens
            .iter()
            .all(|specimen| specimen.maintenance.every_repair_event_surface_replays)
        && model_memory.full_non_memory_coordinates_identical
        && model_memory.candidate_support_identical
        && memory_kernel.initial_state_is_well_formed
        && memory_kernel.observed_state_is_reachable_and_well_formed
        && memory_kernel.absorb_event_surface_replays
        && memory_kernel.observed_event_surface_replays
        && memory_kernel.intervened_event_surface_is_defined
        && memory_kernel.intervention_is_not_an_autonomous_transition
        && memory_kernel.intervened_state_is_off_manifold_scm_surgery
        && information.every_construction_quotient_reported
        && information.every_information_identity_passes;
    let forbidden_non_use = poison.passes()
        && builder_admission.clean_authoritative_state_admitted
        && builder_admission.target_control_passes()
        && builder_admission.traffic_control_passes();
    let causal_loop = adaptive_loop.graph_state_changes_candidate_set
        && adaptive_loop.thermo_work_cell_causally_enables_action
        && adaptive_loop.fresh_changes_binding_and_records_pending
        && adaptive_loop.alias_quarantines_as_terminal_failure
        && protocol.all_driven_channels_exercised
        && model_memory.absorb_changes_no_graph_coordinate
        && model_memory.absorb_writes_one_terminal_record
        && alias_before_fresh.channel_count(DrivenChannel::AliasQuarantine) > 0
        && alias_before_fresh.channel_count(DrivenChannel::Fresh) > 0
        && information.every_information_identity_passes;
    let memory_causal = memory.absorption_is_reachable_blank_to_success
        && memory.listed_non_memory_coordinates_are_identical
        && model_memory.full_non_memory_coordinates_identical
        && model_memory.candidate_support_identical
        && model_memory.observed_state_is_well_formed
        && model_memory.intervention_is_off_manifold_scm
        && model_memory.exactly_one_memory_coordinate_replaced
        && model_memory.historical_absorb_record_is_unchanged
        && model_memory.post_intervention_kernel_defined
        && model_memory.every_post_intervention_transition_replays
        && memory_kernel.intervention_holds_non_memory_coordinates_fixed
        && memory_kernel.intervention_holds_candidate_support_fixed
        && memory_kernel.intervened_event_surface_is_defined
        && memory_kernel.intervention_is_not_an_autonomous_transition
        && memory_kernel.intervened_state_is_off_manifold_scm_surgery
        && memory_kernel.exact_hazard_change_matches_frozen_values
        && model_memory.observed_live_bucket_counts == (1, 1)
        && model_memory.intervened_live_bucket_counts == (0, 0)
        && memory.next_action_law_changes
        && memory.expected_work_changes
        && memory.frozen_observed_values_hold
        && memory.frozen_intervened_values_hold;
    let creation_reachability = specimens.iter().all(|specimen| {
        creation_rank_report(specimen.base).finite_rank_certificate_closes_creation
            && attempt_bit_capacity_certificate(usize::from(specimen.base), false).passes()
            && !specimen.trace.receipts.is_empty()
            && specimen.trace.terminal_state.P == Phase::Thermal
            && specimen.trace.terminal_state.F.is_empty()
    });
    let actual_multiplicity = specimens.iter().all(|specimen| {
        let expected = prefix_fixture(specimen.base)
            .expected_shell_counts
            .into_iter()
            .map(u128::from)
            .collect::<Vec<_>>();
        specimen.state.shell_counts == expected
            && specimen.state.count_integrity
            && specimen.state.prefix_law.complete_prefix_realization()
    });
    let unique_state_temperature = specimens.iter().all(|specimen| {
        specimen.state.classification == StateTemperatureClass::UniqueConstant
            && specimen.state.temperature.is_some()
            && specimen.state.common_multiplicity_ratio
                == Some(Ratio::integer(i128::from(specimen.base)))
    });
    let independent_thermal = specimens.iter().all(|specimen| {
        specimen.probe.operational_closure_passes() && specimen.probe.event_replay.passes()
    }) && authoritative_probe_replay;
    let temperature_equality = temperatures_match_before_controls
        && specimens.iter().all(|specimen| {
            specimen.state.temperature.is_some()
                && specimen.probe.ldb_temperature.is_some()
                && specimen.state.temperature == specimen.probe.ldb_temperature
        });
    let construction_ledger_closure = protocol.construction_trace_inverse_exact
        && protocol.construction_ledgers_close
        && specimens.iter().all(|specimen| {
            specimen.trace.every_forward_map_has_exact_local_inverse
                && specimen.trace.full_reverse_lifo_restores_initial
                && specimen.trace.every_working_ledger_closes
                && specimen.trace.every_augmented_ledger_closes
                && specimen.trace.every_complete_ledger_closes
                && specimen
                    .trace
                    .every_forward_energy_difference_matches_ledger
                && specimen
                    .trace
                    .every_forward_resource_difference_matches_ledger
                && specimen
                    .trace
                    .every_state_has_nontruncating_attempt_bit_capacity
        });
    let creation = complete_state
        && creation_reachability
        && actual_multiplicity
        && unique_state_temperature
        && independent_thermal
        && temperature_equality
        && construction_ledger_closure;
    let physical_ledgers = protocol.fresh_inverse_exact
        && protocol.alias_inverse_exact
        && protocol.absorb_inverse_exact
        && protocol.perturb_inverse_exact
        && protocol.construction_trace_inverse_exact
        && protocol.alias_before_fresh_trace_inverse_exact
        && protocol.repair_trace_inverse_exact
        && protocol.construction_ledgers_close
        && protocol.repair_ledgers_close
        && protocol.perturb_complete_energy_closes
        && protocol.perturb_unlocks_no_injected_repair_cells
        && protocol.ledger_leak_is_rejected
        && model_memory.absorb_consumes_one_observation_cell
        && model_memory.absorb_has_exact_inverse
        && model_memory.every_post_intervention_transition_replays
        && protocol.physical_branch_records_are_injective
        && information.passes()
        && all_protocol_transcripts_close
        && specimens.iter().all(|specimen| {
            specimen.trace.every_complete_ledger_closes
                && specimen
                    .trace
                    .every_forward_resource_difference_matches_ledger
                && specimen
                    .trace
                    .every_state_has_nontruncating_attempt_bit_capacity
                && specimen.maintenance.every_repair_inverse_and_ledger_closes
                && specimen.maintenance.every_repair_event_surface_replays
                && specimen.probe.event_replay.passes()
        })
        && equal_contact.contact.event_replay.passes()
        && unequal_contact.contact.event_replay.passes()
        && specimens
            .iter()
            .all(|specimen| specimen.probe_kernel.passes)
        && equal_contact_kernel.passes()
        && unequal_contact_kernel.passes()
        && authoritative_thermal_contact_replay;
    let maintenance = specimens.iter().all(|specimen| {
        let structural = maintenance_report(specimen.base);
        structural.every_leaf_deletion_has_frozen_damaged_counts
            && structural.every_leaf_repair_restores_complete_counts
            && structural.every_deleted_descriptor_is_returned
            && structural.every_damage_breaks_geometric_shell_recursion
            && structural.reserve_covers_worst_repair_ordering
            && structural.same_local_rank_certificate_applies
            && structural.every_repair_hazard_is_positive
            && structural.probability_one_finite_mean_repair_certified
            && structural.perturb_uses_separate_external_cell
            && specimen.maintenance.every_damaged_count_matches
            && specimen.maintenance.every_damaged_temperature_rejected
            && specimen.maintenance.every_repair_restores
            && specimen.maintenance.every_repair_inverse_and_ledger_closes
            && specimen.maintenance.every_repair_event_surface_replays
            && specimen.maintenance.every_repair_uses_existing_reserve
    });
    let contact = equal_contact.passes()
        && unequal_contact.passes()
        && equal_contact.contact.prepared_event_current_a_to_b == Ratio::ZERO
        && equal_contact.contact.prepared_energy_current_into_left == Ratio::ZERO
        && unequal_contact.contact.prepared_event_current_a_to_b == Ratio::integer(2)
        && unequal_contact.contact.prepared_energy_current_into_left == Ratio::integer(4)
        && equal_contact.contact.event_replay.passes()
        && unequal_contact.contact.event_replay.passes()
        && equal_contact_kernel.passes()
        && unequal_contact_kernel.passes()
        && authoritative_contact_replay;
    let controls_pass = controls.iter().all(|outcome| outcome.passed);
    let resources_frozen = ALPHABET_BASES.into_iter().all(|base| {
        let resource = resource_report(base);
        let off_shell = off_shell_report(base);
        let primary_attempt_bits = attempt_bit_capacity_certificate(usize::from(base), false);
        let off_shell_attempt_bits = attempt_bit_capacity_certificate(usize::from(base), true);
        resource.build_capacity_covers_worst_ordering
            && resource.total_internal_capacity_is_build_plus_repair
            && resource.off_shell_capacity_is_primary_plus_two
            && primary_attempt_bits.passes()
            && off_shell_attempt_bits.passes()
            && off_shell.workspace_is_excluded_from_multiplicity
            && off_shell.promoted_core_unchanged
            && off_shell.descriptor_permutation_preserves_classification
    });
    let no_hidden_fit = source_manifest_is_frozen()
        && resources_frozen
        && legacy_ap_no_go_report().strict_legacy_gate_deadlocks
        && ControlId::ALL.len() == controls.len();

    let initial_observations = specimens
        .iter()
        .map(|specimen| {
            format!(
                "b{}:{}",
                specimen.base,
                state_classification_label(specimen.initial.classification)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let state_observations = specimens
        .iter()
        .map(|specimen| {
            format!(
                "b{}:{}:counts={}",
                specimen.base,
                state_classification_label(specimen.state.classification),
                join_counts(&specimen.state.shell_counts)
            )
        })
        .collect::<Vec<_>>()
        .join(";");
    let traffic_observations = specimens
        .iter()
        .map(|specimen| {
            format!(
                "b{}:{}:fibers={},{},{}",
                specimen.base,
                traffic_classification_label(&specimen.probe),
                specimen.probe.fiber_sizes[0],
                specimen.probe.fiber_sizes[1],
                specimen.probe.fiber_sizes[2]
            )
        })
        .collect::<Vec<_>>()
        .join(";");
    let probe_authoritative_rows = specimens
        .iter()
        .map(|specimen| {
            format!(
                "b{}:states={}:events={}:pass={}",
                specimen.base,
                specimen.probe_kernel.complete_coordinate_count,
                specimen.probe_kernel.event_row_count,
                specimen.probe_kernel.passes,
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let passed_controls = controls.iter().filter(|outcome| outcome.passed).count();
    let manifest_frozen = source_manifest_is_frozen();

    let mut gates = vec![
        gate(
            GateId::E01,
            complete_state,
            format!(
                "complete_state={complete_state}; primary_event_replay={primary_event_surfaces_replay}; thermal_contact_authoritative_replay={authoritative_thermal_contact_replay}; observed_reachable_well_formed={}; do_M_off_manifold={} do_M_kernel_defined={} do_M_not_autonomous={}",
                memory_kernel.observed_state_is_reachable_and_well_formed,
                memory_kernel.intervened_state_is_off_manifold_scm_surgery,
                memory_kernel.intervened_event_surface_is_defined,
                memory_kernel.intervention_is_not_an_autonomous_transition,
            ),
        ),
        gate(
            GateId::E02,
            initial_nonthermal,
            format!("initial={initial_observations}"),
        ),
        gate(
            GateId::E03,
            forbidden_non_use,
            format!(
                "poison_full_trace_invariant={}; clean_builder_admitted={}; target_rejected={}; traffic_rejected={}; primary_contamination_demonstrated={}",
                poison.passes(),
                builder_admission.clean_authoritative_state_admitted,
                builder_admission.target_control_passes(),
                builder_admission.traffic_control_passes(),
                builder_admission.demonstrates_primary_contamination(&poison),
            ),
        ),
        gate(
            GateId::E04,
            causal_loop,
            format!("causal_loop={causal_loop}; driven_channels={}", protocol.all_driven_channels_exercised),
        ),
        gate(
            GateId::E05,
            memory_causal,
            format!(
                "do(M): P(F<A) {} -> {}; expected work {} -> {}",
                memory.observed_law.fresh_before_alias_probability,
                memory.intervened_law.fresh_before_alias_probability,
                memory.observed_law.expected_work_cells,
                memory.intervened_law.expected_work_cells,
            ),
        ),
        gate(
            GateId::E06,
            creation,
            format!(
                "complete_state={complete_state}; reachability_rank={creation_reachability}; state_counts={actual_multiplicity}; state_temperature={unique_state_temperature}; probe={independent_thermal}; equality={temperature_equality}; construction_ledgers={construction_ledger_closure}; {state_observations}"
            ),
        ),
        gate(
            GateId::E07,
            actual_multiplicity,
            state_observations.clone(),
        ),
        gate(
            GateId::E08,
            unique_state_temperature,
            state_observations.clone(),
        ),
        gate(
            GateId::E09,
            independent_thermal,
            format!("{traffic_observations}; authoritative_rows={probe_authoritative_rows}"),
        ),
        gate(
            GateId::E10,
            temperature_equality,
            format!("exact_temperature_equality={temperature_equality}"),
        ),
        gate(
            GateId::E11,
            physical_ledgers,
            format!(
                "protocols={} all_round_trips={}; probe_authoritative_rows={}; equal_contact_rows={} unequal_contact_rows={}; information_reports={} all_quotients={} physical_anchor={} I_equals_H={}",
                evidence.protocols.len(),
                all_protocol_transcripts_close,
                probe_authoritative_rows,
                equal_contact_kernel.event_rows.len(),
                unequal_contact_kernel.event_rows.len(),
                information.reports.len(),
                information.every_construction_quotient_reported,
                information.construction_anchor_matches_physical_branch,
                information.every_information_identity_passes,
            ),
        ),
        gate(
            GateId::E12,
            maintenance,
            format!("maintenance_all_leaf_arms={maintenance}"),
        ),
        gate(
            GateId::E13,
            contact,
            format!(
                "equal_event={} equal_energy={} unequal_event={} unequal_energy={} equal_authoritative_rows={} equal_authoritative_pass={} unequal_authoritative_rows={} unequal_authoritative_pass={}",
                equal_contact.contact.prepared_event_current_a_to_b,
                equal_contact.contact.prepared_energy_current_into_left,
                unequal_contact.contact.prepared_event_current_a_to_b,
                unequal_contact.contact.prepared_energy_current_into_left,
                equal_contact_kernel.event_rows.len(),
                equal_contact_kernel.passes(),
                unequal_contact_kernel.event_rows.len(),
                unequal_contact_kernel.passes(),
            ),
        ),
        gate(
            GateId::E14,
            controls_pass,
            format!("controls_passed={passed_controls}/{}", controls.len()),
        ),
        gate(
            GateId::E15,
            no_hidden_fit,
            format!("manifest_content_valid={manifest_frozen}; resource_audit={resources_frozen}"),
        ),
    ];
    let all_gates_pass = gates.iter().all(|outcome| outcome.passed);
    let state_dependent_positive = specimens.iter().all(|specimen| {
        specimen.state.classification == StateTemperatureClass::StateDependent
            && specimen
                .state
                .adjacent_ratios
                .iter()
                .all(|ratio| ratio.is_some_and(|value| value > Ratio::ONE))
    });
    let operational_thermodynamic_graph = actual_multiplicity
        && unique_state_temperature
        && independent_thermal
        && temperature_equality;
    let static_structural_theorem = ALPHABET_BASES.into_iter().all(|base| {
        let recursion = prefix_count_report(base);
        let thermal = verify_static_perfect_tree_control(base, Ratio::ONE);
        recursion.actual_enumeration_matches_prediction
            && recursion.shell_recursion_holds
            && recursion.canonical_words_are_injective
            && recursion.descriptor_encoding_is_injective
            && recursion.unique_parentage_holds
            && recursion.local_port_completeness_holds
            && thermal.state.exact_prefix_temperature_passes()
            && thermal.probe.operational_closure_passes()
            && thermal.state.temperature == thermal.probe.ldb_temperature
    });
    let adaptive_thermodynamic_construction = static_structural_theorem
        && complete_state
        && initial_nonthermal
        && forbidden_non_use
        && causal_loop
        && memory_causal
        && creation
        && operational_thermodynamic_graph
        && physical_ledgers;
    let endogenous_nonadaptive_constructor = static_structural_theorem
        && complete_state
        && initial_nonthermal
        && forbidden_non_use
        && creation
        && operational_thermodynamic_graph
        && !memory_causal;
    let state_dependent_relevant_controls_pass = [
        ControlId::LegacyAPGate,
        ControlId::ExternalThermostat,
        ControlId::GibbsInstalled,
        ControlId::TargetCountBuilder,
        ControlId::TrafficTrainedBuilder,
        ControlId::PoisonInput,
        ControlId::DescriptorPermutation,
        ControlId::MemoryClamp,
        ControlId::MemoryPermutation,
        ControlId::HiddenMemory,
        ControlId::IdealCounters,
        ControlId::LedgerLeak,
        ControlId::ReverseMismatch,
    ]
    .into_iter()
    .all(|control| control_passed(&controls, control));
    let state_dependent_independent_compatibility =
        independent_thermal && temperature_equality && contact;
    let state_dependent_matching_gates = complete_state
        && initial_nonthermal
        && forbidden_non_use
        && causal_loop
        && memory_causal
        && creation_reachability
        && state_dependent_positive
        && specimens
            .iter()
            .all(|specimen| specimen.state.count_integrity)
        && state_dependent_independent_compatibility
        && physical_ledgers
        && maintenance
        && state_dependent_relevant_controls_pass;
    let primary_contamination_demonstrated =
        builder_admission.demonstrates_primary_contamination(&poison);
    let nonmaintenance_controls_pass = controls.iter().all(|outcome| {
        matches!(
            outcome.control,
            ControlId::LeafDeletion | ControlId::RepairDisabled | ControlId::DescriptorWithheld
        ) || outcome.passed
    });
    let classification = if primary_contamination_demonstrated {
        FinalClassification::NotAdmissible
    } else if all_gates_pass && controls_pass {
        FinalClassification::EndogenouslyThermodynamicAdaptiveXypher
    } else if !manifest_frozen {
        FinalClassification::NoResult
    } else if state_dependent_matching_gates && no_hidden_fit {
        FinalClassification::EndogenousStateDependentTemperature
    } else if adaptive_thermodynamic_construction && nonmaintenance_controls_pass && no_hidden_fit {
        FinalClassification::AdaptiveThermodynamicConstruction
    } else if endogenous_nonadaptive_constructor && controls_pass && no_hidden_fit {
        FinalClassification::EndogenousNonadaptiveConstructor
    } else if operational_thermodynamic_graph && controls_pass && no_hidden_fit {
        FinalClassification::OperationalThermodynamicGraph
    } else if static_structural_theorem && controls_pass && no_hidden_fit {
        FinalClassification::StaticStructuralTheorem
    } else {
        FinalClassification::NoResult
    };
    let summaries = specimens.iter().map(specimen_summary).collect();
    gates.shrink_to_fit();
    Ok(ProofReport {
        gates,
        controls,
        specimens: summaries,
        information: information.reports,
        protocols: std::mem::take(&mut evidence.protocols),
        observations: std::mem::take(&mut evidence.observations),
        thermal_observations: std::mem::take(&mut evidence.thermal_observations),
        contact_observations: std::mem::take(&mut evidence.contact_observations),
        classification,
    })
}
