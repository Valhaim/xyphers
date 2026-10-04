---
title: Endogenous Xypher Thermodynamics Boundary
aliases:
  - CAL-ENDO Boundary
  - Self-Grounding Xypher Temperature Boundary
tags:
  - domain/physics
  - type/preregistration
  - topic/thermodynamics
  - topic/xypher
  - topic/adaptation
domain: Physics
type: preregistration
status: frozen
created: 2026-07-31
updated: 2026-07-31
td: td-5131f1
reviewed-draft-sha256: "217e0725ab982699bba8638fc47e5cb455f8ce693e4bfd41f147eb434d1d46b7"
related:
  - "[[How a Digital Xypher Can Have a Temperature]]"
  - "[[Xypher–Thermodynamic Graph-System Equivalence Boundary]]"
  - "[[Xypher Operational Thermodynamics Result]]"
  - "[[Thermodynamic Sensory Memory Xypher Result]]"
  - "[[Xypher Alpha and TAU Constitutive Result]]"
---

# Endogenous Xypher thermodynamics boundary

## 0. Status and chronology

This document freezes the scientific boundary for CAL-ENDO-0 (`td-5131f1`).
It precedes any new endogenous-construction apparatus or confirmatory result.

The order is binding:

1. define the complete state, admissible inputs, endogeneity intervention,
   mathematical claims, controls, and failure labels here;
2. obtain independent scientific review;
3. freeze, commit, and push this boundary;
4. implement an exact verifier without changing the boundary;
5. freeze and independently review the verifier before its first
   confirmatory execution;
6. execute once and publish every gate and control, including failures.

CAL-XTHERM, CAL-MEMORY, and CAL-ALPHA-0 are inherited evidence. They are not
evaluation data for this experiment. No result may be imported from a new
apparatus while this document has `draft-under-review` status.

## 1. The question

The minimal existence question is closed in a restricted class: one finite
digital Crystal + Thermo + Praxion construction has an independently grounded
operational temperature and exact equilibrium thermodynamics.

The present question is stronger:

> Can an adaptive Xypher start without a thermodynamic reservoir law, use
> its own graph-changing dynamics to create that law, and restore the law
> after admissible damage—without being given a temperature, a Gibbs
> distribution, a target multiplicity curve, or rates fitted to the answer?

Within the adopted finite operational class, the generality target is:

> Derive necessary and sufficient conditions under which an adaptive digital
> graph is admitted as an endogenously thermodynamic Xypher.

This boundary separates three claims that must not be collapsed:

1. **thermodynamic representation:** a graph can be described with thermal
   notation;
2. **operational thermodynamics:** independent state counts, reciprocal
   traffic, accounting, relaxation, and contact agree;
3. **endogenous thermodynamics:** the system's admissible dynamics create and
   maintain the state-count structure that grounds those operational facts.

Only the third claim is new here.

## 2. Meaning of “endogenous”

“Endogenous” does not mean that a system has no constitution or lawful update
rules. No physical system meets that standard. A gas is not disqualified
because molecules have collision laws.

It means that temperature is an output of the authoritative state and lawful
dynamics, not an instruction supplied to the constructor.

The candidate constructor may read:

- the graph and its local ports;
- the finite inventory of available state-descriptor atoms;
- declared energy labels and the energy quantum `lambda`;
- local action affordances;
- action outcomes and its own physical memory;
- explicit heat, work, information, and resource ledgers.

It may not read, directly or through a fitted global equivalent:

- `T`, `beta`, or a desired value of either;
- target energy-shell counts;
- target entropy slopes;
- target Gibbs or Boltzmann weights;
- transition ratios from the channels later used to test temperature;
- a loss, reward, or success flag defined by agreement with any of the above.

The predeclared local alphabet, unique-parent law, and rule “one executable
child per locally present symbol” are explicitly allowed constitutive
primitives. They mathematically constrain the possible shell law, just as a
Hamiltonian constrains a physical system's density of states. What is banned
is a global shell-count/depth/deficit profile, thermal target, or feedback
signal used to make an observed profile pass. Cross-alphabet and cross-energy
predictions must be frozen from the same local rule.

This is a causal non-use requirement. Let `I_forbidden` denote all forbidden
inputs and let `C` be the construction/repair transition kernel. For fixed
authoritative state `z`, the following intervention equality must hold for
every pair of syntactically valid replacements `i` and `i'`:

$$
C(z'\mid z,\operatorname{do}(I_{\mathrm{forbidden}}=i))
=
C(z'\mid z,\operatorname{do}(I_{\mathrm{forbidden}}=i'))
$$

for every successor `z'`.

The equality will be checked in two ways:

1. a data-flow audit must find no forbidden field in the construction path;
2. poison values substituted for every forbidden interface must leave the
   construction and repair action/successor trace unchanged after projecting
   away the inert poison field itself.

An independently computed temperature classifier may inspect the completed
state afterward. Its output must not feed back into the confirmatory builder.

Three nested notions are therefore kept distinct:

1. **state endogeneity:** temperature is computed from the system's own
   authoritative configuration space rather than supplied as a number;
2. **constitutive endogeneity:** the system's driven graph dynamics create and
   repair the multiplicity relation without a thermal target;
3. **grammar endogeneity:** the system also discovers the local construction
   law rather than receiving it as part of its substrate.

CAL-ENDO's primary witness targets the first two. Its alphabet, unique-parent
law, energy quantum, and allowed graph rewrites remain designed physical
primitives. A pass would not establish parameter-free spontaneous discovery
of the grammar. That stronger question is frozen outside the first claim.

## 3. Inherited evidence and explicit nonclaims

### 3.1 Inherited evidence

CAL-XTHERM establishes, for its frozen 16-state finite reciprocal-equilibrium
class:

- a complete microstate space;
- independently declared body energy and reservoir multiplicity;
- a state-count reservoir temperature;
- reciprocal channels satisfying local detailed balance at that temperature;
- exact pathwise energy accounting, equilibrium, relaxation, and contact;
- a conditional representation theorem.

CAL-MEMORY establishes, for its frozen 64-state open driven class:

- a physical energetic sensory bit;
- causal use of memory by action routing;
- separate heat, work, information, and entropy-production accounts;
- a bounded reversible reset lift.

CAL-ALPHA-0 establishes that:

$$
g_R(\lambda m)=b^m
\quad\Longrightarrow\quad
T_R=\frac{\lambda}{\ln b},
$$

when the multiplicity relation is declared independently of the tested
traffic. It also establishes the distinction among reservoir temperature
`T_R`, rate compatibility `T_LDB`, adaptive gate statistic `a_P`, accounting
price `p_o`, opportunity clock `kappa`, and TAU receipts.

### 3.2 What is not inherited

No inherited result shows that an adaptive Xypher creates its own reservoir
multiplicity law. None shows attraction or repair of a thermodynamic manifold.
None makes the historical Praxor statistic `a_P` a temperature. None proves a
network-size power law, a self-powered infinite-time steady state, a new law
of fundamental physics, or blockchain thermodynamics.

The older procedure “choose `beta`, install Gibbs weights, and sample them” is
an externally thermostatted construction. It is retained only as a negative
control for endogeneity.

## 4. Complete authoritative state

The verifier must expose a Markov-complete state. A proposed minimal state is

$$
Z=(\Gamma,r,A,\lambda,X,F,B_W,M,O,P,Q),
$$

where:

- `Gamma=(Gamma_R,Gamma_W)` is the mutable labelled configuration-space
  record: `Gamma_R` is the promoted executable reservoir graph and `Gamma_W`
  is the unfinished, invalid, detached, or quarantined construction workspace;
- `r` is the reservoir microstate currently occupied within `Gamma_R`;
- `A` is the finite set of locally available port symbols;
- `lambda>0` is the declared energy quantum;
- `X` is the probe body's mesostate and microstate label;
- `F` is the finite inventory of free state-descriptor atoms from which new
  executable alternatives can be encoded;
- `B_W=(B_build,B_repair,B_obs)` is the finite typed work store. `B_build` is
  available to BUILD/PROMOTE/QUARANTINE; a four-cell `B_repair` reserve is
  locked during construction and unlocked only by the recorded PERTURB phase;
  the separate causal fixture contains its one observation cell in `B_obs`;
