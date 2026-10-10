---
title: Causal Entropic Thermodynamics Result
aliases:
  - CAL-CEF-1 Result
  - Committed-Future Thermodynamic Graph Result
tags:
  - domain/physics
  - type/result
  - topic/thermodynamics
  - topic/xypher
  - topic/causal-entropic-force
domain: Physics
type: result
status: exact-family-passed
td: td-ab39db
created: 2026-10-08
updated: 2026-10-08
related:
  - "[[Causal Entropic Thermodynamics Boundary]]"
  - "[[Xypher Operational Thermodynamics Boundary]]"
  - "[[Xypher Operational Thermodynamics Result]]"
---

# Causal entropic thermodynamics result (CAL-CEF-1)

## 0. Verdict

**The frozen CAL-CEF-1 construction passed.** All twelve gates passed on the
primary witness `(N, tau, b) = (3, 2, 2)`. All eight preregistered controls
failed first at exactly the preregistered gate, check, and, where named,
configuration. Every case of the eight-case family passed every applicable
gate and reproduced its frozen state counts, partition function, and
equilibrium mean bridge count.

```text
OVERALL PASS
```

| Question (boundary section 0) | Answer |
|---|---|
| Is the walked-plan world an operational thermodynamic graph system? | **Yes**: T1--T5 across the eight-case family (reservoir lift explicit for `N = 3`); T6 for the identical and heterogeneous contact pairs. |
| Is its configuration law `N_tau(G) exp[-U(G)/alpha] / Z`? | **Yes, exactly**, in all eight cases. |
| Do the equilibrium-averaged build/dismantle odds equal `exp[Xi/alpha]`, with no channel reading `N_tau`? | **Yes**, on every neighbouring pair with positive toggle flux (11 of the 12 neighbouring pairs in the primary witness, checked in both directions; the one zero-flux pair `none <-> {1,2}` is reported). This is an identity given the configuration law. |
| Does the home-bridge preference at `tau = 2` follow exactly from the entropy? | **Yes**: closed forms and odds `[N_2 + d_u + 3]/[b N_2]` hold on every configuration and unbuilt bridge in every `tau = 2` case. |
| Is a random-walk destination readout an equiprobable channel over committable futures? | **No** (C02): the first non-uniform law is `[4/9, 5/18, 5/18]` at `{0,1},{0,2}`. |
| Can a take-only-gains builder hold the temperature? | **No** (C01): reciprocal support fails at the first BUILD. |

No fitting, sampling, random seed, simulation length, tolerance, or
floating-point acceptance path was involved.

## 1. The result in plain language

A traveller lives on a small archipelago. It holds one plan: a short walk from
home, a few steps long, which may stay put at any step. It walks that plan
forward and back. Only at home may it pick a new plan, uniformly among the
plans the current bridges allow. It may build or dismantle a bridge only at
the island where it stands, and it never dismantles a bridge its plan still
uses. Every bridge holds one packet of timber drawn from a reservoir whose
arrangements multiply by `b` per packet, which fixes the temperature
`alpha = lambda / ln b` before anything moves.

No rule in this world looks at how many futures a configuration offers. Every
move is equally likely as its reverse at the microscopic level.

The result: the world settles into a law over bridge configurations in which
each configuration is weighted by the number of walks the traveller could
commit to, times the Boltzmann factor of its timber. The log-count of
committable futures is the entropy of the configuration. On average, building
a bridge is favoured over dismantling it by exactly the factor that the
causal entropic term `alpha * Delta ln N_tau`, set against the timber cost,
predicts at the reservoir's temperature. At horizon two, bridging home to an
island with `d` bridges opens exactly `d + 3` new futures, so the traveller
prefers connecting to well-connected islands. Nobody wrote that preference
down.

Why it works: a held plan must be a valid walk, so each configuration has
exactly `(tau + 1) N_tau(G)` system states. Uniform microscopic dynamics give
every system state the same weight up to its energy. Counting states
configuration by configuration gives the future count.

