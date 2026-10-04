---
title: Xypher Alpha and TAU Constitutive Boundary
aliases:
  - CAL-ALPHA-0 Boundary
  - Xypher Temperature Identifiability Boundary
tags:
  - domain/physics
  - type/preregistration
  - topic/thermodynamics
  - topic/xypher
  - topic/alpha
  - topic/tau
domain: Physics
type: preregistration
status: frozen
created: 2026-07-30
updated: 2026-07-30
td: td-10b593
related:
  - "[[Xypher–Thermodynamic Graph-System Equivalence Boundary]]"
  - "[[Xypher Operational Thermodynamics Result]]"
  - "[[Thermodynamic Sensory Memory Xypher Boundary]]"
  - "[[Thermodynamic Sensory Memory Xypher Result]]"
---

# Xypher Alpha and TAU constitutive boundary

## 0. Status and chronology

This is the pre-apparatus boundary for CAL-ALPHA-0 (`td-10b593`). Its job is
to decide what can be proved exactly before any further native or
production-scale experiment.

The chronology requirement is strict:

1. independently review the definitions, theorems, confirmatory matrix,
   controls, and claim limits in this document;
2. change the status to `frozen`, commit, and push it;
3. only then write the new standalone verifier;
4. independently review and freeze the verifier source before its first
   confirmatory execution;
5. execute once and report every gate and control, including failures.

No CAL-ALPHA-0 verifier or evaluation artifact exists at this draft revision.
The already closed CAL-XTHERM and CAL-MEMORY artifacts may be read as inherited
evidence; they are not evaluation data for a new fitted map.

## 1. Question

The corpus has historically used `alpha` for several different quantities.
Shared notation is not a physical identity. This chapter asks:

1. Which quantity already earns the operational name temperature?
2. What can reciprocal dynamics identify when entropy and energy are declared
   independently?
3. What remains arbitrary because of the energy-unit gauge?
4. Is `TAU_+ = alpha*max(0,Delta S_tau)` a state function, heat, work, an
   equation of state, or a path receipt?
5. Under what exact condition can a variable price multiplying `dS_tau`
   integrate to a state potential?

This chapter does **not** re-test whether a thermodynamic Xypher exists. That
is closed by CAL-XTHERM. It also does not re-test whether bounded physical
memory can participate in an open digital NESS. That is closed by CAL-MEMORY.

## 2. Inherited evidence and nonclaims

### 2.1 Inherited facts

CAL-XTHERM establishes, within its fixed finite reciprocal-equilibrium class:

- an independently declared body energy `U` and Crystal entropy `S_tau`;
- a finite reservoir with `g_R(E)=2^E` and
  `T_R = 1/ln(2)` digital-energy units per nat;
- reciprocal labelled channels satisfying local detailed balance;
- exact equilibrium, relaxation, energy closure, and same-temperature
  contact;
- `TAU_+` as a one-sided expansion receipt;
- the forward-minus-reverse TAU identity and signed affinity
  `Xi = T_R*Delta S_tau - Delta U`.

CAL-MEMORY adds, within its frozen 64-state open driven construction:

- an energetic sensory bit;
- explicit thermal and work channels;
- stored mutual information and continuous information flow;
- causal use of the memory by reciprocal routing;
- exact work, heat, and entropy-production ledgers;
- a bounded reversible reset lift.

### 2.2 What is not inherited

Neither result identifies the historical adaptive gate statistic as
temperature, derives a universal network-size equation of state, calibrates
digital energy in joules, turns TAU into heat or work, establishes a closed
self-powered NESS, or validates native blockchain thermodynamics.

## 3. Typed quantities: one symbol split into distinct objects

All thermodynamic entropies below are in **nats**. A bits-valued Crystal
observable must be multiplied by `ln(2)` before it enters a natural-log local
detailed-balance equation. The corresponding price per bit is
`T_bit = T_R*ln(2)`.