- `M` is all policy, outcome, absorption, teaching, and crystallized memory;
- `O` is the finite bank of blank/written physical operation records together
  with the inverse-protocol order pointer;
- `P` is the current construction, promotion, repair, or contact phase;
- `Q` contains the state of every finite random source or scheduling queue.

Per-transition ledger labels are functions of an edge of `Z`; cumulative
heat, work, information, and entropy production are trajectory
functionals, not hazard-bearing coordinates. If a policy reads a summary,
that finite summary must instead appear explicitly in `M` or `Q`. This keeps
the construction chain finite even though a diagnostic path sum can grow.

Vertices of `Gamma_R` are mutually exclusive possible reservoir
configurations, not material components simultaneously occupied by one
realization. At any instant exactly one reservoir vertex `r` is occupied.
Construction changes the system's executable configuration space; THERMAL
dynamics later move the live coordinate `r` within that space. Workspace
descriptors in `Gamma_W` are not executable reservoir alternatives and cannot
inflate a shell count. A diagram containing `b^m` vertices is not a
multiplicity proof unless all `b^m` labels are distinct, executable, uniformly
resolved alternatives of the complete live system.

All thermodynamic classifications are conditional on the current architecture
`Gamma`. BUILD, PROMOTE, REPAIR, and QUARANTINE change that architecture and
are driven work processes. THERMAL classification is performed on the frozen
`Gamma_R` sector. This is analogous to assigning equilibrium properties at a
fixed volume while tracking piston motion separately as work.

If an implementation uses a deterministic event order, that order and cursor
belong to `Q`. If it uses pseudorandom choice, the generator state belongs to
`Q`. A history-dependent result with omitted `M`, `O`, `P`, or `Q` fails complete
state rather than becoming evidence for non-Markovian physics.

The transition set is typed into disjoint channel families:

| Family | Role | Required account |
|---|---|---|
| `BUILD` | encode a new executable alternative through a local port | work/resource |
| `PROMOTE` | admit a locally complete frontier as a reservoir shell | work/resource |
| `REPAIR` | restore an admissibly damaged local binding | work/resource |
| `QUARANTINE` | detach an invalid binding | work/resource |
| `LEARN` | update physical policy memory | information and, if erased, work/heat |
| `THERMAL` | reciprocal body–reservoir exchange | heat |
| `WORK` | driven state or topology change | work |
| `PERTURB` | frozen damage intervention | named external work/resource transfer |

TAU is not instantiated in the primary apparatus and therefore cannot
substitute for any row in this table. The energy label
`E_R(v)` is the energy when alternative `v` is occupied. It is not the sum of
the construction costs of all vertices in `Gamma`. Work used to change
`Gamma` is separately debited from `B_W`.

## 5. The actual state-count observable

Let the active reservoir configuration-space graph be layered by declared
energy. Its shell at
energy `lambda*m` is

$$
R_m(Z)=\{v\in \Gamma_R:\ E_R(v)=\lambda m\},
$$

with **actual**, enumerated multiplicity

$$
g_m(Z)=|R_m(Z)|,
\qquad
S_m(Z)=\ln g_m(Z).
$$

Every member of `R_m` must be a canonical, injectively encoded, executable
complete alternative. Prefix aliases, duplicate descriptors, locks,
construction records, and hidden repair flags either belong to the complete
microstate or invalidate the count. Merely drawing multiple paths to one
underlying configuration does not increase `g_m`.

No formula-generated count may replace the enumeration in the state
classifier. A formula may predict a count before execution; passing requires
the graph itself to contain exactly that many distinct authoritative
microstates.

For adjacent nonempty shells define the finite-difference inverse
temperature

$$
\beta_m^{\mathrm{state}}(Z)
=\frac{S_{m+1}(Z)-S_m(Z)}{\lambda}
=\frac{1}{\lambda}\ln\frac{g_{m+1}(Z)}{g_m(Z)}.
$$

The local temperature exists and is positive and finite only when
`g_{m+1}>g_m>0`:

$$
T_m^{\mathrm{state}}=\frac{1}{\beta_m^{\mathrm{state}}}.
$$

A single constant temperature exists across the admitted reservoir interval
exactly when every adjacent slope agrees. The state classifier must report
one of:

- `UNDEFINED`: too few nonempty shells or a zero multiplicity;
- `NONPOSITIVE`: at least one admitted slope is zero or negative;
- `STATE_DEPENDENT`: positive slopes exist but differ;
- `UNIQUE_CONSTANT(T_state)`: all admitted slopes agree and are positive.

This entropy is microcanonical state-count entropy. It is not automatically
the same object as causal path entropy `S_tau`. Any equality between them
requires a separate proof.

## 6. Candidate constructive witness: a local prefix reservoir

### 6.1 Local substrate law

The primary witness uses a rooted labelled graph. The substrate exposes a
finite alphabet of physical port types

$$
A=\{a_1,\ldots,a_b\},\qquad b=|A|\ge2.
$$

Every encoded alternative has at most one outgoing port of each type. A valid child of
a node labelled by word `w` through port `a` has word `wa`, unique parent
`w`, a canonical injective encoding, and occupied-state energy one quantum
higher. Executing `wa` must place the live reservoir coordinate in a complete
configuration distinguishable from every other word of the same depth. These
are local graph laws. They do not
mention a temperature, a target shell size, or a probability distribution.

Prefix parent/child edges are typed structural-incidence and certification
edges. They carry no automatic stochastic hazard for the occupied coordinate
`r`. In the frozen operating sector, `r` moves only on the explicitly
enumerated THERMAL/REFRESH edges; in contact it moves only on CONTACT edges.
This prevents an unledgered prefix walk from adding hidden heat channels.

The constructor sees individual open ports and free descriptor atoms. It does not
receive `b^m`, although it can observe the individual symbols in `A`. A shell
becomes active only after local closure certificates show that every port of
every node in the preceding active shell is filled exactly once. The
certificate is accumulated from local events; it is not supplied as a target
shell count.

Only the deepest promoted shell emits BUILD candidates for the next
frontier. A fresh descriptor remains in `Gamma_W` until every locally emitted
port has one canonical, injective child, every attempted alias is a terminal
FAILURE, and no alias is live on a closed port. A FRESH-first path may leave
its disabled, untried ALIAS record blank. PROMOTE then moves the whole
certified frontier into `Gamma_R`. If finite
descriptors or work run out first, the incomplete workspace remains visible
but the last promoted reservoir interval is unchanged. The promotion rule and
the minimum interval required by the classifier are frozen before execution;
an incomplete shell cannot be included, and a complete bad shell cannot be
excluded.

Free descriptor atoms and an unfinished frontier remain explicit workspace
coordinates in `Gamma_W`. They do not disappear from the authoritative state
and are not executable or counted as reservoir microstates before a frozen
local-closure rule promotes the complete frontier. The promotion boundary is
fixed before execution; it may not be moved afterward to exclude a bad shell.

### 6.2 Why this is not yet the answer

The local grammar predicts a geometric reservoir, but prediction is not
evidence. The confirmatory witness must still show all of the following:

- it starts with only a root or with a frozen port-incomplete, nonthermal
  graph;
- the adaptive loop actually binds and promotes the shells;
- actual graph enumeration, not the grammar, gives the predicted counts;
- memory causally changes construction choices;
- admissible defects are detected and repaired;
- temperature inputs and held-out traffic are causally unused by the builder;
- independently aggregated reciprocal exchange traffic returns the same
  temperature;
- all topology work, heat, and information flows remain typed.

An initially perfect prefix tree is thermodynamic representation evidence,
not endogenous-creation evidence.

## 7. Adaptive loop under test

The adaptive requirement is operational rather than rhetorical. The
confirmatory construction must contain a closed causal chain:

$$
\text{perceive}\to\text{integrate}\to\text{select}\to
\text{act}\to\text{observe outcome}\to\text{update memory}.
$$

### 7.1 Crystal

