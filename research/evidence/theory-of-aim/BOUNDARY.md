---
title: Theory of Aim Boundary
aliases:
  - CAL-CEF-5 Boundary
  - Can a Theory Predict the Price of Open Future
tags:
  - domain/physics
  - type/preregistration
  - topic/thermodynamics
  - topic/xypher
  - topic/nonequilibrium
domain: Physics
type: preregistration
status: draft-no-evaluation
td: td-3966fe
created: 2026-10-11
updated: 2026-10-11
related:
  - "[[Cavity Theory of the Price of Open Future]]"
  - "[[Aimed Foresight Boundary]]"
  - "[[Aimed Foresight Result]]"
---

# Theory of aim boundary (CAL-CEF-5)

## 0. The question

> Can a theory predict, in advance and in numbers, how much open future an
> agent's energy buys?

CAL-CEF-4 measured, exactly, the edge `E` of agents that spend fuel on
purpose. `E` is the open future an agent holds for a traveller, divided by the
open future that random placement of the same fuel would hold. After that run,
a cavity mean-field theory with no fitted parameter reproduced all 48
non-trivial edges within 0.62% (Cavity Theory of the Price of Open Future,
section 2). That match is retrospective: the theory was written after the
data.

This boundary tests the theory prospectively. Its numbers for 56 new steady
states are frozen here (section 4.3), before any exact computation of those
states. The worlds differ from every CAL-CEF-4 world in at least one of three
ways: two new pairs of stores, travellers four and sixteen times faster, and a
measure that looks three steps ahead. The theory also makes four
qualitative predictions, tested in these worlds:
- faster travellers gain less from aim (H2);
- fuel aimed further along a route chosen blind lowers the edge (H3);
- fuel on the route's bridges at the traveller's island beats fuel on all of
  its bridges (H4);
- a closed-form law over-predicts the edge of local fuel by at most 3% (H5).

Each prediction can fail, and any failure is reported as a result.

## 1. The world

As the Aimed Foresight Boundary (CAL-CEF-4), sections 1.1--1.5, with two
declared parameters added:

- **Speed `v`** (a positive integer). STEP runs at rate `v` each way and REPLAN
  at rate `v` to each other route. All other rates are unchanged: WEATHER
  builds at 1 and wears at `b_c`; FUEL builds at 1 and returns at `b_h`.
- **Measure radius `R`** (2 or 3). The open future of the traveller is
  `N_R(G, x) = e_x^T (I + A_G)^R 1`, the number of `R`-step walks from the
  island where it stands, each step a stay or a crossing.

Routes, states, permission sets (LOCAL, two-sided AIM-`k`, GLOBAL), the
undriven law `pi_c proportional to b_c^(-|G|)`, the GLOBAL product law
`q^|G|` with `q = 2/(b_c + b_h)`, the relabelling symmetry, the orbit
solution, and its lifting are unchanged. None of them depends on `v` or `R`.

## 2. What is measured

As CAL-CEF-4 section 2, with `Phi_N` replaced by the radius-`R` closed form

```text
Phi_{N,R}(r) = sum over walks w of R steps from home on the complete graph of r^{b(w)},
```

where `b(w)` is the number of distinct bridges the walk crosses. This is
`<N_R>` under independent bridges each present with probability `r`:
`Phi_{N,2}(r) = 1 + 3(N-1) r + (N-1)(N-2) r^2` and
`Phi_{3,3}(r) = 1 + 14 r + 10 r^2 + 2 r^3`. Then
`H`, `J`, `Y`, `rho = <|G|>_p / K`, and `E = H / [Phi_{N,R}(rho) -
Phi_{N,R}(r_0)]`. The identity `J = (b_c + 1)<|G|>_p - K`, the warming
family, `s*`, `J_max`, `sigma`, `sigma_floor`, `eta`, and `Y_bound` are as in
CAL-CEF-4. Here `b_c >= 2 b_h` at both pairs, so `J < J_max` holds for every
agent.

The theory's value `E_th` for each agent is the frozen decimal of section 4.3,
read as an exact rational. The theory's simple law for LOCAL, used in H5, is

```text
L(N, rho) = N (3 + (N - 2) r_0) / (2 (3 + (N - 2)(rho + r_0))),
```