| Symbol | Definition or role | Units | Status before this test |
|---|---|---|---|
| `T_R` | `(Delta S_R/Delta E_R)^(-1)` for a declared reservoir | energy/nat | operational temperature in CAL-XTHERM's scope |
| `beta_R` | `1/T_R` | nat/energy | inverse temperature, not temperature |
| `a_P` | `N_acted / sum_{i in acted} Phi_i`, defined only when the denominator is positive | dimensionless unless separately calibrated | adaptive gate/history statistic; not an admitted temperature |
| `p_o(z)` | candidate state-derived price for declared entropy observable `S^o` | work unit/nat_o | accounting interface, not yet a temperature |
| `A(N)` | candidate aggregate scaling observable | declared aggregate unit | empirical hypothesis |
| `p_N=A(N)/N` | corresponding per-unit scaling observable | aggregate unit/node | distinct empirical hypothesis |
| `kappa_a` | symmetric channel activity or opportunity clock | 1/time | kinetic speed, not temperature |
| `TAU_+(a)` | `p_o*max(0,Delta S^o_a)` for `p_o>=0` | work unit | one-sided gross event receipt |

Use bare `alpha` only as a typed placeholder when its source, observable,
units, log base, and role are stated. No equality among `T_R`, `a_P`, `p_o`,
`A(N)`, `p_N`, and `kappa` is inherited from the shared historical symbol.

The older `E_TGS^o(...)` equation-of-state contract is a useful admissibility
interface: it requires a replayable, state-derived, observable-matched price.
Those conditions are necessary for a non-governed accounting price but are
not sufficient for temperature. Temperature additionally requires an
independent energy/entropy boundary and reservoir or lawful contact evidence.

The corpus also contains two different scaling observables under the same
name. `A(N) proportional to N^(0.75..0.82)` is aggregate and increasing;
the Kleiber note's per-unit price has fitted exponents from about `-0.25` to
`-0.16` across its tested topologies.
They are not competing estimates of one scalar. CAL-SCALE remains relevant if
either scaling branch is revived, but it is not required to identify `T_R` in
the exact reservoir family below.

At `sum Phi_i=0`, `a_P` is undefined. A declared application bootstrap may
choose a gate behavior, but `a_P=0` at bootstrap is not a universal physical
zero-temperature statement. Two histories may also reach the same graph with
different counters and hence different `a_P`; it is graph-state-derived only
if those counters are included in the authoritative state.

## 4. Reciprocal-channel setting

Let each labelled channel `a:x->y` have a declared reverse `bar(a):y->x` and
strictly positive hazards under one common clock:

```text
k_a(x,y) > 0,
k_bar(a)(y,x) > 0.
```

Freeze independently of the rate ratios:

```text
Delta_a S = S(y) - S(x),
Delta_a U = U(y) - U(x),
rho_a     = k_a(x,y) / k_bar(a)(y,x),
ell_a     = ln(rho_a),
d_a       = Delta_a S - ell_a.
```

For an autonomous one-reservoir channel with zero external work, local
detailed balance is

```text
ell_a = Delta_a S - beta_R*Delta_a U,
beta_R = 1/T_R > 0,
```

or equivalently

```text
d_a = beta_R*Delta_a U.
```

The hazards must remain channel resolved. Aggregating different labelled
mechanisms with the same endpoints can hide a failed reverse pairing and is
not admissible evidence for this theorem.

## 5. Constitutive theorem ladder

### C1. Independent finite-reservoir temperature

Suppose the body receives heat `q` and a declared reservoir changes from
`E_R` to `E_R-q`. A single constant reservoir temperature exists on the
allowed exchange graph exactly when

```text
S_R(E_R-q) - S_R(E_R) = -q/T_R
```

for every allowed `E_R` and `q`. Equivalently, every nonzero finite-difference
slope agrees:

```text
beta_R = [S_R(E+q)-S_R(E)]/q.
```

On each connected reservoir-energy component, `S_R(E)` must therefore be
affine with common slope `beta_R`. A finite reservoir with nonlinear entropy
has a state-dependent finite-difference temperature, not one global `T_R`.

For an energy quantum `lambda>0` and multiplicity

```text
g_R(lambda*m) = b^m,   b >= 2 integer,
```

the independently declared relation is

```text
S_R(lambda*m) = m*ln(b),
beta_R = ln(b)/lambda,
T_R = lambda/ln(b).
```

The reservoir relation must be evaluated before the system channel ratios.
A slope inferred from those ratios alone is `beta_LDB`, not yet an
independently grounded reservoir temperature.

### C2. Necessary-and-sufficient LDB identifiability

Define the rate-compatible positive finite-temperature set

```text
A_LDB = {T>0 : T*d_a = Delta_a U for every reciprocal channel a}.
```

Exactly one of three classifications occurs.

#### Unique

`A_LDB={T_LDB}` exactly when at least one channel has `Delta U != 0` and:

1. every zero-energy channel has `d_a=0`;
2. every informative channel has `d_a*Delta_a U>0`;
3. all informative channels share one slope:

```text
d_a*Delta_b U = d_b*Delta_a U.
```

Then every informative channel gives the same value

```text
beta_LDB = d_a/Delta_a U,
T_LDB    = Delta_a U/d_a.
```

This is a kinetic compatibility value, not yet the reservoir temperature.
The operational identity is the separately falsifiable equality

```text
T_LDB = T_R,
```

where `T_R` was derived from C1 before the channel ratios were inspected.

One informative edge is algebraically sufficient for uniqueness but gives no
overidentifying test. The confirmatory family therefore includes zero-energy
edges and multiple nonzero energy gaps.

#### Compatible but unidentified

If `Delta U=0` and `d=0` on every channel, then
`A_LDB=(0,infinity)`. The rates
validate `ell=Delta S` but contain no temperature information. A verifier must
return `COMPATIBLE_UNIDENTIFIED`, never manufacture an estimate.

#### Incompatible

All other cases give `A_LDB=empty`. This includes a nonzero `d` on a
zero-energy edge, a zero `d` on a nonzero-energy edge, inconsistent slopes,
and a common negative slope outside the declared positive-temperature class.

Finite positive reciprocal support cannot identify zero temperature. The
singular `beta=infinity` boundary requires vanishing reverse support and lies
outside this theorem.

**Proof.** The LDB constraints are the linear equations
`d_a=beta_LDB*Delta_a U`. If one energy-changing edge exists, it fixes the
only possible slope; the sign and cross-product conditions are exactly the
conditions that every other equation accepts that same positive value.
Zero-energy equations reduce to `d_a=0`. If every energy difference is zero,
the system is either inconsistent because some `d_a!=0`, or every positive
temperature is compatible. These exhaust the cases.

### C3. Disconnected components

Compute a compatibility set `A_LDB,j` for each reciprocal-support component. One
global bath exists algebraically exactly when

```text
intersection_j A_LDB,j != empty.
```

It is uniquely identified only if that intersection is compatible and at
least one component contains an informative energy-changing edge. Energy-flat
components can test consistency but not temperature. Two informative
components requiring different slopes reject a single global bath even if
each component separately admits a temperature.

Without an independently declared common energy unit or contact relation,
separate components may absorb separate scale choices and cannot establish
temperature equality merely by using the same symbol.

### C4. Energy-unit gauge

For any global `lambda>0` and component constants `c_C`,

```text
U'(x) = lambda*U(x) + c_C,
T_R'  = lambda*T_R
```

leaves every LDB ratio invariant because

```text
Delta U'/T_R' = Delta U/T_R.
```

Conversely, if the same `S` and reciprocal ratios admit both `(U,T_R)` and
`(U',T_R')`, then on every support component

```text
U' = (T_R'/T_R)*U + c_C.
```

Thus dynamics identify the dimensionless field `beta_R*U`, up to component
constants. They do not identify an absolute energy scale and numerical
temperature separately. If `U=lambda*n`, the rates identify
`theta=beta_R*lambda`, while joules require an independent calibration of
`lambda`.

Under the same global units change,

```text
Xi'      = lambda*Xi,
TAU_+'   = lambda*TAU_+,
Xi'/T_R' = Xi/T_R.
```

No TAU receipt can calibrate the energy unit when its own unit was introduced
through the same price. A fixture-specific scale or post-hoc monotone map is
an invalid rescue.

**Proof.** Substitution proves the forward invariance. Conversely, equality
of the two dimensionless energy differences on every support edge gives
`Delta[U'/T_R'-U/T_R]=0`. A function with zero edge difference is constant on
each connected component, which yields the stated affine relation.

### C5. TAU is a gross transition receipt

For a reciprocal edge at one frozen `T_R`, let

```text
R_xy = TAU_+(x->y) = T_R*max(0,S(y)-S(x)).
```

It is neither a state-function difference nor an equation of state.

A state differential must be antisymmetric. If `Delta S != 0`, both
`R_xy` and `R_yx` are nonnegative and

```text
R_xy + R_yx = T_R*abs(Delta S) > 0.
```

An out-and-back path therefore returns to the same state with a positive gross
receipt. On any closed cycle `C`,

```text
sum_C R
  = T_R*sum_C max(0,Delta S)
  = (T_R/2)*sum_C abs(Delta S),
```

