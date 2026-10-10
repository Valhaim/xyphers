---
title: Control Information Thermodynamics Boundary
aliases:
  - CAL-CEF-2 Boundary
  - Foresight Notebook Boundary
tags:
  - domain/physics
  - type/preregistration
  - topic/thermodynamics
  - topic/xypher
  - topic/information-thermodynamics
domain: Physics
type: preregistration
status: draft-no-evaluation
td: td-ac9ae6
created: 2026-10-10
updated: 2026-10-10
related:
  - "[[Xypher Operational Thermodynamics Boundary]]"
  - "[[Causal Entropic Thermodynamics Boundary]]"
  - "[[Causal Entropic Thermodynamics Result]]"
---

# Control information thermodynamics boundary (CAL-CEF-2)

## 0. The landing point

CAL-CEF-1 showed that a traveller who holds one tau-step plan as executable
state, on an archipelago whose timber comes from a reservoir of independently
counted temperature, makes the log-count of its committable futures the
configuration entropy of its world. Its section 11 named the next step: give
the destination of the held plan its own register, measure the control of plan
over destination inside the same energy account, and test the correlation free
energy `alpha ln 2` per bit against a shuffled pairing with equal marginals and
equal average energy.

[[Intelligence as Physical Units]] specifies the same experiment as the matched
control instrument: command register and outcome inside one complete state, the
exact outcome law of every command, the control capacity, a shuffled deck with
equal energy, and a temperature that still reads the same on the enlarged
world.

This boundary asks:

> When a notebook inside the world records where the traveller's held plan
> ends, is a correct record worth exactly `alpha ln 2` of free energy per bit
> of information it carries about the held plan's destination, as the
> free-energy gap to a shuffled pairing of equal energy and as the equilibrium
> exchange constant of a reversible feedback move that trades the record's
> correlation for one stored timber packet, while wiping the notebook carries
> the Landauer price and the enlarged world keeps the reservoir's temperature?

With a reservoir whose arrangements double per packet, `alpha ln 2 = lambda`:
one bit of foresight is worth one timber packet.

No verifier output, trajectory, or evaluated gate result has been generated
under this version. Design-time calculations used to state predictions are
disclosed in section 5.4. The document remains a draft until its exact commit
is independently reviewed and pushed.

## 1. The observable decision

The command is the held plan `p`. The outcome is the island where the plan
ends, `d(p) = v_tau`. Execution of a held plan is deterministic along its
route, so the command-to-outcome channel at configuration `G` is the map
`d : Pi_tau(G) -> V`. The record equals the realized arrival only if the plan
is walked without an intervening REPLAN; this boundary speaks of the held
plan's destination, not of realized trajectories.

The control capacity at `G` is `log2 |D(G)|` bits, where `D(G)` is the set of
islands at which some valid plan ends. The capacity witness law puts weight
`1/|D(G)|` on the lexicographically first plan ending at each reachable
destination.

The notebook `y` is a physical register in the complete state. A correct
record means `y = d(p)`. Two information values are tested:

```text
I(G)   = H(d) under the Crystal's uniform plan law
       = - sum_y (n_y/N) ln(n_y/N),          exp[N I(G)] = N^N / prod_y n_y^(n_y),
I*(G)  = ln |D(G)| under the capacity witness law,  exp[|D| I*(G)] = |D|^|D|,
```

with `N = N_tau(G)` and `n_y = n_y(G)` the number of valid plans ending at
`y`. `I(G) <= I*(G)`, with equality exactly when every reachable destination
ends the same number of plans. The article's control value
`V_ctrl = alpha ln 2 * I_ctrl` is the capacity value `alpha I*(G)`. The
feedback exchange of section 4.5 runs under the Crystal's law, so its average
worth is `alpha I(G)`.

Every identity in this boundary is checked exactly, without evaluating a
logarithm (section 6). The record's worth is not assigned to any channel. It is
predicted as an equilibrium exchange constant of the constructed dynamics
(section 4.5) and as a free-energy gap between prepared laws over the same
registers (section 4.4).

## 2. Independent thermodynamic definition

The target class is the finite equilibrium thermodynamic graph system defined
by T1--T6 in [[Xypher Operational Thermodynamics Boundary]] (section 2),
unchanged. T1--T5 are claimed for the enlarged world. T6 (contact) is not
claimed here; CAL-CEF-1 established it for the uninstrumented world.

The T4 register clause applies: the notebook is an information register that
affects future behaviour, so every reset of it must move energy to a named
counter-reservoir. In this construction the reset is ERASE, the only channel
that maps more than one written notebook value to the same blank system state
at fixed plan, cursor, configuration, and stockpile. MEASURE and UNMEASURE are
one-to-one on the joint (plan, notebook) register, because UNMEASURE fires only
when `y = d(p)`; like the plan-register writes of CAL-CEF-1 (its gate G04),
they are reversible channels within the complete state and need no
counter-reservoir.

## 3. The construction

### 3.1 Archipelago, plans, configurations

