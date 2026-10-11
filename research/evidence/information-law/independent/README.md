# Independent check of the information law

I wrote this check from `research/physics/derivable/xypher-information-law.md`
alone. I did not open the author's `infolaw.py`, `verify.py` or
`verification.txt`.

- Run 1 checked the note at commit 1206ea7f. Its output is kept unchanged in
  `output-run1.txt`.
- Run 2 checks the revised note at commit 66040060 (SHA-256 of the note
  `da7f820c…`). Its output is in `output.txt`.

## How to run

```sh
cd research/physics/xypher-information-law/independent
python3 check.py > output.txt
```

You need Python 3 with python-flint 0.6.0. The program uses its own random
generator, `random.Random(271828)`. Two runs gave identical output. The last line
of the output is `OVERALL PASS` or `OVERALL FAIL`.

## Method

- **Worlds.** The program generates its own worlds, with `|X|` from 2 to 5 and
  `|M|` from 2 to 4.
  - A U channel is a family over all `m`, with rates `w(x->x') = c pi_c(x')`
    and `w(x'->x) = c pi_c(x)`. The `pi_c` weights are integers from 1 to 5,
    `c` is an integer from 1 to 3, and parallel U channels are allowed.
  - A and R channels have integer rates from 1 to 5.
  - Every chain is checked to be irreducible.
- **Stationary law.** I wrote Gauss-Jordan elimination over
  `fractions.Fraction`. A full pivot set proves that the solution is unique.
  Stationarity is then re-checked exactly, state by state.
- **Quantities.** Every quantity in section 2 is built directly from its
  definition, with each transition of each family counted. Each quantity has
  the form `sum r ln(s)` with `r` and `s` rational.
- **Identities: full prime factorisation.** Every integer inside a logarithm is
  factored with `flint.fmpz.factor`. The program checks that each factor is
  prime (`fmpz.is_prime`) and that the factors multiply back to the integer.
  A quantity is stored as its exact rational coefficient vector over `ln p`,
  with `p` prime. An identity is accepted only if every coefficient of
  `lhs - rhs` is exactly 0. Logarithms of distinct primes are linearly
  independent over Q, so this decides the identity in both directions.
- **Inequalities.** Each inequality is decided by exact termwise signs, or by a
  certified sign of the whole linear form:
  - If the prime vector is 0, the value is exactly 0.
  - Otherwise the program computes an `arb` enclosure and raises the precision
    until the interval excludes 0.
