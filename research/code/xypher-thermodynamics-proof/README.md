# Minimal Thermodynamic Xypher Proof

Standalone exact verifier for the boundary frozen at
`8c719c7f33da5fe9695358cf6b84fb57e1f71809` in the
[operational thermodynamics boundary](https://github.com/OneManMobile/thaim/blob/c0e32bc1fc787c5322df07422a183a4977b20857/research/physics/derivable/xypher-operational-thermodynamics-boundary.md).

The apparatus answers only two questions:

1. Does the frozen 16-state Crystal + Thermo + Praxion kernel instantiate an
   equilibrium thermodynamic digital graph system?
2. Do the prospective Xypher construction and independent microscopic
   enumeration produce exactly the same dynamics?

Ruby is empty. Memory is one-state and non-adaptive. Growing graphs, richer
Praxions, blockchain mechanics, and consciousness are outside this crate.

The frozen apparatus passed all eleven gates and all four falsifying controls.
See the
[operational thermodynamics result](https://github.com/OneManMobile/thaim/blob/c0e32bc1fc787c5322df07422a183a4977b20857/research/physics/derivable/xypher-operational-thermodynamics-result.md)
for the exact output, provenance, and bounded scientific verdict.

## Structure

- `xypher.runa` is the prospective Futuruna dataflow. It constructs Crystal
  entropy, the forward/reverse TAU pair, signed Xi, symmetric activity, and
  squared Praxion hazards without consuming rates or stationary weights.
- `src/lib.rs` independently enumerates the complete state, exact generators,
  macro quotient, contact shell, and frozen controls.
- `src/main.rs` prints the compact deterministic evaluation report.
- `tests/source_contract.rs` applies the same evaluator as a test gate.

The Rust crate has no dependencies and is not a member of the repository
workspace.

## Review embargo

Before independent source review, only nonexecuting checks are permitted:

```text
runa check xypher.runa
runa fmt --check xypher.runa
cargo check --manifest-path Cargo.toml
cargo test --manifest-path Cargo.toml --no-run
```

When `runa` is not installed on `PATH`, skip the two Futuruna checks. The Rust
verifier remains independently runnable, and `xypher.runa` remains available
for inspection. The compiler source lives in the
[THAIM repository](https://github.com/OneManMobile/thaim/tree/c0e32bc1fc787c5322df07422a183a4977b20857/futuruna).

After the exact source hash is approved, execute the frozen evaluation once:

```text
cargo test --manifest-path Cargo.toml
cargo run --quiet --manifest-path Cargo.toml
```

No random seed, simulation length, burn-in, tolerance, network service, or
large artifact is involved. The report is a few lines of text.

## Exactness boundary

Entropy is represented in binary units. The reservoir law
`S_R(E)=E ln 2` gives `alpha ln 2 = 1` energy unit, so TAU and Xi are exact
integers. Exponentials and square roots are never floating-point acceptance
gates: the verifier checks rational affinity ratios and squared-rate
identities by integer cross-multiplication.

A complete pass establishes constructive existence for the frozen witness.
Generality comes separately from the reviewed representation proof in the
boundary document; the executable witness does not prove that theorem by
enumeration. Together they cover the minimal kernel within the declared finite
reciprocal equilibrium class. They do not establish that every legacy Xypher
or any deployed payment network is thermodynamic, nor do they constitute a new
fundamental law of matter.
