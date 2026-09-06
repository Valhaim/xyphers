# CHEM-V8: blinded phosphorus-site entropy validation

Date frozen: 2026-07-14
Task: `td-4e6875`

## Question

Does finite-horizon endpoint entropy rooted at phosphorus recover independently computed local steric crowding better than ordinary molecular-size and local-topology descriptors?

This is a test of a molecular graph descriptor. A positive result does not by itself establish catalytic activity prediction, a full Crystal/Thermo/Praxor circuit, or a new thermodynamic law.

## Blind source

The source is the DFT portion of Gensch et al., *A Comprehensive Discovery Platform for Organophosphorus Ligands for Catalysis*, JACS 2022, DOI `10.1021/jacs.1c09718`.

- ACS Figshare dataset DOI: `10.1021/jacs.1c09718.s002`
- Archive: `ja1c09718_si_002.zip`
- Archive MD5: `e8edb32328458b214cd645d2186249c1`
- `descriptors.xlsx` SHA-256: `6959fbbe838303408e8c19d2b2149b8c61455669f952b38120b514a74ec5dbaf`
- Workbook sheet: `DFT_data`, 1,558 rows
- Structure columns opened before prediction: `ID`, `smiles`
- Extracted structures SHA-256: `4e0b7514858715bb3d2ac681007e32d06aab1d787b78ffbf473622b360f203b9`

The per-ligand values in the target columns have not been inspected at the time this protocol is frozen.

After the predictor artifact is committed and pushed, the evaluator will extract exactly:

- Primary target: `vbur_vbur_boltz`, absolute buried volume in Å³, Boltzmann average at 298.15 K.
- Secondary target: `sterimol_burB5_boltz`, buried Sterimol B5 in Å, Boltzmann average.
- Three-dimensional positive control: `volume_boltz`.

The secondary target cannot rescue failure on the primary target.

## Target-independent cohort

Rows are retained mechanically when all of the following hold:

1. The source SMILES is connected and carries no explicit formal charge.
2. The frozen repository parser accepts it without a bracket form known to lose element or explicit-hydrogen identity.
3. The parsed graph has exactly one phosphorus and exactly three single P–C bonds.
4. Its element-labelled constitutional connectivity does not match any of the 50 development ligands or 10 prior held-out ligands in `tolman50.rs` and `tolman_heldout.rs`.
5. No ligand bond returns the frozen BDE table's numeric fallback sentinel.
6. Duplicate element-labelled constitutional graphs collapse to the lexicographically first source ID.

The fixed Rh(CO)(H)(L) scaffold retains the historical C≡O fallback value of 750 kJ/mol. It is common to every row. Ligand fallbacks are not allowed. Numeric-sentinel filtering is conservative: an explicit table value numerically equal to a fallback can be excluded, but an unsupported ligand bond cannot silently enter.

The frozen structural strata are:

- `p_direct_aromatic3`: all three P-bound carbon atoms use the source's aromatic representation.
- `p_direct_saturated3`: all three P-bound carbon atoms are nonaromatic and every incident bond at each is single.
- `p_mixed_or_unsaturated3`: all other eligible P–C3 structures; included in the pooled result but not used as a required replication family.

Kekulé aromatic structures are never called saturated; ambiguous Kekulé or unsaturated structures enter the mixed stratum. P–CH₂ and remote-bulk structures remain included. They cannot be removed after reveal.

## Frozen descriptor

For every eligible ligand:

1. Parse the source SMILES with the existing implicit-hydrogen expansion.
2. Construct exactly Rh(CO)(H)(L): Rh–C single, C≡O triple, Rh–H single, Rh–P single.
3. Root at the unique ligand phosphorus.
4. Set the horizon to `τ = 5`.
5. Propagate the root vector for exactly five steps.

The primary graph score uses unit edge weights:

\[
P_{ij}=\frac{1}{\deg(i)}, \qquad
S_5(P)=H_2(e_P^\top P^5).
\]

The physical-weighting test uses the frozen BDE table:

\[
P^{\mathrm{BDE}}_{ij}=\frac{E_{ij}}{\sum_kE_{ik}}, \qquad
S^{\mathrm{BDE}}_5(P)=H_2(e_P^\top(P^{\mathrm{BDE}})^5).
\]

No atom normalization, target transform, τ scan, graph repair, class removal, or alternative root is permitted after reveal.

## Frozen simple baselines

All non-3D baselines are computed on the ligand graph including the parser's implicit hydrogens:

- all-atom count, heavy-atom count, molecular weight;
- exact shell counts and cumulative ball counts for radii one through five;
- exact-step-five endpoint support;
- radius-five branch excess and mean P-to-atom distance;
- P degree and P BDE-weighted degree as zero-variance controls where applicable;
- Wiener, Randić, Zagreb M1, and Balaban J indices.

The null champion is the maximum absolute tie-aware Spearman correlation across this complete frozen set. `volume_boltz` and buried Sterimol B5 are reported as three-dimensional positive controls, not as simple graph nulls.

## Statistics

- Pearson `r` is descriptive.
- The primary statistic is Spearman `ρ` with average ranks for exact ties in both variables.
- Response-label permutations: 20,000, add-one p-value, fixed seed `0x5859504845525638` and deterministic derivations.
- Pooled permutations shuffle the target only within the three frozen structural strata.
- The incremental permutation statistic recomputes `ρ(candidate) - max |ρ(simple baseline)|` after every shuffle.
- Confidence intervals use 5,000 paired bootstrap samples. Values are resampled first and midranks are recomputed inside every replicate.
- Because constitutional duplicates are removed before reveal, a bootstrap row is one unique retained graph.
- Family analyses use separate permutations and bootstraps within `p_direct_aromatic3` and `p_direct_saturated3`.

## Frozen decision rule

The primary unweighted graph claim passes only if all conditions hold:

1. Pooled `ρ ≥ 0.50` and family-stratified permutation `p ≤ 0.001`.
2. Pooled advantage over the strongest simple baseline is `Δρ ≥ 0.10`, its 95% paired-bootstrap lower bound is above zero, and incremental permutation `p ≤ 0.001`.
3. Both required families contain at least 30 retained graphs.
4. Each required family has `ρ ≥ 0.70`, two-sided permutation `p ≤ 0.025`, and a 95% bootstrap lower bound above zero.
5. In each required family, the advantage over the strongest simple baseline is `Δρ ≥ 0.10` with a 95% bootstrap lower bound above zero.

The BDE-weighted descriptor is evaluated by the same rule, but it is secondary. Added physical information from BDE weighting passes only if the primary graph claim passes and

\[
\rho(S^{\mathrm{BDE}}_5)-\rho(S_5)\ge 0.05
\]

with a paired-bootstrap lower bound above zero.

Interpretation is frozen as follows:

- Primary passes, weighting fails: local finite-horizon topology is supported; BDE weighting is not.
- Association passes but baseline margin fails: ordinary local complexity explains the result; there is no distinct entropy-descriptor win.
- Either required family fails: the broad monophosphine generalization claim is falsified.
- Only target-independent QA/sample gates fail: inconclusive, not a pass.
- Any post-reveal parser, BDE, graph, τ, endpoint, cohort, or threshold change invalidates CHEM-V8 and requires a new experiment version.

## Pre-reveal artifacts

The final freeze commit, predictor SHA-256, code SHA-256, compiler version, and exact cohort counts are recorded in `CHEM-V8-Manifest.txt`. The target table and result report must not predate that commit.
