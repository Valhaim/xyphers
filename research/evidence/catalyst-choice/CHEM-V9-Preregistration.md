# CHEM-V9: outcome-blind ORD catalyst-yield test

Date prepared: 2026-07-14
Task: `td-6706c1`

## Question and claim boundary

Does the previously defined site-local endpoint entropy of a phosphine ligand carry reproducible information about catalyst yield after each experimental recipe context is held fixed?

The primary claim is **context-sensitive ligand informativeness**. The sign of a steric effect may reverse across catalyst, substrate, base, solvent, and temperature contexts, so CHEM-V9 does not test or claim one universal monotone yield direction. It also does not establish a thermodynamic equation of state, kinetic transition rates, a complete Crystal, or a Crystal/Thermo/Praxor circuit.

## Sealed source and physical blindness boundary

The source is Das et al., *A 50,688-Reaction Data Set Reveals General Ligands and Mechanistic Diversity in C–N Couplings*, JACS 2026, DOI `10.1021/jacs.6c05959`.

- ORD dataset ID: `ord_dataset-805ad863feef48579d95d86a728035f4`
- Official file: `data/80/ord_dataset-805ad863feef48579d95d86a728035f4.parquet`
- File size: 5,088,296 bytes
- SHA-256: `31b7c4762145e5dad973050373cd967bdd85caf3a9858767c83a17fed6c4af36`
- Rows: 50,688; row groups: 51
- Physical Parquet columns: `reaction_id: string`, `reaction: binary`

The outcome is not a separately projectable Parquet column. It is embedded in the opaque `Reaction` protobuf. The feature extractor therefore walks only the top-level protobuf wire envelope and discards field 8, `Reaction.outcomes`, without interpreting its payload. Only the filtered message is decoded. No official outcome, product, analysis, conversion, yield, or target value may enter any CHEM-V9 pre-reveal artifact.

After all frozen artifacts are committed and pushed, a separate reveal process may extract exactly:

```text
Reaction.outcomes[*]
  .products[is_desired_product == true]
  .measurements[type == YIELD]
  .percentage.value
```

The reveal joins to `loaded_memberships.tsv` by `reaction_id`. Every one of the 49,632 ligand-loaded memberships, including ineligible structures used only to train the position control, must have exactly one finite desired-product yield. Any missing, duplicated, ambiguous, or nonfinite target makes CHEM-V9 inconclusive; rows may not be mechanically excluded and no cohort or descriptor repair is allowed. Zero-loading singleton controls are not revealed or analyzed.

## Exact blocks

The non-ligand block ID is SHA-256 over deterministic protobuf serialization after all of the following target-independent operations:

1. Strip `Reaction.outcomes` at the wire level before decode.
2. Remove the unique `reaction_id` and provenance.
3. Retain only the reaction-type and `details="experiment"` identifiers; remove sample-name and coded-recipe row identifiers.
4. Blank only the ligand component's chemical identity identifiers. Preserve its amount/loading, catalyst role, preparation, source, and every other nonidentity field.
5. Preserve every other input, amount, addition detail, solvent component, catalyst precursor, base, electrophile, nucleophile, setup, and condition exactly.

This produces 2,112 dose-exact blocks. The confirmatory design uses 1,056 ligand-loaded blocks of exactly 47 rows. The zero-loading controls form 1,056 separate singleton blocks and are frozen but unused in confirmatory inference. Treating a zero-dose control as exchangeable with a ligand-loaded row is forbidden. The 17 source experiments contain 64 loaded blocks each except `XZ-01-1`, which contains 32.

## Target-independent ligand cohort

The source contains 164 real ligand IDs plus `L_empty`, collapsing to 162 canonical isomeric SMILES. Canonical SMILES, not source ligand ID, is the inferential structure unit.

A structure enters the confirmatory cohort only when all gates hold:

1. The molecular record is one connected component and has total formal charge zero.
2. It contains exactly one phosphorus atom.
3. Phosphorus has formal charge zero, no attached hydrogen, exactly three heavy-atom neighbours, and all three neighbours are carbon.
4. None of those directly P-bound carbons is a nonaromatic CH₂ group.

The final target-blind cohort is 45 canonical structures across 17,632 reaction rows. An earlier metadata count of 46 was corrected before freeze: `L146`, tri-tert-butylphosphonium tetrafluoroborate, is a disconnected protonated phosphonium salt and is excluded. It is never a stress-case or rescue analysis.

Three source experiments are structurally non-estimable for within-block rank association: `XZ-01-115` and `XZ-01-55` contain one eligible structure per block, and `XZ-01-73` contains two. They are excluded before reveal. The frozen minimum panel size is **9**, the smallest panel among retained experiments; there are no source panels of size 3 through 8. The primary statistic therefore has 45 structure units, 17 source experiments, and **14 estimable experiment clusters**. The 864 blocks in those 14 experiments all contain at least nine eligible structures.

## Frozen descriptor

