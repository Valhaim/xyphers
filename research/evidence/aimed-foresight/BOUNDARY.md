---
title: Aimed Foresight Boundary
aliases:
  - CAL-CEF-4 Boundary
  - Does Seeing Further Ahead Buy More Open Future
tags:
  - domain/physics
  - type/preregistration
  - topic/thermodynamics
  - topic/xypher
  - topic/nonequilibrium
domain: Physics
type: preregistration
status: draft-no-evaluation
td: td-47e999
created: 2026-10-10
updated: 2026-10-10
related:
  - "[[Open Future Price Boundary]]"
  - "[[Open Future Price Result]]"
  - "[[Xypher Operational Thermodynamics Boundary]]"
---

# Aimed foresight boundary (CAL-CEF-4)

## 0. The question

> Does seeing further ahead let an agent buy more open future per unit of
> energy?

CAL-CEF-3 measured the open future an agent holds per unit of fuel flow in a
driven archipelago. Fuel spent where the traveller stood beat fuel spread over
every bridge in all twelve worlds. Its two foresight hypotheses were refuted:
the yield did not reliably rise with the planning horizon, and the edge of
targeted fuel shrank as the horizon grew. Its result record suggested two
features of the design that may have kept foresight from mattering. First, the
held plan protected its bridges from the weather for free, which helped the
spreading agent as much as the targeted one. Second, the fuel never read the
plan.

This boundary removes both. The held route is an intention, not a shield. No
bridge is protected for free: the weather may wear any bridge, and a worn
bridge blocks the traveller's next step until the weather or the fuel rebuilds
it. Agents that read their route spend fuel on the route's bridges within `k`
steps of where they stand, ahead or behind: every bridge they can cross in
their next `k` moves along the route. The depth `k` is the agent's foresight,
varied while the world, the routes, and the measure stay fixed.

Yield per packet depends on how much fuel flows; for the rival that warms every
bridge it rises with the flow, so raw yields of agents with different flows are
not comparable. Every agent is therefore compared with the world of
independent bridges at the same fuel flow, which the warming rival realises
whenever it can (`J < J_max`). Since every agent satisfies
`J = (b_c + 1)<|G|>_p - K` (section 2), the rival is the world of independent
bridges with the agent's own mean number of bridges, and the comparison
measures where the agent puts its bridges, not how many it holds.

The hypotheses of section 7 can be refuted, and any refutation is reported as
a result. No steady state of an aiming agent was computed by the designer, at
any temperature, before the freeze.

## 1. The world

### 1.1 Archipelago and routes

Islands `V = {0, ..., N-1}`, home `0`, candidate bridges the `K = N(N-1)/2`
unordered pairs in lexicographic order; bridge `i` is bit `i` of a
configuration `G`, and configurations are ordered by bit pattern. `A_G` is the
adjacency matrix of `G`.

A **route** is `r = (v_1, ..., v_tau) in V^tau`, read as the walk
`v_0 = 0, v_1, ..., v_tau`. Each step either stays (`v_{s+1} = v_s`) or moves
to another island; a moving step uses the bridge `{v_s, v_{s+1}}`. Routes are
ordered lexicographically; there are `N^tau` of them, and none needs its bridges
to exist.

A state is `z = (G, r, t)` with cursor `t in {0, ..., tau}`; the traveller
stands on `x(z) = v_t`. States are ordered by configuration, then route, then
cursor. Every combination is a state: `2^K N^tau (tau + 1)` states. System
energy `U(z) = lambda |G|`, `lambda = 1`.

### 1.2 Two stores

As CAL-CEF-3 section 1.2: a cold store with ratio `b_c` and a hot store with
ratio `b_h`, `b_c > b_h >= 2`, each declared by its equation of state, with
temperatures `alpha_c = lambda / ln b_c < alpha_h = lambda / ln b_h`. A packet
into a bridge from store `i` and a packet back into store `i` have rates in the
ratio `1 : b_i`.

### 1.3 Channels

All rates are exact integers.

- **STEP**: `(G, r, t) <-> (G, r, t+1)` for `0 <= t < tau`, rate 1 each way,
  present exactly when step `t` stays or its bridge is in `G`.
- **REPLAN**: at `t = 0`, `(G, r, 0) <-> (G, r', 0)` for every route
  `r' != r`, rate 1 each way.
- **WEATHER** (cold store), unconditionally, at every state and for every
  candidate bridge: build at rate 1 when absent (one packet from the cold
  store), wear at rate `b_c` when present (one packet to the cold store).
- **FUEL** (hot store): for every bridge in the agent's permission set
  `A(r, t)` (section 1.4), build at rate 1 when absent (one packet from the hot
  store), return at rate `b_h` when present (one packet to the hot store).

