# Open Future Price Proof (CAL-CEF-3)

Standalone exact verifier for the preregistered boundary
`../derivable/xypher-open-future-price-boundary.md`, frozen at commit
`e5474cb773c3da74ce4162d20e07f37d904a945e`.

The CAL-CEF-1 traveller holds one tau-step plan from home on an archipelago.
CAL-CEF-3 drives that world out of equilibrium with two timber stores. A cold
store (ratio `b_c`) acts through WEATHER: it builds every absent bridge at
rate 1 and wears every built bridge the held plan does not use at rate `b_c`.
A hot store (ratio `b_h < b_c`) acts through FUEL: the LOCAL agent toggles
bridges incident to the traveller's island, the GLOBAL agent every bridge, at
build rate 1 and return rate `b_h`. STEP and REPLAN move the cursor and the
held plan at rate 1. For each of twelve worlds and both agents the apparatus
solves the exact steady state and measures the open future held above the
undriven world, `H = <N_2>_p - <N_2>_(pi_c)`, the fuel flow `J`, the yield
`Y = H/J`, and, through certified logarithm enclosures, the efficiency
`eta = sigma_floor / (J ln(b_c/b_h))` and the yield bound
`Y_bound = H ln(b_c/b_h) / sigma_floor`.

No result has been generated. The evaluator is under review embargo.

## Structure

- `xypher.runa` is the prospective Futuruna construction: archipelago, plan
  validity, `N_tau` and the open future `N_R` as walk counts, the two stores
  with their ratios, the four channel classes with their permission rules,
  packet directions and rate rule, the LOCAL and GLOBAL agents, the
  observables `H`, `J`, and `Y` as declarations over externally supplied
  expectations, and the Xypher slot reading. It consumes no steady state.
- `src/big.rs`: signed arbitrary-precision integers over 64-bit limbs
  (schoolbook multiplication, 2-adic exact division, binary long division,
  binary gcd, decimal conversion, and an in-place Bareiss update that reuses
  its limb buffers).
- `src/rat.rs`: reduced rationals with positive denominators, exact ordering,
  and decimal rounding (half away from zero) from exact integers.
- `src/logs.rs`: certified logarithm enclosures with dyadic endpoints.
- `src/solve.rs`: fraction-free Bareiss elimination on a dense integer
  matrix, and the integer stationary vector `n_z` with `p(z) = n_z / D`,
  `D = sum n_z`, of a generator given as `(from, to, rate)` channels.
- `src/model.rs` builds each world from the section 1 rules: candidate
  bridges in lexicographic order, configurations in bit order, plans by
  depth-first search, `N_tau` and `N_R` by integer matrix powers, system
  states `(G, p, t)` in canonical order, and every STEP, REPLAN, WEATHER, and
  FUEL channel with its source, destination, class, store, integer rate, and
  packet direction. Parallel WEATHER and FUEL channels stay separate. Rates
  come from `channel_rate(RateInputs)`, whose input type holds only the state,
  the bridge, the class, and the store ratio; a compile-time assertion keeps
  `N_tau`, `N_R`, `H`, `J`, `Y`, and `eta` out of the declared rate reads.
  Controls C01--C06 enter through named hooks.
- `src/gates.rs` evaluates G01--G10 in the order of boundary section 5 and
  holds the section 4.1 and 4.2 values verbatim as rational strings.
- `src/lib.rs` runs the twelve worlds with LOCAL then GLOBAL, the controls on
  the primary witness `(3,2,2,8,2)`, and the hypotheses H0--H3, and renders
  the section 6 report.
- `src/main.rs` prints the report and exits 1 on `OVERALL FAIL`.
- `tests/source_contract.rs` applies the same evaluator as a test gate. Unit
  tests inside `big.rs`, `rat.rs`, `logs.rs`, and `solve.rs` cover the
  arithmetic.

The crate has no dependencies and is not a member of the repository
workspace.

## Report shape

```text
XYPHER_OPEN_FUTURE_PRICE_PROOF CAL-CEF-3 primary=(3,2,2,8,2) agent=LOCAL worlds=12 agents=LOCAL,GLOBAL
G01 PASS|FAIL grounding [first failure <gate> check k CODE world (N,tau,R,b_c,b_h) agent A configuration C|world-level] -- note|detail
...                                    (one line per gate, G01..G10)
C01 PASS|FAIL one-way-fuel observed <first failure> expected G03 check 1 G03_REVERSE_IN_CLASS agent LOCAL configuration none -- detail
...                                    (one line per control, C01..C06)
WORLD (N,tau,R,b_c,b_h) LOCAL|GLOBAL states=.. N2_p=exact ~0.0000 N2_pi_c=.. H=.. J=.. Y=.. eta=[lo, hi] Y_bound=[lo, hi]
...                                    (one line per world and agent, 24 lines)
H0 SUPPORTED|REFUTED <statement> failing=none|<worlds or pairs>
...                                    (H0..H3)
OVERALL PASS|FAIL
```

