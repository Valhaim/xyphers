---
title: Open Future Price Result
aliases:
  - CAL-CEF-3 Result
  - How Much Open Future Can a Unit of Energy Buy Result
tags:
  - domain/physics
  - type/result
  - topic/thermodynamics
  - topic/xypher
  - topic/nonequilibrium
domain: Physics
type: result
status: exact-run-passed
td: td-67b58b
created: 2026-10-10
updated: 2026-10-10
related:
  - "[[Open Future Price Boundary]]"
  - "[[Control Information Thermodynamics Result]]"
  - "[[Causal Entropic Thermodynamics Result]]"
---

# Open future price result (CAL-CEF-3)

## 0. Verdict

**The frozen CAL-CEF-3 apparatus passed, and the world answered.** All ten
gates passed across twelve driven worlds and both agents (24 exact steady
states). All six preregistered controls failed first at exactly the
preregistered gate, check, agent, and configuration. Two hypotheses held and
two were refuted.

```text
OVERALL PASS
H0 SUPPORTED   fuel buys open future                 (12 of 12 worlds)
H1 SUPPORTED   targeting beats heating               (12 of 12 worlds)
H2 REFUTED     yield rises with horizon              (fails at (b_c, b_h) = (3, 2))
H3 REFUTED     foresight sharpens targeting          (fails at every pair)
```

| Question (boundary section 0) | Answer |
|---|---|
| Does fuel buy open future? | **Yes.** A traveller that spends hot-store fuel only where it stands holds more two-step futures open than the cold world, in every world. |
| Where should the energy go? | **Where the traveller stands.** Per unit of fuel flow, targeted fuel holds 21% to 79% more open future than the same fuel spent everywhere. |
| Does planning further ahead buy more per unit of fuel? | **Not reliably.** At the colder pairs `(8, 2)` and `(6, 3)` the yield rises from `tau = 1` to `3`; at `(3, 2)` it peaks at `tau = 2` and falls. |
| Does foresight sharpen targeting? | **No.** At every pair the advantage of targeted over spread fuel shrinks as the horizon grows. |
| How close do the agents come to the price floor? | Every agent pays 2.2 to 3.0 times the Horowitz--Zhou--England floor for the state it holds (`eta` 0.33 to 0.45). The targeted agent sits slightly further from the floor than the spreading agent, yet buys more open future per unit of fuel. |

No fitting, sampling, random seed, simulation length, tolerance, or
floating-point acceptance path was involved. Every hypothesis was decided by
exact rational comparison.

## 1. The result in plain language

The archipelago of CAL-CEF-1 now has weather. A cold store builds bridges
anywhere and wears them down anywhere, except the bridges the traveller's held
plan crosses. Left alone, the weather settles the world at the cold store's
temperature, with a certain number of near futures open from home.

The traveller has a second, hotter store of timber and can spend it, but only
on bridges touching the island where it stands. Spending it costs energy:
packets leave the hot store, pass through the bridges, and end up in the cold
one. The question was how many open futures that flow buys.

It buys them. And where the fuel is spent matters more than how much: fuel
spent where the traveller stands keeps between a fifth and four fifths more
futures open per packet than the same fuel spread over the whole world. The
spreading agent keeps the world warmer everywhere; the targeted agent keeps
open the part of the world it is about to use.

Foresight, in the form this world gives it, did not compound that advantage.
At every pair, the targeted agent's edge shrank as the horizon grew, and at the
warmest pair planning further ahead even lowered the yield per packet. The
design points to why. In this world the plan does one thing for the
traveller: it decides which bridges the weather leaves alone, and that
protection is free, whoever paid for the bridge. A longer plan covers more of
the world with free protection, for the spreading agent as much as for the
targeted one. And the traveller spends fuel on every bridge around the island
it is on, whether or not its plan crosses that bridge next. Foresight that aims
the fuel, and protection that has to be paid for, are the next experiment
(section 7).

The price floor separates two things that are easy to conflate. `eta` asks how
cheaply an agent holds the state it holds; `Y` asks how much open future that
state contains per unit of fuel. The targeted agent holds its state a little
less cheaply than the spreading agent (`eta` 0.4307 against 0.4497 at
`(3, 2, 2, 3, 2)`), and still buys about 29% more future per packet. Choosing
what to hold mattered more than holding it cheaply.

## 2. What was frozen before execution

| Object | Revision |
|---|---|
| Boundary (preregistration) | `e5474cb773c3da74ce4162d20e07f37d904a945e` (2026-10-10T15:01:30+02:00) |
| Boundary SHA-256 | `6adf3ad6976bcbb9d8a5fd3deda9b6213465d74370f45fb1715715e7be09a2e9` |
| Apparatus commit (approved) | `33b96e33ebdced33befdb72dbeb42d6ff54f68bf` (2026-10-10T15:44:02+02:00) |
| Apparatus Git tree | `7ad5e7e3edab89549b1b6e6ecfd203e43ef9d61b` |

