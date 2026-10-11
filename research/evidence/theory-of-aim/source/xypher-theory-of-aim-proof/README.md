# Theory of Aim Proof (CAL-CEF-5)

Standalone exact verifier for the preregistered boundary
`../derivable/xypher-theory-of-aim-boundary.md`, frozen at commit
`665da9b276bb0f3c5b89a21c06500502aa39ae44`. That boundary extends the
CAL-CEF-4 boundary `../derivable/xypher-aimed-foresight-boundary.md` for
everything it leaves unchanged, and this crate extends the CAL-CEF-4
apparatus `../xypher-aimed-foresight-proof/`.

CAL-CEF-5 asks whether a theory can predict, in advance and in numbers, how
much open future an agent's energy buys. The world is that of CAL-CEF-4: a
traveller holds a route `(v_1, ..., v_tau)` in `V^tau` from home `0` on an
archipelago of `N` islands; WEATHER (cold store, ratio `b_c`) builds every
absent bridge at rate 1 and wears every built bridge at rate `b_c`,
unconditionally; FUEL (hot store, ratio `b_h`) builds at rate 1 and returns
at rate `b_h` on the agent's permission set `A(r, t)` (LOCAL, two-sided
AIM-k, GLOBAL). Two parameters are added:

- the speed `v`: STEP runs at rate `v` each way and REPLAN at rate `v` to
  each other route;
- the measure radius `R`: the open future is
  `N_R(G, x) = e_x^T (I + A_G)^R 1`, the `R`-step walks from the
  traveller's island.

For twenty worlds and 76 steady states the apparatus measures
`H = <N_R(G, x)>_p - <N_R(G, x)>_(pi_c)`, the fuel flow `J`, `Y = H/J`,
`rho = <|G|>_p / K`, the rival's inverse `s*(J)`, the edge
`E = H / H_warm` with `H_warm = Phi_{N,R}(rho) - Phi_{N,R}(r_0)`, and,
through certified logarithm enclosures, `eta` and `Y_bound`. It compares
each of the 56 LOCAL and AIM edges with the theory value `E_th` frozen in
boundary section 4.3, and decides H1--H5.

No result has been generated. The evaluator is under review embargo.

## Structure

- `xypher.runa` is the prospective Futuruna construction: the CAL-CEF-4
  declaration with the archipelago carrying `v` and `R`, STEP and REPLAN at
  rate `v` in the rate rule, `N_R` as an `R`-step walk count from the
  traveller's island, `Phi_{N,R}` by walk enumeration on the complete graph,
  and the simple law `L(N, rho)` of H5. It holds no theory value and
  consumes no steady state.
- `src/big.rs`, `src/rat.rs`, `src/logs.rs`, `src/solve.rs`: the exact
  arithmetic of CAL-CEF-3 and CAL-CEF-4, unchanged (see below).
- `src/model.rs` builds each world `(N, tau, b_c, b_h, v, R)`: candidate
  bridges, configurations, routes, `N_R(G, x)` by the `R`-th integer matrix
  power of `I + A_G`, every state `(G, r, t)`, the permission sets, and every
  channel. Rates come from `channel_rate(RateInputs)`, whose input type holds
  only the state, the bridge, the class, the store ratio, `A(r, t)`, and the
  speed given to STEP and REPLAN; a compile-time assertion keeps `N_R`, `H`,
  `J`, `Y`, `E`, `eta`, and every theory value out of the declared rate
  reads. Controls C01--C08 enter through named hooks.
- `src/symmetry.rs`: relabellings of islands `1, ..., N-1` with home fixed,
  orbits, the lumped generator, and the lifting (unchanged apart from its
  unit tests).
- `src/rival.rs`: `Phi_{N,R}(r)` by exact enumeration of the `R`-step walks
  from home on the complete graph, `sum_w r^b(w)` with `b(w)` the number of
  distinct bridges crossed; `r_0`, `J_max`, `J_warm(s)`, `r_s`, `s*(J)`,
  `H_global`, `J_global`, and `L(N, rho)`.
