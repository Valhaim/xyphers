use xypher_endogenous_thermodynamics_proof::{
    exact::Ratio,
    sha256::{digest, hex},
    thermal::ExactLog,
    ControlId, ControlOutcome, FinalClassification, GateId, GateOutcome, InformationReport,
    ProofReport, ProtocolTranscript, SpecimenSummary, BOUNDARY_COMMIT, BOUNDARY_SHA256,
};

const EXPECTED_BOUNDARY_COMMIT: &str = "7ee278ed14ade12689540481903e4832f53442d7";
const EXPECTED_BOUNDARY_SHA256: &str =
    "a3aafdc4d29dc3b6c8a2c5bf8deb554745a311b2f3485e1577f3697b1aa14c80";

const README_SOURCE: &str = include_str!("../README.md");
const FUTURUNA_SOURCE: &str = include_str!("../xypher.runa");
const CARGO_SOURCE: &str = include_str!("../Cargo.toml");
const RUST_MODEL_SOURCE: &str = include_str!("../src/model.rs");
const RUST_THERMAL_SOURCE: &str = include_str!("../src/thermal.rs");
const RUST_LIB_SOURCE: &str = include_str!("../src/lib.rs");
const RUST_MAIN_SOURCE: &str = include_str!("../src/main.rs");
const SOURCE_MANIFEST: &str = include_str!("../FROZEN-SOURCE.sha256");
const LOCAL_MODEL_TEST_SOURCE: &str = include_str!("model_local.rs");

const AUTHORITATIVE_PATHS: [&str; 13] = [
    "Cargo.toml",
    "Cargo.lock",
    "README.md",
    "xypher.runa",
    "src/exact.rs",
    "src/model.rs",
    "src/adaptive.rs",
    "src/thermal.rs",
    "src/sha256.rs",
    "src/lib.rs",
    "src/main.rs",
    "tests/source_contract.rs",
    "tests/model_local.rs",
];

fn marked_section<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = source
        .find(start)
        .unwrap_or_else(|| panic!("missing source marker {start}"));
    let body_start = start_index + start.len();
    let end_offset = source[body_start..]
        .find(end)
        .unwrap_or_else(|| panic!("missing source marker {end}"));
    &source[body_start..body_start + end_offset]
}

fn authoritative_sources() -> [(&'static str, &'static [u8]); 13] {
    [
        ("Cargo.toml", include_bytes!("../Cargo.toml")),
        ("Cargo.lock", include_bytes!("../Cargo.lock")),
        ("README.md", include_bytes!("../README.md")),
        ("xypher.runa", include_bytes!("../xypher.runa")),
        ("src/exact.rs", include_bytes!("../src/exact.rs")),
        ("src/model.rs", include_bytes!("../src/model.rs")),
        ("src/adaptive.rs", include_bytes!("../src/adaptive.rs")),
        ("src/thermal.rs", include_bytes!("../src/thermal.rs")),
        ("src/sha256.rs", include_bytes!("../src/sha256.rs")),
        ("src/lib.rs", include_bytes!("../src/lib.rs")),
        ("src/main.rs", include_bytes!("../src/main.rs")),
        (
            "tests/source_contract.rs",
            include_bytes!("source_contract.rs"),
        ),
        ("tests/model_local.rs", include_bytes!("model_local.rs")),
    ]
}

fn parse_source_manifest(source: &str) -> Vec<(&str, &str)> {
    let mut lines = source.lines();
    assert_eq!(
        lines.next(),
        Some("CAL-ENDO-1 authoritative source manifest v1")
    );
    let entries = lines
        .map(|line| {
            let payload = line
                .strip_prefix("sha256:")
                .unwrap_or_else(|| panic!("malformed manifest entry `{line}`"));
            let (digest, path) = payload
                .split_once("  ")
                .unwrap_or_else(|| panic!("manifest entry has no exact path separator `{line}`"));
            assert_eq!(digest.len(), 64, "digest length for `{path}`");
            assert!(
                digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
                "digest for `{path}` is not lowercase hexadecimal"
            );
            assert!(!path.is_empty(), "manifest path is empty");
            assert!(
                !path.contains("  "),
                "manifest path contains a second separator"
            );
            (digest, path)
        })
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), AUTHORITATIVE_PATHS.len());
    entries
}

#[test]
fn boundary_provenance_is_pinned_on_every_public_surface() {
    assert_eq!(BOUNDARY_COMMIT, EXPECTED_BOUNDARY_COMMIT);
    assert_eq!(BOUNDARY_SHA256, EXPECTED_BOUNDARY_SHA256);

    for source in [README_SOURCE, FUTURUNA_SOURCE] {
        assert!(source.contains(EXPECTED_BOUNDARY_COMMIT));
        assert!(source.contains(EXPECTED_BOUNDARY_SHA256));
    }
}