which is positive unless `S` is constant around the cycle. Repeating a loop
can accumulate unlimited gross TAU. That can be a ledger policy, but the
issuance and counter-reservoir must then be explicit state; the receipt itself
is not conserved energy.

The formula also consumes `T_R` or `p_o` as an input. It does not determine
that price, so it is not an equation of state.

**Proof.** On a closed path, `sum Delta S=0`, so the sum of positive entropy
increments equals half the total variation. The two-edge out-and-back path is
already a counterexample whenever `Delta S!=0`.

### C6. The reciprocal TAU pair recovers an exact signed term

When the same frozen price applies to both orientations,

```text
R_xy - R_yx = T_R*[S(y)-S(x)],
R_xy + R_yx = T_R*abs[S(y)-S(x)].
```

At constant `T_R`, the signed difference is exact:

```text
omega = T_R*dS = d(T_R*S).
```

With independently declared `U`,

```text
Xi = T_R*Delta S - Delta U
   = -Delta F_T,
F_T = U - T_R*S.
```

Therefore the energy-valued drive numerator `Xi` is antisymmetric and has zero
cycle sum in this fixed-temperature state-function class; `Xi/T_R` is the
dimensionless affinity. This algebra does not by itself establish
LDB: reciprocal support, channel ratios, symmetric activity, the independent
reservoir, and accounting closure remain separate requirements.

If forward and reverse recompute different prices, the simple TAU-pair
identity is unavailable. A protocol must freeze whether it uses a shared bath
price, a symmetric edge price, or a destination-recomputed price.

**Proof.** The scalar identity `max(0,d)-max(0,-d)=d` gives the signed pair.
At constant `T_R`, both `T_R*Delta S` and `Delta U` telescope around every
cycle, establishing exactness and antisymmetry.

### C7. Variable-price integrability

For a symmetric edge price `p_xy=p_yx`, define the antisymmetric edge form

```text
omega_xy = p_xy*[S(y)-S(x)].
```

A state potential exists exactly when

```text
sum_C p_e*Delta S_e = 0
```

for every cycle. Checking a fundamental-cycle basis is sufficient.

In a continuous state space,

```text
omega = p*dS,
d(omega) = dp wedge dS.
```

Local exactness requires `dp wedge dS=0`; on a simply connected domain,
vanishing periods make this sufficient. Locally this normally means
`p=f(S)`, with a potential `A(S)` satisfying `A'(S)=f(S)`.

For finite steps, departure-point multiplication `p(x)*Delta S` is not the
exact integral. The exact symmetric edge coefficient is the divided
difference

```text
p_xy = [A(S(y))-A(S(x))]/[S(y)-S(x)]
```

when `S(y)!=S(x)`. A variable state price that fails the cycle test is a
`NONINTEGRABLE_EDGE_FORM`. It may be called a nonconservative physical
affinity only after a kinetic/path-ratio relation and the drive or reservoirs
producing it are independently named and accounted.

**Proof.** An antisymmetric edge form is a graph gradient exactly when all of
its periods vanish; a spanning tree plus its fundamental chords supplies a
cycle basis. In the smooth limit, exterior differentiation gives
`d(p*dS)=dp wedge dS`. The divided difference is the exact finite increment
of the proposed potential and therefore telescopes.

### C8. TAU does not identify heat or work

Use the sign convention

```text
Delta U = Q + W_on,
```

where `Q` is heat into the system and `W_on` is external work done on it. For
an ideal bath at `T_R`, total entropy production is

```text
Sigma = Delta S - Q/T_R.
```

Then

```text
Xi = T_R*Delta S - Delta U = T_R*Sigma - W_on.
```

Consequently:

- with zero external work, `Xi=T_R*Sigma`;
- on a reversible path, `T_R*Delta S=Q` and `Xi=-W_on`;
- in general, neither gross TAU nor signed `T_R*Delta S` is automatically
  heat;
- `Delta U` is not automatically work required;
- the same `(Delta S,Delta U,TAU,Xi)` can admit different heat/work splits
  unless the channel and reservoir ledgers name them independently.

In CAL-XTHERM, `Delta U` is heat into the body and external work is zero. In
CAL-MEMORY, heat and work are channel resolved. Those scopes must not be
merged by notation.

### C9. Equation-of-state distinction

An equation of state is an independent relation that selects a state
variable, for example

```text
S_R(lambda*m)=m*ln(b)
    => T_R=lambda/ln(b).
```

By contrast,

```text
TAU_+ = p_o*max(0,Delta S^o)
```