## 2. What was frozen before execution

| Object | Revision |
|---|---|
| Boundary (preregistration) | `3a6405c43390400eff43ba722273c96f76e29dac` |
| Boundary SHA-256 | `097f8f07123176ce15e3610853c35a52c785211218b0dfcdacdc3b03f854845d` |
| Apparatus commit (approved) | `31307b6171bead8c502fb767b8dca2a398f1d178` |
| Apparatus Git tree | `e170da4e0e240abe027dac85e577b88ee0fd44d5` |

Apparatus file digests (SHA-256):

```text
751a37a62a26b7554f9e73f24a69f574ce0e316dcd49b082f800dffbb6978e57  Cargo.lock
b7becbca51b96cc0d6dc9ad4e31119283e6802505c1153826fb3835a883fb423  Cargo.toml
94c8fa2d9d8f4e26923d1811effed0b2b7ec406685aad0e88f6a57714dbcef9b  README.md
a3000cdf95dbdc07c33db2a4f1707a4eb4d90b7769381c325d47b2770f17c397  src/contact.rs
20a9f9aaa6865dec51ae4a273fccaa6e00da67d07ad71b5ebedddc5b3d6ce668  src/gates.rs
6dd757f99d32ee7cecbd9a1608e09d258200fdee468abeacb90890764b53cb51  src/lib.rs
a46078cb442af044f9750d4c6edd1ccbcb7a20ba5e941626e10dee42bacc2a85  src/main.rs
973d7866e5309f6b00a933bfe74df06d314329891ace7f33d0e39c84b363387f  src/model.rs
bbbebb17ab317e273f84b2c57604b5841dcbd2ae5694be0b23cd6997a378bc76  src/ratio.rs
72feec83985556f54cdc073dbaa884c7e7f846ad0c63422b53a21f42e41b3d66  tests/source_contract.rs
003991652556b41f87e6cea13f7b8c33d366bd3e974d84f406480ecd509f609a  xypher.runa
```

### 2.1 Review chronology

1. **Boundary v1** was reviewed and judged incorrect: its plans were held but
   never read, so the headline held for any positive count by construction.
   The boundary was redesigned (v2) so the traveller walks its plan and acts
   only locally, and no channel reads the future count.
2. **Boundary v2** review found disconnected contact shells, zero-flux
   neighbour pairs, a REPLAN rate misstatement, a mislabelled target class,
   and scope wording. All were corrected (v2.1).
3. **Boundary v2.1** was judged READY TO FREEZE after two label clarifications
   and was frozen at `3a6405c4`.
4. **Apparatus `9ba56af4`** was approved without execution by three
   independent static reviewers (mathematical correspondence, adversarial
   checker behaviour, Futuruna/Xypher semantics). Their non-blocking findings
   made several checks independent of the generator builders, added
   failure-location detail to control lines, made the primary block always
   report all twelve gates, reported contact current per shell, and bounded
   the Futuruna declaration's islands and candidate bridges.
5. **Apparatus `31307b61`** was approved by all three reviewers without
   execution, pushed, and confirmed equal to the fetched remote before the
   run.

Only non-executing checks ran before approval: `cargo check`,
`cargo test --no-run`, `cargo fmt --check`, `cargo clippy`, `runa check`,
`runa fmt --check`. Design-time calculations used to state predictions are
disclosed in boundary section 5.3.

## 3. Execution

```text
time:        2026-10-08T18:06:53Z
toolchain:   rustc 1.94.0 (4a4ef493e 2026-03-02); cargo 1.94.0 (85eff7c80 2026-01-15)
host:        aarch64-apple-darwin
HEAD:        31307b6171bead8c502fb767b8dca2a398f1d178
commands:    cargo test --locked --manifest-path Cargo.toml        (exit 0, 3.07 s incl. build)
             cargo run --locked --quiet --manifest-path Cargo.toml (exit 0, 0.75 s)
report:      31 lines, SHA-256 7d8629680960ceb785a969d9310b1661947cfcd1d98f161792d0cd46a54404b7
```