#[test]
fn crate_is_standalone_and_dependency_free() {
    assert!(CARGO_SOURCE.contains("[workspace]"));
    assert!(!CARGO_SOURCE.contains("[dependencies]"));
    assert!(!CARGO_SOURCE.contains("[dev-dependencies]"));
}

#[test]
fn source_manifest_is_exact_ordered_unique_and_content_valid() {
    let entries = parse_source_manifest(SOURCE_MANIFEST);
    let sources = authoritative_sources();
    for (((recorded_digest, recorded_path), expected_path), (source_path, bytes)) in
        entries.into_iter().zip(AUTHORITATIVE_PATHS).zip(sources)
    {
        assert_eq!(recorded_path, expected_path);
        assert_eq!(source_path, expected_path);
        assert_eq!(recorded_digest, hex(&digest(bytes)));
    }
}

#[test]
fn authoritative_e15_parser_checks_actual_manifest_bytes() {
    let parser = marked_section(
        RUST_LIB_SOURCE,
        "fn source_manifest_is_frozen()",
        "\npub fn evaluate()",
    );
    for required in [
        "CAL-ENDO-1 authoritative source manifest v1",
        "const SOURCES: [(&str, &[u8]); 13]",
        "src/sha256.rs",
        "tests/source_contract.rs",
        "tests/model_local.rs",
        "strip_prefix(\"sha256:\")",
        "split_once(\"  \")",
        "sha256::digest(bytes)",
        "lines.next().is_none()",
    ] {
        assert!(
            parser.contains(required),
            "authoritative E15 manifest parser is missing `{required}`"
        );
    }
    assert!(
        RUST_LIB_SOURCE
            .matches("source_manifest_is_frozen()")
            .count()
            >= 2
    );
}

#[test]
fn authoritative_rust_exposes_one_restricted_event_surface() {
    let builder = marked_section(
        RUST_MODEL_SOURCE,
        "// BEGIN AUTHORITATIVE_BUILDER_SURFACE",
        "// END AUTHORITATIVE_BUILDER_SURFACE",
    );
    let builder_view = marked_section(builder, "pub struct BuilderView", "\n}");
    let scheduler_state = marked_section(builder, "pub struct SchedulerState", "\n}");
    let admission = marked_section(
        builder,
        "// BEGIN AUTHORITATIVE_BUILDER_ADMISSION",
        "// END AUTHORITATIVE_BUILDER_ADMISSION",
    );
    let lower_view = builder_view.to_ascii_lowercase();

    for forbidden in [
        "temperature",
        "beta",
        "gibbs",
        "boltzmann",
        "target_count",
        "target_slope",
        "energy_quantum",
        "held_out",
        "traffic",
        "poison",
        "probe",
        "occupied",
        "pub r:",
        "pub x:",
    ] {
        assert!(
            !lower_view.contains(forbidden),
            "forbidden Rust builder input `{forbidden}` entered BuilderView"
        );
    }

    for required in [
        "pub struct BuilderView",
        "pub enum EnabledEvent",
        "pub fn enabled_events",
        "pub struct EventTransition",
        "pub fn event_transition",
        "episode",
        "repair_target",
        "pub attempt_bits: AttemptBitStore",
        "external_perturbation_cell",
        "hazard",
        "successor",
        "ledger",
        "inverse",
        "Absorb",
        "Fresh",
        "Alias",
        "Credit",
        "Promote",
        "Perturb",
        "impl BuilderView<'_>",
        "attempt_bit_capacity_nontruncating_now",
        "enabled_candidate_events",
        "events.extend(self.builder_view().enabled_candidate_events(support))",
        "AttemptBitCapacityCertificate",
        "construction_candidate_identities",
        "candidate_identities",
        "assigned_attempt_bits",
        "repair_slots_are_predeclared_blank_resources",
        "every_admissible_leaf_can_bind_the_same_repair_slots",
        "self.Q.attempt_bits.has_unattempted(candidate)",
    ] {
        assert!(
            builder.contains(required),
            "authoritative event surface is missing `{required}`"
        );
    }
    assert!(!scheduler_state.contains("attempt_bit_capacity"));
    assert!(!builder.contains("attempt_capacity"));
    for required in [
        "pub enum AttemptLane",
        "pub struct AttemptBit",
        "pub lane: AttemptLane",
        "pub identity: Option<CandidateKey>",
        "pub attempted: bool",
        "pub struct AttemptBitStore",
        "pub bits: Vec<AttemptBit>",
        "let repair_attempt_slots = 2;",
        "identity: None",
        "store.bind_repair_episode(1, &port)",
    ] {
        assert!(
            RUST_MODEL_SOURCE.contains(required),
            "physical attempt-bit source is missing `{required}`"
        );
    }
    assert!(
        RUST_MODEL_SOURCE.contains("pub every_state_has_nontruncating_attempt_bit_capacity: bool")
    );
    for required in [
        "pub struct TargetCountRequest",
        "pub struct HeldOutTrafficRequest",
        "pub enum BuilderRequest",
        "pub enum BuilderAdmissionError",
        "pub fn admit_builder_request",
        "BuilderRequest::TargetCount",
        "BuilderRequest::HeldOutTraffic",
        "Err(BuilderAdmissionError::TargetCountInput)",
        "Err(BuilderAdmissionError::HeldOutTrafficInput)",
    ] {
        assert!(
            admission.contains(required),
            "authoritative builder admission is missing `{required}`"
        );
    }
}

