# Aimed Foresight Proof (CAL-CEF-4)

Standalone exact verifier for the preregistered boundary
`../derivable/xypher-aimed-foresight-boundary.md`, frozen at commit
`cec8eb84034144a6adae018fd4df8cefbce64b20`.

CAL-CEF-4 asks whether seeing further ahead lets an agent buy more open
future per unit of energy. A traveller holds a route `(v_1, ..., v_tau)` in
`V^tau` from home `0` on an archipelago of `N` islands. The route is an
intention, not a shield: a cold store (ratio `b_c`) acts through WEATHER,
which builds every absent bridge at rate 1 and wears every built bridge at
rate `b_c`, unconditionally, and a worn bridge blocks the traveller's next
step until WEATHER or FUEL rebuilds it. A hot store (ratio `b_h < b_c`) acts
through FUEL, the Praxion's moves, on the bridges of its permission set
`A(r, t)` at build rate 1 and return rate `b_h`: LOCAL fuels the bridges at
the traveller's island, AIM-k the route's bridges within `k` moves ahead or
behind, GLOBAL every bridge. STEP moves the cursor through open steps and
REPLAN changes the route at `t = 0`, both at rate 1. For eighteen worlds and
66 steady states the apparatus measures the open future at the traveller's
island held above the undriven world, `H = <N_2(G, x)>_p - <N_2(G, x)>_(pi_c)`,
the fuel flow `J`, the yield `Y = H/J`, the density `rho = <|G|>_p / K`, the
rival's inverse `s*(J)`, the edge at equal power `E = H / H_warm` with
`H_warm = Phi_N(rho) - Phi_N(r_0)`, and, through certified logarithm
enclosures, `eta = sigma_floor / (J ln(b_c/b_h))` and
`Y_bound = H ln(b_c/b_h) / sigma_floor`.

No result has been generated. The evaluator is under review embargo.

## Structure

- `xypher.runa` is the prospective Futuruna construction: archipelago,
  routes as intentions, the step rule, `N_2` as a walk count from the
  traveller's island, the two stores with their ratios, the four channel
  classes with WEATHER unconditional, the LOCAL, two-sided AIM-k, and GLOBAL
  permission sets, the undriven partition, packet directions and the rate
  rule, the observables `H`, `J`, `Y`, `rho`, and `E` (with `Phi_N` and
  `H_warm`) as declarations over externally supplied expectations, and the
  Xypher slot reading. It consumes no steady state.
- `src/big.rs`, `src/rat.rs`, `src/logs.rs`, `src/solve.rs`: the exact
  arithmetic of CAL-CEF-3, unchanged (see below).
- `src/model.rs` builds each world from section 1: candidate bridges in
  lexicographic order, configurations in bit order, routes `V^tau` in
  lexicographic order, `N_2(G, x)` by integer matrix products, every state
  `(G, r, t)` in canonical order, the permission sets `A(r, t)`, and every
  STEP, REPLAN, WEATHER, and FUEL channel with its source, destination,
  class, store, integer rate, and packet direction. Parallel WEATHER and FUEL
  channels stay separate. Rates come from `channel_rate(RateInputs)`, whose
  input type holds only the state, the bridge, the class, the store ratio,
  and `A(r, t)`; a compile-time assertion keeps `N_2`, `H`, `J`, `Y`, `E`,
  and `eta` out of the declared rate reads. Controls C01--C07 enter through
  named hooks.
- `src/symmetry.rs` builds the relabellings of islands `1, ..., N-1` with
  home fixed, the canonical representative (least index) of every state, the
  orbits, the lumped generator, and the lifting.
- `src/rival.rs` holds the closed forms of sections 2 and 4.2: `Phi_N`,
  `r_0`, `J_max`, `J_warm(s)`, `r_s`, `s*(J)`, `H_global`, `J_global`.
- `src/gates.rs` evaluates G01--G11 in the order of boundary section 5 and
  holds the section 4.1 and 4.2 values verbatim as rational strings.
- `src/lib.rs` runs the eighteen worlds with their agents, the controls on
  the primary witness `(3,2,8,2)`, and the hypotheses H0--H4, and renders the
  section 6 report. `evaluate_primary` runs the primary witness and the
  controls only.
