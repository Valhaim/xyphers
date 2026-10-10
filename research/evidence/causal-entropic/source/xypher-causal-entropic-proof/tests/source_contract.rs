use xypher_causal_entropic_proof::{evaluate, GateId};

#[test]
fn frozen_source_contract_has_twelve_gates_and_eight_literal_controls() {
    let report = evaluate();

    assert_eq!(report.gates.len(), 12, "{}", report.render());
    assert_eq!(report.controls.len(), 8, "{}", report.render());
    assert_eq!(
        report.gates.first().map(|gate| gate.gate),
        Some(GateId::G01)
    );
    assert_eq!(report.gates.last().map(|gate| gate.gate), Some(GateId::G12));
    assert!(report.is_success(), "{}", report.render());
}