evaluated exactly with `r_0 = 1/(1 + b_c)` of the world's own cold store
and `rho = rho_LOCAL` of the same world.

## 3. Cases

Two pairs of stores, `(b_c, b_h) = (5, 2)` and `(12, 3)`. For each pair, ten
worlds `(N, tau, v, R)`, in this order:

| # | `(N, tau)` | `v` | `R` | Agents |
|---:|---|---:|---:|---|
| 1 | (3, 2) | 1 | 2 | LOCAL, AIM-1, AIM-2, GLOBAL |
| 2 | (3, 2) | 4 | 2 | LOCAL, AIM-1, AIM-2, GLOBAL |
| 3 | (3, 2) | 16 | 2 | LOCAL, AIM-1, AIM-2, GLOBAL |
| 4 | (4, 2) | 1 | 2 | LOCAL, AIM-1, AIM-2, GLOBAL |
| 5 | (4, 2) | 4 | 2 | LOCAL, AIM-1, AIM-2, GLOBAL |
| 6 | (4, 2) | 16 | 2 | LOCAL, AIM-1, AIM-2, GLOBAL |
| 7 | (3, 1) | 1 | 2 | LOCAL, AIM-1, GLOBAL |
| 8 | (4, 1) | 1 | 2 | LOCAL, AIM-1, GLOBAL |
| 9 | (5, 1) | 1 | 2 | LOCAL, AIM-1, GLOBAL |
| 10 | (3, 3) | 1 | 3 | LOCAL, AIM-1, AIM-2, AIM-3, GLOBAL |

That is 20 worlds and 76 steady states: 56 LOCAL and AIM agents and 20
GLOBAL. Worlds run pair by pair, `(5, 2)` first, in the row order above;
within a world, agents run in the order listed. The primary witness for
controls is world 2 at `(5, 2)`: `(N, tau, b_c, b_h, v, R) = (3, 2, 5, 2, 4,
2)` with its four agents.

## 4. Predictions fixed in advance

### 4.1 State counts and the undriven world

State and orbit counts are those of CAL-CEF-4 section 4.1:

| `(N, tau)` | States | Orbits |
|---|---:|---:|
| (3, 1) | 48 | 28 |
| (3, 2) | 216 | 114 |
| (3, 3) | 864 | 440 |
| (4, 1) | 512 | 120 |
| (4, 2) | 3072 | 612 |
| (5, 1) | 10240 | 660 |

`<N_R>` under `pi_c` is `Phi_{N,R}(1/(1 + b_c))`:

| `(N, R)` | at `(5, 2)` | at `(12, 3)` |
|---|---:|---:|
| (3, 2) | 37/18 | 249/169 |
| (4, 2) | 8/3 | 292/169 |
| (5, 2) | 10/3 | 337/169 |
| (3, 3) | 391/108 | 4695/2197 |

### 4.2 The GLOBAL agent

`H_global = Phi_{N,R}(2/(b_c + b_h + 2)) - Phi_{N,R}(1/(1 + b_c))` and
`J_global = K (b_c - b_h)/(b_c + b_h + 2)`, independent of `tau` and `v`:

| `(b_c, b_h)` | `(N, R)` | `H_global` | `J_global` | `Y_global` | `J_max` |
|---|---|---:|---:|---:|---:|
| (5, 2) | (3, 2) | 61/162 | 1 | 0.3765 | 3 |
| (5, 2) | (4, 2) | 17/27 | 2 | 0.3148 | 6 |
| (5, 2) | (5, 2) | 25/27 | 10/3 | 0.2778 | 10 |
| (5, 2) | (3, 3) | 2935/2916 | 1 | 1.0065 | 3 |
| (12, 3) | (3, 2) | 12708/48841 | 27/17 | 0.1638 | 27/4 |
| (12, 3) | (4, 2) | 20223/48841 | 54/17 | 0.1304 | 27/2 |
| (12, 3) | (5, 2) | 28512/48841 | 90/17 | 0.1103 | 45/2 |
| (12, 3) | (3, 3) | 7034562/10793861 | 27/17 | 0.4103 | 27/4 |

`eta_global = ln[2 b_c/(b_c + b_h)]/ln(b_c/b_h)` is `0.3893` at `(5, 2)` and
`0.3390` at `(12, 3)`. These values calibrate the apparatus; they are not
hypotheses.