#[test]
fn public_classification_contract_uses_exact_outputs_and_no_generic_partial() {
    let outputs = [
        "STATIC STRUCTURAL THEOREM",
        "OPERATIONAL THERMODYNAMIC GRAPH",
        "ENDOGENOUS NONADAPTIVE CONSTRUCTOR",
        "ADAPTIVE THERMODYNAMIC CONSTRUCTION",
        "ENDOGENOUS STATE-DEPENDENT TEMPERATURE",
        "ENDOGENOUSLY THERMODYNAMIC ADAPTIVE XYPHER",
        "NOT ADMISSIBLE",
        "NO_RESULT",
    ];
    for label in outputs {
        assert!(
            RUST_LIB_SOURCE.contains(label),
            "Rust report omits `{label}`"
        );
        assert!(
            FUTURUNA_SOURCE.contains(label),
            "Futuruna surface omits `{label}`"
        );
        assert!(README_SOURCE.contains(label), "README omits `{label}`");
    }
    assert!(!RUST_LIB_SOURCE.contains("PARTIAL"));
    assert!(!FUTURUNA_SOURCE.contains("PARTIAL"));
    for required in [
        "pub enum FinalClassification",
        "pub classification: FinalClassification",
        "self.classification.label()",
    ] {
        assert!(
            RUST_LIB_SOURCE.contains(required),
            "public classification contract is missing `{required}`"
        );
    }
}

#[test]
fn public_report_contract_is_complete_without_evaluating_it() {
    assert_eq!(GateId::ALL.len(), 15);
    assert_eq!(ControlId::ALL.len(), 22);

    let success_method: fn(&ProofReport) -> bool = ProofReport::is_success;
    let render_method: fn(&ProofReport) -> String = ProofReport::render;
    let _public_surface_only = (success_method, render_method);
    assert!(!LOCAL_MODEL_TEST_SOURCE.contains("evaluate("));
}

#[test]
fn authoritative_thermal_surface_uses_one_full_state_event_transition_seam() {
    for required in [
        "pub struct ProbeAuthoritativeState",
        "pub xypher: CompleteState",
        "pub struct ContactAuthoritativeState",
        "pub left: CompleteState",
        "pub right: CompleteState",
        "pub enum AuthoritativeState",
        "Driven(CompleteState)",
        "Probe(ProbeAuthoritativeState)",
        "Contact(ContactAuthoritativeState)",
        "pub enum AuthoritativeEvent",
        "pub struct AuthoritativeTransition",
        "pub source: AuthoritativeState",
        "pub successor: AuthoritativeState",
        "pub fn enabled_authoritative_events",
        "pub fn authoritative_transition",
        "pub struct SealedProbeProtocol",
        "pub body_degeneracies: [u32; 3]",
        "pub body_energy_indices: [u32; 3]",
        "pub total_energy_index: u32",
        "pub struct ContactProtocol",
        "pub fn frozen_contact_protocol",
        "pub struct ProbeHeatLedger",
        "pub struct ContactHeatLedger",
        "pub enum AuthoritativeLedger",
        "pub struct ProbeAuthoritativeEventRow",
        "pub struct ProbeAuthoritativeKernelAudit",
        "pub reference_xypher_state: CompleteState",
        "pub struct ContactAuthoritativeEventRow",
        "pub struct ContactAuthoritativeKernelAudit",
        "pub reference_left_state: CompleteState",
        "pub reference_right_state: CompleteState",
        "pub complete_coordinates:",
        "pub event_rows:",
        "pub every_event_has_exact_reciprocal_and_heat_ledger: bool",
    ] {
        assert!(
            RUST_THERMAL_SOURCE.contains(required),
            "authoritative thermal seam is missing `{required}`"
        );
    }
    assert_eq!(
        RUST_THERMAL_SOURCE
            .matches("pub enum AuthoritativeState")
            .count(),
        1
    );
    assert_eq!(
        RUST_THERMAL_SOURCE
            .matches("pub enum AuthoritativeEvent")
            .count(),
        1
    );
    assert_eq!(
        RUST_THERMAL_SOURCE
            .matches("pub struct AuthoritativeTransition")
            .count(),
        1
    );
}