`A(r, t)` depends only on the route and the cursor, which no bridge change
alters, so every FUEL build has its return. No rate reads any count of
futures.

CAL-CEF-3 section 7 proposed that a worn bridge under the plan force a replan.
A forced replan has no local reverse. Here a worn bridge blocks the step
instead, which keeps every channel local and reciprocal.

### 1.4 Agents

Each driven world is run with each of these agents. Agents differ only in
`A(r, t)`.

- **LOCAL**: the bridges incident to `x(z)`.
- **AIM-k**, for `1 <= k <= tau`: the bridges used by the moving steps among
  steps `max(0, t - k), ..., min(tau, t + k) - 1` of the route. These are the
  bridges the traveller can cross in its next `k` moves along its route, ahead
  or behind; the cursor moves both ways, and the traveller walks back along
  its route to replan. AIM-1 is the route's bridges at the traveller's island,
  the step behind and the step ahead (none, one, or two bridges), and
  AIM-`tau` is the whole route at every cursor.
- **GLOBAL**: every candidate bridge.

AIM-`k` for `k = 1, ..., tau` is the foresight ladder: the world, routes, and
measure are fixed, and only how far ahead the fuel is aimed changes.

### 1.5 The undriven world, the driven world, and symmetry

The **undriven world** has STEP, REPLAN, and WEATHER. STEP and REPLAN have
equal rates both ways, and every WEATHER toggle has the cold store's ratio, so
its stationary law is `pi_c(z) proportional to b_c^(-|G|)`, with detailed
balance on every channel. Bridges are independent, each built with probability
`r_0 = 1/(1 + b_c)`, and route and cursor are uniform and independent of `G`.
Without protection there is no committed-futures bias: the undriven world
owes the traveller nothing.

The **driven world** adds FUEL. Its steady state `p` is the unique solution of
`p Q = 0`, `sum p = 1`.

The GLOBAL agent's steady state is `p(z) proportional to q^|G|` with
`q = 2/(b_c + b_h)`, uniform over route and cursor. The **warming family** has
fuel at every bridge with rates `s` and `s b_h` for real `s > 0`; its law is
`q_s^|G|` with `q_s = (1 + s)/(b_c + s b_h)`, each bridge built with
probability `r_s = (1 + s)/(b_c + 1 + s (b_h + 1))`. GLOBAL is `s = 1`.

Every channel rule commutes with relabelling the islands `1, ..., N-1`, with
home fixed. The steady state is therefore constant on the orbits of that group,
and it is solved on the orbits and lifted to every state (section 6).

## 2. What is measured

The **open future of the traveller** is `N_2(G, x) = e_x^T (I + A_G)^2 1`, the
number of two-step walks, each step a stay or a crossing, from the island where
it stands. Unlike CAL-CEF-3, which counted from home, it is counted from the
traveller's own island: its own future freedom of action. For each agent:

```text
H = <N_2(G, x)>_p - <N_2(G, x)>_(pi_c)    open future held above the undriven world,
J = net packets per unit time from the hot store into the bridges (fuel flow),
Y = H / J                                 open future per unit of fuel flow.
```

Under any law with independent bridges, each present with the same
probability `r`, and route and cursor independent of `G`,
`<N_2(G, x)> = Phi_N(r)` with

```text
Phi_N(r) = 1 + (N - 1) r + (N - 1) r (2 + (N - 2) r),
```

so the undriven value is `Phi_N(r_0)` and the warming family's is
`Phi_N(r_s)`.

**The warming rival at equal power.** Because the weather acts on every
bridge unconditionally, the cold store's net flow and energy balance give, for
every agent in steady state,

```text
J = (b_c + 1) <|G|>_p - K.
```

Fuel flow fixes the mean number of bridges, and the mean number of bridges
fixes the fuel flow. The rival with the agent's flow is the world of
independent bridges at the agent's own density `rho = <|G|>_p / K`:

```text
H_warm = Phi_N(rho) - Phi_N(r_0),        E = H / H_warm = Y / Y_warm,
```

where `Y_warm = H_warm / J` is the yield of that rival. The agent's **edge at
equal power** `E` is defined for every agent with `J > 0`, since then
`rho > r_0`. `E > 1` means the agent's placement of bridges holds more open
future for the traveller than the same mean number of bridges placed
independently, for the same fuel flow. `E = 1` for GLOBAL.

The warming family realises that rival with fuel when it can. Its flow is

```text
J_warm(s) = K s (b_c - b_h) / (b_c + 1 + s (b_h + 1)),
```

strictly increasing in `s` toward `J_max = K (b_c - b_h)/(b_h + 1)`. For
`0 < J < J_max` the unique `s` with `J_warm(s) = J` is

