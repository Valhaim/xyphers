# Causal Entropic Thermodynamics Proof (CAL-CEF-1)

Standalone exact verifier for the preregistered boundary
`../derivable/xypher-causal-entropic-thermodynamics-boundary.md`.

A traveller on a fixed archipelago holds one tau-step plan from home, walks
it forward and back, replans uniformly among valid plans only at home, and
builds or dismantles only a bridge at its own island that its held plan does
not use. Timber comes from a reservoir with `b^(E_R)` arrangements. No rule
reads how many futures a configuration offers. The apparatus asks whether
this world satisfies the independent operational definition T1--T6 of
`../derivable/xypher-operational-thermodynamics-boundary.md` (section 2) and
whether its equilibrium configuration law is `N_tau(G) b^(-|G|) / Z`, with the
causal entropic odds `exp[Xi/alpha]` emerging as equilibrium averages.

No result has been generated. The evaluator is under review embargo.

## Structure

- `xypher.runa` is the prospective Futuruna construction: archipelago, plan
  validity, the REPLAN channel, the local BUILD/DISMANTLE permission rule, the
  reservoir law, the THAIM pair, and Xi as the predicted configuration odds in
  multiplicative `ExactRatio` form. It consumes no stationary weight, averaged
  rate, or rate ratio.
- `src/model.rs` builds each case from the local rules: configurations in
  bit order, plans by depth-first search, `e_h^T L_G^tau 1` by integer matrix
  powers, a brute-force `Pi_tau(G)` reference, system states `(G, p, t)`,
  STEP / REPLAN / BUILD / DISMANTLE channels, lumped system-state hazards, and
  for `N = 3` explicit complete states with reservoir labels and unit-hazard
  micro channels. Controls C01--C03 and C05--C07 mutate this construction.
- `src/gates.rs` evaluates G01--G12 in the within-gate check order of
  boundary section 7 and holds the frozen section 5 values. C04 mutates the
  Harness Xi prediction there. G08 check 3 compares the computed section 5.1
  and 5.2 tables (configuration, system- and complete-state counts, `N_tau`
  range, per-configuration values) with their frozen copies.
- `src/contact.rs` builds the contact shells of section 4.5 for the identical
  pair `(3,2,2)x(3,2,2)` and the heterogeneous pair `(3,2,2)x(3,1,2)`
  (C08 prepares B of the identical pair at `b = 3`). The witness line reports
  the signed contact current of every shell separately.
- `src/ratio.rs` is the exact reduced rational type over `i128`.
- `src/lib.rs` runs the primary witness, controls C01--C08, and the
  eight-case family, and renders the section 9 report.
- `src/main.rs` prints the report and exits 1 on failure.
- `tests/source_contract.rs` applies the same evaluator as a test gate.

The crate has no dependencies and is not a member of the repository workspace.

## Review embargo

Before independent approval, only non-executing checks are permitted:

```text
runa check xypher.runa
runa fmt --check xypher.runa
cargo check --manifest-path Cargo.toml --all-targets
cargo test --manifest-path Cargo.toml --no-run
cargo fmt --manifest-path Cargo.toml -- --check
```

When `runa` is not on `PATH`, use `../../../futuruna/target/debug/runa` from
this directory.

After the exact source hash is approved, execute the frozen evaluation once:

```text
cargo test --manifest-path Cargo.toml
cargo run --quiet --manifest-path Cargo.toml
```

## Exactness boundary

All arithmetic is exact integer or reduced rational arithmetic. No
floating-point acceptance path, random seed, sampling, simulation length, or
tolerance exists. Entropies and affinities are compared multiplicatively:
`exp[THAIM_+/alpha] = max(1, N'/N)` and
`exp[Xi/alpha] = (N'/N) b^(-(|G'|-|G|))`, so no logarithm is evaluated. The
stationary law is not assumed: the candidate `b^(-U(z))` is verified against
the constructed generator by connectivity, exact global balance, and detailed
balance. For `N = 4` the reservoir lift is not enumerated (G05 N/A); all work
there is at system-state level.

Expected hazards and exit rates are written independently of the functions
that build the generator: G03 check 2 (system level), G03 check 3, and G10
check 3 recompute `b^(E_R(z'))` and each state's exit rate directly from the
section 3.4 rules, and G01 check 5 checks that every brute-force plan appears
at every cursor. G10 check 3 also relies on a structural guarantee: channel
hazards are computed from a `HazardInputs` type with no plan-count, THAIM, or
Xi field. The emergent odds of G09 check 2 and the G12 odds clause are
identities given the stationary law (boundary section 4.2); they add no test
independent of G07 and G08.

## Scope

A pass establishes, within the declared finite model: T1--T5 across the
eight-case family (reservoir lumpability verified on the explicit lift for
`N = 3`); T6 for the identical `(3,2,2)` pair and the heterogeneous
`(3,2,2)`/`(3,1,2)` pair; the configuration law `N_tau(G) exp[-U(G)/alpha]/Z`;
and equilibrium-averaged toggle odds `exp[Xi/alpha]` between configurations
with positive toggle flux, an identity given the configuration law. It does
not establish that trajectory or endpoint Shannon entropy equals `ln N_tau`, a
configuration-level rate rule, growth, non-equilibrium drive, learning,
control capacity, or new fundamental physics (boundary section 10).
