# Quantum Born-Row Validator Audit Findings

Date: 2026-05-17
Task: td-a213e0

## Question

Does the existing `quantum-born-rule` harness derive Born weighting from raw
Xypher entropy objectives?

The proof obligation from [[Xypher Quantum Information-to-State Bridge]] is:

```text
Given psi = sum_i c_i |i>,
show p_i = |c_i|^2 is the unique resolution weighting
that preserves replayable expected state-admission accounting.
```

The existing binary tests a narrower question:

```text
candidate p-rule: probability(i) proportional to |psi_i|^p
which p maximizes observer/system S_tau-style objectives?
```

## Validation

Command:

```bash
cargo run -q --release --bin quantum-born-rule
```

The run completed. The build emitted existing unused-code warnings, including
warnings inside `quantum_born_rule.rs`, but the binary produced a full report.

## Result

The synthesis table did not select Born `p=2` for any declared objective:

| Objective | Winner p | Born p=2? |
|---|---:|---|
| Observer node `S_tau` | 0.25 | no |
| Observer average `S_tau` | 10.00 | no |
| Combined observer+system `S_tau` | 0.25 | no |
| Observer `Delta S_tau` | 0.25 | no |
| Information gain | 0.25 | no |

The detailed cases show why this is not an accidental miss. Depending on graph
and observer shape, flattening the branch distribution (`p=0.25`) often wins
information-like objectives, while sharpening toward a dominant branch
(`p=10`) can win observer-average or node-local objectives. Born `p=2` appears
inside the sweep but is not selected by the raw objectives.

## Reading

This is a useful negative result.

It rejects the shortcut:

```text
Born rule = argmax raw observer/system S_tau
```

The stronger conclusion is:

```text
Born, if derivable in Xypher terms, must come from state-admission invariants,
not from naked entropy maximization.
```

The missing invariants are exactly the ones already named in the bridge:

```text
BornWeightRow
DeltaSRow
WorkRow
XiResolutionRow
ResolutionFrequencyRow
```

Raw `S_tau` objectives can select probabilities that are better for the
observer's immediate entropy or information gain, but quantum probabilities
must also preserve composition, basis consistency, empirical frequencies,
apparatus work, and replayable state-admission accounting. Those constraints
are not present in this binary.

## Law Pressure

The audit suggests a sharper law candidate:

```text
Quantum resolution weights are not branch rewards.
They are admissible expectation weights for state formation under composition,
work, and replay invariance.
```

In Xypher language:

```text
branch score is not branch probability
state-admission accounting is the object to conserve
```

This is aligned with the broader Xypher calculus pattern:

```text
Delta S_tau proposes
Xi gates
resolution records
composition constrains
```

## What This Does Not Prove

This audit does not prove Born's rule.

It also does not refute the Xypher quantum bridge. It refutes only one tempting
derivation path:

```text
choose the p that maximizes raw S_tau-like observer gain
```

The bridge remains a theorem/conjecture map:

```text
potentiality -> candidate futures
decoherence -> branch readability
measurement -> state-forming effect row
Born weighting -> still open
```

## Gaps To A True Born-Row Validator

The existing binary lacks:

| Missing row/control | Why it matters |
|---|---|
| `BornWeightRow(i)=|c_i|^2` as an explicit row | the current code sweeps p-rules but does not emit branch rows |
| `WorkRow(i)` | measurement and integration cost are not priced |
| `XiResolutionRow(i)` | net state-admission value is not computed |
| `ResolutionFrequencyRow` | no repeated empirical measurement trace is compared to weights |
| basis/composition controls | Born uniqueness is a composition/invariance question, not only a local graph objective |
| decoherence-vs-readback controls | the harness does not vary branch readability separately from recorded state |
| `d_eff` apparatus variation | the measurement-selection conjecture is not tested |

## Next

The next validator should not ask:

```text
which p maximizes raw S_tau?
```

It should ask:

```text
which p-rule preserves expected Xi/state-admission accounting under basis
changes, composition, apparatus work, and empirical resolution frequencies?
```

Minimum next harness:

```text
qubit with controlled amplitudes
fixed apparatus and basis
explicit BornWeightRow / DeltaSRow / WorkRow / XiResolutionRow
alternative p-rule controls
composition or basis-change control
resolution frequency row, if empirical simulation is included
```

Parallel measurement harness:

```text
same decohered branch weights
vary readback/effect/integration coupling
check whether reusable state records differ
```

Killing condition remains:

```text
If another p-rule preserves the same state-admission accounting under all
tested bases and compositions, Born-from-Xypher is not unique.
```
