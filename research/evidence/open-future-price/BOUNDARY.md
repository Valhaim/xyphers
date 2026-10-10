---
title: Open Future Price Boundary
aliases:
  - CAL-CEF-3 Boundary
  - How Much Open Future Can a Unit of Energy Buy
tags:
  - domain/physics
  - type/preregistration
  - topic/thermodynamics
  - topic/xypher
  - topic/nonequilibrium
domain: Physics
type: preregistration
status: draft-no-evaluation
td: td-67b58b
created: 2026-10-10
updated: 2026-10-10
related:
  - "[[Causal Entropic Thermodynamics Result]]"
  - "[[Control Information Thermodynamics Result]]"
  - "[[Xypher Operational Thermodynamics Boundary]]"
---

# Open future price boundary (CAL-CEF-3)

## 0. The question

> How much open future can a unit of energy buy?

CAL-CEF-1 showed that in a world where a traveller holds its futures as state,
the log-count of those futures is the configuration entropy, and the world
settles at the temperature of its timber store. CAL-CEF-2 priced foresight in that world:
`alpha ln 2` per bit. Both worlds sat at equilibrium. Nothing was fed, nothing
was kept, and every result followed from the setup once the setup was exact.

This boundary leaves equilibrium. The archipelago now has weather: a cold store
builds and wears bridges everywhere, as an environment does. The traveller has
fuel: a hot store it may draw timber from, but only where it stands. Fed this
way, the world settles into a steady state through which energy flows from the
hot store, through the bridges, into the cold one. Whether that steady state
keeps more futures open than the cold world would is hypothesis H0.

Physics already sets the price floor. Holding a state away from equilibrium by
adding driven transitions, without changing the energies or the undriven
rates, costs at least the rate at which the undriven world would relax it
(Horowitz, Zhou, and England, *Phys. Rev. E* 95, 042102, 2017): you pay at
least as fast as the world forgets. This boundary measures how much open future
real agents hold per unit of fuel flow, how close they come to that floor, and
whether targeted action and foresight buy more. The agents' outcomes are not
computed before execution. The hypotheses in section 7 can be refuted, and any
refutation is reported as a result.

## 1. The world

### 1.1 Archipelago and plans

As CAL-CEF-1, sections 3.1--3.2: islands `V = {0, ..., N-1}`, home `h = 0`,
candidate bridges the `K = N(N-1)/2` unordered pairs in lexicographic order,
configurations `G` ordered by bit pattern, plans `p = (v_0, ..., v_tau)` from
home with each step a stay or a crossing of a built bridge,
`N_tau(G) = |Pi_tau(G)| = e_h^T L_G^tau 1`. A plan uses a bridge if some step
crosses it.

A system state is `z = (G, p, t)` with `p` in `Pi_tau(G)` and cursor
`t in {0, ..., tau}`; the traveller stands on `x(z) = v_t`. System energy
`U(z) = lambda |G|`, `lambda = 1`.

### 1.2 Two stores

Two ideal timber stores exchange packets with the bridges. Each is declared by
its equation of state: its arrangements multiply by a fixed factor per packet
it holds, `b_c` for the cold store and `b_h` for the hot store, with
`b_c > b_h >= 2`. Their temperatures are

```text
alpha_c = lambda / ln b_c,      alpha_h = lambda / ln b_h,      alpha_h > alpha_c.
```

Each store is large enough that its temperature does not change. A packet
moving into a bridge from store `i` and a packet moving back into store `i`
have rates in the ratio `1 : b_i`, which is local detailed balance with that
store's temperature.

### 1.3 Channels

All rates are exact integers; the unit rate fixes the clock.

- **STEP**: `(G, p, t) <-> (G, p, t+1)` for `0 <= t < tau`, rate 1 each way.
- **REPLAN**: at `t = 0`, `(G, p, 0) <-> (G, p', 0)` for every `p' != p` in
  `Pi_tau(G)`, rate 1 each way.
