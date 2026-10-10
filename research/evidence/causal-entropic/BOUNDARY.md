---
title: Causal Entropic Thermodynamics Boundary
aliases:
  - CAL-CEF-1 Boundary
  - Committed-Future Thermodynamic Graph Boundary
tags:
  - domain/physics
  - type/preregistration
  - topic/thermodynamics
  - topic/xypher
  - topic/causal-entropic-force
domain: Physics
type: preregistration
status: draft-no-evaluation
td: td-ab39db
created: 2026-10-07
updated: 2026-10-07
related:
  - "[[Xypher Operational Thermodynamics Boundary]]"
  - "[[Xypher Operational Thermodynamics Result]]"
---

# Causal entropic thermodynamics boundary (CAL-CEF-1)

## 0. The landing point

CAL-XTHERM established that a fixed finite digital graph can be an operational
thermodynamic system and that its minimal Xypher kernel reproduces its
dynamics exactly. Its Crystal read a one-step reshuffle over declared body
arrangements. A count of futures several steps ahead, the quantity behind the
causal entropic force, did not appear.

This boundary asks:

> When an actor holds one of its possible futures as executable state and
> acts only locally, does the count of futures it could hold become the
> thermodynamic entropy that drives its world?

Concretely: on a fixed archipelago, a traveller holds a plan, a tau-step walk
from home, and walks it forward and back. It replans only at home, uniformly
among the plans valid on the current bridges. It builds or dismantles only a
bridge at its own island, and never one its held plan still uses. Timber for
bridges comes from a reservoir of independently counted temperature. No rule
reads how many futures any configuration offers.

The questions are whether this world passes the independent operational
definition T1--T6, and whether its equilibrium law over bridge configurations
is

```text
pi(G) proportional to N_tau(G) * exp[-U(G)/alpha],
```

where `N_tau(G)` is the number of tau-step plans valid in `G`. If so, the
configuration free energy is `F(G) = U(G) - alpha ln N_tau(G) + const`, and
the drive toward configurations with more holdable futures is the entropic
force of the traveller's own committed futures, appearing without being
computed by any rule.

No verifier output, trajectory, or evaluated gate result has been generated
under this version. Design-time calculations used to state predictions are
disclosed in section 5.3. The document remains a draft until its exact commit
is independently reviewed and pushed.

## 1. The observable decision

Wissner-Gross and Freer define causal path entropy as the Shannon entropy of
the distribution of a system's own future paths under its own dynamics, with
temperature as a model parameter. The common Xypher Diamond readout is the
Shannon entropy of a random walker's tau-step endpoint distribution. That
distribution is generally nonuniform. Its entropy is then strictly less than
the logarithm of its own outcome count and is generally not the logarithm of
any integer, so it is not the log-count of that channel's equally weighted
executable outcomes. The CAL-XTHERM boundary (X1) restricts its theorem to
combinatorial Crystals for this reason.

This boundary uses a different observable:

> the number of tau-step plans the traveller could commit to from home,
> counting every route separately and allowing a stay at each step.

The plans are not labels beside the dynamics. The traveller's position is
read from its held plan, and which bridges it may build or dismantle depends
on that position and on which bridges the plan uses. A plan that is not a
valid walk on the current bridges can never be held.

The quantity is a uniform count of committed futures. It equals Wissner-Gross
and Freer's path entropy only for a deterministic executor of uniformly chosen
plans. This boundary does not claim that the entropy of the traveller's
realized, cursor-level trajectory equals `ln N_tau`.

## 2. Independent thermodynamic definition

The target class is the finite equilibrium thermodynamic graph system defined
by T1--T6 in [[Xypher Operational Thermodynamics Boundary]] (section 2),
unchanged. Those requirements mention no Xypher component.

The CAL-XTHERM representation theorem is not used. In this construction the
future count is not a micro-label multiplicity of a mesostate; it is a count
of system states grouped by configuration, and the configuration grouping is
not a Markov projection (section 4.3). T1--T6 are verified directly on the
system states, and the configuration law is derived from them.

## 3. The construction

### 3.1 Archipelago and configurations

Fix integers `N >= 3` (islands), `tau >= 1` (plan horizon), and `b >= 2`
(reservoir base). Islands are `V = {0, ..., N-1}`; island `h = 0` is home. The
candidate bridges are all `K = N(N-1)/2` unordered pairs, listed in
lexicographic order. A configuration `G` is any subset of candidate bridges.
Configurations are ordered by the integer whose bit `i` records whether
candidate bridge `i` is built. `A_G` is the adjacency matrix and
`L_G = A_G + I`.

