use xypher_open_future_price_proof::{evaluate, GateId};

#[test]
fn frozen_source_contract_has_ten_gates_six_controls_and_passes() {
    let report = evaluate();
    let rendered = report.render();
    let lines: Vec<&str> = rendered.lines().collect();

    assert_eq!(report.gates.len(), 10, "{rendered}");
    assert_eq!(report.controls.len(), 6, "{rendered}");
    assert_eq!(report.worlds.len(), 24, "{rendered}");
    assert_eq!(report.hypotheses.len(), 4, "{rendered}");
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
    assert_eq!(gate_lines, 10, "{rendered}");
    let control_lines = (1..=6)
        .filter(|index| {
            lines
                .iter()
                .filter(|line| line.starts_with(&format!("C{index:02} ")))
                .count()
                == 1
        })
        .count();
    assert_eq!(control_lines, 6, "{rendered}");
    let world_lines = lines
        .iter()
        .filter(|line| line.starts_with("WORLD "))
        .count();
    assert_eq!(world_lines, 24, "{rendered}");
    let hypothesis_lines = (0..=3)
        .filter(|index| {
            lines
                .iter()
                .filter(|line| line.starts_with(&format!("H{index} ")))
                .count()
                == 1
        })
        .count();
    assert_eq!(hypothesis_lines, 4, "{rendered}");
    assert_eq!(
        report.gates.first().map(|gate| gate.gate),
        Some(GateId::G01)
    );
    assert_eq!(report.gates.last().map(|gate| gate.gate), Some(GateId::G10));
    assert!(
        matches!(lines.last(), Some(&"OVERALL PASS") | Some(&"OVERALL FAIL")),
        "{rendered}"
    );
    assert!(report.is_success(), "{rendered}");
}