- **WEATHER** (the environment, cold store): for every candidate bridge `e`,
  anywhere: if `e` is not in `G`, build at rate 1 (one packet from the cold
  store); if `e` is in `G` and the held plan does not use it, wear it down at
  rate `b_c` (one packet to the cold store). Weather never removes a bridge the
  held plan uses.
- **FUEL** (the agent, hot store), in one of two declared agents:
  - **LOCAL**: for every candidate bridge `e` incident to the traveller's
    island `x(z)`: if `e` is not in `G`, build at rate 1 (one packet from the
    hot store); if `e` is in `G` and the held plan does not use it, return it at
    rate `b_h` (one packet to the hot store).
  - **GLOBAL**: the same, for every candidate bridge anywhere.

The traveller has no unfuelled bridge moves: it acts only with fuel, and the
world acts only through weather. No rate reads any count of futures. The
protection of bridges the held plan uses is a kinetic constraint that costs no
energy; it inherits the CAL-CEF-1 rule, acts in the undriven world too, and
grows with `tau`.

### 1.4 The undriven world and the driven world

The **undriven world** has STEP, REPLAN, and WEATHER. Every channel is
reciprocal; STEP and REPLAN have equal rates, and every WEATHER toggle has the
cold store's ratio. Its stationary law is
`pi_c(z) proportional to b_c^(-|G|)` and its configuration law is
`N_tau(G) b_c^(-|G|)`, the CAL-CEF-1 law at the cold temperature.

The **driven world** adds FUEL. Its steady state `p` is the unique solution of
`p Q = 0` with `sum p = 1` on the connected system-state graph. Energy flows from
the hot store, through the bridges, into the cold store.

For the GLOBAL agent, FUEL and WEATHER act on the same moves, so every toggle
has total build rate 2 and total wear rate `b_c + b_h`. Its steady state is the
equilibrium law of a single effective store, `p(z) proportional to q^|G|` with
`q = 2/(b_c + b_h)`: fuel spent everywhere is the same as warming the whole
world. The LOCAL agent's steady state has no such form.

## 2. What is measured

Fix a reference horizon `R`. The **open future** of a configuration is
`N_R(G)`, the number of `R`-step plans from home: how many near futures are
open. `R` is the same for every agent, whatever horizon `tau` it plans with.

For each agent:

```text
H = <N_R>_p - <N_R>_(pi_c)           open futures held above the undriven world,
J = net packets per unit time from the hot store into the bridges (fuel flow),
Y = H / J                            open futures held per unit of fuel flow.
```

`Y` is the open future held above the undriven world per unit of fuel flow, at
this agent's operating point; it is not a marginal price. It answers the
question of section 0 for one agent at its own operating point. `H`, `J`, and
`Y` are exact rationals. Parallel WEATHER and FUEL channels between the same
pair of states are separate channels in `sigma`, `sigma_floor`, and gate G08;
they are never lumped.

The price paid is the entropy production of the steady state,

```text
sigma = J ln(b_c / b_h),
```

and the price floor is the entropy production of the undriven channels at the
held state,

```text
sigma_floor = sum over undriven channel pairs of
              [p(z) k(z -> z') - p(z') k(z' -> z)] ln{[p(z) k(z -> z')] / [p(z') k(z' -> z)]}
            = - d/dt D(p || pi_c) under the undriven dynamics.
```

No agent whose moves are added transitions on the same states, each in local
detailed balance with the hot store, leaving WEATHER, STEP, and REPLAN
unchanged, can hold `p` with a smaller fuel flow than
`J_floor = sigma_floor / ln(b_c / b_h)` (Horowitz, Zhou, and England). The
agent's efficiency and the yield bound for the same held state are

```text
eta = sigma_floor / sigma < 1,       Y_bound = Y / eta = H ln(b_c / b_h) / sigma_floor.
```