Apparatus file digests (SHA-256):

```text
8579a33106f877d654d8e74e1dce19f12b962d512944b46d0853eb2286580d4b  Cargo.lock
3f2f9c82272b549089d7392911909eddcb71e1b05c680a2cd26940257beddbf2  Cargo.toml
d8425ed32e98eebde54cbaed39cff5c9a4821a3c15552175ca5166de9b38b48a  README.md
09969f43da13c369235f990ae6e8242be65cde97c7112943c490ee3d5e964a63  src/big.rs
a04d09603c071d42ee116023eedc8f9f98c399b86747e60e4d90b9065288bf7c  src/gates.rs
4b132c1e59bfffe3f368d38d24ccedb5ec9baa23afe42e72077a9b52021898cf  src/lib.rs
1c061bdd18e0d33d8c8336cbed0c07ac9678f96a0e5603ec89de9b0a3e8f3c5c  src/logs.rs
ba08f3b5761e03bf288dd0b4d7ea5a724e93dc3b0ee4699bb8201a120b751c47  src/main.rs
051c2bacadf920f420fbebca58a9816b33e28a40ec3db13cc3e03aa8e47eb032  src/model.rs
361c7baead4a52c994073dcb46d32db8dfb9e329632b6612c55c815f3534c826  src/rat.rs
904b0db2d5253ae5c6feaab3e4464217d1071592622a78f71a9768981d270a9a  src/solve.rs
d252f31498079534e130d7a5ec67d99f93e3c564e6f016d3c1eb500b2a9de1da  tests/source_contract.rs
f2a6f02792bd24f83baf2d7048d87af2ef6581548901007248e64cf79c94257b  xypher.runa
```

### 2.1 Review chronology

1. **Pilot.** Before the boundary, a floating-point pilot at non-confirmatory
   temperatures dropped a first design whose agency questions could not fail
   (fuel and cold store on the same moves; `eta` fixed by the two
   temperatures) and found the falling targeting ratio that made H3 a genuine
   risk. Boundary section 9 discloses it in full.
2. **Boundary draft** was reviewed by an independent referee who rebuilt the
   undriven, LOCAL, and GLOBAL generators of every world in a separate
   throwaway implementation, confirmed every section 4 value and the GLOBAL
   product law by exact solves, and checked every control's expected first
   failure. It required five major changes (M1--M5): name every pilot pair;
   the floor bounds but cannot be reached with a fixed hot store (`Y_bound`,
   not `Y_best`); a G02 wording that fixed C01's first failure; section 0 must
   not assert H0; and H2 must be stated as confounded by the change of world,
   leaving H3 as the test of foresight. Fourteen minor changes followed. A
   second round confirmed all nineteen and required five residual fixes
   (R1--R5), applied before the freeze at `e5474cb7`. The referee computed no
   LOCAL steady state at a confirmatory pair.
3. **Apparatus `33b96e33`** was built under embargo and approved without
   execution by three independent static reviewers: mathematical
   correspondence (every gate check, frozen constant, sign convention, and
   hypothesis traced against the boundary), adversarial checker behaviour (no
   path to a false PASS, a wrong first failure, a wrong verdict, or a crash;
   exact-division, enclosure, and rounding soundness), and Futuruna/Xypher
   semantics (the declaration faithful to section 1; no rate reads `N_tau`,
   `N_R`, `H`, `J`, `Y`, or `eta`). It was pushed and confirmed equal to the
   fetched remote before the run.

### 2.2 Arithmetic test exception

Boundary section 12 permits only non-executing checks before approval. The
generic arithmetic of the apparatus was nevertheless executed once before
approval, outside the repository, and is disclosed here. Copies of
`src/big.rs`, `src/rat.rs`, `src/logs.rs`, and `src/solve.rs`, equal to the
approved files under the digests above, were placed in a throwaway crate. In
that crate:

- the eleven unit tests inside those four modules ran (big-integer identities,
  rational reduction and rounding, `ln 2` and `ln 3` enclosures, Bareiss on
  2 x 2 systems, and the stationary vector of two 3-state chains);
- a harness compared 119,224 randomized and edge cases against Python `int`,
  `fractions.Fraction`, and `mpmath` at 1700 digits, with no disagreement:
  integer operations on operands up to about 5200 bits, the Bareiss update and
  scale primitives, rationals and decimal rounding, logarithm enclosures of
  1526 integers, enclosure arithmetic, 1501 random Bareiss systems, and 600
  random stationary systems of up to 12 states;
- the stationary solver timed three random sparse generators of 108, 320, and
  344 states, unrelated to the archipelago.

No model, world, gate, report, or test code of the apparatus ran anywhere
before the frozen run, and no quantity of any archipelago world was computed by
this exercise.

### 2.3 Non-blocking review findings carried into this record

These did not block approval; changing the approved commit would have voided
the reviews, so they are recorded here instead.

- The apparatus README describes the arithmetic exception only by its
  differential comparison; section 2.2 above is the complete account.