As CAL-CEF-1, sections 3.1--3.2, unchanged: islands `V = {0, ..., N-1}`, home
`h = 0`, candidate bridges the `K = N(N-1)/2` unordered pairs in lexicographic
order, configurations ordered by bit pattern, plans `p = (v_0, ..., v_tau)`
with `v_0 = h` and each step a stay or a crossing of a built bridge,
`N_tau(G) = |Pi_tau(G)| = e_h^T L_G^tau 1`, plans ordered lexicographically.

For each `G` and island `y`, `n_y(G)` is the number of plans in `Pi_tau(G)`
ending at `y`, and `D(G) = {y : n_y(G) > 0}`.

### 3.2 System, registers, and reservoir

A system state is `z = (G, p, t, y, w)`:

- `p` in `Pi_tau(G)`, cursor `t` in `{0, ..., tau}`, traveller on `v_t`;
- notebook `y` in `V u {blank}`; the written alphabet is `V`, of size
  `A = N`;
- stockpile level `w` in `{0, ..., W}`: a single-arrangement energy register,
  a weight, holding `w` timber packets.

The thermodynamic system comprises the bridges, plan, cursor, notebook, and
stockpile. Each bridge and each stockpile packet stores energy `lambda = 1`, so
the system energy is

```text
U(z) = lambda (|G| + w),
```

and `S(z) = 0` up to a common constant. A reservoir holds the remaining packets
of the fixed total `E_tot = K + W`, so `E_R = K + W - |G| - w >= 0` in every
state, with `b^(E_R)` distinguishable arrangements labelled
`0, ..., b^(E_R) - 1`. This is the independently declared equation of state:

```text
1/alpha = Delta S_R / Delta E_R = ln b,      alpha = lambda / ln b.
```

Every packet that leaves the reservoir is heat into the system, and external
work is zero; the witness is undriven, as T4 requires. A HARVEST increment of
the stockpile is work-like only in the pathwise sense that it raises the
system's energy without raising its entropy.

A complete state is `(z, r)` with `r` a reservoir label for `E_R(z)`. Complete
states are ordered by configuration, plan, cursor, notebook (`blank` first,
then islands ascending), stockpile, then reservoir label.

### 3.3 Channels

All micro channels have unit hazard `kappa = 1`. Every channel is listed with
its reverse.

- **STEP, REPLAN, BUILD, DISMANTLE**: exactly as CAL-CEF-1, section 3.4. They
  never read or write the notebook or the stockpile. REPLAN may leave a
  previously correct record stale.
- **MEASURE / UNMEASURE** (copy the destination of the held plan): at cursor
  `t = 0`, `(G, p, 0, blank, w, r) <-> (G, p, 0, d(p), w, r)`. MEASURE writes
  into a blank notebook only; UNMEASURE clears a record only when it equals
  `d(p)`. No energy changes.
- **HARVEST / COMMIT** (feedback): at cursor `t = 0`, for a record `y` with
  `y = d(p)` and `w < W`, connect `(G, p, 0, y, w, r)` to
  `(G, p', 0, y, w + 1, r')` for every `p'` in `Pi_tau(G)` (including `p`)
  and every reservoir label `r'` at energy `E_R - 1`. HARVEST releases the
  held plan to the full plan set while one packet moves from the reservoir to
  the stockpile; the record stays. COMMIT is the reverse: from any plan, with
  record `y`, one stockpile packet returns to the reservoir and the plan
  becomes one ending at `y`.
- **ERASE / SCRIBBLE** (Landauer): at cursor `t = 0`, for a written record `y`
  and `w >= 1`, connect `(G, p, 0, y, w, r)` to `(G, p, 0, blank, w - 1, r'')`
  for every reservoir label `r''` at energy `E_R + 1`. ERASE resets the
  notebook while one stockpile packet is released as heat into the reservoir.
  SCRIBBLE is the reverse: a blank notebook takes any written symbol while one
  packet moves from the reservoir to the stockpile.

COMMIT reads only the record: it compresses any plan into those ending at `y`.
HARVEST is its reverse, so its domain is the image of COMMIT, the states with
`d(p) = y`. The feedback is therefore a joint interaction of the plan and the
notebook, as in autonomous Maxwell-demon models whose transitions read the
demon and the bit together. Both registers and the reservoir are in the
complete state, so the interaction carries its own accounting. A controller
that acts on stale records using the record alone is outside this boundary.

No channel hazard reads `N_tau`, `n_y`, `I`, `I*`, any exchange constant,
THAIM, or Xi.

### 3.4 Xypher slot reading

