---
title: THAIM Reward Result
aliases:
  - CAL-THAIM-1 Result
  - Can THAIM Train an Agent Result
tags:
  - domain/physics
  - type/result
  - topic/thermodynamics
  - topic/xypher
  - topic/learning
domain: Physics
type: result
status: complete
td: td-8ef64f
created: 2026-10-13
updated: 2026-10-13
related:
  - "[[THAIM Reward Boundary]]"
  - "[[The Information Law]]"
---

# CAL-THAIM-1 result: Can THAIM train an agent?

## 1. Verdict

**OVERALL PASS.** All thirteen gates passed, on 78 of 78 evaluated tables. All
six deliberately broken controls failed first at the gate the boundary named.

| hypothesis | verdict | evidence |
|---|---|---|
| H1 the THAIM rule | **supported** | `NET-full` fuels exactly the builds whose futures ratio exceeds `b_c`: agreement 1.0000 in all six worlds |
| H2 THAIM pays only where physics says | **supported** | Warm worlds: `R_net(NET-full)/R_net(ORACLE)` is 0.99997, 0.99997, 0.9998 and 0.9996. Cold worlds: `J(NET-full)` is 0.000054 and 0.000066, against `J_LOCAL` of 2.25 and 2.72 |
| H3 receipts price futures too cheaply | **supported** | In every warm world, `RECEIPT-full` fuels non-paying builds (weight 0.23 to 0.37) and loses net THAIM (for example W1: -0.222 against +0.435) |
| H4 THAIM beats hand-written aim | **refuted in W1** | W1: `E(NET-full)` 1.5880 < `E(LOCAL)` 1.6195. W2, W3, W5: 1.611 > 1.538, 2.167 > 1.742, 2.482 > 1.820 |
| H5 the map is worth what the rule needs | **supported** | W3 and W5: `NET-local` idles (`R_net` about -2e-5) while `NET-full` earns 0.095 and 0.070. W1 and W2: `NET-local` equals `NET-full` |

Report SHA-256: `a419d1cf735e10daea9b77d083ae08535fb3d9c028eaaeeb9e8486a3c7a8f0a4` (138 lines).

## 2. What was learned

**THAIM, booked on the agent's own channels, is a consistent training signal.**
A model given nothing but `R_net = alpha · Sdot_A - J` learned one rule in every
world:

> Fuel a bridge exactly when it multiplies the traveller's futures by more than
> the cold store's factor per packet.

- **No constant was tuned.** The rule was predicted before any declared world
  was solved. It held with agreement 1.0000 across six worlds, two horizons and
  four temperatures, and from two initializations.
- **It does not depend on the fuel's temperature.** W1 and W2 have different
  hot stores and traveller speeds, yet they learned the same rule and earned the
  oracle's value.
- **Where no build pays, the agent does nothing** (W4, W6).
- **Booked for the whole world, THAIM carries no signal.** Theorem 1 says so,
  and G13 confirmed it: the world-form agent idled everywhere.
- **Receipts are not a reward.** Paid one-sided, they priced futures too
  cheaply in every warm world and bought fuel the THAIM price would not. They did
  so in the cold world W4 as well: J 0.615 and net THAIM -0.103, where the net
  form idles.

**The refutation: the THAIM price is not the matched-power edge.** In W1,
hand-written aim spends its fuel on every bridge at the traveller, including
builds worth less than their timber. It reaches a matched-power edge of 1.6195,
against 1.5880 for the THAIM agent. The two yardsticks value futures
differently:
- THAIM prices the logarithm of the futures count at the world's temperature;
- the edge counts futures linearly against uniform warming at the same fuel.

The pilots had already shown that the edge rewards selectivity beyond the
THAIM price. In this run, the `c = 3/4` agent in W1, which is more selective,
reached `E = 1.97`. The THAIM price is a thermodynamic exchange rate, not an
edge-maximizer.

## 3. Reported, with no hypothesis