is conditional accounting after `p_o` has been supplied. The same transition
satisfies the receipt formula for infinitely many prices and different receipt
values. The formula cannot select one and therefore cannot be an equation of
state.

## 6. Exact combinatorial realization

When

```text
S(x)=ln(g(x)),
U(x)=lambda*n(x),
b_R=exp(lambda/T_R),
```

local detailed balance can be checked without floating logarithms. Define

```text
C_a = g(y)*k_bar(a)(y,x) / [g(x)*k_a(x,y)].
```

Then LDB is exactly

```text
C_a = b_R^[n(y)-n(x)].
```

Signed integer powers are compared by rational cross multiplication. A
zero-energy channel must have `C_a=1`; informative channel pairs must satisfy

```text
C_a^[Delta_b n] = C_b^[Delta_a n].
```

The independently declared reservoir supplies `b_R`; the rates do not choose
it.

### 6.1 Micrograph construction

For each confirmatory case, freeze:

```text
body energy indices n = (0,0,1,3),
E_total index         = 3,
body degeneracies     = one declared g-vector,
reservoir base        = b,
energy quantum        = lambda.
```

The complete fixed-energy fiber over body state `x` has size

```text
N_x = g(x)*b^[3-n(x)].
```

Connect every microscopic state in adjacent cycle fibers with reciprocal unit
hazard. Exact lumping gives

```text
k(x->y)=N_y,
k(y->x)=N_x,
k(x->y)/k(y->x)
  = [g(y)/g(x)]*b^-[n(y)-n(x)].
```

This derives the channel ratios by independently counting reciprocal
microedges; it does not insert the target LDB ratio into a fitted rate law.

### 6.2 Exact representation

The verifier must not approximate logarithms. It uses:

- positive rationals as reduced integer numerator/denominator pairs;
- `LogMonomial(q)` as the exact prime-exponent vector of a positive rational
  `q`, so addition of logarithms is multiplication of rationals;
- `Temperature(lambda,b)` as the unevaluated token `lambda/ln(b)`;
- `ScaledLog(lambda,b,q)` as the unevaluated energy-valued token
  `lambda*ln(q)/ln(b)`.

Equality, sign, addition, and cancellation are decided from the rational and
prime-exponent representations. Equality of two temperature tokens clears
rational `lambda` denominators and reduces to exact integer-power equality
`b_2^lambda_1=b_1^lambda_2`. Energy-valued scaled logs are compared only
inside one identical temperature token; no gate asks the verifier to decide an
unsupported equality between unrelated ratios of logarithms. Variable-price
and heat/work controls use ordinary exact rationals. No binary floating-point
approximation participates in an admission gate.

### 6.3 Calibration and small confirmatory factorial

The existing CAL-XTHERM case is a calibration row only:

```text
b=2, lambda=1, g=(1,4,4), n=(0,1,2),
T_R=1/ln(2), fibers=(4,8,4), rates=(8,4,4,8).
```

The new confirmatory matrix is the full `2x2x2` cross product

```text
b      in {3,5},
lambda in {1,3},
g      in {(1,2,4,8), (1,3,5,11)},
n      = (0,0,1,3).
```

It contains eight exact cases. Together with the inherited `b=2` calibration
row, it exercises three distinct reservoir bases without fitting. Before any
execution, the predictions are:

```text
T_R = lambda/ln(b),
beta_R = ln(b)/lambda,
N_x = g(x)*b^[3-n(x)],
k(x->y)=N_y on the declared four-cycle,
rho_xy=[g(y)/g(x)]*b^-[n(y)-n(x)].
```

Changing `g` at fixed `(b,lambda)` must not change `T_R`. Changing `lambda`
must scale `U`, `T_R`, `TAU`, and `Xi` together while leaving every fiber,
rate, equilibrium ratio, and dimensionless affinity unchanged. Changing `b`
must change `T_R` exactly as `lambda/ln(b)`.

This matrix validates only the narrow reservoir identity. It does not identify
`a_P`, prove a native scaling law, or calibrate joules.

## 7. Admission gates

The verifier must print every gate in this order.