| Slot | Binding |
|---|---|
| Graph Substrate | bridge configurations, held plan, cursor, notebook, stockpile |
| Crystal | REPLAN: uniform executable resolution over `Pi_tau(G)`; `S_tau(G) = ln N_tau(G)` |
| Thermo | reservoir law `g_R = b^(E_R)`, energy `U = lambda (|G| + w)`, THAIM pair and Xi of CAL-CEF-1 (neither computed nor evaluated here) |
| Praxion | base actor: makes any permitted move at unit symmetric hazard |
| Action | BUILD/DISMANTLE one bridge at the traveller's island; HARVEST/COMMIT; ERASE/SCRIBBLE |
| Resolution | STEP; REPLAN writes the plan; MEASURE writes the notebook |
| Thaw | the declared reverse of each channel |
| Memory | held plan and cursor: written by REPLAN, STEP, HARVEST, and COMMIT; read by STEP, BUILD, DISMANTLE, MEASURE, UNMEASURE, and HARVEST. Notebook: written by MEASURE and SCRIBBLE; cleared by UNMEASURE and ERASE; read by MEASURE, UNMEASURE, HARVEST, COMMIT, ERASE, and SCRIBBLE. No learning. |
| Ruby | empty |
| Opal / Phi | absent |

## 4. Derived consequences

The verifier checks each of these on the enumerated object; it does not assume
them. Section 7 classifies which checks are independent tests and which are
consequences of earlier gates.

### 4.1 System-state thermodynamics

Lumping reservoir labels gives system-state hazards: a channel from `z` to
`z'` that changes `E_R` has hazard `b^(E_R(z'))`; channels that do not change
`E_R` have hazard 1. Local detailed balance reads

```text
k(z -> z') / k(z' -> z) = b^(-[U(z') - U(z)]/lambda) = exp{-[U(z') - U(z)]/alpha}.
```

One reservoir governs bridge traffic (BUILD/DISMANTLE), feedback traffic
(HARVEST/COMMIT), and erasure traffic (ERASE/SCRIBBLE) alike. If the
system-state graph is connected, the unique stationary law is

```text
pi(z) proportional to b^(-(|G| + w)),
```

uniform over complete states, with detailed balance on every channel.

### 4.2 The undistorted world

Notebook and stockpile values are independent of configuration in the state
space, so

```text
pi(G) = N_tau(G) b^(-|G|) / Z,      Z = sum_G N_tau(G) b^(-|G|),
```

as in CAL-CEF-1. At equilibrium the notebook is uniform over its `A + 1`
values, `pi(w) proportional to b^(-w)`, and at cursor 0, for every `(G, w)`,
plan and notebook are independent: the fraction of written records that are
correct is `1/A`. The equilibrium world carries no record information. A
correct record is a prepared condition whose free energy is measured against
equilibrium-connected alternatives in sections 4.4--4.6.

### 4.3 The control channel

For each `G`, the verifier lists `n_y(G)` for every island and `|D(G)|`. Under
the capacity witness law, the outcome law is uniform on `D(G)`. Under the
uniform plan law, the outcome law is `n_y/N`.

### 4.4 Correlation free energy and the shuffled deck

Fix `G`, cursor `0`, and stockpile `w`. The correlated law `A_G` is the
stationary law conditioned on a correct written record, `y = d(p)`: it is
uniform over the `N` pairs `(p, d(p))`. The shuffled law `B_G` is the product
of the two marginals of `A_G`: plan uniform over `Pi_tau(G)`, record `y` with
probability `n_y/N`, independent. `A_G` and `B_G` are prepared laws, not the
stationary law.

Both laws put all weight on states of energy `lambda (|G| + w)`, and no energy
in the model depends on the notebook value. The nonequilibrium free energy
`F = <U> - alpha S` therefore differs only through entropy:

```text
F(A_G) - F(B_G) = alpha [S(B_G) - S(A_G)] = alpha I(G),
exp{N [S(B_G) - S(A_G)]} = N^N / prod_y n_y^(n_y).
```

The capacity pair is formed the same way: `A*_G` is uniform over the
`|D(G)|` pairs `(p*_y, y)` of the capacity witness, `B*_G` the product of its
marginals, and `exp{|D| [S(B*_G) - S(A*_G)]} = |D|^|D|`, a gap of
`alpha I*(G) = alpha ln |D(G)|`.

Given the conditioned laws, these gaps are identities of information theory.
Their content is that the conditioned law `A_G` computed from the generator is
uniform over correct pairs, and that the energy function does not see the
pairing. In bits, a correct record is worth `alpha ln 2` per bit.

### 4.5 The feedback exchange

For `G`, a reachable destination `y`, and `w < W`, define two macrostates at
cursor 0:

```text
C(G, y, w)     = { (G, p, 0, y, w) : d(p) = y }          (correct record),
L(G, y, w + 1) = { (G, p', 0, y, w + 1) : p' any plan }  (released, one packet stored).
```

`L(G, y, w + 1)` contains `C(G, y, w + 1)`; this is harmless because only
HARVEST and COMMIT connect `C(G, y, w)` with `L(G, y, w + 1)`. HARVEST
connects every state of `C(G, y, w)` to every state of `L(G, y, w + 1)`, and
COMMIT is its reverse. With

```text
kbar(X -> Y) = [ sum over z in X of pi(z) sum over z' in Y of k(z -> z') ] / pi(X),
```

the exchange constant is

```text
K(G, y) = pi(L(G, y, w + 1)) / pi(C(G, y, w)) = kbar(C -> L) / kbar(L -> C)
        = N_tau(G) / [n_y(G) b],
```

