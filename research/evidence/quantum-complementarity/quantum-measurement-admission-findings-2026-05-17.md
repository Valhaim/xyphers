# Quantum Measurement Admission Findings

Date: 2026-05-17
Task: td-081041

## Question

Does decoherence/readability by itself count as Xypher state admission?

The bridge claim under test from [[Xypher Quantum Information-to-State Bridge]]:

```text
decoherence = branch readability
measurement = state formation through readback/effect/work/integration rows
```

The harness keeps branch weights identical across cases so that any admission
difference must come from the measurement rows, not from changing the quantum
probabilities.

## Validation

Harness:

```text
thaim-core/src/bin/quantum_measurement_admission.rs
```

Command:

```bash
cargo run -q --release --bin quantum-measurement-admission
```

The command completed. The shared crate emitted existing unused-code warnings;
the QMEAS harness passed.

## Results

All cases use the same decohered branch weights:

```text
[0.64, 0.36]
```

| Case | Same weights | Branches readable | Xi | Admitted | Reason |
|---|---|---|---:|---|---|
| decoherence_only | true | true | 0.000000 | false | no_resolution_bind |
| integrated_readback | true | true | 3.000000 | true | admitted |
| d_eff_one_apparatus | true | true | 3.000000 | false | insufficient_integration |
| no_work_paid | true | true | 4.000000 | false | missing_work_row |
| no_effect_row | true | true | 3.000000 | false | missing_effect_row |
| wrong_readback | true | true | 3.000000 | false | readback_mismatch |
| non_positive_xi | true | true | -0.500000 | false | non_positive_xi |

Checks:

```text
all_cases_share_same_decohered_weights=PASS
decoherence_only_rejected=PASS
integrated_readback_admitted=PASS
d_eff_one_rejected=PASS
no_work_rejected=PASS
no_effect_rejected=PASS
wrong_readback_rejected=PASS
non_positive_xi_rejected=PASS
decoherence_readback_admission_harness=PASS
passes=true
```

## Reading

The non-trivial result is:

```text
same branch weights
same readability
different state admission
```

That validates the Xypher distinction:

```text
decoherence is not measurement
readability is not state
```

State admission requires:

```text
ResolutionBind
matching readback
EffectRow
WorkRow
integration d_eff >= 2
positive Xi
```

This makes the particle/wave bridge sharper:

```text
wave = unresolved potentiality
decohered mixture = readable branches
particle/record = admitted state at an effect boundary
```

## Law Finding

The harness validates this law slice:

```text
A branch-readable quantum substrate does not form Xypher state unless
readback, effect, work, integration, and Xi admission rows pass.
```

This is the measurement analogue of the broader Xypher pattern:

```text
candidate futures are not state
positive score is not state
readable branches are not state
resolution rows make state
```

## Limits

Not validated:

```text
physical collapse mechanism
Born empirical frequencies
real apparatus thermodynamics
general POVMs
neural/conscious d_eff threshold
```

The result is a row-accounting validator for the Xypher bridge, not a complete
quantum measurement theory.

## Next

The next quantum validator should join QRES and QMEAS:

```text
Born/invariant weights
empirical ResolutionFrequencyRow
apparatus WorkRow
decoherence/readback split
d_eff variation
```

The open question:

```text
Can the same row contract predict real measurement frequencies and apparatus
state formation without adding a hidden selection thumb?
```
