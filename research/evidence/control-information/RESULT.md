---
title: Control Information Thermodynamics Result
aliases:
  - CAL-CEF-2 Result
  - Foresight Notebook Result
tags:
  - domain/physics
  - type/result
  - topic/thermodynamics
  - topic/xypher
  - topic/information-thermodynamics
domain: Physics
type: result
status: exact-family-passed
td: td-ac9ae6
created: 2026-10-10
updated: 2026-10-10
related:
  - "[[Control Information Thermodynamics Boundary]]"
  - "[[Causal Entropic Thermodynamics Result]]"
  - "[[Xypher Operational Thermodynamics Boundary]]"
---

# Control information thermodynamics result (CAL-CEF-2)

## 0. Verdict

**The frozen CAL-CEF-2 construction passed.** All fourteen gates passed on the
primary witness `(N, tau, b, W) = (3, 2, 2, 2)`. All nine preregistered
controls failed first at exactly the preregistered gate, check, and
configuration, and C09 at its preregistered value. Every case of the
seven-case family passed every applicable gate and reproduced its frozen state
counts, partition function, means, erasure ratio, blank probability, and list
of configurations with `K = 1`.

```text
OVERALL PASS
```

| Question (boundary section 0) | Answer |
|---|---|
| Does the instrumented world keep the reservoir's temperature? | **Yes**: T1--T5 across the seven-case family (reservoir lift explicit for `N = 3`), with bridge, feedback, and erasure traffic governed by the same `b`. |
| Does the instrument distort the world it measures? | **No**: the configuration law stays `N_tau(G) b^(-|G|)/Z`, and at equilibrium plan and notebook are independent (correct-record fraction `1/3` in the primary). |
| What is a correct record of the held plan's destination worth? | **`alpha ln 2` per bit**: the shuffled-deck gap is `alpha I(G)` under the Crystal's law and `alpha I*(G)` under the capacity witness, and the feedback exchange constant is `K(G, y) = N_tau(G)/[n_y(G) b]` at every configuration and destination. |
| One bit, one packet? | **Yes, exactly**: `K = 1` at `{0,1}` and `{0,2}` in the primary, where the record carries one bit and the reservoir doubles per packet. One trit trades evenly for one packet at `b = 3` in `(3,2,3,2)` and `(3,1,3,1)`. |
| What does wiping the notebook cost? | **`alpha ln A`**: erasure ratio `b/A = 2/3` for a three-symbol notebook at `b = 2`, and exactly `1` at `b = 3`. |
| Does the measure-harvest-erase cycle produce net work at equilibrium? | **No**: net stockpile and notebook currents are exactly zero. |

No fitting, sampling, random seed, simulation length, tolerance, or
floating-point acceptance path was involved.

## 1. The result in plain language

The traveller of CAL-CEF-1 still lives on its archipelago, holds one short
plan, walks it, replans at home, and builds or dismantles bridges where it
stands. Now it carries a notebook. At home it can write down where its held
plan ends. It also keeps a stockpile: a stack of timber packets beside the
shore, each worth one packet of energy and carrying no arrangements of its own.

With a correct note in hand, the traveller can let go of its plan, keeping the
note, while one packet moves from the reservoir onto the stockpile. The reverse
move spends a stockpile packet to commit again to a plan that ends where the
note says. Wiping a written note back to blank releases a stockpile packet into
the reservoir; scribbling on a blank page draws one onto the stockpile.

Every move is as likely as its reverse at the microscopic level, and no rule
reads how much information the note carries. The world settles exactly where
counting says it must. A correct note trades for a stored packet at odds
`N_tau/(n_y b)`: when the note says which of two equally likely destinations
the walk will reach, and the reservoir doubles per packet, the odds are
exactly even. One bit of foresight is worth one packet of timber.

## 2. What was frozen before execution