A failure is reported as `(gate, check, world, agent, configuration)`; world-
level checks have no configuration. `OVERALL PASS` requires every gate to
pass and every control to fail first where section 8 names. Hypotheses never
enter `OVERALL`. Exact rationals are reduced; the `~` decimals are rounded from
them. Enclosures are printed with their lower endpoint rounded down and their
upper endpoint rounded up to twelve decimals.

## Controls

Each control mutates the primary witness `(3,2,2,8,2)`, which is then run
with LOCAL and GLOBAL through the gates until the first failing check;
nothing after it is evaluated, so no quantity dividing by `J` or `sigma` is
computed. A control passes when gate, check, world, agent, and configuration
(or, for C03, the recorded `J = 0`) equal section 8; C01--C05 are expected at
the LOCAL agent, the primary witness's agent, and C06 at GLOBAL.

| Control | Hook | Expected first failure |
|---|---|---|
| C01 one-way fuel | `fuel_returns_present` is false | G03 check 1 at configuration none |
| C02 plan-breaking weather | `weather_may_wear` ignores plan use | G02 check 1 at configuration `{0,1}` |
| C03 equal stores | `Mutation::constructed` sets `b_h = b_c = 8` | G08 check 1 with `J = 0` |
| C04 biased weather | `bias_first_weather_build` doubles one rate | G03 check 2 at configuration none |
| C05 wrong store | `fuel_ratio` returns `b_c` | G03 check 2 at configuration none |
| C06 wrong calibration | `reference_q` returns `1/b_c` | G07 check 1, GLOBAL, configuration none |

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
binary, or replica of the dynamics has been executed, and no steady state,
`H`, `J`, `Y`, or `eta` of any agent was computed; in particular no steady
state of a LOCAL agent at a confirmatory temperature pair. One exception is
disclosed: the generic arithmetic of `src/big.rs`, `src/rat.rs`,
`src/logs.rs`, and `src/solve.rs`, copied without any model, world, gate, or
report code into a throwaway crate, was compared against Python integers,
fractions, and mpmath (integer and rational operations, logarithm
enclosures, Bareiss on random small integer matrices, and stationary vectors
of random rate matrices unrelated to the archipelago). Boundary section 9
discloses the design-time pilot and the pre-freeze referee; this apparatus
uses neither their code nor their outputs.

After the exact source hash is approved, execute the frozen evaluation once,
in this order:

```text
cargo test --release --locked
cargo run --release --locked --quiet
```

The release profile is required: exact elimination over 344 states is slow
without optimisation.

## Exactness boundary

All decisions use exact integer or reduced rational arithmetic; no
floating-point value exists in the crate. Each steady state is the exact
solution of `p Q = 0`, `sum p = 1`: the last balance equation of `Q^T` is
replaced by the normalisation, and the dense system is solved by Bareiss
elimination, whose intermediate entries are minors, so every division is
exact. Back substitution gives `n_z = det x_z` in integers; G06 checks
positivity and the balance of every state exactly.

Logarithms enter only `sigma_floor`, `eta`, `Y_bound`, and `eta_global`.
`ln n` for an integer `n` is enclosed by truncating `n` to a 128-bit mantissa,
`n = m 2^k`, so `ln n` lies in `[k ln 2 + ln m_lo, k ln 2 + ln m_hi]` with
`m_hi <= m_lo + 1`; `ln x` for `x` in `[1, 2]` is `2 atanh((x-1)/(x+1))`,
summed in fixed point with 192 fractional bits, every lower bound rounded
down and every upper bound rounded up, plus the geometric remainder bound
`u^(2J+1) / ((2J+1)(1-u^2))`. Endpoints are rounded outward to the dyadic
denominator `2^128`. Each enclosure has width below about `2^-126`, so
`sigma_floor = -(1/D) sum_z m_z [ln n_z + |G(z)| ln b_c]`, with
`|m_z|/D` summing to at most twice the largest undriven exit rate (at most
`2 * 52` in these worlds),
has width below `10^-30`, far inside the `10^-9` required by G08 checks 4
and 5. An enclosure that misses `10^-9` is a gate failure, not a retry.

## Scope

A pass establishes, within the declared finite worlds, that the apparatus
constructs the driven worlds exactly, solves their steady states exactly, and
measures `H`, `J`, `Y`, `sigma`, and the price floor correctly (boundary
section 10). The hypotheses H0--H3 then stand or fall on their own lines. It
does not establish anything about worlds that grow new islands, agents that
learn, several travellers, the least fuel flow attainable with a hot store of
fixed `b_h`, or THAIM.