For each eligible ligand, construct the same standardized topology used by the prior Tolman experiment:

```text
Rh–C≡O
│
H

Rh–P(ligand)
```

The graph contains the ligand's phosphorus-connected component with the same explicit implicit-hydrogen expansion as the prior operator, plus four scaffold atoms: Rh, carbonyl C, carbonyl O, and hydride H. The actual Pd, Ni, Cu, or control precursor from the catalytic reaction never enters the ligand descriptor.

All graph edges have unit weight. Starting at ligand phosphorus, propagate an unbiased random walk for exactly `τ=5` steps:

\[
P_{ij}=\frac{1}{\deg(i)},\qquad
S_5(P)=H_2(e_P^\top P^5).
\]

Transition propagation uses exact rational arithmetic. Shannon entropy uses 80-digit decimal arithmetic and is quantized to 12 decimal places in the frozen table.

Primary predictor: standardized-complex unweighted `S_5(P)`.

Frozen non-rescue secondary: free-ligand-component unweighted `S_5(P)`. It is reported regardless of sign but cannot rescue failure of the primary predictor.

No actual-metal graph, BDE weighting, target transform, atom normalization, alternative root, horizon scan, cohort repair, or descriptor refit is permitted after reveal.

## Frozen simple graph baselines

The baseline family is fixed before reveal:

- explicit-H all-atom count, heavy-atom count, and explicit-H edge count;
- molecular weight;
- exact shell counts at radii 1 through 5;
- cumulative ball counts at radii 1 through 5;
- exact-step-five endpoint support on both the standardized complex and free ligand;
- radius-five branch excess;
- mean phosphorus-to-atom graph distance;
- Wiener, Randić, Zagreb M1, and Balaban J graph indices.

For a block where a predictor or outcome has zero variance, its block association is defined as zero. This retains the fixed block and experiment support rather than selecting informative contexts after reveal.

## Frozen plate-position controls

No source randomization is documented. The target-blind layout parser resolves all 50,688 sample names into subplate and 16-by-24 well coordinates. There are 33 experiment-temperature runs of 1,536 unique positions each. Sixteen experiments repeat the same actual-ligand and coded-recipe assignment at each position at two temperatures; `XZ-01-1` has one temperature. Actual ligand IDs are reassigned to coded slots across experiments, but placement is only partially mobile: 39 of 45 eligible structures occupy at least two slot-derived rows across source experiments, while six never change row.

Two mandatory position baselines are frozen:

1. Ordinal `coded_ligand_slot`, evaluated like any other predictor within each eligible block.
2. A flexible leave-one-experiment-out slot score. For held-out experiment `e`, rank all 47 yields within every ligand-loaded block in the other 16 experiments using average ranks, transform rank `r` to `(r-1)/46`, and average that rank-percentile by coded ligand slot. Assign the resulting slot score to structures in `e`. No outcome from `e` enters its score. If any required slot lacks training coverage, CHEM-V9 is inconclusive; there is no imputation.

Both position scores stay fixed under descriptor-row permutations. Their per-experiment `T_e` values also stay fixed when experiment clusters are resampled. Both enter the strongest-baseline maximum. This controls ordinal trends and arbitrary *stable cross-experiment* slot patterns, but cannot identify experiment-specific or otherwise unstable well bias. CHEM-V9 therefore permits no causal catalyst-effect language.

The report must include per-experiment correlations of standardized-complex `S_5(P)` with both position controls. It must also report a non-rescue movers-only sensitivity: restrict to the 39 structures observed in at least two slot-derived rows, recompute `T`, signed mean rho, and retained experiment/block counts. This descriptive result cannot rescue the primary decision and has no causal interpretation.

## Primary statistic

Within each frozen block `b`, compute tie-aware Spearman rank correlation with average ranks:

\[
\rho_b=\rho_{\mathrm{Spearman}}(x_l,Y_{bl}).
\]

The context-sensitive score uses magnitude, not a fitted per-block direction:

\[
T_e(x)=\frac{1}{|B_e|}\sum_{b\in B_e}|\rho_b|,
\qquad
T(x)=\frac{1}{14}\sum_{e=1}^{14}T_e(x).
\]

Every block has equal weight within its experiment and every estimable experiment has equal weight. No row-level standard error, row bootstrap, or claim of `n=17,632` independent observations is allowed.

The signed experiment-balanced mean `mean(ρ_b)`, positive/negative/zero block counts, and the fraction of nonzero block correlations with positive sign are mandatory secondary sign-consistency diagnostics. They cannot rescue the primary result.

## Exact panel-preserving null

The 17 experiments use different ligand panels, so an unrestricted global permutation is invalid: it changes the descriptor and tie distribution inside panels.

The frozen null groups structures by their exact 17-experiment incidence/multiplicity vector. Complete molecular-descriptor rows—not individual predictor columns—may be permuted only within identical incidence strata. One structure mapping is then applied consistently to every occurrence of that structure in every block. This preserves every experiment's exact panel, molecular-predictor covariance, and repeated-measurement pattern. The two position controls remain fixed. The strongest baseline is reselected inside every permutation.