### 4.3 The theory's predictions

Computed by `research/physics/xypher-agency-theory/cavity.py` (SHA-256
`743b9c03c8854e804622c3938643c1c5ffbbc1ab09e3c3735a877cb64776278f`) through
`predictions_cef5.py` (SHA-256
`a76eed4479154815d689be3db2b44e0579da8af4b14af2bce7b98f823e4d355a`). The
output `predictions_cef5.txt` has SHA-256
`27afb36675ece665b0b77dffc18d27a6bdfa2a42193d41bd9c26c3b1ac7a9fc5`. All three
were committed at `c06ffaebbf3fad3a07742549f93cb8a369db29c6`. Each line gives
the world as `(N, tau, b_c, b_h, v, R)`, the agent, and the theory's `E_th`:

```text
(3,2,5,2,v=1,R=2) LOCAL 1.390775
(3,2,5,2,v=1,R=2) AIM-1 1.408448
(3,2,5,2,v=1,R=2) AIM-2 1.240044
(3,2,5,2,v=4,R=2) LOCAL 1.347621
(3,2,5,2,v=4,R=2) AIM-1 1.380819
(3,2,5,2,v=4,R=2) AIM-2 1.225313
(3,2,5,2,v=16,R=2) LOCAL 1.256795
(3,2,5,2,v=16,R=2) AIM-1 1.315088
(3,2,5,2,v=16,R=2) AIM-2 1.195344
(4,2,5,2,v=1,R=2) LOCAL 1.755343
(4,2,5,2,v=1,R=2) AIM-1 1.789761
(4,2,5,2,v=1,R=2) AIM-2 1.474870
(4,2,5,2,v=4,R=2) LOCAL 1.672729
(4,2,5,2,v=4,R=2) AIM-1 1.737555
(4,2,5,2,v=4,R=2) AIM-2 1.447227
(4,2,5,2,v=16,R=2) LOCAL 1.500032
(4,2,5,2,v=16,R=2) AIM-1 1.614504
(4,2,5,2,v=16,R=2) AIM-2 1.391200
(3,1,5,2,v=1,R=2) LOCAL 1.395191
(3,1,5,2,v=1,R=2) AIM-1 1.417151
(4,1,5,2,v=1,R=2) LOCAL 1.763832
(4,1,5,2,v=1,R=2) AIM-1 1.805372
(5,1,5,2,v=1,R=2) LOCAL 2.112110
(5,1,5,2,v=1,R=2) AIM-1 2.170470
(3,3,5,2,v=1,R=3) LOCAL 1.342026
(3,3,5,2,v=1,R=3) AIM-1 1.358722
(3,3,5,2,v=1,R=3) AIM-2 1.171789
(3,3,5,2,v=1,R=3) AIM-3 1.139421
(3,2,12,3,v=1,R=2) LOCAL 1.445631
(3,2,12,3,v=1,R=2) AIM-1 1.455677
(3,2,12,3,v=1,R=2) AIM-2 1.269867
(3,2,12,3,v=4,R=2) LOCAL 1.430886
(3,2,12,3,v=4,R=2) AIM-1 1.447266
(3,2,12,3,v=4,R=2) AIM-2 1.262955
(3,2,12,3,v=16,R=2) LOCAL 1.384205
(3,2,12,3,v=16,R=2) AIM-1 1.418012
(3,2,12,3,v=16,R=2) AIM-2 1.248591
(4,2,12,3,v=1,R=2) LOCAL 1.873447
(4,2,12,3,v=1,R=2) AIM-1 1.894422
(4,2,12,3,v=1,R=2) AIM-2 1.542982
(4,2,12,3,v=4,R=2) LOCAL 1.844688
(4,2,12,3,v=4,R=2) AIM-1 1.878159
(4,2,12,3,v=4,R=2) AIM-2 1.528385
(4,2,12,3,v=16,R=2) LOCAL 1.754138
(4,2,12,3,v=16,R=2) AIM-1 1.822746
(4,2,12,3,v=16,R=2) AIM-2 1.501549
(3,1,12,3,v=1,R=2) LOCAL 1.446938
(3,1,12,3,v=1,R=2) AIM-1 1.458810
(4,1,12,3,v=1,R=2) LOCAL 1.875998
(4,1,12,3,v=1,R=2) AIM-1 1.900107
(5,1,12,3,v=1,R=2) LOCAL 2.289413
(5,1,12,3,v=1,R=2) AIM-1 2.325015
(3,3,12,3,v=1,R=3) LOCAL 1.416430
(3,3,12,3,v=1,R=3) AIM-1 1.427271
(3,3,12,3,v=1,R=3) AIM-2 1.204813
(3,3,12,3,v=1,R=3) AIM-3 1.166972
```