#[test]
fn report_surface_preserves_exact_evidence_and_guards_claim_labels() {
    for required in [
        "pub record_variable: String",
        "pub record_is_exact_copy: bool",
        "struct EvidenceJournal",
        "Err(detail) => ProofReport::fatal(detail, evidence)",
        "specimens: evidence.specimens",
        "information: evidence.information",
        "protocols: evidence.protocols",
        "observations: evidence.observations",
        "thermal_observations: evidence.thermal_observations",
        "contact_observations: evidence.contact_observations",
        "pub initial_complete_state: String",
        "pub terminal_complete_state: String",
        "source_complete_state={:?} successor_complete_state={:?}",
        "INITIAL_COMPLETE_STATE",
        "TERMINAL_COMPLETE_STATE",
        "OBSERVATION {observation}",
        "THERMAL {observation}",
        "first_operation_record_entry",
        "immutable_first_action_operation_entry",
        "fresh_immediate_semantic_state",
        "alias_immediate_semantic_state",
        "fn demonstrates_primary_contamination",
        "self.target_payload_admitted_to_policy",
        "self.traffic_payload_admitted_to_policy",
        "!poison.construction_trace_identical",
        "!poison.off_shell_trace_identical",
        "!poison.repair_trace_identical",
    ] {
        assert!(
            RUST_LIB_SOURCE.contains(required),
            "public evidence surface is missing `{required}`"
        );
    }
    for required in [
        "semantic_record.state == SemanticState::Pending",
        "semantic_record.state == SemanticState::Failure",
    ] {
        assert!(
            RUST_MODEL_SOURCE.contains(required),
            "immediate causal branch evidence is missing `{required}`"
        );
    }

    let causal_report = marked_section(
        RUST_LIB_SOURCE,
        "let mut reports = vec![information_report(",
        ")];",
    );
    assert!(causal_report.contains("first_operation_record_entry"));
    assert!(causal_report.contains("immutable_first_action_operation_entry"));
    assert!(!causal_report.contains("terminal_semantic_record_state"));

    let claim_logic = marked_section(
        RUST_LIB_SOURCE,
        "let state_dependent_relevant_controls_pass =",
        "let summaries =",
    );
    for required in [
        "let state_dependent_independent_compatibility =",
        "independent_thermal",
        "&& temperature_equality",
        "&& contact;",
        "&& state_dependent_independent_compatibility",
        "let primary_contamination_demonstrated =",
        "builder_admission.demonstrates_primary_contamination(&poison)",
        "let classification = if primary_contamination_demonstrated",
        "FinalClassification::NotAdmissible",
    ] {
        assert!(
            claim_logic.contains(required),
            "claim guard is missing `{required}`"
        );
    }
    assert!(!claim_logic.contains("post_hoc_fit_detected"));
    assert!(
        !RUST_LIB_SOURCE.contains("evidence.protocols.clear()"),
        "late-error journal must not discard already accumulated protocol evidence"
    );
    assert!(!RUST_LIB_SOURCE.contains("evidence.protocols.clone()"));
    for required in [
        "protocols: std::mem::take(&mut evidence.protocols)",
        "observations: std::mem::take(&mut evidence.observations)",
        "thermal_observations: std::mem::take(&mut evidence.thermal_observations)",
        "contact_observations: std::mem::take(&mut evidence.contact_observations)",
        "pub fn write_to<W: Write>",
    ] {
        assert!(RUST_LIB_SOURCE.contains(required));
    }
    assert!(RUST_MAIN_SOURCE.contains("report.write_to(&mut output)?"));
    assert!(!RUST_MAIN_SOURCE.contains(".render()"));

    let maintenance_report = marked_section(
        RUST_LIB_SOURCE,
        "struct MaintenanceModelReport",
        "struct ProbeKernelEvidence",
    );
    assert!(!maintenance_report.contains("protocols:"));

    let creation_predicate =
        marked_section(RUST_LIB_SOURCE, "let creation =", "let physical_ledgers =");
    assert!(
        creation_predicate
            .trim_start()
            .starts_with("complete_state"),
        "E06 creation must directly require the complete-state classifier"
    );
}

