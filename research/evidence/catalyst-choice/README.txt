XYPhERS CATALYST-CHOICE EVIDENCE MAP

Article
  Can a Xypher Help Choose a Catalyst?

Read first
  CHEM-V8-Preregistration.md
  CHEM-V8-Result.md
  CHEM-V10-Preregistration.md
  CHEM-V10-Result.md

Integrity boundary
  CHEM-V8-Manifest.txt records the target-blind freeze.
  CHEM-V9-Preregistration.md, CHEM-V9-Manifest.txt, and
  CHEM-V9-Result.md record the stopped first yield analysis.
  CHEM-V10-Preregistration.md and CHEM-V10-Manifest.txt disclose the
  post-reveal, pre-statistic mechanical repair.
  required_slot_preflight.json is the target-free repair audit.

Inspect the calculations
  chem_v8_blind.rs is the frozen shape-test predictor and evaluator.
  molecular.rs is the exact frozen molecular implementation used by V8.
  extract_kraken_targets.py extracts the declared target columns after reveal.
  CHEM-V8-Manifest.txt preserves the frozen SHA-256 values for both the
  licensed source-structure table and its 934-row prediction table. Those two
  SMILES-bearing tables are intentionally not redistributed.
  chem_v9_evaluate.py supplies the unchanged inferential core used by V10.
  chem_v10_evaluate.py contains the required-slot repair.
  CHEM-V10-Result.json and CHEM-V10-Numerical-Audit.json expose the
  machine-readable result and arithmetic audit.

Earlier and separate evidence
  tolman_heldout.rs and Insight-278.md show why the favorable ten-ligand
  phosphorus-rooted value is exploratory rather than confirmatory.
  hydroformylation_diphosphine.rs and Insight-279.md contain the separate
  eight-ligand bite-angle comparison. That comparison did not explain V10.

Hashes
  SHA256SUMS covers every file in this directory except SHA256SUMS itself.
  Hashes for omitted frozen V8 artifacts remain in CHEM-V8-Manifest.txt.
  The download archive contains a separate internal SHA256SUMS and has a
  sidecar SHA-256 file beside it.

External sources
  Tolman review: https://doi.org/10.1021/cr60307a002
  Kraken paper: https://doi.org/10.1021/jacs.1c09718
  Kraken supplement: https://doi.org/10.1021/jacs.1c09718.s002
  Coupling screen: https://doi.org/10.1021/jacs.6c05959
  Immutable ORD object:
  https://github.com/open-reaction-database/ord-data/blob/ddb0d25770c80a0a6fcf9948c26e1c8f828cb8ad/data/80/ord_dataset-805ad863feef48579d95d86a728035f4.parquet

Licensing boundary
  This public evidence directory and its archive do not redistribute the
  Kraken supplement, the derived 934-row Kraken SMILES/prediction table, or
  the raw ORD parquet. RUN.txt explains how those sources enter a
  reconstruction or replay.

  The retained ORD-derived tables and numerical artifacts are adapted from
  Open Reaction Database data licensed CC BY-SA 4.0. ORD-DATA-NOTICE.txt gives
  exact attribution, the immutable source, the license, and the modifications.