- `src/main.rs` prints the report and exits 1 on `OVERALL FAIL`.
- `tests/source_contract.rs` builds only the primary witness and the
  controls: every gate passes on `(3,2,8,2)`, every control fails first
  where section 8 names, and the report has 11 gate lines and 7 control
  lines. It does not solve the other 62 confirmatory steady states. Unit
  tests cover the new helpers without any LOCAL or AIM steady state: the
  section 7 permission counts, orbit counts of `(3,1)` and `(4,1)`, `Phi_N`
  against enumeration, and the `s*`/`J_warm` algebra.

The crate has no dependencies and is not a member of the repository
workspace.

## Reused arithmetic

Boundary section 6 reuses the exact arithmetic of the CAL-CEF-3 apparatus
unchanged. The four modules here are unchanged copies of
`../xypher-open-future-price-proof/src/`, at the digests recorded in the Open
Future Price Result (section 2):

```text
09969f43da13c369235f990ae6e8242be65cde97c7112943c490ee3d5e964a63  src/big.rs
361c7baead4a52c994073dcb46d32db8dfb9e329632b6612c55c815f3534c826  src/rat.rs
1c061bdd18e0d33d8c8336cbed0c07ac9678f96a0e5603ec89de9b0a3e8f3c5c  src/logs.rs
904b0db2d5253ae5c6feaab3e4464217d1071592622a78f71a9768981d270a9a  src/solve.rs
```

They were executed in the CAL-CEF-3 arithmetic test and run (boundary
section 12). Nothing in this crate was executed for CAL-CEF-4; there is no
arithmetic exception this time.

## Report shape

```text
XYPHER_AIMED_FORESIGHT_PROOF CAL-CEF-4 primary=(3,2,8,2) worlds=18 steady_states=66 agents=LOCAL,AIM-k,GLOBAL
G01 PASS|FAIL grounding [first failure <gate> check k CODE world (N,tau,b_c,b_h) agent A configuration C|world-level] -- note|detail
...                                    (one line per gate, G01..G11)
C01 PASS|FAIL one-way-fuel observed <first failure> expected G03 check 1 G03_REVERSE_IN_CLASS agent LOCAL configuration none -- detail
...                                    (one line per control, C01..C07)
WORLD (N,tau,b_c,b_h) AGENT states=.. orbits=.. N2_p=exact ~0.0000 N2_pi_c=.. H=.. J=.. Y=.. rho=.. s*=..|none E=.. eta=[lo, hi] Y_bound=[lo, hi]
...                                    (one line per world and agent, 66 lines)
H0 SUPPORTED|REFUTED <statement> failing=none|<worlds, ladders, or pairs>
...                                    (H0..H4)
OVERALL PASS|FAIL
```

A failure is reported as `(gate, check, world, agent, configuration)`;
world-level checks have no configuration. G10 is reported as three numbered
checks. `OVERALL PASS` requires every gate to pass and every control to fail
first where section 8 names. Hypotheses never enter `OVERALL`; an undefined
value counts as a failure of the hypothesis that needs it. Exact rationals
are reduced; the `~` decimals are rounded from them. Enclosures are printed
with their lower endpoint rounded down and their upper endpoint rounded up to
twelve decimals.

## Controls

Each control mutates the primary witness `(3,2,8,2)`, which is then run with
LOCAL, AIM-1, AIM-2, and GLOBAL, in that order, through the gates until the
first failing check; nothing after it is evaluated, so no quantity dividing
by `J`, `sigma`, or `H_warm` is computed. A control passes when gate, check,
world, agent, and configuration (world-level for C03 and C07; for C03 the
recorded value `J = 0`) equal section 8.

| Control | Hook | Expected first failure |
|---|---|---|
| C01 one-way fuel | `fuel_returns_present` is false | G03 check 1, LOCAL, configuration none |
| C02 free protection | `weather_may_wear` spares the held route's bridges | G02 check 3, LOCAL, configuration `{0,1}` |
| C03 equal stores | `Mutation::constructed` sets `b_h = b_c = 8` | G08 check 1, LOCAL, world-level (`J = 0`) |
| C04 biased step | `forward_step_factor` is 2 | G03 check 2, LOCAL, configuration none |
| C05 aim off by one | `aim_depth` returns `k + 1` | G02 check 4, AIM-1, configuration none |
| C06 wrong calibration | `reference_q` returns `1/b_c` | G07 check 1, GLOBAL, configuration none |
| C07 wrong rival | `rival_hot_weight` returns `b_h` | G09 check 1, LOCAL, world-level |