### 3.2 Plans

A plan is a sequence `p = (v_0, ..., v_tau)` with `v_0 = h` and, for each
step, `v_(t+1) = v_t` or `{v_t, v_(t+1)}` in `G`. The set of plans valid in
`G` is `Pi_tau(G)` and

```text
N_tau(G) = |Pi_tau(G)| = e_h^T L_G^tau 1 >= 1.
```

Plans are ordered lexicographically as integer sequences. A plan uses bridge
`{u, w}` if some step moves between `u` and `w`.

### 3.3 System states and complete states

A system state is `z = (G, p, t)` with `p` in `Pi_tau(G)` and cursor
`t` in `{0, ..., tau}`. The traveller stands on island `x(z) = v_t`. Each
configuration has exactly `(tau + 1) N_tau(G)` system states.

Each bridge stores one timber packet of energy `lambda = 1`, so
`U(z) = lambda |G|`. A reservoir holds the remaining packets of a fixed total
`E_tot = K`. At reservoir energy `E_R = K - |G|` it has `b^(E_R)`
distinguishable arrangements, labelled `0, ..., b^(E_R) - 1`. This is the
independently declared equation of state:

```text
1/alpha = Delta S_R / Delta E_R = ln b,      alpha = lambda / ln b.
```

A complete state is `(z, r)` with `r` a reservoir label for `E_R(z)`.
Complete states are ordered by configuration, then plan, then cursor, then
reservoir label.

### 3.4 Channels

All micro channels have unit hazard `kappa = 1`; `kappa` fixes only the clock
unit.

- **STEP** (walk the plan): `(G, p, t, r) <-> (G, p, t+1, r)` for
  `0 <= t < tau`. The traveller moves from `v_t` to `v_(t+1)` along a bridge
  of `G` or stays. No energy changes.
- **REPLAN** (Crystal): `(G, p, 0, r) <-> (G, p', 0, r)` for every
  `p' != p` in `Pi_tau(G)`. Replanning happens only at home, at cursor 0.
  Equivalently, a uniform resolution over `Pi_tau(G)` at marked rate
  `N_tau(G)` with the no-change outcome cancelled, so each `p' != p` has
  hazard 1. No energy changes.
- **BUILD / DISMANTLE** (Action): at complete state `(G, p, t, r)` with the
  traveller on island `x = v_t`, for every candidate bridge `e` incident to
  `x`:
  - if `e` is not in `G`: connect to `(G + e, p, t, r')` for every reservoir
    label `r'` at energy `E_R - 1`;
  - if `e` is in `G` and `p` does not use `e`: connect to
    `(G - e, p, t, r')` for every reservoir label `r'` at energy `E_R + 1`.

  BUILD moves one packet from the reservoir to the bridge; DISMANTLE returns
  it. A bridge used by the held plan cannot be dismantled.

Every channel is listed together with its reverse: if `p` is valid in `G` and
does not use `e`, then `p` is valid in `G + e`, the traveller still stands on
an endpoint of `e`, and the reverse DISMANTLE is permitted.

### 3.5 Xypher slot reading

This object is a thermodynamic graph system read through Xypher slots. It is
not a CAL-XTHERM X1--X3 kernel: its Crystal entropy exists at configuration
resolution, while the Praxion acts on system states, and Xi is a
coarse-grained prediction rather than a hazard factor.

| Slot | Binding |
|---|---|
| Graph Substrate | bridge configurations on a fixed archipelago, plus the traveller's held plan and cursor |
| Crystal | REPLAN: uniform executable resolution over the plans valid in the current configuration; `S_tau(G) = ln N_tau(G)` |
| Thermo | reservoir law `g_R = b^(E_R)`, energy `U = lambda |G|`, THAIM pair, and Xi |
| Praxion | base actor: makes any permitted STEP, REPLAN, BUILD, or DISMANTLE at unit symmetric hazard |
| Action | BUILD or DISMANTLE one bridge at the traveller's island |
| Resolution | STEP arrival at the next or previous plan position; REPLAN writes the held plan |
| Thaw | the declared reverse of each channel |
| Memory | the held plan and cursor: written by REPLAN and STEP, read by STEP, BUILD, and DISMANTLE; no learning |
| Ruby | empty |
| Opal / Phi | absent |