## 5. Gates

As CAL-CEF-4 section 5 (G01--G11, with the same check order, classification,
and failure reporting), with these changes:

- G01 check 2: `N_R(G, x)` by depth-first walk enumeration equals
  `e_x^T (I + A_G)^R 1` for the world's own `R`, for every configuration and
  island.
- G02 check 2: STEP presence as section 1.3 of CAL-CEF-4; REPLAN exactly at
  `t = 0` to every other route.
- G03 check 2: at the source state, forward : reverse rates are in the ratio
  `1 : 1` for STEP and for REPLAN (their common value `v` is tested only by
  G10 check 3), `1 : b_c` for WEATHER build : wear, and `1 : b_h` for FUEL
  build : return.
- G09 check 1: every LOCAL and AIM agent has `J < J_max` (a theorem at both
  pairs, section 2), `s*(J) > 0`, `J_warm(s*) = J`, and `r_(s*) = rho`
  exactly. G09 check 3 (new): for every agent, `Phi_{N,R}(rho)` equals
  `sum_G rho^|G| (1 - rho)^(K - |G|) N_R(G, 0)` exactly.
- G10 check 3: every rate is computed from the state, bridge, channel class,
  store ratio, the agent's permission set, and the declared speed `v`, and
  equals the rule of section 1. No rate reads `N_R`, `H`, `J`, `Y`, `E`,
  `eta`, or any theory value.
- G11: check 1 compares `<N_R>` under `pi_c` with section 4.1; check 2
  compares `H_global` and `J_global` with section 4.2 exactly, and the
  four-decimal rounding of `H_global / J_global` with `Y_global`; check 3
  compares the four-decimal rounding of `eta_global` with section 4.2.
- G12 frozen theory: (1) the apparatus's embedded copy of
  `predictions_cef5.txt` has SHA-256
  `27afb36675ece665b0b77dffc18d27a6bdfa2a42193d41bd9c26c3b1ac7a9fc5`,
  computed by the apparatus; (2) its 56 lines parse, each `E_th` a positive
  exact rational written with exactly six decimals; (3) the set of
  `(world, agent)` keys equals the set of LOCAL and AIM steady states of
  section 3, each key once. G12 failures are reported as table-level.

Classification as CAL-CEF-4, except that `eta < 1` in G08 check 4 is a theorem
given checks 1 and 3; G09 check 3 and G12 are independent tests.

Gates run G01 to G12. Controls run C01 to C09.

## 6. Apparatus

A standalone dependency-free Rust crate at
`research/physics/xypher-theory-of-aim-proof/`, with a prospective Futuruna
declaration `xypher.runa`. It extends the CAL-CEF-4 apparatus with the speed
`v`, the radius `R`, the radius-`R` closed form `Phi_{N,R}` (exact walk
enumeration), the frozen theory table, and the hypotheses below. The four
arithmetic modules `src/big.rs`, `src/rat.rs`, `src/logs.rs`, `src/solve.rs`
are unchanged copies of the CAL-CEF-3 and CAL-CEF-4 apparatus files (same
SHA-256). The theory itself is not part of the apparatus: the apparatus
reads only the frozen decimals of section 4.3.

The report has:
- one line per gate;
- one line per control, with its observed and expected first failure;
- one line per world and agent, giving states, orbits, `<N_R>_p`,
  `<N_R>_(pi_c)`, `H`, `J`, `Y`, `rho`, `s*`, `E` (exact reduced rationals
  with four-decimal roundings), `E_th`, and the exact ratio `E / E_th` to six
  decimals, plus the certified enclosures of `eta` and `Y_bound`;
- one line per hypothesis, with `SUPPORTED` or `REFUTED` and every failing
  item;