The target-free incidence audit produces:

- 34 strata;
- 26 singleton strata;
- five strata of size two;
- three strata of size three;
- 19 movable structures;
- exactly `2!^5 × 3!^3 = 6,912` globally consistent permutations.

All 6,912 permutations, including the identity, are enumerated. The exact upper-tail p-value is the fraction whose statistic is at least the observed statistic. Its minimum is `1/6,912 ≈ 0.000145`. Because `|ρ|` has a positive, panel-size-dependent null, the material association quantity is null-centered:

\[
E=T_{obs}(S_5)-\operatorname{mean}_{\pi}T_{\pi}(S_5).
\]

The p-value is exact only under exchangeability of structure/outcome profiles within identical incidence strata; the molecular descriptors were not randomized treatments. It also has severe power limitations: 26 of 45 structures cannot move. The exact p-values therefore test only the 19 movable structures conditional on the 26 fixed structures, observed panels, and fixed position controls. Failure means CHEM-V9 does not support the preregistered claim; it does not prove the descriptor has zero effect. A pass remains associational evidence limited to these ligand panels and recipe contexts.

## Uncertainty and baseline comparison

The 95% confidence interval uses 10,000 paired cluster-bootstrap replicates with frozen seed `0x4348454d5639b007`. The 14 estimable experiments are resampled with replacement; all blocks and ligand rows belonging to a sampled experiment remain together. Ranks are recomputed inside every replicate. The percentile interval is fixed to zero-based sorted replicate indices 249 and 9,749, equivalent to nearest-rank 2.5% and 97.5% quantiles.

For each frozen molecular or position baseline `k`, compute `T_k` by the same procedure. Let

\[
T_{null}=\max_k T_k,\qquad \Delta=T(S_5(P))-T_{null}.
\]

The baseline champion is recomputed inside each exact permutation and each cluster-bootstrap replicate. Molecular baseline rows move jointly with the candidate; ordinal and cross-fitted position scores stay fixed. The baseline-competition exact p-value is the fraction of the 6,912 permutations whose permuted `Δ` is at least the observed `Δ`. Under the stated within-stratum exchangeability assumption, this tests the global no-molecular-descriptor-association null; it establishes a higher univariate contextual score than every frozen baseline, not conditional or nested incremental information. The bootstrap is an uncertainty statement scoped to 14 experiments and these 45 ligand structures; the exact permutation is scoped to the 19 movable structures. Neither calculation treats reaction rows as independent replicates.

## Frozen decision rule

CHEM-V9 passes only if every condition holds:

1. The primary standardized-complex `S_5(P)` exact panel-preserving permutation p-value is `p ≤ 0.001`.
2. Its null-centered material association is `E ≥ 0.05`.
3. Its advantage over the strongest frozen molecular or position baseline is `Δ ≥ 0.05`.
4. The exact baseline-competition permutation p-value for `Δ` is `p ≤ 0.001`.
5. The 95% 14-experiment cluster-bootstrap lower bound for `Δ` is above zero.
6. Every frozen QA gate, panel identity, position-control coverage check, row join, and complete 49,632-target coverage audit passes without model repair.

All primary quantities are reported even when one gate fails. No secondary target, signed statistic, free-ligand entropy, catalyst subset, metal subset, or post-reveal exclusion can rescue failure.

Interpretation:

- Pass: site-local finite-horizon topology carries context-sensitive catalyst-yield association stronger than the frozen molecular-topology and position controls in this ORD design.
- Association p-value passes but margin fails: a frozen molecular or position control matches the signal; no distinct entropy win.
- Primary p-value or material-effect gate fails: CHEM-V9 does not support a reproducible yield-ranking signal on this design.
- Target coverage or integrity gate fails: inconclusive, never a pass.

## Alias handling

Two canonical structures in the full source have multiple ligand IDs. Aliases are collapsed to canonical SMILES before eligibility or inference, and their within-block co-occurrence is recorded in `alias_audit.tsv`. The target-free audit finds zero blocks where both IDs for either alias structure co-occur, and neither alias structure enters the 45-structure confirmatory cohort. The reveal rule nevertheless aggregates any duplicate `(block, canonical structure)` targets to one arithmetic-mean yield before ranks. Alias multiplicity and incidence remain fixed in every permutation; alias IDs can never be treated as independent structures.

## Freeze rule

The source hash, toolchain, outcome-blind preparer, separate reveal/evaluation program, synthetic outcome tests, target-free tables, panel strata, alias and plate audits, preregistration, and manifest must be committed and pushed before `chem_v9_evaluate.py extract-targets` is run. The evaluator may be tested only on synthetic outcomes before that point. Any later change to the parser, wire filter, canonicalization, cohort, scaffold, τ, descriptor, baseline set, position control, panel null, statistic, or decision rule creates CHEM-V10; it cannot repair CHEM-V9.
