use xypher_thermodynamics_proof::{evaluate, GateId};

#[test]
fn frozen_source_contract_has_eleven_gates_and_four_literal_controls() {
    let report = evaluate();

    assert_eq!(report.gates.len(), 11, "{}", report.render());
    assert_eq!(report.controls.len(), 4, "{}", report.render());
    assert_eq!(
        report.gates.first().map(|gate| gate.gate),
        Some(GateId::G01)
    );
    assert_eq!(report.gates.last().map(|gate| gate.gate), Some(GateId::G11));
    assert!(report.is_success(), "{}", report.render());
}