- `OVERALL PASS` or `OVERALL FAIL`, for the gates and controls only.

The frozen commands are `cargo test --release --locked` and then
`cargo run --release --locked --quiet`. Exact elimination takes about 13 to 16
minutes per full run (612-orbit worlds about 18 to 31 s per steady state,
growing with `v`; 660-orbit worlds 25 to 33 s); the test suite solves only the
primary world (114 orbits) and its controls.

## 7. Hypotheses

Each hypothesis is decided by exact rational comparison and reported as
`SUPPORTED` or `REFUTED`, listing every item at which it fails. It is
`SUPPORTED` only if it holds at every item it names. An undefined value counts
as a failure. A refutation is a result.

- **H1 The theory predicts every edge to within one percent.** For each of the
  56 LOCAL and AIM steady states, `(99/100) E_th <= E <= (101/100) E_th`.
- **H2 Faster travellers gain less from aim.** In each of the four speed
  ladders (worlds 1--3 and 4--6 at each pair) and for each of LOCAL, AIM-1,
  and AIM-2, `E` strictly decreases from `v = 1` to `v = 4` to `v = 16`:
  twelve sequences.
- **H3 Blind foresight never pays.** In each of the fourteen worlds with
  `tau >= 2` (worlds 1--6 and 10 at each pair), `E_AIM-k` strictly decreases
  with `k = 1, ..., tau`, and `E_AIM-tau < E_LOCAL`.
- **H4 The nearest aim leads.** `E_AIM-1 > E_LOCAL` in all 20 worlds.
- **H5 The simple law.** In each of the six worlds with `tau = 1` (worlds 7--9
  at each pair), `E_LOCAL <= L(N, rho_LOCAL) <= (103/100) E_LOCAL`. The law
  over-predicts the exact edge, but by no more than 3%.

H2--H4 are the theory's own qualitative predictions, and the frozen values of
section 4.3 satisfy all of them. A failure of H1 with H2--H4 holding means the
theory has the right shape but the wrong size. The theory's own margins for
H2 and H4 fall as low as 0.55% at `(12, 3)`, below H1's tolerance. A failure of
H2--H4 at an item where H1 holds therefore means only that the theory's error
there exceeds its margin. It shows the shape is wrong where H1 also fails or
where the theory's margin exceeds `101/99 - 1` (about 2.02%).

Permission-set coincidences are those of CAL-CEF-4 section 7, independent of
`v` and `R`. AIM-1 is a subset of LOCAL. The two are equal on 2 of 27 classes
of `(3, 2)` and 12 of 108 of `(3, 3)`, and on none for `N >= 4` or `tau = 1`.
AIM-`tau` equals LOCAL on 2 of 27 and 16 of 108.

## 8. Preregistered falsifying controls

As CAL-CEF-4 section 8, on the primary witness `(3, 2, 5, 2, 4, 2)` only, in
the agent order of section 3. Under a control, no quantity that divides by
`J`, `sigma`, or `H_warm` is evaluated after the first failing check.

| Control | Mutation | Expected first failure |
|---|---|---|
| C01 one-way fuel | Delete every FUEL return. | G03 check 1, LOCAL, configuration none |
| C02 free protection | WEATHER never wears a bridge the held route uses. | G02 check 3, LOCAL, configuration `{{0,1}}` |
| C03 equal stores | `b_h = b_c = 5`. | G08 check 1, LOCAL, world-level (`J = 0`) |
| C04 biased step | Forward STEP rate `2v`. | G03 check 2, LOCAL, configuration none |
| C05 aim off by one | AIM-`k` aims at the route steps within `k + 1` moves. | G02 check 4, AIM-1, configuration none |
| C06 wrong calibration | G07 checks GLOBAL against `q = 1/b_c`. | G07 check 1, GLOBAL, configuration none |
| C07 wrong rival | The rival's inverse uses `b_h` in place of `b_h + 1`. | G09 check 1, LOCAL, world-level |
| C08 slow replanning | REPLAN runs at rate 1 instead of `v`. | G10 check 3, LOCAL, configuration none |
| C09 corrupted theory | In the embedded table, the first `E_th`, 1.390775, becomes 1.390776. | G12 check 1, table-level (no world or agent) |

