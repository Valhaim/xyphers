use xypher_control_information_proof::{evaluate, GateId};

#[test]
fn frozen_source_contract_has_fourteen_gates_and_nine_literal_controls() {
    let report = evaluate();
    let rendered = report.render();
    let lines: Vec<&str> = rendered.lines().collect();

    assert_eq!(report.gates.len(), 14, "{rendered}");
    assert_eq!(report.controls.len(), 9, "{rendered}");
    let gate_lines = GateId::ALL
        .iter()
        .filter(|gate| {
            lines
                .iter()
                .filter(|line| line.starts_with(&format!("{} ", gate.code())))
                .count()
                == 1
        })
        .count();
    assert_eq!(gate_lines, 14, "{rendered}");
    let control_lines = (1..=9)
        .filter(|index| {
            lines
                .iter()
                .filter(|line| line.starts_with(&format!("C{index:02} ")))
                .count()
                == 1
        })
        .count();
    assert_eq!(control_lines, 9, "{rendered}");
    assert_eq!(
        report.gates.first().map(|gate| gate.gate),
        Some(GateId::G01)
    );
    assert_eq!(report.gates.last().map(|gate| gate.gate), Some(GateId::G14));
    assert!(
        matches!(lines.last(), Some(&"OVERALL PASS") | Some(&"OVERALL FAIL")),
        "{rendered}"
    );
    assert!(report.is_success(), "{rendered}");
}