## Lumping and lifting

The steady state is solved on the orbits of the relabellings of islands
`1, ..., N-1` (home fixed), which act on `(G, r, t)` through the bridges of
`G` and the islands of the route and keep the cursor. The canonical
representative of a state is the least state index in its orbit. The lumped
rate from orbit `O` to orbit `O'` is the sum over `z'` in `O'` of
`k(z -> z')` for the representative `z`; channels inside an orbit drop out
with the diagonal. `solve::stationary_vector` gives integer orbit totals
`n_O`, which are lifted to `w(z) = n_O (N-1)!/|O|`, an integer because `|O|`
divides `(N-1)!`, with `p(z) = w(z) / D` and `D = sum w`. G06 check 2 tests
the symmetry the lumping needs channel by channel, on the image bridge; G06
check 3 tests `n_O > 0`; G06 check 4 tests that `w` balances every state of
the full chain, inflow against outflow, in integers, independent of the
lumping. `H`, `J`, `rho`, `sigma`, `sigma_floor`, and G08 are read on the
full lifted chain, channel by channel; parallel WEATHER and FUEL channels
are never lumped. The largest lumped systems have 612 and 660 orbits.

## Review embargo

Before approval, boundary section 12 permits only these non-executing checks:

```text
runa check xypher.runa
cargo check --release --all-targets
cargo test --release --no-run
cargo fmt --check
cargo clippy --release --all-targets -- -D warnings
```

When `runa` is not on `PATH`, use `../../../futuruna/target/debug/runa` from
this directory.

The apparatus was written under this embargo. No evaluator, test body,
binary, or replica of the model, world, gate, or report code has been
executed in any language, and no steady state, `H`, `J`, `Y`, `E`, `rho`, or
`eta` of any LOCAL or AIM agent was computed, at any temperature. Boundary
section 9 discloses the design-time computations (the closed forms, the
state and orbit counts, and GLOBAL steady states at the non-confirmatory
pair `(4, 2)`) and the two pre-freeze referee rounds, whose LOCAL and AIM
solves at non-confirmatory pairs reported yes-or-no degeneracy flags only;
this apparatus uses neither their code nor their outputs.

After the exact source hash is approved, execute the frozen evaluation once,
in this order:

```text
cargo test --release --locked
cargo run --release --locked --quiet
```

The release profile is required. Boundary section 6 estimates about ten
minutes of exact elimination per full run, about 35 s per steady state of
the 660-orbit worlds.

## Exactness boundary

All decisions use exact integer or reduced rational arithmetic; no
floating-point value exists in the crate, and collections are `Vec` or
`BTreeMap`. Each steady state is the exact solution of the lumped
`p Q = 0`, `sum p = 1`: the last balance equation is replaced by the
normalisation, and the dense system is solved by Bareiss elimination, whose
intermediate entries are minors, so every division is exact. `J`, `rho`,
`s*`, `J_warm(s*)`, `r_(s*)`, `H_warm`, `E`, and `Y` are exact rationals;
`s*` is reported as `none` when `J >= J_max = K (b_c - b_h)/(b_h + 1)`.

Logarithms enter only `sigma_floor`, `eta`, `Y_bound`, and `eta_global`, as
in CAL-CEF-3: `ln n` for an integer `n` is enclosed by truncating `n` to a
128-bit mantissa and summing `2 atanh((x-1)/(x+1))` in fixed point with
outward rounding, endpoints rounded outward to the dyadic denominator
`2^128`. `sigma_floor = -(1/D) sum_z m_z [ln w(z) + |G(z)| ln b_c]`, with
`m_z = D (p L_undriven)(z)` on the full chain, has width far inside the
`10^-9` required by G08 checks 4 and 5. An enclosure that misses `10^-9` is a
gate failure, not a retry.

## Scope

A pass establishes, within the declared finite worlds, that the apparatus
constructs the driven worlds exactly, solves their steady states exactly,
measures `H`, `J`, `Y`, the warming rival, and the edge `E` exactly, and
encloses `sigma_floor`, `eta`, and `Y_bound` in certified intervals
(boundary section 10). The hypotheses H0--H4 then stand or fall on their own
lines, in the traveller-centred measure `N_2` and relative to the warming
rival at equal fuel flow. It does not establish anything about routes chosen
by what they open, several travellers, worlds that grow, horizons and sizes
beyond those listed, or THAIM.
