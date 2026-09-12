# Xyphers

**A physics for digital agency.**

A Xypher brings together a graph of possibilities, a thermodynamic account of change, and a **Praxion** that perceives and acts. The ambition is digital systems that develop, preserve, and expand their capacity to act.

Start with a small thermodynamic graph you can run. It is a complete system with its own energy reservoir, and every arrangement and exchange can be inspected. The code connects the research at [xyphers.com](https://xyphers.com) to experiments you can reproduce on your own computer.

[Read the Core Thesis](https://xyphers.com/core-thesis/) · [Start with the experiment](research/code/xypher-thermodynamics-proof/) · [Research collection](research/README.md)

## Run the exact thermodynamic system

With [Rust](https://rustup.rs/) installed:

```sh
git clone https://github.com/Valhaim/xyphers.git
cd xyphers
cargo test --locked --manifest-path research/code/xypher-thermodynamics-proof/Cargo.toml
cargo run --locked --quiet --manifest-path research/code/xypher-thermodynamics-proof/Cargo.toml
```

The program enumerates all **16 complete states**. A thermometer derived from reservoir arrangements agrees exactly with one recovered from movement. Energy closes on every transition, and two equally prepared bodies carry zero expected equilibrium energy current in contact.

The report ends with `OVERALL PASS`: eleven acceptance gates pass, and four deliberately broken constructions fail at their predicted gates.

Read [A Purely Digital Thermodynamic System](research/articles/proving-true-thermodynamic-graph-systems.md) for the explanation, or enter the [study](research/code/xypher-thermodynamics-proof/) for the exact source, boundary, and result. The finite construction establishes existence. The separate mathematical argument establishes its scoped representation theorem.

## Explore a molecule's possible futures

```sh
cargo run --locked --quiet --manifest-path research/code/molecular-properties/Cargo.toml --example phosphines -- 5
```

A molecule becomes a graph of atoms and bonds. This example starts at phosphorus and measures the spread of destinations after five bond choices, in both bits and nats. Change `5` to explore a different horizon.

The [molecular-properties study](research/code/molecular-properties/) includes the frozen Rust calculation and blinded shape-test evaluator. A graph readout associated with a measured molecular-shape target; reliable catalytic-performance prediction remains an open objective. [The catalyst article](research/articles/can-a-xypher-help-choose-a-catalyst.md) explains the empirical comparisons and their limits.

## What is here

| Path | Purpose |
|---|---|
| [`research/articles/`](research/README.md) | Readable arguments, experiments, and open questions. |
| [`research/code/xypher-thermodynamics-proof/`](research/code/xypher-thermodynamics-proof/) | Exact finite thermodynamics, prospective Futuruna construction, independent Rust verifier. |
| [`research/code/molecular-properties/`](research/code/molecular-properties/) | Frozen molecular graph implementation, a small example, and the shape evaluator. |
| [`research/evidence/`](research/evidence/) | Protocols, results, provenance, and additional reproduction sources. |
| [`research/downloads/`](research/downloads/) | Portable evidence packets with checksum sidecars. |

The scientific foundation and the richer organism are distinct achievements. The exact kernel is non-adaptive. Learning, growth, chemical experiment selection, and an economy that sustains shared possibility each require their own complete construction and evidence.

**Possibility. Consequence. Agency.**

## Develop and reproduce

```sh
./scripts/verify.sh
```

This runs the two Rust studies, checks frozen source and packet hashes, and checks local documentation links. It needs Rust and Python 3.9 or newer. It does not fetch the external chemistry datasets or run the large exhaustive graph enumerations.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the source and claim boundaries. The payment runtime lives in [Valhaim/valhaim-node](https://github.com/Valhaim/valhaim-node); the early Valhaim alpha is at [valhaim.com](https://valhaim.com). Read the live research at [xyphers.com/research](https://xyphers.com/research/), or use the portable articles in this checkout.

Code and data licensing are described in [LICENSING.md](LICENSING.md).