- `src/theory.rs`: the 56 lines of boundary section 4.3, embedded verbatim
  as one string (lines joined by `\n`, with a trailing `\n`; the same bytes
  as `../xypher-agency-theory/predictions_cef5.txt`), the frozen SHA-256, the
  parser (world, agent, `E_th` as an exact rational with six decimals), and
  the C09 corruption.
- `src/sha256.rs`: a dependency-free SHA-256 (FIPS 180-4), used only by G12
  check 1.
- `src/gates.rs` evaluates G01--G12 in the order of boundary section 5 and
  holds the section 4.1 and 4.2 tables verbatim as rational strings.
- `src/lib.rs` runs the twenty worlds with their agents, the controls on the
  primary witness `(3,2,5,2,v=4,R=2)`, and the hypotheses H1--H5, and
  renders the section 6 report. `evaluate_primary` runs the primary witness
  and the controls only.
- `src/main.rs` prints the report and exits 1 on `OVERALL FAIL`.
- `tests/source_contract.rs` solves only the primary witness and its
  controls: every gate passes on `(3,2,5,2,v=4,R=2)`, every control fails
  first where section 8 names, and the report has 12 gate lines, 9 control
  lines, and 4 world lines. It does not solve the other 72 steady states.
  Unit tests cover the new helpers without any LOCAL or AIM steady state:
  `Phi_{N,R}` against the closed forms `Phi_{N,2}(r) = 1 + 3(N-1) r +
  (N-1)(N-2) r^2` and `Phi_{3,3}(r) = 1 + 14 r + 10 r^2 + 2 r^3` and against
  enumeration over configurations, SHA-256 known vectors, the theory-table
  parse against the section 3 keys, the section 7 permission counts, and
  orbit counts of `(3,1)` and `(4,1)`.

The crate has no dependencies and is not a member of the repository
workspace.

## What changed from CAL-CEF-4

| Boundary item | Code |
|---|---|
| Speed `v` (section 1) | `WorldSpec::speed`; `RateInputs::speed`, `channel_rate` (STEP, REPLAN at `v`); `Model::class_speed` |
| Radius `R` (section 1) | `WorldSpec::radius`; `open_future_by_matrix(.., radius)` |
| `Phi_{N,R}` (section 2) | `rival::walk_polynomial`, `rival::phi` |
| Worlds and order (section 3) | `model::PAIRS`, `model::WORLDS`, `model::PRIMARY` |
| G01 check 2 for the world's `R` | `gates::gate_01`, `walks_by_enumeration` |
| G03 check 2, STEP and REPLAN by ratio `1 : 1` | `gates::gate_03`, `declared_toggle_rates` |
| G09 check 1 for LOCAL and AIM, `J < J_max` required | `gates::gate_09` |
| G09 check 3 (new) | `gates::gate_09`, `independent_bridge_mean` |
| G10 check 3 with the declared `v` | `gates::rule_rate` |
| G11 tables (sections 4.1, 4.2) | `FROZEN_WORLDS`, `FROZEN_UNDRIVEN`, `FROZEN_GLOBAL`, `FROZEN_ETA_GLOBAL` |
| G12 (new, table-level) | `gates::gate_12`, `theory`, `sha256` |
| C08, C09 | `Mutation::SlowReplanning`, `Mutation::CorruptedTheory`; `lib::CONTROLS` |
| H1--H5 (section 7) | `lib::hypotheses` |
| Report (section 6) | `lib::world_line`, `ProofReport::render` |

## Reused arithmetic

The four arithmetic modules are unchanged copies of the CAL-CEF-3 and
CAL-CEF-4 apparatus files, at the same digests:

```text
09969f43da13c369235f990ae6e8242be65cde97c7112943c490ee3d5e964a63  src/big.rs
361c7baead4a52c994073dcb46d32db8dfb9e329632b6612c55c815f3534c826  src/rat.rs
1c061bdd18e0d33d8c8336cbed0c07ac9678f96a0e5603ec89de9b0a3e8f3c5c  src/logs.rs
904b0db2d5253ae5c6feaab3e4464217d1071592622a78f71a9768981d270a9a  src/solve.rs
```

They were executed in the CAL-CEF-3 arithmetic test and in the CAL-CEF-3
and CAL-CEF-4 runs.