```text
s*(J) = J (b_c + 1) / [K (b_c - b_h) - J (b_h + 1)],
```

and then `r_(s*) = rho` exactly. `J < J_max` is equivalent to
`rho < 1/(b_h + 1)`. It is a theorem when `b_c >= 2 b_h`: each bridge's hot
flow is strictly below `b_c/(b_c + 2)` for every agent (the law is strictly
positive, and some state holds the bridge present and fuelled or absent and
unfuelled), and `b_c/(b_c + 2) <= (b_c - b_h)/(b_h + 1)` exactly when
`b_c >= 2 b_h`. So it holds at `(8, 2)` and `(6, 3)`. At `(3, 2)` it is not
guaranteed. An agent with
`J >= J_max` has no warming realisation at this hot temperature. Its `s*` is
reported as `none` and its `E` is still defined. `H`, `J`, `Y`, `rho`, `s*`,
and `E` are exact rationals.

**Price and floor**, reported for every agent with no hypothesis attached, as
CAL-CEF-3 section 2: `sigma = J ln(b_c/b_h)`; `sigma_floor`, the entropy
production of the undriven channels at the held state,
`= - d/dt D(p || pi_c)` under the undriven dynamics; `eta = sigma_floor/sigma`;
and `Y_bound = H ln(b_c/b_h)/sigma_floor`. Parallel WEATHER and FUEL channels
between the same two states are separate channels and are never lumped.
`sigma`, `sigma_floor`, and gate G08 are evaluated on the full lifted chain,
channel by channel, never on the orbits.

## 3. Cases

Three temperature pairs `(b_c, b_h) = (3, 2)`, `(8, 2)`, `(6, 3)`; for each,
six worlds:

| World `(N, tau)` | Agents |
|---|---|
| (3, 1) | LOCAL, AIM-1, GLOBAL |
| (3, 2) | LOCAL, AIM-1, AIM-2, GLOBAL |
| (3, 3) | LOCAL, AIM-1, AIM-2, AIM-3, GLOBAL |
| (4, 1) | LOCAL, AIM-1, GLOBAL |
| (4, 2) | LOCAL, AIM-1, AIM-2, GLOBAL |
| (5, 1) | LOCAL, AIM-1, GLOBAL |

That is 18 worlds and 66 steady states. Worlds run pair by pair in the order
`(3, 2)`, `(8, 2)`, `(6, 3)`, and within a pair in the row order above; within a
world, agents run in the order listed. The primary witness for controls is
`(N, tau, b_c, b_h) = (3, 2, 8, 2)` with all four of its agents.

## 4. Exact predictions fixed in advance

These are closed forms and structural counts; none involves the steady state
of a LOCAL or AIM agent.

### 4.1 State counts, orbits, and the undriven world

| `(N, tau)` | States | Orbits | `<N_2>` under `pi_c` at `b_c = 3` | at `b_c = 8` | at `b_c = 6` |
|---|---:|---:|---:|---:|---:|
| (3, 1) | 48 | 28 | 21/8 | 137/81 | 93/49 |
| (3, 2) | 216 | 114 | 21/8 | 137/81 | 93/49 |
| (3, 3) | 864 | 440 | 21/8 | 137/81 | 93/49 |
| (4, 1) | 512 | 120 | 29/8 | 56/27 | 118/49 |
| (4, 2) | 3072 | 612 | 29/8 | 56/27 | 118/49 |
| (5, 1) | 10240 | 660 | 19/4 | 67/27 | 145/49 |

Orbits are those of the relabellings of islands `1, ..., N-1` acting on
states.

### 4.2 The GLOBAL agent

`H_global = Phi_N(2/(b_c + b_h + 2)) - Phi_N(1/(1 + b_c))` and
`J_global = K (b_c - b_h)/(b_c + b_h + 2)`, independent of `tau`:

| `(b_c, b_h)` | `N` | `H_global` | `J_global` | `Y_global` | `J_max` |
|---|---:|---:|---:|---:|---:|
| (3, 2) | 3 | 99/392 | 3/7 | 0.5893 | 1 |
| (3, 2) | 4 | 171/392 | 6/7 | 0.5089 | 2 |
| (3, 2) | 5 | 129/196 | 10/7 | 0.4607 | 10/3 |
| (8, 2) | 3 | 59/162 | 3/2 | 0.2428 | 6 |
| (8, 2) | 4 | 16/27 | 3 | 0.1975 | 12 |
| (8, 2) | 5 | 23/27 | 5 | 0.1704 | 20 |
| (6, 3) | 3 | 1536/5929 | 9/11 | 0.3166 | 9/4 |
| (6, 3) | 4 | 2529/5929 | 18/11 | 0.2607 | 9/2 |
| (6, 3) | 5 | 3672/5929 | 30/11 | 0.2271 | 15/2 |

