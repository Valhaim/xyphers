---
title: Adaptive Xypher Contact Result
aliases:
  - CAL-ADAPT Result
  - Reversible Digital Memory Reservoir Result
tags:
  - domain/physics
  - type/result
  - topic/thermodynamics
  - topic/xypher
  - topic/information-reservoir
domain: Physics
type: result
status: exact-control-passed-native-law-open
created: 2026-07-29
updated: 2026-07-29
related:
  - "[[Adaptive Xypher Contact Boundary]]"
  - "[[Q_H Thermodynamic Compatibility and Weak-Contact Frontier]]"
  - "[[Native Contact Identifiability for a Xypher Payment Substrate]]"
---

# Adaptive Xypher contact result

## 0. Split verdict

The exact CAL-ADAPT experiment matched every preregistered prediction.

1. **Append-only adaptive state is not a reversible equilibrium state.** A
   balance-reversing transfer does not reverse its receipt or monotone learning
   update. Hiding that state is valid only after an exact lumpability proof.
2. **A finite digital information reservoir can produce a nontrivial canonical
   subsystem law exactly.** The generated reversible causal-stack model passes
   microscopic reverse pairing, generator conservation, equilibrium detailed
   balance, strong lumpability, retained-rate no-quench, and exact current
   identities.
3. **This is not yet native Xypher thermodynamics.** The stack alphabet,
   reversible memory operations, and additive clocks are engineered protocol
   hypotheses. The current ledger and Praxor do not instantiate them.

This is a positive constructibility result and a negative result for one
invalid coarse-graining. It is not a derivation of temperature from $alpha$,
heat from TAU, or an equation of state.

## 1. Frozen chronology

The hypotheses and literal predictions were committed and pushed as
`c4eb3af3042d03641bc70311c550c960f253276e` before the checker existed. Two
independent reviews approved that preregistration. Implementation and execution
then occurred in the independent crate
`research/physics/xypher-calorimeter-analysis`.

The final instrument was separately reviewed after adversarial hardening. It
rejects malformed symbol endpoints, duplicate or non-unit channel instances,
misclassified DTMC channels, and missing reverse IDs. It derives a quotient
only after exact lumpability, freezes the full-support uniform reference
instead of choosing an absorber-supported measure, and derives signed
generator diagonals before checking exact zero row sums.

## 2. Exact DTMC boundary witnesses

| Fixture | Exact outcome | Interpretation |
|---|---|---|
| Monotone two-state update | Rows normalize; one strict increase; reverse missing; only $z_1$ is recurrent; uniform reference is not stationary | A stationary delta on the absorber cannot be used as vacuous equilibrium evidence for the lost forward channel |
| Apparent reciprocal support | Fiber-level support contains $A\to B$ and $B\to A$, but $A_0$ and $A_1$ give next-$B$ probabilities $1$ and $0$ | No autonomous balance kernel exists; projected detailed balance is `NotApplicable` |
| Lumpable hidden-label control | Exact quotient $A\leftrightarrow B$; uniform law reversible; two closed full-state components | Hidden labels can be harmless when they do not change balance kinetics |
| Reversible-memory square | Twelve labeled channels at probability $1/3$; one closed class; uniform law; zero currents; $Q_{\rm cross}=1/3$ and $Q_{\rm self}=2/3$ | Explicit write and erase channels restore a valid equilibrium model |
| Separate `MA-` / `MB-` ablations | Row normalization and projected detailed balance still pass; full reverse support and full detailed balance fail | A reversible quotient does not make the full dynamics reversible and cannot substitute for a full-state check |

The result does not prohibit learning. It requires learning, reset, erasure,
clock, and receipt state to be modeled at the level where thermodynamic claims
are made.

## 3. Exact causal-stack result

For alphabet size $r\ge2$ and stack depth $D\ge1$, the generated microstates
are

$$
\mathcal Z_{r,D}
=
\{(n,s):0\le n\le D,\ s\in\Sigma_r^{D-n}\}.
$$

The checker generated every unit-rate `REPLACE`, `POP`, and reverse `APPEND`
channel. It derived the signed generator diagonal as

$$
G_{zz}=-\sum_{z'\ne z}G_{zz'}
$$

and verified every row sum exactly zero. The contacted graph is connected and
symmetric, so the full stationary law is uniform. Counting the reservoir
states gives

$$
\Omega(n)=r^{D-n},
\qquad
\pi(n)=\frac{r^{D-n}}{\sum_{j=0}^{D}r^{D-j}},
\qquad
e^{-\beta\epsilon}=\frac1r.
$$

