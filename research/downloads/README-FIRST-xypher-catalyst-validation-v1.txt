XYPHERS CATALYST-CHOICE VALIDATION PACKET

This packet supports “Can a Xypher Help Choose a Catalyst?” It separates the
three frozen July controls from the earlier exploratory and separate evidence.

Read first
  1. evidence/CHEM-V8-Preregistration.md
  2. evidence/CHEM-V8-Result.md
  3. evidence/CHEM-V9-Result.md
  4. evidence/CHEM-V10-Preregistration.md
  5. evidence/CHEM-V10-Result.md

Inspect
  evidence/ contains protocols, manifests, results, machine-readable outputs,
  the frozen descriptor implementation, and the early/separate source record.
  evidence/SHA256SUMS covers that directory. The manifest retains hashes for
  the omitted Kraken-derived structure and prediction tables.

Reproduce
  RUN.txt gives an honest V8 inspection/reconstruction path and the executable
  V10 replay. V8 is not turnkey: the packet omits the licensed Kraken workbook,
  the derived 1,558-row structure table, and the derived 934-row prediction
  table. Exact reconstruction requires matching the frozen manifest hashes.

  chem_v9/frozen contains the target-free feature freeze needed for the V10
  numerical replay. The raw ORD parquet is not redistributed; obtain it from
  the immutable dataset path named in RUN.txt.

Licensing
  Retained ORD-derived tables and numerical artifacts are adapted from Open
  Reaction Database data under CC BY-SA 4.0. ORD-DATA-NOTICE.txt provides the
  exact attribution, source, license link, share-alike statement, and record of
  modifications.

Archive integrity
  SHA256SUMS covers every payload file except itself. The website publishes a
  separate SHA-256 sidecar for the final zip.

Scientific boundary
  CHEM-V8 found an association with buried volume, but a radius-four atom count
  was stronger and the full descriptor claim failed. CHEM-V9 stopped before
  hypothesis statistics. CHEM-V10 was a disclosed post-reveal, pre-statistic,
  target-independent mechanical repair; all five yield gates failed.

  V8 did compare the graph score with a quantum-chemically calculated Kraken
  target. It did not compare a complete catalyst-selection system with DFT.
  The packet contains no complete Xypher, prospective ligand-selection policy,
  randomized catalyst experiment, or new-catalyst discovery.