**Distance to the floor.** THAIM-trained agents sit much closer to the
Horowitz-Zhou-England floor than either fixed baseline:

| world | `eta(NET-full)` | `eta(LOCAL)` | `eta(GLOBAL)` |
|---|---|---|---|
| W1 | 0.714 | 0.375 | 0.389 |
| W2 | 0.755 | 0.398 | 0.437 |
| W3 | 0.517 | 0.322 | 0.327 |
| W5 | 0.455 | 0.290 | 0.292 |

They pay 1.3 to 1.4 times the floor in the warm worlds with `b_c = 5`. Local
fuel pays 2.5 to 2.7 times.

**The small network.** `MLP-full` mostly collapsed to idling:
- partial rule in W1 at seed 1 (`R_net` 0.078, agreement 0.713);
- partial rule in W2 at seed 0 (0.055, 0.727);
- idle at every other seed and world.

The tabular model learned the rule everywhere. Gradient training on a shared
network falls into the "dead policy", as the design pilots had shown.

**Price scan.**
- **At `c = 3/2`**, the agent learns the same rule as `c = 1` in W1 and W2,
  because the window between ratios 2.625 and 8 holds no ratio. In W3 to W6 it
  over-fuels like receipts, with negative net THAIM.
- **At `c = 3/4`**, the agent is more selective in W1 and W2 (`E` 1.97 and
  2.06), idles in W3, and nearly idles in W5.

**Other reported results:**
- The initialization `u = 1/4` learned the same rule as `u = 1` in every world.
- `NET-blind` idled in every world: without an observation it cannot tell which
  builds pay.

**Receipt agreement with the isolated-channel threshold `theta_R`:**

| world | W1 | W2 | W3 | W4 | W5 | W6 |
|---|---|---|---|---|---|---|
| agreement | 0.849 | 0.975 | 0.987 | 0.595 | 1.000 | 1.000 |

## 4. Review chronology

1. **Pilots.** Design pilots ran on `tau = 2` worlds, together with the
   frontier pilot. These are declared in boundary section 11.
2. **Boundary draft.** Theory values were computed from combinatorics only.
3. **Blind referee, round 1: MAJOR REVISION.**
   - The optimizer budget decided H3(a).
   - No gate order or control witnesses were declared.
   - H6 used sign-dependent tolerances.
4. **Calibration** on non-declared analogues only. It set T = 8000 and learning
   rate 0.2 by a stated rule, and is disclosed with the script hashes.
5. **Round 2: MINOR REVISION.**
   - C04 against the scaled forms. The fix declares `b_alpha`.
   - The calibration disclosure. This replaced a tuned J-ratio cut in H3 with
     the theory mechanism.
6. **Round 3: MINOR REVISION.**
   - H3 needed a positivity precondition.
   - The N = 4 receipt-window statement needed correcting.
7. **Round 4: ACCEPT.**
8. **Boundary frozen** at commit `6af3757852beb0a5594f45872482a1265bfea624`
   (SHA-256 `bfac95e0ecefb98e57bc6bc82ce088b38ef79c6a339dca5347979c0a8642744c`).
9. **Apparatus built under embargo.** Embargo-safe tests ran on non-declared
   worlds only.
10. **First static review.** Three static reviewers (boundary conformance,
    exactness, training and controls) approved
    `b12663832330a0fdcc500e128de7033c51e711ae`.
11. **Fix commit.** Their non-blocking findings were applied as
    `637d8e0ae1ae0f9df0f4609c1969efe2bcf1629a` (apparatus tree
    `da4f86d667b907b0a8349a9ba4030bdbbb12c2ff`).
    - Tables are written to disk with SHA-256.
    - G09 compares digests.
    - Enclosures are reported with radius, and the floor is reported.
    - The final objective change is reported.
    - The UNDECIDED label was added.
    - The exact `theta_R` classification was added.
    - G06 gained the ambiguous-key identity.
    - A FIRST FAILURE line was added.

    All three reviewers re-approved this exact hash. They confirmed that the
    trained rates are identical to those at `b126638`.