| Object | Revision |
|---|---|
| Boundary (preregistration) | `79d344024eb0169d4b458213936fc5cec8c476cd` (2026-10-10T13:22:19+02:00) |
| Boundary SHA-256 | `90c6c7598cfa1d75a7113be98187c82c90b4c5676c91d51f272dd557a9e9e9db` |
| Apparatus commit (approved) | `6b517442cc35d4721cdee72ad2998b31a825934a` (2026-10-10T13:44:38+02:00) |
| Apparatus Git tree | `11b396bbccdffedc539860ea5bfc6616fe790bcf` |

Apparatus file digests (SHA-256):

```text
bb9c7cc338eede417bb1ce8f12f528bfc7a578e96a44632e7643e1ca2bfa5152  Cargo.lock
81469a8e3488bfb6a1d0c6603c8126ccd7a4e70f884203049069d7efd1021562  Cargo.toml
99ee9737a1086f8091e55c3a41d6294626bdc996e09152e52988eb7bc041a65c  README.md
80d17506edfe0904431192928d6a97cc3f1d85c01f502fe590180941ba94ac0b  src/gates.rs
349a923165d457125ba521457ce11ceb5257accd24465fcdf9770592bca4c11f  src/lib.rs
edab2415f176a1411099e674ff1312deed722226861f12f077e348d2a9ebb4e9  src/main.rs
3cf069f22eb8a8c2ef6155df688f827449ea511d79e720455f676bfe3f1cf399  src/model.rs
f323d84c2785fa8b3d3d1f67ccfe697600904500c459f127f1bae88c2d6b6d90  src/primes.rs
069a7e890ff56e9d0e8f47feb998169c1799d1067e862b8c20e20d1e53c0b710  src/ratio.rs
71bf40effcf77fc36f81740cd267d1615319e671cf54edff62971424f0cebc61  tests/source_contract.rs
a176ef0dad76f0212d538576155831df0d0a701a8e23a57bedbac8bb43979cc9  xypher.runa
```

### 2.1 Review chronology

1. **Boundary draft** was reviewed by an independent referee, who rebuilt the
   generator of every family case and every control in a separate throwaway
   implementation and reproduced every prediction and expected first failure.
   The referee required sixteen changes (F1--F16), chiefly: the stockpile
   counts as system energy, `U = lambda (|G| + w)`, so that one energy function
   governs local detailed balance; a reset is defined as the many-to-one ERASE,
   distinct from the reversible UNMEASURE; the capacity value `alpha I*(G)` is
   tested separately from the Crystal-law value `alpha I(G)`; the equilibrium
   independence of plan and notebook is checked; exact entropy arithmetic over
   `ln(prime)` and the stationary-law propagation are specified; and every check
   is classified as an independent test or a consistency check.