- Parts of G09 are declarative: the slot binding, the empty Ruby and absent
  Opal, and the disjointness of the declared and forbidden rate reads test
  compile-time constants. The operative guards are the exhaustive
  `RateInputs` type and the G09 scan that recomputes every rate from the
  section 1.3 rule and every channel's store from its class.
- In `xypher.runa`, `channel_rate` takes the archipelago to map a bridge to its
  flag, so its signature is looser than the Rust `RateInputs`; the body reads
  only the island count. The declaration names the four channel classes but not
  the undriven partition (STEP, REPLAN, WEATHER) on which `sigma_floor`
  depends.
- The Rust field `Measures::open_future_held` holds `<N_R>_p`, while
  `Quantity::OpenFutureHeld` in Rust and `open_future_held` in Futuruna mean
  `H`. The report labels are correct.
- G10 check 3 also requires the measured GLOBAL `eta` to round to the frozen
  decimal, which is stricter than the boundary; G08 check 5 already ties the
  two enclosures together.
- G08 check 3 establishes nonnegative pair terms through positivity of every
  pair's two one-way flows and zero net pair flow at every state, as a
  theorem, rather than listing each term's sign.
- The exact 2-adic division does not itself verify exactness; every call is
  exact by Sylvester's identity, Cramer's rule, or a gcd, and G06 check 3
  verifies the steady state independently of the solver.
- Once `H` and `J` equal their frozen rationals, the `Y = H/J` part of G10
  check 2 cannot fail.
- A channel with an unresolved destination would be labelled as lacking a
  reverse in G08; G02 check 1 fails first on any such channel.
- Plan validity in G02 and the depth-first enumeration share one step rule;
  G01 check 2 compares the enumeration with separately built matrix powers.
- Readings the apparatus fixed where the boundary was silent: G09 is reported
  as three numbered checks; C01--C05 are expected at the LOCAL agent, the
  primary's agent, which runs first; C03 is matched by its world-level value
  `J = 0`.

## 3. Execution

```text
time:        2026-10-10T13:57:18Z (test), 2026-10-10T13:57:36Z (run)
toolchain:   rustc 1.94.0 (4a4ef493e 2026-03-02); cargo 1.94.0 (85eff7c80 2026-01-15)
host:        aarch64-apple-darwin
HEAD:        33b96e33ebdced33befdb72dbeb42d6ff54f68bf
commands:    cargo test --release --locked          (exit 0; 13.9 s; source contract 12.43 s)
             cargo run --release --locked --quiet   (exit 0; about 13 s)
report:      46 lines, SHA-256 82c0b86b5b921df60ee0fc6ecd67895184aab69336f99fac888bd281fad33bb5
```

The unit tests (11) and the source-contract test (ten gates, six controls,
success) passed. The frozen commands were executed once.

### 3.1 Complete report

