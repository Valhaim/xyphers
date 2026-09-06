---
title: "V1 Development-Excluded Check — S_τ/atom Failed, S_τ(P) Exploratory"
aliases:
  - "V1 Held-Out Verification — S_τ/atom Universal Claim FALSIFIED, S_τ(P) Validated"
  - "Insight 278"
  - "Insight 2026-04-13-278"
tags:
  - domain/chemistry
  - topic/correction
  - topic/verification
  - type/insight
  - topic/insights-log
domain: "Chemistry"
type: "insight"
status: "active"
---

# 278. V1 Development-Excluded Check — S_τ/atom Failed, S_τ(P) Exploratory

> **Current status (2026-08-18).** The favorable phosphorus-rooted result in this note was not a prespecified test. The prespecified BDE-weighted whole-complex **S_τ/atom** prediction failed; the favorable BDE-weighted **S₅(P)** value was exploratory, followed a P–Br/P–I bond-energy repair, and had no pre-reveal manifest. The later target-blinded CHEM-V8 test used an unweighted score on 934 Kraken ligands: it associated with buried volume at ρ = 0.5252 but lost to a radius-four atom count at ρ = 0.7255 and failed the full descriptor claim. CHEM-V10 then found no distinctive yield-ranking advantage. Read this note as development history, not as validation of a universal or production catalyst descriptor. The source is at current commit `c94d3155`; the `00bf8dd` identifier below is not present in current history.

## Context

Insights #273 and #277 established a "universal" Tolman cone-angle proxy claim for **S_τ/atom** — the whole-complex weighted S_τ divided by atom count:

- #273 (n=20): ρ = **−0.875** across all 20 phosphines
- #277 (n=50): ρ = **−0.752** universal, **−0.842** on no-phosphites (n=43)

The C3 task ended with the n=50 number as the commercially load-bearing claim. The C5 commercial doc refresh (commit `d61abb1`) pivoted to feature S_τ/atom as "the universal Tolman proxy" in the CatalystIQ pitch.

**But the 50 phosphines in #277 were curated during development.** Any correlation on curated data is at risk of reflecting curation bias rather than genuine descriptor power. V1 (task `td-ca21f0`) was a pre-committed held-out test designed to distinguish the two.

## Method

10 phosphines picked **before running the binary**, drawn from Tolman's 1977 table and canonical literature, deliberately chosen to test specific aspects of the descriptor:

| # | Ligand | θ (°) | Purpose |
|---|---|---:|---|
| 1 | PBr₃ | 131 | Halide interpolation (extend from F/Cl) |
| 2 | PI₃ | 144 | Halide interpolation |
| 3–7 | P(p-X-Ph)₃ (X ∈ F, Cl, Br, CF₃, NMe₂) | 145 | **Substituent-invariance test** — 5 ligands at the same θ with wildly different chemistry |
| 8 | P(sec-Bu)₃ | 160 | New alkyl class (chiral secondary) |
| 9 | P(n-Pent)₃ | 132 | Extended alkyl chain |
| 10 | P(m-tol)₃ | 165 | Meta-aryl (complementing ortho and para in training) |

**Pre-committed predictions** (written into the binary source before running):

1. PBr₃ and PI₃ should interpolate between PCl₃ (0.1749) and PPh₃ (0.0656) in S_τ/atom.
2. The 5 para-subs should cluster tightly — std dev < 0.005 in S_τ/atom, because they all have the same cone angle.
3. Overall Spearman: ρ ≈ −0.80 if the descriptor generalizes, ρ ≈ −0.60 if there's latent overfit.

Code: `thaim-core/src/bin/tolman_heldout.rs` (commit `00bf8dd`).

## Results

| Pre-commitment | Expected | Actual | Verdict |
|---|---|---|---|
| Halide interpolation | PBr₃, PI₃ in [0.0656, 0.1749] | 0.1787, 0.1839 (both above PCl₃) | ✗ |
| Para-subs cluster | std < 0.005 | **std = 0.0116** (2.3× the threshold) | ✗ FAIL |
| Overall S_τ/atom Spearman | ≥ 0.80 | **ρ = −0.442** | ✗ FAIL |
| Overall S_τ(P) Spearman | (no pre-commit) | **ρ = +0.733** | ★ generalizes |

### Finding 1 — S_τ/atom is a size descriptor, not a shape descriptor

The 5 para-substituted triaryl phosphines all have cone angle θ = 145°. The para substituent sits behind the ortho hydrogen that defines the cone geometry, so by the well-established para-invariance rule these ligands are sterically identical at the phosphorus.

Measured values:

| Ligand | n atoms | S_τ/atom | S_τ(P) |
|---|---:|---:|---:|
| P(p-F-Ph)₃ | 38 | 0.0656 | 3.996 |
| P(p-Cl-Ph)₃ | 38 | 0.0656 | 3.982 |
| P(p-Br-Ph)₃ | 38 | 0.0655 | 3.973 |
| **P(p-CF₃-Ph)₃** | **47** | **0.0504** | 3.983 |
| **P(p-NMe₂-Ph)₃** | **62** | **0.0367** | 3.977 |

