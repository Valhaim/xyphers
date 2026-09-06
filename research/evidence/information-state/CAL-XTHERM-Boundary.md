---
title: Xypher–Thermodynamic Graph-System Equivalence Boundary
aliases:
  - CAL-XTHERM Boundary
  - Thermodynamic Xypher Existence Boundary
tags:
  - domain/physics
  - type/preregistration
  - topic/thermodynamics
  - topic/xypher
  - topic/theorem
domain: Physics
type: preregistration
status: draft-no-evaluation
td: td-72a601
created: 2026-07-29
updated: 2026-07-29
---

# Xypher–thermodynamic graph-system equivalence boundary

## 0. The landing point

This chapter asks exactly two questions.

1. **Existence.** Is there at least one fixed finite Crystal + Thermo +
   Praxion construction that is an operational thermodynamic digital graph
   system?
2. **Generality.** What independently checkable conditions make such a
   Xypher thermodynamic, and does every finite equilibrium thermodynamic graph
   system admit the same minimal Xypher-kernel factorization?

The first-round object has an empty Ruby slot. Graph growth, multiple
Praxions, infinite-horizon optimization, blockchain realization, consensus,
money, consciousness, and deployment are outside this boundary. They are not
allowed to rescue or weaken the two claims above.

The proof target is deliberately finite and exact. No long stochastic run is
needed. Simulation may illustrate the result but cannot establish it.

No verifier output, trajectory, or evaluated result has been generated under
this version. The document remains a draft until its exact commit is
independently reviewed and pushed.

## 1. The semantic correction

The three Xypher roles and thermodynamic behavior are defined independently.
This prevents the conclusion from being hidden in a definition.

- A **thermodynamic graph system** is defined by its complete-state dynamics,
  accounting, reciprocity, equilibrium law, and lawful contact.
- A **Xypher** is defined by a decomposition: a Crystal constructs the
  future-freedom observable, Thermo prices its change and closes energy, and a
  Praxion turns candidate graph channels into actions.

The theorem will not say that every object called a Xypher is automatically
thermodynamic. It will characterize the **minimal thermodynamic Xypher
kernels**.
Conversely, it will show that every system in the declared finite equilibrium
thermodynamic class can be represented by those three Xypher roles.

This boundary uses the smallest scientific core:

```text
X0 = Crystal + Thermo + Praxion
```

Ruby is empty. Memory, resolution, and thaw may be added as ordinary complete
state coordinates and reciprocal channels, but adaptive learning is not part
of the equivalence claim. If learning is made constitutive of the word
"Xypher", then the reverse claim is false: many thermodynamic systems do not
learn. In that vocabulary, the theorem is an equivalence with a **Xypher
kernel**, and adaptive Xyphers are a stricter extension.

## 2. Independent definition: finite equilibrium thermodynamic graph system

Let `Z` be a finite complete mesostate space. Let `a: z -> z'` be a labelled
continuous-time transition channel and `bar(a): z' -> z` its reverse. A finite
equilibrium thermodynamic graph system is the tuple

```text
TGS = (Z, A, bar, k, S, U, alpha)
```

with the following operational requirements.

### T1. Complete state and closed rows

`Z` contains every coordinate read or changed by the dynamics. The generator
has nonnegative off-diagonal channel hazards and diagonal row closure. Every
claimed projection is proved strongly lumpable; otherwise it is only an
observation, not a Markov state.

A mesostate may quotient dynamically invisible micro-labels only when an
explicit microscopic lift is supplied and the partition is strongly
lumpable. The mesostate is then operationally complete because the discarded
labels cannot alter any future mesostate hazard.

### T2. Independent thermodynamic coordinates

Before the rates are inspected, the construction declares:

- an energy state function `U: Z -> energy units`;
- a positive integer multiplicity `g(z)` grounded by an explicit digital
  microstate or causal-endpoint construction, with
  `S(z) = ln g(z)` up to one common additive constant; and
- a positive `alpha` in energy per nat, derived from a named reservoir or
  equation of state.