#[test]
fn streaming_and_in_memory_renderers_are_byte_identical_for_a_synthetic_report() {
    let half = Ratio::new(1, 2);
    let ln_two = ExactLog::ln_ratio(Ratio::integer(2));
    let report = ProofReport {
        gates: vec![GateOutcome {
            gate: GateId::E01,
            passed: true,
            detail: "synthetic-gate".to_string(),
        }],
        controls: vec![ControlOutcome {
            control: ControlId::StaticPerfectTree,
            passed: true,
            expected: "SYNTHETIC_EXPECTED",
            observed: "synthetic-control".to_string(),
        }],
        specimens: vec![SpecimenSummary {
            base: 2,
            initial_classification: "UNDEFINED".to_string(),
            operations: 1,
            shell_counts: vec![1, 2],
            state_classification: "UNIQUE_CONSTANT".to_string(),
            traffic_classification: "UNIQUE_LDB_EQUILIBRIUM".to_string(),
            state_temperature: "1/(ln(2))".to_string(),
            ldb_temperature: "1/(ln(2))".to_string(),
            probe_fibers: [1, 2, 3],
            maintenance_leaf_arms: 4,
        }],
        information: vec![InformationReport {
            scope: "synthetic-info".to_string(),
            record_variable: "synthetic-record".to_string(),
            branch_evidence: "synthetic-evidence".to_string(),
            outcome_probabilities: [half, half],
            record_probabilities: [half, half],
            joint_distribution: [[half, Ratio::ZERO], [Ratio::ZERO, half]],
            outcome_entropy: ln_two.clone(),
            record_outcome_mutual_information: ln_two,
            record_is_exact_copy: true,
        }],
        protocols: vec![ProtocolTranscript {
            scope: "synthetic-protocol".to_string(),
            initial_complete_state: "synthetic-initial".to_string(),
            terminal_complete_state: "synthetic-terminal".to_string(),
            forward: vec!["synthetic-forward".to_string()],
            inverse: vec!["synthetic-inverse".to_string()],
            round_trip_exact: true,
        }],
        observations: vec!["stage=SYNTHETIC".to_string()],
        thermal_observations: vec!["scope=SYNTHETIC".to_string()],
        contact_observations: vec!["fixture=SYNTHETIC".to_string()],
        classification: FinalClassification::NoResult,
    };
    let expected = concat!(
        "CAL_ENDO_1_EXACT_VERIFIER v1\n",
        "BOUNDARY commit=7ee278ed14ade12689540481903e4832f53442d7 sha256=a3aafdc4d29dc3b6c8a2c5bf8deb554745a311b2f3485e1577f3697b1aa14c80\n",
        "OBSERVATION stage=SYNTHETIC\n",
        "SPECIMEN b=2 initial=UNDEFINED operations=1 counts=1,2 state=UNIQUE_CONSTANT traffic=UNIQUE_LDB_EQUILIBRIUM T_state=1/(ln(2)) T_LDB=1/(ln(2)) fibers=1,2,3 repair_arms=4\n",
        "E01_COMPLETE_STATE PASS synthetic-gate\n",
        "C01_STATIC_PERFECT_TREE PASS expected=SYNTHETIC_EXPECTED observed=synthetic-control\n",
        "INFORMATION scope=synthetic-info record_variable=synthetic-record p=1/2,1/2 record_p=1/2,1/2 joint=1/2,0,0,1/2 H=ln(2) I=ln(2) exact_copy=true evidence=synthetic-evidence\n",
        "PROTOCOL scope=synthetic-protocol round_trip_exact=true\n",
        "INITIAL_COMPLETE_STATE synthetic-initial\n",
        "FORWARD synthetic-forward\n",
        "TERMINAL_COMPLETE_STATE synthetic-terminal\n",
        "INVERSE synthetic-inverse\n",
        "THERMAL scope=SYNTHETIC\n",
        "CONTACT fixture=SYNTHETIC\n",
        "TAU=NOT_INSTANTIATED Xi=NOT_INSTANTIATED\n",
        "CLASSIFICATION NO_RESULT\n",
        "OVERALL FAIL\n",
    );
    let mut actual = Vec::new();
    report.write_to(&mut actual).unwrap();
    assert_eq!(actual, expected.as_bytes());
    assert_eq!(report.render(), expected);
}

