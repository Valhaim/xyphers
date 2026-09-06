# VAL-KERNEL exact runner

This is the deliberately small verifier for the frozen VAL-KERNEL experiment.
It enumerates the finite microscopic law directly, derives its macro kernels,
runs the ten frozen mutations, and performs the exact contact calculation.

The runner has no dependency on Valhaim, the earlier calorimeter apparatus, or
floating-point numerical libraries. `jsonschema` is used only to validate the
evidence contract before publication; every scientific quantity is exact
integer or rational arithmetic.

Its first execution is permitted only after the final binary has been built,
its hash and build receipt have been recorded in `runner-seal.json`, and that
seal, this crate, its lockfile, tests, and `SCHEMA.json` have been committed
together. Run the exact hashed binary directly; do not relink between sealing
and execution.

`source-set.txt` is the closed scientific source inventory. `build.rs` embeds
hashes of that set and `Cargo.lock`, together with the build-time Rust
compiler, Cargo, and target identities. Runtime preflight compares those
receipts with the current files, the seal, and the committed tree that
introduced `runner-seal.json`.

The sealed command is documented in the experiment-level `README.md`.