Defining `U`, `S`, or `alpha` from the measured rate ratio fails T2.

### T3. Reciprocal support and local detailed balance

Every positive channel has a positive reverse channel and obeys

```text
ln[k_a(z,z') / k_bar(a)(z',z)]
    = S(z') - S(z) - [U(z') - U(z)] / alpha.       (LDB)
```

The right-hand side is the total entropy change in nats for a one-temperature
exchange when the system energy change is supplied by the reservoir.

### T4. Pathwise accounting

Each transition names the signed system-energy change, reservoir-energy
change, heat, and externally supplied work. In the undriven witness,

```text
Delta U_system + Delta U_reservoir = 0,
work_on_system = 0.
```

An information register, receipt, or ledger balance is part of the complete
state if it affects future behavior. Its reset or creation must then have a
named counter-reservoir.

### T5. Equilibrium and relaxation

On every connected recurrent class, the independently predicted law is

```text
pi(z) = exp[S(z) - U(z)/alpha] / Z_partition.      (Gibbs)
```

It is unique when the class is irreducible. LDB implies channel detailed
balance and therefore stationarity; irreducibility gives convergence.

### T6. Temperature through contact

`alpha` earns the name temperature only through a reservoir or contact law,
not by dimensional analogy. For two systems at the same independently
declared `alpha`, entropy and energy compose additively and reciprocal
exchange channels obey LDB for the combined state. Their conditioned Gibbs
law is stationary and their equilibrium expected energy current is zero.

For finite bodies, the exact combined entropy difference controls an exchange.
The slogan "heat always flows from hot to cold" is not used as a substitute
for that calculation.

T1--T6 define the declared target class. None mentions Crystal, Thermo,
Praxion, TAU, or Xi.

## 3. Independent definition: minimal thermodynamic Xypher kernel

A fixed finite minimal thermodynamic Xypher kernel is

```text
X = (G, P, C, Theta, Pi)
```

where `G` is the replayable graph substrate, `P` is its fixed perspective,
`C` is Crystal, `Theta` is Thermo, and `Pi` is Praxion.

### X1. Crystal

Crystal supplies a replayable typed one-step endpoint channel

```text
R_C: z -> {p_(z,1), ..., p_(z,g(z))},
R_C(z,p_(z,i)) = 1/g(z).
```

The ports are executable causal outcomes in the declared perspective, not
decorative labels. The endpoint channel is row-stochastic and complete. Its
horizon is `tau = 1`, so

```text
S_tau(z) = H(R_C(z,.)) = ln g(z).
```

This first theorem is restricted to such combinatorial Crystals. Nonuniform
future kernels may be useful Xyphers, but their Shannon entropy is not
automatically a thermodynamic state-counting entropy. The perspective,
endpoint alphabet, executable resolution rule, and probabilities are frozen
before the Praxion transition law is built.

### X2. Thermo

Thermo supplies independently declared `U` and `alpha` and reports two
different quantities that must not be conflated:

```text
TAU_plus(a) = alpha * max(0, Delta S_tau(a))        (gross receipt)

Xi(a) = alpha * Delta S_tau(a) - Delta U(a)         (signed affinity energy)
```

Thus

```text
Xi(a) / alpha = Delta S_tau(a) - Delta U(a)/alpha.
```

`TAU_plus` is the minimal expansion harness. It records positive creation of
the declared future-freedom observable. It is not the reversible transition
affinity because the positive-part operation destroys antisymmetry:

```text
TAU_plus(bar(a)) != -TAU_plus(a).
```

It nevertheless contains the full signed entropic term when a channel and its
declared reverse are considered together. The identity

```text
max(0,d) - max(0,-d) = d
```

gives

```text
TAU_plus(a) - TAU_plus(bar(a)) = alpha * Delta S_tau(a),

Xi(a) = TAU_plus(a) - TAU_plus(bar(a)) - Delta U(a).   (TAU-pair)
```

This is the exact role of the original minimal TAU equation in reciprocal
thermodynamics: each directed receipt is one-sided, while the forward/reverse
pair reconstructs the antisymmetric entropy contribution.

