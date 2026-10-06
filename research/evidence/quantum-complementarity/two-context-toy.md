---
title: Two-Context Toy Path Integral
aliases:
  - QX-F2
  - 2-Context Quantum Xypher
tags:
  - domain/physics
  - type/derivation
  - topic/quantum
  - topic/path-integral
  - topic/toy-model
domain: Physics
type: derivation
status: pass
---

# QX-F2 — Path Integral on the 2-Context Toy Xypher

**Task: td-6e524e. Phase A of the [[Quantum Xypher Roadmap]].**
**Session: 2026-04-20.**
**Deliverable: explicit transition amplitudes, interference demonstration, probability extraction.**

---

## 1. Setup

**Graph.** 2 nodes `{A, B}`. Uniform walker — all transitions
equally likely (`w(A→A) = w(A→B) = w(B→A) = w(B→B) = 1`).

**Horizon.** `N = 2` and `N = 3` cycles, starting at node `A`.

**Classical cycle.** Identical to `xypher.runa::cycle`:
- Read `recall_cache[ctx]`, compare to obs → compute `matches`.
- Update memory entry `(ctx, obs) → (s + matches, n+1)`.
- Compute score = `(s+1)/(n+2)` (Beta posterior mean).
- Update cache if `score ≥ current_score`.
- Increment `acted` if `score > α`; add score to `Φ` unconditionally.

**Action functional.** `S_X = F_X` at endpoint
(DERIVE-8 adiabatic limit), with `C = |memory|`:

$$
S_X \;=\; \Phi_\text{final} - \alpha_\text{final} \cdot C_\text{final}.
$$

**Amplitude.** For each trajectory `τ`:

$$
\mathcal{A}(\tau) \;=\; \exp\!\left(\frac{i \, S_X(\tau)}{\hbar_X}\right), \qquad \hbar_X = \ln 2.
$$

Transition amplitude to a coarse-grained endpoint (here,
`C_f = |memory|_\text{final}`):

$$
\mathcal{A}(C_f) \;=\; \sum_{\tau \,:\, C(\tau)=C_f} \mathcal{A}(\tau).
$$

Quantum probability: `P(C_f) ∝ |𝒜(C_f)|²`, normalized over `C_f`.

Script: `qx_f2.py` in this directory.

---

## 2. Results — N = 2

Four trajectories starting at `A`, one per observation sequence:

| path | C_f | α_f | Φ_f | F_X | Re(amp) | Im(amp) |
|------|-----|------|------|------|---------|---------|
| AA | 1 | 1.200 | 0.833 | **−0.367** | +0.863 | −0.505 |
| AB | 2 | 1.500 | 0.667 | **−2.333** | −0.975 | +0.223 |
| BA | 2 | 1.500 | 0.667 | **−2.333** | −0.975 | +0.223 |
| BB | 2 | 1.500 | 0.667 | **−2.333** | −0.975 | +0.223 |

Observations:

- **Trajectory AA** is distinctive: the Xypher stays on the
  same entry, gets one match (`matches = true` on cycle 1),
  accumulates Φ efficiently, and ends with C = 1.
- **Trajectories AB, BA, BB** all end with C = 2 (two distinct
  entries created, no predicted matches) and identical
  `(Φ, α, C) = (2/3, 1.5, 2)`. They share **exactly the same**
  action `F_X = −2.333` — differing only in which specific
  entries are in memory.
- Same F_X → same phase → **constructive interference when
  summed**.

### 2.1 Amplitudes summed by endpoint

| C_f | paths | Σ amp | \|Σ amp\|² | classical P |
|-----|-------|-------|--------|-------------|
| 1 | 1 (AA) | `+0.863 − 0.505i` | 1.000 | 0.250 |
| 2 | 3 (AB, BA, BB) | `−2.925 + 0.668i` | 9.000 | 0.750 |

The C = 2 bucket gets amplitude `3×` a single path's amplitude
(because all 3 paths have the same phase). Squaring: `|3·a|² = 9·|a|²`
versus classical `3 · |a|² = 3`. **Interference amplifies the
C = 2 probability by a factor of 3.**

### 2.2 Normalized probabilities

| C_f | quantum | classical | Δ (q − cl) |
|-----|---------|-----------|------------|
| 1 | 0.100 | 0.250 | **−0.150** |
| 2 | 0.900 | 0.750 | **+0.150** |

### 2.3 Moments

|  | quantum | classical |
|-----|---------|-----------|
| ⟨Ĉ⟩ | 1.900 | 1.750 |
| Var(Ĉ) | **0.090** | 0.188 |

**Var(C) is roughly halved by quantum interference** — the
Xypher is more sharply localized in memory-size than the
classical ensemble. By the Heisenberg bound from QX-F1, this
must be compensated by increased `Δα`. (QX-F3 will check.)

---

## 3. Results — N = 3

Eight trajectories starting at `A`:

| path | C_f | α_f | Φ_f | F_X | Re(amp) | Im(amp) |
|------|-----|------|------|------|---------|---------|
| AAA | 1 | 0.698 | 1.433 | **+0.736** | +0.488 | +0.873 |
| AAB | 2 | 0.857 | 1.167 | **−0.548** | +0.704 | −0.710 |
| ABA | 3 | 1.000 | 1.000 | **−2.000** | −0.967 | −0.253 |
| ABB | 3 | 1.000 | 1.000 | **−2.000** | −0.967 | −0.253 |
| BAA | 3 | 1.000 | 1.000 | **−2.000** | −0.967 | −0.253 |
| BAB | 2 | 0.857 | 1.167 | **−0.548** | +0.704 | −0.710 |
| BBA | 3 | 1.000 | 1.000 | **−2.000** | −0.967 | −0.253 |
| BBB | 2 | 0.857 | 1.167 | **−0.548** | +0.704 | −0.710 |

