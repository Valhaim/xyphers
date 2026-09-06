XYPHERS LOCAL-FUTURE-TEST EVIDENCE

Question
  When a proposed unit-weight edge raises average endpoint entropy over its
  affected set, must the average over the entire graph also rise?

What this packet establishes
  * Algebraic locality proves the implication for connected simple
    unit-weight graphs at horizons tau <= 2.
  * The exhaustive N=3..6, tau=1..5 classifier contains 947,935
    graph-candidate-edge-horizon cases. Of these, 919,585 pass locally;
    none of those lowers the graph-wide average.
  * The rule is not universal. The exhaustive N=7 record contains 117,180
    locally-positive/globally-negative cases across tau=3,4,5, including
    5,040 at tau=3. The stored notebook disagrees about the tau=4/tau=5
    split, so this packet does not publish that split.
  * Requiring minimum degree two still leaves 34,020 N=7 failures at tau=4
    and 384,720 N=8, tau=3 failures among 1,952,212,864 locally positive
    checks.
  * A restricted distance-three tree family fails at tau=3 for N=1,770,
    D=1,762, d_v=7. Its affected-set sum is +0.01407462, the outside sum is
    -0.01419718, and the total is -0.00012256.

What it does not establish
  The affected-set average is not automatically one actor's payoff. The
  graph-wide average is not welfare, generosity, a Pareto improvement, or
  AI alignment. The observable is endpoint entropy, not full path entropy.
  No Thermodynamic Harness or Praxion is proved here.

Layout
  reproduction/   Standalone Rust crate. Its copied verifier and entropy
                  source retain their original text. The small support
                  modules make that source compile outside THAIM.
  source/         Additional original inspection-only probes.
  proof/          Research proof/boundary records. These are notebooks;
                  later corrections control earlier conjectures.
  results/        Expected result dictionary for the public claims.
  RUN.txt         Reproduction commands and cost warnings.
  SHA256SUMS      Content manifest for this directory.

Source-status warning
  The copied programs preserve their historical comments and conclusions.
  In particular, converse_verify.rs names a generalized potential game even
  though it does not define the actor, owned actions, or all deviations needed
  for that inference. dist3_limit.rs searches one declared tree family; its
  "smallest" label means smallest found inside that family. Run that program
  in release mode and use only its N=1,770 result section. Later diagnostics
  in the copied source include an invalid d_v=0 row, a stale "4998 pendants"
  sentence, and a debug-only unsigned-underflow panic; none controls the
  published case.

  proof/later-boundary-roadmap.md is a historical, noncontrolling notebook.
  Its claims that a maximum-edge policy is proved or "safe by construction"
  are superseded because its score and action boundary do not match the
  canonical affected-set-average claim. It is included only to expose the
  later boundary counts and the N=163-to-N=1,770 provenance correction. The
  article and this README state the controlling, narrower interpretation.

Provenance
  Verifier and entropy sources were copied from the THAIM working tree at
  baseline commit 9f836dfc. This public packet adds only the minimal crate
  shell needed to compile the selected sources independently. Each file is
  hashed in SHA256SUMS.

  proof/two-step-and-tree-record.md maps to
  research/alignment/proof-trees.md at that baseline. The post-baseline
  proof/later-boundary-roadmap.md snapshot maps to
  research/alignment/roadmap-filter-proof.md on 2026-08-18, after its false
  N=163 classification was corrected to the controlling N=1,770 case. Its
  noncontrolling status is stated above rather than hidden by editing the
  notebook's historical claims.