If cumulative TAU later becomes state, its issuance and counter-reservoir must
be added to T4. In this first theorem it is an event receipt, not a secretly
created conserved energy.

The actual thermodynamic equation of state is the independently supplied
relation that determines `alpha`. In the finite witness it is the exact
reservoir relation

```text
S_R(E_R) = E_R * ln 2,
1/alpha = Delta S_R / Delta E_R = ln 2.
```

In the autonomous witness, `Delta U` is heat into the system and external work
is zero. Calling `Delta U` work would be incorrect. A driven extension would
need to split heat and work explicitly before applying LDB.

### X3. Praxion

Praxion supplies the reciprocal action channels and their hazards. It is a
finite-temperature Praxion when each channel can be written

```text
k_a(z,z') = c_a(z,z') * exp[Xi(a)/(2*alpha)],       (P)
```

with strictly positive symmetric activity

```text
c_a(z,z') = c_bar(a)(z',z).
```

The activity controls kinetics, not equilibrium. The signed Xi controls the
forward/reverse odds. Negative-Xi actions remain possible as fluctuations;
they are merely less frequent than their reverse.

### X4. Empty and optional slots

For the first witness:

```text
Ruby       = empty
memory     = one-state / no adaptive claim
resolution = the executable Crystal endpoint and MOVE arrival
thaw       = the declared reverse MOVE channel
promotion  = none
TAU ledger = none; TAU_plus is a path observable
```

Adding memory or learning does not disturb the theorem if the memory is
included in `Z`, its energy and entropy are accounted, and every learning or
thaw channel obeys the same reciprocity law. That extension is not required
for the existence result.

## 4. Representation theorem

### Theorem 1. Xypher-to-thermodynamics direction

Let a finite minimal thermodynamic Xypher kernel satisfy X1--X4. Assume:

1. its operational state is complete, every generator row closes, and every
   quotient used by the Praxion is strongly lumpable;
2. Crystal has an explicit positive integer endpoint multiplicity `g(z)` and
   `S_tau(z) = ln g(z)` at the same resolution as `U` and the Praxion;
3. `U` is independently declared and `alpha > 0` is independently supplied
   in one of T2's allowed ways: a named reservoir fundamental relation or an
   independently declared equation of state; if T6 is claimed, the contact
   hypothesis below must use and calibrate that same `alpha`;
4. every channel records an exact T4 balance for system energy, reservoir
   energy, heat, external work, and any information coordinate;
5. every positive channel has a positive reverse and each Praxion activity is
   symmetric under channel reversal; and
6. the complete-state graph is finite. Equilibrium and convergence are stated
   separately on each connected recurrent class.

Then

```text
ln[k_a(z,z') / k_bar(a)(z',z)]
  = Xi(a)/alpha
  = Delta S_tau(a) - Delta U(a)/alpha.
```

Therefore the kernel satisfies T1--T5 and has, on each connected recurrent
class,

```text
pi(z) proportional to exp[S_tau(z) - U(z)/alpha].
```

**Proof.** T1, T2, and T4 are hypotheses 1--4. Divide (P) by its reverse.
Symmetric activity cancels and
`Xi(bar(a)) = -Xi(a)`. The resulting ratio is LDB. Multiplying LDB by the
Gibbs weights gives channel detailed balance. Finite irreducibility on each
connected recurrent class gives a unique stationary law and convergence.
QED.

If the kernel additionally declares a two-body composition with additive
`S_tau` and `U`, exact conserved exchange accounting, reciprocal contact
channels, and symmetric contact activities obeying (P) for the combined
state, the same argument proves T6. A reservoir source for `alpha` alone does
not manufacture a contact generator.

### Theorem 2. Thermodynamics-to-Xypher direction

