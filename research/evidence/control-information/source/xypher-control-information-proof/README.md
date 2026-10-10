# Control Information Thermodynamics Proof (CAL-CEF-2)

Standalone exact verifier for the preregistered boundary
`../derivable/xypher-control-information-thermodynamics-boundary.md`, frozen
at commit `79d344024eb0169d4b458213936fc5cec8c476cd`.

The CAL-CEF-1 traveller holds one tau-step plan from home on an archipelago
whose timber comes from a reservoir with `b^(E_R)` arrangements. CAL-CEF-2
adds two registers to the same complete state: a notebook `y` that MEASURE
can copy the held plan's destination `d(p)` into, and a stockpile of `w <= W`
timber packets. HARVEST trades a correct record for one stored packet by
releasing the held plan to every valid plan; COMMIT is its reverse. ERASE
wipes a written notebook against one stockpile packet paid into the
reservoir; SCRIBBLE is its reverse. The apparatus asks whether this
instrumented world satisfies T1--T5 of
`../derivable/xypher-operational-thermodynamics-boundary.md` (section 2),
whether a correct record trades for a stored packet at the exchange constant
`K(G, y) = N_tau(G) / [n_y(G) b]`, whether the free-energy gap to a shuffled
pairing of equal energy is `alpha I(G)` (and `alpha I*(G)` for the capacity
witness), and whether erasure costs the Landauer price `b/A`.

No result has been generated. The evaluator is under review embargo.

## Structure

- `xypher.runa` is the prospective Futuruna construction: archipelago, plan
  validity, the notebook and stockpile registers, the permission rule of
  every channel (STEP, REPLAN, BUILD/DISMANTLE, MEASURE/UNMEASURE,
  HARVEST/COMMIT, ERASE/SCRIBBLE), the reservoir law with `E_tot = K + W`,
  the lumped hazard rule, and the predicted exchange constants
  `K = N/(n_y b)` and erasure ratio `b/A` as `ExactRatio`. It consumes no
  stationary weight, averaged rate, or rate ratio.
- `src/model.rs` builds each case from the local rules: configurations in
  bit order, plans by depth-first search, `e_h^T L_G^tau 1` and
  `e_h^T L_G^tau e_y` by integer matrix powers, a brute-force `Pi_tau(G)`
  reference, system states `(G, p, t, y, w)` in canonical order, every
  section 3.3 channel, lumped system-state hazards `b^(E_R(z'))`, and for
  `N = 3` explicit complete states with reservoir labels and unit-hazard
  micro channels. Controls C01--C07 and C09 mutate this construction through
  named hooks; the C08 hook replaces the shuffled record marginal.
- `src/gates.rs` propagates the stationary law from the generator along a
  breadth-first spanning tree (`pi(z0) = 1`), evaluates G01--G14 in the
  within-gate check order of boundary section 7, and holds the frozen
  section 5.1 and 5.2 values. Macrostates `C`, `L`, `R`, `B` and the laws
  `A_G`, `B_G`, `A*_G`, `B*_G` are computed from the propagated law.
- `src/primes.rs` represents logarithms exactly: prime-exponent vectors for
  products such as `N^N / prod_y n_y^(n_y)`, and entropies as
  rational-coefficient vectors over `ln(prime)`.
- `src/ratio.rs` is the exact reduced rational type over `i128` (the
  CAL-CEF-1 type with numerator/denominator accessors; the unused `max` is
  dropped).
- `src/lib.rs` runs the primary witness `(3,2,2,2)`, controls C01--C09, and
  the seven-case family, and renders the section 9 report.
- `src/main.rs` prints the report and exits 1 on failure.
- `tests/source_contract.rs` applies the same evaluator as a test gate.

The crate has no dependencies and is not a member of the repository
workspace.

## Report shape

```text
XYPHER_CONTROL_INFORMATION_PROOF CAL-CEF-2 primary=(3,2,2,2)
G01 PASS|FAIL Crystal and command grounding [check k CODE at <configuration>] [-- detail]
...                                                   (one line per gate, G01..G14)
C01 PASS|FAIL one-way-harvest observed <gate> check k CODE at <configuration> expected G03 check 1 G03_RECIPROCAL_SUPPORT at none [-- detail]
...                                                   (one line per control, C01..C09)
WITNESS <configuration> N_2=.. n=.. |D|=.. K=.. exp[N I]=.. exp[|D| I*]=..
...                                                   (one line per primary configuration)
INSTRUMENT (3,2,2,2) erasure_ratio=.. mean_stockpile=.. blank=.. correct_fraction=..
FAMILY (N,tau,b,W) configurations=.. system=.. complete=.. Z=.. mean_bridges=.. erasure=.. mean_stockpile=.. blank=.. K1=[..] gates PASS=.. FAIL=.. N/A=..
...                                                   (one line per family case)
OVERALL PASS|FAIL
```

`OVERALL PASS` requires every primary gate to pass (N/A is allowed only for
G05 at `N = 4`), every control to fail first at exactly the gate, check, and
configuration named in boundary section 8 (C09 additionally at mean
stockpile `11/15`), and every family case to pass every applicable gate.

## Review embargo

Before independent approval, only non-executing checks are permitted:

```text
runa check xypher.runa
runa fmt --check xypher.runa
cargo check --manifest-path Cargo.toml --all-targets
cargo test --manifest-path Cargo.toml --no-run
cargo fmt --manifest-path Cargo.toml -- --check
cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings
```

When `runa` is not on `PATH`, use `../../../futuruna/target/debug/runa` from
this directory.

The apparatus was written under this embargo: no evaluator, test body,
binary, or replica of the dynamics has been executed, and no output of the
dynamics has been inspected. After the exact source hash is approved,
execute the frozen evaluation once, in this order:

```text
cargo test --locked
cargo run --locked --quiet
```

## Exactness boundary

All arithmetic is exact integer or reduced rational arithmetic. No
floating-point acceptance path, random seed, sampling, simulation length, or
tolerance exists, and no logarithm is evaluated. The stationary law is not
assumed: it is propagated from the constructed system generator, compared
with `b^(-(|G| + w))` by G07 check 2, and checked for detailed balance on
every channel by G07 check 3. For `N = 4` the reservoir lift is not
enumerated (G05 N/A) and G03 is checked at system-state level.

Expected hazards and exit rates are written independently of the functions
that build the generator: G03 check 2 (system level), G03 check 3, G05, and
G14 check 3 recompute `b^(E_R(z'))` and each state's exit rate directly from
the section 3.3 rules. Channel hazards are computed from a `HazardInputs`
type holding only the source and destination reservoir energies and `b`.
G04 check 1, G06, G07 check 3, G08 checks 1--4, G10 checks 2--5, G11,
G12 checks 2--3, and G13 are consistency checks given G03, G05, G07, and the
declared representation (boundary section 7, classification).

## Scope

A pass establishes, within the declared finite model, the four statements of
boundary section 10. It does not establish net work extraction or an engine,
information about realized trajectories, the capacity value from the
feedback exchange where destinations are unequally reachable, T6 for the
instrumented world, noisy records, record-only controllers, learning, or any
statement about THAIM.
