# Molecular graph properties

Start at an atom. Follow the molecular bonds for a declared number of steps. How widely are the possible destinations spread?

This study packages the frozen Rust molecular implementation used in the CHEM-V8 blinded shape comparison. It also includes that study's evaluator and a small example requiring no external data.

## Run the example

From this directory:

```sh
cargo test --locked
cargo run --locked --quiet --example phosphines -- 5
```

The example reads three explicitly listed phosphines from [`examples/phosphines.tsv`](examples/phosphines.tsv). It builds each ligand graph using the frozen parser, starts at phosphorus, and prints its formula, atom count, and endpoint entropy at the requested horizon.

Every available bond is chosen with equal probability. The historical implementation measures entropy in bits; the example also prints nats by multiplying by `ln 2`. A horizon of zero means no steps and gives zero endpoint entropy.

These are **ligand-only teaching examples**. The frozen blinded experiment used a standardized `Rh(CO)(H)(L)` complex instead. The example is not a replay of that experiment or a ranking of catalytic efficiency.

## What the evidence establishes

The primary five-step readout associated with Boltzmann-average absolute buried volume across 934 target-blinded ligand graphs at **ρ = 0.5252**. A simpler local atom count reached **ρ = 0.7255**, and the full descriptor claim failed its required gates. A later observational reaction-yield analysis failed all five gates.

The significance and boundary belong together: molecular topology contains information associated with a chemical property, while this tested readout has not demonstrated an advantage for selecting ligands.

Read [Can a Xypher Help Choose a Catalyst?](../../articles/can-a-xypher-help-choose-a-catalyst.md) and the [frozen evidence](../../evidence/catalyst-choice/README.txt).

## Reconstruct the blinded shape calculation

The evaluator is runnable here:

```sh
cargo run --locked --bin chem-v8-blind -- predict STRUCTURES.tsv exposed PREDICTIONS.tsv
cargo run --locked --bin chem-v8-blind -- evaluate PREDICTIONS.tsv TARGETS.tsv REPORT.md
```

`STRUCTURES.tsv` and `TARGETS.tsv` must come from the named Kraken source and obey the frozen protocol. They are not bundled inputs. [`RUN.txt`](../../evidence/catalyst-choice/RUN.txt) gives the source, reconstruction rules, and required hashes. Its archive-relative `chem_v8` directory corresponds to this study when using the commands above. `extract_kraken_targets.py` is included here unchanged.

The [portable validation packet](../../downloads/xypher-catalyst-validation-v1.zip) also contains the complete CHEM-V9 feature freeze, CHEM-V10 evaluator, and reproduction instructions. Replaying the yield analysis additionally requires obtaining the identified ORD dataset. Retained ORD-derived data keep their [CC BY-SA 4.0 notice](../../evidence/catalyst-choice/ORD-DATA-NOTICE.txt).

## Source identity

`src/molecular.rs`, `src/bin/chem_v8_blind.rs`, `src/lib.rs`, `exposed/`, and the target extractor are byte-identical to the prepared public evidence at THAIM revision `c46f4a808`. [`SOURCE-SHA256SUMS`](SOURCE-SHA256SUMS) records them. The upstream experiment manifests retain their original freeze identities.

The package manifest and teaching example are new packaging. The library identifier `thaim_core` is retained because the frozen evaluator imports it; this crate has no dependency on the large THAIM repository and no external Rust dependencies.

The molecular module includes historical chemistry helper functions and assumptions beyond the tested unweighted readout. Its bond-energy tables, parser scope, numerical cutoffs, and functions named `alpha_chem` or `phi_chem` remain frozen for provenance. Those names do not establish an operational thermometer or a complete Xypher. This study has no independently grounded thermodynamic harness or acting Praxion.

New experiments should receive new source and evidence identities. Do not rewrite a frozen calculation to improve its historical result.