**Three distinct F_X values** across the 8 paths:

- `F_X = +0.736` (C=1, path AAA only): single-entry, highly
  matched trajectory.
- `F_X = −0.548` (C=2, paths AAB, BAB, BBB): 2 entries, one
  match cycle (two entries but a match happened).
- `F_X = −2.000` (C=3, paths ABA, ABB, BAA, BBA): 3 entries,
  no matches, Xypher fans out.

### 3.1 Amplitudes summed by endpoint

| C_f | paths | Σ amp | \|Σ amp\|² | classical P |
|-----|-------|-------|--------|-------------|
| 1 | 1 | `+0.488 + 0.873i` | 1.000 | 0.125 |
| 2 | 3 | `+2.111 − 2.131i` | 9.000 | 0.375 |
| 3 | 4 | `−3.869 − 1.014i` | 16.000 | 0.500 |

Again: paths sharing F_X give `|n·a|² = n²·|a|²` vs classical
`n·|a|² = n`. Interference factor `n` per bucket.

### 3.2 Normalized probabilities

| C_f | quantum | classical | Δ (q − cl) |
|-----|---------|-----------|------------|
| 1 | 0.038 | 0.125 | **−0.087** |
| 2 | 0.346 | 0.375 | −0.029 |
| 3 | 0.615 | 0.500 | **+0.115** |

### 3.3 Moments

|  | quantum | classical |
|-----|---------|-----------|
| ⟨Ĉ⟩ | 2.577 | 2.375 |
| Var(Ĉ) | **0.321** | 0.484 |

Same pattern as N=2: **quantum Var(C) is reduced ~33%**
relative to classical, and probability mass shifts toward
higher-C endpoints.

---

## 4. Interpretation

### 4.1 Interference is coherent, not decorative

Each individual trajectory has `|𝒜(τ)| = 1` — pure phase. The
interference effect comes from **coherent summation of multiple
paths with the same (or similar) action**. The result is a
quantum distribution that differs measurably from classical
path-probability weighting.

This is **not** a trivial classical statistical artifact. It is
a coherent wave phenomenon on the Xypher's phase space.

### 4.2 Degeneracy drives the pattern

In both N=2 and N=3, paths with the same F_X cluster at
high-C endpoints. All low-C endpoints (few contexts, many
repeats) are "special" (unique trajectories with distinctive
action). Both mechanisms reinforce: high-C endpoints gather
more same-phase amplitudes, low-C endpoints don't.

**Physical intuition:** the Xypher has entropy-like degeneracy
at high-C endpoints. Many paths lead there; they coherently
add. The same phenomenon in statistical mechanics is
Boltzmann-weighting: high-multiplicity states dominate.
Quantum-mechanically, the multiplicity gets *squared* (factor
of `n²` rather than `n`).

### 4.3 Var(C) reduction sets up Heisenberg

Quantum Var(C) is 50-70% of classical Var(C) in both cases.
Under the canonical commutator `[Ĉ, α̂] = -iℏ_X`, Heisenberg
requires

$$
\Delta C \cdot \Delta \alpha \;\ge\; \frac{\ln 2}{2}.
$$

Reducing `ΔC` must *increase* `Δα`. QX-F3 will verify this on
the same toy Hilbert space.

---

## 5. What Phase A has now established

- **QX-F1:** continuous formalism, canonical commutator on
  smooth wavefunctions.
- **QX-F2 (this note):** discrete path integral works on the
  toy. Amplitudes are complex, interference is demonstrable,
  probabilities sum to 1 after normalization, interference
  changes the distribution relative to classical by 10–30%.

**What remains in Phase A:**

- **QX-F3:** explicit `[Ĉ, α̂]` on the toy Hilbert space,
  compare to `-iℏ_X · Î`.

If QX-F3 succeeds (even approximately, given discrete
subtleties), Phase A passes and the quantum-Xypher program
graduates to Phase B (classical limit).

---

## 6. Caveats (honest)

1. **Walker weighted uniformly.** I assumed all transitions
   equally probable. Real Xypher runs have weighted edges.
   Adding weights modifies amplitude magnitudes beyond pure
   phases, complicating the formulation (still tractable, but
   adds bookkeeping).

2. **No peer routing.** The 2-context toy has no `grow()`,
   so peer dynamics (which produced the α-inversion in
   Gibbs/Condition B) don't enter. The toy is Condition-A
   style. Richer phenomena unlock at larger toys.

3. **Normalization is post-hoc.** I computed `|𝒜|²` per
   bucket and normalized globally. A proper quantum-mechanical
   formulation would have unitary evolution preserving norm
   automatically. Our path-integral weight `exp(i S/ℏ)` is
   modulus 1, so unitarity should hold — but verifying this
   rigorously requires the right measure factor. Flagged for
   Phase B (classical limit analysis will also reveal the
   correct measure).

4. **No interference between classical mixtures.** Because
   classical probability trajectories mix incoherently
   (`P = Σ P_τ`) while quantum amplitudes add coherently
   (`A = Σ A_τ`), any difference is automatically an
   interference signature. This particular toy shows a 10-30%
   deviation; other setups might show more or less.

## 7. Verdict

**QX-F2 passes.** The path-integral formulation produces
well-defined transition amplitudes with measurable quantum
effects (shifted probabilities, reduced Var(C)) on the 2-context
toy. Interference is manifestly present and of physical
magnitude. QX-F3 (commutator check) is the last Phase A step.

## Related

- [[Quantum Xypher Formalism]] (QX-F1)
- [[Quantum Xypher Roadmap]]
- [[Xypher Lagrangian]]
- [[Noether Conservation]]
- [[Uncertainty Principle]]
- [[Physics]]