- **No floating-point number decides any check.** Enclosures of both sides of
  (1), (2), (3) and (3') are printed for four worlds.

## Run 1 (note at 1206ea7f)

Run 1 checked 480 worlds in the class:

| set | worlds |
|---|---|
| general | 260 |
| V4 | 70 |
| V5 | 70 |
| V4 and V5 | 30 |
| V7 two-store | 50 |

It also checked 85 worlds built by theorem T: the demon world at five values of
`kappa`, and 40 random worlds at two values of `kappa`. In total there were 60
claims.

Result: `OVERALL FAIL`, with 13,159 of 13,161 checks agreeing.

- The 2 disagreements were one sign error in section 5. The demon-world limit
  was printed as `(p0-p1) ln[p1 a/(p0(1-a))]`, which is `+(1/2) ln 3 > 0` and not
  negative. The true limit is `-(1/2) ln 3`.
- Run 1 also reported six ambiguities:
  - `Phi` per family or per pair;
  - families present at every `x` or `m` in V4 and V5;
  - `b_c > b_h` in V7;
  - connectivity of `X` in theorem T;
  - that theorem T's bound covers A and R only;
  - what a "channel" is.

The run took about 19 s.

## What changed in the note

- Section 5: the demon limit now reads `(p0-p1) ln[p0(1-a)/(p1 a)]`. It is
  negative when `a/(1-a) > p0/p1`, which is equivalent to `a > p0`.
- The six ambiguities are now explicit:
  - a U channel is a family over `m`;
  - `Phi_A` and `Phi_R` are counted per channel family;
  - V4 and V5 require families at every `x` or `m`;
  - V7 states its packet relations and `b_c > b_h`;
  - theorem T assumes `X` is connected by U and A;
  - theorem T's bound covers A and R only.
- New statements:
  - strictness in V1;
  - "never attained" in section 0;
  - the remark in section 1.2 that (1)–(3') need only U rates that do not
    depend on `m`, without detailed balance;
  - the V5 world marginal as an autonomous chain;
  - V6 with a register temperature;
  - the U dissipation in theorem T;
  - the blind-agent criterion in section 6.

## What changed in check.py

All run-1 checks still run on the same worlds. New draws come after all run-1
draws, so the run-1 worlds are unchanged. Only the demon-text claims were
replaced, because the text they tested was withdrawn. The run-1 formula is still
printed, and a diagnostic confirms that it is positive.

The new checks are:

- **The corrected demon limit.**
  - The note's formula equals `floor - Idot_act` exactly, and it is negative
    for `p = (3/4, 1/4)`, `a = 9/10`.
  - The demon world was also checked on a grid of 35 worlds,
    `p0` in {3/5, 2/3, 3/4, 4/5, 9/10} and
    `a` in {11/20, 3/5, 2/3, 3/4, 4/5, 9/10, 19/20}, at `kappa = 1000`. On the
    grid, the program checks the closed forms, the limit formula, the exact
    equivalence `limit < 0 <=> a/(1-a) > p0/p1 <=> a > p0`, and V3. The grid
    has 15 negative, 5 zero (`a = p0`) and 15 positive cases.
- **V1 strictness.**
  - If some A transition carries current, then `Sigma_A > 0` (certified).
  - If `Idot_act != 0`, then some A current is nonzero.
  - If `Idot_act != 0`, then `sigma_world > floor - Idot_act`, strictly and
    certified.
  - If there is no A current, then `Sigma_A = Idot_act = 0` exactly.
  - In every T world with an A current, the bound is never attained
    (certified).
- **Per-family Phi.** `Phi_A` and `Phi_R` are summed per channel family, each
  counted once. Merging families per pair still fails, as a diagnostic.
- **V5 autonomous chain.** `p` is exactly stationary for the chain on `X`
  formed by the U and A families.
- **V7 hypotheses.** `b_c > b_h`, and `ln(k/k') = -Delta|G| ln b` holds exactly
  for every U and A jump.
- **V6.** With `alpha = 1/ln b_c`, `W_floor - W <= alpha Idot_act` holds.
- **Theorem T.**
  - The connectivity hypothesis is checked in every constructed world.
  - The U dissipation `sum_U S_l` does not depend on `kappa`.
  - A diagnostic shows that without connectivity, `P*` still balances exactly
    but the stationary law is not unique.
- **Section 1.2 remark.** 60 worlds have U rates that do not depend on `m` but
  have no detailed-balance law. In them the program checks (1)–(4), Lemmas 1
  and 2, and V1–V3, under the heading `[sec.1.2 non-DB U]`.
- **Section 6.** For each general world, the program builds a blind agent with
  `M` a single state that holds the same `p`, using the recipe of theorem T at
  `kappa = 1` and `kappa = 1000`. It then checks three things:
  - the blind agent's stationary law is `p`;
  - the blind total is `floor + Sigma_A(blind)`, with the same floor;
  - the informed agent is cheaper in total exactly when
    `Idot_noise + Sigma_A + Sigma_R < Sigma_A(blind)`.

## Run 2 result (note at 66040060)

The result is **`OVERALL PASS`: 18,496 checks agree and 0 disagree, across 102
claims.** It covers:

- 480 worlds in the class;
- 60 worlds with U rates that are not detailed balanced;
- 120 worlds built by theorem T (5 demon, 35 demon grid, 80 random);
- 520 blind-agent comparisons.

A run takes about 34 s of wall time on an Apple arm64 laptop (`time -p`: real
33.8 s). Every claim and its count is in `output.txt`.

The demon world, `p = (3/4, 1/4)` and `a = 9/10`, gives:

- `floor = (1/2) ln 3`
- `Idot_act = ln 3`
- `Idot_noise = (8/5) ln 3`
- `Idot_learn = (13/5) ln 3`
- limit `-(1/2) ln 3`

Certified values of the ratio `(floor - sigma_world)/Idot_act`:

| kappa | ratio |
|---|---|
| 1 | 0.815464876785729 |
| 10 | 0.977794639349710 |
| 100 | 0.997730072036111 |
| 1000 | 0.999772497064339 |
| 10^6 | 0.999999772440250 |

Remaining remarks. None of them is a contradiction.

- Section 0 says the bound "is never attained". This holds whenever
  `Idot_act != 0`, as V1 states. In the 16 in-class worlds with no A current,
  `sigma_world = floor` exactly, so there the bound of zero is attained.
- The yield bound `Y <= H ln(b_c/b_h)/[floor - Idot_act]` in V7 uses `Y` and
  `H` from formula III, which this note does not define. It was not checked.
- V6's register-temperature statement is (2) multiplied by `T_R > 0`. It is
  covered by V2 and was not checked separately.
- Without detailed balance, the floor counted per U channel differs from the
  floor merged per pair in 44 of the 60 worlds. Only the per-channel floor
  satisfies (1)–(3'). The note defines it per channel, so it is consistent.