The receipt and signed affinity are those of CAL-XTHERM X2, with THAIM the
current name of the frozen TAU receipt, evaluated between configurations:

```text
THAIM_+(G -> G') = alpha * max(0, ln N_tau(G') - ln N_tau(G)),
Xi(G -> G')      = THAIM_+(G -> G') - THAIM_+(G' -> G) - [U(G') - U(G)].
```

The Praxion's hazards do not use THAIM, Xi, or `N_tau`. Xi is the Harness's
prediction of the configuration-level odds that emerge (section 4.2).
All identities are checked multiplicatively, so no logarithm is evaluated:

```text
exp[Xi(G -> G') / alpha] = [N_tau(G') / N_tau(G)] * b^(-(|G'| - |G|)).
```

## 4. Derived consequences

These follow from the construction. The verifier checks each on the
enumerated object; it does not assume them.

### 4.1 System-state thermodynamics

Lumping reservoir labels gives system-state hazards: BUILD/DISMANTLE from `z`
to `z'` has hazard `b^(E_R(z'))`; STEP and REPLAN have hazard 1. Each system
state is one arrangement of the system's coordinates, so `S(z) = 0` up to a
common constant and local detailed balance reads

```text
ln[k(z -> z') / k(z' -> z)] = -[U(z') - U(z)] / alpha.
```

If the system-state graph is connected, the unique stationary law is
`pi(z) proportional to b^(-U(z))`, the complete-state law is uniform, and
every channel satisfies detailed balance.

### 4.2 The configuration law and the emergent force

Summing over the `(tau + 1) N_tau(G)` system states of each configuration,

```text
pi(G) = N_tau(G) b^(-|G|) / Z,      Z = sum_G N_tau(G) b^(-|G|).
```

For neighbouring configurations, let the equilibrium-averaged toggle rate be
`kbar(G -> G') = [sum over z in G, z' in G' of pi(z) k(z -> z')] / pi(G)`.
When the equilibrium toggle flux between `G` and `G'` is positive,
reversibility gives

```text
kbar(G -> G') / kbar(G' -> G) = pi(G') / pi(G) = exp[Xi(G -> G') / alpha].
```

Some neighbouring pairs have zero flux in both directions because no
traveller position in either configuration touches the toggled bridge; in
the primary witness this is exactly `none <-> {1,2}`. Those pairs are
excluded from the odds and reported; the configuration law still covers them
through connectivity.

This ratio is an identity of reversible dynamics given the configuration law,
so it adds no test independent of the stationary law. The local rules matter
through connectivity and through keeping every held plan valid, which fixes
the number of system states in each configuration at `(tau + 1) N_tau(G)`.
The content is that local, symmetric rules that never read `N_tau` produce,
at the configuration level, the odds of the causal entropic term
`alpha * Delta ln N_tau` against the timber cost at the reservoir's
temperature.

### 4.3 The configuration projection is not Markov

Toggle availability depends on the traveller's island and on the bridges its
plan uses. In the primary witness, at configuration `{0,1}`, a system state
with the traveller at home can build `{0,2}` but not `{1,2}`; a system state
with the traveller on island 1 can build `{1,2}` but not `{0,2}`. The grouping
by configuration is therefore not strongly lumpable. The causal entropic odds
of section 4.2 exist only as equilibrium averages, not as a configuration-level
rate rule. This is the sense in which the force is emergent.

### 4.4 The home-bridge law at tau = 2

For `tau = 2`,

```text
N_2(G) = sum over v in N[h] of (d_v + 1),
```

where `N[h]` is the closed neighbourhood of home and `d_v` the bridge degree.
Building one bridge `e` not in `G` changes the count by

```text
Delta N_2 = d_u + 3                       if e = {h, u},
Delta N_2 = [u ~ h] + [w ~ h]             if e = {u, w} with h not in e,
```

with `d_u` evaluated before building and `[u ~ h]` equal to 1 when `u`
already shares a bridge with home. Proof: for `e = {h, u}` the home term gains
1 and `u` enters `N[h]` with the term `(d_u + 1) + 1`; for a bridge away from
home `N[h]` is unchanged and each endpoint already in it gains one degree.

The emergent equilibrium odds of a home bridge are

