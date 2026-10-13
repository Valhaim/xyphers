# CAL-THAIM-1 independent check

This is an independent recomputation of the CAL-THAIM-1 run. It uses only:
- the frozen boundary `../derivable/xypher-thaim-reward-boundary.md`
  (SHA-256 `bfac95e0…8642744c`, checked at start);
- the recorded report `../xypher-thaim-reward-proof/report.txt`
  (SHA-256 `a419d1cf…f0a4`, checked at start);
- the 78 frozen rate tables `../xypher-thaim-reward-proof/tables/<world>/<agent>.txt`.

None of the apparatus code (`thaimworld.py`, `exact.py`, `run.py`, `tests.py`, `pilots/`) was read or imported.

## Run

```sh
cd research/physics/xypher-thaim-reward-check
/tmp/thaimvenv/bin/python check.py > output.txt
```

Environment: Python 3.9.6 with python-flint 0.6.0; numpy and scipy are not used. The runtime goes to
stderr, so `output.txt` is deterministic. Two consecutive runs produced the same file:
- `output.txt` SHA-256 `9e4dc7b34bfaf06a0e1339462a289e1bfd598f2b53fc9c3f117826617b172294`;
- `check.py` SHA-256 `62088585726b0062d8e00dab4cd658ada732aa7138c5950d28a8d798c834a227`.

**Runtime:** 36–40 s wall time per run on Darwin arm64. Measured runs took 36.4 s, 38.6 s and 39.8 s.

## Method

- **Worlds (§1, §6).** The six worlds W1–W6 are rebuilt from the boundary text:
  - 256 states `G·4 + pos`;
  - `N_tau` from `(I + A_G)^tau 1`;
  - 768 WEATHER/fuel pairs and 192 WALK pairs;
  - blind, local and full keys, ranked among the sorted distinct raw keys (§4).
- **Tables (§3, §4).** Each line `index<TAB>p/q` is parsed into an exact rational. The canonical text is
  regenerated and must match the file exactly. Each pair's rate is the entry of its key in the class
  declared in §3.
- **Steady states (own solver).** The solver is multi-modular elimination:
  1. Take the integer-scaled generator. Use the transposed system, with the last equation replaced by
     the normalisation.
  2. Solve it with `nmod_mat` over 62-bit primes.
  3. Combine the residues by the Chinese remainder theorem, then apply rational reconstruction.
  4. Accept a candidate only if the integer check `p M = 0`, `sum p = 1` holds exactly. Otherwise
     double the number of primes and repeat.

  G03 then rechecks `p > 0`, `sum p = 1` and `pQ = 0` in rationals, using the channel currents.
  Each solve used 16–256 primes, and the common denominators reach 7810 bits.
- **Quantities (§1.4, §5).** All of these are exact rationals:
  - `J`, residual `J`, `<|G|>`, `H`, `rho*`;
  - `H_warm = Phi_tau(rho*) - Phi_tau(r_0)`, with `Phi_tau` the exact polynomial;
  - `E`, aim, rule agreement;
  - the fuelled non-paying weight;
  - the agreement of RECEIPT-full with `theta_R`.

  `Sdot_A`, `Sdot_U`, the receipt flow and `ln b_c · R_net`, `ln b_c · R_receipt`, `ln b_c · R_world`
  (with `b_alpha = b_c`) are linear forms in logarithms. They are expanded over the **primes** using
  `flint fmpz.factor`, so an exact zero is a zero coefficient vector. A sign is the sign of a
  certified `arb` enclosure, starting at 256 bits and doubling until the enclosure excludes zero.
  `r > theta_R` is decided as `r^den > b_c^num` on integers. No float decides anything.
- **Gates.** G01–G08, G11, G12 and G13 are recomputed with the report's granularity: per table
  (78) for G01–G05 and G12, per world (6) for the others.
  - G06 recomputes the following, against values transcribed from §6:
    - paying counts and sets;
    - maxima, the 36 distinct ratios, no ties with `b_c`, all ratios at least 1;
    - the complete multiplicity tables for `tau = 3, 4`;
    - local ambiguity: 12 of 144 keys, equal to the keys holding a paying pair, with 8 members
      each, 2 of them non-paying at `r = 8` or `r = 16`;
    - the non-paying `pi_c` shares 0.810 and 0.907;
    - "paying pairs are builds at an isolated traveller";
    - the receipt windows;
    - the printed `theta_R`;
    - the §2 post-hoc observation that the window ratio is at least 1.8 `theta_R`.
  - G08 solves the N = 2 reference world. It also checks the §2 isolated-channel current and the sign
    rule `R_net > 0` if and only if `r > b_c`.
  - G12 also checks the ORACLE, LOCAL and GLOBAL tables against their §3 definitions.
  - G09 and G10 need training and float rates, so they are **not reproducible** here and are not
    counted.