`Y_bound` is an upper bound on the yield of any agent of this class holding the
same `p`. With a hot store of fixed `b_h`, no agent attains it: Horowitz, Zhou,
and England reach the floor only with driving forces matched to `p` on each
transition.

`sigma_floor` involves logarithms of the steady-state probabilities; it and
`eta` are reported as certified rational enclosures (section 6), and no
hypothesis depends on them.

## 3. Cases

The reference horizon is `R = 2`. Three temperature pairs:
`(b_c, b_h) = (3, 2)`, `(8, 2)`, and `(6, 3)`. No LOCAL steady state was
computed at any of them before freeze; GLOBAL steady states there are the
closed forms of section 4.2, which the referee also solved explicitly
(section 9). The dropped first design of section 9 item 1 was evaluated at
`(3, 2)`.

| Case set | `N` | `tau` | Agents |
|---|---:|---|---|
| Foresight ladder | 3 | 1, 2, 3 | LOCAL, GLOBAL |
| Wider archipelago | 4 | 1 | LOCAL, GLOBAL |

That gives twelve worlds, each run with both agents: 24 steady states. The
primary witness for gates and controls is `(N, tau, R, b_c, b_h) = (3, 2, 2, 8, 2)`
with the LOCAL agent.

## 4. Exact predictions fixed in advance

These follow from closed forms and do not involve the LOCAL agent's steady
state.

### 4.1 State counts and the undriven world

| `(N, tau)` | System states | `<N_2>` under `pi_c` at `(b_c, b_h) = (3,2)` | at `(8,2)` | at `(6,3)` |
|---|---:|---:|---:|---:|
| (3, 1) | 32 | 55/16 | 1955/891 | 367/147 |
| (3, 2) | 108 | 117/28 | 3497/1233 | 99/31 |
| (3, 3) | 344 | 262/55 | 6827/1955 | 1411/367 |
| (4, 1) | 320 | 19/4 | 226/81 | 797/245 |

(The `b_c` of the pair determines `pi_c`.)

### 4.2 The GLOBAL agent

With `q = 2/(b_c + b_h)`, `H_global = <N_2>_q - <N_2>_(pi_c)` and
`J_global = [(b_c - b_h)/(b_c + b_h)] (K - <|G|>_q)`:

| `(N, tau, b_c, b_h)` | `H_global` | `J_global` | `Y_global` |
|---|---:|---:|---:|
| (3, 1, 3, 2) | 2531/8624 | 29/77 | 0.7792 |
| (3, 2, 3, 2) | 1115/3948 | 323/987 | 0.8630 |
| (3, 3, 3, 2) | 28563/110605 | 579/2011 | 0.8969 |
| (3, 1, 8, 2) | 7429/14256 | 11/8 | 0.3790 |
| (3, 2, 8, 2) | 54815/91242 | 91/74 | 0.4885 |
| (3, 3, 8, 2) | 1143/1955 | 429/391 | 0.5329 |
| (3, 1, 6, 3) | 6308/17787 | 41/55 | 0.4757 |
| (3, 2, 6, 3) | 3852/9889 | 635/957 | 0.5870 |
| (3, 3, 6, 3) | 77916/211025 | 1019/1725 | 0.6250 |
| (4, 1, 3, 2) | 1245/2548 | 72/91 | 0.6176 |
| (4, 1, 8, 2) | 65/81 | 17/6 | 0.2832 |
| (4, 1, 6, 3) | 278676/503965 | 288/187 | 0.3590 |

`Y_global` is the exact rational `H_global / J_global`; the decimal is shown
for reading. For the GLOBAL agent every toggle carries equal and opposite
WEATHER and FUEL currents, so its efficiency is independent of `N` and `tau`:

```text
eta_global = ln[2 b_c / (b_c + b_h)] / ln(b_c / b_h),
```

