---
title: The Information Law
aliases:
  - Formula V
  - What Knowing Is Worth in Fuel
tags:
  - domain/physics
  - type/theorem
  - topic/thermodynamics
  - topic/information
  - topic/xypher
  - topic/nonequilibrium
domain: Physics
type: theorem
status: proved in the declared class; exact verification passed; blind referee ACCEPT (round 2); independent check in section 11
td: td-4e4cf3
created: 2026-10-11
updated: 2026-10-11
related:
  - "[[Open Future Price Boundary]]"
  - "[[Agency Cavity Theory]]"
---

# The information law

## 0. Claim

An agent holds a world away from equilibrium by acting on it. Without knowledge
of the world, its fuel cannot fall below the Horowitz--Zhou--England floor
(formula III), the greatest lower bound on the entropy production of any
control that holds the same state. This note proves what knowledge buys, for
every finite world in a declared class:

```text
sigma_world = sigma_floor(p) - Idot_act + Sigma_A          (exact)
sigma_reg   = Idot_learn + Sigma_R                          (exact)
Idot_learn  = Idot_act + Idot_noise,   Idot_noise >= 0,  Sigma_A >= 0,  Sigma_R >= 0.
```

The world's fuel can fall below the floor by at most the information the
agent's actions consume. The bound is approached as the actions become
reversible, and it is never attained while the actions carry current (V1). The agent's memory pays for that
information at the same rate, plus whatever the world's noise erases. In
entropy, one nat of information stands in for at most one nat of fuel, at par
in the reversible limit. At the world's temperature `alpha` a nat is worth
`alpha` of free energy, so a bit of information saves at most `alpha ln 2` of
the fuel's free energy: Landauer's rate. The coefficient one is sharp
(section 5).

Two consequences close the account.
- In entropy, the total never falls below the floor:
  `sigma_world + sigma_reg >= sigma_floor(p) + Idot_noise`.
- Beating the floor needs both halves of agency: a memory whose moves read the
  world, and actions that read the memory. Take away either and the fuel stays
  at or above the floor.

## 1. The class of worlds

### 1.1 States

A finite set of world states `X` and a finite set of register states `M`. The
system state is `z = (x, m)`. The register is the agent's memory: what it has
sensed, planned, or stored.

### 1.2 Channels

Every channel is a reciprocal pair of transitions with positive rates in both
directions. Three classes:

- **U, the undriven world.** A U channel is a family over all register states:
  `(x, m) <-> (x', m)` for every `m`, with rates `w(x -> x')`, `w(x' -> x)` that
  do not depend on `m`. One law `pi_c` on `X`, positive (`c` for the cold store
  of a Xypher world), satisfies detailed balance with every U channel:
  `pi_c(x) w(x -> x') = pi_c(x') w(x' -> x)`. Several U channels may join the
  same pair of world states.