#[test]
fn futuruna_builder_surface_has_no_forbidden_observer_input() {
    let builder = marked_section(
        FUTURUNA_SOURCE,
        "-- BEGIN CONSTRUCTION_POLICY",
        "-- END CONSTRUCTION_POLICY",
    );
    let lower = builder.to_ascii_lowercase();

    for forbidden in [
        "temperature",
        "beta",
        "gibbs",
        "boltzmann",
        "target_count",
        "target_slope",
        "energy_quantum",
        "traffic",
        "statecountreport",
        "thermalreport",
        "forbiddensentinel",
        "poisonenvelope",
        "probecoordinate",
        "occupiedreservoirstate",
        "sealedprobeprotocol",
        "contactprotocol",
        "probeheatledger",
        "contactheatledger",
        ".probe",
        ".occupied",
    ] {
        assert!(
            !lower.contains(forbidden),
            "forbidden builder input `{forbidden}` entered the Futuruna construction surface"
        );
    }

    for required in [
        "# BuilderView(",
        "# BuilderQueueView(",
        "# CandidateSlot = UntriedCandidate(proposal: CandidateProposal) | TriedCandidate(identity: CandidateIdentity) | MissingAttemptBit(identity: CandidateIdentity)",
        "# CandidateEnablement = Enabled(proposal: CandidateProposal, cell: WorkCell) | DisabledNoWork(proposal: CandidateProposal) | DisabledNoAttemptBit(proposal: CandidateProposal)",
        "# EnabledEvent(channel: ForwardChannel, hazard: ExactRatio)",
        "# ActionReceipt(event: ForwardChannel, hazard: ExactRatio, operation_record: OperationRecord, ledger: WorkLedger)",
        "# EventTransition(enabled: EnabledEvent, hazard: ExactRatio, channel: ForwardChannel, successor: BuilderView, forward_receipt: ActionReceipt, ledger: WorkLedger, inverse: ActionReceipt, inverse_restores_source: Bool)",
        "# ProtocolStep(index: Int, selected: EnabledEvent, receipt: ActionReceipt)",
        "# ProtocolTrace(forward_steps: List(ProtocolStep), reverse_lifo_receipts: List(ActionReceipt), full_reverse_lifo_restores_initial: Bool)",
        "> crystal_readback(",
        "> memory_projection(",
        "> one_port_candidates(",
        "> matching_attempt_bit(",
        "> enable_with_work(",
        "> fresh_operation(",
        "> alias_operation(",
        "> credit_operation(",
        "> promote_operation(",
        "> perturb_operation(",
        "> absorb_operation(",
        "> enabled_event(",
        "> enabled_events(",
        "> event_successor(",
        "> event_ledger(",
        "> event_inverse(",
        "> perturb_work_ledger(",
        "Unfresh",
        "Unfail",
        "Uncredit",
        "Demote",
        "Unperturb",
        "Unabsorb",
        "matching_attempt_bit(attempt_bits.bits, proposal.identity, lane, false)",
        "enable_from_cells(proposal, view.work.build, view.queue.attempt_bits, BuildAttemptBit)",
        "enable_from_cells(proposal, view.work.repair, view.queue.attempt_bits, RepairAttemptBit)",
        "# AttemptBitCapacityCertificate(base: Int, off_shell_inventory: Bool, descriptor_capacity: Int, construction_candidate_identities: Int, repair_attempt_slots: Int, declared_attempt_bits: Int, semantic_record_capacity: Int, candidate_identities: List(CandidateIdentity), assigned_attempt_bits: List(AttemptBitAssignment), every_candidate_identity_has_distinct_bit: Bool, repair_slots_are_predeclared_blank_resources: Bool, every_admissible_leaf_can_bind_the_same_repair_slots: Bool, capacity_cannot_truncate_construction: Bool)",
        "AttemptBitCapacityCertificate(base, off_shell_inventory, descriptor_capacity, length(candidate_identities), 2,",
    ] {
        assert!(
            builder.contains(required),
            "missing construction declaration `{required}`"
        );
    }

    for forbidden in [
        "# EnabledEvent(channel: ForwardChannel, hazard: ExactRatio, successor:",
        "# ProtocolStep(index: Int, event: EnabledEvent)",
        "reverse: List(ProtocolStep)",
        "attempts: List(AttemptRecord)",
        "> identity_was_attempted(",
        "available_attempt_bits",
    ] {
        assert!(
            !builder.contains(forbidden),
            "stale construction declaration `{forbidden}` remains in the Futuruna construction surface"
        );
    }

    for required in [
        "# CompleteState(",
        "# ConfigurationSpace(",
        "# TypedWorkStore(",
        "# MemoryLog(",
        "# OperationBank(",
        "# CandidateIdentity(episode:",
        "# AttemptLane = BuildAttemptBit | RepairAttemptBit",
        "# OptionalCandidateIdentity = NoCandidateIdentity | SomeCandidateIdentity(value: CandidateIdentity)",
        "# AttemptBit(id: Int, lane: AttemptLane, identity: OptionalCandidateIdentity, attempted: Bool)",
        "# AttemptBitStore(bits: List(AttemptBit))",
        "# SchedulerQueue(",
        "# SchedulerCapacities(payload_capacity: Int)",
        "# ExternalPerturbationCell",
        "# WorkingBoundary(",
        "# AugmentedBoundary(",
        "# CompleteBoundary(",
        "> structural_incidence(",
        "> terminal_bucket_counts(",
        "> predictive_mean(",
        "> integrated_phi(",
        "> absorbed_memory_prediction(",
        "> intervened_memory_prediction(",
        "> frozen_leaf_defect(",
    ] {
        assert!(
            FUTURUNA_SOURCE.contains(required),
            "missing state or exact-policy declaration `{required}`"
        );
    }

    let scheduler_queue = FUTURUNA_SOURCE
        .lines()
        .find(|line| line.starts_with("# SchedulerQueue("))
        .expect("missing Futuruna SchedulerQueue declaration");
    let builder_queue = FUTURUNA_SOURCE
        .lines()
        .find(|line| line.starts_with("# BuilderQueueView("))
        .expect("missing Futuruna BuilderQueueView declaration");
    for queue in [scheduler_queue, builder_queue] {
        assert!(queue.contains("attempt_bits: AttemptBitStore"));
        assert!(!queue.contains("attempt_bit_capacity"));
        assert!(!queue.contains("attempts: List(AttemptRecord)"));
    }
    assert!(!FUTURUNA_SOURCE.contains("# AttemptRecord("));
    assert!(!FUTURUNA_SOURCE.contains("attempt_bit_capacity: Int"));

    let admission = marked_section(
        FUTURUNA_SOURCE,
        "-- BEGIN BUILDER_ADMISSION",
        "-- END BUILDER_ADMISSION",
    );
    for required in [
        "# TargetCountRequest(",
        "# HeldOutTrafficRequest(",
        "# BuilderRequest = LocalBuilderRequest",
        "# BuilderAdmissionError = TargetCountInput | HeldOutTrafficInput",
        "> admit_builder_request(",
        "RejectedBuilder(TargetCountInput)",
        "RejectedBuilder(HeldOutTrafficInput)",
    ] {
        assert!(
            admission.contains(required),
            "missing Futuruna builder admission declaration `{required}`"
        );
    }

    let intervention = marked_section(
        FUTURUNA_SOURCE,
        "-- BEGIN SCM_MEMORY_INTERVENTION",
        "-- END SCM_MEMORY_INTERVENTION",
    );
    for required in [
        "# ScmMemoryIntervention(",
        "# PostInterventionKernel(",
        "OffManifoldScmState",
        "PreserveHistoricalAbsorbRecord",
        "> scm_memory_intervention(",
        "> post_intervention_kernel(",
    ] {
        assert!(
            intervention.contains(required),
            "missing Futuruna SCM declaration `{required}`"
        );
    }
    for required in [
        "pub struct ScmMemoryIntervention",
        "pub struct PostInterventionKernelAudit",
        "pub fn scm_memory_intervention",
        "pub fn post_intervention_kernel",
        "intervention_is_off_manifold_scm",
        "every_post_intervention_transition_replays",
    ] {
        assert!(
            RUST_MODEL_SOURCE.contains(required),
            "missing Rust SCM audit surface `{required}`"
        );
    }
}