independent of `w`. A correct record of destination `y` trades for one stored
packet at equilibrium odds `K`; it is worth `log_b[N/n_y]` packets, its
surprisal in base `b`. Averaged under the uniform plan law,

```text
prod_y [b K(G, y)]^(n_y) = N^N / prod_y n_y^(n_y) = exp[N I(G)],
```

so the average worth of a correct record, in packets, is `I(G)/ln b`; at
`b = 2` it is the record's information in bits. Where `n_y = N/b` for every
reachable `y`, `K = 1` exactly: one base-`b` digit of foresight trades evenly
for one packet. In the primary witness this happens at `{0,1}` and `{0,2}`,
where the held plan ends at home or at the bridged island with two plans each:
one bit, one packet.

`K` is a ratio of equilibrium masses and the kbar equality follows from flux
equality, so both are identities once the stationary law of section 4.1 holds.
The construction's content is that a local reversible feedback move, a joint
interaction of the held plan and the notebook, realizes this exchange between
a correct record and the stockpile at the temperature of an independently
counted reservoir, and that the generator-derived constants reproduce the
shuffled-deck gap of section 4.4.

### 4.6 Landauer erasure and the blank page

For `G`, cursor 0, plan `p`, and `w >= 1`, the written-notebook macrostate
`R(G, p, w)` (any of the `A` symbols, at fixed plan) and the blank state
`B(G, p, w - 1)` satisfy

```text
pi(B(G, p, w - 1)) / pi(R(G, p, w)) = b / A.
```

Wiping, through ERASE, a notebook whose symbol is uniform over the `A` values
at a fixed plan costs one packet, and that exchange is balanced exactly when
the alphabet holds one base-`b` digit, `A = b`. With three islands at `b = 2`
the ratio is `2/3`: a uniform three-symbol record holds `log2 3` bits, more
than one packet can pay to erase. A record that matches the held plan can
instead be uncopied at no energy cost through UNMEASURE, as in Bennett's
reversible measurement. Read the other way, SCRIBBLE turns a blank page into
stored timber at odds `A/b`: a blank register is fuel.

### 4.7 No free lunch

Every channel obeys detailed balance, so every net current vanishes at
equilibrium: the expected net packet current into the stockpile is exactly
zero, and so is the expected net current between written and blank notebook
states. A packet can be gained along several routes, for example MEASURE,
HARVEST, REPLAN to a plan ending at the record, then UNMEASURE. Each such route
is exactly `b` times less probable than its reverse, so nothing is gained on
average.

## 5. Exact predictions

### 5.1 Primary witness: N = 3, tau = 2, b = 2, W = 2

Candidate bridges `{0,1}`, `{0,2}`, `{1,2}`; home `0`; `alpha = 1/ln 2`;
`E_tot = 5`; notebook alphabet `A = 3`.

| Configuration | `N_2` | `n_0, n_1, n_2` | `|D|` | `K(G, y)` for reachable `y` | `exp[N I]` |
|---|---:|---|---:|---|---:|
| none | 1 | 1, 0, 0 | 1 | 1/2 | 1 |
| {0,1} | 4 | 2, 2, 0 | 2 | 1, 1 | 16 |
| {0,2} | 4 | 2, 0, 2 | 2 | 1, 1 | 16 |
| {0,1},{0,2} | 7 | 3, 2, 2 | 3 | 7/6, 7/4, 7/4 | 823543/432 |
| {1,2} | 1 | 1, 0, 0 | 1 | 1/2 | 1 |
| {0,1},{1,2} | 5 | 2, 2, 1 | 3 | 5/4, 5/4, 5/2 | 3125/16 |
| {0,2},{1,2} | 5 | 2, 1, 2 | 3 | 5/4, 5/2, 5/4 | 3125/16 |
| all three | 9 | 3, 3, 3 | 3 | 3/2, 3/2, 3/2 | 19683 |

Capacity witness plans (lexicographically first per reachable destination):
none and `{1,2}`: `000`; `{0,1}`: `000, 001`; `{0,2}`: `000, 002`;
`{0,1},{0,2}` and all three: `000, 001, 002`; `{0,1},{1,2}`: `000, 001, 012`;
`{0,2},{1,2}`: `000, 021, 002` (listed by destination `0, 1, 2`). `exp[|D| I*] = |D|^|D|` is `1`, `4`, `4`,
`27`, `1`, `27`, `27`, `27` in configuration order. `I(G) = I*(G)` exactly at
none, `{0,1}`, `{0,2}`, `{1,2}`, and all three.

There are 1,296 system states and 7,308 complete states. The configuration law
is `N_2(G) 2^(-|G|) / (87/8)`, with equilibrium mean bridge count `131/87`;
the equilibrium mean stockpile is `4/7`; the notebook is blank with
probability `1/4`; the erasure ratio is `2/3`.

### 5.2 Confirmatory family

