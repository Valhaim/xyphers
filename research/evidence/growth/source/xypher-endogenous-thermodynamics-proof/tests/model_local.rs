use xypher_endogenous_thermodynamics_proof::{
    exact::Ratio,
    model::{
        admit_builder_request, attempt_bit_capacity_certificate, causal_memory_fixture,
        construction_quotient_maximum_prior_alias_failures, construction_quotient_physical_audit,
        descriptor_order_audit, instantiate_alias_before_fresh_support_trace,
        instantiate_one_support_trace, memory_intervention_audit, physical_branch_record_audit,
        primary_fixture, ActionError, AttemptLane, BuilderAdmissionError, BuilderRequest,
        CandidateKind, DrivenChannel, EnabledEvent, HeldOutTrafficRequest, KernelSupport,
        ModelEvent, OperationEntry, Phase, SemanticState, TargetCountRequest,
    },
};

#[test]
fn fresh_and_alias_maps_have_literal_local_inverses() {
    let initial = primary_fixture(2, false);
    for kind in [CandidateKind::Fresh, CandidateKind::Alias] {
        let mut state = initial.clone();
        let candidate = state
            .enabled_candidates()
            .into_iter()
            .find(|candidate| candidate.kind == kind)
            .expect("local candidate exists");
        let expected_hazard = state
            .candidate_hazard(&candidate)
            .expect("enabled candidate has an exact hazard");
        let before_resources = state.resource_snapshot();
        let before_working = state.working_energy();
        let before_augmented = state.augmented_internal_energy();
        let before_complete = state.augmented_energy_with_external_perturbation_cell();
        let receipt = state
            .apply_event(&ModelEvent::Candidate(candidate.clone()))
            .expect("candidate event is enabled");
        assert_eq!(receipt.hazard, expected_hazard);
        assert!(receipt.ledger.working_first_law_closes());
        assert!(receipt.ledger.internal_augmented_first_law_closes());
        assert!(receipt.ledger.complete_first_law_closes());
        assert_eq!(
            state.resource_snapshot().difference_from(&before_resources),
            receipt.ledger.resources
        );
        assert_eq!(
            state.working_energy() as i128 - before_working as i128,
            receipt.ledger.delta_working
        );
        assert_eq!(
            state.augmented_internal_energy() as i128 - before_augmented as i128,
            receipt.ledger.delta_augmented
        );
        assert_eq!(
            state.augmented_energy_with_external_perturbation_cell() as i128
                - before_complete as i128,
            receipt.ledger.delta_complete
        );
        let inverse = state.reverse_last().expect("named inverse is enabled");
        assert_eq!(inverse.event, receipt.event);
        assert_eq!(inverse.hazard, receipt.hazard);
        assert!(inverse.ledger.working_first_law_closes());
        assert!(inverse.ledger.internal_augmented_first_law_closes());
        assert!(inverse.ledger.complete_first_law_closes());
        assert_eq!(state, initial);
    }
}