## The SHA-256 exception

`src/sha256.rs` is new. Before approval it was executed once, alone, in a
throwaway crate outside the repository (`/tmp`), which contained only a copy
of `src/sha256.rs` and a `main` printing the digest of each file named on
its command line. Its digests of 144 generic byte strings (random bytes of
every length 0 to 129 and of lengths 183, 191, 192, 193, 447, 448, 1000,
4096, 65535, and 1000000; `abc`; the 448-bit NIST message; one million
`a`; and a sentence) equalled `shasum -a 256` on the same files. The
embedded theory table was not among them. No other code of this crate was
part of that crate, and the throwaway crate was deleted afterwards. The
module tested has SHA-256
`202ffa1f50c868fbf060b96e19c9cb3e77b4e50fe47352efe4198a78679dfc0d`.

The embedded table itself was checked without executing the apparatus: the
string literal was extracted from `src/theory.rs` by a text script and
compared with `shasum -a 256`, giving the frozen
`27afb36675ece665b0b77dffc18d27a6bdfa2a42193d41bd9c26c3b1ac7a9fc5`, the
digest of `predictions_cef5.txt` and of the section 4.3 block of the frozen
boundary.

## Report shape

```text
XYPHER_THEORY_OF_AIM_PROOF CAL-CEF-5 primary=(3,2,5,2,v=4,R=2) worlds=20 steady_states=76 agents=LOCAL,AIM-k,GLOBAL
G01 PASS|FAIL grounding [first failure <gate> check k CODE world W agent A configuration C|world-level|table-level] -- note|detail
...                                    (one line per gate, G01..G12)
C01 PASS|FAIL one-way-fuel observed <first failure> expected G03 check 1 G03_REVERSE_IN_CLASS agent LOCAL configuration none -- detail
...                                    (one line per control, C01..C09)
WORLD (N,tau,b_c,b_h,v=V,R=R) AGENT states=.. orbits=.. NR_p=exact ~0.0000 NR_pi_c=.. H=.. J=.. Y=.. rho=.. s*=..|none E=.. E_th=0.000000|none E/E_th=exact ~0.000000|none eta=[lo, hi] Y_bound=[lo, hi]
...                                    (one line per world and agent, 76 lines)
H1 SUPPORTED|REFUTED <statement> failing=none|<items>
...                                    (H1..H5)
OVERALL PASS|FAIL
```

A failure is reported as `(gate, check, world, agent, configuration)`;
world-level checks have no configuration, and the table-level checks of G12
have no world, agent, or configuration. `OVERALL PASS` requires every gate to
pass and every control to fail first where section 8 names. Hypotheses never
enter `OVERALL`; an undefined value counts as a failure of the hypothesis
that needs it, and every failing item is listed. Exact rationals are reduced;
the `~` decimals are rounded half away from zero. Enclosures are printed with
their lower endpoint rounded down and their upper endpoint rounded up to
twelve decimals.

## Controls

Each control mutates the primary witness `(3,2,5,2,v=4,R=2)`, which is then
run with LOCAL, AIM-1, AIM-2, and GLOBAL, in that order, through the gates
until the first failing check; nothing after it is evaluated, so no quantity
dividing by `J`, `sigma`, or `H_warm` is computed. A control passes when
gate, check, world, agent, and configuration equal section 8 (world-level for
C03 and C07, with the recorded value `J = 0` for C03; table-level, with no
world or agent, for C09).

| Control | Hook | Expected first failure |
|---|---|---|
| C01 one-way fuel | `fuel_returns_present` is false | G03 check 1, LOCAL, configuration none |
| C02 free protection | `weather_may_wear` spares the held route's bridges | G02 check 3, LOCAL, configuration `{0,1}` |
| C03 equal stores | `Mutation::constructed` sets `b_h = b_c = 5` | G08 check 1, LOCAL, world-level (`J = 0`) |
| C04 biased step | `forward_step_factor` is 2 (forward STEP at `2v`) | G03 check 2, LOCAL, configuration none |
| C05 aim off by one | `aim_depth` returns `k + 1` | G02 check 4, AIM-1, configuration none |
| C06 wrong calibration | `reference_q` returns `1/b_c` | G07 check 1, GLOBAL, configuration none |
| C07 wrong rival | `rival_hot_weight` returns `b_h` | G09 check 1, LOCAL, world-level |
| C08 slow replanning | `class_speed` gives REPLAN rate 1 | G10 check 3, LOCAL, configuration none |
| C09 corrupted theory | `theory::embedded_table(true)`: first `E_th` 1.390775 becomes 1.390776 | G12 check 1, table-level |