```text
kbar(G -> G + {h,u}) / kbar(G + {h,u} -> G) = [N_2(G) + d_u + 3] / [b N_2(G)].
```

Bridging home to a well-connected island is favoured over bridging to an
isolated one, with no attachment rule supplied.

### 4.5 Contact

Two archipelagos `A` and `B` each run the dynamics above against their own
reservoirs. Both are prepared in their equilibria against reservoirs of the
same base `b`, then detached. In contact, a joint TRANSFER channel dismantles
a permitted bridge at `A`'s traveller island while building a bridge at `B`'s
traveller island, or the reverse direction, each at unit hazard. Each
archipelago's STEP and REPLAN continue. `M = |G_A| + |G_B|` is conserved.

On each shell `M`, the prepared law conditioned on `M` is proportional to
`b^(-|G_A|) b^(-|G_B|) = b^(-M)`, hence uniform over contact system states on
the shell. The contact dynamics has symmetric unit hazards, so the uniform
law is stationary and satisfies detailed balance on every channel. A shell
need not be connected: when neither archipelago can dismantle any bridge, as
for `(none, {1,2})` in the primary pair, that configuration pair is isolated.
On every connected component of a shell, the prepared law therefore equals
that component's unique stationary law, and the expected signed packet
current into `A` is exactly zero. If the reservoirs differ in base, the
prepared law fails detailed balance across some TRANSFER pair, for example
`(none, {0,1}) <-> ({0,1}, none)`.

## 5. Exact predictions

### 5.1 Primary witness: N = 3, tau = 2, b = 2

Candidate bridges in order: `{0,1}`, `{0,2}`, `{1,2}`. Home is island 0.
`alpha = 1/ln 2`. Each configuration has `3 N_2(G)` system states.

| Configuration | `|G|` | `N_2(G)` | System states | Complete states | `N_2 b^(-|G|)` |
|---|---:|---:|---:|---:|---:|
| none | 0 | 1 | 3 | 24 | 1 |
| {0,1} | 1 | 4 | 12 | 48 | 2 |
| {0,2} | 1 | 4 | 12 | 48 | 2 |
| {0,1},{0,2} | 2 | 7 | 21 | 42 | 7/4 |
| {1,2} | 1 | 1 | 3 | 12 | 1/2 |
| {0,1},{1,2} | 2 | 5 | 15 | 30 | 5/4 |
| {0,2},{1,2} | 2 | 5 | 15 | 30 | 5/4 |
| all three | 3 | 9 | 27 | 27 | 9/8 |

There are 108 system states and 261 complete states. The configuration law is
`N_2(G) b^(-|G|) / (87/8)`; the equilibrium mean bridge count is `131/87`.
For example, `kbar(none -> {0,1}) / kbar({0,1} -> none) = 2 = (1 + 0 + 3)/(2 * 1)`.

### 5.2 Confirmatory family

| `(N, tau, b)` | Configurations | System states | Complete states | `N_tau` range | `Z` | Equilibrium mean bridges |
|---|---:|---:|---:|---|---:|---:|
| (3, 1, 2) | 8 | 32 | 90 | 1..3 | 45/8 | 19/15 |
| (3, 2, 2) | 8 | 108 | 261 | 1..9 | 87/8 | 131/87 |
| (3, 3, 2) | 8 | 344 | 740 | 1..27 | 185/8 | 313/185 |
| (3, 2, 3) | 8 | 108 | 504 | 1..9 | 56/9 | 5/4 |
| (4, 1, 2) | 64 | 320 | 2,916 | 1..4 | 729/32 | 7/3 |
| (4, 2, 2) | 64 | 1,344 | 10,206 | 1..16 | 1701/32 | 55/21 |
| (4, 2, 3) | 64 | 1,344 | 44,544 | 1..16 | 14848/729 | 123/58 |
| (4, 3, 2) | 64 | 5,248 | 34,344 | 1..64 | 4293/32 | 151/53 |

Here `Z = sum_G N_tau(G) b^(-|G|)`, equal to the complete-state count divided
by `(tau + 1) b^K`.

For `N = 3` cases, complete states with reservoir labels and every micro
channel are enumerated explicitly, so G03 and G05 test the reservoir lift
directly. For `N = 4` cases, system states and every system-state channel are
enumerated explicitly with lumped reservoir hazards `b^(E_R)`. The reservoir
lift is not re-enumerated there; G05 is then not applicable and G03 is checked
at system-state level.