#[test]
fn absorb_precedes_candidates_and_changes_the_exact_hazards() {
    let initial = causal_memory_fixture();
    assert_eq!(initial.M.records.len(), 3);
    assert!(initial.enabled_candidates().is_empty());
    assert_eq!(
        initial.enabled_events(),
        vec![EnabledEvent::Absorb { hazard: Ratio::ONE }]
    );
    let transition = initial
        .event_transition(&initial.enabled_events()[0], KernelSupport::PRIMARY)
        .expect("ABSORB has a complete transition surface");
    assert_eq!(transition.channel, DrivenChannel::Absorb);
    assert!(transition.inverse_restores_source);

    let graph = initial.Gamma.clone();
    let mut absorbed = transition.successor;
    assert_eq!(absorbed.B_W.B_obs.charged_count(), 0);
    assert!(absorbed
        .M
        .records
        .iter()
        .any(|record| record.candidate.is_none() && record.state == SemanticState::Success));
    let candidate_events = absorbed
        .enabled_events()
        .into_iter()
        .filter(|event| matches!(event, EnabledEvent::Candidate { .. }))
        .collect::<Vec<_>>();
    assert_eq!(candidate_events.len(), 2);
    let fresh = candidate_events
        .iter()
        .find(|event| {
            matches!(
                event,
                EnabledEvent::Candidate { candidate, .. }
                    if candidate.kind == CandidateKind::Fresh
            )
        })
        .expect("FRESH is enabled after ABSORB");
    let alias = candidate_events
        .iter()
        .find(|event| {
            matches!(
                event,
                EnabledEvent::Candidate { candidate, .. }
                    if candidate.kind == CandidateKind::Alias
            )
        })
        .expect("ALIAS is enabled after ABSORB");
    assert_eq!(fresh.hazard(), Ratio::new(175, 216));
    assert_eq!(alias.hazard(), Ratio::new(45, 64));
    assert_eq!(absorbed.Gamma, graph);

    let branch = physical_branch_record_audit(&absorbed)
        .expect("both first-event branches leave physical records");
    assert_eq!(branch.fresh.probability, Ratio::new(280, 523));
    assert_eq!(branch.alias.probability, Ratio::new(243, 523));
    assert!(branch.records_injectively_identify_first_event);

    absorbed.reverse_last().expect("UNABSORB is enabled");
    assert_eq!(absorbed, initial);
}

#[test]
fn every_construction_information_quotient_is_a_reachable_physical_branch_pair() {
    for base in 2..=4 {
        for frontier_depth in 1..=3 {
            let maximum_alias_failures =
                construction_quotient_maximum_prior_alias_failures(base, frontier_depth);
            for prior_alias_failures in 0..=maximum_alias_failures {
                let audit = construction_quotient_physical_audit(
                    base,
                    frontier_depth,
                    prior_alias_failures,
                )
                .expect("the frozen quotient has one reachable representative state");
                assert_eq!(audit.representative.base, base);
                assert_eq!(audit.representative.frontier_depth, frontier_depth);
                assert_eq!(
                    audit.representative.requested_prior_alias_failures,
                    prior_alias_failures
                );
                assert!(audit.representative_is_reachable_quotient);
                assert!(
                    audit
                        .representative
                        .every_state_has_nontruncating_attempt_bit_capacity
                );
                assert!(audit.branch_hazards_match_representative);
                assert!(audit.branch_probabilities_are_normalized);
                assert!(audit.records_injectively_copy_first_outcome);
                assert!(audit.both_branches_complete_required_record_protocol);
                assert_eq!(
                    audit.fresh_first.immediate_fresh_state,
                    SemanticState::Pending
                );
                assert_eq!(
                    audit.fresh_first.terminal_fresh_state,
                    SemanticState::Success
                );
                assert!(audit.fresh_first.eventual_credit_updates_same_fresh_record);
                assert_eq!(
                    audit.alias_first.immediate_alias_state,
                    SemanticState::Failure
                );
                assert_eq!(
                    audit.alias_first.terminal_alias_state,
                    SemanticState::Failure
                );
                assert!(audit.alias_first.alias_then_fresh_is_forced);
                assert_eq!(
                    audit.alias_first.immediate_fresh_state,
                    SemanticState::Pending
                );
                assert_eq!(
                    audit.alias_first.terminal_fresh_state,
                    SemanticState::Success
                );
                assert!(audit.alias_first.eventual_credit_updates_same_fresh_record);
                assert!(audit.alias_first.alias_record_stays_failure_through_credit);
            }
        }
    }
}

#[test]
fn builder_view_contains_only_local_construction_state() {
    let state = primary_fixture(2, false);
    let view = state.builder_view();
    assert_eq!(view.Gamma, &state.Gamma);
    assert_eq!(view.A, state.A.as_slice());
    assert_eq!(view.F, &state.F);
    assert_eq!(view.B_W, &state.B_W);
    assert_eq!(view.M, &state.M);
    assert_eq!(view.O, &state.O);
    assert_eq!(view.P, &state.P);
    assert_eq!(view.Q, &state.Q);
}