| Gate | Requirement |
|---|---|
| `G01_TYPED_SOURCES` | Every candidate names its symbol, source, observable, log base, units, and role; bare-symbol equality is rejected. |
| `G02_RESERVOIR_SLOPE` | Every allowed reservoir finite difference gives the predeclared `beta_R=ln(b)/lambda`. |
| `G03_MICRO_LUMPING` | Reciprocal unit microedge counts independently lump to the predicted macro hazards. |
| `G04_LDB_CLASSIFICATION` | Each confirmatory case returns `A_LDB={T_LDB}` from channel-resolved zero and nonzero energy-gap checks, and `T_LDB` exactly equals the independently predeclared `T_R=lambda/ln(b)`. Aggregated endpoint rates cannot substitute for labelled channels. |
| `G05_BODY_INVARIANCE` | The two body degeneracy vectors at fixed `(b,lambda)` give the same temperature. |
| `G06_ENERGY_GAUGE` | Global energy rescaling co-transforms `T_R`, TAU, and Xi and leaves all dimensionless dynamics invariant; component offsets are invisible. |
| `G07_COMPONENT_INTERSECTION` | Literal energy-flat and disconnected controls return the exact empty, singleton, or all-positive `A_LDB` intersection. |
| `G08_TAU_GROSS_PATH` | With `T_R=2` and cycle entropies `(0,1,0)`, gross receipts are `(2,0,0)`: the loop closes in state but accumulates `2`. |
| `G09_TAU_SIGNED_PAIR` | At one shared fixed price, forward-minus-reverse TAU equals `T_R*Delta S`; `Xi=R_forward-R_reverse-Delta U` is antisymmetric; both signed forms have zero cycle sums. |
| `G10_VARIABLE_INTEGRABILITY` | On the exact `S=(0,1,2)` triangle, departure price `p=2S+1` has period `1+3-10=-6`, while divided differences of `A=S^2+S` have period `2+4-6=0`. |
| `G11_HEAT_WORK_NONIDENTITY` | Equal `(Delta S,Delta U,TAU,Xi)` with two declared heat/work splits remains distinguishable by channel ledgers. |
| `G12_PROTOCOL_PRICE_SCOPE` | `a_P` and `p_o` remain gate/accounting prices unless they independently satisfy the temperature boundary. |
| `G13_EOS_SCOPE` | The receipt formula is classified as conditional accounting and the reservoir relation as the equation selecting `T_R`. |

No statistical tolerance, fit, seed, trajectory length, or monotone transform is
allowed. Exact integer/rational identities decide every gate; symbolic
`lambda/ln(b)` labels carry the temperature value.

## 8. Literal controls and expected first failures

Controls are typed lanes. Every lane runs `G01`; after that, only gates owning
that lane's declared input run, and all others print `N/A`. A cross-case or
gauge lane imports the immutable identities of the passing primary reservoir
and channel facts rather than mutating and rerunning them. “First failure”
means the first **applicable** gate in the published order. This prevents a
later conceptual control from accidentally failing an unrelated parser or
from hiding an earlier scientific defect.