`0.4497` at `(3, 2)`, `0.3390` at `(8, 2)`, and `0.4150` at `(6, 3)`. These
values calibrate the solver and the logarithms (gates G07 and G08); they are not
hypotheses.

## 5. Gates

Gates run G01 to G10; checks within a gate run in order over all worlds and
states in canonical order. Worlds run in the row order of the table in section 4.2;
within a world, LOCAL before GLOBAL. The first failure is the first failing
check of the lowest-numbered failing gate, at the first world and
configuration in that order. The gates test the
apparatus and the construction. The hypotheses of section 7 are evaluated
afterwards and reported whether they hold or not.

| Gate | Checks in order |
|---|---|
| G01 grounding | (1) `N_tau(G) >= 1`; (2) depth-first plan counts equal `e_h^T L_G^tau 1` for `tau` and for `R`; (3) every world has the system-state count of section 4.1. |
| G02 executable rules | (1) every channel's destination is a system state of the world, with a plan valid in its configuration; (2) STEP and REPLAN as declared; (3) every WEATHER channel toggles a candidate bridge, every permitted WEATHER build is present, and no WEATHER channel removes a bridge the held plan uses; (4) every FUEL channel toggles a bridge its agent may toggle (LOCAL: incident to `x(z)`; GLOBAL: any candidate bridge), every permitted FUEL build is present, and no FUEL channel removes a bridge the held plan uses; the presence of WEATHER wear and FUEL returns is tested by G03 check 1; (5) no channel changes the plan or cursor except STEP and REPLAN. |
| G03 reciprocity | (1) every channel has a reverse in the same class; (2) STEP and REPLAN rates are 1 both ways, WEATHER build : wear = `1 : b_c`, FUEL build : return = `1 : b_h`. |
| G04 accounting | (1) every bridge change moves exactly one packet between the bridges and exactly one named store (WEATHER: cold; FUEL: hot), and STEP and REPLAN move none; (2) in every driven steady state, the net packet flow from the hot store equals the net packet flow into the cold store, exactly. |
| G05 undriven world | (1) the undriven system-state graph is connected; (2) `b_c^(-|G|)` is stationary with detailed balance on every channel. |
| G06 driven steady state | (1) the driven system-state graph is connected; (2) the exact solution `p` of `p Q = 0`, `sum p = 1` is strictly positive; (3) every state's inflow equals its outflow under `p`, exactly. |
| G07 GLOBAL calibration | (1) the GLOBAL steady state equals `q^|G|` normalised, with `q = 2/(b_c + b_h)` from the world's own stores; (2) `H_global` and `J_global` equal their closed forms of section 4.2 evaluated at the world's own stores. |
| G08 second law and the floor | (1) `J > 0` in every driven world; (2) `sum over all channel pairs of [p(z) k - p(z') k'] ln(k/k') = J ln(b_c/b_h)`, checked as exact rational coefficients of two formal per-store symbols `L_c` and `L_h`; (3) every channel pair contributes a nonnegative term to `sigma_floor` or to the fuel part `sigma - sigma_floor`, so `0 <= sigma_floor <= sigma`; (4) certified enclosures of `sigma_floor` and `eta` have width at most `10^-9`, and the enclosure of `eta` lies strictly inside `(0, 1)`; (5) for the GLOBAL agent, the certified enclosure of `ln[2 b_c/(b_c + b_h)] / ln(b_c/b_h)`, of width at most `10^-9`, intersects the certified enclosure of `eta`. |
| G09 Xypher slot reading | Graph Substrate: configurations, plan, cursor; Crystal: REPLAN over `Pi_tau(G)`; Thermo: two declared stores, `U = lambda |G|`; Praxion: base actor with FUEL moves; environment: WEATHER; Ruby empty; Opal absent; every rate function takes only the state, the bridge, the channel class, and its store's ratio, so no rate reads `N_tau`, `N_R`, `H`, `J`, `Y`, or `eta`. |
| G10 frozen values | (1) `<N_2>` under `pi_c` matches section 4.1 for every world; (2) `H_global` and `J_global` match section 4.2 exactly for every world, `Y_global = H_global / J_global` exactly, and its four-decimal rounding matches section 4.2; (3) `eta_global` rounded to four decimals matches section 4.2. |