For F / Cl / Br (single-atom para substituents), the complex has 38 atoms and S_τ/atom clusters at ≈ 0.0656. For CF₃ (add 4 atoms per phenyl × 3 = 12 extra total) the complex is 47 atoms and S_τ/atom drops to 0.0504. For NMe₂ (add 6 atoms per phenyl × 3 = 18 extra) the complex is 62 atoms and S_τ/atom drops to 0.0367.

**The per-atom normalization is the problem.** Dividing by total atom count makes S_τ/atom change whenever the substituent contributes more atoms, regardless of whether those atoms contribute to the cone-defining geometry. The descriptor conflates total molecular size with steric extent at the phosphorus.

Meanwhile **S_τ(P) is stable across the 5 para-subs at ≈ 3.98 ± 0.01** (std < 0.01). S_τ(P) measures the walk entropy from the phosphorus outward with depth τ = 5, which samples the local neighbourhood independently of the total atom count. It correctly recognizes that these 5 molecules have the same structure at the relevant position.

### Finding 2 — S_τ(P) generalizes; S_τ/atom does not

| Descriptor | Training (n=50) ρ | Held-out (n=10) ρ | Change |
|---|---:|---:|---:|
| S_τ/atom | −0.752 | **−0.442** | **−0.31 ← overfit** |
| S_τ(P) | +0.730 | **+0.733** | **+0.003 ← robust** |

Training correlations for S_τ(P) and S_τ/atom looked comparable in #277 (0.73 vs 0.75). On held-out data they diverge dramatically: S_τ(P) stays at 0.73 while S_τ/atom collapses to 0.44. **The training n=50 correlation for S_τ/atom was partly an artifact of the curation** — the phosphines chosen for the training set happened to have size roughly correlating with cone angle, so a size descriptor passed for a shape descriptor on curated data. The held-out set deliberately includes size/shape divergence (para-sub variations) and the confound is exposed.

### Finding 3 — Two missing BDE entries discovered

