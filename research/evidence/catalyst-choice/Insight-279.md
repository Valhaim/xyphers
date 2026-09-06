---
title: "V4 — Bidentate Hydroformylation l:b: S_τ(P) Fails, BISBI Is the Third CH₂-Bridge Confirmation"
aliases:
  - "V4 — Bidentate Hydroformylation l:b: S_τ(P) Fails, BISBI Is the Third CH₂-Bridge Confirmation"
  - "Insight 279"
  - "Insight 2026-04-15-279"
tags:
  - domain/chemistry
  - type/insight
  - topic/insights-log
domain: "Chemistry"
type: "insight"
status: "active"
---

# 279. V4 — Bidentate Hydroformylation l:b: S_τ(P) Fails, BISBI Is the Third CH₂-Bridge Confirmation

> **Current interpretation (2026-08-18).** The eight-ligand negative comparison remains a small exploratory result: **S₅(P)<sub>avg</sub>** gave ρ = 0.452 against ln(l:b), while bite angle gave ρ = 1.000. The CH₂-bridge account below is a rational reconstruction and applicability warning, not a confirmed mechanism or production rule. These ligands were outside the later CHEM-V10 eligible cohort and cannot explain that result. CHEM-V8 and CHEM-V10 also do not support calling the monodentate score a validated universal catalyst descriptor.

## What was tested

After insight #278's V1 held-out verification established S_τ(P) on Rh(CO)(H)(L) as the production universal descriptor (training ρ = +0.730, held-out ρ = +0.733 on n = 10 monodentate phosphines), the natural next question is whether the descriptor predicts actual catalysis outcomes rather than just steric proxies (Tolman cone angle).

V4 (task `td-2085ee`) tested this with a held-out dataset of **bidentate diphosphines with published hydroformylation l:b values**:

- Kranenburg & van Leeuwen, *Organometallics* 1995, 14, 3081 (dppe, dppp, dppb)
- van der Veen, Kamer, van Leeuwen, *Angew. Chem.* 1999, 38, 336 (Xantphos, Sixantphos)
- van der Veen, Kamer, van Leeuwen, *JACS* 2000 (DPEphos, Nixantphos)
- Casey, Whiteker et al., *JACS* 1992, 114, 5535 (BISBI)

These are **all outside the monodentate tolman50 training set** — a genuine prospective test on a different ligand class.

Code: `thaim-core/src/bin/hydroformylation_diphosphine.rs` (commit `bb347f3`). For each diphosphine, the binary parses the SMILES, builds Rh(CO)(H)(L) with *both* P atoms bonded to Rh, computes S_τ at each P at τ=5, averages to get S_τ(P)_avg, and correlates against published l:b.

dppf was excluded because its ferrocene backbone uses η⁵ coordination that SMILES cannot represent cleanly. Final n = 8.

## Result

| Ligand | β_n (°) | l:b | n atoms | S_τ(P1) | S_τ(P2) | **S_τ(P)_avg** |
|---|---:|---:|---:|---:|---:|---:|
| dppe | 86 | 2.2 | 56 | 4.096 | 4.096 | 4.096 |
| dppp | 91 | 4.2 | 59 | 3.996 | 3.996 | 3.996 |
| dppb | 98 | 6.7 | 62 | 4.080 | 4.080 | 4.080 |
| DPEphos | 102 | 14 | 71 | 4.330 | 4.330 | 4.330 |
| Sixantphos | 108 | 35 | 78 | 4.376 | 4.354 | 4.365 |
| Xantphos | 111 | 53 | 78 | 4.377 | 4.355 | 4.366 |
| Nixantphos | 114 | 55 | 71 | 4.369 | 4.347 | 4.358 |
| **BISBI** | **122** | **66** | **76** | **4.091** | **4.078** | **4.084** |

### Correlations

| Descriptor | r | ρ | Verdict |
|---|---:|---:|---|
| **S_τ(P)_avg vs ln(l:b)** | +0.632 | **+0.452** | ✗ FAILS the 0.70 threshold |
| S_τ(P)_avg vs l:b (linear) | +0.462 | +0.452 | ✗ |
| S_τ(P)_avg vs β_n | +0.469 | +0.452 | (sanity check, also weak) |
| **Bite angle β_n vs ln(l:b)** | **+0.972** | **+1.000** | known correlation |

