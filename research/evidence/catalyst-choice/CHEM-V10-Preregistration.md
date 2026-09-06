# CHEM-V10: required-slot LOEO repair

Date prepared: 2026-07-14
Task: `td-0ceeba`

## Status and claim boundary

CHEM-V10 is a **post-reveal, pre-statistic, target-independent mechanical repair**, not a pristine confirmation. CHEM-V9 loaded and processed the sealed outcome table in memory before encountering a deterministic coverage exception, but printed no hypothesis statistic. During extraction verification, the root operator displayed the target-table header and its first two reaction-ID-sorted rows; both displayed yields were zero. No target distribution, descriptor pairing, association, permutation result, bootstrap result, or decision quantity was inspected before this repair was frozen. The repair itself was derived solely from target-free membership and plate-layout artifacts.

The only analytical change is the domain of the leave-one-experiment-out coded-slot control. For held-out primary experiment `e`, construct scores only for coded ligand slots used by `e`'s frozen eligible primary panel. Within every training block, rank all 47 ligand-loaded rows exactly as CHEM-V9 specified; no row is removed before ranking. Average `(rank-1)/46` by required slot across the other 16 source experiments. A required slot with no training occurrence makes CHEM-V10 inconclusive. A globally observed slot that no eligible primary panel requires is not a missing predictor.

Everything else is inherited unchanged from frozen CHEM-V9: source and membership rows, 45-structure cohort, 14 primary experiments and 864 primary blocks, standardized Rh-complex `S_5(P)`, secondary and baseline family, fixed coded-slot treatment, statistic, exact 6,912-permutation null, 10,000-replicate cluster bootstrap, decision gates, and associational/causal limits.

The one-shot evaluator must consume the exact preserved CHEM-V9 target artifact with SHA-256 `58dae5feac0aad193b3a8d6590120b5abfea02447aed093e4b146ccaf9a8f407`. Its existing exact-ID and finite-value coverage checks remain mandatory.

The target-free frozen-artifact preflight must pass before outcome-table access. It must prove:

- all 49,632 loaded reaction IDs are unique and all 1,056 training blocks contain 47 unique coded slots;
- each primary experiment's eligible structure-to-slot panel mapping is constant across its blocks;
- every slot required by every held-out primary experiment has training coverage;
- required missing count is zero;
- slots 1–48 are globally observed, slots 1–47 are required, and slot 48 is globally observed but not required;
- the sole nonstandard block contains slot 48 in place of slot 16; slot 48 occurs once, in `XZ-01-1`.

The implementation and preflight are committed and pushed before any CHEM-V10 hypothesis evaluation. The evaluator then runs once without analytical repair. A pass is only post-reveal, repaired corroborating associational evidence under CHEM-V9's existing limitations; it is not evidence for an equation of state, physical circuitry, or causality.