- **A, the agent's actions.** `(x, m) <-> (x', m)` with any positive rates
  `a_m(x -> x')`, `a_m(x' -> x)`. An action channel may exist in some register
  states and not others, and may read both the world and the register.
- **R, the register.** `(x, m) <-> (x, m')` with any positive rates
  `r_x(m -> m')`, `r_x(m' -> m)`. A register channel may exist at some world
  states and not others, and may read both.

No channel changes `x` and `m` together; the system is bipartite. Each channel
exchanges entropy `ln(k/k')` per forward jump with a store, by local detailed
balance (`k`, `k'` its forward and backward rates). For U channels this store is
the world's own: in a Xypher world it is the cold store that fixes the world's
temperature `alpha = lambda / ln b_c` (formula I).

Detailed balance is used only for two things: the relative-entropy form of the
floor, and its identification with the Horowitz--Zhou--England floor.
Identities (1)--(3') hold for any U rates that do not depend on `m`.

### 1.3 Steady state

The chain on `X x M` is irreducible. `P` is its unique stationary law, with
marginals `p(x)` and `q(m)`. All results are statements about `P`.

### 1.4 What the class excludes

The undriven world must not read the register. A world whose own noise depends
on the agent's memory lies outside the class. The CAL-CEF-1 to 5 archipelagos
are such worlds: weather never wears a bridge the held plan uses, and plans
exist only over built bridges. Section 8 places them.

## 2. Quantities

Below, `l` indexes single transitions: a channel from `z` to `z'` with forward
rate `k` and backward rate `k'`.

```text
J_l     = P(z) k - P(z') k'                        net current
F_l     = J_l ln(k / k')                           entropy flow to the channel's store
S_l     = J_l ln[P(z) k / (P(z') k')]   >= 0       entropy production
Idot_l  = J_l [i(z') - i(z)],   i(x, m) = ln[P(x, m) / (p(x) q(m))]
                                                   change of mutual information I(X; M)
```

Sums over channel classes (every transition of every family counted):

```text
sigma_world = sum over U and A of F_l        entropy the world's channels send to stores
sigma_reg   = sum over R of F_l              entropy the register sends to its store
Sigma_A     = sum over A of S_l              dissipation in the actions
Sigma_R     = sum over R of S_l              dissipation in the register
Idot_act    = - sum over A of Idot_l         information the actions consume
Idot_learn  =   sum over R of Idot_l         information the register gains (learning rate)
Idot_noise  = - sum over U of Idot_l         information the world's noise erases
```

The floor of the held world law `p`, with one term per U channel between `x`
and `x'` and `j(x, x') = p(x) w(x -> x') - p(x') w(x' -> x)`:

```text
sigma_floor(p) = sum over U channels of j(x, x') ln[p(x) w(x -> x') / (p(x') w(x' -> x))]
               = - d/dt D(p || pi_c)  under the U channels alone.
```

The second line uses detailed balance: the logarithm equals
`ln[p(x)/pi_c(x)] - ln[p(x')/pi_c(x')]`. Because one law serves every U
channel, parallel U channels between the same pair give the same floor whether
they are counted separately or merged. This is the floor of Horowitz, Zhou, and
England (2017), formula III: the greatest lower bound on the entropy production
of any control, acting on the world alone, that holds `p`.

## 3. Theorem

In every world of the class, at the stationary law:

```text
(1)  sigma_world = sigma_floor(p) - Idot_act + Sigma_A
(2)  sigma_reg   = Idot_learn + Sigma_R
(3)  Idot_learn  = Idot_act + Idot_noise
(3') sum over U of S_l = sigma_floor(p) + Idot_noise
(4)  Sigma_A >= 0,  Sigma_R >= 0,  sigma_floor(p) >= 0,  Idot_noise >= 0.
```

`Idot_noise = 0` exactly when `P(m | x) = P(m | x')` for every register state
and every pair joined by a U channel.

### Corollaries

- **V1 (formula V).** `sigma_world >= sigma_floor(p) - Idot_act`. Since
  `Idot_act <= Idot_learn`, also `sigma_world >= sigma_floor(p) - Idot_learn`.
  If some action channel carries current then `Sigma_A > 0`, so in every world
  of the class the inequality is strict whenever `Idot_act != 0`. Section 5
  shows it is approached.
- **V2 (the price of knowing).** `sigma_reg >= Idot_learn = Idot_act + Idot_noise`.
- **V3 (the total, in entropy).**
  `sigma_world + sigma_reg = sigma_floor(p) + Idot_noise + Sigma_A + Sigma_R >= sigma_floor(p) + Idot_noise`.
  The lower bound is Horowitz and England's (2017, eq. 19): the entropy
  production of the world's own channels on the joint state, which by (3')
  equals `sigma_floor(p) + Idot_noise`. Their weaker form, `total >= sigma_floor(p)`
  (eqs. 20--21), discards `Idot_noise`.
- **V4 (a memory that does not read the world).** Suppose every R channel is a
  family over all world states with rates that do not depend on `x`. Then
  `Idot_learn <= 0` and
  `sigma_world = sigma_floor(p) + Idot_noise - Idot_learn + Sigma_A >= sigma_floor(p)`.
- **V5 (actions that do not read the memory).** Suppose every A channel is a
  family over all register states with rates that do not depend on `m`. Then
  `sigma_world = sigma_floor(p) + Phi_A >= sigma_floor(p)`, with
  `Phi_A = sum over A channel families, each counted once, of (a p(x) - a' p(x')) ln[a p(x) / (a' p(x'))] >= 0`.
  The world marginal is then an autonomous Markov chain, and V5 is the
  Horowitz--Zhou--England result applied to it.
- **V6 (Landauer's rate on the fuel side).** At the world's temperature `alpha`,
  the fuel's dissipated free-energy power is `W = alpha sigma_world`. Against
  the floor,
  `W_floor - W = alpha [sigma_floor(p) - sigma_world] <= alpha Idot_act = alpha ln 2 · Idot_act,bits`.
  The memory's bill in free energy is `T_R sigma_reg`, where `T_R` is the
  temperature of the register's store. It is at least `T_R ln 2` per bit
  learned. The exchange is at par in free energy only when `T_R = alpha`.
- **V7 (the fuel floor in a two-store world).** Suppose the following:
  - every U jump exchanges packets with a cold store, `ln(k/k') = -Delta|G| ln b_c`;
  - every A jump exchanges packets with a hot store, `ln(k/k') = -Delta|G| ln b_h`;
  - `b_c > b_h`, so the hot store is hotter.

  Then `sigma_world = J ln(b_c / b_h)`, with `J` the net fuel flow from the hot
  store. Hence `J >= [sigma_floor(p) - Idot_act] / ln(b_c / b_h)`, and while
  `sigma_floor(p) > Idot_act` the yield bound of formula III becomes
  `Y <= H ln(b_c/b_h) / [sigma_floor(p) - Idot_act]`.

## 4. Proof

**Lemma 1 (stationary marginals).**
- For any function `f` on `X`,
  `sum over U and A of J_l [f(x') - f(x)] = sum_x f(x) dp(x)/dt = 0`, because R
  channels do not change `x`.
- Likewise, for any `g` on `M`, `sum over R of J_l [g(m') - g(m)] = 0`.
- And `sum over all transitions of J_l [i(z') - i(z)] = sum_z i(z) dP(z)/dt = 0`.

**Lemma 2 (split of the entropy production).** For every transition,
`S_l = F_l - J_l [ln P(z') - ln P(z)]`, and
`ln P(x, m) = ln p(x) + ln q(m) + i(x, m)`. For a U or A transition `m` is
fixed, so `ln P(z') - ln P(z) = [ln p(x') - ln p(x)] + [i(z') - i(z)]`. For an R
transition `x` is fixed, so `ln P(z') - ln P(z) = [ln q(m') - ln q(m)] + [i(z') - i(z)]`.
Summing, and applying Lemma 1 to `f = ln p` and `g = ln q`:

```text
(a)  sum over U and A of S_l = sigma_world - sum over U and A of Idot_l
(b)  Sigma_R = sigma_reg - Idot_learn.
```

(b) is (2).

**Lemma 3 (the floor).** Take one U channel between `x` and `x'` with rates
`w, w'`, present in every register state `m`. Write `u_m = P(x, m) w` and
`v_m = P(x', m) w'`, so `J_m = u_m - v_m`. Because `w` and `w'` do not depend on
`m`, `sum_m u_m = p(x) w` and `sum_m v_m = p(x') w'`, and

```text
ln(u_m / v_m) = ln[p(x) w / (p(x') w')] + ln[P(m | x) / P(m | x')].
```

Sum `J_m ln(u_m / v_m)` over `m`.
- The first part gives the channel's floor term `j(x, x') ln[p(x) w / (p(x') w')]`.
- The second part gives `- sum_m J_m [i(x', m) - i(x, m)]`, since
  `i(x', m) - i(x, m) = ln P(m|x') - ln P(m|x)`.

Summed over U channels this is (3'): `sum over U of S_l = sigma_floor(p) + Idot_noise`.

**Proof of (1).** By Lemma 2(a), Lemma 3, and the definitions,

```text
sigma_world = sum over U of S_l + Sigma_A + sum over U of Idot_l + sum over A of Idot_l
            = [sigma_floor(p) + Idot_noise] + Sigma_A - Idot_noise - Idot_act.
```

**Proof of (3).** Lemma 1, last line: the three information sums add to zero.

**Proof of (4).**
- In each `S_l` the current `J_l = u - v` and the logarithm `ln(u/v)` have the
  same sign. So every term is nonnegative and vanishes exactly when `u = v`. The
  floor terms have the same form.
- For `Idot_noise`, by Lemma 3 it is a sum over U channels of

  ```text
  sum_m (u_m - v_m) ln(u_m / v_m) - (U - V) ln(U / V),      U = sum_m u_m,  V = sum_m v_m.
  ```

  The log-sum inequality gives `sum_m u_m ln(u_m/v_m) >= U ln(U/V)` and
  `sum_m v_m ln(v_m/u_m) >= V ln(V/U)`. Adding them shows each term is
  nonnegative. Equality holds exactly when `u_m / v_m` does not depend on `m`,
  that is when `P(m | x) = P(m | x')`.

**Proof of the corollaries.**
- V1 and V2 follow from (1)--(4). Strictness in V1: if
  `Idot_act = - sum over A of J_l [i(z') - i(z)]` is nonzero, some A transition
  has `J_l != 0`, and its `S_l > 0`.
- V3 is (1) + (2), with (3).
- V4: repeat Lemma 3 on each R channel family, exchanging the roles of `x` and
  `m`. This gives `Sigma_R = Phi_R - Idot_learn`, with
  `Phi_R = sum over R channel families of (r q(m) - r' q(m')) ln[r q(m) / (r' q(m'))]`.
  The log-sum argument of (4), with `x` and `m` exchanged, gives
  `Sigma_R >= Phi_R`. Hence `Idot_learn <= 0`, and (1) with (3) gives the stated
  form.
- V5: repeat Lemma 3 on each A channel family to get `Sigma_A = Phi_A + Idot_act`;
  then (1) gives `sigma_world = sigma_floor(p) + Phi_A`. Each term of `Phi_A`
  has the sign form of (4).
- V6 multiplies V1 by `alpha`, and (2) by `T_R`.
- V7: by the packet relations,
  `sigma_world = - ln b_c · (net builds by U) - ln b_h · (net builds by A)`.
  Lemma 1 with `f = |G|` makes the two net build currents equal and opposite,
  with `J` from the hot store, so `sigma_world = J ln b_c - J ln b_h`.

QED.

## 5. The exchange rate is exactly one

**Theorem T (tightness).** Fix the following:
- the U channels;
- a positive law `P*` on `X x M`;
- a set of A channels with currents `j_A` that conserve the world marginal of
  `P*`: with the U currents of `P*`, the net flow into every world state `x` is
  zero;
- R channels that connect `M` at every `x`.

Assume also that the graph on `X` with an edge for every U channel, and for
every A channel in any register state, is connected.

Then for every `kappa > 0` there are rates on the A and R channels under which
`P*` is the unique stationary law, the A currents are `j_A`, and

```text
Sigma_A + Sigma_R <= (sum of j^2 over A and R channels) / kappa.
```

*Construction.*
- The U currents of `P*` are fixed by `P*`.
- At each `x`, the R channels on a spanning tree of `M` carry the unique
  currents that cancel the remaining net flow at every `(x, m)`. This is
  possible because the flows at fixed `x` sum to zero. Other R channels carry
  zero.
- A channel from `z` to `z'` that must carry current `j` gets

  ```text
  k(z -> z') = [kappa + max(j, 0)] / P*(z),      k(z' -> z) = [kappa + max(-j, 0)] / P*(z').
  ```

  Then `P*(z) k - P*(z') k' = j`, and every state balances.
- The connectivity assumption and the R channels make the chain irreducible, so
  `P*` is its unique stationary law.
- Each A and R channel dissipates `|j| ln[(kappa + |j|)/kappa] <= j^2 / kappa`.
  The U channels' dissipation is fixed by `P*`: it is `sigma_floor + Idot_noise`.

This is the fast, reversible control of Horowitz and England (2017, eq. 14),
applied with a memory.

The information sums depend only on `P*` and the currents, not on `kappa`. As
`kappa` grows, `sigma_world -> sigma_floor(p*) - Idot_act` and
`sigma_reg -> Idot_learn`. Whenever `Idot_act > 0` is possible, no coefficient
below one can multiply `Idot_act` in V1. In entropy, the exchange rate is one
nat per nat.

**The demon world.** One world with an exact closed form:
- `X = {0, 1}`, with one U channel at rate 1 each way, so `pi_c` is uniform;
- `M = {0, 1}`;
- the held law is `P*(x, m) = p(x) P(m | x)`, with `P(m = x | x) = a > 1/2`
  and `p(0) > p(1)`;
- one A channel, present only in register state 1, moves `x` from 1 to 0 and
  carries the whole holding current `p(0) - p(1)`.

Then, exactly:

```text
sigma_floor = (p0 - p1) ln(p0/p1)
Idot_act    = (p0 - p1) ln[a / (1 - a)]
Idot_noise  = (2a - 1) ln[a / (1 - a)]
Idot_learn  = (p0 - p1 + 2a - 1) ln[a / (1 - a)].
```

The limit is `sigma_world -> (p0 - p1) ln[p0 (1 - a) / (p1 a)]`. It is negative
when `a/(1-a) > p0/p1`, which is equivalent to `a > p0`. The actions then return
fuel while holding the world out of equilibrium. The memory pays `Idot_learn`,
and the total stays above `sigma_floor + Idot_noise`. This is Maxwell's demon,
accounted against the floor.

## 6. Reading the law

In entropy, knowledge is a currency at par, with the following accounts:
- The information an agent's actions use replaces at most one nat of fuel per
  nat, at par in the reversible limit (V1, T).
- Gathering that information costs the memory at least as much (V2).
- The world's noise erodes what the memory holds, and that erosion is paid on
  top (V3).
- Knowledge therefore never makes holding a world cheaper than the floor in
  total entropy. It moves part of the bill from the fuel to the memory.
- In free energy the two bills are paid at their own stores' temperatures. In
  the reversible limit the memory pays `T_R ln 2` per bit learned, and the fuel
  saves `alpha ln 2` per bit used. The memory also pays for the bits the noise
  erases, so it costs less in free energy than the fuel saves only when
  `T_R / alpha < Idot_act / Idot_learn` (5/13 in the demon world of section 5).
  Landauer's principle allows this (V6).

That is the ideal limit, with reversible actuators. Real agents act with fixed
forces. A CAL-CEF agent spends its hot store's packets at a fixed ratio and pays
2.2 to 3 times the floor (CAL-CEF-3), so its `Sigma_A` is large. The total
balance (V3) then states exactly when knowing pays. Compare, at equal held law
and in entropy, an informed agent with a blind one, meaning `M` a single state
(blind total `sigma_floor(p) + Sigma_A(blind)`). The informed agent costs less
in total precisely when

```text
Idot_noise + Sigma_A(informed) + Sigma_R  <  Sigma_A(blind).
```

Knowledge pays when it lets fuel be spent only where it is needed, and when
gathering it costs less than the waste it removes. This is the question of the
next experiment.

## 7. Xypher reading

The class is a minimal thermodynamic structure, not a full adaptive Xypher;
the full-adaptive checklist items (Thompson memory, Opal and `Phi`, the five
primes) are not applicable.

- **Graph Substrate:** the world `X` (in the archipelago, the configuration of
  bridges).
- **Environment:** the U channels (weather), reciprocal with the world's own
  store.
- **Crystal:** the register `M`, which perceives the world through R channels.
- **Praxion:** the A channels, which act on the world and may read the register.
- **Thermo:** the stores. `alpha` is fixed by the cold store's declared equation
  of state (`b_c`), before any rate is inspected; A and R channels exchange
  entropy with their own stores by local detailed balance.
- **Ruby** empty; **Opal** absent. No constant is tuned: the theorem holds for
  every rate in the class, and `kappa` in section 5 indexes a family whose
  limit is the statement.

## 8. Where the CAL-CEF worlds sit

In CAL-CEF-1 to 5 the system state is `(G, plan, cursor)`. The weather reads
the plan: it never wears a bridge the held plan uses, and a plan exists only
over built bridges. With `X = G` and `M = (plan, cursor)`, the undriven world
reads the register, so those worlds lie outside the class of section 1.

Their floor was computed on the full chain, with the plan inside the world:
`X = (G, plan, cursor)` and `M` a single state. In that reading `Idot = 0` and
the theorem reduces to formula III, `sigma = sigma_floor + Sigma_A`, which is
what CAL-CEF-3 to 5 measured. Those experiments are the empty-memory case of
the law; they do not test its information term. An earlier sentence on the
formula sheet read them as tests at `Idot = 0`. This section replaces that
reading.

## 9. What is borrowed, and what is new

Borrowed:
- the subsystem second law with information flow (Horowitz and Esposito 2014),
  of which Lemma 2 is a form;
- the learning-rate bound on a sensor's dissipation (Barato, Hartich, and
  Seifert 2014), which V2 restates;
- the floor (Horowitz, Zhou, and England 2017), and its ancillary-control form
  (Horowitz and England 2017):
  - their eq. 19 is the bound in V3;
  - the step from their eq. 19 to eq. 20 is the inequality content of Lemma 3;
  - their fast reversible control (eq. 14) is the mechanism of theorem T;
- V5, which is the Horowitz--Zhou--England result applied to the autonomous
  marginal chain;
- the gap of Lemma 3 in relative-entropy form, which appears for sensors in
  Hartich, Barato, and Seifert (2016);
- the observation that a physical controller must pay for the entropy its
  control removes (Ehrich, Still, and Sivak 2023).

New here, as far as our search and our referee found:
- identity (1), which sets the information the actions consume directly against
  the floor, so that the world's fuel can go below the floor by at most that
  information;
- the reading of the gap between Horowitz and England's eqs. 19 and 20 as
  exactly the information the world's noise erases, `Idot_noise`, which closes
  the information budget (3);
- theorem T for the information term: the coefficient one in V1 is sharp;
- V4, and its pairing with V5: going below the floor needs a memory that reads
  the world and actions that read the memory.

## 10. Verification

`research/physics/xypher-information-law/` holds the library `infolaw.py`, the
program `verify.py`, and its output `verification.txt`. They need python-flint
(tested with 0.6.0), and the run takes about nine seconds.

- Steady states are exact rationals (flint `fmpq_mat`).
- Every quantity is a finite sum of rational multiples of logarithms of
  integers. Identities are decided exactly: the integers are reduced to a
  pairwise coprime base, whose logarithms are linearly independent over the
  rationals, and every coefficient must vanish.
- Inequalities are decided by exact sign arguments, or by certified arb
  enclosures at 256 bits.
- No floating-point value decides a check.

Results:
- **Random worlds.** Identities (1), (2), (3), (3'), the term signs, and
  certified `Idot_noise >= 0` hold in all 500 random worlds:
  - 300 general;
  - 100 with a memory that does not read the world, where `Idot_learn <= 0` is
    certified and none goes below the floor;
  - 100 with actions that do not read the memory, where V5 holds exactly and
    none goes below the floor.
- **Demon world** (`p = (3/4, 1/4)`, `a = 9/10`):
  - the four closed forms of section 5 hold exactly;
  - the ratio `[sigma_floor - sigma_world] / Idot_act` rises from 0.815 at
    `kappa = 1` to 0.99999977 at `kappa = 10^6`;
  - the dissipation bound of theorem T is certified at every `kappa`;
  - `sigma_world` converges to `-(1/2) ln 3`.
- **Three-island archipelago** (weather `b_c = 3`):
  - a blind matched agent reaches the floor;
  - a sensing agent holding the same world law converges to
    `sigma_floor - Idot_act = -1.6636`, with ratio 0.99999991 at `kappa = 10^6`.
- **Fixed-force sensing agent** (`b_c = 3`, `b_h = 2`): every identity holds,
  and its world entropy flow equals `J ln(b_c / b_h)` exactly (V7).

An independent program is in `independent/`. It was written from this note
alone, factors every integer into primes instead of using a coprime base, and
uses its own generator and solver (section 11). Its second run agrees on all
18,496 checks.

## 11. Review

**Independent check, run 1** (on the first version of this note):
- It agreed on 13,159 of 13,161 checks over 480 random worlds and 85 worlds
  built by theorem T.
- The two disagreements were one error in the text. The demon-world limit was
  printed with the argument of its logarithm inverted. It is corrected in
  section 5.
- It also named six places where two implementations could read the note
  differently. Each is now stated explicitly:
  - `Phi_A` and `Phi_R` are counted per channel family;
  - V4 and V5 need each family at every `x` or `m`;
  - V7 needs `b_c > b_h` and one packet per jump;
  - theorem T needs `X` connected by U and A;
  - theorem T's bound covers A and R only;
  - a U channel is a family over `m`.

**Independent check, run 2** (on the revised note, commit 66040060):
- OVERALL PASS: 18,496 checks agree and none disagree, across 102 claims.
- It covers the run-1 worlds and a demon grid of 35 worlds, where the limit is
  negative exactly when `a > p0`. It also covers V1 strictness (certified in
  437 worlds), the blind comparison of section 6 (520 cases), and theorem T in
  120 worlds.
- It also covers 60 worlds whose U rates do not depend on `m` but are not
  detailed balanced. There, (1)--(3') hold as section 1.2 states.
- The run-1 output is kept beside it, unchanged.

**Blind referee, round 1:** MINOR REVISION. The referee recomputed the
definitions, the lemmas, (1)--(4), V1--V7, and theorem T by hand and with exact
arithmetic, and found them correct. Three findings were blocking:
- the same demon-world sign;
- the connectivity hypothesis missing from theorem T;
- the claim that V3 sharpens Horowitz and England. It does not: their eq. 19 is
  the same bound. The novelty claims of section 9 were narrowed accordingly.

Nine non-blocking findings were all applied:
- "exactly" and "least" replaced: the floor as a greatest lower bound, never
  attained, with strictness in V1;
- `Phi_A` and `Phi_R` counted once per channel family (also raised by the
  independent check);
- the free-energy reading with a memory store at its own temperature;
- the blind comparison defined as `M` a single state;
- the explicit hypotheses of V7;
- V5 credited to Horowitz, Zhou, and England;
- the log-sum step in V4;
- the channel index renamed `l`, and the power symbol renamed `W`;
- this status line.

**Blind referee, round 2:** ACCEPT. All twelve round-1 findings are resolved,
and the revision introduced no mathematical error. The referee rechecked:
- the corrected demon limit and its equivalence with `a > p0`;
- strictness in V1;
- the free-energy reading;
- the narrowed novelty list.

Three non-blocking refinements were applied:
- the free-energy sentence of section 6 now states the threshold
  `T_R / alpha < Idot_act / Idot_learn`, because the memory also pays for erased
  bits;
- section 0 qualifies "never attained" by "while the actions carry current";
- this record lists the round-1 findings as reported.

## References

- J. M. Horowitz and M. Esposito, "Thermodynamics with continuous information
  flow," Phys. Rev. X 4, 031015 (2014), doi:10.1103/PhysRevX.4.031015.
- A. C. Barato, D. Hartich, and U. Seifert, "Efficiency of cellular information
  processing," New J. Phys. 16, 103024 (2014), doi:10.1088/1367-2630/16/10/103024.
- D. Hartich, A. C. Barato, and U. Seifert, "Sensory capacity: An information
  theoretical measure of the performance of a sensor," Phys. Rev. E 93, 022116
  (2016), doi:10.1103/PhysRevE.93.022116.
- J. M. Horowitz, K. Zhou, and J. L. England, "Minimum energetic cost to
  maintain a target nonequilibrium state," Phys. Rev. E 95, 042102 (2017),
  doi:10.1103/PhysRevE.95.042102.
- J. M. Horowitz and J. L. England, "Information-theoretic bound on the entropy
  production to maintain a classical nonequilibrium distribution using
  ancillary control," Entropy 19, 333 (2017), doi:10.3390/e19070333.
- J. Ehrich, S. Still, and D. A. Sivak, "Energetic cost of feedback control,"
  Phys. Rev. Research 5, 023080 (2023), doi:10.1103/PhysRevResearch.5.023080.
- T. Sagawa and M. Ueda, "Generalized Jarzynski equality under nonequilibrium
  feedback control," Phys. Rev. Lett. 104, 090602 (2010),
  doi:10.1103/PhysRevLett.104.090602.
- R. Landauer, "Irreversibility and heat generation in the computing process,"
  IBM J. Res. Dev. 5, 183 (1961), doi:10.1147/rd.53.0183.