`Y_global = H_global / J_global` exactly; decimals are for reading. As in
CAL-CEF-3, every GLOBAL toggle carries equal and opposite WEATHER and FUEL
currents, so

```text
eta_global = ln[2 b_c / (b_c + b_h)] / ln(b_c / b_h),
```

`0.4497` at `(3, 2)`, `0.3390` at `(8, 2)`, `0.4150` at `(6, 3)`. These
values calibrate the solver, the warming rival, and the logarithms; they are
not hypotheses.

## 5. Gates

Gates run G01 to G11. Checks within a gate run in order over all worlds and
agents (section 3 order) and over states, channels, or configurations in
canonical order, before the next check starts. The first failure is the first
failing check of the lowest-numbered failing gate, at the first world, agent,
and configuration in that order. Configuration "none" is the empty
configuration (bit pattern 0), and `{{0,1}}` is the configuration whose only
bridge is `{0,1}` (bit pattern 1). The gates test the apparatus and the
construction; the hypotheses of section 7 are evaluated afterwards and reported
whether they hold or not.

| Gate | Checks in order |
|---|---|
| G01 grounding | (1) the route set is `V^tau` in lexicographic order; (2) for every configuration and island, `N_2(G, x)` by depth-first walk enumeration equals `e_x^T (I + A_G)^2 1` computed by integer matrix products; (3) every world has the state and orbit counts of section 4.1. |
| G02 executable rules | (1) every channel's destination is a state of the world; (2) STEP is present exactly when section 1.3 permits it, in both directions, and REPLAN exactly at `t = 0` to every other route; (3) at every state, every candidate bridge has its WEATHER build when absent and its WEATHER wear when present, and no other WEATHER channel exists; (4) every FUEL channel toggles a bridge in the agent's declared `A(r, t)`, and every absent bridge in `A(r, t)` has its FUEL build (the presence of FUEL returns is tested by G03 check 1); (5) only STEP changes the cursor, only REPLAN changes the route, and only WEATHER and FUEL change the configuration. |
| G03 reciprocity | (1) every channel has a reverse in the same class, on the same bridge, with the same store; (2) at the source state, STEP and REPLAN are `1 : 1`, WEATHER build : wear `1 : b_c`, FUEL build : return `1 : b_h`. |
| G04 accounting | (1) every bridge change moves exactly one packet between the bridges and its named store, and STEP and REPLAN move none; (2) in every driven steady state, the net packet flow from the hot store equals the net flow into the cold store, exactly; (3) `J = (b_c + 1)<|G|>_p - K` exactly. |
| G05 undriven world | (1) the undriven state graph is strongly connected; (2) `b_c^(-|G|)` satisfies detailed balance on every undriven channel. |
| G06 driven steady state | (1) the driven state graph is strongly connected; (2) for every relabelling of islands `1, ..., N-1` and every channel, the image is a channel of the same class, store, and rate, on the image bridge; (3) the exact solution on the orbits is strictly positive; (4) the law lifted to every state balances every state of the full chain exactly, inflow against outflow, in integers. |
| G07 GLOBAL calibration | (1) every GLOBAL steady state equals `q^|G|` normalised, with `q = 2/(b_c + b_h)` from the world's own stores; (2) `H_global` and `J_global` equal their section 4.2 closed forms evaluated at the world's own stores. |
| G08 second law and the floor | (1) `J > 0` in every driven steady state; (2) `sum over channel pairs of [p(z) k - p(z') k'] ln(k/k') = J ln(b_c/b_h)`, as exact rational coefficients of two formal per-store symbols `L_c`, `L_h`; (3) every channel pair contributes a nonnegative term to `sigma_floor` or to `sigma - sigma_floor`; (4) certified enclosures of `sigma_floor` and `eta` have width at most `10^-9`, and the enclosure of `eta` lies strictly inside `(0, 1)`; (5) for GLOBAL, the certified enclosure of `ln[2 b_c/(b_c + b_h)]/ln(b_c/b_h)`, of width at most `10^-9`, intersects the enclosure of `eta`. |
| G09 warming rival | (1) for every agent with `J < J_max`, `s*(J) > 0`, `J_warm(s*(J)) = J` exactly, and `r_(s*) = rho` exactly; (2) for GLOBAL, `rho = 2/(b_c + b_h + 2)`, `s* = 1`, and `E = 1` exactly. |
| G10 Xypher slot reading | (1) slots: Graph Substrate (configuration, route, cursor), Crystal (REPLAN over all routes), Thermo (two declared stores, `U = lambda |G|`), Praxion (base actor with FUEL moves), environment (WEATHER); (2) Ruby empty, Opal absent; (3) every rate is computed from the state, the bridge, the channel class, the store ratio, and the agent's `A(r, t)` alone, and equals the section 1.3 rule; no rate reads `N_2`, `H`, `J`, `Y`, `E`, or `eta`. |
| G11 frozen values | (1) `<N_2>` under `pi_c` matches section 4.1 for every world; (2) `H_global` and `J_global` match section 4.2 exactly, and the four-decimal rounding of `H_global / J_global` matches `Y_global`; (3) the four-decimal rounding of `eta_global` matches section 4.2. |

