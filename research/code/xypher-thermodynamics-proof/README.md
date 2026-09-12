# Exact Thermodynamic Xypher

A small thermodynamic graph you can run and inspect in full. It includes the graph, its energy reservoir, and the mechanism that moves between states. Together they have sixteen possible arrangements.

Run the code to check every energy exchange, compare two independent thermometers, and test what happens when two copies are placed in thermal contact. The construction supplies an experimental foundation for [a physics for digital agency](https://xyphers.com/#framework).

## Run

With Rust installed, from this directory:

```sh
cargo test --locked
cargo run --locked --quiet
```

No external crate, dataset, random seed, burn-in, numerical tolerance, or network service is required by the verifier. The report ends with `OVERALL PASS`, following eleven passing gates and four controls that detect deliberately broken constructions.

These commands reproduce an already reviewed and executed result. The original pre-execution review restrictions belong to the historical experiment; independent reproduction is welcome.

## Read the experiment

- [A Purely Digital Thermodynamic System](https://xyphers.com/research/proving-true-thermodynamic-graph-systems/) explains the body, reservoir, two thermometers, signed accounting, and contact. A [portable article](../../articles/proving-true-thermodynamic-graph-systems.md) is also included in this checkout.
- [The frozen boundary and representation argument](evidence/BOUNDARY.md) defines the operational T1–T6 requirements independently of Xypher terminology.
- [The exact result](evidence/RESULT.md) records the eleven gates, four mutations, output, and execution provenance.

The construction establishes one minimal thermodynamic Xypher. A separate analytic theorem characterizes the declared fixed-finite, one-temperature, reciprocal equilibrium class. Enumerating this example does not establish that theorem by testing it once.

## How the two implementations meet

| File | Role |
|---|---|
| [`xypher.runa`](xypher.runa) | Prospective Futuruna construction: endpoint entropy, signed accounting, and reciprocal Praxion hazards. |
| [`src/lib.rs`](src/lib.rs) | Independent Rust enumeration of complete states, microscopic movement, projected rates, equilibrium, contact, and mutations. |
| [`src/main.rs`](src/main.rs) | Compact deterministic report. |
| [`tests/source_contract.rs`](tests/source_contract.rs) | Acceptance test for the exact evaluator. |

The Rust check runs independently of a Futuruna compiler. The `.runa` source remains inspectable; compiler work belongs to [Futuruna](https://github.com/Futuruna/futuruna).

The reservoir fixes **α · ln 2 = one energy unit**. Integer and rational identities therefore check the result exactly. THAIM is a positive expansion receipt; its forward-minus-reverse pair restores the signed entropy contribution in Ξ. Both directions remain possible at finite temperature.

Ruby is empty, Opal is absent, and memory has one non-adaptive condition. Growth, learning, multiple Praxions, and consciousness are later architectural questions.

## Provenance

The executable source, manifests, and test are preserved from the apparatus frozen at `17b57589a0bd84dfd13d94f09d7340f5d4e85392`. The scientific records are copied from THAIM revision `c46f4a808` and retain the original boundary and execution identities. [`SOURCE-SHA256SUMS`](SOURCE-SHA256SUMS) covers these files. The public README is an onboarding wrapper.

Historical `TAU` identifiers remain in frozen source and output. New prose uses **THAIM**. Changing the spelling in a frozen artifact would change its identity without changing the physics.
