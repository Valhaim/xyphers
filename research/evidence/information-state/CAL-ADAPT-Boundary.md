---
title: Adaptive Xypher Contact Boundary
aliases:
  - CAL-ADAPT
  - Learning-Memory Contact No-Go
  - Xypher Adaptive Contact Boundary
tags:
  - domain/physics
  - type/derivation
  - type/experiment
  - topic/thermodynamics
  - topic/xypher
  - topic/information-reservoir
domain: Physics
type: derivation
status: frozen-before-adaptive-contact-checker
created: 2026-07-29
updated: 2026-07-29
related:
  - "[[Q_H Thermodynamic Compatibility and Weak-Contact Frontier]]"
  - "[[Native Contact Identifiability for a Xypher Payment Substrate]]"
  - "[[Native Xypher Allocation Contact Audit]]"
  - "[[Native Information Energy of a Xypher Substrate]]"
---

# Adaptive Xypher contact boundary

**Workstream CAL-NATIVE-LAW (`td-b6abf9`). Frozen on 2026-07-29 after
CAL-COMPAT rejected the unmodified $Q_H$ actor, but before implementing or
running the CAL-ADAPT checker.** This note does not install a new transfer
policy. It asks whether the learning and append-only state required by a
complete Xypher can be omitted while claiming equilibrium on balances.

## 0. Question and split verdict

CAL-COMPAT established two facts:

1. a finite digital graph can implement exact microcanonical contact by an
   engineered fixed-affordance construction; and
2. the unmodified causal-path $Q_H$ actor is incompatible with the declared
   exact-state energy law and quenches internal clocks when contact opens.

The natural response is to add a complete transfer Praxor. But a complete
Xypher learns. The payment ledger also appends receipts. This creates a prior
question:

> Can an adaptive, receipt-producing transfer Xypher be treated as a closed
> equilibrium Markov chain on balances while its learning and history state
> are hidden?

The frozen answer is conditional rather than universal:

- **No on the literal append-only full state.** A strictly monotone internal
  coordinate destroys the microscopic reverse of every transition that
  advances it.
- **Only with a separate proof on the balance projection.** Hiding memory is
  valid only when the projection is strongly lumpable. If two histories with
  the same balances give different next-balance laws, the projected process
  is not an autonomous Markov chain.
- **Yes after the enlarged dynamics supplies positive reverse channels.** A
  learning Xypher can still be a thermodynamic system, but memory writes,
  erasures, receipts, randomness, and clocks must enter the declared state or
  reservoir accounting. A reservoir entropy change can account for a finite
  forward/reverse ratio; merely naming a reservoir cannot replace a missing
  reverse. The appropriate generic object is then a driven or bipartite
  information-thermodynamic process, not an equilibrium balance chain by
  default.

This is a boundary theorem. It does not prove that adaptive digital
thermodynamics is impossible.

## 1. Declared state and projection

Let the complete time-homogeneous state be

$$
z=(b,m,r,c),
$$

where:

- $b$ is the exact labeled `MicroTau` balance allocation;
- $m$ is every policy-memory coordinate read by the transfer actor;
- $r$ is append-only protocol history needed by future behavior, including
  receipts or receipt-derived state; and
- $c$ contains any autonomous clock phase, randomness state, and reservoir
  coordinates needed to predict the next elementary event.