```text
XYPHER_OPEN_FUTURE_PRICE_PROOF CAL-CEF-3 primary=(3,2,2,8,2) agent=LOCAL worlds=12 agents=LOCAL,GLOBAL
G01 PASS grounding -- N_tau(G) >= 1; depth-first counts equal e_h^T L^tau 1 and e_h^T L^R 1; state counts 32/108/344/320
G02 PASS executable rules -- destinations are system states; STEP/REPLAN/WEATHER/FUEL follow section 1.3
G03 PASS reciprocity -- every channel has a reverse in its class; STEP/REPLAN 1:1, WEATHER 1:b_c, FUEL 1:b_h
G04 PASS accounting -- one packet per bridge change with its named store; hot outflow equals cold inflow
G05 PASS undriven world -- undriven graph connected; b_c^(-|G|) stationary with detailed balance on every channel
G06 PASS driven steady state -- driven graph connected; exact p strictly positive; every state balanced
G07 PASS GLOBAL calibration -- GLOBAL steady states are q^|G| with q = 2/(b_c + b_h); H_global and J_global equal their closed forms
G08 PASS second law and the floor -- J > 0; second law in store symbols; nonnegative pair terms; certified eta in (0,1); GLOBAL eta meets its closed form
G09 PASS Xypher slot reading -- slots bound; Ruby empty, Opal absent; rates read only [State, Bridge, Class, StoreRatio] (RateInputs) and equal the section 1.3 rule
G10 PASS frozen values -- section 4.1 and 4.2 values reproduced exactly; Y_global and eta_global decimals match
C01 PASS one-way-fuel observed G03 check 1 G03_REVERSE_IN_CLASS world (3,2,2,8,2) agent LOCAL configuration none expected G03 check 1 G03_REVERSE_IN_CLASS agent LOCAL configuration none -- FUEL bridge {0,1} rate 1 z0[G=none p=000 t=0] -> z3[G={0,1} p=000 t=0]: no reverse FUEL channel
C02 PASS plan-breaking-weather observed G02 check 1 G02_DESTINATION_STATES world (3,2,2,8,2) agent LOCAL configuration {0,1} expected G02 check 1 G02_DESTINATION_STATES agent LOCAL configuration {0,1} -- WEATHER bridge {0,1} rate 8 z6[G={0,1} p=001 t=0] -> [G=none p=001 t=0] (not a system state): plan 001 is not valid in none
C03 PASS equal-stores observed G08 check 1 G08_POSITIVE_FUEL_FLOW world (3,2,2,8,8) agent LOCAL world-level value 0 expected G08 check 1 G08_POSITIVE_FUEL_FLOW agent LOCAL world-level value 0 (J = 0) -- J = 0 is not positive
C04 PASS biased-weather observed G03 check 2 G03_RATE_RATIOS world (3,2,2,8,2) agent LOCAL configuration none expected G03 check 2 G03_RATE_RATIOS agent LOCAL configuration none -- WEATHER bridge {0,1} rate 2 z0[G=none p=000 t=0] -> z3[G={0,1} p=000 t=0]: rates 2 : 8 (forward : reverse), declared 1 : 8
C05 PASS wrong-store observed G03 check 2 G03_RATE_RATIOS world (3,2,2,8,2) agent LOCAL configuration none expected G03 check 2 G03_RATE_RATIOS agent LOCAL configuration none -- FUEL bridge {0,1} rate 1 z0[G=none p=000 t=0] -> z3[G={0,1} p=000 t=0]: rates 1 : 8 (forward : reverse), declared 1 : 2
C06 PASS wrong-calibration observed G07 check 1 G07_PRODUCT_LAW world (3,2,2,8,2) agent GLOBAL configuration none expected G07 check 1 G07_PRODUCT_LAW agent GLOBAL configuration none -- z0[G=none p=000 t=0]: p = 125/1332 differs from q^|G| / Z with q = 1/8
WORLD (3,1,2,3,2) LOCAL states=32 N2_p=189000324707150/51214587512901 ~3.6904 N2_pi_c=55/16 ~3.4375 H=207202882104845/819433400206416 ~0.2529 J=4060661226767/17071529170967 ~0.2379 Y=207202882104845/194911738884816 ~1.0631 eta=[0.439039067605, 0.439039067606] Y_bound=[2.421333599761, 2.421333599762]
WORLD (3,1,2,3,2) GLOBAL states=32 N2_p=2011/539 ~3.7310 N2_pi_c=55/16 ~3.4375 H=2531/8624 ~0.2935 J=29/77 ~0.3766 Y=2531/3248 ~0.7792 eta=[0.449660286786, 0.449660286787] Y_bound=[1.732972182269, 1.732972182270]
WORLD (3,2,2,3,2) LOCAL states=108 N2_p=1779690019007012271791552559470941600438712956/404767525678892987948180144810476581742470153 ~4.3968 N2_pi_c=117/28 ~4.1786 H=50480000566650286127069280048175607110509283/231295728959367421684674368463129475281411516 ~0.2182 J=79504276189998213582412320668710994337569350/404767525678892987948180144810476581742470153 ~0.1964 Y=353360003966552002889484960337229249773564981/318017104759992854329649282674843977350277400 ~1.1111 eta=[0.430677253071, 0.430677253072] Y_bound=[2.579971908770, 2.579971908771]
WORLD (3,2,2,3,2) GLOBAL states=108 N2_p=629/141 ~4.4610 N2_pi_c=117/28 ~4.1786 H=1115/3948 ~0.2824 J=323/987 ~0.3273 Y=1115/1292 ~0.8630 eta=[0.449660286786, 0.449660286787] Y_bound=[1.919233522137, 1.919233522138]
WORLD (3,3,2,3,2) LOCAL states=344 N2_p=7601410388681059552638578785222361427612616909883937421403282987737341278182633233703953317752785307137419866674299037077622540307281/1537502125707990359994472401587209338693112617775107079672741879938990598633126633852793119663661430861050146750458372591962326429292 ~4.9440 N2_pi_c=262/55 ~4.7636 H=1386546767451345552415460361034639252827129471503500300265653798321657587105968162207785011320354273360268565315123038197737290220541/7687510628539951799972362007936046693465563088875535398363709399694952993165633169263965598318307154305250733752291862959811632146460 ~0.1804 J=63852512302244765146839542879763381481530684129065646486752259978589966159958673934850005249315794876323534483245729654369101855825/384375531426997589998618100396802334673278154443776769918185469984747649658281658463198279915915357715262536687614593147990581607323 ~0.1661 Y=1386546767451345552415460361034639252827129471503500300265653798321657587105968162207785011320354273360268565315123038197737290220541/1277050246044895302936790857595267629630613682581312929735045199571799323199173478697000104986315897526470689664914593087382037116500 ~1.0857 eta=[0.423553213550, 0.423553213551] Y_bound=[2.563412846528, 2.563412846529]
WORLD (3,3,2,3,2) GLOBAL states=344 N2_p=10099/2011 ~5.0219 N2_pi_c=262/55 ~4.7636 H=28563/110605 ~0.2582 J=579/2011 ~0.2879 Y=9521/10615 ~0.8969 eta=[0.449660286786, 0.449660286787] Y_bound=[1.994702047795, 1.994702047796]
WORLD (3,1,2,8,2) LOCAL states=32 N2_p=1032899160810696079/386582721620358879 ~2.6719 N2_pi_c=1955/891 ~2.1942 H=1662059914288167656/3479244494583229911 ~0.4777 J=5387311472562710/6136233676513633 ~0.8780 Y=831029957144083828/1527302802471528285 ~0.5441 eta=[0.336060444065, 0.336060444066] Y_bound=[1.619101719579, 1.619101719580]
WORLD (3,1,2,8,2) GLOBAL states=32 N2_p=391/144 ~2.7153 N2_pi_c=1955/891 ~2.1942 H=7429/14256 ~0.5211 J=11/8 ~1.3750 Y=7429/19602 ~0.3790 eta=[0.339035952556, 0.339035952557] Y_bound=[1.117851769820, 1.117851769821]
WORLD (3,2,2,8,2) LOCAL states=108 N2_p=79613033850942196645989023758292552303760157412705873/23753251949131225589405297928228728736234632436840237 ~3.3517 N2_pi_c=3497/1233 ~2.8362 H=1677527630233314730928237715439872511102640495359559180/3254195517030977905748525816167335836864144643847112469 ~0.5155 J=1972704878532985190024110369783464457442168895567172/2639250216570136176600588658692080970692736937426693 ~0.7474 Y=22072731976754141196424180466314111988192638096836305/32004540989883825517101685341355416789818345371504251 ~0.6897 eta=[0.332960603937, 0.332960603938] Y_bound=[2.071341165115, 2.071341165116]
WORLD (3,2,2,8,2) GLOBAL states=108 N2_p=763/222 ~3.4369 N2_pi_c=3497/1233 ~2.8362 H=54815/91242 ~0.6008 J=91/74 ~1.2297 Y=54815/112203 ~0.4885 eta=[0.339035952556, 0.339035952557] Y_bound=[1.440950940400, 1.440950940401]
WORLD (3,3,2,8,2) LOCAL states=344 N2_p=12763173854274917096326632997678671357692994342506670810427128556214692140971265139552095847762084809422133919527752238853758719305918630441257851142989574631/3225044396970136316544014347117165883282839905056955429306045060039768021333707772725938134106252884021161419785086262703116677235372577658890183814085584112 ~3.9575 N2_pi_c=6827/1955 ~3.4921 H=2934626786992342290272581562692911019117855907776706718512666702508226853953600383424367740831487363207802799803971711484920740757182334835415814085782335670981/6304961796076616498843548048614059301817952014386347864293318092377746481707398695679209052177724388261370575679843643584593103995153389323130309356537316938960 ~0.4654 J=8192518595711864645228242374455357169266227712265629719935406837124305783632079523687424477815678990230042247192161547811345655893243432321680682630062631055/12900177587880545266176057388468663533131359620227821717224180240159072085334831090903752536425011536084645679140345050812466708941490310635560735256342336448 ~0.6351 Y=11738507147969369161090326250771644076471423631106826874050666810032907415814401533697470963325949452831211199215886845939682963028729339341663256343129342683924/16016373854616695381421213842060223265915475177479306102473720366578017807000715468808914854129652425899732593260675825971180757271290910188885734541772443712525 ~0.7329 eta=[0.329570510238, 0.329570510239] Y_bound=[2.223823559088, 2.223823559089]
WORLD (3,3,2,8,2) GLOBAL states=344 N2_p=1594/391 ~4.0767 N2_pi_c=6827/1955 ~3.4921 H=1143/1955 ~0.5847 J=429/391 ~1.0972 Y=381/715 ~0.5329 eta=[0.339035952556, 0.339035952557] Y_bound=[1.571712760399, 1.571712760400]
WORLD (3,1,2,6,3) LOCAL states=32 N2_p=70744772818366385/25095537154463271 ~2.8190 N2_pi_c=367/147 ~2.4966 H=132157718734648682/409893773522900093 ~0.3224 J=3980399759819178/8365179051487757 ~0.4758 Y=66078859367324341/97519794115569861 ~0.6776 eta=[0.410544845085, 0.410544845086] Y_bound=[1.650475791333, 1.650475791334]
WORLD (3,1,2,6,3) GLOBAL states=32 N2_p=345/121 ~2.8512 N2_pi_c=367/147 ~2.4966 H=6308/17787 ~0.3546 J=41/55 ~0.7455 Y=31540/66297 ~0.4757 eta=[0.415037499278, 0.415037499279] Y_bound=[1.146252971969, 1.146252971970]
WORLD (3,2,2,6,3) LOCAL states=108 N2_p=124910463385850611212984520364120936802470625126001/35467122004589372863030685374036420075178825767557 ~3.5219 N2_pi_c=99/31 ~3.1935 H=360979286507021034162482279258143453433885627917888/1099480782142270558753951246595129022330543598794267 ~0.3283 J=14292495410939404728189043200688303380751904646478/35467122004589372863030685374036420075178825767557 ~0.4030 Y=180489643253510517081241139629071726716942813958944/221533678869560773286930169610668702401654522020409 ~0.8147 eta=[0.406038781227, 0.406038781228] Y_bound=[2.006527015662, 2.006527015663]
WORLD (3,2,2,6,3) GLOBAL states=108 N2_p=1143/319 ~3.5831 N2_pi_c=99/31 ~3.1935 H=3852/9889 ~0.3895 J=635/957 ~0.6635 Y=11556/19685 ~0.5870 eta=[0.415037499278, 0.415037499279] Y_bound=[1.414440803811, 1.414440803812]
WORLD (3,3,2,6,3) LOCAL states=344 N2_p=1346598808995138924321497580732917981019813382897941975347481548059725032798339049578694384706495236375409541661799310747107332546114202947588060575/326014813637071387172672155309854742047925176388816187435875290791045978420460424262355219927199949199698335467251467865251410636989920894181732097 ~4.1305 N2_pi_c=1411/367 ~3.8447 H=34194860859308257925349200986775858004649087638925064480505692831753211485720772561197623870004623429000950445588525886318650635631134100074394242158/119647436604805199092370680998716690331588539734695540788966231720313874080308975704284365713282381356289289116481288706547267703775300968164695679599 ~0.2858 J=111508925484242357732635202216855243974175860448488822611088052549216620781116298853438707467131087035592785958215678889358518826277202374466170989/326014813637071387172672155309854742047925176388816187435875290791045978420460424262355219927199949199698335467251467865251410636989920894181732097 ~0.3420 Y=34194860859308257925349200986775858004649087638925064480505692831753211485720772561197623870004623429000950445588525886318650635631134100074394242158/40923775652716945287877119213585874538522540784595397898269315285562499826669681679212005640437108942062552446665154152394576409243733271429084752963 ~0.8356 eta=[0.401318724720, 0.401318724721] Y_bound=[2.082071896864, 2.082071896865]
WORLD (3,3,2,6,3) GLOBAL states=344 N2_p=2423/575 ~4.2139 N2_pi_c=1411/367 ~3.8447 H=77916/211025 ~0.3692 J=1019/1725 ~0.5907 Y=233748/373973 ~0.6250 eta=[0.415037499278, 0.415037499279] Y_bound=[1.505983860939, 1.505983860940]
WORLD (4,1,2,3,2) LOCAL states=320 N2_p=808756818221235173429613995857094995/157889320933108423617133931850766867 ~5.1223 N2_pi_c=19/4 ~4.7500 H=235130175155880644992911278263809507/631557283732433694468535727403067468 ~0.3723 J=58902140339977240821392875515084096/157889320933108423617133931850766867 ~0.3731 Y=78376725051960214997637092754603169/78536187119969654428523834020112128 ~0.9980 eta=[0.432400591012, 0.432400591013] Y_bound=[2.307974579155, 2.307974579156]
WORLD (4,1,2,3,2) GLOBAL states=320 N2_p=3337/637 ~5.2386 N2_pi_c=19/4 ~4.7500 H=1245/2548 ~0.4886 J=72/91 ~0.7912 Y=415/672 ~0.6176 eta=[0.449660286786, 0.449660286787] Y_bound=[1.373391295510, 1.373391295511]
WORLD (4,1,2,8,2) LOCAL states=320 N2_p=12319338333773162063597120488826622914843347/3551587265617793928953782146761044947519669 ~3.4687 N2_pi_c=226/81 ~2.7901 H=65069227668668233069270664808986765987621971/95892856171680436081752117962548213583031063 ~0.6786 J=4763379397131605537701315129437439460352116/3551587265617793928953782146761044947519669 ~1.3412 Y=65069227668668233069270664808986765987621971/128611243722553349517935508494810865429507132 ~0.5059 eta=[0.333837581632, 0.333837581633] Y_bound=[1.515519358773, 1.515519358774]
WORLD (4,1,2,8,2) GLOBAL states=320 N2_p=97/27 ~3.5926 N2_pi_c=226/81 ~2.7901 H=65/81 ~0.8025 J=17/6 ~2.8333 Y=130/459 ~0.2832 eta=[0.339035952556, 0.339035952557] Y_bound=[0.835381612881, 0.835381612882]
WORLD (4,1,2,6,3) LOCAL states=320 N2_p=144411748566299157968427631961349208321217/38884440999723099507262231673659615392805 ~3.7139 N2_pi_c=797/245 ~3.2531 H=877995784392796678995354237324768514126516/1905337608986431875855849352009321154247445 ~0.4608 J=28391086077657891013444914094706457006477/38884440999723099507262231673659615392805 ~0.7301 Y=877995784392796678995354237324768514126516/1391163217805236659658800790640616393317373 ~0.6311 eta=[0.407277945710, 0.407277945711] Y_bound=[1.549613712405, 1.549613712406]
WORLD (4,1,2,6,3) GLOBAL states=320 N2_p=7829/2057 ~3.8060 N2_pi_c=797/245 ~3.2531 H=278676/503965 ~0.5530 J=288/187 ~1.5401 Y=7741/21560 ~0.3590 eta=[0.415037499278, 0.415037499279] Y_bound=[0.865089365480, 0.865089365481]
H0 SUPPORTED fuel-buys-open-future H_local>0 in every world failing=none
H1 SUPPORTED targeting-beats-heating Y_local>Y_global in every world failing=none
H2 REFUTED yield-rises-with-horizon Y_local strictly increasing tau=1,2,3 (N=3) for each pair failing=(3,2)
H3 REFUTED foresight-sharpens-targeting Y_local/Y_global strictly increasing tau=1,2,3 (N=3) for each pair failing=(3,2);(8,2);(6,3)
OVERALL PASS
```

