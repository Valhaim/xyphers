# CHEM-V8 blinded Kraken result

Evaluable ligands: **934**. Horizon: **τ=5**.

## Primary target: Boltzmann-average buried volume

| Predictor | Pearson r | tie-aware Spearman ρ |
|---|---:|---:|
| BDE-weighted Sτ(P), τ=5 | +0.3988 | +0.5195 |
| Unweighted Sτ(P), τ=5 | +0.4003 | +0.5252 |
| All-atom count | +0.4840 | +0.5429 |
| Heavy-atom count | +0.4067 | +0.4150 |
| Molecular weight | +0.4016 | +0.4201 |
| Shell-1 count | +0.0000 | +0.0000 |
| Shell-2 count | +0.1832 | +0.2024 |
| Shell-3 count | +0.4296 | +0.5131 |
| Shell-4 count | +0.4543 | +0.5202 |
| Shell-5 count | +0.3966 | +0.3748 |
| Ball-1 count | +0.0000 | +0.0000 |
| Ball-2 count | +0.1832 | +0.2024 |
| Ball-3 count | +0.4389 | +0.5355 |
| Ball-4 count | +0.5938 | +0.7255 |
| Ball-5 count | +0.6092 | +0.6653 |
| Exact-step-5 endpoint support | +0.4809 | +0.4971 |
| Radius-5 branch excess | +0.5386 | +0.6144 |
| Mean P-to-atom distance | +0.2843 | +0.2746 |
| P degree | +0.0000 | +0.0000 |
| P BDE-weighted degree | +0.0000 | +0.0000 |
| Wiener index | +0.2666 | +0.5031 |
| Randić index | +0.4755 | +0.5236 |
| Zagreb M1 | +0.4873 | +0.5419 |
| Balaban J | -0.0511 | +0.0230 |
| DFT molecular volume (positive control) | +0.4443 | +0.4830 |
| Buried Sterimol B5 (positive control) | +0.3664 | +0.3713 |

### Unweighted Sτ(P)

ρ=+0.5252, 95% CI [+0.4705, +0.5754], permutation p=0.000050. Association threshold: **PASS**.

Advantage over the bootstrap-strongest simple baseline: Δρ=-0.2003, 95% CI [-0.2394, -0.1631], family-stratified incremental permutation p=0.999700. Incremental threshold: **FAIL**.

P–direct-saturated3: n=141, ρ=+0.6327, CI [+0.5185, +0.7238], p=0.000050. P–direct-aromatic3: n=329, ρ=+0.3483, CI [+0.2386, +0.4492], p=0.000050. Family replication: **FAIL**.

Within-family margin over the strongest simple baseline: alkyl Δρ=-0.0869, CI [-0.2152, -0.0312]; aryl Δρ=-0.4777, CI [-0.5868, -0.3726].

### BDE-weighted Sτ(P)

ρ=+0.5195, 95% CI [+0.4648, +0.5703], permutation p=0.000050. Association threshold: **PASS**.

Advantage over the bootstrap-strongest simple baseline: Δρ=-0.2060, 95% CI [-0.2458, -0.1682], family-stratified incremental permutation p=0.999700. Incremental threshold: **FAIL**.

P–direct-saturated3: n=141, ρ=+0.6209, CI [+0.5052, +0.7182], p=0.000050. P–direct-aromatic3: n=329, ρ=+0.3260, CI [+0.2170, +0.4313], p=0.000050. Family replication: **FAIL**.

Within-family margin over the strongest simple baseline: alkyl Δρ=-0.0988, CI [-0.2268, -0.0381]; aryl Δρ=-0.5001, CI [-0.6127, -0.3925].

## Physical-weighting test

Weighted minus unweighted ρ: **-0.0057**, 95% paired-bootstrap CI **[-0.0108, -0.0009]**. Preregistered weighting advantage: **FAIL**.

## Secondary target: Boltzmann-average buried Sterimol B5

Complete cases: n=934. Weighted Sτ(P): r=+0.4173, ρ=+0.3400. Unweighted Sτ(P): r=+0.4137, ρ=+0.3304.

## Frozen decision

- Primary unweighted topology-descriptor claim: **FAIL**.
- Secondary weighted graph-descriptor claim: **FAIL**.
- BDE weighting adds physical information: **FAIL**.

Passing the graph claim requires ρ≥0.50 with permutation p≤0.001, an advantage of at least 0.10 over the strongest preregistered simple topology baseline with bootstrap lower bound above zero, and ρ≥0.70 with p≤0.025 plus the same baseline margin in both P–direct-saturated3 and P–direct-aromatic3 families (n≥30 each).