| Control | Frozen mutation | Expected first failure or classification |
|---|---|---|
| `C01_HALF_RESERVOIR_T` | Declare `T_R=lambda/[2ln(b)]` against unchanged `g_R(lambda*m)=b^m` | `G02_RESERVOIR_SLOPE` |
| `C02_INVERSE_LABEL` | Label `beta_R=ln(b)/lambda` as temperature without inverting its units | `G01_TYPED_SOURCES` |
| `C03_BODY_DEPENDENT_MAP` | On the cross-case lane, propose `T_hat(g)=g(1)*T_R`; `g(1)` is `2` or `3` | `G05_BODY_INVARIANCE` |
| `C04_SCALE_U_ONLY` | On a channel lane, send `U->3U` while holding `T_R` fixed | `G04_LDB_CLASSIFICATION` |
| `C05_FLAT_ENERGY_ALL` | Use fixture `D_ALL` below | `G07_COMPONENT_INTERSECTION`: `(0,infinity)` |
| `C06_INFORMATIVE_PLUS_FLAT` | Use fixture `D_SINGLETON` below | `G07_COMPONENT_INTERSECTION`: `{1/ln(3)}` |
| `C07_MIXED_COMPONENT_BATHS` | Use fixture `D_EMPTY` below | `G07_COMPONENT_INTERSECTION`: empty |
| `C08_NONLINEAR_RESERVOIR` | Use reservoir multiplicities `(1,2,8)` at energy indices `(0,1,2)` | `G02_RESERVOIR_SLOPE` |
| `C09_ZERO_ENERGY_RATIO` | Set `S=(0,0)`, `U=(0,0)`, and `rho=2` | `G04_LDB_CLASSIFICATION`: incompatible |
| `C10_INCONSISTENT_SLOPES` | On one three-state component set `S=(0,0,0)`, `U=(0,1,2)`, and successive ratios `(1/3,1/5)` | `G04_LDB_CLASSIFICATION`: incompatible |
| `C11_AGGREGATION_TRAP` | Two parallel pairs have forward rates `(2,1)` and reverse rates `(1,2)` at `Delta S=Delta U=0`; aggregate ratio is `1`, labelled ratios are `2` and `1/2` | `G04_LDB_CLASSIFICATION` |
| `C12_COMPONENT_OFFSET_GAUGE` | Send `U->3U+c_C` and `T_R->3T_R` with component offsets `(7,11)` | `G06_ENERGY_GAUGE`: pass unchanged |
| `C13_PER_COMPONENT_SCALE` | Start with two `b=3`, `lambda=1` informative components; scale their energy maps by `2` and `3` and their local temperatures accordingly while claiming one common temperature/unit map | `G07_COMPONENT_INTERSECTION`: `{2/ln(3)} intersect {3/ln(3)}=empty` |
| `C14_TAU_AS_STATE` | Assert the frozen gross `(2,0,0)` loop is a potential increment | `G08_TAU_GROSS_PATH` |
| `C15_UNPAIRED_TAU_XI` | Use `Xi_bad=TAU_+(a)-Delta U`; then `Xi_bad(a)+Xi_bad(bar(a))=T_R*abs(Delta S)>0` | `G09_TAU_SIGNED_PAIR` |
| `C16_SOURCE_PRICE` | Use departure-state `p(x)*Delta S` with `S=(0,1,2)` and `p=2S+1` | `G10_VARIABLE_INTEGRABILITY` |
| `C17_HEAT_WORK_CONFLATION` | Declare the two frozen heat/work ledgers below identical because TAU and Xi agree | `G11_HEAT_WORK_NONIDENTITY` |
| `C18_RELABELED_GATE` | Set `a_P=1/2` and attach a declared unit conversion of `1 energy/nat per gate-unit`, but supply no reservoir, LDB, or contact relation | `G12_PROTOCOL_PRICE_SCOPE` |
| `C19_TAU_IS_EOS` | Claim the receipt formula determines its own price | `G13_EOS_SCOPE` |

The component fixtures use one common energy quantum:

```text
D_ALL:
  one flat component with S=(0,ln(2)), U=(0,0), rho=2
  A_LDB=(0,infinity)

D_SINGLETON:
  component I: S=(0,0), U=(0,1), rho=1/3
               A_LDB,I={1/ln(3)}
  component F: the D_ALL flat component
               A_LDB,F=(0,infinity)
  intersection={1/ln(3)}

D_EMPTY:
  component I3: S=(0,0), U=(0,1), rho=1/3
                A_LDB,I3={1/ln(3)}
  component I5: S=(0,0), U=(0,1), rho=1/5
                A_LDB,I5={1/ln(5)}
  intersection=empty
```

The literal gross/signed control uses one shared `T_R=2` and

```text
A: (S,U)=(0,0)
B: (S,U)=(1,3)
C: (S,U)=(0,0)
cycle A->B->C->A

gross TAU_+ = (2,0,0),       sum=2
signed TAU  = (2,-2,0),      sum=0
Xi          = (-1,1,0),      sum=0
```

For `C15`, omitting the reverse receipt gives `Xi_bad(A->B)=-1` and
`Xi_bad(B->A)=3`, so the reciprocal sum is `2=T_R*abs(Delta S)` rather
than zero.

The positive variable-price control uses

```text
p(S)=2S+1,
A(S)=S^2+S,
p_xy=[A(S_y)-A(S_x)]/[S_y-S_x].
```

On the oriented `S=(0,1,2)` triangle, the divided-difference increments are
`(2,4,-6)` and sum to zero. Departure-point multiplication gives
`(1,3,-10)` and sums to `-6`.

The heat/work nonidentity control fixes

```text
T_R=1, Delta S=1, Delta U=1, TAU_signed=1, Xi=0
```

and compares the distinct ledgers

```text
(Q,W_on)=(1,0),
(Q,W_on)=(0,1).
```

The shared TAU and Xi values cannot choose between them.

## 9. Verdict rules

The result must classify the candidates separately.