Every finite equilibrium thermodynamic graph system satisfying T1--T6 admits
a minimal thermodynamic Xypher-kernel representation, component by component.
If its T2 grounding already includes an executable uniform endpoint channel,
the representation is a factorization of the system itself. If T2 supplies
only counted micro-labels, Crystal is instead an explicitly auxiliary marked
readout: on request at state `z`, it samples `i` uniformly from
`1,...,g(z)`, reports port `p_(z,i)`, and leaves `z` and every persistent
state coordinate unchanged. The report is not read by the original dynamics
and cannot alter a future hazard. A claim of **native** factorization therefore
requires the executable channel as one additional bridge condition.

Construct:

1. **Crystal:** use the explicit `g(z)` micro-labels required by T2 as the
   outcomes of the complete uniform endpoint channel `R_C`. When `R_C` is not
   already a native channel, use the nonpersistent marked readout defined
   above. Because it changes no state and feeds no original transition, it
   changes neither the original generator nor its energy accounting,
   equilibrium law, or contact dynamics. Then `S_tau = H(R_C) = ln g = S`
   up to an irrelevant common constant.
2. **Thermo:** copy the independently declared `U` and `alpha`; report
   `TAU_plus = alpha*max(0,Delta S_tau)` and
   `Xi = alpha*Delta S_tau - Delta U`.
3. **Praxion:** retain the original channels and define their activity by

```text
c_a(z,z') = sqrt[k_a(z,z') * k_bar(a)(z',z)].
```

LDB gives exactly

```text
k_a(z,z') = c_a(z,z') * exp[Xi(a)/(2*alpha)].
```

The geometric mean is symmetric. T4 supplies complete accounting and T6
supplies the contact composition, so X1--X4 and the extra hypotheses of
Theorem 1 hold for the native system when `R_C` already exists, or for the
explicitly instrumented readout representation otherwise. QED.

### Corollary. Exact boundary of the equivalence

Within the fixed finite, one-temperature, reciprocal equilibrium class, the
two directions have slightly different strengths:

```text
native minimal thermodynamic Xypher kernel
    => thermodynamic graph system;

thermodynamic graph system
    => minimal Xypher kernel with an auxiliary Crystal readout.
```

The second implication upgrades to literal native equivalence exactly when
the thermodynamic graph system already exposes the executable uniform Crystal
endpoint channel. Otherwise the equivalence is representational: the left
side is a structural decomposition with an auxiliary readout, while the right
side is the unchanged operational dynamics class. The bridge is entropy
compatibility, independent temperature and energy, pathwise closure,
reciprocal support, and local detailed balance.

The frozen existence witness lies in the stronger native branch: its
executable `REFRESH` channel is part of the complete 16-state dynamics. This
clarification changes none of its inputs, predictions, gates, or results.

The theorem does not cover driven nonequilibrium steady states, multiple
reservoirs, nonreciprocal channels, unbounded state, or a growing topology.
Those require extra affinities or an enlarged complete state.

### Corollary. Hard positive-gain gate is insufficient

Let `L(z) = S_tau(z) - U(z)/alpha`. If a finite policy allows transitions only
when `Delta L >= 0`, then any bidirectional allowed edge has `Delta L = 0`.
On an irreducible recurrent class, `L` is constant.

Therefore a deterministic `Xi > 0` gate cannot by itself realize nontrivial
finite-temperature equilibrium. It can describe one-way relaxation or an
absorbing optimizer. The minimal positive-part TAU harness remains lawful as
an output, while the finite-temperature Praxion must retain reciprocal
fluctuations.

## 5. Frozen existence witness

The witness is deliberately small enough to enumerate by inspection.

The engineered construction inputs are the three energies, the Crystal port
counts, the reservoir density of states, and one symmetric hazard per
microscopic edge. They are not discoveries and do not dynamically emerge from
the three-state graph. The macro hazards, equilibrium weights, rate ratios,
contact law, and TAU/Xi identities are consequences to be derived from those
inputs rather than supplied as targets.

### 5.1 Fixed system and Crystal

There are three system macrostates on a fixed path graph:

```text
x = 0  <->  x = 1  <->  x = 2.
```

Their independently declared energy and Crystal causal-future multiplicity
are:

| `x` | `U(x)` | `g_X(x)` | `S_tau(x)` |
|---:|---:|---:|---:|
| 0 | 0 | 1 | `0` |
| 1 | 1 | 4 | `ln 4` |
| 2 | 2 | 4 | `ln 4` |

Crystal is the following fixed typed resolution action, declared before the
kinetic edges. From a complete lifted state `(x,i,j)`, a marked `REFRESH`
event samples

```text
(x,i,j) --REFRESH--> (x,i',j),
1 <= i' <= g_X(x),
R_C((x,i,j),(x,i',j)) = 1/g_X(x).
```

`REFRESH` is an executable row-stochastic channel, including the possible
no-change outcome `i'=i`; it is not an unexecuted observation edge. Marked
refresh ticks occur at unit rate. Their CTMC generator contribution is frozen
as

```text
Q_REFRESH = R_C - I.
```

Thus a distinct `i'` has off-diagonal hazard `1/g_X(x)`, while the refresh
diagonal is `-(g_X(x)-1)/g_X(x)`. The marked no-change outcome remains part of
the endpoint entropy even though it cancels from the state-change generator.
`REFRESH` preserves `x`, `j`, and total energy and is doubly stochastic within
each `(x,j)` block. It therefore preserves the microcanonical equilibrium and
does not change the macro `x` generator.

The resolved label `i'` is the same system microstate coordinate used in the
microscopic lift below. The horizon is `tau = 1`, the endpoint observable is
that resolved coordinate, and

```text
S_tau(x) = H(R_C((x,i,j),.)) = ln g_X(x).
```

Thus the causal endpoint entropy is independent of the starting `i` and `j`
and is not fitted from the Praxion rates. `REFRESH` defines what Crystal can
resolve; the reciprocal `MOVE` channels below define what Praxion can do.
Both are executable typed relations on the same fixed substrate.

### 5.2 Finite reservoir and temperature

Total energy is fixed at `E_total = 2`. The reservoir energy is

```text
E_R(x) = 2 - U(x),
```

and its explicit multiplicity is

```text
g_R(E_R) = 2^E_R,       E_R in {0,1,2}.
```

Therefore

```text
S_R(E_R) = E_R ln 2,
alpha = 1/ln 2 energy units per nat.
```

`alpha` is obtained before the Praxion rates and is not a fitted knob.
More generally, the construction family `g_R(E)=b^E` gives
`alpha=1/ln b` for any integer `b >= 2`; `b=2` is the smallest nontrivial
member, not a value selected after evaluation.

### 5.3 Microscopic grounding and strong-lumpable lift

The authoritative Xypher state is the mesostate `x`. Its entropy, temperature,
and action law are grounded by the following explicit microscopic lift rather
than fitted from a target macro distribution.

The complete energy-shell microstates are

```text
omega = (x, i, j),
1 <= i <= g_X(x),
1 <= j <= g_R(2-U(x)).
```

The three macro fibers therefore contain

```text
n = (4, 8, 4)
```

microstates, for 16 complete microstates total.

Every microstate in fiber `x` is connected by a typed `MOVE` relation to every
microstate in adjacent fiber `x+1`. Each undirected `MOVE` micro-edge has the
same continuous-time hazard `kappa = 1` in both directions. `kappa` only fixes
the clock unit. `Q_MOVE` has diagonal `-8` at every witness microstate, the
negative sum of its off-diagonal `MOVE` hazards. The full generator is

```text
Q_TOTAL = Q_MOVE + Q_REFRESH.
```

Its diagonal is `-8` in fiber `x=0` and `-8-3/4` in fibers `x=1,2`; every
full row sums to zero. The within-fiber `Q_REFRESH` contribution sums to zero
under the `x` partition, so the lumped macro generator below is unchanged.

This graph is finite, connected, energy conserving, and microscopically
reversible. The uniform law on its 16 complete microstates is stationary.
The partition by `x` is strongly lumpable because every microstate in a fiber
has the same number of neighbors in each adjacent fiber.

### 5.4 Generative Crystal–Thermo–Praxion dataflow