#[test]
fn futuruna_surface_is_prospective_exact_and_observer_separated() {
    assert!(!FUTURUNA_SOURCE.contains("Float"));
    assert!(!FUTURUNA_SOURCE.contains("f32"));
    assert!(!FUTURUNA_SOURCE.contains("f64"));
    assert!(!FUTURUNA_SOURCE.contains("OVERALL PASS"));
    assert!(!FUTURUNA_SOURCE.contains("E01 PASS"));

    let observer = marked_section(
        FUTURUNA_SOURCE,
        "-- BEGIN POST_CONSTRUCTION_OBSERVER",
        "-- END POST_CONSTRUCTION_OBSERVER",
    );
    for required in [
        "# ShellObservation(",
        "# StateTemperature",
        "# StateCountReport(",
        "# ThermalMicrostate(",
        "# ThermalEdge(",
        "# LabelledTraffic(",
        "# ThermalReport(",
        "# ProbeExchangeCoordinate(",
        "# SealedProbeProtocol(body_degeneracies: List(Int), body_energy_indices: List(Int), total_energy_index: Int, microscopic_clock: ExactRatio, deleted_directed_edge: OptionalProbeEdge)",
        "# ProbeAuthoritativeState(",
        "# ContactCoordinate(",
        "# ContactProtocol(",
        "# ContactAuthoritativeState(",
        "# AuthoritativeState = DrivenAuthoritativeState",
        "# AuthoritativeEvent = DrivenAuthoritativeEvent",
        "# ProbeHeatLedger(",
        "# ContactHeatLedger(",
        "# AuthoritativeLedger = DrivenAuthoritativeLedger",
        "# AuthoritativeInverse = DrivenAuthoritativeInverse",
        "# AuthoritativeTransition(",
        "# ProbeAuthoritativeEventRow(",
        "# ProbeAuthoritativeKernelAudit(",
        "# ContactAuthoritativeEventRow(",
        "# ContactAuthoritativeKernelAudit(",
        "# InformationScope",
        "# OutcomeRecordBranch(",
        "# OutcomeCopyInformation(",
        "# CausalRecordVariable = ImmutableFirstOperationRecordEntry",
        "# CausalMYDefinition(",
        "# InformationReport(scope: InformationScope, record_variable: String, branch_evidence: String",
        "> sealed_probe_protocol(",
        "SealedProbeProtocol([1, 2, 1], [0, 1, 2], 3, microscopic_clock, NoDeletedProbeEdge)",
        "> frozen_contact_protocol(",
        "> probe_heat_ledger_closes(",
        "> contact_heat_ledger_closes(",
        "> outcome_copy_information(",
        "> causal_m_y_definition(",
        "> information_report(",
        "> branch_record_is_copy(",
        "> every_branch_record_is_copy(",
        "# ContactFixture(",
        "# ContactGenerator(",
        "> equal_contact_prediction(",
        "> unequal_contact_prediction(",
    ] {
        assert!(
            observer.contains(required),
            "missing observer declaration `{required}`"
        );
    }

    for required in [
        "# ForwardProtocolEvidence(source_complete_state: CompleteState, successor_complete_state: CompleteState, receipt: ActionReceipt)",
        "# InverseProtocolEvidence(source_complete_state: CompleteState, successor_complete_state: CompleteState, receipt: ActionReceipt)",
        "# ProtocolTranscript(scope: String, initial_complete_state: CompleteState, terminal_complete_state: CompleteState",
        "# EvidenceJournal(controls: List(String), specimens: List(String), information: List(InformationReport), protocols: List(ProtocolTranscript), observations: List(String), thermal_observations: List(String), contact_observations: List(String))",
        "# EvaluationResult = CompletedEvidence(evidence: EvidenceJournal) | LateErrorWithPreservedEvidence",
        "> preserve_late_error(",
        "# PrimaryContaminationEvidence(",
        "> demonstrated_primary_contamination(",
        "> may_emit_not_admissible(",
        "# StateDependentMatchingEvidence(",
        "> state_dependent_label_is_admissible(",
        "independent_probe_operation_matches_state_slopes",
        "independent_contact_operation_matches_state_slopes",
        "IMMEDIATE_EVIDENCE_ONLY_NOT_M_Y",
    ] {
        assert!(
            FUTURUNA_SOURCE.contains(required),
            "missing prospective evidence contract `{required}`"
        );
    }
    assert!(!FUTURUNA_SOURCE.contains("# SealedExchangeRule("));
    assert!(!FUTURUNA_SOURCE.contains("# InformationReport(quotients:"));

    assert!(FUTURUNA_SOURCE.contains("NOT_INSTANTIATED"));
    assert!(FUTURUNA_SOURCE.contains("# ObservationStage = RecordInitialState"));
    assert!(FUTURUNA_SOURCE.contains("> observation_plan()"));
    assert!(FUTURUNA_SOURCE.contains("# ClaimLabel = StaticStructuralTheorem"));
    assert!(FUTURUNA_SOURCE.contains("> claim_label_text("));
    assert!(FUTURUNA_SOURCE.contains("# GateId = E01"));
    assert!(FUTURUNA_SOURCE.contains(
        "GatePlan(E01, E02, E03, E04, E05, E06, E07, E08, E09, E10, E11, E12, E13, E14, E15)"
    ));
    assert!(FUTURUNA_SOURCE.contains("# ControlId = StaticPerfectTree"));
    for control in [
        "StaticPerfectTree",
        "NonadaptiveBuilder",
        "LegacyAPGate",
        "ExternalThermostat",
        "GibbsInstalled",
        "TargetCountBuilder",
        "TrafficTrainedBuilder",
        "PoisonInput",
        "OffShellInventory",
        "DescriptorPermutation",
        "EnergyShuffle",
        "LeafDeletion",
        "RepairDisabled",
        "DescriptorWithheld",
        "PortLawMutation",
        "MemoryClamp",
        "MemoryPermutation",
        "HiddenMemory",
        "IdealCounters",
        "LedgerLeak",
        "ActivityOnly",
        "ReverseMismatch",
    ] {
        assert!(
            FUTURUNA_SOURCE.contains(control),
            "missing frozen control `{control}`"
        );
    }
}