#[test]
fn builder_admission_rejects_concrete_c06_and_c07_requests_before_policy() {
    let state = primary_fixture(2, false);
    let admitted = admit_builder_request(BuilderRequest::Local(&state))
        .expect("the local authoritative state is admitted");
    assert_eq!(admitted.enabled_candidates(), state.enabled_candidates());

    let target = TargetCountRequest {
        shell_counts: vec![1, 2, 4, 8],
        target_multiplicity_ratios: vec![Ratio::integer(2); 3],
        global_deficit: vec![0, 2, 4, 8],
    };
    assert!(matches!(
        admit_builder_request(BuilderRequest::TargetCount(&state, &target)),
        Err(BuilderAdmissionError::TargetCountInput)
    ));

    let traffic = HeldOutTrafficRequest {
        labelled_rate_pairs: vec![(Ratio::new(2, 3), Ratio::new(1, 3))],
        agreement_reward: Ratio::ONE,
    };
    assert!(matches!(
        admit_builder_request(BuilderRequest::HeldOutTraffic(&state, &traffic)),
        Err(BuilderAdmissionError::HeldOutTrafficInput)
    ));
}

#[test]
fn attempt_bits_are_injective_certificates_not_stopping_parameters() {
    for base in 2..=4 {
        for off_shell_inventory in [false, true] {
            let certificate = attempt_bit_capacity_certificate(base, off_shell_inventory);
            let state = primary_fixture(base, off_shell_inventory);
            assert!(certificate.passes());
            assert_eq!(
                certificate.candidate_identities.len(),
                certificate.construction_candidate_identities
            );
            assert_eq!(
                certificate.assigned_attempt_bits.len(),
                certificate.candidate_identities.len()
            );
            assert_eq!(
                state.Q.attempt_bits.declared_count(),
                certificate.declared_attempt_bits
            );
            assert_eq!(
                state
                    .Q
                    .attempt_bits
                    .bits
                    .iter()
                    .filter(|bit| bit.lane == AttemptLane::Repair && bit.identity.is_none())
                    .count(),
                certificate.repair_attempt_slots,
            );
            assert!(certificate.every_admissible_leaf_can_bind_the_same_repair_slots);
            assert!(state.attempt_bit_capacity_nontruncating_now());
        }
    }
}

#[test]
fn scm_memory_surgery_has_a_defined_replayable_post_intervention_kernel() {
    let audit = memory_intervention_audit().expect("the declared do(M) audit is defined");
    assert!(audit.observed_state_is_well_formed);
    assert!(audit.intervention_is_off_manifold_scm);
    assert!(audit.exactly_one_memory_coordinate_replaced);
    assert!(audit.historical_absorb_record_is_unchanged);
    assert!(audit.full_non_memory_coordinates_identical);
    assert!(audit.post_intervention_kernel_defined);
    assert!(audit.every_post_intervention_transition_replays);
    assert!(!audit
        .intervention
        .intervened_state
        .complete_state_is_well_formed());
    assert_eq!(
        audit.post_intervention_kernel.enabled_events.len(),
        audit.post_intervention_kernel.transitions.len()
    );
}

#[test]
fn promote_rejects_a_complete_frontier_until_every_fresh_record_is_credited() {
    let mut state = primary_fixture(2, false);
    while matches!(&state.P, Phase::Build { depth: 1 }) {
        let candidate = state
            .enabled_candidates()
            .into_iter()
            .find(|candidate| candidate.kind == CandidateKind::Fresh)
            .expect("the next depth-one FRESH candidate exists");
        state.fresh(candidate).expect("depth-one FRESH succeeds");
    }
    assert_eq!(
        state.P,
        Phase::Credit {
            depth: 1,
            maintenance: false
        }
    );
    assert!(
        state
            .M
            .records
            .iter()
            .filter(|record| record.state == SemanticState::Pending)
            .count()
            > 0
    );
    state.P = Phase::Promote {
        depth: 1,
        maintenance: false,
    };
    assert_eq!(state.promote(), Err(ActionError::PromotionCreditIncomplete));
}