### 5.3 Design-time disclosure

Before this draft, a design script computed `N_tau(G)` for the family, the
configuration-law means in section 5.2, the system- and complete-state counts,
the lazy random-walk endpoint laws used in C02, and the home-bridge law of
section 4.4 on all 251,084 (configuration, unbuilt bridge) cases for
`N = 3..6`. G12 is therefore a confirmatory reproduction of a pre-checked
identity, not a blind prediction. No gate of the dynamics (reciprocity,
lumping, connectivity, stationarity, contact, emergent odds, controls) was
evaluated.

## 6. Apparatus

A standalone dependency-free Rust crate at
`research/physics/xypher-causal-entropic-proof/`, with a prospective Futuruna
construction `xypher.runa`, mirroring the CAL-XTHERM apparatus. All arithmetic
is exact integer or reduced rational arithmetic. No floating-point acceptance
path, random seed, sampling, simulation length, or tolerance is permitted.

The Futuruna file declares the archipelago, plan validity, the REPLAN channel,
the local BUILD/DISMANTLE permission rule, the reservoir law, the THAIM pair,
and Xi as the predicted configuration odds. It does not consume stationary
weights, averaged rates, or rate ratios.

The Rust verifier enumerates plans by depth-first search, computes
`e_h^T L_G^tau 1` by integer matrix powers, enumerates system and complete
states, constructs every channel from the local rules of section 3.4, and
evaluates the gates. The stationary law is obtained from the constructed
generator (connectivity plus detailed-balance verification), not assumed.

## 7. Acceptance gates

Within each gate, checks run in the listed order, and each check runs over all
configurations (and states) in canonical order before the next check begins.

| Gate | Checks in order |
|---|---|
| G01 Crystal grounding | (1) `N_tau(G) >= 1`; (2) depth-first plan count equals `e_h^T L_G^tau 1`; (3) the REPLAN channel at each cursor-0 system state is uniform over its positive-probability outcomes; (4) its outcome alphabet equals `Pi_tau(G)`; (5) each configuration has `(tau + 1) N_tau(G)` system states. |
| G02 executable plans | (1) every enumerated plan is valid in its configuration, and every channel's destination holds a plan valid in the destination configuration; (2) every STEP moves along a bridge of the current configuration or stays; (3) every BUILD/DISMANTLE is incident to the traveller's island; (4) no DISMANTLE removes a bridge used by the held plan; (5) plan contents are read: in the primary witness, two system states of one configuration with equal cursor have different BUILD/DISMANTLE sets. |
| G03 reciprocal generator | (1) every positive channel has a positive reverse; (2) equal unit hazards in both directions at micro level (`N = 3`) or `b^(E_R)` lumped hazards (`N = 4`); (3) exact row closure. |
| G04 pathwise accounting | Every BUILD/DISMANTLE changes `U` by `+-lambda` and reservoir energy by `-+lambda`; heat into the system equals `Delta U`; external work is zero; STEP and REPLAN change no energy; the plan-register writes of STEP and REPLAN are reversible channels within the complete state and need no counter-reservoir. |
| G05 reservoir lumpability | (`N = 3`) every complete state in a system state sends the same total rate into every other system state; lumped hazards are `b^(E_R(z'))` for toggles and 1 otherwise. N/A for `N = 4`. |
| G06 system-state local detailed balance | For every system-state channel, `k(z -> z')/k(z' -> z) = b^(-(U(z') - U(z)))`, with `alpha` from the reservoir law and `S(z)` constant. |
| G07 connectivity and equilibrium | (1) the system-state graph is connected; (2) `pi(z) proportional to b^(-U(z))` is stationary; (3) every channel's flux products are equal. |
| G08 configuration law | The configuration marginal equals `N_tau(G) b^(-|G|)/Z`; `Z` and the equilibrium mean bridge count match section 5. |
| G09 emergent causal entropic odds | (1) `Xi(G' -> G) = -Xi(G -> G')`; (2) for every pair of neighbouring configurations with positive equilibrium toggle flux, `kbar(G -> G')/kbar(G' -> G) = exp[Xi(G -> G')/alpha]`, checked cross-multiplied, with zero-flux pairs reported (primary witness: exactly `none <-> {1,2}`); (3) the configuration projection is not strongly lumpable, with the witness pair of section 4.3 reported. |
| G10 Xypher slot reading | Every binding of the section 3.5 table is named; Ruby is empty; no channel hazard reads `N_tau`, THAIM, or Xi. |
| G11 same-temperature contact | Identical pair `(3,2,2)` with `(3,2,2)` and heterogeneous pair `(3,2,2)` with `(3,1,2)`, every shell `M`: (1) the prepared law conditioned on `M` is stationary under the contact generator and satisfies detailed balance on every TRANSFER, STEP, and REPLAN channel; (2) on each connected component of each shell it equals that component's uniform stationary law, with the components reported; (3) the expected signed packet current into `A` is exactly zero. |
| G12 home-bridge law | For `tau = 2` cases, the closed forms of section 4.4 hold for every configuration and unbuilt bridge, and every home-bridge pair has emergent odds `[N_2 + d_u + 3]/[b N_2]`. N/A when `tau != 2`. |