### 3.2 Independent check after the run

After the frozen run, a separate implementation written from the boundary
alone, sharing no code with the apparatus, rebuilt every driven world, solved
each steady state by exact elimination over Python integers, and re-decided
the hypotheses: `research/physics/xypher-open-future-price-check/independent_check.py`.
`H` and `J` agree with the frozen report in all 24 steady states, as exact
rationals, and the verdicts agree: H0 and H1 supported, H2 refuted at
`(3, 2)`, H3 refuted at every pair. It needs only the Python standard library
and takes about ninety seconds.

## 4. Gate and control results

### 4.1 Gates

| Gate | Result |
|---|---|
| G01 grounding | Pass. Depth-first plan counts equal matrix powers for `tau` and `R`; state counts 32, 108, 344, 320. |
| G02 executable rules | Pass. Every destination a system state with a valid plan; WEATHER everywhere and FUEL only where permitted; no channel removes a bridge the held plan uses. |
| G03 reciprocity | Pass. Every channel reversed in its class; STEP and REPLAN 1 : 1, WEATHER 1 : `b_c`, FUEL 1 : `b_h`. |
| G04 accounting | Pass. One packet per bridge change with its named store; in every driven steady state the hot store's outflow equals the cold store's inflow exactly. |
| G05 undriven world | Pass. Connected; `b_c^(-|G|)` stationary with detailed balance on every channel. |
| G06 driven steady state | Pass in all 24. Connected; the exact solution strictly positive; every state balanced in integers. |
| G07 GLOBAL calibration | Pass. Every GLOBAL steady state is `q^|G|` with `q = 2/(b_c + b_h)`; `H_global` and `J_global` equal their closed forms. |
| G08 second law and the floor | Pass. `J > 0` everywhere; the store-symbol identity `sigma = J ln(b_c/b_h)`; nonnegative pair terms; certified `eta` in `(0, 1)` with width below `10^-9`; GLOBAL `eta` meets `ln[2 b_c/(b_c+b_h)]/ln(b_c/b_h)`. |
| G09 Xypher slot reading | Pass. Rates read only state, bridge, class, and store ratio, and equal the section 1.3 rule. |
| G10 frozen values | Pass. Sections 4.1 and 4.2 reproduced exactly, including the four-decimal `Y_global` and `eta_global`. |