Classification. Independent tests: G01 checks 2--3, G02, G03, G04 check 1,
G05, G06 checks 2--3 (check 3 verifies the solver), G07 check 1, the width part
of G08 check 4, G08 check 5 (verifies the logarithms), G09, and G10.
Consistency checks or theorems given earlier gates: G01 check 1 (the stay
plan), G04 check 2, G06 check 1 (implied by G05 check 1), G07 check 2, G08
checks 1--3, and the sign part of G08 check 4; G08 check 1 fails only if
`b_c = b_h` (C03) or on a sign error.

## 6. Apparatus

A standalone dependency-free Rust crate at
`research/physics/xypher-open-future-price-proof/`, with a prospective
Futuruna declaration `xypher.runa`. Arithmetic is exact: arbitrary-precision
integers implemented in the crate, reduced rationals over them, and
fraction-free (Bareiss) elimination for every steady state. Logarithms enter
only through `sigma_floor`, `eta`, and `eta_global`. Each logarithm argument is
first enclosed between rationals of bounded size (the logarithm is monotone),
then the logarithm is enclosed by a rational series with an explicit remainder
bound; equivalently
`sigma_floor = - sum_z (p L_undriven)(z) [ln n_z + |G(z)| ln b_c]` with
`p(z) = n_z / D` from the integer solution, so the common denominator cancels.
An enclosure that does not meet G08 check 4 is a failure. No floating-point
value enters any computation that decides a gate, a control, or a hypothesis.
Decimals in the report are rounded from exact rationals or from certified
enclosures, for reading only.

The frozen commands are `cargo test --release --locked` and then
`cargo run --release --locked --quiet`. The release profile is used because
exact elimination over 344 states is slow without optimisation.

The report has one line per gate (`G01`--`G10`, `PASS` or `FAIL`); one line
per control with observed and expected first failure; one line per world and
agent with system states, `<N_2>_p`, `<N_2>_(pi_c)`, `H`, `J`, `Y` (exact and
to four decimals), and the certified enclosures of `eta` and `Y_bound`; one
line per hypothesis with `SUPPORTED` or `REFUTED` and every world or pair at
which it fails; and `OVERALL PASS` or `OVERALL FAIL` for the gates and
controls.

## 7. Hypotheses

Each hypothesis is decided by exact rational comparison and reported as
`SUPPORTED` or `REFUTED`, with every world or temperature pair at which it
fails listed. A hypothesis is `SUPPORTED` only if it holds at every world or
pair it names. A refutation is a result, not an apparatus failure: the
report's `OVERALL` line reflects the gates and controls only.

- **H0 Fuel buys open future.** `H_local > 0` in every one of the twelve
  worlds.
- **H1 Targeting beats heating.** `Y_local > Y_global` in every one of the
  twelve worlds, at unit fuel rates. Fuel spent where the traveller stands buys
  more open future per unit of fuel flow than fuel spent everywhere.
- **H2 Yield rises with horizon.** For each temperature pair, `Y_local`
  strictly increases from `tau = 1` to `tau = 2` to `tau = 3` (`N = 3`).
  Raising `tau` changes the world (states, plan multiplicity, protected
  bridges, and the undriven baseline) as well as the agent; GLOBAL, which does
  no targeting, already shows this rise (section 4.2), so H2 alone does not
  attribute the rise to foresight.
