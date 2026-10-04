---
title: Xypher Alpha and TAU Constitutive Result
aliases:
  - CAL-ALPHA-0 Result
  - Xypher Temperature Identifiability Result
tags:
  - domain/physics
  - type/result
  - type/experiment
  - topic/thermodynamics
  - topic/xypher
  - topic/alpha
  - topic/tau
domain: Physics
type: result
status: exact-family-passed
created: 2026-07-30
updated: 2026-07-30
td: td-10b593
related:
  - "[[Xypher Alpha and TAU Constitutive Boundary]]"
  - "[[Xypher Operational Thermodynamics Result]]"
  - "[[Thermodynamic Sensory Memory Xypher Result]]"
---

# Xypher Alpha and TAU constitutive result

## 0. Verdict

**The frozen CAL-ALPHA-0 exact family passed.** In all eight new reciprocal
micrograph cases, the independently declared finite reservoir relation gives

```text
T_R = lambda/ln(b),
```

and independently enumerated labelled channel rates identify one unique
kinetic compatibility temperature with

```text
T_LDB = T_R.
```

All thirteen admission gates passed. All nineteen preregistered controls
produced exactly their expected outcome at the first applicable gate. The
first confirmatory process exited successfully in 1.37 seconds; no fitting,
floating-point tolerance, random seed, trajectory horizon, or large artifact
was involved.

The result settles the narrow constitutive question as follows:

| Quantity | Result |
|---|---|
| independently declared `T_R` | **Operational temperature** in the frozen exact reservoir-coupled family |
| rate-derived `T_LDB` | **Unique**, and exactly equal to `T_R` in all eight cases |
| historical `a_P=N_acted/sum(Phi)` | **Gate statistic**, not temperature on present evidence |
| generic `p_o` | **Accounting-price interface**, not temperature without the independent boundary |
| `A(N)` and `p_N` | **Separate scaling hypotheses** |
| `TAU_+` | **Gross path receipt** |
| forward-minus-reverse TAU | **Signed entropy term** at one shared fixed price |
| departure-price `p*dS` fixture | **Nonintegrable edge form** |
| divided-difference fixture | **State-potential increment** |
| TAU as heat or work | **Unidentified** without separate channel ledgers |
| TAU receipt formula as equation of state | **Rejected** |

This is a positive exact result without a universal overclaim. The reviewed
C1--C9 proofs establish the general statements under their written
assumptions. The finite run validates their implementation and supplies
constructive witnesses; it does not prove the general theorem by enumeration.

## 1. Frozen chronology and provenance

The chronology remained clean:

1. The no-data boundary was independently reviewed, frozen, committed, and
   pushed at `262cb0c076c78c7cc381fd9db00db563bf341b3b`.
2. Its frozen file SHA-256 was
   `780e60cb3f08318edcb269718aea2e1ae7f98252bc143290b60264201cbe8859`.
3. The verifier was written afterward and received independent static reviews
   of G01--G07 and G08--G13.
4. Both reviews approved the same canonical source-tree digest:
   `54a6dc7f4d356f86727440e498e0512d0442dd987a60d7ceac45e1fc0ef3bd7c`.
5. The reviewed source was committed and pushed unchanged at
   `bedca80ad6c3732497549b55f94db25aa2dd5a73`.
6. Only then was `cargo run --quiet` executed for the first confirmatory run.

Run record:

```text
date:        2026-07-30 CEST
apparatus:   research/physics/xypher-alpha-tau-proof
source:      bedca80ad6c3732497549b55f94db25aa2dd5a73
command:     cargo run --quiet
exit:        0
wall time:   1.37 s
verdict:     OVERALL PASS
```

Post-result regression verification used the already frozen source:

```text
command:     cargo test
tests:       3 passed, 0 failed
evaluator:   1.39 s in the golden-report test
```

The inherited CAL-XTHERM `b=2` row was printed only as
`INHERITED_NON_EVALUATION`. It was not counted among the eight new cases.

## 2. Exact confirmatory family

The body energy indices and cycle were fixed as