### 4.2 Controls

| Control | Observed first failure | Matched |
|---|---|---|
| C01 one-way fuel | G03 check 1, LOCAL, at none: FUEL build of `{0,1}` without a return | yes |
| C02 plan-breaking weather | G02 check 1, LOCAL, at `{0,1}`: wearing `{0,1}` under plan `001` leaves no valid plan | yes |
| C03 equal stores | G08 check 1, LOCAL, world-level: `J = 0` at `(b_c, b_h) = (8, 8)` | yes |
| C04 biased weather | G03 check 2, LOCAL, at none: WEATHER rates 2 : 8 against 1 : 8 | yes |
| C05 wrong store | G03 check 2, LOCAL, at none: FUEL rates 1 : 8 against 1 : 2 | yes |
| C06 wrong calibration | G07 check 1, GLOBAL, at none: `p = 125/1332` against `q = 1/8` | yes |

## 5. The measurements

Decimals are rounded from the exact rationals of section 3.1.

### 5.1 Open future held per unit of fuel flow

| World `(N, tau, R, b_c, b_h)` | `Y_local` | `Y_global` | `Y_local / Y_global` | `eta_local` | `eta_global` | `Y_bound` local |
|---|---:|---:|---:|---:|---:|---:|
| (3, 1, 2, 3, 2) | 1.0631 | 0.7792 | 1.3642 | 0.4390 | 0.4497 | 2.4213 |
| (3, 2, 2, 3, 2) | 1.1111 | 0.8630 | 1.2875 | 0.4307 | 0.4497 | 2.5800 |
| (3, 3, 2, 3, 2) | 1.0857 | 0.8969 | 1.2105 | 0.4236 | 0.4497 | 2.5634 |
| (3, 1, 2, 8, 2) | 0.5441 | 0.3790 | 1.4357 | 0.3361 | 0.3390 | 1.6191 |
| (3, 2, 2, 8, 2) | 0.6897 | 0.4885 | 1.4117 | 0.3330 | 0.3390 | 2.0713 |
| (3, 3, 2, 8, 2) | 0.7329 | 0.5329 | 1.3754 | 0.3296 | 0.3390 | 2.2238 |
| (3, 1, 2, 6, 3) | 0.6776 | 0.4757 | 1.4243 | 0.4105 | 0.4150 | 1.6505 |
| (3, 2, 2, 6, 3) | 0.8147 | 0.5870 | 1.3878 | 0.4060 | 0.4150 | 2.0065 |
| (3, 3, 2, 6, 3) | 0.8356 | 0.6250 | 1.3368 | 0.4013 | 0.4150 | 2.0821 |
| (4, 1, 2, 3, 2) | 0.9980 | 0.6176 | 1.6160 | 0.4324 | 0.4497 | 2.3080 |
| (4, 1, 2, 8, 2) | 0.5059 | 0.2832 | 1.7863 | 0.3338 | 0.3390 | 1.5155 |
| (4, 1, 2, 6, 3) | 0.6311 | 0.3590 | 1.7578 | 0.4073 | 0.4150 | 1.5496 |

