use xypher_alpha_tau_proof::exact::Ratio;
use xypher_alpha_tau_proof::fixtures::{
    confirmatory_cases, inherited_calibration, ControlId, BODY_DEGENERACIES, BODY_ENERGY_INDICES,
    CYCLE_EDGES,
};
use xypher_alpha_tau_proof::{evaluate, ControlGateStatus, GateId};

const GOLDEN_REPORT: &str = concat!(
    "CAL_ALPHA_0_EXACT_VERIFIER v1\n",
    "BOUNDARY commit=262cb0c076c78c7cc381fd9db00db563bf341b3b sha256=780e60cb3f08318edcb269718aea2e1ae7f98252bc143290b60264201cbe8859\n",
    "CALIBRATION b=2 lambda=1 g=1,4,4 n=0,1,2 fibers=4,8,4 rates=8,4,4,8 status=INHERITED_NON_EVALUATION\n",
    "G01_TYPED_SOURCES PASS candidates=10 bare-alpha-equality=rejected\n",
    "G02_RESERVOIR_SLOPE PASS reservoirs=4 ordered_nonzero_differences=48 beta=ln(b)/lambda\n",
    "G03_MICRO_LUMPING PASS cases=8 directed_channels=64 unit_edges=lazy_complete_bipartite\n",
    "G04_LDB_CLASSIFICATION PASS cases=8 labelled_pairs=32 A_LDB=UNIQUE T_LDB=T_R\n",
    "G05_BODY_INVARIANCE PASS groups=4 body_variants=2 temperature_invariant\n",
    "G06_ENERGY_GAUGE PASS cases=8 scale=3 offsets=7,11 rates_and_Xi_over_T=invariant\n",
    "G07_COMPONENT_INTERSECTION PASS D_ALL=ALL_POSITIVE D_SINGLETON={1/ln(3)} D_EMPTY=EMPTY\n",
    "G08_TAU_GROSS_PATH PASS T=2 S=0,1,0 gross=2,0,0 period=2\n",
    "G09_TAU_SIGNED_PAIR PASS signed=2,-2,0 Xi=-1,1,0 pair_and_cycles=exact\n",
    "G10_VARIABLE_INTEGRABILITY PASS source_increments=1,3,-10 period=-6 divided=2,4,-6 period=0\n",
    "G11_HEAT_WORK_NONIDENTITY PASS coarse=(DeltaS=1,DeltaU=1,TAU=1,Xi=0) ledgers=(1,0)!=(0,1)\n",
    "G12_PROTOCOL_PRICE_SCOPE PASS a_P=GATE_STATISTIC p_o=ACCOUNTING_PRICE_INTERFACE\n",
    "G13_EOS_SCOPE PASS receipt=CONDITIONAL_ACCOUNTING reservoir=EQUATION_OF_STATE\n",
    "C01 PASS half-reservoir-temperature lane=reservoir expected=FAIL@G02 observed=FAIL@G02 gates=G01:PASS,G02:FAIL,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C02 PASS inverse-label lane=typed-sources expected=FAIL@G01 observed=FAIL@G01 gates=G01:FAIL,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C03 PASS body-dependent-map lane=cross-case expected=FAIL@G05 observed=FAIL@G05 gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:FAIL,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C04 PASS scale-energy-only lane=channel expected=FAIL@G04 observed=FAIL@G04 gates=G01:PASS,G02:N/A,G03:N/A,G04:FAIL,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C05 PASS flat-energy-all lane=component expected=CLASS@G07:ALL_POSITIVE observed=CLASS@G07:ALL_POSITIVE gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:PASS,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C06 PASS informative-plus-flat lane=component expected=CLASS@G07:{1/ln(3)} observed=CLASS@G07:{1/ln(3)} gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:PASS,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C07 PASS mixed-component-baths lane=component expected=CLASS@G07:EMPTY observed=CLASS@G07:EMPTY gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:PASS,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C08 PASS nonlinear-reservoir lane=reservoir expected=FAIL@G02 observed=FAIL@G02 gates=G01:PASS,G02:FAIL,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C09 PASS zero-energy-ratio lane=channel expected=FAIL@G04 observed=FAIL@G04 gates=G01:PASS,G02:N/A,G03:N/A,G04:FAIL,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C10 PASS inconsistent-slopes lane=channel expected=FAIL@G04 observed=FAIL@G04 gates=G01:PASS,G02:N/A,G03:N/A,G04:FAIL,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C11 PASS aggregation-trap lane=channel expected=FAIL@G04 observed=FAIL@G04 gates=G01:PASS,G02:N/A,G03:N/A,G04:FAIL,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C12 PASS component-offset-gauge lane=gauge expected=PASS@G06 observed=PASS@G06 gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:PASS,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C13 PASS per-component-scale lane=component expected=CLASS@G07:EMPTY observed=CLASS@G07:EMPTY gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:PASS,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C14 PASS tau-as-state lane=gross-tau expected=FAIL@G08 observed=FAIL@G08 gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:N/A,G08:FAIL,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C15 PASS unpaired-tau-xi lane=signed-tau expected=FAIL@G09 observed=FAIL@G09 gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:FAIL,G10:N/A,G11:N/A,G12:N/A,G13:N/A\n",
    "C16 PASS source-price lane=variable-price expected=FAIL@G10 observed=FAIL@G10 gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:FAIL,G11:N/A,G12:N/A,G13:N/A\n",
    "C17 PASS heat-work-conflation lane=heat-work expected=FAIL@G11 observed=FAIL@G11 gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:FAIL,G12:N/A,G13:N/A\n",
    "C18 PASS relabeled-gate lane=protocol-price expected=FAIL@G12 observed=FAIL@G12 gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:FAIL,G13:N/A\n",
    "C19 PASS tau-is-equation-of-state lane=equation-of-state expected=FAIL@G13 observed=FAIL@G13 gates=G01:PASS,G02:N/A,G03:N/A,G04:N/A,G05:N/A,G06:N/A,G07:N/A,G08:N/A,G09:N/A,G10:N/A,G11:N/A,G12:N/A,G13:FAIL\n",
    "VERDICTS T_LDB=UNIQUE T_R=OPERATIONAL_TEMPERATURE a_P=GATE_STATISTIC p_o=ACCOUNTING_PRICE_INTERFACE TAU_PLUS=GROSS_PATH_RECEIPT\n",
    "VERDICTS_MORE A_N=SCALING_HYPOTHESIS p_N=SCALING_HYPOTHESIS TAU_SIGNED=SIGNED_ENTROPY_TERM source_p_dS=NONINTEGRABLE_EDGE_FORM divided_p_dS=STATE_POTENTIAL TAU_HEAT_WORK=UNIDENTIFIED\n",
    "OVERALL PASS\n",
);