- **Diamond:** an enabled proposal to fill one currently open local port has
  exactly one unit of topological opportunity, `D_c=1`. Closed ports produce
  no candidate. Thus topology changes the candidate set without a fitted
  importance scale.
- **Ruby:** for the meaningful bucket `(port symbol, rewrite kind)`, let `s`
  be prior successes and `n` prior terminal trials in physical memory. The
  symmetric Bernoulli posterior-predictive value is

  $$
  R_c=\frac{s+1}{n+2}.
  $$

  The unit pseudocount on each outcome is the exchange-symmetric uniform
  prior. It is identical for every bucket and is not fitted. This is a
  Beta–Bernoulli posterior-predictive mean, not random Thompson sampling.
- **Opal:** the two-channel project integration is frozen exactly. Because
  `D_c=1` and `0<R_c<1`, it simplifies without floating-point ambiguity to

  $$
  \operatorname{var}_{\mathrm{pop}}(1,R_c)=\frac{(1-R_c)^2}{4},
  \qquad
  d_{\mathrm{eff}}(1,R_c)=\frac{1+R_c}{2},
  $$

  and therefore

  $$
  \Phi_c
  =\operatorname{mean}(1,R_c)
   \left(1+\sqrt{\operatorname{var}(1,R_c)}\right)
   d_{\mathrm{eff}}(1,R_c)
  =\frac{(1+R_c)^2(3-R_c)}{8}.
  $$

All values are exact rationals. Diamond and Ruby both affect the live
candidate hazard. At fixed non-memory state, intervening on a bucket's
success/trial record changes `R_c`, `Phi_c`, and therefore its next-event law.

### 7.2 Praxor

In construction and repair, the finite queue coordinate exposes one open port
`(w,a)` at a time in canonical word/symbol order. While the free pool offers
one descriptor and the Thermo work store offers one charged cell, that port's
candidate set has two rewrite kinds:

1. `FRESH(w,a)` binds the next free descriptor to the canonical new
   executable word `wa`;
2. `ALIAS(w,a)` proposes reusing the parent descriptor `w` itself, which the
   local injectivity/acyclicity validator quarantines as a failure.

The free pool exposes its lowest immutable descriptor ID through the queue
coordinate in `Q`; changing that ID order is an invariance control because
descriptor IDs carry no energy or policy meaning.

Candidate buckets are `(a,FRESH)` and `(a,ALIAS)`, not arbitrary hashes. Each
syntactically distinct candidate identity may be attempted at most once in a
construction or repair episode; that attempt state is part of `Q`. An alias
failure leaves the port open, so its still-untried fresh candidate remains
enabled. A successful fresh binding closes the port and disables the alias.
No retry loop or numeric attempt cap exists.

Every enabled candidate has continuous-time hazard

$$
q_c=\Phi_c.
$$

The unit opportunity clock defines the time unit. The two hazards at the live
port compete without a tie rule: from state `z`, candidate `c` is the next
action with exact probability `q_c/sum_j q_j`. After ALIAS fails, FRESH is the
only remaining candidate at that port. After FRESH closes it, the cursor moves
to the next open port. The verifier represents this exact rational generator,
discharges global creation by the rank certificate in Section 9.1, and solves
only the frozen small quotients and controls.

The Section 9.3 causal-memory fixture uses this same one-port scheduler and
intervenes only on the live FRESH/ALIAS bucket records.

The primary BUILD/REPAIR phase is a gas-phase action policy: every enabled
positive-hazard candidate can act. Before the reservoir relation exists,
physical temperature is undefined, so using it as a gate would be circular.
The historical statistic

$$
a_P=\frac{N_{\mathrm{acted}}}{\sum_{i\in\mathrm{acted}}\Phi_i}
$$

may be reported as a dimensionless policy-history observable, but it is not
`T_state`, does not price BUILD/REPAIR, and does not enter the endogeneity
proof. No arbitrary bootstrap or strict-versus-nonstrict `Phi` threshold is
present. This is a deliberate first-principles choice: CAL-ALPHA-0 already
shows that `a_P` is not an admitted thermodynamic temperature. The action gate
is local executability plus positive hazard, not a mislabeled thermal scalar.

There is an exact incompatibility with the historical gate in this apparatus.
For `0<R<1`,

$$
\frac38<\Phi(R)<1.
$$

After any nonempty acted history,

$$
a_P=\frac{1}{\operatorname{mean}(\Phi)}>1,
$$

so a rule requiring `Phi>a_P` can never fire; at zero history `a_P` is
undefined unless a bootstrap is inserted. Adding a bootstrap or rescaling the
scores merely to avoid this no-go would be a thumb. The older seven-property
“full adaptive” bundle is a proposed application architecture, not a
first-principles condition for thermodynamics. This witness is consequently
labelled an **adaptive Xypher**, not a legacy full-adaptive implementation.

Memory is a finite physical append-only outcome log. Its capacity is exactly
the number of frozen passive observations and syntactically distinct
candidate identities in the finite specimen; it is derived from the
authoritative opportunity set, not chosen as a runtime cap. Each record has
states `BLANK`, `PENDING`, `SUCCESS`, and `FAILURE`. Beta–Bernoulli sufficient
statistics are
derived from terminal records rather than stored in unbounded counters.

A fresh binding becomes `PENDING`. It becomes `SUCCESS` only when its whole
frontier is locally certified and promoted. An alias or invalid initial
binding becomes `FAILURE` when quarantined. Promotion therefore propagates
credit backward to every contributing earlier binding. Initial valid and
invalid bindings may provide passive absorption records through the same
memory path without an action by this Praxor. The exact ABSORB transition and
its resource cell are frozen in Sections 9.3 and 13; prose observation alone
cannot modify memory.

The memory and construction media receive a finite reversible lift. Every
forward record or binding change consumes a charged work cell of declared
energy `epsilon_W`; the enlarged audit transition graph contains the paired
reverse operation and the work-cell state. The live confirmatory protocol is
forward driven and finite, but no logical overwrite, erasure, or missing work
source is allowed. Reset is not claimed. An ideal or unbounded counter arm is
a downgraded logical control and cannot receive the full label.

The primary arm must demonstrate:

- **perception:** graph state changes candidate scores;
- **action:** selected candidates alter authoritative bindings;
- **outcome learning:** both success and failure update the same physical
  memory later read by selection;
- **absorption:** observed port outcomes can update that memory without an
  action by this Praxor;
- **backward learning:** promotion or quarantine updates contributing earlier
  actions;
- **memory causality:** a `do(M=m')` intervention at identical non-memory state
  changes at least one next-action distribution and at least one downstream
  path, mean hitting time, or work-consumption distribution.

An external teaching interface may write only through the same finite memory path.
It is disabled in the confirmatory endogenous arm because teaching the target
grammar would compromise causal non-use. The primary arm has no crystallized
shortcut cache; all selection reads the live log-derived value. A later cache
may form only when the finite local opportunity set has been exhaustively
observed, and the first contradiction must thaw it. No confidence threshold
may be tuned to make the result pass.

### 7.3 Thermo

The Thermo layer has three separate jobs:

1. retain declared energy labels and exact channel ledgers throughout graph
   change;
2. after a reservoir interval exists, compute `T_state` from actual shell
   counts for the frozen classifier.

It also participates causally in every adaptive action: candidate enablement
requires a charged `B_W` cell, and the accepted FRESH or ALIAS map must debit
that cell through its frozen `Delta E,Q,W_on` row. At identical Crystal and
memory state, `do(B_W=0)` disables the action while a charged cell enables it.
Thus Crystal, Thermo, and Praxor are wired into one construction cycle even
though physical temperature is deliberately not an action gate.

The primary apparatus does not declare a causal-path observable `S_tau` or an
accounting price `p_o`. Its TAU and Xi fields are `NOT_INSTANTIATED`, not zero.
This avoids inventing a bridge between causal path entropy and microcanonical
state-count entropy merely to populate framework slots.

Before the state-count relation exists, temperature is `UNDEFINED`, not zero.
After it exists, the confirmatory construction/repair kernel still may not
read it. Thus the result cannot be explained by a thermostat correcting its
own target error.