Let $P(z,z')$ be the elementary-event kernel. The balance projection is

$$
B:Z\to\mathcal B,
\qquad B(z)=b.
$$

An external height index need not be included in an equilibrium microstate
when the law is stationary and height-blind. If the policy reads height, uses
a nonstationary schedule, or derives randomness from it, the relevant clock
phase must be included in $c$ or treated as an external drive. Merely deleting
height from a history-dependent law does not make the process autonomous.

All executable CAL-ADAPT fixtures declare identity time reversal
$\Theta z=z$: balances, labels, counters, and stack symbols are treated as
even variables. A later model with odd clock, momentum, or current coordinates
must instead freeze a physical involution $\Theta$ and pair each channel
$\gamma:z\to z'$ with
$\bar\gamma:\Theta z'\to\Theta z$.

## 2. The monotone-coordinate obstruction

### Theorem 1: positive monotone moves lack full-state reverses

Let $L:Z\to\mathbb N$ be a coordinate such that every positive labeled
channel $\gamma:z\to z'$ in the identity-reversal CAL-ADAPT fixtures obeys

$$
q_\gamma(z,z')>0\Longrightarrow L(z')\ge L(z).
$$

If a positive channel $\gamma:z\to z'$ satisfies $L(z')>L(z)$, then its
declared reverse channel $\bar\gamma:z'\to z$ has