#[test]
fn inverse_preflight_failure_is_atomic() {
    let mut state = primary_fixture(2, false);
    let candidate = state
        .enabled_candidates()
        .into_iter()
        .find(|candidate| candidate.kind == CandidateKind::Fresh)
        .expect("FRESH candidate exists");
    state.fresh(candidate).expect("FRESH succeeds");
    let semantic_record = state
        .M
        .records
        .iter()
        .find(|record| record.state == SemanticState::Pending)
        .map(|record| record.id)
        .expect("FRESH wrote a pending record");
    state.M.records[semantic_record].state = SemanticState::Success;
    let corrupted = state.clone();
    assert_eq!(state.reverse_last(), Err(ActionError::InverseStateMismatch));
    assert_eq!(state, corrupted);
}

#[test]
fn full_construction_traces_are_forward_explicit_and_lifo_reversible() {
    let fresh_first = instantiate_one_support_trace(primary_fixture(2, false))
        .expect("FRESH-first construction trace exists");
    let alias_first = instantiate_alias_before_fresh_support_trace(primary_fixture(2, false))
        .expect("ALIAS-before-FRESH construction trace exists");
    for trace in [&fresh_first, &alias_first] {
        assert_eq!(trace.forward_steps.len(), trace.receipts.len());
        assert_eq!(trace.reverse_lifo_receipts.len(), trace.receipts.len());
        assert!(trace.every_forward_map_has_exact_local_inverse);
        assert!(trace.full_reverse_lifo_restores_initial);
        assert!(trace.every_working_ledger_closes);
        assert!(trace.every_augmented_ledger_closes);
        assert!(trace.every_complete_ledger_closes);
        assert!(trace.every_forward_energy_difference_matches_ledger);
        assert!(trace.every_forward_resource_difference_matches_ledger);
        assert!(trace.every_state_has_nontruncating_attempt_bit_capacity);
        assert_eq!(trace.terminal_state.P, Phase::Thermal);
        assert!(trace.terminal_state.complete_state_is_well_formed());
    }
    assert_eq!(fresh_first.channel_count(DrivenChannel::AliasQuarantine), 0);
    assert!(alias_first.channel_count(DrivenChannel::AliasQuarantine) > 0);
    let alias_then_fresh = alias_first.receipts.windows(2).any(|pair| {
        matches!(
            (&pair[0].event, &pair[1].event),
            (ModelEvent::Candidate(alias), ModelEvent::Candidate(fresh))
                if alias.kind == CandidateKind::Alias
                    && fresh.kind == CandidateKind::Fresh
                    && alias.port == fresh.port
                    && alias.episode == fresh.episode
        )
    });
    assert!(alias_then_fresh);
}

#[test]
fn descriptor_identity_is_quotiented_out_of_the_path_law() {
    let audit =
        descriptor_order_audit(2, false).expect("both descriptor-order fixtures instantiate");
    assert!(audit.path_law_quotient_identical);
    assert!(audit.promoted_words_identical);
    assert!(audit.shell_counts_identical);
    assert_ne!(
        audit.forward.terminal_state.Gamma.reservoir,
        audit.reversed.terminal_state.Gamma.reservoir
    );
}