BUILD and REPAIR are driven work channels, not thermal relaxation. A
one-directional controller that removes defects creates entropy elsewhere or
depletes `B_W`; it is not evidence for finite-temperature equilibrium. The
thermal claim applies to the held-out reciprocal body–reservoir sector after
the constructed interval is frozen. The maintenance claim applies only to
the declared finite repair episode and available work store.

## 8. Structural theorem ladder

### E1. Prefix-shell recursion

Suppose the active configuration-space graph has one canonical root, an
injective map from words to executable alternatives, unique parentage, and
every node in shell `m` has exactly one child through each of the `b` port
symbols, with no aliases or other nodes promoted into shell `m+1`. Then the
map from `A^m` to shell `m` is a bijection and

$$
g_0=1,
\qquad
g_{m+1}=b g_m,
$$

and therefore, by induction,

$$
g_m=b^m.
$$

This is a global multiplicity law derived from a local port law.

Within this prefix class, exact local port completeness is sufficient for the
recursion. It is also necessary for the stronger claim of a regular labelled
prefix realization. Geometric shell counts alone do not imply that every
node has one child of every label; irregular graphs can share the same shell
histogram.

### E2. Constant-temperature shell theorem

For any finite layered graph with `g_0=1`, equal energy spacing `lambda>0`,
and at least two adjacent shell gaps, a positive constant state-count
temperature exists across the admitted interval if and only if there is one
constant `r>1` such that

$$
g_{m+1}=r g_m
$$

on every admitted gap. Then

$$
T_{\mathrm{state}}=\frac{\lambda}{\ln r}.
$$

If the interval begins at the unit root and every `g_m` is an integer, `r`
must be an integer on the full interval. In the prefix witness, `r=b`, so

$$
T_{\mathrm{state}}=\frac{\lambda}{\ln b}.
$$

For a general layered graph with unequal ratios, the same finite differences
define state-dependent local temperatures. Failure of constant temperature
does not justify fitting an average slope and calling it exact.

### E3. Held-out reciprocal-traffic theorem

Let a probe body have mesostates `x`, energies `U_x`, and actual, enumerated
intrinsic microstate counts `h_x`. Require every tested body gap to be an
integer multiple of `lambda`. At fixed total body-plus-reservoir energy, the
complete live microstate is

$$
(\Gamma,M,\ldots; x,i; r),
\qquad
i\in\{1,\ldots,h_x\},
\quad
r\in R_{(E_{\mathrm{tot}}-U_x)/\lambda}(\Gamma).
$$

The adaptive context is fixed during this held-out sector. Before any run,
seal a reciprocal microscopic exchange rule that connects the two complete
fibers by either:

1. complete bipartite unit-clock edges; or
2. a weighted biregular bipartite graph whose total outgoing microscopic
   clock is constant within each source fiber and whose edge clock is the
   same in both directions.

This condition makes the macro partition strongly lumpable. If the two fiber
sizes are `Omega_x` and `Omega_y`, and their constant weighted degrees are
`d_x` and `d_y`, double-counting the same undirected edge weights gives

$$
\Omega_x d_x=\Omega_y d_y,
\qquad
k_{x\to y}=d_x,
\qquad
k_{y\to x}=d_y.
$$

Consequently the coarse hazards obey

$$
\frac{k_{x\to y}}{k_{y\to x}}
=
\frac{h_y g_R(E_{\mathrm{tot}}-U_y)}
     {h_x g_R(E_{\mathrm{tot}}-U_x)}.
$$

For `g_R(lambda*m)=b^m` and `Delta U=U_y-U_x`, this becomes

$$
\ln\frac{k_{x\to y}}{k_{y\to x}}
=
\underbrace{\ln\frac{h_y}{h_x}}_{\Delta S_X}
-
\underbrace{\frac{\ln b}{\lambda}}_{\beta_{\mathrm{state}}}
\Delta U.
$$

Therefore an independently evaluated labelled-channel classifier gives the
unique compatible value

$$
T_{\mathrm{LDB}}=T_{\mathrm{state}},
$$

provided at least one channel has nonzero energy gap and all zero-gap and
additional nonzero-gap channels pass the frozen compatibility conditions.

The frozen probe fixture has three body mesostates with actual microstate
counts and energies

$$
(h_0,h_1,h_2)=(1,2,1),
\qquad
(U_0,U_1,U_2)=(0,\lambda,2\lambda).
$$

Sealed complete-bipartite unit-clock channels test both adjacent gaps, the
two-quantum gap, and energy-preserving REFRESH inside the twofold middle
fiber at the frozen total energy `E_tot=3lambda`. The builder cannot inspect
this probe or its generative edge rule.

Every microscopic off-diagonal edge has its identical reverse, and every
diagonal is the negative outgoing row sum. The resulting fixed-architecture
generator is finite, connected, and symmetric on complete microstates. It
therefore has the unique uniform microcanonical stationary law and converges
to it from every initial distribution. Its body macro projection is

$$
\pi_x
=\frac{h_xg_R(E_{\mathrm{tot}}-U_x)}
       {\sum_y h_yg_R(E_{\mathrm{tot}}-U_y)}
=\frac{h_xe^{-\beta_{\mathrm{state}}U_x}}
       {\sum_y h_ye^{-\beta_{\mathrm{state}}U_y}}.
$$

Thus the held-out arm tests row closure, reverse support, strong lumpability,
local detailed balance, the Gibbs/microcanonical stationary law,
irreducibility, uniqueness, and relaxation—not rate ratios alone.

This agreement is not installed through a rate formula. It follows from
actual microstate counts plus a predeclared symmetric strongly lumpable
micrograph. The exchange rule is sealed before construction and hidden from
the builder; “expose after state classification” means expose it to the
observer, not design it after seeing `g_m`.

Forward/reverse asymmetric activity and non-biregular microscopic weighting
are forbidden in the held-out equilibrium classifier. If present in another
sector, their affinity and work source must be accounted and their raw rate
ratio cannot identify `T_LDB`. The activity-only control may multiply all
microscopic clocks by a common positive factor, or use another symmetric
weighted-biregular clock family, without changing the ratio.

### E4. Finite-chain creation and repair theorem

Let `C` be the finite Markov kernel for a fixed construction or repair phase,
including policy memory, queue/random state, work store, and workspace. Let
`M` be the set of states whose promoted architecture has the required
thermodynamic structure.

For any initial state `z`, the first hitting time `tau_M` is finite with
probability one if and only if no closed communicating class contained in
the complement of `M` is reachable from `z`.

The same criterion is necessary and sufficient after a defect: apply it to
every state produced by the frozen defect kernel. Because the admitted chain
is finite, almost-sure hitting also gives finite mean hitting time. Between
defects, exact invariance is the separate support condition

$$
C(z,M)=1
\qquad\text{for every }z\in M
$$

during the maintenance phase. If reverse construction events remain live and
violate that support condition, the correct object is a fluctuating
nonequilibrium distribution around `M`, not exact invariant maintenance.

This theorem converts creation and repair into an exact support calculation.
It may be discharged by explicit communicating-class enumeration or by a
finite well-founded rank certificate proving that every live transition stays
within the ranked support and every state outside `M` has a positive path to a
strictly lower rank. A long successful trajectory is not a substitute.

### E5. Dynamic endogeneity admission criterion

Let `M_therm` be the set of complete states that pass the state-count,
reciprocal-traffic, complete-state, and ledger classifiers. Let `I_0` be the
frozen nonthermal initial ensemble and `D` the frozen admissible defect family.

For the adopted finite operational definition, a construction is an
**endogenously thermodynamic Xypher** if and only if all of the following
hold:

1. **independent structure:** energy, actual shell counts, and microscopic
   reverse pairing are declared independently of the tested macro traffic;
2. **causal non-use:** the construction/repair kernel passes the forbidden-
   input intervention equality in Section 2;
3. **creation:** from every required `z in I_0`, the first hitting time of
   `M_therm` is finite with probability one and is not zero;
4. **maintenance:** between perturbations `M_therm` is invariant, and after
   every `d in D` the return time to `M_therm` is finite with probability one,
   within the declared finite episode, provided the perturbation leaves the
   declared repair descriptors and sufficient `B_W` available;