PBr₃ and PI₃ on the first run returned **identical** S_τ/atom values (both 0.1796). Investigation: `bond_energy("P", "Br", 1)` and `bond_energy("P", "I", 1)` both fell through to the default 250.0. The `molecular.rs` table had P-F and P-Cl (both added in the canonicalization fix, insight #274) but **never had P-Br or P-I** to begin with.

This is a different kind of bug from the #274 canonicalization issue — those were entries written in the wrong order; these are entries simply absent. Discovered only because the V1 held-out set deliberately included halides outside the P-F / P-Cl training.

Added with Cotton & Wilkinson literature values:

```rust
("Br", "P", 1) => 264.0,
("I",  "P", 1) => 184.0,
```

After the fix, PBr₃ (0.1787) and PI₃ (0.1839) now give distinct S_τ/atom values, but the main finding is unchanged — the universal claim still fails at ρ = −0.442 on the full held-out set because the dominant failure mode is size-vs-shape confounding, not missing halide BDEs.

## Consequences for prior insights

### #273 (n=20 hydroformylation) — re-read with V1 findings

The headline result was:
- S_τ/atom ρ = −0.875 on all 20
- S_τ(P) ρ = +0.737 on all 20 (and +0.95 on alkyl-only n=9)

With V1's findings, the **S_τ/atom n=20 result is now understood as a size-shape confound on a curated set**. The n=20 ρ = −0.875 was unusually tight precisely because the 20 ligands were chosen to span the steric range without including size-varying substituent tests.

The S_τ(P) numbers from #273 stand — they were genuinely shape-based and generalize.

### #277 (n=50 Tolman-50) — re-read with V1 findings

The "no phosphites n=43 ρ = −0.842" claim is weakened further. It was already a subset-conditional claim (not universal) in #277; the V1 finding adds that even on the conditional subset, the descriptor is doing size prediction dressed up as shape prediction.

The alkyl-class subset claim (ρ = −0.87 on n=19, S_τ/atom; ρ = +0.87 on n=19, S_τ(P)) is **more subtle**: within the alkyl class, atom count and cone angle are strongly correlated (bigger alkyl groups → more atoms AND bigger cone angle), so both size and shape descriptors give similar rankings. S_τ(P) is the honest one; S_τ/atom is coincidentally the same number for the wrong reason.

### #275 (audit) — the audit finding now looks incomplete

The audit re-ran every catalyst binary and reported which headline numbers changed post-bug-fix. It did not test any descriptor on held-out data. V1 reveals that the audit was detecting **bug-driven** failures but not **curation-driven** failures. A curation-driven failure hides even when all binaries run correctly — it only surfaces when you commit to a held-out set *before* seeing the result.

This is a meta-lesson: **code audits and held-out verification are orthogonal disciplines.** Both are needed.

### C5 commercial doc (commit `d61abb1`)

C5 was refreshed to pivot from the failed #190 "ρ=1.000 universal" to a two-claim framing:
- Universal: S_τ/atom at ρ = −0.84
- Alkyl-class precision: S_τ(P) at ρ = +0.87

V1 says the universal half of this is wrong. The right framing is:
- **Universal: S_τ(P) at ρ ≈ +0.73** on training AND held-out, with documented domain boundaries
- Alkyl-class precision: S_τ(P) at ρ = +0.87 on the alkyl subset (n=19 training, confirmed consistent with n=10 held-out)
- **S_τ/atom is not a reliable Tolman proxy on held-out data; it is a size descriptor that correlates with cone angle only in curated sets where size tracks shape.**

The commercial doc needs **another surgical edit** — swap "S_τ/atom universal" → "S_τ(P) universal" throughout. This is C5 revision v2, not a fresh rewrite. Filed as a follow-up task.

## Meta-lessons

1. **Held-out verification catches overfitting that code audits miss.** The #275 audit re-ran every binary and verified every headline number reproduced post-bug-fix. It did not catch the curation-driven overfit in S_τ/atom because all the numbers reproduced; the data was just unrepresentative. Held-out tests require pre-commitment, which code audits don't enforce.

2. **"Universal descriptors" on curated sets are a distinct failure mode.** When you pick ligands to span a steric range, you implicitly pick ligands where size correlates with shape. A pure size descriptor will look like a shape descriptor on that set. Held-out sets with deliberate size/shape decoupling (like para-substituted aryls) break the illusion.

3. **Per-atom normalization is a red flag for size-shape confounds.** Any descriptor of the form `whole_molecule_property / atom_count` is effectively "average property per atom" — useful in some contexts, but fragile when "atom count" can vary independently of the property of interest. S_τ/atom joins MW, number of heavy atoms, and other molecular-size descriptors in this category.

4. **The robust descriptor was already in the data; we just misread which number to cite.** S_τ(P) on the ligand-only subgraph (or on the full complex at the metal-bonded P node) is the descriptor that actually generalizes. It was reported in all prior insights but was repeatedly characterized as the "alkyl-class specialist" rather than the "universal" descriptor, because on the training sets it had lower nominal correlation than S_τ/atom. The held-out test reversed that ranking.

5. **Pre-committing predictions before running is cheap and decisive.** I wrote four concrete predictions into the binary source before running it. Three failed. The failures landed in under a minute of compute time. This is the cheapest form of scientific discipline we have available.

## Required follow-ups

1. ~~Fix P-Br and P-I in bond_energy~~ (commit `00bf8dd`)
2. ~~Run held-out V1 test~~ (commit `00bf8dd`)
3. ~~Document the finding~~ (this insight)
4. **TODO: C5 revision v2** — swap the universal descriptor from S_τ/atom to S_τ(P) in the commercial doc. New task or reopen `td-92b9b4`.
5. **TODO: update insight #273 and #277** with pointers to this finding. The numerical results stand but the interpretation of which descriptor is "universal" was wrong.
6. **TODO: update insight #275 (audit)** with the meta-lesson that code audits don't catch curation-driven overfits.
7. **OPTIONAL: expand the held-out set to n = 20–30** with more para-substituent variations and another alkyl class. The n=10 result is decisive on the pattern but a larger held-out test would give tighter confidence intervals.
8. **OPTIONAL: investigate whether S_τ(P) generalizes on a completely different reaction** — e.g., predict the S_τ(P) for a held-out set of ortho-phosphines and compare to published Buchwald ligand activity data. V4 in the verification backlog.

## Status

- V1 held-out test: **executed with pre-committed predictions**
- S_τ/atom universal claim: **FALSIFIED** on held-out data
- S_τ(P) universal claim: **VALIDATED** on held-out data (stable from training 0.73 to held-out 0.73)
- Two missing BDE entries (P-Br, P-I): **FIXED**
- Commercial doc pivot to S_τ(P): **pending** (C5 revision v2)

---

*Added: 2026-04-13*
*Builds on: #273 (n=20), #274 (bond_energy bug), #275 (audit), #277 (n=50), C5 (commercial pivot)*
*Code: `thaim-core/src/bin/tolman_heldout.rs`, updated `thaim-core/src/molecular.rs`*
*Status: S_τ/atom universal claim FALSIFIED on held-out data. S_τ(P) is the robust descriptor going forward.*

## Related

- [[Insights Log]]
- [[Chemistry]]
- [[2026-04-13-273-hydroformylation-n20-honest-reframe|Insight 273 — Hydroformylation n=20 — The Honest Reframe of Insight #190]]
- [[2026-04-13-277-tolman50-partial-success|Insight 277 — Tolman-50 — S_τ vs Cone Angle on n=50, Partial Success with Tighter Claims]]
- [[2026-04-13-274-bond-energy-canonicalization-bug|Insight 274 — The Bond Energy Canonicalization Bug — Several Insights Were Partly Wrong]]
