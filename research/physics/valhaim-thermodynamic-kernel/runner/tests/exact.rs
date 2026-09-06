use valhaim_thermodynamic_kernel_runner::{
    derive_contact, derive_controls, derive_kernel_exact, validate_artifact_semantics, GateId,
    Provenance, Toolchain, FREEZE_COMMIT, FREEZE_MANIFEST_SHA256, PROTOCOL_SHA256,
};

fn provenance() -> Provenance {
    Provenance {
        freeze_commit: FREEZE_COMMIT.to_owned(),
        runner_seal_commit: "1".repeat(40),
        protocol_sha256: PROTOCOL_SHA256.to_owned(),
        freeze_manifest_sha256: FREEZE_MANIFEST_SHA256.to_owned(),
        schema_sha256: "2".repeat(64),
        relevant_source_clean: true,
        relevant_source_paths: vec!["sealed-source".to_owned()],
        relevant_source_sha256: "3".repeat(64),
        executable_sha256: "4".repeat(64),
        command_argv: vec!["test-runner".to_owned()],
        toolchain: Toolchain {
            rustc: "test-rustc".to_owned(),
            cargo: "test-cargo".to_owned(),
            target: "test-target".to_owned(),
            cargo_lock_sha256: "5".repeat(64),
        },
    }
}

#[test]
fn exact_fixtures_are_derived_from_the_raw_law() {
    let source = provenance();
    let controls = derive_controls(source.clone()).expect("control derivation");
    let exact = derive_kernel_exact(source, &controls).expect("exact derivation");
    assert_eq!(exact.status, "pass");
    assert_eq!(
        exact
            .fixtures
            .iter()
            .map(|fixture| fixture.state_count)
            .collect::<Vec<_>>(),
        vec![9, 16, 49]
    );
    assert_eq!(
        exact
            .fixtures
            .iter()
            .map(|fixture| fixture.rows.len())
            .collect::<Vec<_>>(),
        vec![36, 96, 392]
    );
    assert_eq!(
        exact
            .fixtures
            .iter()
            .map(|fixture| fixture.crystal_readout.outcome_count)
            .collect::<Vec<_>>(),
        vec![13, 22, 77]
    );
    let primary = &exact.fixtures[0];
    assert_eq!(primary.fiber_counts, vec![4, 4, 1]);
    assert_eq!(
        primary
            .generator_eigenvalues_over_activity
            .as_ref()
            .expect("primary generator eigenvalues")
            .iter()
            .map(|value| value.text())
            .collect::<Vec<_>>(),
        vec!["0/1", "-3/1", "-6/1"]
    );
    assert_eq!(
        primary
            .discrete_eigenvalues
            .as_ref()
            .expect("primary discrete eigenvalues")
            .iter()
            .map(|value| value.text())
            .collect::<Vec<_>>(),
        vec!["1/1", "1/4", "-1/2"]
    );
}

#[test]
fn every_frozen_mutation_is_discriminated_at_its_first_gate() {
    let controls = derive_controls(provenance()).expect("control derivation");
    assert_eq!(controls.status, "pass");
    assert!(controls
        .mutations
        .iter()
        .all(|mutation| mutation.verdict == "matched"));
}

#[test]
fn contact_closes_the_claim_ladder_and_detects_temperature_direction() {
    let source = provenance();
    let controls = derive_controls(source.clone()).expect("control derivation");
    let exact = derive_kernel_exact(source.clone(), &controls).expect("exact derivation");
    let contact = derive_contact(source, &exact).expect("contact derivation");
    assert_eq!(contact.status, "pass");
    assert_eq!(
        contact.cases[0]
            .preparation
            .iter()
            .map(|v| v.text())
            .collect::<Vec<_>>(),
        vec!["1/6", "2/3", "1/6"]
    );
    assert_eq!(
        contact.cases[2]
            .preparation
            .iter()
            .map(|v| v.text())
            .collect::<Vec<_>>(),
        vec!["1/13", "8/13", "4/13"]
    );
    assert_eq!(
        contact.cases[2].current_into_left.coefficient.text(),
        "-12/13"
    );
    assert_eq!(
        contact.cases[3].current_into_left.coefficient.text(),
        "12/13"
    );
    assert_eq!(contact.cases[1].right_body, "C");
    assert_eq!(contact.clock_evidence.dormant_rows.len(), 24);
    assert_eq!(contact.clock_evidence.open_rows.len(), 24);
    assert!(contact.clock_evidence.clock_unchanged);
}

#[test]
fn semantic_contract_accepts_the_complete_evidence_set() {
    let source = provenance();
    let controls = derive_controls(source.clone()).expect("control derivation");
    let exact = derive_kernel_exact(source.clone(), &controls).expect("exact derivation");
    let contact = derive_contact(source, &exact).expect("contact derivation");
    validate_artifact_semantics(&exact, &controls, &contact).expect("semantic contract");
}

#[test]
fn contact_cannot_rescue_a_failed_kernel_gate() {
    let source = provenance();
    let controls = derive_controls(source.clone()).expect("control derivation");
    let mut exact = derive_kernel_exact(source.clone(), &controls).expect("exact derivation");
    exact.status = "fail";
    exact.first_failed_gate = Some(GateId::K02);
    exact.gates[1].status = "fail";
    let contact = derive_contact(source, &exact).expect("contact derivation");
    assert_eq!(contact.status, "fail");
    assert_eq!(contact.first_failed_gate, Some(GateId::K02));
    assert!(contact.summary.earned_claim_codes.is_empty());
}

#[test]
fn a_surviving_frozen_mutation_suppresses_kernel_and_contact_claims() {
    let source = provenance();
    let mut controls = derive_controls(source.clone()).expect("control derivation");
    controls.status = "fail";
    controls.mutations[0].observed_first_failure = None;
    controls.mutations[0].verdict = "survived";
    controls.summary.earned_claim_codes.clear();
    let exact = derive_kernel_exact(source.clone(), &controls).expect("exact derivation");
    assert_eq!(exact.status, "invalidated");
    assert!(exact.summary.earned_claim_codes.is_empty());
    let contact = derive_contact(source, &exact).expect("contact derivation");
    assert_eq!(contact.status, "invalidated");
    assert!(contact.summary.earned_claim_codes.is_empty());
}