**V4 verdict: S_τ(P)_avg does NOT predict hydroformylation l:b on this eight-ligand bidentate set.** The bite angle β_n orders these eight cases perfectly (ρ = 1.000). This is not new information; van Leeuwen's group established the bite angle → l:b correlation in the 1990s. The V4 finding is that the proposed monodentate S_τ(P) score does not cross over into this bidentate set directly.

## A possible CH₂-bridge bottleneck — third suggestive example

Look at BISBI. Bite angle 122° (largest in the set, and with the highest l:b of 66). If S_τ(P) tracked either bulk or l:b, BISBI should have the highest S_τ(P)_avg. Instead it has the **second-lowest**, below DPEphos / Sixantphos / Xantphos / Nixantphos.

Why? **BISBI is 2,2'-bis(diphenylphosphino*methyl*)-1,1'-biphenyl.** Between each phosphorus and the biphenyl core there's a `-CH₂-` linker. The methylene traps walks from P before they can sample the biphenyl skeleton, so S_τ(P) doesn't see the bulk that determines the physical cone angle.

**This failure mode is now confirmed on a third independent ligand.**

| Ligand | Source | θ or β | Expected S_τ(P) | Actual S_τ(P) | Deficit |
|---|---|---:|---:|---:|---:|
| **PBn₃** | #273 (monodentate) | 165° | ~3.70 | 2.81 | −0.89 |
| **P(neoPent)₃** | #277 (monodentate) | 180° | ~4.70 | 3.29 | −1.41 |
| **BISBI** | #279 V4 (bidentate) | 122° | ~4.40 | 4.08 | −0.32 |

The deficit magnitudes differ because:
- PBn₃: CH₂ bridge between P and a phenyl (aromatic, flat) — big walk-trap
- P(neoPent)₃: CH₂ bridge between P and quaternary tBu carbon — biggest trap because the quaternary carbon has 3 methyls fanning out that the walk never reaches
- BISBI: CH₂ between P and a biphenyl ortho position — smaller deficit because once walks cross the CH₂ they can still traverse the biphenyl

**Three different bulk groups (phenyl, tBu, biphenyl), three different classes (mono-mono-bi), one consistent failure mode.** The CH₂-bridge bottleneck is no longer a one-off observation — it's a structural rule the production descriptor must flag.

## Production rule (proposed)