5. **adaptive closure:** perception, action, outcome memory, learning, and the
   memory-causality intervention all pass;
6. **thermodynamic closure:** heat, work, information, resources, and receipts
   are typed; first-law accounts close on every channel; thermal/contact path
   probability ratios close; and driven operations have an injective forward
   protocol with an explicit inverse protocol;
7. **independent operation:** held-out channels give a unique
   `T_LDB=T_state`, and frozen contact predictions pass when contact is
   claimed.

Conditions 3 and 4 turn static representation into endogenous creation and
maintenance. Conditions 1, 2, and 7 prevent circular fitting. Condition 5
distinguishes an adaptive Xypher from a nonadaptive graph grammar.
Condition 6 prevents omitted memory or construction work from masquerading
as thermodynamics.

The “if and only if” in E5 is an operational admission criterion, not by
itself a discovery theorem. Necessary means necessary relative to the adopted
finite definition, not necessary for every adaptive digital graph imaginable.
The nontrivial results are E1's combinatorial bijection, E2's shell
characterization, E3's independent traffic derivation, and E4's exact
communicating-class criterion. Together with causal non-use, physical
adaptive closure, ledger closure, and contact, they form a constructive
sufficiency chain for E5.

Neither E4 nor E5 turns a driven repair controller into an equilibrium
process or asserts indefinite resistance to recurring damage from a finite
work store.

## 9. Frozen construction family

The theorems remain symbolic in alphabet size `b`, energy quantum `lambda`,
and completed depth. The confirmatory construction fixtures are exact:

$$
b\in\{2,3,4\},
\qquad
\lambda=1,
\qquad
m\in\{0,1,2,3\}.
$$

Each begins with one promoted root, no other executable reservoir state, an
empty workspace, blank candidate memory, and

$$
P_b=b+b^2+b^3
$$

free descriptors. Thus `P_2=14`, `P_3=39`, and `P_4=84`. These are exactly
the atoms needed for three complete frontiers; no numeric depth condition is
read by the builder.

There are `2P_b` syntactically distinct FRESH/ALIAS candidate identities and
therefore `2P_b` finite semantic memory records and attempt bits. In the
worst live ordering every ALIAS is tried before its FRESH partner. Construction
then contains at most:

- `2P_b` candidate-attempt operations;
- `P_b` automatic PENDING-to-SUCCESS credit operations; and
- three automatic PROMOTE operations.

The primary BUILD charged work-cell and finite operation-record capacities
are therefore

$$
C_b=3P_b+3,
$$

namely `45`, `120`, and `255`. Capacity is a proof-derived worst-case resource
bound, not a stopping threshold.

The initial authoritative state additionally contains four charged
repair-reserve cells, four blank repair operation records, and two blank
episode-keyed repair semantic/attempt records. They are present from time zero
but typed and locked against BUILD. Thus the total initial internal work-cell
capacity is `C_b+4`; even the worst positive-probability construction ordering
cannot consume maintenance resources. PERTURB changes the phase lock but does
not inject those cells.

For the primary creation gate, `M_therm^(b)` first becomes eligible only after
the third PROMOTE, when actual promoted counts on shells zero through three
are `(1,b,b^2,b^3)`, the work/record maps close, and the sealed probe properties
hold. Earlier one- or two-gap prefixes are not silently counted as a pass.
Subsequent incomplete workspace does not change the conditional promoted
reservoir classifier.

Finite inventory is a physical scale variable and necessarily constrains how
much configuration space can be encoded; it is not hidden from the Xypher.
To show that the constructor is not reading a target depth, each `b` also has
an off-shell inventory control with `P_b+1` descriptors, two additional
episode-keyed candidate/attempt records, `C_b+2` BUILD work/operation cells,
and the same untouched four-cell repair reserve.
The sequential port cursor permits at most one ALIAS and one FRESH before the
extra descriptor is consumed. After completing shell three, the same local
rule therefore begins one depth-four binding and exhausts the free pool. That
incomplete descriptor remains in
`Gamma_W`; no fourth shell is promoted, and the depth-three thermodynamic core
and temperature stay unchanged. The code may read individual availability
and `F=0`, but it may not read `P_b`, the numeral three, or a target depth.

After the last local closure certificate for a frontier, the protocol order
is frozen: validate injectivity and port completeness; apply SUCCESS credit to
the frontier's pending FRESH records in immutable descriptor-ID order; append
one promotion record; then atomically move the certified frontier from
`Gamma_W` to `Gamma_R`. Only after PROMOTE may the next frontier emit
candidates. Descriptor-ID permutation is an invariance control.

### 9.1 Exact creation proof, not a giant matrix

For a fixed frontier let `O` be the number of open required ports and let
`A_cursor` be one when the current port's ALIAS remains untried and zero
otherwise. Define

$$
V=2O+A_{\mathrm{cursor}}.
$$

An ALIAS action changes `A_cursor` from one to zero and lowers `V` by one. A
FRESH-after-ALIAS strictly lowers `V` by at least one; a FRESH-first action
strictly lowers it by at least two. On the last port the absent next cursor
lowers it by one additional unit. Every enabled hazard is greater than `3/8`,
every candidate
identity acts at most once, and the work and descriptor resources cover the
worst ordering. Thus `V` reaches zero
almost surely in finite mean time. Include the automatic phases in the
lexicographic rank

$$
\mathcal R=
\begin{cases}
(3-m,2,V), & \text{BUILD},\\
(3-m,1,N_{\mathrm{pending}}), & \text{CREDIT},\\
(3-m,0,1), & \text{PROMOTE}.
\end{cases}
$$

BUILD actions decrease `V`; entering CREDIT lowers the phase coordinate;
each CREDIT lowers `N_pending`; entering PROMOTE lowers the phase coordinate;
and PROMOTE lowers `3-m` before the next BUILD phase. This proves
probability-one construction through depth three, includes every automatic
transition, and rules out a closed class outside the target. It also gives the
finite operation bound above. The verifier checks this support/rank
certificate and uses small exact rational recurrences for local waiting-time
and causal-memory controls. It must not enumerate the full history matrix: at
`b=4`, each of 84 sequential ports has the positive-probability candidate
sequence FRESH or ALIAS-then-FRESH, giving exactly `2^84` candidate-kind
histories before physical record substates are expanded.

Monte Carlo may illustrate a trajectory but is not confirmatory evidence. A
numeric runtime guard may return `NO_RESULT`; it cannot classify success.

### 9.2 Frozen maintenance defect

For every `b` representative, separately delete each possible depth-two to
depth-three leaf edge while the live reservoir coordinate is the root. Move
that one leaf descriptor from `Gamma_R` to the free pool; no descendants exist.
The immediate counts are

$$
g_3=b^3-1,
$$

so the constant-slope classifier must fail. The repair episode exposes the
same rewrite kinds at that port using the two episode-keyed semantic records
and attempt bits reserved in the initial state; construction-era terminal
records are never overwritten. It reuses the returned descriptor, credits the
new successful FRESH record, and automatically re-certifies shell three. Each
leaf-defect arm starts from an independent copied completed snapshot whose
four charged repair cells and records remain untouched. PERTURB unlocks that
reserve and has its own fifth external protocol cell and record; it does not
add internal resources. REPAIR-disabled
and descriptor-withheld copies must fail. Internal-edge deletion and cascading
subtree demotion are not part of the primary maintenance claim.

### 9.3 Frozen causal-memory intervention

The causal fixture uses the actual sequential scheduler at live port `a_1`.
It begins with one finite observation payload in `Q`,
`(a_1,FRESH,SUCCESS)`, one blank observation-memory record, one charged
observation work cell, and one blank operation record. The unit-clock ABSORB
transition copies that payload into the terminal observation record without
changing `Gamma` or acting on the port; its inverse is UNABSORB. The live
FRESH candidate retains its own blank episode record and attempt bit. After
ABSORB, the FRESH bucket contains one reachable passive SUCCESS observation
while the ALIAS bucket is blank. Hence