## Review embargo

Before approval, boundary section 12 permits only non-executing checks. The
checks run on this crate were exactly:

```text
runa check xypher.runa
runa fmt --check xypher.runa
cargo check --release --all-targets
cargo test --release --no-run
cargo fmt --check
cargo clippy --release --all-targets -- -D warnings
cargo generate-lockfile
```

When `runa` is not on `PATH`, use `../../../futuruna/target/debug/runa` from
this directory.

The apparatus was written under this embargo. No evaluator, test body,
binary, or replica of the model, world, gate, or report code has been
executed in any language, and no steady state, `H`, `J`, `Y`, `E`, or `rho`
of any LOCAL or AIM agent was computed at any CAL-CEF-5 world. The only
code of this crate that was executed is `src/sha256.rs`, alone, as described
above.

Boundary section 9 discloses that the theory was built after the CAL-CEF-4
run from its exact results at `(3, 2)`, `(8, 2)`, and `(6, 3)` with `v = 1`
and `R = 2`, with no fitted parameter; that every CAL-CEF-5 world differs
from every CAL-CEF-4 world in its pair of stores, and worlds 2, 3, 5, 6,
and 10 also in speed or radius; that before the freeze the designer
computed the theory's predictions of section 4.3 in floating point, the
closed forms of section 4, and the theory at other speeds, radii, replanning
rates, and asymmetric step rates (summarised qualitatively only); that the
direction and the 3% tolerance of H5 were set from the CAL-CEF-4 exact LOCAL
edges; that no exact steady state of any LOCAL or AIM agent was computed at
any CAL-CEF-5 world, by the designer or the referee, before the freeze; and
that the pre-freeze referee solved LOCAL and AIM agents exactly only at pairs
outside `(5, 2)`, `(12, 3)`, and every CAL-CEF-4 pair, for degeneracy checks,
reporting no value, accuracy, or direction. This apparatus uses neither the
theory's code nor the referee's code or outputs; it reads only the frozen
decimals of section 4.3.

After the exact source hash is approved, execute the frozen evaluation once,
in this order:

```text
cargo test --release --locked
cargo run --release --locked --quiet
```

The release profile is required. Boundary section 6 estimates 13 to 16
minutes of exact elimination per full run (612-orbit worlds about 18 to 31 s
per steady state, growing with `v`; 660-orbit worlds 25 to 33 s).

## Exactness boundary

All decisions use exact integer or reduced rational arithmetic; no
floating-point value exists in the crate, and collections are `Vec` or
`BTreeMap`. Each steady state is the exact solution of the lumped
`p Q = 0`, `sum p = 1`, by Bareiss elimination, lifted to the full chain and
verified there by G06 check 4. `J`, `rho`, `s*`, `J_warm(s*)`, `r_(s*)`,
`Phi_{N,R}`, `H_warm`, `E`, `Y`, `E_th`, `E / E_th`, and `L(N, rho)` are
exact rationals; each `E_th` is its six-decimal string read as
`m / 10^6`. Logarithms enter only `sigma_floor`, `eta`, `Y_bound`, and
`eta_global`, as certified enclosures of width at most `10^-9`; an enclosure
that misses `10^-9` is a gate failure, not a retry.

## Scope

A pass establishes, within the declared finite worlds, that the apparatus
builds the declared worlds exactly, solves their steady states exactly,
measures `H`, `J`, `Y`, `rho`, the rival, and `E` exactly, and reads the
frozen theory correctly (boundary section 10). The hypotheses H1--H5 then
stand or fall on their own lines. It does not establish anything about
worlds larger than five islands, routes chosen by what they open
(CAL-CEF-6), several travellers, or THAIM.