- **H3 Foresight sharpens targeting.** For each temperature pair, the
  advantage `Y_local / Y_global` strictly increases from `tau = 1` to
  `tau = 2` to `tau = 3` (`N = 3`). Section 4.2 shows `Y_global` itself rising
  with `tau`: configurations with more bridges carry more `tau`-step plans, so
  the held law sits closer to the full archipelago and leaves fewer bridges to
  build. H3 is the test of foresight beyond that change of world; where H0
  holds, H3 SUPPORTED implies H2 SUPPORTED.

Reported for every world, with no hypothesis attached: `H`, `J`, `Y` (exact),
`eta`, and `Y_bound` (certified).

## 8. Preregistered falsifying controls

Each control mutates one declared input or reference value of the primary
witness and must fail first at the named gate and check. A pass of any control
is an apparatus failure. Inputs are not validated for `b_c > b_h`; that
declaration is tested by G08 check 1. After a control's first failing check,
no quantity that divides by `J` or `sigma` is evaluated.

| Control | Mutation | Expected first failure |
|---|---|---|
| C01 one-way fuel | Delete every FUEL return channel, keeping FUEL builds. | G03 check 1, first at configuration none |
| C02 plan-breaking weather | WEATHER may also remove bridges the held plan uses. | G02 check 1, first at configuration `{0,1}` |
| C03 equal stores | Set `b_h = b_c = 8`. | G08 check 1 (`J = 0`) |
| C04 biased weather | Double the first WEATHER build rate in canonical state order. | G03 check 2, first at configuration none |
| C05 wrong store | FUEL uses the cold store's ratio `1 : b_c` instead of `1 : b_h`. | G03 check 2, first at configuration none |
| C06 wrong calibration | Mutates G07's reference law, not a world input: check the GLOBAL steady state against `q = 1/b_c` instead of `2/(b_c + b_h)`. | G07 check 1, GLOBAL agent of the primary, first at configuration none |

## 9. Design-time disclosure

Before this draft, a pilot with floating-point arithmetic explored two designs
at `N = 3`, with the open future measured as `<ln N_tau>` and the cost as
entropy production `sigma`.

1. In a first design, fuel and the cold store acted on the same moves (both
   local to the traveller, no weather). It was evaluated at
   `(b_c, b_h) = (4, 2)`, `(3, 2)`, and `(9, 3)`. `eta` was identical at every
   horizon tried (`0.4150` at `(4, 2)`, `0.4497` at `(3, 2)`, `0.3691` at
   `(9, 3)`) and equals `ln[2 b_c/(b_c + b_h)] / ln(b_c/b_h)`, because that
   world behaves as a single store at an intermediate temperature. That design
   was dropped: its agency questions could not fail. It is a different world
   from the present design, whose LOCAL steady states at `(3, 2)` were not
   computed.
2. In the present design (local fuel, weather everywhere), evaluated only at
   `(4, 2)` and `(9, 3)`, the local agent held more extra `<ln N_tau>` per unit
   of `sigma` than the global one: at `(4, 2)`, `0.238` against `0.158` at
   `tau = 1`, `0.477` against `0.330` at `tau = 2`, and `0.614` against
   `0.448` at `tau = 3`. Both rose with `tau`, while their ratio fell (about
   `1.51`, `1.45`, `1.37`). The local agent's `eta` was slightly below the
   global agent's `0.4150` at `(4, 2)` (`0.4073`, `0.4007`, `0.3947` for
   `tau = 1, 2, 3`). The falling ratio motivates H3 as a genuine risk.

The confirmatory measures differ from the pilot's: the open future is the count
`<N_2>` at a fixed reference horizon, and the cost is the fuel flow `J`. No
steady state of a LOCAL agent at a confirmatory temperature pair was computed.
Section 4 values are closed forms of the undriven world and of the GLOBAL
agent.