12. **The single run** started with HEAD equal to the remote:
    - 2026-10-13T17:44:29Z to 18:08:26Z, real 1437 s;
    - python 3.9.6, numpy 2.0.2, scipy 1.13.1, python-flint 0.6.0, Darwin arm64.
13. **Independent check:** section 6.

**Non-blocking notes carried from the static reviews:**
- File names in `tables/` replace spaces in agent names with `_` and `/` with
  `-` (for example `NET-full_c=3-4.txt`).
- The `tail` columns in `pilots/calib.txt` and `pilots/receipt_rule.txt` use
  the definition before `637d8e0`: the last minus the first of the last 100
  pre-update objective values. The report uses the corrected definition.
- The MLP's learning-rate schedule runs over `t = 0..T-1`, as in the frontier
  pilot.

## 5. Files

| file | SHA-256 or commit |
|---|---|
| boundary | `bfac95e0ecefb98e57bc6bc82ce088b38ef79c6a339dca5347979c0a8642744c` |
| apparatus | commit `637d8e0`, tree `da4f86d667b907b0a8349a9ba4030bdbbb12c2ff` |
| report | `a419d1cf735e10daea9b77d083ae08535fb3d9c028eaaeeb9e8486a3c7a8f0a4` |
| tables | 78 files under `research/physics/xypher-thaim-reward-proof/tables/`; each SHA-256 is printed on its agent's report line |

## 6. Independent check

A separate program, `research/physics/xypher-thaim-reward-check/check.py`, was
written from the frozen boundary and the 78 tables alone. Its author did not
read the apparatus or the pilots.

**Method.**
- It rebuilds the worlds, keys and rate tables itself.
- It solves every steady state with its own exact solver: multi-modular
  elimination over 62-bit primes, the Chinese remainder theorem, and rational
  reconstruction, accepted only after an exact integer check.
- It decides logarithmic forms by full prime factorization and certified
  enclosures.
- Runtime: about 37 seconds.

**Result: OVERALL AGREE.** 907 comparisons agree and none disagree:
- every exact value and enclosure in the report;
- every table fingerprint;
- every reproducible gate (G09 and G10 need training and were not
  reproduced);
- all six controls, each failing first at its stated gate;
- all five hypothesis verdicts. H4 is refuted in W1 by
  `E(NET-full) - E(LOCAL) = -0.031514`.

When run against a copy of the report with three values altered, it flagged
exactly those three.

**Readings.** It recorded six places where the boundary needed a reading. None
changes a result:
- G08 is stated both as a sign claim (section 2) and as an identity (section 7);
  both were checked;
- gate counting is per table or per world;
- the raw-key tuple shape;
- the weight behind the 0.810 and 0.907 shares;
- W2's paying-ratio set;
- the all-undecided label.

```text
check.py    62088585726b0062d8e00dab4cd658ada732aa7138c5950d28a8d798c834a227
output.txt  9e4dc7b34bfaf06a0e1339462a289e1bfd598f2b53fc9c3f117826617b172294
```

## 7. What it means, and what comes next

THAIM works as a reward when it is booked as the net flow of the agent's own
moves. In that form it needs no tuned constant. It teaches the world's exchange
rate between futures and timber, and it makes agents act only where agency pays.
The agents it trains sit 1.6 to 1.9 times closer to the thermodynamic floor than
hand-written aim.

The receipt form THAIM uses as an accounting record must not be paid as a
reward. It buys churn. The edge measure and the THAIM price are different
questions, and W1 shows they can disagree.

The next steps:
- Train a network that does not collapse, through a parameterization or
  curriculum that avoids the dead policy.
- Add a memory, so that information is priced by the same ledger, in the
  information-law class.
- Train two or more agents on their own net THAIM, and test the generosity and
  rivalry map of the frontier pilot.