| `(N, tau, b, W)` | Configurations | System states | Complete states | `Z` | Mean bridges | Erasure `b/A` | Mean stockpile | Blank | Configurations with `K = 1` for every reachable `y` and `|D| >= 2` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| (3, 1, 2, 2) | 8 | 384 | 2,520 | 45/8 | 19/15 | 2/3 | 4/7 | 1/4 | {0,1}; {0,2}; {0,1},{1,2}; {0,2},{1,2} |
| (3, 2, 2, 2) | 8 | 1,296 | 7,308 | 87/8 | 131/87 | 2/3 | 4/7 | 1/4 | {0,1}; {0,2} |
| (3, 3, 2, 2) | 8 | 4,128 | 20,720 | 185/8 | 313/185 | 2/3 | 4/7 | 1/4 | {0,1}; {0,2} |
| (3, 2, 3, 2) | 8 | 1,296 | 26,208 | 56/9 | 5/4 | 1 | 5/13 | 1/4 | all three |
| (3, 1, 3, 1) | 8 | 256 | 3,072 | 32/9 | 1 | 1 | 1/4 | 1/4 | {0,1},{0,2}; all three |
| (4, 1, 2, 1) | 64 | 3,200 | 43,740 | 729/32 | 7/3 | 1/2 | 1/3 | 1/5 | the 24 configurations in which home has exactly one bridge |
| (4, 2, 2, 1) | 64 | 13,440 | 153,090 | 1701/32 | 55/21 | 1/2 | 1/3 | 1/5 | {0,1}; {0,2}; {0,3}; {0,3},{1,2}; {0,2},{1,3}; {0,1},{2,3} |

`K(G, y) = N_tau(G) / [n_y(G) b]` for every configuration and reachable
destination of every case. For `N = 3` cases, complete states and every micro
channel are enumerated explicitly. For `N = 4`, system states and every
system-state channel are enumerated with lumped reservoir hazards `b^(E_R)`;
G05 is then not applicable and G03 is checked at system-state level.

### 5.3 Headline numbers

- `(3, 2, 2, 2)` at `{0,1}`: one bit of foresight, `K = 1`.
- `(3, 2, 3, 2)` and `(3, 1, 3, 1)` with every destination equally reachable
  over three islands: one trit of foresight at `b = 3`, `K = 1`.
- `(3, 2, 2, 2)`, all three bridges: `log2 3` bits against one packet,
  `K = 3/2`.
- A record of a certain outcome (`|D| = 1`): `K = 1/b`. It is worth nothing.
- `(3, *, 3, *)`: erasing a uniform three-symbol notebook costs exactly one
  packet.

### 5.4 Design-time disclosure

Before this draft, a design script computed `N_tau(G)`, `n_y(G)`, `K(G, y)`,
`exp[N I(G)]`, the capacity witness plans, the system- and complete-state
counts, `Z`, the mean bridge count, the erasure ratio, the blank probability,
and the mean stockpile for the family and for `W = 3`, by direct enumeration of
plans and closed-form counting. An independent pre-freeze referee then built,
in a separate throwaway implementation, the generator of every family case
(explicit complete states for `N = 3`) and reported agreement with sections
5.1--5.2. For the primary under each control C01--C09 it evaluated G01--G05
and G11 checks 1--2 in the stated order, plus G08 for C09, and reported
agreement with every expected first failure in section 8. It did not run the
apparatus of section 6, which did not yet exist.

## 6. Apparatus

A standalone dependency-free Rust crate at
`research/physics/xypher-control-information-proof/`, with a prospective
Futuruna construction `xypher.runa`, mirroring the CAL-CEF-1 apparatus. All
arithmetic is exact integer or reduced rational arithmetic. No floating-point
acceptance path, random seed, sampling, simulation length, or tolerance is
permitted.

Products of the form `prod_y n_y^(n_y)` are compared as exact prime-exponent
vectors. Entropies are represented as rational-coefficient vectors over
`ln(prime)`: for a law with rational atoms `q`, `S = -sum q ln q` is the
vector `-sum_q q e(q)`, where `e(q)` is the prime-exponent vector of `q`. Two
such vectors are equal if and only if their coefficients agree, which is exact
because the logarithms of distinct primes are linearly independent over the
rationals.

The stationary law is obtained from the constructed generator: fix
`pi(z_0) = 1` at the first system state, propagate
`pi(z') = pi(z) k(z -> z') / k(z' -> z)` along a breadth-first spanning tree,
verify detailed balance on every edge, and normalise. G07 check 2 compares
this propagated law with `b^(-(|G| + w))`; every later gate uses the propagated
law. Macrostate masses, exchange constants, and conditioned laws are computed
from it, never from the closed forms they are checked against.

The Futuruna file declares the archipelago, plan validity, the notebook and
stockpile registers, every channel's permission rule, the reservoir law, and
the predicted exchange constants. It does not consume stationary weights,
averaged rates, or rate ratios.

The Rust verifier enumerates plans by depth-first search, computes
`e_h^T L_G^tau 1` by integer matrix powers, enumerates system and complete
states, constructs every channel from the local rules of section 3.3, and
evaluates the gates.