Classification. Independent tests: G01, G02, G03, G04 check 1, G05, G06 checks
2--4 (check 4 verifies the solver and the lifting on the full chain), G07 check
1, G08 check 4 (widths, and `eta < 1`, which is not a theorem for LOCAL and
AIM), G08 check 5 (logarithms), G10 check 3, and G11. Implementation checks of
identities: G09 check 1 (`s*` against `J_warm`, and the rival's density against
the agent's, given G04 check 3). Consistency checks or theorems given earlier
gates: G04 checks 2--3, G06 check 1 (implied by G05 check 1), G07 check 2, G08
checks 1--3 and `eta > 0`, G09 check 2 (given G07 check 2), and G10 checks
1--2 (declarative).

## 6. Apparatus

A standalone dependency-free Rust crate at
`research/physics/xypher-aimed-foresight-proof/`, with a prospective Futuruna
declaration `xypher.runa`. The exact arithmetic modules of the CAL-CEF-3
apparatus are reused unchanged: `src/big.rs`, `src/rat.rs`, `src/logs.rs`,
`src/solve.rs` at the digests recorded in the Open Future Price Result (section
2). New code builds the worlds and agents, the island-relabelling orbits, the
lumped generator, the lifting, the warming rival, the gates, the hypotheses,
and the report.

The steady state is solved by fraction-free elimination on the orbit-lumped
generator, whose rate from orbit `O` to orbit `O'` is the sum over `z'` in `O'`
of `k(z -> z')` for any `z` in `O`. Orbits are exact under the symmetry tested
by G06 check 2. The lumped solve gives integer orbit-total weights `n_O`,
lifted to integer weights on every state, `w(z) = n_O (N-1)!/|O|` for `z` in
`O`; `w` is an integer because `|O|` divides `(N-1)!`. G06 check 4 verifies the
lifted law on the full chain, independent of the lumping. Logarithms enter only through `sigma_floor`, `eta`, and
`eta_global`, as certified enclosures, as CAL-CEF-3 section 6. No
floating-point value enters any computation that decides a gate, a control, or
a hypothesis.

The report has:
- one line per gate (`PASS` or `FAIL` with the first failure);
- one line per control with its observed and expected first failure;
- one line per world and agent, giving states, orbits, `<N_2>_p`,
  `<N_2>_(pi_c)`, `H`, `J`, `Y`, `rho`, `s*` (or `none`), and `E` (exact
  reduced rationals with four-decimal roundings), and the certified enclosures
  of `eta` and `Y_bound`;
- one line per hypothesis, with `SUPPORTED` or `REFUTED` and every world,
  ladder, or pair at which it fails;
- `OVERALL PASS` or `OVERALL FAIL`, for the gates and controls only.

The frozen commands are `cargo test --release --locked` and then
`cargo run --release --locked --quiet`. Exact elimination takes about ten
minutes per full run (the 660-orbit worlds about 35 s per steady state); the
test suite solves only the primary world `(3, 2, 8, 2)` and its controls, not
the other 62 confirmatory steady states.

## 7. Hypotheses

Each hypothesis is decided by exact rational comparison and reported as
`SUPPORTED` or `REFUTED`, with every world, ladder, or pair at which it fails.
It is `SUPPORTED` only if it holds everywhere it names. An undefined value
counts as a failure where it occurs. A refutation is a result.

- **H0 Fuel buys open future.** `H > 0` for every LOCAL and AIM agent in all 18
  worlds.
- **H1 Local fuel beats warming at equal power.** `E_LOCAL > 1` in all 18
  worlds: fuel spent on the bridges at the traveller's island holds more open
  future for it than the same fuel flow spent warming every bridge. It is the
  analogue of CAL-CEF-3 H1, not a replication: the open future here is counted
  from the traveller's island rather than from home, the comparator is the
  warming rival at equal fuel flow rather than GLOBAL at unit rates, and no
  bridge is protected for free.

Two features of the design bear on H2 and H3 and are fixed before any aimed
agent is solved. First, the measure `N_2(G, x) = 1 + 2 deg(x) + sum over
neighbours y of deg(y)` reads only bridges at the traveller's island and its
neighbours. The bridges at its island, which LOCAL fuels, carry the most
weight; route bridges two or more moves away pay only after the traveller
arrives. Second, the permission sets are nested and partly coincide. AIM-1 is
always a subset of LOCAL, and equals it on 2 of the 27 route-and-cursor classes
of `(3, 2)` and 12 of the 108 of `(3, 3)`, never for `N >= 4`. AIM-`(k + 1)`
differs from AIM-`k` only where step `t + k` ahead or step `t - k - 1` behind
moves on a bridge AIM-`k` does not already hold: 8 of 27 classes in `(3, 2)`,
18 of 48 in `(4, 2)`, and 44 (`k = 1`) and 20 (`k = 2`) of 108 in `(3, 3)`.
AIM-`tau` equals LOCAL on 2 of 27 classes in `(3, 2)` and 16 of 108 in
`(3, 3)`. A refutation of H2 or H3 is a statement about
route-aimed fuel under this measure, not about foresight in general.

- **H2 Foresight sharpens aim.** In every world with `tau >= 2`
  (`(3, 2)`, `(3, 3)`, `(4, 2)`) at every pair, `E_AIM-k` strictly increases
  with `k = 1, ..., tau`: nine ladders.
- **H3 A prepared route beats looking around.** In every world with
  `tau >= 2` at every pair, `E_AIM-tau > E_LOCAL`: fuel on the whole held
  route has a larger equal-power edge than fuel on every bridge at the
  traveller's island.
- **H4 The edge of local fuel grows with the size of the world.** At
  `tau = 1` and every pair, `E_LOCAL` strictly increases from `N = 3` to
  `N = 4` to `N = 5`. CAL-CEF-3 saw a larger ratio `Y_local / Y_global` (home
  measure) with four islands than with three (not preregistered); H4 asks
  whether the equal-power edge of local fuel rises with `N` in this world.

Reported for every world and agent, with no hypothesis attached: `H`, `J`, `Y`,
`rho`, `s*`, `E` (exact), `eta`, `Y_bound` (certified).

## 8. Preregistered falsifying controls

Each control mutates one declared input or reference value of the primary
witness `(3, 2, 8, 2)` and must fail first at the named gate, check, agent, and
configuration. Under a control, the gates run on the primary world only, in the
agent order of section 3. A pass of any control is an apparatus failure. Inputs are not
validated for `b_c > b_h`; that declaration is tested by G08 check 1. After a
control's first failing check, no quantity that divides by `J`, `sigma`, or
`H_warm` is evaluated.

| Control | Mutation | Expected first failure |
|---|---|---|
| C01 one-way fuel | Delete every FUEL return. | G03 check 1, LOCAL, configuration none |
| C02 free protection | WEATHER never wears a bridge the held route uses (the CAL-CEF-3 rule). | G02 check 3, LOCAL, configuration `{{0,1}}` |
| C03 equal stores | `b_h = b_c = 8`. | G08 check 1, LOCAL, world-level (`J = 0`) |
| C04 biased step | Forward STEP rate 2. | G03 check 2, LOCAL, configuration none |
| C05 aim off by one | AIM-`k` aims at the route steps within `k + 1` moves. | G02 check 4, AIM-1, configuration none |
| C06 wrong calibration | G07 checks GLOBAL against `q = 1/b_c`. | G07 check 1, GLOBAL, configuration none |
| C07 wrong rival | The rival's inverse uses `b_h` in place of `b_h + 1`: `s*(J) = J (b_c + 1)/[K (b_c - b_h) - J b_h]`. | G09 check 1, LOCAL, world-level |

C07 must fail: `J < J_max` is a theorem at `(8, 2)`, `J_warm` is strictly
increasing, and the mutated value equals `s*(J)` only if `J = 0`, which G08
check 1 has excluded. Explicitly, with the mutated inverse `s_w`,
`J_warm(s_w(J)) = J K (b_c - b_h)/[K (b_c - b_h) + J] < J` for `J > 0`, and
`s_w(J) > 0` since `J < J_max < K (b_c - b_h)/b_h`.

## 9. Design-time disclosure and blinding

Before this draft, the designer computed:
- the closed forms of sections 2 and 4;
- the state and orbit counts, by enumeration;
- the GLOBAL steady state by exact orbit-lumped elimination, at
  `(b_c, b_h) = (4, 2)` (not a confirmatory pair) for worlds `(3, 2)` and
  `(5, 1)`, confirming the product law and timing the solve.

No steady state of a LOCAL or AIM agent was computed by the designer, at any
temperature. CAL-CEF-3 data bear on H1 and H4 only as described in section 7.
The pre-freeze referee may compute LOCAL and AIM steady states only at
non-confirmatory pairs. The referee may use them only to find exact identities
or degeneracies that would decide a hypothesis by construction. The referee
does not report yields, edges, or their directions to the designer.

In the first review round, an independent referee rebuilt every world in a
separate implementation. It confirmed every count and closed form of sections
2 and 4, solved every GLOBAL world exactly at the three confirmatory pairs, and
the warming family at `s = 1/2` and `s = 3` in worlds `(3, 2)` and `(4, 1)` at
`(3, 2)` and `(8, 2)`. It checked the symmetry and the strong connectivity of
every generator, and simulated the controls' first failures structurally. It
derived the identity `J = (b_c + 1)<|G|>_p - K` and the bound on `J_max`. At
the non-confirmatory pairs `(4, 2)`, `(9, 3)`, `(5, 3)`, and `(5, 4)` it solved
LOCAL and AIM agents only to produce yes-or-no degeneracy flags (no
non-positive or unbalanced law, no `H = 0`, no `J <= 0` or `J >= J_max`, no
`E = 1` outside GLOBAL, no identical steady states, no exact ties). It reported
none, and reported no value or direction. It also timed solves at `(9, 3)` and
solved the equal-store world of C03. Its findings were applied before the
freeze; the aim of AIM-`k` was then made two-sided (section 1.4), because the
cursor moves both ways and a forward-only aim never fuelled the crossings back
along the route. In a second round it repeated the connectivity and symmetry
checks, the structural counts of section 7, the control simulation, and the
degeneracy scan at the same four non-confirmatory pairs for the two-sided
AIM-`k`, again reporting flags only; none was raised.

## 10. What a pass establishes

A pass of every gate and every control establishes that the apparatus
constructs the declared worlds exactly, solves their steady states exactly,
measures `H`, `J`, `Y`, the warming rival, and the edge `E` exactly, and
encloses `sigma_floor`, `eta`, and `Y_bound` in certified intervals. The
hypotheses then stand or fall on their own lines of the report.

Supported hypotheses would establish, within these worlds:
- H0: fuel buys the traveller open future when nothing is protected for free;
- H1: fuel spent at the traveller's island has an equal-power edge over
  warming larger than 1;
- H2: aiming further along the route raises the equal-power edge;
- H3: fuel on the whole held route has a larger equal-power edge than fuel on
  every bridge at the traveller's island;
- H4: the equal-power edge of local fuel rises from three to five islands.

All readings are in the traveller-centred measure `N_2` and relative to the
warming rival at equal fuel flow. Each refuted hypothesis establishes that its
claim fails at the worlds, ladders, or pairs listed in the report; it does not
establish the reverse ordering.

Outside this boundary: routes chosen by what they open (learning or
evaluation); several travellers; worlds that grow; horizons and sizes beyond
those listed. The controller itself is not charged: holding, reading, and
changing the route cost no energy, so `J` counts bridge fuel only (compare
Ehrich, Still, and Sivak 2023).

## 11. Xypher review before execution

Target class: a driven Markov graph system with two declared stores; T1--T5 of
[[Xypher Operational Thermodynamics Boundary]] apply to the undriven world, and
each driven channel obeys local detailed balance with its own store.

- Diamond: configuration, route, and cursor. Crystal resolution: REPLAN over
  all routes.
- Ruby: empty. Opal/Phi: absent. Thompson memory, learning, teach, absorb,
  crystallize: not applicable.
- alpha: two temperatures, each grounded by its store's equation of state before
  any rate.
- Declared parameters `N`, `tau`, `R = 2`, `b_c`, `b_h`, `k`, `lambda = 1`, unit
  rates, and home island `0` are enumerated experimental inputs. The
  permission sets of section 1.4 are the declared agents; no cap, threshold,
  or content-based branch acts on the dynamics beyond them.

## 12. Review and freeze

Before execution, the exact commits of this boundary and of the apparatus are
reviewed for mathematical correspondence, adversarial checker behaviour, and
Futuruna/Xypher semantics. Only non-executing checks (`runa check`,
`cargo check`, `cargo test --no-run`, `cargo fmt --check`, `cargo clippy`) run
on the apparatus before approval. The reused arithmetic modules were executed
in the CAL-CEF-3 arithmetic test and run. The approved commits are pushed, the
frozen commands run once, and the result is recorded in
[[Aimed Foresight Result]] with provenance and an independent check written
from this boundary alone.

## 13. Related work

Measures of open future without an energy price: empowerment, the channel
capacity from action sequences to future states (Klyubin, Polani, and
Nehaniv 2005); causal entropic forces over future paths (Wissner-Gross and
Freer 2013), with Kappen's observation that path entropy carries no force
unless structure makes it state-dependent (2013); future-state maximisation
(Charlesworth and Turner 2019); and the maximum occupancy principle
(Ramirez-Ruiz et al. 2024). Prices without a measure of open future: the
minimum entropy production to hold a nonequilibrium state (Horowitz, Zhou, and
England 2017), extended to ancillary control (Horowitz and England 2017); the
thermodynamics of feedback, prediction, and information flow (Sagawa and Ueda
2010; Still, Sivak, Bell, and Crooks 2012; Horowitz and Esposito 2014;
Parrondo, Horowitz, and Sagawa 2015; Ehrich, Still, and Sivak 2023); semantic
information as what a system needs to stay viable (Kolchinsky and Wolpert
2018); and dissipative adaptation (England 2015). The closest combination is
Takahashi and Hayashi's empowerment per joule, measured above a null policy
and at a fixed energy budget (2026). Here the open future is the excess count
of near walks in an exact nonequilibrium steady state, the price is the net
heat flux from a declared hot store set against the Horowitz--Zhou--England
floor for the same state, the null rival warms every bridge with the same fuel
flow, and foresight is the depth to which that fuel is aimed along a held
route.

- A. S. Klyubin, D. Polani, and C. L. Nehaniv, "Empowerment: A universal
  agent-centric measure of control," 2005 IEEE Congress on Evolutionary
  Computation 1, 128 (2005), doi:10.1109/CEC.2005.1554676.
- A. D. Wissner-Gross and C. E. Freer, "Causal entropic forces," Phys. Rev.
  Lett. 110, 168702 (2013), doi:10.1103/PhysRevLett.110.168702.
- H. J. Kappen, "Comment: Causal entropic forces," arXiv:1312.4185 (2013).
- H. J. Charlesworth and M. S. Turner, "Intrinsically motivated collective
  motion," Proc. Natl. Acad. Sci. USA 116, 15362 (2019),
  doi:10.1073/pnas.1822069116.
- J. Ramirez-Ruiz, D. Grytskyy, C. Mastrogiuseppe, Y. Habib, and
  R. Moreno-Bote, "Complex behavior from intrinsic motivation to occupy future
  action-state path space," Nat. Commun. 15, 6368 (2024),
  doi:10.1038/s41467-024-49711-1.
- J. M. Horowitz, K. Zhou, and J. L. England, "Minimum energetic cost to
  maintain a target nonequilibrium state," Phys. Rev. E 95, 042102 (2017),
  doi:10.1103/PhysRevE.95.042102.
- J. M. Horowitz and J. L. England, "Information-theoretic bound on the entropy
  production to maintain a classical nonequilibrium distribution using
  ancillary control," Entropy 19, 333 (2017), doi:10.3390/e19070333.
- T. Sagawa and M. Ueda, "Generalized Jarzynski equality under nonequilibrium
  feedback control," Phys. Rev. Lett. 104, 090602 (2010),
  doi:10.1103/PhysRevLett.104.090602.
- S. Still, D. A. Sivak, A. J. Bell, and G. E. Crooks, "Thermodynamics of
  prediction," Phys. Rev. Lett. 109, 120604 (2012),
  doi:10.1103/PhysRevLett.109.120604.
- J. M. Horowitz and M. Esposito, "Thermodynamics with continuous information
  flow," Phys. Rev. X 4, 031015 (2014), doi:10.1103/PhysRevX.4.031015.
- J. M. R. Parrondo, J. M. Horowitz, and T. Sagawa, "Thermodynamics of
  information," Nat. Phys. 11, 131 (2015), doi:10.1038/nphys3230.
- J. Ehrich, S. Still, and D. A. Sivak, "Energetic cost of feedback control,"
  Phys. Rev. Research 5, 023080 (2023),
  doi:10.1103/PhysRevResearch.5.023080.
- A. Kolchinsky and D. H. Wolpert, "Semantic information, autonomous agency and
  non-equilibrium statistical physics," Interface Focus 8, 20180041 (2018),
  doi:10.1098/rsfs.2018.0041.
- J. L. England, "Dissipative adaptation in driven self-assembly," Nat.
  Nanotechnol. 10, 919 (2015), doi:10.1038/nnano.2015.250.
- K. Takahashi and Y. Hayashi, "Thermodynamic limits of physical
  intelligence," in Artificial General Intelligence (AGI 2026), Lecture Notes
  in Computer Science, 339 (Springer, 2026),
  doi:10.1007/978-3-032-33195-3_24.
- CAL-CEF-1, CAL-CEF-2, and CAL-CEF-3 result records: the archipelago, its
  temperature, the price of foresight, and the open future price.