### 5.2 Reading the table

1. **H0.** `H_local` is positive in every world, between 0.18 and 0.68 open
   two-step futures above the cold world.
2. **H1.** The targeted agent holds fewer extra futures than the spreading
   agent in every world (`H_local < H_global`) but draws much less fuel; per
   unit of fuel flow it wins everywhere, by a factor of 1.21 to 1.79.
3. **H2.** `Y_local` rises with the horizon at `(8, 2)` and `(6, 3)`. At
   `(3, 2)` it rises from `tau = 1` to `tau = 2` and falls at `tau = 3`
   (1.1111 to 1.0857). Refuted at `(3, 2)`.
4. **H3.** The ratio `Y_local / Y_global` falls with the horizon at all three
   pairs: 1.364, 1.288, 1.211 at `(3, 2)`; 1.436, 1.412, 1.375 at `(8, 2)`;
   1.424, 1.388, 1.337 at `(6, 3)`. Refuted at every pair, in the direction
   the pilot pointed to.
5. **Price floor.** `eta_global` equals `ln[2 b_c/(b_c+b_h)]/ln(b_c/b_h)` for
   every world, as preregistered. `eta_local` lies below `eta_global` in all
   twelve worlds and falls with the horizon. `Y_bound`, the yield of an agent
   that held the targeted state at the floor, ranges from 1.5 to 2.6.
