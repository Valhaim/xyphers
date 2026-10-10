//! Source contract on the primary witness `(3, 2, 8, 2)` and the controls
//! only; the other 62 confirmatory steady states are not solved here.

use xypher_aimed_foresight_proof::{evaluate_primary, GateId};

#[test]
fn primary_passes_every_gate_and_every_control_fails_first_where_declared() {
    let report = evaluate_primary();
    let rendered = report.render();
    let lines: Vec<&str> = rendered.lines().collect();

    assert_eq!(report.gates.len(), 11, "{rendered}");
    assert_eq!(report.controls.len(), 7, "{rendered}");
    assert_eq!(report.worlds.len(), 4, "{rendered}");
    assert!(report.hypotheses.is_empty(), "{rendered}");

    assert_eq!(
        report
            .gates
            .iter()
            .map(|gate| gate.gate)
            .collect::<Vec<_>>(),
        GateId::ALL.to_vec(),
        "{rendered}"
    );
    for gate in GateId::ALL {
        let matching: Vec<&&str> = lines
            .iter()
            .filter(|line| line.starts_with(&format!("{} ", gate.code())))
            .collect();
        assert_eq!(matching.len(), 1, "{rendered}");
        assert!(
            matching[0].starts_with(&format!("{} PASS ", gate.code())),
            "{rendered}"
        );
    }
    for (index, control) in report.controls.iter().enumerate() {
        let code = format!("C{:02}", index + 1);
        assert_eq!(control.code, code, "{rendered}");
        assert!(control.passed, "{rendered}");
        let matching: Vec<&&str> = lines
            .iter()
            .filter(|line| line.starts_with(&format!("{code} ")))
            .collect();
        assert_eq!(matching.len(), 1, "{rendered}");
        assert!(
            matching[0].starts_with(&format!("{code} PASS ")),
            "{rendered}"
        );
    }
    let world_lines: Vec<&&str> = lines
        .iter()
        .filter(|line| line.starts_with("WORLD (3,2,8,2) "))
        .collect();
    assert_eq!(world_lines.len(), 4, "{rendered}");
    for (line, agent) in world_lines
        .iter()
        .zip(["LOCAL", "AIM-1", "AIM-2", "GLOBAL"])
    {
        assert!(
            line.starts_with(&format!("WORLD (3,2,8,2) {agent} states=216 orbits=114 ")),
            "{rendered}"
        );
    }
    assert!(
        lines
            .first()
            .is_some_and(|line| line.starts_with("XYPHER_AIMED_FORESIGHT_PROOF CAL-CEF-4 ")),
        "{rendered}"
    );
    assert_eq!(lines.last(), Some(&"OVERALL PASS"), "{rendered}");
    assert!(report.is_success(), "{rendered}");
}