$$
R_F=\frac23,
\qquad
R_A=\frac12,
$$

and the exact hazards are

$$
q_F=\frac{175}{216},
\qquad
q_A=\frac{45}{64}.
$$

The probability that FRESH fires before ALIAS is

$$
\Pr(F<A)=\frac{q_F}{q_F+q_A}=\frac{280}{523}.
$$

If FRESH wins, the port costs one candidate-attempt work cell; if ALIAS wins,
FRESH then acts alone and the port costs two. Therefore

$$
\mathbb E[N_{\mathrm{attempt}}]
=2-\Pr(F<A)
=\frac{766}{523}.
$$

The predeclared `do(M)` intervention moves the SUCCESS record to an inactive
`a_2` FRESH bucket while keeping `Gamma,F,B_W,P,Q` and the live `a_1` cursor
fixed. Both live `a_1` buckets are then blank, so both hazards are `45/64`,

$$
\Pr(F<A)=\frac12,
\qquad
\mathbb E[N_{\mathrm{attempt}}]=\frac32.
$$

This is the sole primary memory-causality comparison. It changes the actual
primary next-action law and expected work without changing a non-memory
coordinate; no post-result search over memory states or statistics is
allowed.

### 9.4 Sealed held-out source

The probe fixture uses `E_tot=3lambda`. Its generative microedge rule, body
fixture, row-closure logic, and source digest are frozen before construction.
Literal endpoint edges are instantiated only after the words exist, from the
sealed rule and actual enumerated fibers. Neither endpoint multiplicities nor
generated traffic return to the builder.

### 9.5 Designed-input and thumb audit

Every installed choice is exposed before execution.

| Input | Frozen value | Role and status |
|---|---|---|
| Alphabet | `b in {2,3,4}` | constitutive substrate family; predicts different temperatures, not fitted |
| Primary energy quantum | `lambda=1` | shared energy unit; contact separately freezes `lambda_2=2` |
| Representative depth | shells `0..3` | smallest interval with three overidentifying slopes; observer fixture, not builder input |
| Inventory | `P_b` and off-shell `P_b+1` | finite physical scale; builder sees individual availability only |
| Diamond | `D=1` per enabled port proposal | one opportunity unit; no importance multiplier |
| Ruby prior | `Beta(1,1)` predictive mean | unique exchange-symmetric uniform Bernoulli prior |
| Opal | project two-channel formula, exact rational reduction | installed adaptive integration rule; not a thermodynamic law |
| Negative candidate | canonical parent `ALIAS` | explicit learning scaffold and failure control |
| Port scheduler | one canonical word/symbol cursor; two candidates at the live port | installed finite policy constitution; no hidden tie/order search |
| Action clock | `q_c=Phi_c` | declares construction time unit; no fitted multiplier |
| Protocol clock | `1` | time unit for automatic CREDIT/PROMOTE/PERTURB transitions |
| Work lift | `epsilon_W=1`, capacities derived in Section 9 | work unit and finite reversible resource; cannot alter shell/rate slopes |
| Probe body | `h=(1,2,1)`, `U=(0,lambda,2lambda)` | held-out overidentifying fixture |
| Thermal micrograph | sealed complete-bipartite unit edges plus REFRESH | independent strongly lumpable traffic rule |
| Prefix incidence | structural/certification only; zero live hazard | state-space construction scaffold, not an omitted thermal channel |
| Contact micrograph | sealed two-fiber complete-bipartite unit edges | independent finite T6 fixture |
| Maintenance defect | every depth-two to depth-three leaf deletion | exact no-descendant repair family |
| Memory intervention | one reachable FRESH success versus blank, then bucket swap | sole causal-memory test |
| Absorption fixture | one finite `(a_1,FRESH,SUCCESS)` payload and one reversible work/record cell | makes the prior observation reachable and ledgered |

These choices make a constructed existence witness; they are not claimed to
be universal constants of nature. None may change after a result is seen. An
unlisted multiplier, threshold, retry count, tolerance, shell filter, or
depth read is a hidden thumb and fails E15.

## 10. Frozen order of observation

For every specimen the evaluation order is:

1. record the initial complete state and verify that it is not already in
   `M_therm` for the claimed interval;
2. execute or solve the construction process without a traffic classifier;
3. enumerate actual shell microstates and freeze the state classification;
4. freeze `T_state` if and only if the result is `UNIQUE_CONSTANT`;
5. only then expose held-out reciprocal exchange channels;
6. classify `T_LDB` from every labelled rate pair without reading
   `T_state`;
7. compare the two frozen values exactly;
8. apply each defect, memory, poison-input, ledger, and contact control;
9. publish all classifications in one artifact.

No energy rescaling, shell exclusion, reverse-edge reassignment, or unit
change is allowed after step 3. The energy gauge is acknowledged, but it
cannot be used post hoc to force equality.

## 11. Frozen finite contact CTMC

Two independently constructed prefix reservoirs may be compared only in one
declared energy gauge. They have

$$
T_i=\frac{\lambda_i}{\ln b_i}.
$$

An allowed contact quantum `q` must satisfy

$$
q/\lambda_1\in\mathbb N,
\qquad
q/\lambda_2\in\mathbb N.
$$

For integer lattices the smallest choice is their least common multiple in
the shared unit. The contact witness freezes `q=2` and shared total reservoir
energy four. Its only two product-energy fibers are

$$
\mathcal C_A=R^{(1)}_0\times R^{(2)}_2,
\qquad
\mathcal C_B=R^{(1)}_2\times R^{(2)}_1.
$$

`A -> B` transfers two energy units from reservoir 2 to reservoir 1. Let

$$
\Omega_A=|\mathcal C_A|=b_1^0b_2^2,
\qquad
\Omega_B=|\mathcal C_B|=b_1^2b_2^1.
$$

The sealed contact generator places one reciprocal unit-clock edge between
every microstate in `C_A` and every microstate in `C_B`. It is therefore
strongly lumpable, irreducible, and row closed with

$$
K_{A\to B}=\Omega_B,
\qquad
K_{B\to A}=\Omega_A,
\qquad
K_{AA}=-\Omega_B,
\qquad
K_{BB}=-\Omega_A.
$$

The uniform law over the complete product microstates is the unique stationary
law. Its macro projection is

$$
\pi_A=\frac{\Omega_A}{\Omega_A+\Omega_B},
\qquad
\pi_B=\frac{\Omega_B}{\Omega_A+\Omega_B},
$$

and detailed balance gives zero stationary current. The finite irreducible
CTMC converges to this law from every initial distribution.

The diagnostic preparation is frozen before rates are exposed:

$$
p_A(0)=p_B(0)=\frac12,
$$

uniformly within each fiber. Define event current from `A` to `B` and energy
current into reservoir 1 as

$$
j_{A\to B}(0)
=p_AK_{A\to B}-p_BK_{B\to A},
\qquad
J_E(0)=q\,j_{A\to B}(0).
$$

The exact predictions are:

1. Equal temperature: `(b_1,lambda_1)=(2,1)` and
   `(b_2,lambda_2)=(4,2)` both give `T=1/ln 2`.
   `Omega_A=Omega_B=16`, so `j(0)=J_E(0)=0` and the diagnostic preparation is
   already stationary.
2. Unequal temperature: `(b_1,lambda_1)=(2,1)` and
   `(b_2,lambda_2)=(2,2)` give `T_2>T_1`.
   `Omega_A=4`, `Omega_B=8`, `j_A->B(0)=2`, and `J_E(0)=4>0`: energy initially
   flows from hotter reservoir 2 to colder reservoir 1. The final stationary
   current is zero, as it must be for a closed finite reciprocal contact.
3. Multiplying every microscopic contact clock by one common positive factor
   changes relaxation time and transient-current magnitude but not `pi`, the
   current sign, or either temperature.
4. A fitted traffic slope without matching state-count temperature fails
   contact admission before this generator is applied.

This is an endpoint-only finite contact claim. It contains both reservoirs,
all product microstates, the common energy gauge, and the contact generator in
the authoritative model. No ideal infinite bath is introduced, and no
stationary nonequilibrium current is claimed.