The source-contract test (12 gates, 8 controls, success) passed. The frozen
commands were executed once.

### 3.1 Complete report

```text
XYPHER_CAUSAL_ENTROPIC_PROOF CAL-CEF-1 primary=(3,2,2)
G01 PASS Crystal grounding
G02 PASS executable plans -- plan read at z3[G={0,1} p=[0, 0, 0] t=0] vs z6[G={0,1} p=[0, 0, 1] t=0]
G03 PASS reciprocal generator -- row closure: each diagonal is -(exit rate) and each exit rate equals the rate counted independently from the section 3.4 rules
G04 PASS pathwise accounting
G05 PASS reservoir lumpability
G06 PASS system-state local detailed balance
G07 PASS connectivity and equilibrium
G08 PASS configuration law
G09 PASS emergent causal entropic odds -- positive-flux pairs=22 zero-flux=[none<->{1,2}] non-lumpable at z3[G={0,1} p=[0, 0, 0] t=0] vs z6[G={0,1} p=[0, 0, 1] t=0]; section 4.3 witness z3[G={0,1} p=[0, 0, 0] t=0] vs z8[G={0,1} p=[0, 0, 1] t=2]
G10 PASS Xypher slot reading -- hazards are computed from HazardInputs [ChannelKind, DestinationReservoirEnergy, ReservoirBase], a type with no N_tau, THAIM, or Xi field; every system hazard equals the independent b^(E_R(z'))/1 rule and every micro hazard is kappa
G11 PASS same-temperature contact -- identical components M0:9[9] M1:162[144+9+9] M2:1035[1026+9] M3:2916[2916] M4:4059[4059] M5:2754[2754] M6:729[729]; heterogeneous components M0:6[6] M1:84[72+6+6] M2:414[408+6] M3:960[960] M4:1146[1146] M5:684[684] M6:162[162]
G12 PASS home-bridge law -- home-bridge pairs=8
C01 PASS one-way-builder -> G03 check 1 G03_RECIPROCAL_SUPPORT at none (expected G03 check 1 G03_RECIPROCAL_SUPPORT) -- complete-state BUILD channel 0 -> 24 has no positive reverse
C02 PASS destination-readout -> G01 check 3 G01_REPLAN_UNIFORM at {0,1},{0,2} (expected G01 check 3 G01_REPLAN_UNIFORM at {0,1},{0,2}) -- REPLAN at z27[G={0,1},{0,2} p=[0, 0, 0] t=0] has outcome law [4/9,5/18,5/18] with reported multiplicity 3
C03 PASS directed-kinetic-mutation -> G03 check 2 G03_EQUAL_REVERSE_HAZARDS at none (expected G03 check 2 G03_EQUAL_REVERSE_HAZARDS) -- complete-state STEP channel 0 <-> 8: hazards 2 / 1, expected 1 / 1
C04 PASS unpaired-receipt -> G09 check 1 G09_XI_ANTISYMMETRY at none (expected G09 check 1 G09_XI_ANTISYMMETRY) -- exp(Xi/alpha) forward*reverse = 4 for none <-> {0,1}
C05 PASS no-staying -> G01 check 1 G01_COUNT_POSITIVE at none (expected G01 check 1 G01_COUNT_POSITIVE at none) -- N_tau(G)=0 < 1
C06 PASS horizon-slip -> G01 check 4 G01_REPLAN_ALPHABET at none (expected G01 check 4 G01_REPLAN_ALPHABET at none) -- REPLAN alphabet {[0, 0]} != Pi_tau(G) {[0, 0, 0]}
C07 PASS plan-breaker -> G02 check 1 G02_PLAN_VALIDITY at {0,1} (expected G02 check 1 G02_PLAN_VALIDITY) -- DISMANTLE from z6[G={0,1} p=[0, 0, 1] t=0] writes plan [0, 0, 1] invalid in destination none
C08 PASS unequal-reservoirs -> G11 check 1 G11_PREPARED_STATIONARY (expected G11 check 1 G11_PREPARED_STATIONARY) -- identical pair (3,2,2)x(3,2,2)[B prepared at b=3]: shell M=1 state (A z0[G=none p=[0, 0, 0] t=0], B z3[G={0,1} p=[0, 0, 0] t=0]): inflow 8/3 != outflow 7/3
WITNESS (3,2,2) N_2=1,4,4,7,1,5,5,9 system=3,12,12,21,3,15,15,27 complete=24,48,48,42,12,30,30,27 weights=1,2,2,7/4,1/2,5/4,5/4,9/8 contact_current=identical=[M0:0 M1:0 M2:0 M3:0 M4:0 M5:0 M6:0],heterogeneous=[M0:0 M1:0 M2:0 M3:0 M4:0 M5:0 M6:0]
FAMILY (3,1,2) configurations=8 system=32 complete=90 Z=45/8 mean_bridges=19/15 gates PASS=11 FAIL=0 N/A=1(G12)
FAMILY (3,2,2) configurations=8 system=108 complete=261 Z=87/8 mean_bridges=131/87 gates PASS=12 FAIL=0 N/A=0
FAMILY (3,3,2) configurations=8 system=344 complete=740 Z=185/8 mean_bridges=313/185 gates PASS=11 FAIL=0 N/A=1(G12)
FAMILY (3,2,3) configurations=8 system=108 complete=504 Z=56/9 mean_bridges=5/4 gates PASS=12 FAIL=0 N/A=0
FAMILY (4,1,2) configurations=64 system=320 complete=2916 Z=729/32 mean_bridges=7/3 gates PASS=10 FAIL=0 N/A=2(G05,G12)
FAMILY (4,2,2) configurations=64 system=1344 complete=10206 Z=1701/32 mean_bridges=55/21 gates PASS=11 FAIL=0 N/A=1(G05)
FAMILY (4,2,3) configurations=64 system=1344 complete=44544 Z=14848/729 mean_bridges=123/58 gates PASS=11 FAIL=0 N/A=1(G05)
FAMILY (4,3,2) configurations=64 system=5248 complete=34344 Z=4293/32 mean_bridges=151/53 gates PASS=10 FAIL=0 N/A=2(G05,G12)
OVERALL PASS
```