C08 keeps STEP and REPLAN in the ratio `1 : 1`, so G03, which tests ratios
only, passes, and the world stays in a valid steady state. Only the rule check
of G10 sees that REPLAN ignores the declared speed. C07 must fail because `J < J_max` is a theorem at
`(5, 2)` and the mutated inverse satisfies
`J_warm(s_w(J)) = J K (b_c - b_h)/[K (b_c - b_h) + J] < J` for `J > 0`.

## 9. Design-time disclosure and blinding

- The theory was built after the CAL-CEF-4 run, using its exact results at
  `(3, 2)`, `(8, 2)`, and `(6, 3)` with `v = 1` and `R = 2`. It has no fitted
  parameter: its only approximation is the cavity mean field of the Cavity
  Theory note, section 1.
- Every CAL-CEF-5 world differs from every CAL-CEF-4 world in its pair of
  stores. Worlds 2, 3, 5, 6, and 10 also differ in speed or radius.
- Before freezing, the designer computed:
  - the theory's predictions (section 4.3), in floating point;
  - the closed forms of section 4;
  - the theory at other speeds, radii, replanning rates, and asymmetric step
    rates, summarised qualitatively, without numbers, in the Cavity Theory
    note, section 4.
- The direction and the 3% tolerance of H5 were set from the CAL-CEF-4 exact
  LOCAL edges. There `L` lies above every edge, by 0.44% to 2.96% (0.44% to
  2.83% at `tau = 1`).
- No exact steady state of any LOCAL or AIM agent was computed at any
  CAL-CEF-5 world, by the designer or the referee, before the freeze.
- The pre-freeze referee may solve LOCAL and AIM agents exactly only at pairs
  outside `(5, 2)` and `(12, 3)`, and outside every CAL-CEF-4 pair. It may do
  so only to check the design for degeneracies, and reports no value, no
  accuracy of the theory, and no direction.

## 10. What a pass establishes

A pass of every gate and control establishes that the apparatus builds the
declared worlds exactly, solves their steady states exactly, measures `H`,
`J`, `Y`, `rho`, the rival, and `E` exactly, and reads the frozen theory
correctly. The hypotheses then stand or fall on their own lines.

Supported hypotheses would establish, within these worlds:
- H1: a mean-field theory with no fitted parameter, built from CAL-CEF-4 data
  and frozen before this experiment, gives each of the 56 LOCAL and AIM edges
  of these 20 worlds within 1% of its frozen value;
- H2: in the four speed ladders, the edges of LOCAL, AIM-1, and AIM-2 fall as
  the traveller's speed rises from 1 to 4 to 16;
- H3: in the fourteen worlds with `tau >= 2`, each further step of aim lowers
  the edge, and whole-route aim lies below local fuel;
- H4: AIM-1 has a larger edge than LOCAL in all 20 worlds;
- H5: at `tau = 1`, the closed-form law `L` over-predicts the edge of local
  fuel by at most 3%.

Each refuted hypothesis establishes that its claim fails at the items the
report lists; it does not establish the reverse.

Outside this boundary: worlds larger than five islands (where only the theory
reaches); routes chosen by what they open (CAL-CEF-6); and several
travellers.

## 11. Xypher review before execution

As CAL-CEF-4 section 11. The speed `v` and radius `R` are declared
experimental inputs, enumerated in section 3. The theory's numbers are read
only by G12, which checks the frozen table itself, and by the hypothesis lines
of the report; never by a rate or by any gate's decision about a world.

## 12. Review and freeze

As CAL-CEF-4 section 12:
- Before execution, the exact commits of this boundary and of the apparatus
  are reviewed for mathematical correspondence, adversarial checker
  behaviour, and Futuruna/Xypher semantics.
- Only non-executing checks (`runa check`, `cargo check`,
  `cargo test --no-run`, `cargo fmt --check`, `cargo clippy`) run on the
  apparatus before approval.
- The approved commits are pushed, and the frozen commands run once.
- The result is recorded in [[Theory of Aim Result]], with provenance and an
  independent check written from this boundary alone.

## 13. Related work

As CAL-CEF-4 section 13. Mean-field and cavity methods are standard in
statistical physics. The pair approximation and its cavity formulation are
the closest antecedents of section 1 item 3 of the Cavity Theory note. Their
use here is to predict an agency observable before it is computed exactly.