## 7. Acceptance gates

Gates run G01 to G14. Within each gate, checks run in the listed order, each
over all configurations (and states) in canonical order before the next check
begins. The first failure is the first failing check of the lowest-numbered
failing gate, at the first configuration in canonical order.

| Gate | Checks in order |
|---|---|
| G01 Crystal and command grounding | (1) `N_tau(G) >= 1`; (2) depth-first plan count equals `e_h^T L_G^tau 1`; (3) REPLAN at each cursor-0 state is uniform over its positive-probability outcomes with alphabet `Pi_tau(G)`; (4) `sum_y n_y(G) = N_tau(G)`; (5) every `(G, y, w)` slice has `(tau + 1) N_tau(G)` system states. |
| G02 executable registers | (1) every channel's destination is a state of the declared state space and holds a plan valid in its configuration; (2) (a) every STEP moves along a bridge of the current configuration or stays, (b) every BUILD/DISMANTLE is incident to the traveller's island, (c) no DISMANTLE removes a bridge used by the held plan, (d) plan contents are read: in the primary witness, two system states of `{0,1}` at cursor 0 with equal notebook and stockpile have different DISMANTLE sets; (3) MEASURE writes exactly `d(p)` into a blank notebook at cursor 0, and UNMEASURE clears only a record equal to `d(p)`; (4) HARVEST leaves only a record equal to `d(p)` at cursor 0 with `w < W`, keeps the record, and reaches every plan of `Pi_tau(G)` with stockpile `w + 1`; COMMIT reaches only plans ending at the record; (5) ERASE leaves only a written record at cursor 0 with `w >= 1` and arrives at a blank record with stockpile `w - 1`; SCRIBBLE leaves only a blank record at cursor 0 with `w < W` and writes every symbol of `V` with stockpile `w + 1`; (6) STEP, REPLAN, BUILD, and DISMANTLE never change the notebook or stockpile. |
| G03 reciprocal generator | (1) every positive channel has a positive reverse; (2) equal unit hazards in both directions at micro level (`N = 3`) or lumped hazards `b^(E_R)` (`N = 4`); (3) exact row closure. |
| G04 pathwise accounting | (1) every transition conserves `U + E_R`; (2) BUILD/DISMANTLE exchange `lambda` between bridges and reservoir, HARVEST/COMMIT between reservoir and stockpile, ERASE/SCRIBBLE between stockpile and reservoir; external work is zero; (3) every ERASE moves exactly one packet from the stockpile into the reservoir, and every SCRIBBLE one packet from the reservoir into the stockpile; (4) MEASURE/UNMEASURE, STEP, and REPLAN change no energy. |
| G05 reservoir lumpability | (`N = 3`) every complete state in a system state sends the same total rate into every other system state; lumped hazards are `b^(E_R(z'))` for energy-changing channels and 1 otherwise. N/A for `N = 4`. |
| G06 one temperature, three traffics | (1) bridge traffic, (2) feedback traffic, and (3) erasure traffic each satisfy `k(z -> z')/k(z' -> z) = b^(-[U(z') - U(z)]/lambda)` with the `b` of the reservoir law; (4) `S(z)` is constant. |
| G07 connectivity and equilibrium | (1) the system-state graph is connected; (2) the propagated stationary law equals `b^(-(|G| + w))` up to normalisation; (3) every channel's flux products are equal. |
| G08 undistorted world | (1) the configuration marginal equals `N_tau(G) b^(-|G|)/Z`, with `Z` and mean bridges as section 5; (2) the notebook marginal is uniform over `A + 1` values, with blank probability as section 5; (3) `pi(w) proportional to b^(-w)`, with mean stockpile as section 5; (4) at cursor 0, for every `(G, w)`, plan and notebook are independent, and the fraction of written records that are correct is `1/A`; (5) the system-state and complete-state totals match section 5. |
| G09 control channel | For every `G`: (1) `n_y(G)` and `|D(G)|` as computed by enumeration, with primary values as section 5.1; (2) the capacity witness (lexicographically first plan per reachable destination, primary plans as section 5.1) yields a uniform outcome law on `D(G)`; (3) `exp[N I(G)] = N^N / prod_y n_y^(n_y)` from the uniform-plan outcome law. |
| G10 feedback exchange | For every `G`, reachable `y`, and `w < W`: (1) every state of `C(G,y,w)` has HARVEST channels to every state of `L(G,y,w+1)` and to no other state, and every state of `L(G,y,w+1)` has COMMIT channels exactly to the states of `C(G,y,w)`; (2) `pi(L)/pi(C) = N_tau(G)/[n_y(G) b]` from the propagated stationary law; (3) `kbar(C -> L)/kbar(L -> C)` equals it; (4) `prod_y [b K(G,y)]^(n_y) = N^N / prod_y n_y^(n_y)`; (5) primary: `K = 1` at `{0,1}` and `{0,2}`, and the section 5.1 table. |
| G11 shuffled deck | For every `G` and `w`, at cursor 0: (1) the conditioned law `A_G` computed from the propagated stationary law is uniform over `{(p, d(p))}`; (2) `B_G` has the same plan and record marginals as `A_G`; (3) every state of the model has energy independent of the notebook value; (4) `exp{N [S(B_G) - S(A_G)]} = N^N / prod_y n_y^(n_y)`, computed from the two laws; (5) it equals `prod_y [b K(G,y)]^(n_y)` from G10; (6) for the capacity pair, `exp{|D| [S(B*_G) - S(A*_G)]} = |D|^|D|`, with the section 5.1 values. |
| G12 Landauer erasure | For every `G`, plan, and `w >= 1`, at cursor 0: (1) every written state has exactly one ERASE target system state and every blank state with `w - 1 < W` exactly `A` SCRIBBLE target system states; (2) `pi(B(G,p,w-1))/pi(R(G,p,w)) = b/A` from the propagated stationary law; (3) the family values of section 5.2. |
| G13 no free lunch | (1) The expected net packet current into the stockpile at equilibrium is exactly zero; (2) the expected net current from written to blank notebook states is exactly zero. |
| G14 Xypher slot reading | Every binding of section 3.4 is named; Ruby is empty; no channel hazard reads `N_tau`, `n_y`, `I`, `I*`, `K`, THAIM, or Xi. |