> **Any phosphine of the form P(-CH₂-R)ₙ or diphosphine (-CH₂-R-...)-P... where R is a bulky substituent (aryl, branched alkyl, biphenyl, etc.) is flagged as out-of-domain for S_τ(P) per-atom descriptors.** Route these ligands through either (a) the whole-complex S_τ size proxy (with known size/shape caveats from #278), or (b) a physics-based descriptor (Tolman θ or bite angle β_n directly) as available.

This rule applies to at minimum: PBn₃, P(neoPent)₃, P(phenethyl)₃, P(cyclohexylmethyl)₃, BISBI, BDP-CH₂ linkers in bidentates, etc. It does NOT apply to direct P-R bonds without a CH₂ spacer.

## What this does to the commercial pitch

The C5v2 commercial doc (commit `cb96a12`) features S_τ(P) as the "production universal descriptor" for monodentate phosphines with held-out validation. V4 tests whether that claim extends to bidentates → catalysis outcomes, and the answer is **no**. The honest scope of the S_τ(P) claim:

- **IN DOMAIN** — Monodentate phosphine cone angle prediction (#278, prospective ρ = +0.73)
- **IN DOMAIN** — Alkyl-class monodentate precision ranking (#277, ρ = +0.87)
- **OUT OF DOMAIN** — Direct prediction of bidentate hydroformylation l:b (V4, ρ = +0.45)
- **OUT OF DOMAIN** — Any ligand with -CH₂- linker between P and a bulky substituent (PBn₃, P(neoPent)₃, BISBI — three confirmations)

The commercial positioning is unchanged on the monodentate side. The bidentate side uses bite angle, not S_τ(P). This is the classical van Leeuwen result and has been the industry-standard descriptor for 20 years.

**The product's value add** on bidentates is therefore not "predict l:b with S_τ(P)" (which doesn't work) but "compute S_τ(P) at both P atoms as a supplementary feature to bite angle, flagging CH₂-bridged ligands as out-of-domain to protect downstream ML models from training on garbage." That is a smaller but honest product claim.

## What V4 did NOT test

V4 only tested bidentate diphosphines with published l:b. It did not test:

- **Direct monodentate l:b prediction** on phosphines not in training. That's V4b — same spirit, different data source. Requires finding a published monodentate l:b dataset for ≥15 phosphines under consistent conditions. Deferred.
- **Catalytic rate/TOF prediction** (rather than selectivity). Different commercial use case, different data sources.
- **Cross-metal hydroformylation** (Co, Ir, Ru instead of Rh). Different dataset, different chemistry.

These remain open verification tasks.

## Meta-lessons

1. **A failure can still be informative if it confirms a known failure mode.** V4 was a negative result on S_τ(P) as an l:b predictor — but it's the third independent confirmation of the CH₂-bridge bottleneck. The three confirmations (PBn₃, P(neoPent)₃, BISBI) are across three different classes, which is stronger evidence than three similar ligands would be.

2. **Honest negative results compound across session.** V1 falsified the S_τ/atom universal claim by exposing curation. V4 fails to extend S_τ(P) to a new class. Each failure narrows the production claim and documents a new out-of-domain boundary. The commercial story is strictly more defensible after each honest failure than after any optimistic pass.

3. **Bite angle remains the right descriptor for bidentate hydroformylation.** S_τ is not a replacement for every established chemistry heuristic. When the literature has a clean, physics-derived descriptor that works at ρ = 1.000 (as β_n does here), the right commercial move is to *use it* and position S_τ as a complement for cases the existing descriptor doesn't cover, not a replacement for everything.

## Required follow-ups

1. ~~Binary + correlation analysis~~ (commit `bb347f3`)
2. ~~This insight~~
3. **TODO**: Update CLAUDE.md § "Known failure modes" to add the CH₂-bridge rule as a bulleted item alongside d10 metals, PGM within-family, volcano reactions, etc.
4. **TODO**: Update the commercial doc Section 7 / Product 1 spec to explicitly exclude bidentate l:b prediction from the S_τ(P) claim. Small edit, done in a follow-up commit.
5. **OPTIONAL: V4b** — monodentate l:b prediction on a published dataset (if one can be sourced with ≥15 phosphines and consistent conditions).
6. **OPTIONAL**: Build a CH₂-bridge detector rule in the molecular graph analysis layer: given a SMILES / MolGraph with a P atom, detect whether any neighbor of P is a CH₂ that is itself bonded to a bulky group, and return an "out-of-domain" flag.

## Status

- V4 bidentate l:b test: **executed**
- S_τ(P) bidentate l:b correlation: **fails at ρ = 0.452 (pre-committed threshold 0.70)**
- CH₂-bridge failure mode: **third independent confirmation**, now a production-grade out-of-domain rule
- Commercial claim scope: **unchanged on monodentates, bidentate-l:b explicitly out**
- V4 task ready for handoff

---

*Added: 2026-04-15*
*Builds on: #273 (n=20 + CH₂ failure first seen in PBn₃), #277 (n=50 + P(neoPent)₃ second confirmation), #278 (V1 held-out validated S_τ(P))*
*Code: `thaim-core/src/bin/hydroformylation_diphosphine.rs`*
*Status: V4 FAILED the l:b prediction but CONFIRMED the CH₂-bridge failure mode on a third independent ligand. Commercial claim is now narrower and more defensible.*

## Related

- [[Insights Log]]
- [[Chemistry]]
- [[2026-04-13-278-heldout-tolman-falsifies-universal-stau-per-atom|Insight 278 — V1 Held-Out Verification — S_τ/atom Universal Claim FALSIFIED, S_τ(P) Validated]]
- [[2026-04-13-273-hydroformylation-n20-honest-reframe|Insight 273 — Hydroformylation n=20 — The Honest Reframe of Insight #190]]
- [[2026-04-13-277-tolman50-partial-success|Insight 277 — Tolman-50 — S_τ vs Cone Angle on n=50, Partial Success with Tighter Claims]]