## 12. Controls and failure labels

| Arm | Frozen intervention | Required classification | First decisive gate |
|---|---|---|---|
| Static perfect tree | begin with actual `g_m=b^m`; disable builder/memory | operational thermal sector may pass; no endogenous creation or adaptation | E02/E04/E06 |
| Nonadaptive builder | replace every Ruby record by constant `R=1/2` | construction may pass; not adaptive | E05 |
| Legacy `a_P` gate | require `Phi>a_P` after nonempty history | exact deadlock from Section 7 | E06 |
| External thermostat | counts `(1,2,5,10)` but inject `beta=ln 2` into rates | rate-compatible only; no constant `T_state` | E03/E08 |
| Gibbs-installed | same irregular counts; impose `exp(-beta U)` weights | fitted representation only | E03/E08 |
| Target-count builder | pass `b^m`, target slopes, or a global deficit array | causally contaminated even if counts pass | E03 |
| Traffic-trained builder | reward agreement with sealed probe ratios | causally contaminated | E03 |
| Poison input | replace every forbidden field by two distinct sentinels | projected action/successor trace exactly unchanged | E03 passes or primary fails |
| Off-shell inventory | use `P_b+1` descriptors and `C_b+2` cells | one visible incomplete depth-four workspace node; promoted `T` unchanged | E07–E10 invariant |
| Descriptor permutation | reverse immutable free-descriptor IDs | same shell counts, classifications, and path-law quotient | E14 invariant |
| Energy shuffle | in binary witness swap energy labels of shells one and two | nonpositive/nonconstant state slopes; no unit rescue | E08 |
| Leaf deletion | remove each depth-three leaf as Section 9.2 | `g_3=b^3-1` while damaged; exact repair restores it | E08 then E12 pass |
| Repair disabled | same deletion with REPAIR support removed | damaged state remains outside `M_therm` | E12 |
| Descriptor withheld | same deletion without returning its descriptor | declared repair-resource premise false; classified resource obstruction | E12 not claimed |
| Port-law mutation | add one duplicate depth-three child | `g_3=b^3+1`; old temperature rejected | E08 |
| Memory clamp | blank both live Section 9.3 buckets | live FRESH-before-ALIAS probability becomes exactly `1/2` | E05 |
| Memory permutation | move the SUCCESS record from live FRESH to inactive `a_2` FRESH | probability changes from `280/523` to `1/2`; expected work from `766/523` to `3/2` | E05 passes |
| Hidden memory | project away the differing Section 9.3 records | one projected state has two successor laws | E01 |
| Ideal counters | replace finite records/work cells by unbounded counters | logical adaptive control only | E11/full label |
| Ledger leak | omit one charged work cell or operation record | working or augmented first law fails by one work unit | E11 |
| Activity only | multiply every thermal/contact microclock by two | stationary laws and temperatures fixed; relaxation/current magnitudes double | E09/E13 invariant |
| Reverse mismatch | delete one held-out reverse microedge | reverse support/symmetry and strong-lumping audit fail | E09 |

Unexpected control survival is a failure of the primary causal claim, not a
reason to weaken the control after execution.

## 13. Ledger boundary

The live generator is decomposed before evaluation:

$$
\mathcal L
=\mathcal L_{\mathrm{thermal}}
+\mathcal L_{\mathrm{build/repair}}
+\mathcal L_{\mathrm{learn}}
+\mathcal L_{\mathrm{protocol}}.
$$

At fixed `Gamma`, `L_thermal` is the reciprocal strongly lumpable equilibrium
sector tested in E3. BUILD, REPAIR, LEARN, and phase changes are driven
operations. They are not required to obey equilibrium detailed balance at
`T_state`, but they require an explicit controlled forward protocol, an
explicit time-reversed protocol on the enlarged state and finite work cells.
LDB and stochastic path-ratio claims are restricted to THERMAL and CONTACT.
The driven sector instead requires an injective source-to-target map, a named
inverse protocol, and exact work/resource closure. A phase flag that silently
deletes the inverse map fails complete state.

The live and inverse operations are frozen as follows. `epsilon_W` is one
symbolic work unit. Automatic protocol transitions have the unit clock; BUILD
and ALIAS have the rational candidate hazards from Section 7.

| Forward channel | Exact live source → target | Live hazard | Inverse-protocol map |
|---|---|---:|---|
| `FRESH` | open port + free descriptor + blank candidate record → provisional canonical binding in `Gamma_W` + `PENDING` | `Phi_c` | `UNFRESH` |
| `ALIAS/QUARANTINE` | open port + blank alias record → same open port + `FAILURE`; no alias enters `Gamma_R` | `Phi_c` | `UNFAIL` |
| `CREDIT` | next pending frontier record → `SUCCESS` | `1` | `UNCREDIT` |
| `PROMOTE` | fully certified workspace frontier + all credits terminal → same frontier in `Gamma_R` + next phase | `1` | `DEMOTE` |
| `PERTURB` | root occupied + selected depth-three leaf active + external charged cell + locked internal repair reserve → leaf descriptor free + shell damaged + REPAIR phase + reserve unlocked | `1` | `UNPERTURB` |
| `ABSORB` | finite observation payload + blank observation record + charged fixture cell → payload marked consumed + terminal bucket record + written operation record; `Gamma` unchanged | `1` | `UNABSORB` |
| `THERMAL/REFRESH` | fixed `Gamma,M,P,B_W`; `(x,i,r)→(y,j,r')` at fixed total energy | sealed unit microedge | identical reciprocal edge |
| `CONTACT` | fixed two-reservoir architectures; `(r_1,r_2) in C_A↔C_B` | sealed unit microedge | identical reciprocal edge |

ALIAS validation and quarantine are one atomic transition; there is no hidden
invalid intermediate. FRESH atomically changes the free-pool membership,
workspace binding, candidate attempt bit, semantic record, charged work cell,
and operation record. ALIAS/QUARANTINE atomically changes its attempt bit,
semantic record, charged work cell, and operation record while leaving the
port open. These atomic maps are exactly what the `C_b` resource proof counts.

Port closure and injectivity are pure predicates of `Gamma,Q`; their derived
certificate bits update inside the corresponding atomic map and do not create
unledgered transitions. CREDIT visits pending records by immutable descriptor
ID. PROMOTE is enabled only after the last CREDIT, atomically changes
`Gamma_W/Gamma_R`, the phase coordinate, one charged work cell, and one
operation record. It exposes the next frontier if free descriptors remain,
otherwise enters THERMAL. In a repair episode it re-certifies shell three and
returns to THERMAL. The inverse protocol applies the named maps in reverse
record order. The verifier must prove each pair is one-to-one on the enlarged
state and print both protocol traces.

Two energy boundaries are reported for every driven transition. For the
working subsystem excluding the work cells,

$$
\Delta E_{\mathrm{work}}
=Q_{\mathrm{work}}+W_{\mathrm{on}},
\qquad
W_{\mathrm{on}}=-\Delta E_{B_W}.
$$

For the augmented work lift including those cells,

$$
\Delta E_{\mathrm{aug}}
=Q_{\mathrm{external}}+W_{\mathrm{external}}.
$$

The primary BUILD/LEARN/REPAIR protocol has
`Q_external=W_external=Delta E_aug=0`; PERTURB's separately named external
cell is included only when reporting its own augmented boundary. The occupied
graph microstate, construction medium, free descriptors, memory, probe state,
and finite contact resources are included where energetic. The verifier must
not sum energies of mutually exclusive vertices as though every alternative
were occupied at once.

The theorem keeps a symbolic `epsilon_W>0`. The confirmatory fixture fixes
`epsilon_W=1` in its declared work unit. A charged work cell has energy
`epsilon_W`; a discharged cell has zero. Each elementary forward
logical operation converts one charged work cell to discharged while changing
one previously blank finite operation record from energy zero to
`epsilon_W`. For the controlled subsystem,

$$
\Delta E=\epsilon_W,
\qquad
W_{\mathrm{on}}=\epsilon_W,
\qquad
Q=0.
$$