Every N/A is one the boundary permits: G05 at `N = 4` (reservoir lift not
enumerated) and G12 at `tau != 2`.

## 4. Gate and control results

### 4.1 Gates on the primary witness

| Gate | Result |
|---|---|
| G01 Crystal grounding | Pass. Depth-first and matrix-power plan counts agree; REPLAN is uniform over `Pi_tau(G)`; every brute-force plan has a system state at every cursor. |
| G02 executable plans | Pass. Plans are valid walks; plan contents are read: `z3` (plan stays home) and `z6` (plan `[0,0,1]`) at `{0,1}` have different BUILD/DISMANTLE sets. |
| G03 reciprocal generator | Pass at complete-state and system-state level; exit rates equal those counted independently from the local rules. |
| G04 pathwise accounting | Pass. Every packet conserved; heat equals `Delta U`; zero external work. |
| G05 reservoir lumpability | Pass on the explicit 261-state lift. |
| G06 local detailed balance | Pass on every system-state channel with `S(z)` constant. |
| G07 connectivity and equilibrium | Pass: connected; `pi(z) ~ b^(-U(z))` stationary; all flux products equal. |
| G08 configuration law | Pass: marginal `N_2(G) 2^(-|G|) / (87/8)`; mean bridges `131/87`; section 5 tables reproduced. |
| G09 emergent odds | Pass on 11 positive-flux neighbouring pairs (22 directed); zero-flux pair `none <-> {1,2}` reported; configuration projection not strongly lumpable. |
| G10 Xypher slot reading | Pass; no hazard reads `N_tau`, THAIM, or Xi. |
| G11 same-temperature contact | Pass for both pairs on every shell; components reported; current exactly zero on every shell. |
| G12 home-bridge law | Pass on all 8 (configuration, unbuilt home bridge) pairs. |

