//! Source contract on the primary witness `(3, 2, 5, 2, 4, 2)` and the
//! controls only; the other 72 confirmatory steady states are not solved
//! here.

use xypher_theory_of_aim_proof::{evaluate_primary, GateId};

#[test]
fn primary_passes_every_gate_and_every_control_fails_first_where_declared() {
    let report = evaluate_primary();
    let rendered = report.render();
    let lines: Vec<&str> = rendered.lines().collect();

    assert_eq!(report.gates.len(), 12, "{rendered}");
    assert_eq!(report.controls.len(), 9, "{rendered}");
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
    let gate_lines: Vec<&&str> = lines.iter().filter(|line| line.starts_with('G')).collect();
    assert_eq!(gate_lines.len(), 12, "{rendered}");
    for (line, gate) in gate_lines.iter().zip(GateId::ALL) {
        assert!(
            line.starts_with(&format!("{} PASS {} -- ", gate.code(), gate.label())),
            "{rendered}"
        );
    }

    let control_lines: Vec<&&str> = lines.iter().filter(|line| line.starts_with('C')).collect();
    assert_eq!(control_lines.len(), 9, "{rendered}");
    for (index, (control, line)) in report.controls.iter().zip(&control_lines).enumerate() {
        let code = format!("C{:02}", index + 1);
        assert_eq!(control.code, code, "{rendered}");
        assert!(control.passed, "{rendered}");
        let observed = control.observed.as_ref().expect("a control fails");
        assert_eq!(observed.check, control.expected, "{rendered}");
        assert_eq!(observed.agent, control.expected_agent, "{rendered}");
        assert!(line.starts_with(&format!("{code} PASS ")), "{rendered}");
    }
    let table_level = &report.controls[8];
    assert_eq!(table_level.code, "C09", "{rendered}");
    assert!(
        table_level
            .observed
            .as_ref()
            .is_some_and(|failure| failure.world.is_none() && failure.configuration.is_none()),
        "{rendered}"
    );

    let world_lines: Vec<&&str> = lines
        .iter()
        .filter(|line| line.starts_with("WORLD "))
        .collect();
    assert_eq!(world_lines.len(), 4, "{rendered}");
    for (line, agent) in world_lines
        .iter()
        .zip(["LOCAL", "AIM-1", "AIM-2", "GLOBAL"])
    {
        assert!(
            line.starts_with(&format!(
                "WORLD (3,2,5,2,v=4,R=2) {agent} states=216 orbits=114 NR_p="
            )),
            "{rendered}"
        );
    }
    for (line, frozen) in world_lines.iter().zip(["1.347621", "1.380819", "1.225313"]) {
        assert!(
            line.contains(&format!(" E_th={frozen} E/E_th=")),
            "{rendered}"
        );
    }
    assert!(
        world_lines[3].contains(" E=1 ~1.0000 E_th=none E/E_th=none "),
        "{rendered}"
    );

    assert_eq!(lines.len(), 1 + 12 + 9 + 4 + 1, "{rendered}");
    assert!(
        lines
            .first()
            .is_some_and(|line| line.starts_with("XYPHER_THEORY_OF_AIM_PROOF CAL-CEF-5 ")),
        "{rendered}"
    );
    assert_eq!(lines.last(), Some(&"OVERALL PASS"), "{rendered}");
    assert!(report.is_success(), "{rendered}");
}