| Candidate | Allowed verdicts |
|---|---|
| rate-derived `T_LDB` | `UNIQUE`, `COMPATIBLE_UNIDENTIFIED`, or `INCOMPATIBLE` from channel algebra |
| independently declared `T_R` | `OPERATIONAL_TEMPERATURE` only when the reservoir relation passes and unique `T_LDB=T_R`; otherwise the exact failure is retained |
| `a_P` | `GATE_STATISTIC`; temperature only after a separate global calibration and operational test |
| `p_o` | `ACCOUNTING_PRICE_INTERFACE`; temperature only after the same independent boundary |
| `A(N)`, `p_N` | `SCALING_HYPOTHESIS`; aggregate and per-unit claims remain separate |
| `TAU_+` | `GROSS_PATH_RECEIPT` |
| forward-minus-reverse TAU | `SIGNED_ENTROPY_TERM` under one shared fixed price |
| variable `p*dS` | `STATE_POTENTIAL` iff every cycle period vanishes; otherwise `NONINTEGRABLE_EDGE_FORM` unless a separate kinetic/reservoir boundary admits a physical affinity |
| TAU as heat or work | `UNIDENTIFIED` without separate channel/reservoir ledgers |

Independent mathematical review of C1--C9 establishes the general statements
under their written assumptions. Passing the finite verifier would validate
the implementation of those statements on the frozen matrix and controls and
would supply exact constructive witnesses. A finite case matrix does not prove
the general theorem and is not empirical evidence that an uninstrumented
native network spontaneously realizes the construction.

Together, the reviewed proof and passing exact witnesses would establish:

1. an exact theorem and independent combinatorial family for when reciprocal
   dynamics uniquely identify a declared reservoir temperature;
2. the unavoidable global energy-unit gauge;
3. the exact receipt, signed-pair, and integrability roles of TAU;
4. a canonical separation between reservoir temperature and historical
   protocol/accounting prices.

It would **not** establish endogenous full-Xypher temperature, a native
blockchain equation of state, universal network scaling, TAU as physical heat
or work, joule calibration, Landauer optimality, self-power, intelligence,
consciousness, or new fundamental physics.

## 10. Consequences for the remaining frontier

CAL-ALPHA-0 is deliberately narrower than the existing broad CAL-ALPHA task
`td-3a581b`.

- CAL-SCALE remains relevant only to the separate aggregate/per-unit network
  scaling candidates.
- CAL-NESS and CAL-KINETICS remain later native-realization questions; they
  are not prerequisites for the exact reservoir identity.
- CAL-MEMORY supplies one engineered ideal-reservoir NESS witness. CAL-DRIVE
  still owns broader source/sink/battery/bath accounting, finite-time
  fluctuation and drive-removal tests, and broad TAU classification. None is
  required for the narrow `T_R` identity. Its smallest meaningful next
  extension is a finite battery and then finite bath, predicting a depletion
  transient rather than a stationary self-powered NESS.
- No large CAL-CONTACT generation is required. CAL-COMPAT already rejected
  native `Q_H` energy LDB/contact and passed the engineered contact control;
  CAL-XTHERM supplies the valid minimal native witness.

Only after this theorem and exact family are adjudicated should the broad
CAL-ALPHA dependency graph be narrowed. A failed candidate may be classified
`INCOMPATIBLE` or `UNIDENTIFIED`; it may not be rescued by renaming a gate
statistic, refitting units, or importing an unrelated scaling exponent.

Unless `td-3a581b` is separately narrowed, CAL-SCALE, CAL-NESS, and CAL-DRIVE
remain prerequisites for its broad native/scaling/driven acceptance. They are
not prerequisites for CAL-ALPHA-0's reservoir-identifiability theorem.

## 11. External standard checked

The sign conventions in this document are declared locally. The separation
of mesostate entropy, energy, heat, work, reciprocal transition ratios, and
trajectory entropy production was checked against:

- [Christian Maes, “Local detailed balance”](https://arxiv.org/abs/2011.09200)
- [Udo Seifert, “Stochastic thermodynamics: From principles to the cost of precision”](https://arxiv.org/abs/1707.03759)
- [Udo Seifert, “Stochastic thermodynamics, fluctuation theorems, and molecular machines”](https://arxiv.org/abs/1205.4176)
- [André C. Barato and Udo Seifert, “Stochastic thermodynamics with information reservoirs”](https://arxiv.org/abs/1408.1224)

These sources support the operational distinctions; the Xypher-specific
identifiability, TAU-pair, and exact combinatorial statements are proved in
this boundary rather than attributed to those papers.