### 4.2 Controls

| Control | Observed first failure | Matched |
|---|---|---|
| C01 one-way builder | G03 check 1 at none: BUILD has no reverse | yes |
| C02 destination readout | G01 check 3 at `{0,1},{0,2}`: law `[4/9, 5/18, 5/18]` | yes |
| C03 directed kinetic mutation | G03 check 2: STEP hazards 2 / 1 | yes |
| C04 unpaired receipt | G09 check 1 at `none <-> {0,1}`: odds product 4 | yes |
| C05 no staying | G01 check 1 at none: `N_tau = 0` | yes |
| C06 horizon slip | G01 check 4 at none: alphabet `{[0,0]}` | yes |
| C07 plan breaker | G02 check 1 at `{0,1}`: dismantle invalidates plan `[0,0,1]` | yes |
| C08 unequal reservoirs | G11 check 1 at shell `M = 1`: inflow 8/3, outflow 7/3 | yes |

## 5. What this establishes

Within the declared finite model (boundary section 10):

1. A digital world in which a base Praxion holds one tau-step plan as
   executable state, walks it, replans uniformly among valid plans at home,
   and builds or dismantles bridges only at its own island satisfies T1--T5
   across the exact eight-case family, with reservoir lumpability verified on
   the explicit lift for `N = 3`, and T6 for the identical `(3,2,2)` pair and
   the heterogeneous `(3,2,2)`/`(3,1,2)` pair. `alpha` is fixed by an
   independently declared reservoir.
2. Its equilibrium configuration law is `N_tau(G) exp[-U(G)/alpha] / Z`: the
   log-count of committable tau-step futures is the configuration entropy.
3. Although no channel reads `N_tau`, the equilibrium-averaged build/dismantle
   odds between configurations with positive toggle flux equal
   `exp[Xi/alpha]` with `Xi = alpha Delta ln N_tau - Delta U`. This is an
   identity given claim 2. Its content is that local rules which never read
   the future count produce the causal entropic odds at the operational
   temperature.
4. At `tau = 2`, the emergent preference for bridging home to well-connected
   islands follows exactly from that entropy.

## 6. What this does not establish

- That the Shannon entropy of the traveller's realized trajectories, or of a
  random walker's endpoints, equals `ln N_tau`. C02 shows only that a
  destination readout is not an equiprobable channel over committable futures.
- Any effect on agents that do not hold their futures as state.
- A configuration-level rate rule. The configuration projection is not
  Markov; the force exists as an equilibrium average.
- Growth with new islands, sustained driven growth, or any nonequilibrium
  steady state. The island set is fixed.
- Learning, control capacity, intelligence, or the correlation free energy of
  control.
- Grammar endogeneity, artificial life, or consciousness.
- New fundamental physics. This is equilibrium statistical mechanics of a
  declared model; its counting is the same mechanism as the entropic
  elasticity of a polymer, whose conformations are walks. Here the walks are
  committed futures.
- External novelty. The design-time literature search (boundary section 12)
  is a statement about that search, not a novelty proof.

## 7. Relation to CAL-XTHERM

CAL-XTHERM showed that a digital graph can be an operational thermodynamic
system and that a minimal Xypher kernel reproduces it exactly, with a
one-step reshuffle as its future readout. CAL-CEF-1 is not an X1--X3 kernel
and does not use that representation theorem. It replaces the reshuffle with
an actor that commits to multi-step futures and acts only locally, verifies
T1--T6 directly, and finds the future count in the configuration entropy.

## 8. Next boundary

The held plan is a command and its final position is a destination. The next
boundary gives the destination its own register, measures the control
capacity of plan over destination inside this energy account, and tests the
correlation free energy `alpha ln 2` per bit against a shuffled pairing with
equal marginals and equal average energy. It must not reuse CAL-CEF-1 results
to rescue its own gates.