2. **Boundary round 2** confirmed F1--F16 and required two clarifications (the
   `W` used by C09's structural checks; the scope of the referee disclosure),
   applied before the freeze at `79d34402`.
3. **Apparatus `6b517442`** was built under embargo and approved without
   execution by three independent static reviewers: mathematical
   correspondence (every gate check and embedded constant traced against the
   boundary, digit by digit), adversarial checker behaviour (no path to a false
   PASS or a wrong-reason control result; estimated size and runtime), and
   Futuruna/Xypher semantics (the declaration and code faithful to section 3;
   no hazard reads `N_tau`, `n_y`, `I`, `I*`, `K`, THAIM, or Xi). It was
   pushed and confirmed equal to the fetched remote before the run.

Only non-executing checks ran before approval: `cargo check`,
`cargo test --no-run`, `cargo fmt --check`, `cargo clippy`, `runa check`,
`runa fmt --check`. Design-time calculations and the pre-freeze referee's
throwaway replica are disclosed in boundary section 5.4.

### 2.2 Non-blocking review findings carried into this record

These did not block approval; changing the approved commit would have voided
the reviews, so they are recorded here instead.

- The apparatus README lists `runa fmt --check` among permitted pre-approval
  checks, one more than boundary section 14 names, and describes the embargo
  without citing the design-time disclosure of boundary section 5.4.
- A comment in `xypher.runa` says the verifier compares `K` by
  cross-multiplication with the declaration; the Rust verifier does not read
  `xypher.runa` and checks `K` against its own frozen constants.
- G10 check 5 additionally checks the section 5.2 `K = 1` lists of every
  family case; G09 check 1, G03 check 3, and G04 check 4 are slightly stronger
  than the boundary wording; G14 is reported as three numbered sub-checks.
- If a broken primary failed G02 check 2(d) at `{0,1}` together with an
  earlier-numbered sub-check at a later configuration, the label would name
  the later configuration. No control targets G02 check 2, and the run did not
  exercise this path.

## 3. Execution

```text
time:        2026-10-10T12:00:42Z (test), 2026-10-10T12:00:52Z (run)
toolchain:   rustc 1.94.0 (4a4ef493e 2026-03-02); cargo 1.94.0 (85eff7c80 2026-01-15)
host:        aarch64-apple-darwin
HEAD:        6b517442cc35d4721cdee72ad2998b31a825934a
commands:    cargo test --locked          (exit 0; source contract 8.38 s)
             cargo run --locked --quiet   (exit 0; about 10 s)
report:      41 lines, SHA-256 ca303e0db354dbd17d71381efc6fb0a4a814fabfba041a8ce5185263217be7e9
```

The source-contract test (14 gates, 9 controls, G01 to G14 order, success)
passed. The frozen commands were executed once.

### 3.1 Complete report

```text
XYPHER_CONTROL_INFORMATION_PROOF CAL-CEF-2 primary=(3,2,2,2)
G01 PASS Crystal and command grounding
G02 PASS executable registers -- plan read at z36[G={0,1} p=000 t=0 y=blank w=0] vs z72[G={0,1} p=001 t=0 y=blank w=0]
G03 PASS reciprocal generator -- each diagonal is -(exit rate) and each exit rate equals the rate counted independently from the section 3.3 rules
G04 PASS pathwise accounting
G05 PASS reservoir lumpability
G06 PASS one temperature, three traffics
G07 PASS connectivity and equilibrium -- propagated along a breadth-first spanning tree from z0
G08 PASS undistorted world
G09 PASS control channel
G10 PASS feedback exchange
G11 PASS shuffled deck
G12 PASS Landauer erasure
G13 PASS no free lunch
G14 PASS Xypher slot reading -- hazards are computed from HazardInputs [SourceReservoirEnergy, DestinationReservoirEnergy, ReservoirBase], a type with no N_tau, n_y, I, I*, K, THAIM, or Xi field; every system hazard equals the independent b^(E_R(z'))/1 rule and every micro hazard is kappa
C01 PASS one-way-harvest observed G03 check 1 G03_RECIPROCAL_SUPPORT at none expected G03 check 1 G03_RECIPROCAL_SUPPORT at none -- complete-state HARVEST 56 -> 88: no positive COMMIT reverse
C02 PASS free-reset observed G02 check 5 G02_ERASURE_SEMANTICS at none expected G02 check 5 G02_ERASURE_SEMANTICS at none -- SCRIBBLE from z0[G=none p=000 t=0 y=blank w=0] arrives at y=0 w=0
C03 PASS blind-measurement observed G02 check 3 G02_MEASURE_SEMANTICS at {0,1} expected G02 check 3 G02_MEASURE_SEMANTICS at {0,1} -- MEASURE from z72[G={0,1} p=001 t=0 y=blank w=0] writes y=0 with d(p)=1
C04 PASS cheating-harvest observed G02 check 4 G02_FEEDBACK_SEMANTICS at none expected G02 check 4 G02_FEEDBACK_SEMANTICS at none -- HARVEST from z6[G=none p=000 t=0 y=1 w=0] (d(p)=0) to y=1 w=1
C05 PASS telepathic-demon observed G02 check 4 G02_FEEDBACK_SEMANTICS at none expected G02 check 4 G02_FEEDBACK_SEMANTICS at none -- HARVEST from z0[G=none p=000 t=0 y=blank w=0] (d(p)=0) to y=blank w=1
C06 PASS directed-kinetic-mutation observed G03 check 2 G03_HAZARD_VALUES at none expected G03 check 2 G03_HAZARD_VALUES at none -- complete-state HARVEST 56 <-> 88: hazards 2 / 1, expected 1 / 1
C07 PASS hoarding-demon observed G02 check 4 G02_FEEDBACK_SEMANTICS at {0,1} expected G02 check 4 G02_FEEDBACK_SEMANTICS at {0,1} -- HARVEST from z39[G={0,1} p=000 t=0 y=0 w=0] reaches 1 of 4 plans
C08 PASS unequal-shuffle observed G11 check 2 G11_SHUFFLED_MARGINALS at none expected G11 check 2 G11_SHUFFLED_MARGINALS at none -- w=0: B_G record marginal [1/3,1/3,1/3] != A_G record marginal [1,0,0]
C09 PASS stockpile-slip observed G08 check 3 G08_STOCKPILE_MARGINAL at global expected G08 check 3 G08_STOCKPILE_MARGINAL at global value 11/15 -- mean stockpile 11/15; section 5: 4/7
WITNESS none N_2=1 n=1,0,0 |D|=1 K=1/2,-,- exp[N I]=1 exp[|D| I*]=1
WITNESS {0,1} N_2=4 n=2,2,0 |D|=2 K=1,1,- exp[N I]=16 exp[|D| I*]=4
WITNESS {0,2} N_2=4 n=2,0,2 |D|=2 K=1,-,1 exp[N I]=16 exp[|D| I*]=4
WITNESS {0,1},{0,2} N_2=7 n=3,2,2 |D|=3 K=7/6,7/4,7/4 exp[N I]=823543/432 exp[|D| I*]=27
WITNESS {1,2} N_2=1 n=1,0,0 |D|=1 K=1/2,-,- exp[N I]=1 exp[|D| I*]=1
WITNESS {0,1},{1,2} N_2=5 n=2,2,1 |D|=3 K=5/4,5/4,5/2 exp[N I]=3125/16 exp[|D| I*]=27
WITNESS {0,2},{1,2} N_2=5 n=2,1,2 |D|=3 K=5/4,5/2,5/4 exp[N I]=3125/16 exp[|D| I*]=27
WITNESS {0,1},{0,2},{1,2} N_2=9 n=3,3,3 |D|=3 K=3/2,3/2,3/2 exp[N I]=19683 exp[|D| I*]=27
INSTRUMENT (3,2,2,2) erasure_ratio=2/3 mean_stockpile=4/7 blank=1/4 correct_fraction=1/3
FAMILY (3,1,2,2) configurations=8 system=384 complete=2520 Z=45/8 mean_bridges=19/15 erasure=2/3 mean_stockpile=4/7 blank=1/4 K1=[{0,1};{0,2};{0,1},{1,2};{0,2},{1,2}] gates PASS=14 FAIL=0 N/A=0
FAMILY (3,2,2,2) configurations=8 system=1296 complete=7308 Z=87/8 mean_bridges=131/87 erasure=2/3 mean_stockpile=4/7 blank=1/4 K1=[{0,1};{0,2}] gates PASS=14 FAIL=0 N/A=0
FAMILY (3,3,2,2) configurations=8 system=4128 complete=20720 Z=185/8 mean_bridges=313/185 erasure=2/3 mean_stockpile=4/7 blank=1/4 K1=[{0,1};{0,2}] gates PASS=14 FAIL=0 N/A=0
FAMILY (3,2,3,2) configurations=8 system=1296 complete=26208 Z=56/9 mean_bridges=5/4 erasure=1 mean_stockpile=5/13 blank=1/4 K1=[{0,1},{0,2},{1,2}] gates PASS=14 FAIL=0 N/A=0
FAMILY (3,1,3,1) configurations=8 system=256 complete=3072 Z=32/9 mean_bridges=1 erasure=1 mean_stockpile=1/4 blank=1/4 K1=[{0,1},{0,2};{0,1},{0,2},{1,2}] gates PASS=14 FAIL=0 N/A=0
FAMILY (4,1,2,1) configurations=64 system=3200 complete=43740 Z=729/32 mean_bridges=7/3 erasure=1/2 mean_stockpile=1/3 blank=1/5 K1=[{0,1};{0,2};{0,3};{0,1},{1,2};{0,2},{1,2};{0,3},{1,2};{0,1},{1,3};{0,2},{1,3};{0,3},{1,3};{0,1},{1,2},{1,3};{0,2},{1,2},{1,3};{0,3},{1,2},{1,3};{0,1},{2,3};{0,2},{2,3};{0,3},{2,3};{0,1},{1,2},{2,3};{0,2},{1,2},{2,3};{0,3},{1,2},{2,3};{0,1},{1,3},{2,3};{0,2},{1,3},{2,3};{0,3},{1,3},{2,3};{0,1},{1,2},{1,3},{2,3};{0,2},{1,2},{1,3},{2,3};{0,3},{1,2},{1,3},{2,3}] gates PASS=13 FAIL=0 N/A=1(G05)
FAMILY (4,2,2,1) configurations=64 system=13440 complete=153090 Z=1701/32 mean_bridges=55/21 erasure=1/2 mean_stockpile=1/3 blank=1/5 K1=[{0,1};{0,2};{0,3};{0,3},{1,2};{0,2},{1,3};{0,1},{2,3}] gates PASS=13 FAIL=0 N/A=1(G05)
OVERALL PASS
```

Every N/A is one the boundary permits: G05 at `N = 4` (reservoir lift not
enumerated).

## 4. Gate and control results

### 4.1 Gates on the primary witness

| Gate | Result |
|---|---|
| G01 Crystal and command grounding | Pass. Depth-first and matrix-power plan counts agree; REPLAN uniform over `Pi_tau(G)`; `sum_y n_y = N_tau`; every `(G, y, w)` slice complete. |
| G02 executable registers | Pass. Every channel's semantics as declared; plan contents read: `z36` (plan `000`) and `z72` (plan `001`) at `{0,1}` have different DISMANTLE sets. |
| G03 reciprocal generator | Pass at complete-state and system-state level; exit rates equal those counted independently from the section 3.3 rules. |
| G04 pathwise accounting | Pass. `U + E_R` conserved; packets move only between the declared stores; every ERASE pays one packet into the reservoir; zero external work. |
| G05 reservoir lumpability | Pass on the explicit 7,308-state lift. |
| G06 one temperature, three traffics | Pass: bridge, feedback, and erasure traffic each read the reservoir's `b`. |
| G07 connectivity and equilibrium | Pass: connected; the law propagated along a breadth-first spanning tree equals `b^(-(|G|+w))`; all flux products equal. |
| G08 undistorted world | Pass: configuration law `N_2(G) 2^(-|G|)/(87/8)`, mean bridges `131/87`; notebook uniform, blank `1/4`; stockpile `pi(w) ~ 2^(-w)`, mean `4/7`; plan and notebook independent at every `(G, w)`, correct fraction `1/3`; counts reproduced. |
| G09 control channel | Pass: `n_y`, `|D|`, capacity witness plans, and `exp[N I]` as section 5.1. |
| G10 feedback exchange | Pass: HARVEST/COMMIT connect exactly `C` and `L`; `pi(L)/pi(C) = kbar(C -> L)/kbar(L -> C) = N/(n_y b)` at every configuration, destination, and stockpile level; `K = 1` at `{0,1}` and `{0,2}`. |
| G11 shuffled deck | Pass: `A_G` uniform over correct pairs; `B_G` equal marginals; energy blind to the notebook; `exp{N[S(B)-S(A)]} = N^N/prod n_y^(n_y) = prod (bK)^(n_y)`; capacity pair `|D|^|D|`. |
| G12 Landauer erasure | Pass: one ERASE target per written state, `A` SCRIBBLE targets per blank state; `pi(B)/pi(R) = 2/3`. |
| G13 no free lunch | Pass: net stockpile and notebook currents exactly zero. |
| G14 Xypher slot reading | Pass; hazards read only the two reservoir energies and `b`. |

### 4.2 Controls

| Control | Observed first failure | Matched |
|---|---|---|
| C01 one-way harvest | G03 check 1 at none: HARVEST without a COMMIT reverse | yes |
| C02 free reset | G02 check 5 at none: SCRIBBLE writes without lifting a packet | yes |
| C03 blind measurement | G02 check 3 at `{0,1}`: plan `001` recorded as `0` | yes |
| C04 cheating harvest | G02 check 4 at none: HARVEST from a wrong record | yes |
| C05 telepathic demon | G02 check 4 at none: HARVEST from a blank notebook | yes |
| C06 directed kinetic mutation | G03 check 2 at none: HARVEST hazards 2 / 1 | yes |
| C07 hoarding demon | G02 check 4 at `{0,1}`: HARVEST reaches 1 of 4 plans | yes |
| C08 unequal shuffle | G11 check 2 at none: record marginal `[1/3,1/3,1/3]` against `[1,0,0]` | yes |
| C09 stockpile slip | G08 check 3, global: mean stockpile `11/15` against `4/7` | yes |

## 5. What this establishes

Within the declared finite model (boundary section 10):

1. A digital world in which a base Praxion holds a tau-step plan, a notebook
   records where that plan ends, a feedback move trades a correct record for a
   stored timber packet, and a reset of the notebook pays a packet into the
   reservoir, satisfies T1--T5 across the seven-case family, with `alpha`
   fixed by an independently declared reservoir. The same reservoir governs
   bridge, feedback, and erasure traffic.
2. The instrument leaves the CAL-CEF-1 configuration law intact, and the
   equilibrium world carries no plan-notebook information.
3. A correct record of the held plan's destination is worth `alpha ln 2` of
   free energy per bit: as the free-energy gap to a shuffled pairing of equal
   energy, under the Crystal's uniform plan law (`alpha I(G)`) and under the
   capacity witness (`alpha I*(G)`, the control value of Intelligence as
   Physical Units), and as the equilibrium exchange constant
   `K(G, y) = N_tau(G)/[n_y(G) b]` of the feedback move. With a reservoir that
   doubles per packet, one bit of foresight trades evenly for one packet.
4. Erasing, through ERASE, a notebook uniform over its alphabet and
   uncorrelated with the plan costs `alpha ln A` of free energy; every net
   current vanishes at equilibrium.

Section 7 of the boundary classifies the checks: the exchange constants,
shuffled-deck gaps, erasure ratio, and zero currents are consequences of the
stationary law once G03, G05, and G07 hold. The construction's content is the
instrumented world itself: a notebook, a feedback move, and an erasure that
realize the exchange at the temperature of an independently counted
reservoir, with every channel local, reversible, and blind to the information
it trades.

## 6. Scope

The cycle runs at equilibrium, so no net work is extracted. The record is of
the held plan's destination, not of a realized trajectory. Where destinations
are unequally reachable, the feedback exchange averages `I(G)`, below the
capacity value `I*(G)`. Contact between instrumented worlds (T6), noisy or
partial records, controllers that act on records alone, learning, and any
statement about THAIM lie outside this boundary.

## 7. Next boundary

Drive the cycle: couple the stockpile to a load and the notebook to a stream of
blank pages, and measure the steady-state rate at which committed futures
convert free-energy flow into stored work, with the efficiency bounded by the
second law. That is the first exact test of an agency bound in this program.
It must not reuse CAL-CEF-2 results to rescue its own gates.