6. **Wider archipelago.** Not preregistered as a hypothesis: at `tau = 1`,
   the targeting advantage is larger with four islands (1.62 to 1.79) than with
   three (1.36 to 1.44) at every pair.

## 6. What this establishes

Within the declared driven worlds (boundary section 10):

1. A digital world with an environment at one temperature and an agent fed
   from a store at a higher temperature, every channel locally reversible and
   blind to how many futures it opens, settles into an exact nonequilibrium
   steady state; its fuel flow, entropy production, and the
   Horowitz--Zhou--England floor for the state it holds are computed exactly or
   with certified enclosures.
2. Fuel buys open future (H0).
3. Targeted action buys more open future per unit of fuel flow than the same
   kind of fuel spent everywhere (H1), and the advantage grows with the size of
   the world in the cases measured.
4. In these worlds, a longer planning horizon does not reliably raise the yield
   (H2 refuted at `(3, 2)`), and it shrinks the advantage of targeting at every
   pair (H3 refuted).
5. Yield and thermodynamic efficiency are distinct: the agent with the higher
   yield holds its own state slightly further from the floor.

The gates are tests of the apparatus and the construction; the hypotheses are
the experiment. The second law, the GLOBAL product law, and the floor bound
were known before the run. Which agent wins, by how much, and how the horizon
acts were not; two of four preregistered expectations failed.

## 7. Next boundary

The traveller spends fuel on every bridge around the island it is on, whether
or not its plan crosses that bridge next. Its plan decides where it walks and
what the weather leaves alone, and that protection is free. Raising the horizon
changes both at once. The next experiment separates aiming from protection:

- **Aim by the plan.** A PLAN agent spends fuel on the bridges its held plan
  will cross next, wherever they are, and the hypothesis that foresight
  sharpens targeting is tested again with aiming that actually reads the plan.
- **Pay for protection.** The weather may wear any bridge, and a worn bridge
  under the plan forces a replan, so protection is no longer a free gift of the
  horizon.

Each change has its own preregistered hypotheses and controls, and neither
reuses a CAL-CEF-3 number to rescue its own gates.