#[test]
fn authoritative_rust_sources_name_no_inexact_or_stochastic_acceptance_path() {
    let sources = [
        include_str!("../src/exact.rs"),
        include_str!("../src/model.rs"),
        include_str!("../src/adaptive.rs"),
        include_str!("../src/thermal.rs"),
        include_str!("../src/sha256.rs"),
        include_str!("../src/lib.rs"),
        include_str!("../src/main.rs"),
    ];

    for source in sources {
        assert!(!source.contains("f32"));
        assert!(!source.contains("f64"));
        assert!(!source.contains("thread_rng"));
        assert!(!source.contains("StdRng"));
        assert!(!source.contains("rand::"));
        assert!(!source.contains("approx::"));
    }
}

#[test]
fn readme_keeps_the_confirmatory_execution_embargo_explicit() {
    for required in [
        "Do **not** execute",
        "independent reviewer",
        "digest covering every authoritative apparatus file",
        "committed and pushed",
        "declared frozen before the first",
        "including failures",
        "src/sha256.rs",
        "CAL-ENDO-1 authoritative source manifest v1",
        "sha256:<64 lowercase hexadecimal digits>  <path>",
        "complete forward and",
        "every finite construction quotient",
        "one `AuthoritativeState` / `AuthoritativeEvent` /",
        "probe protocol itself seals body degeneracies",
        "An `EvidenceJournal` retains",
        "random variable `M_Y` is the immutable entry",
        "independent two-Xypher contact operation",
        "demonstrated primary contamination",
        "cannot emit `NOT ADMISSIBLE`",
    ] {
        assert!(
            README_SOURCE.contains(required),
            "missing embargo clause `{required}`"
        );
    }
}