The Praxion is built before the macro hazards are evaluated. For each adjacent
candidate `a: x -> y`, the executable dataflow is:

1. Crystal evaluates the frozen endpoint channels at `x` and `y`, returning
   `Delta S_tau = ln g_X(y) - ln g_X(x)`.
2. Thermo computes `TAU_plus(a)`, `TAU_plus(bar(a))`, and their signed
   TAU-pair score

```text
Xi(a) = TAU_plus(a) - TAU_plus(bar(a)) - [U(y)-U(x)].
```

3. The reciprocal substrate supplies a direction-independent opportunity
   activity, derived from fiber counts rather than rates:

```text
c(x,y) = kappa * sqrt[n(x)n(y)] = c(y,x).
```

4. Praxion schedules the candidate with hazard

```text
k_Pi(x,y) = c(x,y) * exp[Xi(x,y)/(2*alpha)].
```

No `pi`, macro hazard, or measured forward/reverse ratio is an input to this
pipeline. For both witness edges, `c = sqrt(4*8) = 4*sqrt(2)`.

The independently specified microscopic graph supplies a second derivation.
Because

```text
Xi(x,y)/alpha
  = ln[g_X(y)/g_X(x)] - [U(y)-U(x)]ln 2
  = ln[n(y)/n(x)],
```

the generated Praxion hazard is

```text
k_Pi(x,y)
  = kappa*sqrt[n(x)n(y)]*sqrt[n(y)/n(x)]
  = kappa*n(y).
```

That is exactly the lumped hazard of the uniform reciprocal micro-edge graph.
The Crystal–Thermo–Praxion pipeline and microscopic state counting must agree;
the equality is a frozen prediction, not a post-hoc factorization.

### 5.5 Exact macro dynamics

The lumped off-diagonal hazards are the number of destination microstates:

```text
k(0,1) = 8,    k(1,0) = 4,
k(1,2) = 4,    k(2,1) = 8.
```

The stationary macro law is

```text
pi = (4,8,4)/16 = (1/4, 1/2, 1/4).
```

For `0 -> 1`:

```text
Delta S_tau = ln 4,
Delta U = 1,
TAU_plus = alpha ln 4 = 2,
TAU_plus(reverse) = 0,
Xi = 2 - 0 - 1 = 1,
Xi/alpha = ln 2,
k(0,1)/k(1,0) = 2 = exp(Xi/alpha).
```

For `1 -> 2`:

```text
Delta S_tau = 0,
Delta U = 1,
TAU_plus = 0,
TAU_plus(reverse) = 0,
Xi = 0 - 0 - 1 = -1,
Xi/alpha = -ln 2,
k(1,2)/k(2,1) = 1/2 = exp(Xi/alpha).
```

The reverse moves give the inverse ratios. Positive structural expansion,
heat exchange, a negative-affinity fluctuation, relaxation, and equilibrium
all occur in the same 16-state construction.

### 5.6 Minimal thermodynamic Xypher-kernel mapping

```text
State           = x in {0,1,2}; the 16-state lift is strongly lumpable
Graph substrate = typed REFRESH and reciprocal MOVE relations
Crystal         = executable uniform REFRESH endpoint channel; S_tau=ln g_X
Thermo          = reservoir-derived alpha, U, TAU pair, and signed Xi
Praxion         = reads Crystal/Thermo output and combines it with the
                  predeclared symmetric opportunity activity c
Ruby            = empty
Action          = schedule one labelled x <-> x' MOVE channel
Resolution      = arrival in the destination mesostate
Thaw            = take the declared reverse MOVE channel
Memory          = one-state; no adaptive-learning claim
```

No stationary distribution or rate ratio is supplied to the Praxion. The
macro bias arises from how many complete causal channels the fixed substrate
provides. That is the constructive sense in which the thermodynamic behavior
is internal to the digital graph rather than imposed by a target table.

### 5.7 Exact same-temperature contact

Contact uses a declared bath-detachment protocol so canonical preparation is
not confused with retained-bath dynamics.

