# Quantum No-Lab Validation Ladder

Date: 2026-06-07
Task: td-6f39c3

## Thesis

The no-lab program should validate the Xypher quantum bridge in this order:

```text
proof obligations
-> deterministic adversarial row sweeps
-> standard experiment replay
-> public/cloud quantum traces
-> apparatus work calibration
-> hard d_eff experiment design
```

The target claim stays disciplined:

```text
quantum information becomes state only through invariant, costed resolution
```

Not:

```text
collapse mechanism proven
full Born theorem complete
laboratory measurement problem solved
```

## Ladder

### 1. Proof Obligations

Born weighting must be derived from admission invariants, not raw branch reward.

Current proof pressure:

```text
same event, same accounting, regardless of lawful refinement
```

Within power rules `mu_p(i) proportional to |c_i|^p`, orthogonal
coarse-graining invariance selects `p=2`. The next proof pass should move from
the power-rule/projector slice to finite effect algebras and then toward a
Gleason-style statement:

```text
mu(E_1 + E_2) = mu(E_1) + mu(E_2)
mu(E | C_1) = mu(E | C_2)
E_mu[Xi] is invariant under lawful detector refinement
```

### 2. Deterministic Adversarial Sweeps

Existing fixtures validate specific named cases. The next no-lab step is
generated counterexample search over many valid state/effect rows.

New harness:

```bash
cd thaim-core
cargo run -q --release --bin quantum-adversarial-admission-sweep
```

The harness generates deterministic qubit states and valid POVM contexts:

```text
coarse context: {E, I-E}
refined context: {fE, (1-f)E, I-E}
alternate context: {E, g(I-E), (1-g)(I-E)}
```

It sweeps nonlinear normalized rules:

```text
q_gamma(E_i | C) = Tr(rho E_i)^gamma / sum_j Tr(rho E_j)^gamma
```

Expected result:

```text
gamma = 1 admits all generated rows
gamma != 1 rejects under refinement/context/Xi-drift residuals
raw non-Born expected-Xi winners still reject before ranking
```

This is a stronger falsification surface than a hand-picked fixture because a
counterexample can arise from generated valid effects, random Bloch states,
split ratios, or Xi assignments.

### 3. Standard Experiment Replay

Replay textbook experiments as row contracts:

```text
Mach-Zehnder phase/loss/decoherence
double slit with which-path erase/readback rows
Stern-Gerlach sequential basis rows
Bell/CHSH effect rows
quantum eraser delayed readback rows
```

The question is not whether standard quantum mechanics predicts the frequencies.
It does. The Xypher question is whether one admission stack accounts for every
record without hidden path state, retrocausal mutation, or raw reward authority.

### 4. Public Or Cloud Quantum Traces

Use published or cloud-hardware counts as `ResolutionFrequencyRow` inputs:

```text
QuantumStateRow
BornWeightRow
ResolutionFrequencyRow
DeltaSRow
WorkRow
XiResolutionRow
ReadbackRow
EffectRow
IntegrationRow
AdmissionReport
```

No local lab is needed for this stage. A public trace is enough to test whether
the deterministic row contract can ingest real noisy frequencies without adding
a hidden selection thumb.

### 5. Apparatus Work Calibration

Current harnesses use dimensionless work rows such as surprisal work:

```text
W_paid >= -log2(P(selected_detector))
```

The next calibration pass should map published detector specs into comparable
work rows:

```text
readout energy
dark-count correction
detector efficiency / loss sink
erasure/reset cost
timing-window cost
```

The requirement is unit discipline, not perfect metrology in the first pass.

### 6. d_eff Selection

The hard claim remains:

```text
d_eff >= 2 integration is necessary for state-forming measurement
```

This is the part most likely to require a custom physical experiment later.
Before that, no-lab work can still sharpen falsifiers:

```text
d_eff=1 apparatus selects state beyond decoherence under controlled replay
decoherence-only emits reusable state records without readback/effect/work
state records erase without work/thaw/accounting
```

## Current QVAL-1 Deliverable

QVAL-1 starts the program with the generated adversarial sweep. It tests the
core proof pressure:

```text
admission first
ranking second
```

If a non-Born rule wins raw expected `Xi`, that is not enough. It must preserve
effect refinement, context invariance, and expected state-admission accounting.
Otherwise it is bookkeeping-dependent and cannot admit state.

## QVAL-1 Result

Command:

```bash
cd thaim-core
cargo run -q --release --bin quantum-adversarial-admission-sweep
```

Result:

```text
adversarial_admission_sweep=PASS
passes=true
```

Summary:

| gamma | admitted rows | raw expected-Xi winner rows |
|---:|---:|---:|
| 0.25 | 0 / 256 | 128 |
| 0.50 | 0 / 256 | 0 |
| 1.00 | 256 / 256 | 0 |
| 1.50 | 0 / 256 | 0 |
| 2.00 | 0 / 256 | 0 |
| 4.00 | 0 / 256 | 128 |

Checks:

```text
born_gamma1_admitted_all=PASS
born_refinement_residual_zero=PASS
born_context_residual_zero=PASS
born_expected_xi_drift_zero=PASS
nonborn_rejected_all=PASS
nonborn_refinement_residual_detected=PASS
nonborn_context_residual_detected=PASS
raw_nonborn_xi_winners_exist=PASS
raw_nonborn_xi_winners_rejected=PASS
raw_nonborn_xi_winner_count=256
```

Reading:

```text
raw branch value can prefer non-Born measures
but admitted state accounting still selects Born-linear gamma=1
```

This strengthens the no-lab route because the pass was not a single named
fixture. It was a generated sweep over valid qubit states, valid effects, event
refinements, alternate detector contexts, split ratios, and `Xi` assignments.