#[test]
fn perturbation_closes_both_energy_boundaries_and_has_an_exact_inverse() {
    let construction = instantiate_one_support_trace(primary_fixture(2, false))
        .expect("complete construction exists");
    let mut state = construction.terminal_state;
    let leaf = state
        .Gamma
        .shell_words(3)
        .into_iter()
        .next()
        .expect("a depth-three leaf exists");
    state
        .schedule_leaf_perturbation(leaf.clone())
        .expect("leaf perturbation can be scheduled");
    let scheduled = state.clone();
    let before_resources = state.resource_snapshot();
    let before_working = state.working_energy();
    let before_internal = state.augmented_internal_energy();
    let before_complete = state.augmented_energy_with_external_perturbation_cell();
    assert_eq!(
        state.enabled_events(),
        vec![EnabledEvent::Perturb {
            leaf: leaf.clone(),
            hazard: Ratio::ONE
        }]
    );
    let receipt = state
        .apply_event(&ModelEvent::Perturb { leaf: leaf.clone() })
        .expect("scheduled PERTURB is enabled");
    assert_eq!(receipt.ledger.delta_working, 1);
    assert_eq!(receipt.ledger.delta_augmented, 1);
    assert_eq!(receipt.ledger.delta_external_work_store, -1);
    assert_eq!(receipt.ledger.delta_complete, 0);
    assert_eq!(state.working_energy(), before_working + 1);
    assert_eq!(state.augmented_internal_energy(), before_internal + 1);
    assert_eq!(
        state.augmented_energy_with_external_perturbation_cell(),
        before_complete
    );
    assert_eq!(
        state.resource_snapshot().difference_from(&before_resources),
        receipt.ledger.resources
    );
    assert!(receipt.ledger.working_first_law_closes());
    assert!(receipt.ledger.internal_augmented_first_law_closes());
    assert!(receipt.ledger.complete_first_law_closes());
    let bound_repair_identities = state
        .Q
        .attempt_bits
        .bits
        .iter()
        .filter(|bit| bit.lane == AttemptLane::Repair)
        .filter_map(|bit| bit.identity.as_ref())
        .collect::<Vec<_>>();
    assert_eq!(bound_repair_identities.len(), 2);
    assert!(bound_repair_identities
        .iter()
        .all(|identity| identity.episode == 1 && identity.port.child() == leaf));
    assert_eq!(
        bound_repair_identities
            .iter()
            .map(|identity| identity.kind)
            .collect::<std::collections::BTreeSet<_>>(),
        std::collections::BTreeSet::from([CandidateKind::Fresh, CandidateKind::Alias]),
    );
    assert!(state.complete_state_is_well_formed());
    assert!(state.repair_disabled_kernel_is_closed());
    assert!(state
        .enabled_events_with_support(KernelSupport::REPAIR_DISABLED)
        .is_empty());
    assert!(state
        .enabled_events_with_support(KernelSupport::PRIMARY)
        .iter()
        .any(|event| matches!(event, EnabledEvent::Candidate { .. })));
    let repair =
        instantiate_one_support_trace(state.clone()).expect("the reachable repair trace is finite");
    assert!(repair.every_state_has_nontruncating_attempt_bit_capacity);
    state.reverse_last().expect("UNPERTURB is enabled");
    assert_eq!(state, scheduled);
}

#[test]
fn complete_state_validator_checks_cross_component_correspondence() {
    let initial = primary_fixture(2, false);
    assert!(initial.complete_state_is_well_formed());

    let mut state = initial.clone();
    let candidate = state
        .enabled_candidates()
        .into_iter()
        .find(|candidate| candidate.kind == CandidateKind::Fresh)
        .expect("FRESH candidate exists");
    let receipt = state.fresh(candidate).expect("FRESH succeeds");
    assert!(state.complete_state_is_well_formed());
    let work_cell = match state.O.records[receipt.operation_record].entry.as_ref() {
        Some(OperationEntry::Fresh { work_cell, .. }) => *work_cell,
        _ => panic!("FRESH operation record exists"),
    };
    state.B_W.B_build.cells[work_cell].charged = true;
    assert!(!state.complete_state_is_well_formed());

    let mut disconnected = initial;
    disconnected.Gamma.structural_incidence.insert(
        xypher_endogenous_thermodynamics_proof::model::IncidenceEdge {
            parent: xypher_endogenous_thermodynamics_proof::model::Word::root(),
            port: 1,
            child: xypher_endogenous_thermodynamics_proof::model::Word(vec![1]),
        },
    );
    assert!(!disconnected.complete_state_is_well_formed());
}