1. Prepare two labelled copies `A` and `B` against independent reservoirs with
   the same `alpha = 1/ln 2`.
2. Detach the reservoirs without changing `x`, `U`, or `g_X`.
3. Condition on the conserved body energy `K = U_A + U_B` and enable only
   reciprocal unit-exchange channels

```text
(x_A, x_B) <-> (x_A + 1, x_B - 1)
```

when both states remain in `{0,1,2}`.

At the microscopic level, every system microstate pair in one product fiber
is connected symmetrically to every pair in the adjacent product fiber. Every
such contact micro-edge has the common hazard

```text
kappa_contact = 1
```

in both directions, and every diagonal generator entry is minus the sum of
its off-diagonal contact hazards. The lumped contact rate is therefore the
destination product multiplicity. On the nontrivial `K = 2` shell,

```text
(0,2), (1,1), (2,0)
```

have multiplicities

```text
g_X(0)g_X(2), g_X(1)g_X(1), g_X(2)g_X(0)
    = (4,16,4).
```

The independently prepared product law, conditioned on `K=2`, is

```text
p_pre(x_A,x_B | K=2)
  proportional to g_X(x_A)g_X(x_B)
                 * exp[-(U_A+U_B)/alpha]
  proportional to (4,16,4),
```

because the Boltzmann factor is constant on the fixed-energy shell. Thus the
pre-contact law and the exact contact equilibrium are both `(1,4,1)/6`; the
equilibrium is not inferred from the contact rates alone.

For a contact transition `s -> s'`, freeze the signed current into `A` as

```text
J_A(s,s') = U_A(s') - U_A(s).
```

The exact expected current is

```text
sum_s p_pre(s) * sum_s' k_contact(s,s') * J_A(s,s') = 0,
```

because each directed term cancels its reverse under detailed balance. A
third identical preparation obeys the same pairwise construction. This is
witness-class same-temperature contact closure, not a theorem about arbitrary
finite-body temperature transitivity.

## 6. Exact verification and falsifiers

The companion verifier must use exact integer or rational checks wherever
possible. It will enumerate only the 16 complete states and must finish in
well under one second on an ordinary laptop.

It must not numerically approximate logarithms, exponentials, or radicals for
an acceptance gate. It will verify

```text
exp[Xi(x,y)/alpha] = n(y)/n(x)
```

as an exact rational identity and square the Praxion equation when necessary:

```text
k_Pi(x,y)^2 = c(x,y)^2 * n(y)/n(x).
```

It must verify:

1. exactly 16 complete states and fiber sizes `(4,8,4)`;
2. every `REFRESH` row has exactly `g_X(x)` equiprobable executable outcomes,
   gives `Q_REFRESH=R_C-I`, is doubly stochastic within `(x,j)`, preserves
   energy, and gives `S_tau=(0,ln4,ln4)`;
3. symmetric `MOVE` micro-edge support, unit reverse hazards, and exact row
   closure of both `Q_MOVE` and `Q_TOTAL=Q_MOVE+Q_REFRESH`;
4. total-energy conservation on every `MOVE` edge;
5. strong lumpability by `x`;
6. the exact macro generator `(8,4,4,8)`;
7. exact equality between the forward Crystal–Thermo–Praxion construction
   `c*exp(Xi/(2*alpha))` and the independently lumped micrograph hazards;
8. exact stationary weights `(4,8,4)` and detailed-balance products;
9. LDB ratios `2` and `1/2` from independently computed `S_tau`, `U`, and
   reservoir density of states;
10. all named minimal-kernel slots and the explicitly empty Ruby slot; and
11. the bath-detached `K = 2` contact shell, common unit microhazard, diagonal
    closure, conditioned canonical weights `(4,16,4)`, and exactly zero signed
    equilibrium current `J_A`.

The following are preregistered failures, not alternate interpretations:

- **Hard gate:** set `k_Pi(1,2)=0` because `Xi(1,2)=-1`, while retaining
  `k_Pi(2,1)=8`. The first expected failure is reciprocal support.