Classification. The independent tests are G01--G03, G04 checks 2--4, G05,
G07 checks 1--2, G08 check 5, G09, the channel structure of G10 check 1 and
G12 check 1, G14, and every count and value against section 5. The remaining checks (G04 check 1,
which holds by the state representation; G06; G07 check 3; G08 checks 1--4;
G10 checks 2--5; G11; G12 checks 2--3; G13) are consequences of G03, G05,
G07, and the declared representation, and are reported as consistency
checks. In particular the three traffic classes of G06
read the same `b` by construction, from one reservoir.

## 8. Preregistered falsifying controls

Each control mutates one declared input of the primary witness. The verifier
must report the named first failing gate and check. A pass of any control is a
verifier failure.

| Control | Mutation | Expected first failure |
|---|---|---|
| C01 one-way harvest | Delete every COMMIT channel, keeping HARVEST. | G03 check 1, first at configuration none |
| C02 free reset | ERASE connects `(G, p, 0, y, w, r)` to `(G, p, 0, blank, w, r)` for every `w`, and SCRIBBLE is its reverse: the notebook resets with no packet moved. | G02 check 5, first at configuration none |
| C03 blind measurement | MEASURE writes `0` regardless of `d(p)`. | G02 check 3, first at configuration `{0,1}` |
| C04 cheating harvest | HARVEST is allowed from any written record, matching or not. | G02 check 4, first at configuration none |
| C05 telepathic demon | HARVEST is also allowed from a blank notebook whenever the held plan ends at home: the feedback reads the future without a record. | G02 check 4, first at configuration none |
| C06 directed kinetic mutation | Double the forward hazard of the first HARVEST micro channel, ordered by source and then target in canonical complete-state order: `(none, 000, 0, 0, 0, 0) -> (none, 000, 0, 0, 1, 0)`. | G03 check 2, first at configuration none |
| C07 hoarding demon | HARVEST keeps the held plan (`p' = p` only) while storing a packet. | G02 check 4, first at configuration `{0,1}` |
| C08 unequal shuffle | The shuffled law uses a uniform record marginal on `V` instead of the marginal of `A_G`. | G11 check 2, first at configuration none |
| C09 stockpile slip | Construct the primary with `W = 3`: the state space, every channel, and every structural check use `W = 3`; only the section 5 predicted values remain those of `W = 2`. | G08 check 3 (global check: mean stockpile `11/15`, predicted `4/7`) |

C02, C05, and C07 remain reversible worlds with a temperature. They fail
because the gates check the declared semantics of reset, record, and release,
not only the existence of a temperature. C09 changes the world itself and
fails at the first prediction it moves.

## 9. Report format

One line per gate (`G01`--`G14`, `PASS`, `FAIL`, or `N/A` where section 7
allows) for the primary witness; one line per control with observed and
expected first failure; one witness line per primary configuration with
`N_2`, `n_y`, `|D|`, the generator-derived `K(G, y)`, `exp[N I]`, and
`exp[|D| I*]`; one line with the erasure ratio, mean stockpile, blank
probability, and the equilibrium fraction of correct written records; one
family line per case with configuration count, system and complete states,
`Z`, mean bridges, erasure ratio, mean stockpile, the configurations with
`K = 1`, and its gate summary; and `OVERALL PASS` or `OVERALL FAIL`.

## 10. What a pass establishes

Within the declared finite model:

1. A digital world in which a base Praxion holds a tau-step plan, a notebook
   records where that plan ends, a feedback move trades a correct record for a
   stored timber packet, and a reset of the notebook pays a packet into the
   reservoir, satisfies T1--T5 across the family, with `alpha` fixed by an
   independently declared reservoir. The same reservoir governs bridge,
   feedback, and erasure traffic by construction, which G06 confirms on the
   enumerated generator.
