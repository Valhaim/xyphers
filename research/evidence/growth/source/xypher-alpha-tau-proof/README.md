# CAL-ALPHA-0 exact verifier

Standalone, dependency-free Rust source for the boundary frozen at commit
`262cb0c076c78c7cc381fd9db00db563bf341b3b`:

`../derivable/xypher-alpha-tau-constitutive-boundary.md`

The evaluator is deliberately narrow. It tests the frozen `2x2x2`
confirmatory matrix, all thirteen ordered admission gates, and all nineteen
typed controls. It uses reduced `i128` ratios, exact prime-exponent logarithms,
and symbolic temperatures. It contains no floating-point values, tolerance,
fit, random seed, simulation horizon, network service, or external crate.

The inherited `b=2`, `lambda=1` CAL-XTHERM row is printed as calibration
metadata and is explicitly not CAL-ALPHA-0 evaluation data. The eight new
cases are the cross product of `b in {3,5}`, `lambda in {1,3}`, and the two
frozen body-degeneracy vectors.

## Source-review and commit embargo

This directory is prospective apparatus source. Do not run the evaluator or
its tests until all of the following are true:

1. an independent reviewer has inspected the exact arithmetic, microscopic
   enumeration, labelled-channel classifier, gate ordering, controls, and
   report contract;
2. every accepted correction has been made;
3. the reviewed source is committed and its commit/hash is recorded;
4. that reviewed commit is frozen before the first confirmatory execution.

Compilation-only inspection may be authorized separately, but it is not a
confirmatory run. After the embargo is lifted, the first execution must print
every gate and every control, including any failure, without modifying the
source in response to its result.

Static checks for the independent reviewer, when authorized:

```text
cargo fmt --manifest-path Cargo.toml -- --check
cargo check --manifest-path Cargo.toml --all-targets
cargo test --manifest-path Cargo.toml --no-run
```

Post-embargo confirmatory commands:

```text
cargo test --manifest-path Cargo.toml
cargo run --quiet --manifest-path Cargo.toml
```

A control line marked `PASS` means the evaluator produced the preregistered
classification or rejected the deliberately bad assertion at its expected
first applicable gate. It does not mean that the bad assertion passed.

## Independent paths

`G03_MICRO_LUMPING` lazily enumerates every complete-bipartite unit microedge
from each microscopic origin and separately compares the lumped hazard with
the prospective `N_y` formula. `G04_LDB_CLASSIFICATION` consumes the hazards
from that enumerator, keeps the four reciprocal mechanisms labelled, derives
`T_LDB` from their ratios, and only then compares it with the reservoir value
predeclared by `G02_RESERVOIR_SLOPE`.

`src/exact.rs` owns exact arithmetic. `src/fixtures.rs` owns only frozen input
data and control routing. `src/lib.rs` owns evaluation. `xypher.runa` is a
short prospective typed dataflow declaration; it does not calculate a
floating logarithm, execute a gate, or assert `PASS`.

## Xypher review scope

This is constitutive-identity apparatus, not a new full adaptive Xypher:

- Diamond/Crystal: present as the independently declared `S_tau=ln(g)` body
  readback.
- Thermo alpha source: present as the finite-reservoir relation
  `T_R=lambda/ln(b)`.
- Action: present as reciprocal labelled unit microedges and their exact
  macro lumping.
- Ruby: not applicable to this narrow equilibrium construction.
- Opal/Phi: not applicable; no integration or consciousness claim is made.
- Adaptive gate `a_P`: classified only as a gate statistic; it is not used as
  temperature.
- Learning signal and adaptive memory: not applicable; no learning claim is
  made.

These N/A slots are architectural scope statements, not failed thermodynamic
requirements. The finite verifier supplies constructive witnesses only. The
general C1--C9 statements remain mathematical results under their written
assumptions, and neither source nor a future pass establishes native
blockchain thermodynamics, joule calibration, universal scaling,
intelligence, consciousness, or new fundamental physics. It also does not
establish endogenous full-Xypher temperature, identify TAU as heat or work,
prove self-power or Landauer optimality, supply a new contact result, or close
the broader CAL-SCALE, CAL-NESS, or CAL-DRIVE programs.