- **Directed kinetic mutation:** on the lexicographically first micro-edge
  `(0,1,1) <-> (1,1,1)`, change only the forward `MOVE` hazard from `1` to
  `2`. The first expected failure is equal reverse microhazards.
- **Crystal/fiber mismatch:** change only `g_X(1)` from `4` to `3`, leaving the
  16-state lift and all kinetic inputs frozen. The first expected failure is
  `fiber_size(1) = g_X(1)g_R(1)` because `8 != 6`.
- **Unpaired TAU:** replace Xi by `TAU_plus(a)-Delta U(a)`. On the pair
  `0 <-> 1`, both directed scores become `+1`; the first expected failure is
  `Xi(a)+Xi(bar(a))=0`.

An accumulated spendable TAU ledger without a counter-reservoir remains
outside the finite witness rather than being simulated as an ambiguous
"unbounded" control.

No Monte Carlo tolerance, burn-in choice, random seed, or large output file is
part of the acceptance boundary.

## 7. What a pass would establish

A proof and exact verifier pass would establish:

1. **constructive existence:** a purely digital finite graph can carry exact
   entropy, energy, reservoir temperature, reciprocal fluctuations,
   relaxation, and equilibrium;
2. **minimal-kernel existence:** that same object has a literal Crystal +
   Thermo + Praxion decomposition with an empty Ruby;
3. **generality in the declared class:** minimal thermodynamic Xypher-kernel
   structure and finite reciprocal equilibrium graph thermodynamics are
   mutually representable exactly when the bridge conditions hold; and
4. **the role of the minimal harness:**
   `TAU_plus = alpha*max(0,Delta S_tau)` can reward expansion without being
   mistaken for the signed law governing fluctuations.

It would not establish that every legacy Xypher, every greedy graph-growth
process, or the deployed THAIM network is thermodynamic. It would not discover
a new fundamental law of matter. The LDB and reversible-Markov ingredients
are established statistical mechanics. The possible novelty is the exact
Xypher representation, the causal-future Crystal realization, and the way the
positive TAU harness can coexist with—not replace—thermodynamic reciprocity.

That is the scientific airplane this chapter is permitted to land.

## 8. Freeze chronology

1. Commit and push this no-data boundary.
2. A different session reviews the exact commit hash.
3. Correct any mathematical or semantic defect and repeat exact-hash review.
4. Only after approval, implement the tiny exact verifier.
5. Review the verifier and its proof correspondence before execution.
6. Execute once, publish the complete tiny result, and report every gate.

No evaluated outcome may be used to revise a prediction in this document.

## 9. Scientific anchors

- U. Seifert, [*Stochastic thermodynamics, fluctuation theorems, and molecular
  machines*](https://arxiv.org/abs/1205.4176), supplies the trajectory-level
  meanings of energy balance, heat, work, and entropy production.
- C. Maes, [*Local detailed balance*](https://arxiv.org/abs/2011.09200),
  reviews the physical meaning and limits of LDB. In particular, LDB is a
  physical modeling condition, not all of thermodynamics by itself.
- G. Pistone and M. P. Rogantin, [*The Algebra of Reversible Markov
  Chains*](https://arxiv.org/abs/1007.4282), gives the established reversible
  Markov parameterization context. The activity/affinity factorization here
  is not claimed as new mathematics.
- D. H. E. Gross and J. F. Kenney, [*The microcanonical thermodynamics of
  finite systems*](https://arxiv.org/abs/cond-mat/0503604), motivates using
  exact combined multiplicities rather than an unqualified macroscopic
  heat-flow slogan for finite bodies.
- A. D. Wissner-Gross and C. E. Freer, [*Causal Entropic
  Forces*](https://doi.org/10.1103/PhysRevLett.110.168702), is the source of
  the causal-path-entropy framing. This boundary uses an explicit finite
  endpoint kernel and does not assume the paper's broader intelligence claim.

These sources anchor terminology. The witness still stands or falls on the
explicit algebra and enumeration above. It establishes a digital
thermodynamic model, not that abstract bits literally become molecular heat.