#[test]
fn frozen_source_contract_matches_the_golden_report() {
    let report = evaluate();
    assert_eq!(report.gates.len(), 13, "{}", report.render());
    assert_eq!(report.controls.len(), 19, "{}", report.render());
    assert!(
        report
            .gates
            .iter()
            .zip(GateId::ALL)
            .all(|(outcome, expected)| outcome.gate == expected),
        "gate identity or order changed"
    );
    assert!(
        report
            .controls
            .iter()
            .zip(ControlId::ALL)
            .all(|(outcome, expected)| outcome.control == expected),
        "control identity or order changed"
    );
    assert!(report.is_success(), "{}", report.render());
    assert_eq!(report.render(), GOLDEN_REPORT);
}

#[test]
fn controls_are_lane_isolated_and_calibration_is_not_evaluation_data() {
    let report = evaluate();
    for outcome in &report.controls {
        let applicable = outcome
            .gate_statuses
            .iter()
            .filter(|status| **status != ControlGateStatus::NotApplicable)
            .count();
        let expected = if outcome.control == ControlId::C02 {
            1
        } else {
            2
        };
        assert_eq!(applicable, expected, "{}", outcome.control.code());
    }
    let calibration = inherited_calibration();
    assert!(!calibration.evaluation_data);
    assert_eq!(calibration.reservoir_base, 2);
    assert_eq!(calibration.lambda, Ratio::ONE);
    assert_eq!(calibration.degeneracies, [1, 4, 4]);
    assert_eq!(calibration.energy_indices, [0, 1, 2]);
    assert_eq!(calibration.fibers, [4, 8, 4]);
    assert_eq!(calibration.rates, [8, 4, 4, 8]);

    let cases = confirmatory_cases();
    assert_eq!(cases.len(), 8);
    assert_eq!(
        cases.iter().map(|case| case.id()).collect::<Vec<_>>(),
        vec![
            "b3-lambda1-g1".to_string(),
            "b3-lambda1-g2".to_string(),
            "b3-lambda3-g1".to_string(),
            "b3-lambda3-g2".to_string(),
            "b5-lambda1-g1".to_string(),
            "b5-lambda1-g2".to_string(),
            "b5-lambda3-g1".to_string(),
            "b5-lambda3-g2".to_string(),
        ]
    );
    for case in &cases {
        assert_eq!(case.energy_indices, BODY_ENERGY_INDICES);
        assert_eq!(case.degeneracies, BODY_DEGENERACIES[case.body_variant]);
        assert!([3, 5].contains(&case.reservoir_base));
        assert!([Ratio::ONE, Ratio::integer(3)].contains(&case.lambda));
    }
    assert_eq!(CYCLE_EDGES, [(0, 1), (1, 2), (2, 3), (3, 0)]);
}

#[test]
fn authoritative_rust_sources_name_no_float_type() {
    for source in [
        include_str!("../src/exact.rs"),
        include_str!("../src/fixtures.rs"),
        include_str!("../src/lib.rs"),
        include_str!("../src/main.rs"),
    ] {
        assert!(!source.contains("f32"));
        assert!(!source.contains("f64"));
    }
}
