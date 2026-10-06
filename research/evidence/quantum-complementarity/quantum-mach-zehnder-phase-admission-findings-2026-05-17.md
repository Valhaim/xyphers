# Quantum Mach-Zehnder Phase Admission Findings

## Question

Can the delayed-choice admission law survive a phase-parametrized interferometer
sweep?

The tested boundary is:

```text
input state
-> first beamsplitter
-> unitary phase shift
-> unresolved path potentiality
-> late open/closed choice
-> detector effect/readback rows
-> state admission
```

This strengthens [[Xypher Quantum Delayed-Choice Admission Law]] and
[[research/physics/xypher-loop/quantum-mach-zehnder-admission-findings-2026-05-17]]
by testing a family of relative phases instead of one bright/dark fixture.

It remains a deterministic row fixture. It is not a full Born proof, collapse
mechanism, or laboratory delayed-choice trace.

## Command

```bash
cd thaim-core
cargo run -q --release --bin quantum-mach-zehnder-phase-admission
```

## Result

The harness passed:

```text
mach_zehnder_phase_admission_harness=PASS
passes=true
```

It emitted deterministic rows for:

```text
InputStateRow
BeamSplitterRow
PhaseShiftRow
PotentialityRow
LateChoiceRow
DetectorEffectRow
ReadbackRow
AdmissionReport
```

## Phase Rows

The sweep uses:

| theta | theta radians | phase | unitary |
|---|---:|---|---|
| theta_0 | 0.000000000000 | `(1.000000000000, 0.000000000000)` | yes |
| theta_pi_over_2 | 1.570796326795 | `(0.000000000000, 1.000000000000)` | yes |
| theta_pi | 3.141592653590 | `(-1.000000000000, 0.000000000000)` | yes |
| theta_3pi_over_2 | 4.712388980385 | `(-0.000000000000, -1.000000000000)` | yes |

The nonunitary control is:

```text
theta_pi_over_2 phase=(0.000000000000, 2.000000000000) unitary=false
```

## Formula

Open interferometer / which-path basis:

```text
P(path0) = 0.5
P(path1) = 0.5
```

Closed interferometer / interference basis:

```text
P(bright) = cos^2(theta / 2)
P(dark)   = sin^2(theta / 2)
```

The fixture checks both formulas as admission residuals.

## Admission Table

| case | theta | basis | second BS | phase unitary | detector | p(detector) | residual | Xi | admitted | reason |
|---|---|---|---:|---:|---|---:|---:|---:|---|---|
| open_which_path_readback | theta_0 | which_path | false | true | path0 | 0.500000 | 1.110e-16 | 3.000 | yes | admitted |
| closed_interference_readback | theta_0 | interference | true | true | bright | 1.000000 | 4.441e-16 | 4.000 | yes | admitted |
| open_which_path_readback | theta_pi_over_2 | which_path | false | true | path0 | 0.500000 | 1.110e-16 | 3.000 | yes | admitted |
| closed_interference_readback | theta_pi_over_2 | interference | true | true | bright | 0.500000 | 2.776e-16 | 4.000 | yes | admitted |
| open_which_path_readback | theta_pi | which_path | false | true | path0 | 0.500000 | 1.110e-16 | 3.000 | yes | admitted |
| closed_interference_readback | theta_pi | interference | true | true | dark | 1.000000 | 4.441e-16 | 4.000 | yes | admitted |
| open_which_path_readback | theta_3pi_over_2 | which_path | false | true | path0 | 0.500000 | 1.110e-16 | 3.000 | yes | admitted |
| closed_interference_readback | theta_3pi_over_2 | interference | true | true | dark | 0.500000 | 2.220e-16 | 4.000 | yes | admitted |
| hidden_path_before_choice | theta_pi_over_2 | interference | true | true | path0 | 0.000000 | 2.776e-16 | 0.000 | no | hidden_path_without_effect |
| retrocausal_prestate_rewrite | theta_0 | interference | true | true | bright | 1.000000 | 4.441e-16 | 4.000 | no | potentiality_history_mutation |
| nonunitary_phase_shift | theta_pi_over_2 | interference | true | false | bright | 1.250000 | 7.500e-1 | 4.000 | no | nonunitary_phase |
| basis_readback_mismatch | theta_0 | interference | true | true | bright | 1.000000 | 4.441e-16 | 4.000 | no | basis_readback_mismatch |
| no_work_paid | theta_0 | which_path | false | true | path0 | 0.500000 | 1.110e-16 | 3.000 | no | missing_work_row |
| d_eff_one_apparatus | theta_0 | which_path | false | true | path0 | 0.500000 | 1.110e-16 | 3.000 | no | insufficient_integration |
| non_positive_xi | theta_0 | which_path | false | true | path0 | 0.500000 | 1.110e-16 | 0.000 | no | non_positive_xi |

Checks:

```text
first_beamsplitter_unitary=PASS
second_beamsplitter_unitary=PASS
unitary_phase_rows_preserve_norm=PASS
open_records_admitted_all_phase_rows=PASS
closed_records_admitted_all_phase_rows=PASS
open_probabilities_phase_invariant=PASS
closed_probabilities_match_formula=PASS
open_and_closed_share_phase_potentiality=PASS
hidden_path_rejected=PASS
retrocausal_rewrite_rejected=PASS
nonunitary_phase_rejected=PASS
basis_mismatch_rejected=PASS
no_work_rejected=PASS
d_eff_one_rejected=PASS
non_positive_xi_rejected=PASS
```

## Interpretation

The non-trivial result is:

```text
phase changes interference readback
but does not create path state
```

The same phase-conditioned potentiality can resolve through:

```text
open choice -> which-path effects [0.5, 0.5]
closed choice -> interference effects [cos^2(theta/2), sin^2(theta/2)]
```

This makes the delayed-choice law sharper:

```text
relative phase is lawful potentiality structure
not hidden classical path state
```

The nonunitary phase control is useful because it has visible detector
probabilities and positive `Xi`, but it rejects before admission:

```text
nonunitary_phase_shift -> nonunitary_phase
```

So phase is not an arbitrary knob. It must be a lawful unitary row.

## What This Adds

| Prior validator | Added by QMPHASE |
|---|---|
| QDELAY | declared which-path/interference effects from same potentiality row |
| QMZI | explicit beamsplitter rows for one bright/dark interferometer trace |
| QMPHASE | phase sweep with formula residuals, open invariance, closed interference curve, and nonunitary phase rejection |

## Limits

Still open:

```text
continuous phase sampling beyond four fixture points
loss/decoherence rows
detector inefficiency rows
real apparatus thermodynamic work
real delayed-choice traces
full Born theorem
collapse mechanism
```

The current status should be:

```text
phase-parametrized Mach-Zehnder fixture: validated
particle/wave admission wording: strengthened
full quantum measurement theory: open
```

## Next

The next quantum steps are:

```text
loss/decoherence/apparatus work rows
continuous or randomized phase sweep
real or standard qubit frequency traces
POVM/context proof pass beyond fixtures
```