The factor $1/r$ was not passed to the transition law. It followed from the
enumerated density of states. The absolute energy quantum $\epsilon$ remains
independently declared.

### Primary fixture: $r=2,D=2$

- seven microstates;
- ten directed internal replacements;
- six `POP` and six separately labeled reverse `APPEND` channels;
- twenty-two contacted off-diagonal channel instances;
- level exit rates $(3,4,2)$;
- macrostate law $(4/7,2/7,1/7)$;
- adjacent factor $1/2$ and two-gap consistency factor $1/4$;
- macro rates $k_{n\to n+1}=1$ and $k_{n+1\to n}=2$;
- low-subsystem-energy currents $(1,0)$; and
- high-subsystem-energy currents $(0,-2)$.

Every equilibrium boundary current is exactly zero.

### Preregistered extrapolation: $r=3,D=2$

- thirteen microstates;
- forty-two replacements;
- twelve `POP` and twelve `APPEND` channels;
- sixty-six contacted off-diagonal instances;
- level exit rates $(5,6,3)$; and
- macrostate law $(9/13,3/13,1/13)$ with adjacent factor $1/3$.

### Metamorphic anti-hardcoding case

The same constructor generated $r=2,D=3$ without a literal state table:
fifteen states, thirty-four replacements, fourteen channels in each contact
direction, sixty-two contacted instances, macro multiplicities $(8,4,2,1)$,
and level exit rates $(4,5,4,2)$. A separate $r=4,D=1$ case also passed.

## 4. What this establishes

Under the declared finite-state stochastic-thermodynamic model, a purely
digital system can have all of the following simultaneously:

- a conserved total energy lattice;
- a counted reservoir density of states;
- exact microscopic reverse channels;
- a connected continuous-time generator;
- exact equilibrium detailed balance;
- a canonical subsystem marginal with a nontrivial fixed Boltzmann factor;
- signed relaxation currents that vanish at the predicted law; and
- additive contact clocks that do not quench retained internal hazards.

That is a genuine exact thermodynamic construction in the standard
statistical-mechanical sense. It is not new fundamental physics: exponential
reservoir degeneracy and information reservoirs are established theory. The
scientific contribution here is the exact application, falsification boundary,
and executable admission gate for the Xypher architecture.

## 5. What remains open

The current Xypher has not yet supplied the construction's load-bearing
physics:

- the Crystal does not yet force an $r$-ary reversible reservoir alphabet;
- the live Praxor does not emit ordinary autonomous transfers;
- the ledger appends receipts without a reverse receipt channel;
- actor-local learning state lacks declared reset-reservoir accounting;
- no protocol rule owns additive elementary transfer clocks; and
- no independently derived energy scale identifies $alpha$ as temperature or
  TAU as heat/work.

Therefore this experiment does not establish
$\mathrm{TAU}=\alpha S_\tau$ as an equation of state or demonstrate a native
blockchain thermodynamic phase.

## 6. Next native candidate

A post-result audit found a mathematically clean graph-transfer family worth
freezing next. For fixed inventory $K$,

$$
\lambda_{i\to j}(n)=\kappa_{ij}u(n_i).
$$

If $w_i\kappa_{ij}=w_j\kappa_{ji}$ and
$g(m)=\prod_{q=1}^{m}u(q)$, its exact reversible occupancy law is

$$
\pi_K(n)
\propto
\prod_i\frac{w_i^{n_i}}{g(n_i)}.
$$

For $u(n)=n$, this is the multinomial law of $K$ independently clocked labeled
units. It supplies reciprocal graph kinetics without a Gibbs sampler, but the
current protocol does not yet justify one independent clock per `MicroTau`-edge
or choose between additive edge clocks and normalized node clocks. Those two
interpretations have different equilibria and different no-quench behavior.

The next frontier is therefore not another long trajectory. It is an exact
protocol-ownership test: derive the elementary clock and conductance from
Crystal / Praxor semantics, or record that they are a newly engineered
mechanism before executing ledger transfers.

## 7. Verification

- focused public CAL-ADAPT integration tests: 4 passed;
- adversarial CAL-ADAPT unit tests: 7 passed;
- complete small-crate suite: 72 passed;
- `cargo clippy --all-targets -- -D warnings`: passed; and
- exact CAL-ADAPT execution time after build: approximately $0.01$ seconds.

No trajectory dataset or large result directory is produced.
