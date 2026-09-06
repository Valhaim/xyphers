XYPHER THERMODYNAMICS PROOF — QUICK START

What this archive contains

This is a small, standalone verifier for one exact digital thermodynamic graph.
It contains sixteen complete graph-plus-reservoir states. The program enumerates
every state and transition, then checks two independent temperature readings,
energy closure, equilibrium, same-temperature contact, and four deliberately
broken controls.

What you need

- Rust and Cargo: https://www.rust-lang.org/tools/install
- No network service, random seed, dataset, or third-party Rust package

How to run it

1. Unzip the archive.
2. Open a terminal in the xypher-thermodynamics-proof-v1 folder.
3. Run:

   cargo test
   cargo run --quiet

What success looks like

The test command reports one passing integration test. The run command prints
eleven gate PASS lines, four control PASS lines, and a final OVERALL PASS line.
The compact summary reports sixteen states grouped as 4, 8, and 4, followed by
zero same-temperature equilibrium current.

What this establishes

The construction is an entirely digital graph whose temperature is fixed by a
declared reservoir state-count relation and independently recovered from
reversible transition traffic. Its complete energy account and contact test
close exactly.

Provenance

The crate files are archived verbatim from repository commit:
17b57589a0bd84dfd13d94f09d7340f5d4e85392

The public copy of the complete technical result is at:
/evidence/information-state/CAL-XTHERM-Result.md

Its recorded source commit is:
c0e32bc1fc787c5322df07422a183a4977b20857

It does not yet perform the action-to-future causal-control experiment described
in “Intelligence as Physical Units.” That is the next construction.

Source-language note

The archive preserves the terminology of the frozen source so its result remains
reproducible byte for byte. It therefore uses TAU in identifiers and output where
the public framework now uses THAIM. Do not rename the frozen files when checking
their recorded hashes.