2. Adding the instrument leaves the CAL-CEF-1 configuration law intact, and
   the equilibrium world carries no plan-notebook information.
3. A correct record of the held plan's destination is worth `alpha ln 2` of
   free energy per bit of information about it: as the free-energy gap to a
   shuffled pairing of equal energy, under the Crystal's uniform plan law
   (`alpha I(G)`) and under the capacity witness (`alpha I*(G)`, the article's
   control value; the capacity pair is a prepared law that the dynamics does
   not produce, and its check is arithmetic), and as the equilibrium exchange constant
   `K(G, y) = N_tau(G)/[n_y(G) b]` of the feedback move, whose average worth is
   `alpha I(G)`. With a reservoir that doubles per packet, one bit of
   foresight trades evenly for one packet.
4. Erasing, through ERASE, a notebook that is uniform over its alphabet and
   uncorrelated with the plan costs `alpha ln A` of free energy; every net
   current vanishes at equilibrium.

A pass does not establish:

- net work extraction, or any engine that runs; the cycle is at equilibrium;
- information about realized trajectories; the record is of the held plan's
  destination;
- the article's capacity value from the feedback exchange where destinations
  are unequally reachable; there the exchange averages `I(G) < I*(G)`;
- T6 for the instrumented world;
- noisy or partial records, or a controller that acts on records alone;
- a Praxion whose hazards depend on the notebook beyond the declared
  permission rules, or any learning;
- any statement about THAIM, which is neither computed nor evaluated here.

## 11. Next boundary (outside this one)

Drive the cycle: couple the stockpile to a load and the notebook to a stream
of blank pages, and measure the steady-state rate at which committed futures
convert free-energy flow into stored work, with the efficiency bounded by the
second law. That is the first exact test of an agency bound in this program.
It must not reuse CAL-CEF-2 results to rescue its own gates.

## 12. Related work

- L. Szilard, "Über die Entropieverminderung in einem thermodynamischen
  System bei Eingriffen intelligenter Wesen," *Z. Phys.* 53, 840 (1929): one
  bit of information, `kT ln 2` of work.
- R. Landauer, "Irreversibility and Heat Generation in the Computing
  Process," *IBM J. Res. Dev.* 5, 183 (1961): the cost of erasure.
- C. H. Bennett, "The thermodynamics of computation---a review," *Int. J.
  Theor. Phys.* 21, 905 (1982): reversible measurement, and erasure as the
  irreducible cost.
- T. Sagawa and M. Ueda, *Phys. Rev. Lett.* 102, 250602 (2009) and 109,
  180602 (2012): the energy cost of measurement and erasure, and the
  thermodynamic role of correlations.
- D. Mandal and C. Jarzynski, "Work and information processing in a solvable
  model of Maxwell's demon," *PNAS* 109, 11641 (2012): an autonomous demon
  whose transitions read jointly the demon and the bit.
- J. M. R. Parrondo, J. M. Horowitz, and T. Sagawa, "Thermodynamics of
  information," *Nat. Phys.* 11, 131 (2015).
- S. Still, D. A. Sivak, A. J. Bell, and G. E. Crooks, "Thermodynamics of
  Prediction," *Phys. Rev. Lett.* 109, 120604 (2012).

Measurement, feedback, and erasure are standard information thermodynamics.
This boundary realizes them with the record of an actor's own committed
future, inside the world in which those futures are the microstates, at the
temperature of an independently counted reservoir.

## 13. Xypher review before execution

Target class: thermodynamic graph system (T1--T5) with a Xypher slot reading.
Thermodynamic claims are judged by T1--T5.

- Diamond: bridge configurations, plan, notebook; the Crystal resolution is
  REPLAN over valid plans.
- Ruby: empty. Opal/Phi: absent. Thompson memory, backward learning, teach,
  absorb, crystallize: not applicable.
- Memory: the held plan, cursor, and notebook are read by the dynamics; the
  notebook is a record, not learned memory, and its resets carry a named
  counter-reservoir.
- alpha: grounded by `g_R = b^(E_R)` before any rate.
- Declared parameters `N`, `tau`, `b`, `W`, `lambda = 1`, `kappa = 1`, home
  island `0`, and the notebook alphabet `V` are enumerated experimental
  inputs, not tuned to an outcome. The stockpile capacity `W` is a finite
  model input like `N`. No cap, threshold, or content-based branch acts on the
  dynamics beyond the declared permission rules.
- THAIM keeps its proved role as a one-sided receipt; no hazard reads it, Xi,
  `K`, `I`, or `I*`.

## 14. Review and freeze

Before execution, the exact commits of this boundary and of the apparatus are
reviewed independently for mathematical correspondence, adversarial checker
behaviour, and Futuruna/Xypher semantics, without running the evaluator. Only
non-executing checks (`runa check`, `cargo check`, `cargo test --no-run`,
`cargo fmt --check`, `cargo clippy`) are permitted before approval. The
approved commits are pushed, the frozen commands are executed once, and the
result is recorded in [[Control Information Thermodynamics Result]] with
provenance.