- **Controls (§8).** These run on the W1 GLOBAL witness, with G08 on the reference world. Each
  control stops at its first failing check. The modifications are:
  - C01: one return rate is `b_h u + 1`;
  - C02: wear is multiplied by `(1 + u)`;
  - C03: the receipt flow replaces `Sdot_A` in G04;
  - C04: `b_alpha = b_h`;
  - C05: `b_h = b_c` in the chain and in G08;
  - C06: the frozen W1 paying count is 97.
- **Hypotheses (§10).** All five use the exact rules, including the undecided rules of H3 and H5.
  Thresholds are applied to scaled linear forms, for example `F_net - 0.98·F_oracle >= 0`.
- **Comparison with the report.** Each agent is compared on these fields:
  - key class;
  - `J`, `residual_J` and `H`, rounded to 6 decimals, and `E`, aim and agreement, rounded to 4. The
    rounding is exact and half-even, and near-ties are flagged; there were none;
  - `R_net` and `R_receipt`: the certified value must lie strictly inside the report's `[m +/- r]`,
    or be an exact zero where the report prints `0`;
  - `sigma = J ln(b_c/b_h)`, which must lie inside the quotient interval of the recorded
    `floor` and `eta` enclosures (§1.4 gives `eta = sigma_floor/sigma`);
  - the table's SHA-256.

  The remaining comparisons are:
  - the `theta_R` value and the RECEIPT agreement with `theta_R`;
  - the H3 weights;
  - every gate count, the first failure and the OVERALL line;
  - every control and every hypothesis verdict.

  Running the check against a copy of the report with three values altered produced exactly three
  value disagreements, plus the hash mismatch, so the comparisons can fail.

Not checked: G09, G10 and `tail` need training. `sigma_floor` and `eta` are defined in the
information-law note, not in the boundary, so only their ratio is checked.

## Result

`OVERALL AGREE`: 907 agreements, 0 disagreements.

| category | agree | disagree |
|---|---|---|
| input hashes | 2 | 0 |
| world parameters | 6 | 0 |
| key class | 78 | 0 |
| J | 78 | 0 |
| residual_J | 78 | 0 |
| H | 78 | 0 |
| E | 78 | 0 |
| aim | 78 | 0 |
| agreement | 78 | 0 |
| R_net (enclosure) | 78 | 0 |
| R_receipt (enclosure) | 78 | 0 |
| sigma = floor/eta | 78 | 0 |
| table sha256 | 78 | 0 |
| theta_R and receipt agreement | 12 | 0 |
| H3 weights | 4 | 0 |
| gates (11 verdicts with counts, first failure, slot line, OVERALL) | 14 | 0 |
| controls | 6 | 0 |
| hypotheses | 5 | 0 |

The recomputed verdicts are:
- **Gates:** G01–G08, G11, G12 and G13 PASS, with the report's counts. The first failure is none.
- **Controls:** C01–C06 each fail first at their stated gate.
- **Hypotheses:** H1, H2, H3 and H5 are SUPPORTED. H4 is REFUTED in W1, where
  `E(NET-full) - E(LOCAL) = -0.031514`.

## Boundary readings that needed a choice

None of these changes a result.
- **G08.** §2 says G08 checks "`R_net > 0` if and only if `r > b_c`", but the §7 table states an
  exact identity. Both are checked.
- **Gate granularity.** §7 does not say whether G01 and the other gates are counted per table or per
  world. The report's 78/6 granularity is mirrored.
- **Key ordering.** Two raw-key choices are open. The local key could be `(pos, (bits), e)` or a flat
  tuple, and the full key's `a` could be an index or `(G, pos)`. Both choices give the same ranks.
- **§6 weights.** The weight behind "carry 0.810 / 0.907" is not defined there. `pi_c(a) + pi_c(b)`,
  the §5 pair weight, reproduces both values.
- **W2.** The paying-ratio set is not stated, so only the count (96) is checked.
- **Undecided H3 and H5.** The verdict when every world is undecided is not stated. That case does not
  occur: every warm world is decided.