## 8. Preregistered falsifying controls

Each control mutates one declared input of the primary witness. The verifier
must report the named first failing gate and check. A pass of any control is a
verifier failure.

| Control | Mutation | Expected first failure |
|---|---|---|
| C01 one-way builder | Delete every DISMANTLE channel, keeping BUILD. | G03 check 1 (reciprocal support) |
| C02 destination readout | REPLAN draws from the tau-step lazy random-walk endpoint law from home: outcome alphabet = islands with positive probability (the support of the home row), probabilities = home row of `(D^-1 L_G)^tau` with `D = diag(L_G 1) = diag(d_v + 1)`, reported multiplicity = support size. | G01 check 3 (uniform channel), first at configuration `{0,1},{0,2}` |
| C03 directed kinetic mutation | Double the forward hazard of the first STEP channel in canonical complete-state order only. | G03 check 2 (equal reverse hazards) |
| C04 unpaired receipt | Use `Xi = THAIM_+(G -> G') - Delta U`, dropping the reverse receipt. | G09 check 1 (Xi antisymmetry) |
| C05 no staying | Plans may not stay put (use `A_G` instead of `L_G`). | G01 check 1, at configuration none |
| C06 horizon slip | REPLAN draws uniformly over plans of length `tau - 1`. | G01 check 4 (alphabet, compared as sets of sequences), first at configuration none |
| C07 plan breaker | DISMANTLE ignores whether the held plan uses the bridge. | G02 check 1 (plan validity) |
| C08 unequal reservoirs | Prepare archipelago `B` of the identical pair against `b = 3`. | G11 check 1 (prepared law stationary with detailed balance) |

C02 shows that a destination readout is not an equiprobable executable
channel over the futures the traveller can commit to. It does not prove that
no construction could make endpoint entropy a state count.

## 9. Report format

One line per gate (`G01`--`G12`, `PASS`, `FAIL`, or `N/A` where section 7
allows) for the primary witness; one line per control with observed and
expected first failure; one witness line with per-configuration `N_2`, system
states, complete states, configuration weights, and contact current; one
family line per case with configuration count, system and complete states,
`Z`, mean bridges, and its gate summary; and `OVERALL PASS` or
`OVERALL FAIL`.

## 10. What a pass establishes

Within the declared finite model:

1. A digital world in which a base Praxion holds one tau-step plan as
   executable state, walks it, replans uniformly among valid plans at home,
   and builds or dismantles bridges only at its own island satisfies T1--T5
   across the exact eight-case family (reservoir lumpability verified on the
   explicit lift for `N = 3`), and T6 for the identical `(3,2,2)` pair and the
   heterogeneous `(3,2,2)`/`(3,1,2)` pair, with `alpha` fixed by an
   independently declared reservoir.
2. Its equilibrium configuration law is `N_tau(G) exp[-U(G)/alpha]/Z`: the
   log-count of committable tau-step futures is the configuration entropy.
3. Although no channel reads `N_tau`, the equilibrium-averaged
   build/dismantle odds between configurations with positive toggle flux
   equal `exp[Xi/alpha]` with `Xi = alpha Delta ln N_tau - Delta U`. This is an
   identity given claim 2; its content is that local rules which never read
   the future count produce the causal entropic odds at the operational
   temperature.
4. At `tau = 2`, the emergent preference for bridging home to
   well-connected islands follows exactly from that entropy.

A pass does **not** establish:

- that the Shannon entropy of the traveller's realized trajectories, or of a
  random walker's endpoints, equals `ln N_tau` (C02 tests a destination
  readout);
- any effect on agents that do not hold their futures as state;
- a configuration-level rate rule; the force exists as an equilibrium average;
- growth with new islands, sustained driven growth, or any nonequilibrium
  steady state; the island set is fixed;
- learning, control capacity, intelligence, or the correlation free energy of
  control;
- grammar endogeneity, artificial life, or consciousness;
- new fundamental physics. The result is equilibrium statistical mechanics of
  a declared model. Its counting is the same mechanism as the entropic
  elasticity of a polymer, whose conformations are walks: here the walks are
  committed futures.

## 11. Next boundary (outside this one)

The held plan is a command, and its final position is a destination. A later
boundary will give the destination its own register, measure the control
capacity of plan over destination inside this energy account, and test the
correlation free energy `alpha ln 2` per bit against a shuffled pairing with
equal marginals and equal average energy. It must not reuse CAL-CEF-1 results
to rescue its own gates.

## 12. Related work

- A. D. Wissner-Gross and C. E. Freer, "Causal Entropic Forces," *Phys. Rev.
  Lett.* 110, 168702 (2013): Shannon entropy of a system's own future paths,
  with a model temperature.
- Z. Burda, J. Duda, J. M. Luck, and B. Waclaw, "Localization of the Maximal
  Entropy Random Walk," *Phys. Rev. Lett.* 102, 160602 (2009): uniform
  weighting of paths on a fixed graph.
- S. Pressé, K. Ghosh, J. Lee, and K. A. Dill, "Principles of maximum entropy
  and maximum caliber in statistical physics," *Rev. Mod. Phys.* 85, 1115
  (2013): path-entropy ensembles.
- A. S. Klyubin, D. Polani, and C. L. Nehaniv, "Empowerment: A Universal
  Agent-Centric Measure of Control," IEEE CEC 2005, 128--135: in
  deterministic settings, the log of the number of distinct reachable states.
- Exponential random graph models: laws over graphs proportional to
  `exp(theta . statistics)` with fitted `theta`. The configuration law here
  has the statistic `ln N_tau` with coefficient one from state counting, and
  its temperature from an independently counted reservoir.
- J. Schnakenberg, "Network theory of microscopic and macroscopic behavior of
  master equation systems," *Rev. Mod. Phys.* 48, 571 (1976): local detailed
  balance on graph edges.
- Entropic elasticity of polymers: forces from counting chain conformations.

A literature search at design time found no construction in which a uniform
count of an actor's committable futures on a fixed vertex set is the entropy
of an exact equilibrium bridge ensemble at an independently grounded
temperature, with the corresponding causal entropic odds emerging from local
rules. That is a statement about the search, not a novelty proof.

## 13. Xypher review before execution

Target class: thermodynamic graph system (T1--T6) with a Xypher slot reading.
It is not a CAL-XTHERM X1--X3 kernel (section 3.5). Thermodynamic claims are
judged by T1--T6.

- Diamond: bridge configurations; the Crystal resolution is REPLAN over valid
  plans.
- Ruby: empty. Opal/Phi: absent. Thompson memory, backward learning, teach,
  absorb, crystallize: not applicable.
- Memory: the held plan and cursor are read by the dynamics; they are
  committed state, not learned memory.
- alpha: grounded by `g_R = b^(E_R)` before any rate; contact checks equal
  bases and C08 checks unequal ones.
- Declared parameters `N`, `tau`, `b`, `lambda = 1`, `kappa = 1`, and home
  island `0` are enumerated experimental inputs, not tuned to an outcome. No
  cap, threshold, or content-based branch acts on the dynamics beyond the
  declared validity rules.
- THAIM keeps its proved role as a one-sided receipt; Xi is the signed
  prediction of emergent odds; no hazard reads either.

## 14. Review and freeze

Before execution, the exact commits of this boundary and of the apparatus are
reviewed independently for mathematical correspondence, adversarial checker
behaviour, and Futuruna/Xypher semantics, without running the evaluator. Only
non-executing checks (`runa check`, `cargo check`, `cargo test --no-run`) are
permitted before approval. The approved commits are pushed, the frozen
commands are executed once, and the result is recorded in
[[Causal Entropic Thermodynamics Result]] with provenance.
