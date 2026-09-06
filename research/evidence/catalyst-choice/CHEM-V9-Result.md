# CHEM-V9 result: inconclusive integrity stop

CHEM-V9 is **inconclusive**. It did not compute a confirmatory association, margin, permutation p-value, or bootstrap interval.

The blinded analysis was frozen in commit `bfb3bdbb0c4c63daf7f530df5522bdd46ffd7a10`, its manifest was frozen in `0b1049488f12cfbdc23b0935b185dde29040de85`, and both were pushed before target extraction. At freeze time the preparer had stripped 50,688 outcome envelopes and decoded zero outcomes. All 20 synthetic and structural tests passed.

The reveal then extracted all 49,632 required ligand-loaded yields. The target file has SHA-256 `58dae5feac0aad193b3a8d6590120b5abfea02447aed093e4b146ccaf9a8f407`.

## Integrity failure

The frozen evaluator stopped while constructing the mandatory leave-one-experiment-out slot control:

```text
ValueError: slot 48 has no cross-fit training coverage for XZ-01-1
```

The structural cause is exact and target-independent. `loaded_memberships.tsv` contains only one row coded as ligand slot 48: the already frozen layout anomaly `ord-1649402b9d164b50b298f00e1df91351` in `XZ-01-1`. Its 47-row block has slots 1–15 and 17–48: slot 48 substitutes for the ordinary slot 16. L106 is not in the primary ligand cohort, but the frozen evaluator applies its coverage check to the union of every loaded training slot. No other experiment can train a held-out score for slot 48 when `XZ-01-1` is held out.

The preregistration states that missing required slot coverage makes CHEM-V9 inconclusive with no imputation, and that any post-reveal change to the cohort, position control, evaluator, or decision rule creates CHEM-V10. The frozen evaluator SHA-256 is `731fc7078ad306d6a363d4cd2b844dc4fb7721009cb72f0e289c784a55133729`. Consequently, no model was repaired and no alternative statistics were searched.

## Meaning

This is not evidence for the Xypher catalyst hypothesis and not evidence against it. The confirmatory test did not execute past its integrity gate. The failure identifies a protocol defect that should have been detected from the target-free plate audit before reveal.

The source remains the Das et al. ORD catalyst screen: <https://pubs.acs.org/doi/10.1021/jacs.6c05959>.
