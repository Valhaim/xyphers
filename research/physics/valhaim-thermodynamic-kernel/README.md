# Valhaim thermodynamic-kernel evidence packet

Status: **exact finite kernel and contact experiment passed**

The sealed first run completed from commit `d551f7003fe52bd5949f76ac4dd58c8aa7157821`.
It establishes an exact finite equilibrium thermodynamic graph kernel and
operational contact temperature for the declared witness. PaymentLedger
ownership, a deployed stochastic clock, monetary calibration, and advanced
feature closure remain untested here.

This folder has one chronology:

```text
frozen protocol -> sealed runner -> execution
```

The frozen protocol commit is
`c3d9a5c6404c1c42aa938a400d3554a1ec670051`.

| Frozen file | SHA-256 |
|---|---|
| `PROTOCOL.md` | `a1900b99160810f2bdc517ac1cbd19e8336987c2b8cca3b6c02d89f589aff07b` |
| `artifacts/freeze-manifest.json` | `1108cd08078bf969ddf0064cf30ca551190e00886e15972d0f76776d0b16b49b` |

The freeze manifest owns constants, gate order, mutation mappings, and claim
limits. `SCHEMA.json` only defines how the first exact runner records evidence.

## First-run artifacts

The sealed runner created three scientific files:

| File | SHA-256 | Result |
|---|---|---|
| `artifacts/kernel-exact.json` | `29af31217a682a74cba31fc859566fb539254cdfe4b85a3eda7890e99f049534` | K01--K10 and X01 pass across all three fixtures |
| `artifacts/controls.json` | `65a476f23c9a043737cee149c2835e640e06a6220c95563a9dc0fc12b2ee9acc` | M01--M10 fail first at their preregistered gates |
| `artifacts/contact.json` | `f0dded9e8b790112911f1e6602890b33a1390204d0e4bba7309da8f4dbb5335e` | C01--C08 pass; equal current is zero and unequal currents reverse sign |

The claim ladder is acyclic:

```text
frozen mutations -> exact kernel -> contact temperature
```

The exact artifact hashes and requires the controls artifact. The contact
artifact hashes and requires the exact artifact. If local gates pass but an
upstream dependency does not, status is `invalidated` and no thermodynamic
claim is earned. A later calculation cannot rescue an earlier failed
falsification test.

Ledger integration and advanced-feature classification are deliberately not
specified yet. This schema can emit only a `not_run` deferred marker for them.
Each requires a small, separately reviewed schema extension sealed before its
own execution.

## Exact values

- Counts and dimensions are JSON integers.
- Scientific rationals are reduced strings `n/d`; integers use `n/1`.
- Energy and current are rational coefficients on a registered symbolic basis.
- Temperature remains the exact reservoir relation
  `alpha=epsilon/ln(r)`.
- No gate is decided by floating-point tolerance or sampled frequency.

Example:

```json
{
  "probability": "4/9",
  "current": {
    "coefficient": "-12/13",
    "basis_id": "kappa_contact*epsilon"
  }
}
```

## Compact evidence

Commit the exhaustive finite tables. They are small:

- primary: 9 states × 4 slots = 36 rows;
- ternary reservoir: 16 × 6 = 96 rows;
- three-node path: 49 × 8 = 392 rows.

Do not commit build directories, terminal transcripts, sampled trajectories,
or repeated validator logs. Later integration evidence should retain compact
canonical event fields and pre/post/finality/replay hashes, with verbose traces
reproducible from the sealed runner.

Each artifact includes:

```text
freeze identity
runner-seal identity
schema identity
relevant scientific source paths and combined hash
literal command and toolchain
gate verdicts and first failure
compact derived summary
domain payload
```

`relevant_source_clean` concerns the runner, schema, lockfile, and imported
scientific modules. Unrelated user work elsewhere in the repository is listed
separately and does not invalidate the experiment.

The summary is the later article-generation surface. It may only restate
claims earned or excluded by the same artifact's gates.

## Seal before execution

Before the first run:

1. Implement the exact enumerator, contact checks, schema and semantic
   validators, and all ten mutations without executing them.
2. Compile the final binary without running it.
3. Record the required relevant-source set, its deterministic combined hash,
   the executable hash, the build toolchain, and the lockfile hash in
   `runner-seal.json`.
4. Commit and push the runner, schema, lockfile, tests, and seal together.
5. Execute that exact binary only after the seal commit, from a clean relevant
   source set, into three absent output paths.

Any schema change after the first output is a declared deviation and a new run
identity.

The combined relevant-source hash is SHA-256 over lexicographically sorted
UTF-8 records of `path`, a NUL byte, the lowercase file SHA-256, and a newline.
Every path is repository-relative and must match the runner-seal commit.

The exact artifact binds controls, and contact binds the exact artifact, with
SHA-256 of `serde_json::to_vec(payload)`: UTF-8 compact JSON in the Rust
struct-field order sealed by this runner. This is a payload binding within
this runner version, not a general canonical-JSON format.

## Sealed execution

The runner derives all three evidence classes in memory, validates their
cross-artifact semantics and JSON Schema, and only then creates the three
absent files. The sealed invocation is:

```bash
research/physics/valhaim-thermodynamic-kernel/runner/target/debug/valhaim-thermodynamic-kernel-runner \
  --repo-root /absolute/path/to/thaim \
  --output-dir /absolute/path/to/thaim/research/physics/valhaim-thermodynamic-kernel/artifacts \
  --runner-seal /absolute/path/to/thaim/research/physics/valhaim-thermodynamic-kernel/runner-seal.json
```

Preflight rejects a changed protocol, manifest, schema, required source set,
lockfile, executable, toolchain, or relevant Git diff before deriving any
result. `build.rs` embeds the combined scientific-source hash, lockfile hash,
and build-time Rust compiler, Cargo, and target identities inside the
executable. Preflight requires exact equality among the worktree, the
runner-seal commit, the seal receipt, and the embedded build receipt.
Existing or temporary result files are never replaced.

These read-only commands check the current preregistration without running the
experiment:

```bash
shasum -a 256 research/physics/valhaim-thermodynamic-kernel/PROTOCOL.md
shasum -a 256 research/physics/valhaim-thermodynamic-kernel/artifacts/freeze-manifest.json
jq -e . research/physics/valhaim-thermodynamic-kernel/SCHEMA.json
git show --stat c3d9a5c6404c1c42aa938a400d3554a1ec670051
```