$$
q_{\bar\gamma}(z',z)=0.
$$

**Proof.** A positive reverse would require
$L(z)\ge L(z')$ by the same monotonicity assumption, contradicting
$L(z')>L(z)$. $\square$

Consequently no finite channel-resolved local-detailed-balance ratio

$$
\log\frac{q_\gamma(z,z')}{q_{\bar\gamma}(z',z)}
$$

exists on that channel. The endpoints of the strict transition cannot belong
to the same communicating class, so no closed recurrent class contains that
edge as an internal transition. A saturating counter does not repair the
earlier missing reverses; it changes the dynamics at the saturation boundary
and may create a closed terminal face.

### Protocol coordinates in scope

The current extracted payment implementation supplies concrete monotone
coordinates:

- `PaymentLedger.transfer_receipts` appends a uniquely identified receipt for
  an accepted canonical transfer. A later balance-reversing payment appends
  another receipt rather than deleting the first.
- the Rust `ThompsonMemory::record_success` increments both successes and
  trials, while `record_trial` increments trials. Neither operation supplies
  a decrement or erasure transition. This `ReferencePraxor::ruby_memory` is
  actor-local rather than part of `PaymentLedger` or its canonical state hash,
  so restart or reconstruction is an external reset unless it is added to the
  declared state or reservoir accounting.
- `futuruna-xypher/lib/xypher.runa::cycle` likewise increments trials and
  conditionally successes without a decrement. The separate legacy
  `lib/praxor.runa` exposes `praxor_thaw`, which halves both counts. Theorem 1
  applies to the executed `cycle` transition set and to Rust memory updates,
  not globally to a Futuruna kernel that admits `praxor_thaw`.

The earlier native allocation audit already executed the first witness: a
forward payment followed by the equal balance reverse restored every balance
but changed the receipt count and canonical state hash.

This does not make stochastic thermodynamics impossible. It says that the
reverse experiment must include a positive reverse memory/history protocol in
an enlarged state. A named memory tape or receipt-log reservoir can account
for the finite ratio only when that enlarged dynamics restores reverse
support. A balance-reversing payment is not the microscopic reverse of the
full transition.

## 3. Projection and strong lumpability

For $b'\in\mathcal B$, write the destination fiber

$$
F_{b'}=\{u\in Z:B(u)=b'\}.
$$

### Theorem 2: exact criterion for a balance Markov law

Assume the declared state space is finite or countable. One projected kernel
$Q$, independent of the initial law inside each fiber, makes the balance
projection a time-homogeneous Markov chain for every initial law if and only
if, for every pair $z_1,z_2$ with $B(z_1)=B(z_2)$ and every destination
balance $b'$,

$$
\boxed{
\sum_{u\in F_{b'}}P(z_1,u)
=
\sum_{u\in F_{b'}}P(z_2,u).
}
$$

When this strong-lumpability identity holds, the common value defines the
projected kernel $Q(b,b')$. When it fails, the next-balance law depends on the
hidden history inside the current balance fiber.

This is the standard finite-state strong-lumpability criterion, stated here
as an exact admission gate. Stationary or weak lumpability for one specially
prepared hidden-state mixture is a different, weaker claim and must freeze
that preparation independently.

For a continuous-time generator $G$, the corresponding criterion is that for
every $z_1,z_2$ in the same source fiber and every *other* destination fiber
$F_{b'}$,

$$
\sum_{u\in F_{b'}}G(z_1,u)
=
\sum_{u\in F_{b'}}G(z_2,u).
$$

The diagonal rate of the projected generator then follows from conservation.
The checker must not apply discrete row-stochastic tests to a generator or
generator-conservation tests to a discrete kernel.

### Corollary 2.1: behaviorally effective transfer memory is load-bearing

Suppose two reachable full states have the same balances but different
memories, and the actor's memory read changes the probability of at least one
next balance. Then strong lumpability fails.

Conversely, a memory coordinate can coexist with a lumpable balance process
when its value does not change any next-balance probability. For the contact
claim that memory is then behaviorally inert. It cannot simultaneously serve
as evidence that the transfer Praxor learns which balance action to take.

The current `ReferencePraxor` illustrates why this distinction must be
checked rather than inferred. Its one Ruby posterior mean is a common
multiplier on every topology candidate, so it need not change candidate
ordering. That is not evidence of an adaptive transfer law; the reference
Praxor emits no ordinary transfers at all.

## 4. Three admissible adaptive-contact models

Under the frozen definitions, a proposed transfer Xypher must declare which
of the following modeling branches supports each thermodynamic claim. The
branches need not be mutually exclusive across different scales or event
classes.

### Branch A: literal full-state equilibrium

Declare balances, policy memory, receipts needed by the policy, randomness,
and clock phase as the system. Every positive elementary channel must have a
declared positive reverse and finite forward/reverse ratio; reservoir entropy
may account for that ratio. A zero reverse is absolute irreversibility or
infinite affinity in the declared model, not finite local detailed balance.
Standard append-only receipts and success/trial counts fail this branch until
enlarged reverse or reset dynamics is supplied.

### Branch B: equilibrium on a quotient

Declare balances as the thermodynamic state and prove strong lumpability or a
separately frozen stationary coarse-graining. A learning signal may remain in
the implementation only if its hidden variation cannot change the projected
transfer law, or if the coarse-graining proof includes its conditional
stationary distribution. The latter may not be fitted from the evaluation
trajectory.

### Branch C: adaptive information thermodynamics

Keep the memory and history dynamics. Treat transfer actions and memory
updates as distinct event classes in a bipartite or otherwise explicitly
resolved process. Account for:

- action-channel forward and reverse path probabilities;
- information written to or erased from memory;
- random-bit and clock exposure;
- TAU work, mint, burn, lock, and payout receipts; and
- any reset or fresh-memory-tape reservoir.

This branch may support a nonequilibrium steady state, fluctuation relation,
or information-engine inequality. It is not required to have zero stationary
current. It is the natural branch for a genuinely learning Xypher.

## 5. Why an action oracle is not the missing Praxor

The existing `futuruna-xypher/lib/calorimeter_transfer_praxion.runa` is
correctly labeled by its own source as an exact tiny-fixture oracle, not an
adaptive Praxor or protocol action source. It independently enumerates the
already frozen $Q_H$ action law.

The existing `praxion_passive_kernel.runa` is also correctly scoped. It turns
**supplied** positive event hazards into a competing-hazard kernel. It does
not derive those hazards from balances, $\Delta S_\tau$, TAU, or learning.

`praxion_control_work.runa` derives the KL-optimal exponential tilt of a
supplied full-support passive measure by supplied rewards. This is valuable
control thermodynamics, but it does not manufacture the passive physical
clock. Installing uniform transfer hazards solely to obtain equilibrium would
remain an engineered scheduler.

These components can participate in Branch C after a real passive rate source
and reservoir boundary are named. None is currently a positive native-contact
candidate by itself.

## 6. Frozen executable witnesses

The checker will be added to the independent small crate
`research/physics/xypher-calorimeter-analysis`. It will use exact rational
probabilities for discrete kernels, exact nonnegative off-diagonal rates for
continuous-time generators, and typed state and channel metadata. Diagonal
generator exits are derived as the sum of off-diagonal rates. No payment
trajectory is admitted by this experiment.

Every channel instance has a stable channel ID, a declared reverse ID, and an
integer multiplicity. Endpoint-collapsed matrices are insufficient because
parallel WAIT or transfer instances can have different physical reverses.

Every discrete fixture freezes the uniform full-state reference measure before
execution. The checker independently tests its strict positivity,
normalization, stationarity, and detailed balance. It also reports exact
communicating and closed recurrent classes. Thus fixtures 6.1 and 6.2 reject
the uniform reference and have no full-support stationary measure because
they contain transient states. Fixture 6.3 admits the uniform reference but
reports two closed classes and non-uniqueness. Fixture 6.4 admits the unique
uniform law. Projected detailed balance is `NotApplicable` unless strong
lumpability first supplies one projected kernel; when applicable, the
projected reference law is the full reference measure aggregated by fiber.

### 6.1 Monotone full-state rejection

The literal two-state system has

$$
z_0=(A,0),\qquad z_1=(B,1),
$$

with $L(z_i)=i$,

$$
P(z_0,z_1)=1,
\qquad
P(z_1,z_1)=1.
$$

Frozen prediction: the checker reports one strictly monotone positive channel,
missing reverse support, no finite channel ratio, and no recurrent class
containing both states.

### 6.2 Apparent projected reciprocity is insufficient

Use four full states

$$
A_0,B_0,A_1,B_1,
$$

where the letter is the balance state and the subscript is hidden memory.
Freeze

$$
A_0\to B_1,
\qquad
B_0\to A_1,
\qquad
A_1\to A_1,
\qquad
B_1\to B_1
$$

with unit probability.

Across separately prepared histories, the observed balance support contains
both $A\to B$ and $B\to A$. Nevertheless:

$$
\Pr(B_{t+1}=B\mid A_0)=1,
\qquad
\Pr(B_{t+1}=B\mid A_1)=0.
$$

Frozen prediction: existential fiber-aggregated support contains both
directions, full channel reciprocity fails, no projected kernel exists, and
the strong-lumpability witness is the exact pair $(A_0,A_1)$ for destination
fiber $B$ with values $1$ and $0$.

### 6.3 Lumpable hidden-state control

Use the same four labels with deterministic paired transitions

$$
A_0\leftrightarrow B_0,
\qquad
A_1\leftrightarrow B_1.
$$

Frozen prediction: the balance projection is the deterministic reciprocal
chain $A\leftrightarrow B$ and passes strong lumpability, even though the
hidden label is not identified from balances. The hidden label is
behaviorally inert at the balance level.

### 6.4 Explicit reversible-memory control

Use a four-state square with symmetric unit-weight edges and one labeled WAIT
at every state:

$$
A_0\leftrightarrow B_0
\leftrightarrow B_1
\leftrightarrow A_1
\leftrightarrow A_0.
$$

The horizontal edges are balance transfers at fixed memory. The vertical
edges are memory write/erase channels at fixed balance. Every state has three
equally weighted opportunities after including WAIT.

Freeze the channel/reverse pairs as `T0+` / `T0-` on
$A_0\leftrightarrow B_0$, `T1+` / `T1-` on
$A_1\leftrightarrow B_1$, `MA+` / `MA-` on
$A_0\leftrightarrow A_1$, and `MB+` / `MB-` on
$B_0\leftrightarrow B_1$. Each state also has a separately identified
self-reverse WAIT channel.

Frozen predictions:

- every labeled channel has an equal-weight reverse;
- the unique stationary law is uniform on the four full states;
- every stationary channel current is zero;
- the balance projection is strongly lumpable with
  $Q(A,B)=Q(B,A)=1/3$ and $Q(A,A)=Q(B,B)=2/3$;
- in one ablation, removing `MA-` and adding a second labeled WAIT of the same
  weight at $A_1$ preserves row normalization but rejects reverse support;
- in a separate ablation, removing `MB-` and adding the corresponding extra
  WAIT at $B_1$ produces the same independently checked rejection; and
- the two ablations are evaluated separately.

This is a checker-positive control, not a native Xypher result. The memory
erase channel is deliberately explicit rather than free.

### 6.5 Causal-stack information-reservoir candidate

A finite constructive escape from monotone memory replaces an append-only
counter by a reversible symbol stack. Freeze integers $r\ge2$ and $D\ge1$,
with $D=2$ the smallest member of this stack family that exposes two adjacent
energy gaps, and freeze an energy quantum $\epsilon>0$. Let

$$
\Sigma_r=\{0,\ldots,r-1\},
$$

and define the full microstate space

$$
\mathcal Z_{r,D}
=
\left\{(n,s):0\le n\le D,\quad
s\in\Sigma_r^{D-n}\right\}.
$$

Here $n$ is the subsystem's energy-quantum count and $s$ is the reservoir
stack. Declare independently

$$
U_S(n)=\epsilon n,
\qquad
U_R(s)=\epsilon|s|,
\qquad
U_S+U_R=\epsilon D.
$$

At every stack position whose current symbol is $a$, the isolated reservoir
has one unit-rate `REPLACE(position, a -> a')` channel for every $a'\ne a$.
This exactly defines the symmetric internal symbol-replacement multiplicity.
Contact adds two channel families:

1. `POP(symbol)`: remove the final symbol and increase $n$ by one; and
2. its exact reverse `APPEND(symbol)`: append that same symbol and decrease
   $n$ by one.

Every microstate channel and its reverse have equal continuous-time rate one.
The diagonal generator entry is the negative total exit rate. Opening contact
adds `POP` / `APPEND` clocks but leaves every retained symbol-replacement rate
unchanged. This meets CAL-COMPAT's operational additive/no-quench definition
of weak contact: opening contact does not perturb retained off-diagonal rates.
Because all admitted rates are one, it is not a claim of perturbatively weak
physical coupling. It is also not the single normalized discrete clock used
by the rejected $Q_H$ contact fixture. Generator diagonals necessarily change
when the contact channels change total exit rates.

The contacted generator is symmetric and connected. Its unique stationary
law is therefore uniform on full microstates. The macrostate multiplicity is

$$
\Omega_{r,D}(n)=r^{D-n},
$$

so the subsystem marginal is

$$
\boxed{
\pi_{r,D}(n)
=
\frac{r^{D-n}}{\sum_{j=0}^{D}r^{D-j}}
=
\frac{b^n}{\sum_{j=0}^{D}b^j},
\qquad b=\frac1r.
}
$$

For the independently declared energy lattice, this is the canonical form
with

$$
e^{-\beta\epsilon}=\frac1r,
\qquad
\beta\epsilon=\ln r.
$$

The Boltzmann factor is not passed to the transition law. It is predicted from
the counted $r$-ary reservoir multiplicity. The absolute energy scale
$\epsilon$, and therefore $\beta$ in inverse-energy units, remains an
independent calibration.

At the macro level the total rates are

$$
k_{n\to n+1}=1,
\qquad
k_{n+1\to n}=r.
$$

For any preparation with macrostate masses $P_n$, the exact initial current
across the $n,n+1$ boundary is

$$
\boxed{J_{n\to n+1}=P_n-rP_{n+1}.}
$$

It vanishes on every adjacent pair under the predicted $\pi_{r,D}$.

#### Primary frozen fixture

For $r=2,D=2$:

- the full state count is $4+2+1=7$;
- the macrostate law is

  $$
  (\pi_0,\pi_1,\pi_2)=\left(\frac47,\frac27,\frac17\right);
  $$

- the adjacent ratios are $\pi_1/\pi_0=1/2$ and
  $\pi_2/\pi_1=1/2$;
- the two-gap ratio is $\pi_2/\pi_0=1/4$;
- there are six `POP` instances and six reverse `APPEND` instances;
- there are ten directed internal symbol-replacement instances; and
- all retained internal rates are exactly one before and after contact.

For the low-subsystem-energy preparation $P=(1,0,0)$, the frozen currents are

$$
J_{0\to1}=1,
\qquad
J_{1\to2}=0.
$$

For the high-energy preparation $P=(0,0,1)$, they are

$$
J_{0\to1}=0,
\qquad
J_{1\to2}=-2.
$$

#### Preregistered extrapolation

For $r=3,D=2$:

- the full state count is $9+3+1=13$;
- the macrostate law is $(9/13,3/13,1/13)$;
- both adjacent ratios are $1/3$ and the two-gap ratio is $1/9$;
- there are twelve `POP` and twelve `APPEND` instances; and
- there are forty-two directed internal symbol-replacement instances.

The implementation must generate this case from $(r,D)$ rather than contain a
literal thirteen-state table.

#### Candidate boundary

This candidate is not yet the current Xypher's memory and does not emit a
ledger transfer. It makes three new protocol assumptions explicit:

1. an $r$-ary finite memory alphabet;
2. reversible append/pop and symbol-replacement channels; and
3. additive continuous-time channel clocks.

The construction is therefore an engineered information-reservoir hypothesis.
It becomes Xypher-native only if the Crystal independently forces the alphabet
and the Praxor's real learning/erasure receipts instantiate these channels.
Nevertheless it is load-bearing as a next control: unlike the earlier
constant-energy fixed-affordance model, it predicts one nontrivial Boltzmann
factor across two distinct adjacent instances of the same energy spacing,
plus their two-step consistency relation, from counted digital reservoir
states. The chosen alphabet size $r$ determines that factor; it is frozen as
a construction parameter, not fitted from the result.

## 7. Admission and interpretation rules

The executable report must keep the applicable verdicts separate. Every gate
returns `Passed`, `Failed`, or `NotApplicable`; `NotApplicable` is never
silently coerced to a pass.

Common structural gates are typed projection completeness, stable channel and
reverse IDs, multiplicity preservation, full channel reverse support,
monotone-coordinate violations, and communicating/recurrent classes.

Discrete-kernel gates are:

1. row normalization;
2. strong lumpability;
3. existential fiber-aggregated support, named separately from a kernel;
4. projected reverse support only when a projected kernel exists;
5. strict positivity and normalization of the frozen reference measure;
6. full reference stationarity and detailed balance; and
7. projected reference stationarity and detailed balance, reported as
   `NotApplicable` when strong lumpability fails.

Continuous-time gates are:

1. nonnegative off-diagonal rates and derived diagonal conservation;
2. generator strong lumpability;
3. exact reverse-channel pairing;
4. full reference stationarity and detailed balance;
5. retained off-diagonal internal-rate no-quench; and
6. causal-stack multiplicity, current, and fixed-$b$ identities.

No aggregate `all_passed` may allow projected reciprocity to substitute for
full reverse support. The reversible-memory positive control may pass every
applicable discrete gate. The causal-stack control may pass every applicable
continuous-time gate. The monotone and apparent-reciprocity fixtures must fail
for their frozen reasons.

The result can establish an exact obstruction for the declared class and an
engineered reversible-memory escape. The causal-stack candidate can establish
a finite canonical information-reservoir construction with a factor predicted
from state multiplicity. It cannot establish:

- that every learning algorithm is thermodynamically inconsistent;
- that a balance quotient can never be valid;
- that Thompson learning cannot be embedded in an information reservoir;
- that the causal-stack alphabet or clocks emerge from the current Xypher;
- that TAU is heat or joules;
- that $\alpha$ is temperature;
- that $\mathrm{TAU}=\alpha S_\tau$ is an equation of state; or
- new fundamental physics.

## 8. Dependency audit for the next actor

Before a transfer actor is executed, its preregistration must trace each of
the following to a source that existed before the thermal verdict:

| Dependency | Required question |
|---|---|
| State | Which balance, memory, receipt, clock, randomness, and reservoir coordinates predict the next event? |
| Passive hazard | What physical or protocol process supplies each elementary transfer opportunity per unit exposure? |
| Crystal | Which declared topology and future-freedom observable scores the opportunity? |
| Ruby | Which directed flow or learned asymmetry changes action probabilities? |
| Opal / $\Phi$ | What integration exists that neither Diamond nor Ruby supplies alone? |
| $\alpha$ | Which state-derived quantity gates action, without a chosen thermal fit? |
| Action | Which authenticated actor emits the actual `NodeAction::Transfer`? |
| Learning | Which success and failure receipts update which meaningful buckets? |
| Reverse | What is the reverse of both the balance action and its memory/history update? |
| Work | Which TAU or information receipt pays control, actuation, reset, and external work without double counting? |

If passive hazards are independently observed external demand, the result is
an open-system controlled-contact experiment. If the actor itself originates
them, its clock and randomness are part of the proposed physics. If neither
source exists, no transition law has been derived.

## 9. Next scientific fork

After the exact checker runs, CAL-NATIVE-LAW should choose by result rather
than preference:

- **Unexpected failure of either theorem witness:** repair or reject this
  boundary before designing an actor.
- **Boundary passes and a strongly lumpable native actor is found:** freeze
  its projected kernel and test CAL-COMPAT on held-out graph families.
- **Boundary passes and learning changes transfer kinetics:** advance to a
  bipartite balance-memory process under CAL-NESS / CAL-MEMORY. Freeze
  trajectory entropy production, information flow, reset accounting, and
  reservoir affinities before execution.
- **No native passive hazard exists:** record the absence. A newly designed
  hazard source is an engineered protocol hypothesis, not discovered
  equilibrium.

The key consequence is constructive: learning does not end the thermodynamic
program. It determines which thermodynamic state space must be measured.

## 10. Prior-art boundary

The monotone-coordinate proof is elementary. Strong lumpability is standard
finite-state Markov-chain theory. Deriving canonical weights from an
engineered exponential density of states, as in the causal-stack candidate,
is standard statistical mechanics rather than new physics. The need to retain
hidden variables or correct entropy production after coarse-graining is
established stochastic thermodynamics, and explicit memory tapes are
established information thermodynamics. CAL-ADAPT's contribution is the
architecture-specific application and exact admission test for the Xypher's
balance, receipt, and learning dynamics before claiming a native equilibrium.

Relevant primary references include:

- J. G. Kemeny and J. L. Snell, *Finite Markov Chains* (originally 1960;
  Springer edition 1976), especially the strong-lumpability criterion,
  [Springer](https://link.springer.com/book/9780387901923).
- A. Puglisi, S. Pigolotti, L. Rondoni, and A. Vulpiani, “Entropy
  production and coarse-graining in Markov processes,” *J. Stat. Mech.*
  P05015 (2010), [arXiv:1002.4520](https://arxiv.org/abs/1002.4520).
- D. Mandal and C. Jarzynski, “Work and information processing in a solvable
  model of Maxwell's demon,” *PNAS* 109, 11641–11645 (2012),
  [doi:10.1073/pnas.1204263109](https://doi.org/10.1073/pnas.1204263109).
- J. Degünther, J. van der Meer, and U. Seifert, “Fluctuating entropy
  production on the coarse-grained level,”
  [arXiv:2309.07665](https://arxiv.org/abs/2309.07665).
- J. M. Horowitz and M. Esposito, “Thermodynamics with Continuous
  Information Flow,” *Phys. Rev. X* 4, 031015 (2014),
  [doi:10.1103/PhysRevX.4.031015](https://doi.org/10.1103/PhysRevX.4.031015).
- A. C. Barato and U. Seifert, “Stochastic thermodynamics with information
  reservoirs,” *Phys. Rev. E* 90, 042150 (2014),
  [doi:10.1103/PhysRevE.90.042150](https://doi.org/10.1103/PhysRevE.90.042150).
