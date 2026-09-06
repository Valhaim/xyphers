# CHEM-V10 result: does not pass

CHEM-V10 **does not pass** the preregistered catalyst-yield test. Target and shape QA passed, but all five inferential gates failed.

This was the one and only CHEM-V10 evaluation. The protocol was committed in `b051a9a0f6939af469b26026e9d98cad20f95eac` and its manifest in `47afd23075b4bc58237ca22bf210dbb083ea99be`; both were pushed before the run. No model variant or rescue analysis was run.

## Primary result

| Quantity | Observed | Required | Gate |
|---|---:|---:|---|
| Candidate `S_5(P)` score, `T` | 0.156148 | — | — |
| Exact-null mean `T` | 0.150604 | — | — |
| Null-centered excess, `E` | 0.005544 | at least 0.05 | fail |
| Candidate exact permutation `p` | 0.039063 (270/6,912) | at most 0.001 | fail |
| Advantage over strongest baseline, `delta` | -0.040253 | at least 0.05 | fail |
| Baseline-competition exact `p` | 0.039063 (270/6,912) | at most 0.001 | fail |
| 95% experiment-cluster bootstrap interval for `delta` | [-0.063511, -0.016179] | lower bound above 0 | fail |

The strongest frozen baseline was the ordinal coded ligand slot, with `T = 0.196401`. The candidate's 95% bootstrap interval for `T` was `[0.122249, 0.189032]`.

Across the 864 primary blocks, candidate correlations were positive in 287, negative in 284, and zero in 293. The fraction positive among nonzero correlations was 0.502627 and the experiment-balanced signed mean rho was 0.008839. This is essentially directionless. The preregistered movers-only non-rescue sensitivity was similar: `T = 0.155634`, signed mean rho `= 0.008340`, with all 39 mover structures, 14 experiments, and 864 blocks retained.

## Integrity and numerical audit

The evaluator verified 49,632 exact finite loaded targets, 17,632 eligible targets, 1,056 position-training blocks, 864 primary blocks, 14 experiments, 45 structures, 19 permutation-movable structures, and all 6,912 exact mappings. The QA gate passed.

NumPy emitted six `matmul` warnings at the two frozen rank-correlation sites: divide-by-zero, overflow, and invalid-value warnings at each site. A post-result integrity audit examined all 96,768 mapping-by-experiment operand pairs without changing or rerunning the analysis:

- every operand and denominator was finite `float64`;
- centered rank operands were bounded within `[-17.5, 17.5]`;
- the conservative maximum numerator bound was 11,025, versus a float64 maximum near `1.8e308`;
- positive denominators ranged from 27.495 to 3,883.9999;
- zero denominators were expected from 288 all-tied outcome blocks and constant `shell_r1` / `ball_r1` predictors, and were masked by the frozen `where=denominators != 0` operation;
- a warned matrix multiplication returned only finite values and matched explicit elementwise multiply-and-sum exactly, with maximum difference 0;
- strict parsing found 2,552 JSON numbers and zero nonfinite values.

The warnings are stale NumPy/Accelerate floating-point exception flags, not arithmetic failure. The result remains numerically valid.

## Interpretation

`S_5(P)` carries a small association in this catalyst screen, but it is only 0.0055 above its exact-null mean and is substantially weaker than the ligand's coded plate slot. The signs split almost exactly in half. On the frozen test, there is no distinctive entropy advantage over ordinary molecular and placement controls.

This result does not support the claim that the tested Xypher descriptor reveals thermodynamic circuitry, an equation of state, or a new physical law in these catalyst data. It also does not prove that every possible Xypher formalization has zero chemical relevance. It rejects this preregistered operationalization on this observational ORD design; strict future confirmation would require new independently randomized ligand-yield data.

Source: Das et al., *A 50,688-Reaction Data Set Reveals General Ligands and Mechanistic Diversity in C-N Couplings*, <https://pubs.acs.org/doi/10.1021/jacs.6c05959>.

Machine-readable result SHA-256: `5387f8a21497be7e671c8e8a93b263df4998092b811802f927e93d77f95b633d`.