An independent pre-freeze referee then built, in a separate throwaway
implementation, the undriven, LOCAL, and GLOBAL generators of every world. It
confirmed the state counts and connectivity of all 36 generators by graph
search; solved every GLOBAL steady state exactly and found the product law
`q^|G|` and every value of section 4; for the primary under each control,
evaluated the structural gates (G01 check 3, G02, G03, connectivity), solved
the equal-store world of C03 (LOCAL and GLOBAL, `p = pi_c`, `J = 0`) and the
GLOBAL primary of C06, and found agreement with every expected first failure
of section 8 under the final gate wording. It computed LOCAL steady states
only at the pilot pairs `(4, 2)` and `(9, 3)`, to check the floor identity, to
time the solver, and to recompute the pilot numbers of item 2, and at the
equal-store world of C03. It also solved GLOBAL with fuel rates scaled by `s`
(product law with `q = (1 + s)/(b_c + s b_h)`) at the confirmatory pairs. It
did not run the apparatus of section 6, which did not yet exist.

## 10. What a pass establishes

A pass of every gate and every control establishes that the apparatus
constructs the declared driven worlds exactly, solves their steady states
exactly, and measures `H`, `J`, `Y`, `sigma`, and the price floor correctly.
The hypotheses then stand or fall on their own lines of the report.

Supported hypotheses would establish, within these worlds:

- H0: fuel, spent by an agent that holds its futures as state, keeps more
  near futures open than the cold world alone;
- H1: spending fuel where the traveller stands buys more open future per unit
  of fuel flow than spending it everywhere, at unit fuel rates;
- H2: `Y_local` rises with `tau` in these worlds; with H3, it rises by a larger
  factor than `Y_global`, the yield of a non-targeting agent;
- H3: foresight makes targeted action more valuable relative to warming the
  whole world.

Each refuted hypothesis establishes its negation for the stated worlds.

Outside this boundary: worlds that grow new islands; agents that learn;
several travellers; and the least fuel flow attainable with a hot store of
fixed `b_h`, which lies strictly above `J_floor`.

## 11. Xypher review before execution

Target class: a driven Markov graph system with two declared stores; T1--T5 of
[[Xypher Operational Thermodynamics Boundary]] apply to the undriven world, and
each driven channel obeys local detailed balance with its own store.

- Diamond: bridge configurations and the held plan; Crystal resolution: REPLAN
  over valid plans.
- Ruby: empty. Opal/Phi: absent. Thompson memory, learning, teach, absorb,
  crystallize: not applicable.
- alpha: two temperatures, each grounded by its store's equation of state before
  any rate.
- Declared parameters `N`, `tau`, `R`, `b_c`, `b_h`, `lambda = 1`, unit rates,
  and home island `0` are enumerated experimental inputs. No cap, threshold, or
  content-based branch acts on the dynamics beyond the declared permission
  rules.

## 12. Review and freeze

Before execution, the exact commits of this boundary and of the apparatus are
reviewed for mathematical correspondence, adversarial checker behaviour, and
Futuruna/Xypher semantics. Reviewers do not compute any steady state of a LOCAL
agent at a confirmatory temperature pair. Only non-executing checks
(`runa check`, `cargo check`, `cargo test --no-run`, `cargo fmt --check`,
`cargo clippy`) run before approval. The approved commits are pushed, the
frozen commands run once, and the result is recorded in
[[Open Future Price Result]] with provenance.

## 13. Related work

- J. M. Horowitz, K. Zhou, and J. L. England, "Minimum energetic cost to
  maintain a target nonequilibrium state," *Phys. Rev. E* 95, 042102 (2017):
  the price floor.
- J. Schnakenberg, *Rev. Mod. Phys.* 48, 571 (1976): entropy production of
  master equations.
- U. Seifert, *Rep. Prog. Phys.* 75, 126001 (2012): stochastic thermodynamics.
- A. D. Wissner-Gross and C. E. Freer, *Phys. Rev. Lett.* 110, 168702 (2013):
  the push toward open futures.
- CAL-CEF-1 and CAL-CEF-2 result records: the archipelago, its temperature,
  and the price of foresight.