```text
n = (0,0,1,3),
0 <-> 1 <-> 2 <-> 3 <-> 0.
```

The eight new cases were the complete cross product

```text
b      in {3,5},
lambda in {1,3},
g      in {(1,2,4,8), (1,3,5,11)}.
```

For each body state `x`, the verifier independently constructed the complete
fixed-energy fiber

```text
N_x = g(x)*b^[3-n(x)].
```

It lazily enumerated reciprocal unit microedges between adjacent fibers and
counted every outgoing hazard per microscopic origin. Strong lumpability then
gave the observed labelled macro hazards. Only after that count did the LDB
classifier form

```text
Delta S = ln[g(y)/g(x)],
Delta U = lambda*[n(y)-n(x)],
ell     = ln[k(x,y)/k(y,x)],
d       = Delta S - ell.
```

Each case returned the singleton compatibility set

```text
A_LDB = {T_LDB},
T_LDB = Delta U/d = lambda/ln(b) = T_R.
```

Changing the body degeneracy vector did not change temperature. Scaling the
energy unit by three co-scaled `U`, `T_R`, TAU, and Xi while leaving the
microscopic counts, rate ratios, and `Xi/T_R` invariant. Component energy
offsets were invisible, as required by the energy gauge.

## 3. Admission-gate results

| Gate | Result | Exact witness |
|---|---|---|
| G01 typed sources | **Pass** | 10 candidates; bare `alpha` is ambiguous and rejected |
| G02 reservoir slope | **Pass** | 4 reservoir laws; all 48 ordered nonzero finite differences |
| G03 micro-lumping | **Pass** | 8 cases; 64 labelled directed macro channels from reciprocal unit microedges |
| G04 LDB classification | **Pass** | 8 singleton sets; `T_LDB=T_R` exactly |
| G05 body invariance | **Pass** | 4 `(b,lambda)` groups, 2 body vectors each |
| G06 energy gauge | **Pass** | scale `3`; offsets `7,11`; rates and `Xi/T` invariant |
| G07 component intersection | **Pass** | all-positive, singleton `{1/ln(3)}`, and empty fixtures |
| G08 gross TAU path | **Pass** | closed `S=(0,1,0)` loop has receipts `(2,0,0)` and period `2` |
| G09 signed TAU pair | **Pass** | signed TAU `(2,-2,0)` and Xi `(-1,1,0)` both close |
| G10 variable integrability | **Pass** | source-price period `-6`; divided-difference period `0` |
| G11 heat/work nonidentity | **Pass** | `(Q,W_on)=(1,0)` and `(0,1)` share coarse TAU/Xi but remain distinct |
| G12 protocol-price scope | **Pass** | `a_P` gate statistic; `p_o` accounting interface |
| G13 equation-of-state scope | **Pass** | receipt consumes price; reservoir relation selects temperature |

## 4. Preregistered controls

A control marked **Pass** means the deliberately broken assertion was rejected
or received its frozen classification. It does not mean the mutation was
scientifically admitted.

| Control | Expected | Observed |
|---|---|---|
| C01 half reservoir temperature | fail G02 | **fail G02** |
| C02 inverse quantity labelled temperature | fail G01 | **fail G01** |
| C03 body-dependent temperature map | fail G05 | **fail G05** |
| C04 rescale `U` without `T_R` | fail G04 | **fail G04** |
| C05 flat-energy component | all positive at G07 | **all positive at G07** |
| C06 informative plus flat components | `{1/ln(3)}` at G07 | **`{1/ln(3)}` at G07** |
| C07 mixed component baths | empty at G07 | **empty at G07** |
| C08 nonlinear reservoir multiplicities | fail G02 | **fail G02** |
| C09 zero-energy ratio mismatch | fail G04 | **fail G04** |
| C10 inconsistent LDB slopes | fail G04 | **fail G04** |
| C11 aggregated-rate trap | fail G04 | **fail G04** |
| C12 global scaling plus component offsets | pass G06 | **pass G06** |
| C13 per-component energy scales claimed global | empty at G07 | **empty at G07** |
| C14 gross TAU claimed as state increment | fail G08 | **fail G08** |
| C15 unpaired TAU used in Xi | fail G09 | **fail G09** |
| C16 departure-state price claimed integrable | fail G10 | **fail G10** |
| C17 equal TAU/Xi used to conflate heat and work | fail G11 | **fail G11** |
| C18 `a_P` relabelled with a unit conversion | fail G12 | **fail G12** |
| C19 TAU receipt claimed as equation of state | fail G13 | **fail G13** |