For the enlarged subsystem including the work cell, `Delta E_complete=0`.
The time-reversed protocol clears that record and recharges the same cell.
Multi-stage records consume one cell per elementary state change. The numeric
choice of `epsilon_W` fixes a work unit but cannot alter construction hazards,
shell counts, `T_state`, or `T_LDB`.

Free/bound descriptors, semantic states `BLANK/PENDING/SUCCESS/FAILURE`, and
workspace/promoted architecture labels are isoenergetic construction media in
this minimal lift. Energy is carried by the occupied reservoir coordinate
`E_R(r)=lambda m`, the probe/contact coordinates, charged work cells, and
written operation records. Thus for the working subsystem excluding `B_W`,
`W_on=-Delta E_BW=epsilon_W`; for the augmented closed work lift including
`B_W`, the same event has zero total energy change. PERTURB uses its separately
named external work cell. THERMAL and CONTACT leave every work/record cell
fixed, conserve their declared total energy, and have `W_on=0`.

For the probe body alone on a THERMAL exchange,

$$
Q_X=\Delta U_X=-\Delta E_R,
\qquad
W_{\mathrm{on},X}=0.
$$

For body plus internal reservoir, total energy change is zero. CONTACT uses
the analogous internal heat transfer `q` between the two finite reservoirs.

The graph builder may consume work. That does not make the resulting
temperature exogenous: work supplies energy, while the forbidden-input test
asks whether it supplies the *target multiplicity law*. The two questions are
different.

The primary arm uses the finite record lift in Section 7. Ideal counters are a
logical control only and cannot receive the full label. No reset or stationary
infinite-learning claim is made. Reusing the finite memory would require a
separately declared reversible reset, saturation rule, or information sink.

Let `Y` be the terminal action outcome and `M_Y` its finite physical record.
The exact generator induces their joint distribution. The information report
is frozen as

$$
I(M_Y;Y)
=\sum_{m,y}p(m,y)\ln\frac{p(m,y)}{p(m)p(y)}.
$$

Because the terminal record is an exact copy of `Y`, `I(M_Y;Y)=H(Y)` on the
recorded support. This information is not assigned an energy by fiat; the
actual record work is already measured by the work-cell lift. The causal
one-port fixture and each construction quotient must report the exact joint
probabilities and stored information. No erasure term is claimed because no
record is reused. This is a finite outcome-copy/memory-causality account, not
a CAL-MEMORY-style stationary information-flow-rate claim.

No `S_tau`, `p_o`, TAU, or Xi value is evaluated in this apparatus. In
particular, `p_o=T_state` is not assumed, and no Xi drive is written without a
proved bridge between causal-path and microcanonical entropy. Heat, work, and
information remain determined by their own channels and physical records.

## 14. Primary gates

The final artifact must report each gate independently.

| Gate | Pass condition |
|---|---|
| E01 Complete state | replay from `Z` alone reproduces every successor hazard and ledger row |
| E02 Initially nonthermal | every primary initial state fails the claimed constant-temperature classifier at time zero |
| E03 Forbidden-input non-use | static audit and poison intervention both pass |
| E04 Xypher causal loop | Crystal perception, Praxor integration/selection, Thermo work-cell enablement/debit, graph action, outcome, and learning are executable and state-changing |
| E05 Memory causal | `do(M)` changes a next-action law and a downstream path, mean-time, or work statistic |
| E06 Creation | every required initial state reaches `M_therm` with exact probability one and finite mean hitting time |
| E07 Actual multiplicity | enumerated counts equal the frozen prediction on every admitted shell |
| E08 Unique state temperature | all shell slopes agree exactly and are positive; no fit tolerance is used |
| E09 Independent thermal sector | sealed labelled channels have reverse support, row closure, strong lumpability, unique `T_LDB`, unique Gibbs/microcanonical stationary law, and relaxation |
| E10 Temperature equality | `T_LDB=T_state` by exact symbolic log/prime-exponent identities |
| E11 Physical ledgers | every channel closes heat/work/resource accounts; the finite record/inverse lift is bijective; and exact outcome-record mutual information is reported |
| E12 Maintenance | every admissible finite defect episode returns with probability one and finite mean return time using declared resources |
| E13 Contact | sealed two-fiber CTMC has reverse support, row closure, unique equilibrium/relaxation, zero equal-T current, and the frozen unequal-T initial current |
| E14 Controls | every negative and invariance control receives its frozen classification |
| E15 No hidden fit | source and artifacts contain no result-dependent parameter, tolerance, shell selection, or unit rescue |

The primary claim passes only if E01–E15 all pass. Partial outcomes retain
their narrower labels.

## 15. Falsifiers and downgrade rules

The full claim is false for this construction if any of the following occurs:

- the builder requires a forbidden input or an equivalent target-coded
  reward;
- actual shell counts do not acquire the predicted relation;
- the relation exists only because nonmatching shells were removed after
  inspection;
- `T_state` is absent, nonunique, or differs from independently classified
  `T_LDB`;
- memory interventions do not affect action and downstream construction;
- a declared repairable defect is not restored with probability one;
- heat/work closure requires treating TAU as an unrecorded energy source;
- omitted memory, scheduling, or finite resources are needed to make the
  process Markov;
- vertices counted as distinct shells are paths or labels for the same live
  configuration rather than injectively distinct executable microstates;
- driven one-way repair is presented as equilibrium or as indefinitely
  self-powered maintenance;
- an externally thermostatted or target-fitted control is indistinguishable
  from the primary arm under the causal tests;
- contact contradicts the frozen total-multiplicity prediction.

Downgrade labels are cumulative and exact:

- structural results E1–E3 without the dynamic primary gates:
  `STATIC STRUCTURAL THEOREM`;
- state-count plus traffic without creation: `OPERATIONAL THERMODYNAMIC GRAPH`;
- creation without causal learning: `ENDOGENOUS NONADAPTIVE CONSTRUCTOR`;
- adaptive creation without maintenance: `ADAPTIVE THERMODYNAMIC CONSTRUCTION`;
- state-dependent positive slopes only: `ENDOGENOUS STATE-DEPENDENT TEMPERATURE`
  if all matching operational gates pass;
- full E01–E15: `ENDOGENOUSLY THERMODYNAMIC ADAPTIVE XYPHER`;
- any circular input or post-hoc fit: `NOT ADMISSIBLE`.

No failure in this finite witness would prove that endogenous digital
thermodynamics is impossible in all constructions. It would identify the
precise failed condition and narrow the theorem boundary.

## 16. Claim boundary if the witness passes

A full pass would establish:

1. at least one finite adaptive digital graph can create and repair an
   energy–multiplicity relation through local graph dynamics and finite
   declared work expenditure;
2. that relation can independently ground an operational temperature;
3. the same temperature can govern held-out reciprocal exchange and finite
   contact without being injected into construction or rates;
4. within the adopted finite operational definition, E5 gives the admission
   conditions, while E1–E4 provide independently checkable structural,
   kinetic, and dynamic necessary-and-sufficient subresults;
5. a local graph recursion is one exact mechanism by which a global
   thermodynamic state variable can arise.

It would not establish:

- that every Xypher is thermodynamic;
- that the prefix mechanism is unique;
- that causal path entropy universally equals thermodynamic entropy;
- that historical `a_P` is physical temperature;
- that TAU is heat, work, money, or an equation of state;
- that the apparatus discovers new fundamental physics of the material
  universe;
- that an unbounded self-powered digital steady state exists;
- that the prefix grammar itself was learned or arose without designed local
  laws;
- that a blockchain implementation inherits the theorem without a separate
  complete-state, reservoir, channel, and ledger proof.

## 17. The frontier after this boundary

The construction is scientifically interesting precisely because it exposes
the next irreducible question:

> Can generic adaptive local laws discover a thermodynamic manifold without
> being born with a prefix-port grammar that already makes such a manifold
> reachable?

CAL-ENDO first asks the cleaner existence and characterization question. If
the prefix witness passes, later work may widen the initial graph class,
weaken the local grammar, allow the alphabet itself to be learned, and test
whether temperature becomes an attractor rather than a consequence of one
known constructive invariant.

That later frontier must not be imported into this first confirmatory claim.