The aggregation control is important. Its total forward and reverse rates are
both three, so the aggregated endpoint ratio is one. The two labelled
mechanisms separately have ratios two and one-half and correctly fail the
zero-energy LDB condition. Channel labels cannot be discarded.

## 5. What is established

### 5.1 Operational digital temperature exists in this family

The reservoir relation is declared before the rates:

```text
g_R(lambda*m)=b^m
    => S_R(lambda*m)=m*ln(b)
    => T_R=lambda/ln(b).
```

The reciprocal micrograph is then counted independently. Its rates return the
same temperature through LDB on zero-energy, unit-gap, multi-gap, and reverse
channels. This is not a fitted relabelling of activity as temperature.

### 5.2 Dynamics identify temperature only relative to an energy unit

The exact gauge passed:

```text
U' = c*U + offset_component,
T' = c*T.
```

Rate ratios identify the dimensionless field `U/T`, not joules and numerical
temperature separately. This is a physical identifiability boundary, not a
defect in the construction.

### 5.3 TAU has a precise but narrower role

The original positive-part formula is coherent as a gross expansion receipt:

```text
TAU_+ = p*max(0,Delta S).
```

It accumulates on a closed out-and-back path and is therefore not a state
function. With one shared fixed price, however,

```text
TAU_+(a)-TAU_+(reverse(a)) = p*Delta S.
```

That signed pair is antisymmetric. With an independently declared energy,

```text
Xi = T_R*Delta S - Delta U
```

is the exact energy-valued drive numerator and `Xi/T_R` is the dimensionless
affinity in the admitted LDB class.

### 5.4 The historical alpha symbols are not one discovered scalar

The run rejects bare-symbol equivalence. Reservoir temperature, inverse
temperature, rate compatibility, adaptive action history, accounting price,
network scaling, and kinetic activity must be distinguished by their declared
sources, units, and roles. A future theorem may connect some of them, but the
connection must be derived and tested rather than inherited from the name
`alpha`.

## 6. What is not established

This result does not establish:

- that `a_P=N_acted/sum(Phi)` is temperature;
- that an uninstrumented full Xypher endogenously creates its own reservoir
  relation or energy unit;
- a universal network-size equation of state or either historical scaling
  exponent;
- native THAIM or blockchain thermodynamics;
- TAU as conserved energy, heat, or work;
- a joule calibration;
- a closed self-powered NESS, finite-battery persistence, or finite-bath
  persistence;
- Landauer optimality;
- Thompson learning, intelligence, consciousness, or self-awareness;
- new fundamental physics.

The construction uses standard statistical-mechanical structure: logarithmic
multiplicity, reciprocal microscopic channels, and local detailed balance.
The Xypher-specific contribution is the exact constitutive separation and the
way Crystal entropy, reservoir temperature, reciprocal action, TAU, and Xi can
be organized without conflating their roles.

## 7. The next bounded frontier

The next question is not whether a digital thermodynamic graph can exist;
CAL-XTHERM and this result have closed that within their declared class. It is
also not whether arbitrary activity may be renamed temperature; this result
rejects that move.

The deeper remaining question is:

> Under what conditions can a full Xypher endogenously construct and maintain
> the energy--multiplicity relation that grounds its own operational
> temperature, while preserving independent work, heat, and information
> ledgers?

That is the scientifically defensible route toward the intuition that Xyphers
may illuminate the emergence of temperature. It requires a new frozen
boundary. CAL-SCALE remains separate for network-size laws, and CAL-DRIVE /
CAL-NESS remain separate for finite resources and native driven realization.
None should be silently folded into the exact result above.
