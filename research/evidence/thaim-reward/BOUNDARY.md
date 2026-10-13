---
title: THAIM Reward Boundary
aliases:
  - CAL-THAIM-1 Boundary
  - Can THAIM Train an Agent
tags:
  - domain/physics
  - type/boundary
  - topic/thermodynamics
  - topic/xypher
  - topic/learning
domain: Physics
type: boundary
status: frozen (blind review ACCEPT after four rounds)
td: td-8ef64f
created: 2026-10-13
updated: 2026-10-13
related:
  - "[[The Information Law]]"
  - "[[Open Future Price Boundary]]"
  - "[[Agency Cavity Theory]]"
---

# CAL-THAIM-1: Can THAIM train an agent?

## 0. The question

THAIM is the framework's one-sided expansion receipt:
`THAIM_+ = alpha · max(0, Delta S_tau)`. It is the accounting unit in which
open future is paid for at the world's temperature `alpha`. This experiment
asks whether THAIM can also be the reward an agent is trained on, and in which
form.

There are three candidate forms. Section 2 derives what each must do:
- the receipt itself;
- the net THAIM flow of the agent's own moves;
- the net THAIM flow of the whole world.

Small models are trained on each. Training uses exact gradients through the
steady state of a driven archipelago. The trained policies are frozen as exact
rational rate tables and evaluated in exact arithmetic.

**The central prediction** is a learned rule, written down before any declared
world is solved. Trained on its own net THAIM flow, an agent fuels a bridge
exactly when the bridge multiplies the traveller's futures by more than `b_c`,
the cold store's factor per packet. The threshold has the algebraic form of
formula I's keep-odds, evaluated at the traveller's current island. This world
has no plan protection, so here it is a pricing rule, not a stationary odds.

Five hypotheses can fail.

## 1. The world

### 1.1 States

- Islands: `V = {0, 1, 2, 3}` (`N = 4`). The candidate bridges are the `K = 6`
  unordered pairs, in lexicographic order. A configuration `G` is a 6-bit
  pattern.
- Traveller position: `pos` in `V`.
- System state: `x = (G, pos)`, 256 states, with index `G · N + pos`. Bit `k` of `G` is the
  `k`-th bridge in lexicographic order.
- Open future: `N_tau(G, pos) = e_pos^T (I + A_G)^tau 1`, the number of
  `tau`-step walks from the traveller's island. Each step stays or crosses a
  built bridge. `S_tau(x) = ln N_tau(x)`.
- Energy: each bridge holds one packet of timber, `lambda = 1`.

### 1.2 Undriven channels (the world's own noise)

- **WEATHER**, using the cold store. For every bridge `e` and every state: build
  `e` at rate 1 if it is absent; wear it at rate `b_c` if it is present. This is
  local detailed balance with the cold store, `alpha = lambda / ln b_c`.
- **WALK**: the traveller crosses a built bridge `(pos, j)` to `j` at rate `v`,
  and back at rate `v`.

The undriven world is reciprocal, with detailed balance against
`pi_c(G, pos) ∝ b_c^(-|G|)`, uniform in `pos`. It never reads the agent: there
is no plan protection. This places the world in the class of the information
law (`xypher-information-law.md`, section 1), with an empty register.

### 1.3 Fuel channels (the agent)

For every bridge `e`, take each state `a` with `e` absent, and let `b` be the
same state with `e` present. The pair `a <-> b` gets:
- build rate `u`, taking one packet from the hot store;
- return rate `b_h · u`, giving one packet back.

The stores satisfy `b_c > b_h >= 2`.

The rate `u = u(o)` is a function of the pair's observation `o`, read from the
**absent endpoint** `a`. Both endpoints therefore share one rate, and every
fuel jump exchanges exactly `ln b_h` with the hot store. An agent that read
bridge `e`'s own bit could set different rates in the two directions; it would
be a demon outside local detailed balance (control C01).

Observation classes (keys for the pair `(a, e)`):
- **blind**: `e` (6 keys);
- **local**: `pos`, the bits of the bridges at `pos` in `a`, and `e`;
- **full**: `(a, e)`, one key per fuel pair (768 keys).

### 1.4 Steady state and the quantities

`p` is the unique stationary law of the chain: rates are positive, or `u = 0`,
on a connected undriven graph.

For a channel pair from `z` to `z'` with rates `k, k'`, the current is
`J = p(z) k - p(z') k'`.

```text
J          = sum over fuel pairs of J_pair                 net packets from the hot store
           = (b_c + 1) <|G|>_p - K                         (stationarity of <|G|>; WALK keeps |G|)
Sdot_A     = sum over fuel pairs of J_pair · ln r_pair     net change of S_tau by the agent
Sdot_U     = sum over WEATHER and WALK pairs of J · Delta S_tau
r_pair     = N_tau(b) / N_tau(a) >= 1                      the futures ratio of the build
H          = <N_tau>_p - <N_tau>_(pi_c)                    open future held above the undriven world
sigma      = J ln(b_c / b_h)                               entropy production
sigma_floor, eta = sigma_floor / sigma                     as in the information-law note
E          = H / H_warm(J)                                 matched-power edge
```

The warming rival spends the same fuel flow `J` uniformly on every bridge. Its
bridges are independent at density `rho*(J) = (J + K) / (K (b_c + 1))`, and
its traveller is uniform. So `H_warm(J) = Phi_tau(rho*) - Phi_tau(r_0)`, where:
- `Phi_tau(rho)` is `<N_tau>` under independent bridges at density `rho` with
  a uniform position, an exact polynomial in `rho`;
- `r_0 = 1/(1 + b_c)`.

## 2. Three THAIM rewards, and what each must do

With `lambda = 1` and `alpha = 1/ln b_c`, every reward is energy per unit time.

```text
R_receipt = alpha · sum over fuel pairs of u p(a) max(0, ln r_pair) - J    one-sided receipts on the agent's builds
R_net     = alpha · Sdot_A - J                                             the agent's own net THAIM flow
R_world   = alpha · (Sdot_A + Sdot_U) - J                                  the world's net THAIM flow
```

**Theorem 1 (the world form is null).** In every steady state,
`Sdot_A + Sdot_U = 0`, so `R_world = -J` for every policy.

*Proof.*
`Sdot_A + Sdot_U = sum over all transitions of J_l [S_tau(x') - S_tau(x)] = sum_x S_tau(x) dp(x)/dt = 0`.

**Theorem 2 (the net form prices builds at the world's temperature).**
`R_net = sum over fuel pairs of J_pair · (ln r_pair / ln b_c - 1)`.

*Proof.* Each fuel jump changes `|G|` by `±1`, so `J = sum of J_pair`.

A net build pays exactly when `r_pair > b_c`. For an isolated fuel channel with
`b_c > b_h`, the net current is `u p(a) (b_c - b_h)/(b_c + b_h u) > 0` for every
`u > 0`. So there `R_net > 0` if and only if `r > b_c`; gate G08 checks this
exactly. With many channels and a walking traveller, currents interact. That
the optimum is the rule "fuel if and only if `r_pair > b_c`" is a prediction
(H1), not a theorem. `R_net` contains no constant except `b_c`, which the world
declares before any rate is inspected.

**The receipt form pays gross builds but charges net fuel.** Returns destroy
futures and are never debited (`max(0, ·)`), while each return refunds its
packet to `J`. Weather wear makes the cycle imperfect. For an isolated channel
at rate `u`, gross builds per net packet are `(b_c + b_h u)/(b_c - b_h)`. So
receipts pay for a net packet when
`r > theta_R = b_c^((b_c - b_h)/(b_c + b_h u))`.

At `u = u_max`:

| world | W1 | W2 | W3 | W4 | W5 | W6 |
|---|---|---|---|---|---|---|
| theta_R | 1.45 | 1.21 | 2.47 | 6.86 | 4.91 | 32.86 |

Receipts therefore price futures too cheaply: a build with `theta_R < r < b_c`
earns receipts while losing net THAIM. Section 6 lists these window ratios for
every world. With interacting channels the isolated-channel threshold is only
an approximation. On the N = 3 analogues, receipts left window builds within
1.13 times `theta_R` unfuelled and fuelled those at 1.24 times `theta_R` and
above. On the N = 4, `tau = 2` analogue, they fuelled window builds down to 1.01
times `theta_R` (section 11). As a post-hoc observation, with no hypothesis
depending on it, every warm declared world has a window ratio at least 1.8 times
`theta_R`:
- W1: 21/8 against 1.45;
- W2: 21/8 against 1.21;
- W3: 8 against 2.47;
- W5: 16 against 4.91.

The prediction (H3) is that receipt training fuels builds that do not pay, and
loses net THAIM. Agreement with the isolated-channel threshold itself is
reported, not hypothesized.

**Training forms.** Training uses forms that depend on `p` alone:
- `R_net = -alpha · Sdot_U(p) - [(b_c + 1)<|G|>_p - K]`, by Theorem 1;
- `R_world = -[(b_c + 1)<|G|>_p - K]`.

`R_receipt` depends on `u` explicitly; its direct derivative is added to the
adjoint term.

**Context: the soft value.** Consider maximum-entropy reinforcement learning
with deterministic moves on a frozen graph, horizon `tau`, temperature `alpha`
and zero extrinsic reward. The soft Bellman equation
`V_t(s) = alpha ln sum_a exp(V_{t-1}(s_a)/alpha)` with `V_0 = 0` gives
`V_tau(s) = alpha ln N_tau(s)` at every temperature, because `alpha` cancels in
the recursion. THAIM uses that value, and the experiment fixes
`alpha = 1/ln b_c`. Here `G` changes over the horizon, so the identity is
motivation only; no hypothesis depends on it.

## 3. Agents

Every policy is stored as a table indexed by its key class. The rate of a fuel
pair is the table entry of its key.

**Tabular, trained.** One parameter `z_k` per key, with
`u = u_max · sigmoid(z_k)` and `u_max = 4`.
- `NET-blind`, `NET-local`, `NET-full`, trained on `R_net`;
- `RECEIPT-full`, trained on `R_receipt`;
- `WORLD-full`, trained on `R_world`;
- reported only: `NET-full` at price multiplier `c` in `{3/4, 3/2}`, trained on
  `c · alpha · Sdot_A - J`;
- reported only: `NET-full init 1/4`, `NET-full` started from `u = 1/4` instead of
  `u = 1`.

**Small network (reported only).** `MLP-full`, the frontier pilot's network
(`research/physics/xypher-frontier-pilots/xypher-eval/xeval.py`):
- input: the absent endpoint's position as one-hot `±1`, and its six bridge bits
  as `±1`;
- a `tanh` hidden layer of 8; `W1` normal with standard deviation `1/sqrt(d)`,
  `b1 = 0`, `W2` normal with standard deviation 0.1, `b2 = logit(2/u_max)`, so the
  initial rate is 2;
- its own Adam with `eps = 1e-8`, learning rate `0.03 · 0.1^(t/T)`, `T = 8000`;
- trained on `R_net` at seeds 0 and 1, with `numpy.random.default_rng(seed)`;
- key class: full.

**Fixed:**
- `LOCAL`, key class local: `u = 1` on bridges at the traveller's island, 0
  elsewhere;
- `GLOBAL`, key class blind: `u = 1` everywhere;
- `ORACLE`, key class full: `u = u_max` exactly on pairs with `r_pair > b_c`, 0
  elsewhere. Its table is precomputed from the section 6 combinatorics.

## 4. Training protocol (frozen)

- Each iteration solves `p` by sparse LU in float64.
- The gradient comes from the adjoint of the bordered stationary system, so one
  LU serves both solves. The explicit `u`-dependence of `R_receipt` is added.
- Adam per parameter: `beta = (0.9, 0.999)`, `eps = 1e-12`, learning rate
  `0.2 · 0.1^(t/T)` with `T = 8000` iterations and `t = 1..T`.
- Initial `z = logit(1/u_max)`, so every rate starts at `u = 1`.
- Tabular training uses no random numbers.

For every trained agent the report gives two numbers:
- the change of its objective over the last 100 iterations;
- its residual fuel flow, meaning the part of `J` carried by pairs whose rate
  is below `u_max/2`.

**Freezing.** At the end of training each rate is rounded to the nearest
multiple of `2^-24`, ties to even (Python `round`). The resulting exact rational
table is the agent; every verdict is computed on it.

**Canonical table text.** The raw key of a pair is a tuple:
- blind: `(e,)`;
- local: `(pos, bits of the bridges at pos in a, in lexicographic bridge order, e)`;
- full: `(a, e)`.

A key's index is its rank among the sorted distinct raw keys of its class. The
canonical text has one line per key, in index order: `index<TAB>numerator/denominator`.

## 5. Exact evaluation

For each frozen table:
- **Steady state.** `p` is solved exactly in rationals (python-flint
  `fmpq_mat`, 256 states).
- **Exact rationals.** `J`, `<|G|>`, `H`, `rho*`, `H_warm`, `E`, and rule
  agreement.
- **The price input.** The price enters as `alpha = 1/ln b_alpha`, where
  `b_alpha` is a declared input of the evaluator, equal to `b_c`.
- **Linear forms.** `Sdot_A`, `Sdot_U` and `sigma` are finite sums of rational
  multiples of logarithms of integers. So are
  `ln b_alpha · R_net = Sdot_A - J ln b_alpha`, `ln b_alpha · R_receipt` and
  `ln b_alpha · R_world`.
- **Decisions involving rewards.** Every sign, identity and comparison
  involving the rewards is decided on these scaled linear forms, as in
  `research/physics/xypher-information-law/infolaw.py`:
  1. test for an exact zero over a pairwise coprime base;
  2. otherwise take the sign of a certified arb enclosure, starting at 256 bits
     and doubling the precision until the enclosure excludes zero.
- **Reported enclosures.** `sigma_floor` and `eta` are reported as certified
  enclosures.
- **Undefined ratios.** `E`, `eta` and aim are reported as `none` when their
  denominator is zero (`J = 0`).

**Rule agreement.** A pair is *fuelled* if its rate is at least `u_max/2`, and
*paying* if `r_pair > b_c`. Agreement is the share of pairs on which fuelled
equals paying, weighted by `p(a) + p(b)` under the agent's own steady state.

**Aim.** The share of `J` carried by pairs whose bridge touches the traveller's
island.

No floating-point value decides a gate, a control, or a hypothesis.

## 6. Worlds

All six worlds have `N = 4`.

| world | tau | b_c | b_h | v | build ratios above b_c | does the rule need bridges the traveller cannot see? |
|---|---|---|---|---|---|---|
| W1 | 3 | 5 | 2 | 1 | 96 of 768 (r in {8, 12, 13, 16, 18}) | no |
| W2 | 3 | 5 | 3 | 4 | 96 of 768 | no |
| W3 | 3 | 9 | 2 | 1 | 72 of 768 (r in {12, 13, 16, 18}) | yes: 12 of 144 local keys, which are all 12 keys holding a paying pair |
| W4 | 3 | 20 | 2 | 1 | 0 of 768 (max r = 18) | no |
| W5 | 4 | 20 | 3 | 1 | 72 of 768 (r in {29, 34, 44, 56}) | yes: 12 of 144 local keys, which are all 12 keys holding a paying pair |
| W6 | 4 | 60 | 2 | 2 | 0 of 768 (max r = 56) | no |

- **Warm worlds** (some builds pay): W1, W2, W3 and W5.
- **Cold worlds** (no build pays): W4 and W6.
- W1 and W2 share `tau` and `b_c` but differ in `b_h` and `v`. The THAIM rule
  depends on neither.
- In W3 and W5, each ambiguous local key has 8 member pairs, of which 2 do not
  pay (`r = 8` in W3, `r = 16` in W5). Under `pi_c`, the non-paying members
  carry 0.810 (W3) and 0.907 (W5) of the ambiguous keys' weight.

**Frozen theory values.** These come from combinatorics and the undriven law
alone; no driven steady state was solved.

| value | W1, W2 | W3 | W4 | W5 | W6 |
|---|---|---|---|---|---|
| `<N_tau>_(pi_c)` | 97/18 | 853/250 | 6388/3087 | 8156/2401 | 24466996/13845841 |
| `r_0` | 1/6 | 1/10 | 1/21 | 1/21 | 1/61 |
| distinct build ratios | 36 | 36 | 36 | 36 | 36 |
| max build ratio | 18 | 18 | 18 | 56 | 56 |
| ratio equal to `b_c` | none | none | none | none | none |

Complete build-ratio multiplicities (768 pairs each):

```text
tau = 3: 1:60, 13/12:24, 9/8:12, 32/27:24, 39/32:24, 21/17:24, 32/25:12, 9/7:24, 25/19:24, 4/3:24,
         19/14:12, 18/13:24, 38/27:12, 13/9:12, 3/2:24, 32/21:24, 25/16:24, 27/17:12, 13/8:24, 64/39:12,
         28/17:12, 7/4:24, 38/21:24, 50/27:12, 2:24, 27/13:24, 17/8:24, 13/6:24, 9/4:24, 32/13:24, 21/8:24,
         8:24, 12:24, 13:24, 16:12, 18:12
tau = 4: 1:60, 34/29:24, 34/27:24, 14/11:12, 55/41:24, 139/102:24, 128/89:12, 81/55:24, 40/27:12, 89/60:24,
         44/29:24, 30/19:12, 28/17:24, 139/81:12, 89/51:24, 29/16:24, 256/139:12, 76/41:12, 102/55:24,
         55/29:24, 81/41:12, 17/8:24, 24/11:24, 178/81:12, 51/22:24, 81/34:24, 139/56:24, 41/16:24, 81/29:24,
         3:24, 55/16:24, 16:24, 29:24, 34:24, 44:12, 56:12
```

Receipt windows (build ratios with `theta_R < r < b_c`):

```text
W1: 3/2, 32/21, 25/16, 27/17, 13/8, 64/39, 28/17, 7/4, 38/21, 50/27, 2, 27/13, 17/8, 13/6, 9/4, 32/13, 21/8
W2: 39/32, 21/17, 32/25, 9/7, 25/19, 4/3, 19/14, 18/13, 38/27, 13/9, then the W1 window
W3: 21/8, 8
W4: 8, 12, 13, 16, 18
W5: 16
W6: 34, 44, 56
```

Every paying pair is a build at an isolated traveller's island.

## 7. Gates

Gates run in order, G01 to G13. Within a gate, checks run over the worlds W1
to W6, and within a world over the agents in this order:
1. LOCAL
2. GLOBAL
3. ORACLE
4. NET-blind
5. NET-local
6. NET-full
7. RECEIPT-full
8. WORLD-full
9. NET-full at `c = 3/4`
10. NET-full at `c = 3/2`
11. NET-full init 1/4
12. MLP-full seed 0
13. MLP-full seed 1

The **first failure** of a run or control is the first failing check of the
lowest-numbered failing gate.

| gate | check |
|---|---|
| G01 undriven law | Every WEATHER and WALK pair satisfies detailed balance with `pi_c` exactly, and `pi_c` is stationary under the undriven channels alone. |
| G02 no demon | Every fuel pair has rates `(u, b_h u)` with one `u`. That `u` is the table entry of the pair's key in the agent's declared class, lies in `[0, u_max]`, and is a multiple of `2^-24`. |
| G03 exact steady state | For every evaluated table, `p > 0`, `sum p = 1`, and `pQ = 0` exactly. |
| G04 THAIM balance | For every evaluated table, `Sdot_A + Sdot_U = 0` exactly over a coprime base. |
| G05 fuel identities | `J = sum of fuel currents = (b_c + 1)<|G|> - K` exactly. `sigma_world = J ln(b_c/b_h)` exactly, as rational coefficients of two formal symbols `L_c, L_h`. `b_c > b_h`. |
| G06 ratio table | The section 6 counts, sets, maxima, absence of ties, local-ambiguity counts, complete multiplicity tables, and receipt windows, recomputed exactly. Window membership `r > theta_R` is decided as `r^den > b_c^num` for the exponent `num/den = (b_c - b_h)/(b_c + b_h u_max)`. |
| G07 warming calibration | `GLOBAL` has independent bridges at density `2/(2 + b_c + b_h)` and a uniform traveller. Its `H` equals `Phi_tau(rho*) - Phi_tau(r_0)` exactly, and its `E = 1` exactly. |
| G08 price calibration | In the reference world `N = 2` (one bridge), with the `tau`, stores and `v` of each declared world and fuel `u = 1`: `ln b_alpha · R_net = Sdot_A - J ln b_alpha` equals `J (tau ln 2 - ln b_c)` exactly, with the declared `b_c`. |
| G09 determinism | A second training of every tabular agent gives the same rational table (SHA-256 of the canonical table text). |
| G10 freezing | Every frozen rate differs from its float value by at most `2^-25`. |
| G11 frozen values | `<N_tau>_(pi_c)` and `r_0` match section 6 exactly. |
| G12 tables | Each agent's table is written to disk as canonical text indexed by its key class, then read back. It must regenerate the evaluated rate of every fuel pair exactly. The section 9 slot reading is printed with the report. |
| G13 optimizer sanity | By Theorem 1, `R_world = -J`. In every world, `WORLD-full` has `J <= J_LOCAL / 20`. A failure here means the optimizer failed, not THAIM. |

## 8. Controls (each must fail where stated)

Witnesses: every control runs G01 to G07 on W1 with the GLOBAL table
(`u = 1` on every pair), and G08 on the reference world with W1's `tau`, `b_c`,
`b_h` and `v`. C05 replaces `b_h` by `b_c` in both. C06 corrupts W1's frozen
record.

A control runs through the gates in order and stops at its first failing check.
Nothing that divides by `J`, `sigma` or `H_warm` is evaluated after that check.

| control | change | must fail first at |
|---|---|---|
| C01 demon fuel | On one pair, the return rate exceeds `b_h` × build rate (the rate reads bridge `e`'s own bit). | G02 |
| C02 weather reads the agent | Each bridge's WEATHER wear rate is multiplied by `(1 + u)` of its fuel pair. | G01 |
| C03 receipts booked as net flow | The receipt flow is used in place of `Sdot_A` in the balance. | G04 |
| C04 price from the hot store | `b_alpha = b_h`, so `alpha = 1/ln b_h`. The two sides of G08 then differ by `J (ln b_c - ln b_h) != 0`. | G08 |
| C05 equal stores | `b_h = b_c`. | G05 |
| C06 corrupted ratio table | The frozen count of paying pairs in W1 is changed from 96 to 97. | G06 |

## 9. Xypher slot reading

This is a minimal thermodynamic structure; the full adaptive items do not
apply.

- **Graph Substrate:** the configuration `G` and the traveller's position.
- **Environment:** WEATHER and WALK, reciprocal with the cold store.
- **Praxion:** the fuel channels.
- **Crystal:** the observation key read by the policy. It holds no memory, so
  the register is empty.
- **Thermo:** two declared stores `b_c` and `b_h`; `alpha = 1/ln b_c`, fixed
  before any rate is inspected; `U = lambda |G|`.
- **Ruby** empty, **Opal** absent, no Thompson memory.

The policy's rates are a function of the key alone. No rate reads `N_tau`,
`H`, `J`, `E`, or any reward, with one declared exception: ORACLE's table is
precomputed from the section 6 combinatorics. Rewards enter only training,
through the gradient. A trained table is a fixed policy, and evaluation never
consults a reward to set a rate.

## 10. Hypotheses

Each hypothesis is decided on exact values, or on the scaled linear forms of
section 5. Theorem 1 already settles the world form; it is checked as gate
G13, not hypothesized.

**H1 (the THAIM rule).** In every world, `NET-full`'s rule agreement is at
least 0.98. In the cold worlds the rule is "fuel nothing", so H1 there overlaps
H2(b).

**H2 (THAIM pays only where physics says).**
- (a) In every warm world: `R_net(ORACLE) > 0`, `R_net(NET-full) > 0`, and
  `R_net(NET-full) >= 0.98 · R_net(ORACLE)`.
- (b) In every cold world: `NET-full` has `J <= J_LOCAL / 20`.

**H3 (receipts price futures too cheaply).** Decided in a warm world only if
`R_net(NET-full) > 0`; otherwise it is undecided there. In every warm world where
it is decided, both of the following hold:
- `RECEIPT-full` fuels builds that do not pay. The steady-state weight
  `p(a) + p(b)` of pairs with rate at least `u_max/2` and `r_pair <= b_c` is at
  least 0.01 of the total pair weight.
- It loses net THAIM against the net form:
  `R_net(NET-full) - R_net(RECEIPT-full) > 0.01 · R_net(NET-full)`.

**H4 (THAIM beats hand-written aim).** In every warm world:
`E(NET-full) > E(LOCAL)`.

**H5 (the map is worth what the rule needs).** Decided in a world only if
`R_net(NET-full) > 0`; otherwise it is undecided there.
- In W3 and W5, where the rule needs bridges the traveller cannot see:
  `R_net(NET-full) - R_net(NET-local) > 0.01 · R_net(NET-full)`.
- In W1 and W2:
  `R_net(NET-full) - R_net(NET-local) <= 0.01 · R_net(NET-full)`.

This outcome is expected from the structure in section 6: in W3 and W5 the
ambiguous local keys hold all the paying pairs, and their non-paying members
carry most of their weight.

**Reported, with no hypothesis:**
- for every agent: `H`, `J`, residual fuel flow, `eta`, aim, `E`, and the final
  objective change;
- `RECEIPT-full`'s agreement with the isolated-channel threshold `theta_R`;
- `NET-blind`;
- `MLP-full` (two seeds);
- the price scan.

**OVERALL PASS** requires every gate to pass and every control to fail first
where stated. Hypotheses are decided separately; a refuted hypothesis is a
result.

## 11. Pilots, blinding, and what was computed before the freeze

**Design pilots** used only N = 4 worlds with `tau = 2`: `(b_c, b_h)` in
{(8, 2), (3, 2)} and `v` in {1, 4}. The frontier pilot
(`research/physics/xypher-frontier-pilots/`) also used N = 4 and N = 5 with
`tau = 2`. The pilots showed four things:
- receipt training overspends;
- `R_world` training idles;
- tabular `R_net` training reaches the ORACLE rule, with agreement 1.000, while
  the pilot MLP mostly collapses to idling;
- the matched-power edge `E` favours more selective thresholds than the THAIM
  price. No hypothesis therefore claims that the price `c = 1` maximizes `E`.

**Calibration after the blind reviews.** As an exception to the embargo, the
apparatus's trainer and evaluator were run on the non-declared analogues below.
Every statistic viewed is listed. Scripts and outputs are in
`research/physics/xypher-thaim-reward-proof/pilots/`, with their SHA-256:

```text
design_check.py   e3ed5aaed6e9aafe1fe325f958e37833ba811d114208662b5d54e1f14ab77ad6   design_check.txt  1f3041a3251e8f902615ff2701c7435db88f07f3e1eb3bf013f832dd0d231e44
design_extra.py   429c5558296f8d7d6fa91defcf61923dac069da66f3067defa302ccc371f6879   design_extra.txt  c7dffad3e358f30f8cd224bc371da3d8e3fd7eca16b403f90a0680c1d2509633
tab.py            b88bcc8d521e59c6d141c29b9a83d209c9ff577324288272035df223f782b831
calib.py          4b1480715b2739878afc4f75588a923fdc5ad176c7f7426b3c44a728583589c4   calib.txt         9723a33e9e1edb803859446e3e6f402d7ba6eeb95c55d8692f2c905bc553ba3c
receipt_rule.py   eb953a45b73f7c155a92eba9d76cb070c4a31d7e760e8a628ea06abff16a3023   receipt_rule.txt  8fccfc8d537731a8f7b09116aa8c8a53e1cbbc0001f32259368e2d0175e85509
frozen_theory.txt 5c510edec1c4edddee0eb6eabe6004e11f678db183ce3a6b0d8e982f7c47abf3   multiplicities.txt 96b3ecec4a01b591967d80285c0042f1be6964bf411ad6b7a8f341aecbe88e6e
```

**Training protocol.**
- Analogues:
  - N = 3, `tau = 3`, `(b_c, b_h, v)` in {(9,2,1), (10,2,1), (5,2,1)};
  - N = 3, `tau = 4`, in {(20,3,1), (9,2,3)};
  - N = 4, `tau = 2`, (3,2,1).
- Settings tried: `(T, learning rate)` in {(1500, 0.05), (1500, 0.2), (4000, 0.2),
  (8000, 0.2), (8000, 0.5)}.
- The ratio `R_net(NET-full)/R_net(ORACLE)`, with rule agreement 1.000 in every
  case:

  | setting | ratio range |
  |---|---|
  | (1500, 0.05) | 0.855–0.994 |
  | (1500, 0.2) | 0.983–0.999 |
  | (4000, 0.2) | 0.997–1.000 |
  | (8000, 0.2) | 0.9996–1.0000 |
  | (8000, 0.5) | 0.9999–1.0000 |

- Rule: the smallest `T` at learning rate 0.2 that reached 0.999 on every
  analogue was chosen, giving (8000, 0.2). The referee's independent code
  reached 0.9991–0.99998 with this protocol.

**Receipts** (protocol (8000, 0.2)):

| analogue | agreement with `theta_R` | `R_net` RECEIPT | `R_net` NET | `J` ratio RECEIPT/NET |
|---|---|---|---|---|
| N=4 tau=2 (3,2,1) | 0.986 | -0.532 | +0.137 | 3.4 |
| N=4 tau=2 (8,2,1) | 1.000 | -0.677 | 0 (NET idles) | — |
| N=3 tau=3 (9,2,1) | 1.000 | -0.071 | +0.036 | 7.4 |
| N=3 tau=4 (20,3,1) | 1.000 | -0.209 | +0.025 | 16.3 |
| N=3 tau=3 (20,2,1) | 0.397 | -0.038 | 0 | — |
| N=3 tau=4 (60,2,2) | 1.000 | 0 | 0 | both idle |
| N=3 tau=3 (5,2,1) | 0.841 | +0.109 | +0.334 | 1.4 |

The referee's independent analogues gave the following.
- `J` ratios:
  - 1.000 at N=3 `tau = 4` (9,2,1) and (9,2,3), where RECEIPT-full fuelled exactly
    the paying pairs. The window there holds only 41/16 and 81/29, within 1.13
    times `theta_R = 2.47`.
  - 1.37 at (5,2,1) `tau = 3`, 1.43 at (5,2,4), and 1.67 at (5,3,4).
  - 3.38 at N=4 `tau = 2` (3,2,1), and 7.4–16 elsewhere.
- Positive receipt `R_net` in three warm analogues: +0.109 at `tau = 3` (5,2,1),
  and +0.587 at `tau = 4` (9,2,1) and (9,2,3).

The first H3, `J(RECEIPT) >= 1.3 J(NET)`, had its cut 5% below W1's analogue. It
was replaced after these numbers were seen. The present H3 states the mechanism
of section 2: receipts fuel builds that do not pay, and lose net THAIM. Both of
its 1% margins are the same minimal margin as H5, not fitted to any analogue.
The factor criterion in section 2 (window ratios at least 1.8 times `theta_R`
in every warm declared world) was also noticed after the analogues. It is
disclosed here as design information.

**H4 on analogues** (referee): `E(NET-full)/E(LOCAL)` was:
- 0.977 at N=3 `tau = 3` (5,2,1), the analogue of W1;
- 0.928 and 0.940 at `tau = 4` (9,2,1) and (9,2,3);
- 1.009 at (5,2,4), and 1.016 at (5,3,4), the analogue of W2;
- 1.08–1.46 elsewhere.

H4 is unchanged. It is a genuine risk in W1 and W2.

**Blinding.** No declared world (`tau` in {3, 4}) has been solved for any
policy. For the declared worlds, only the section 6 values were computed
before the freeze: walk-count combinatorics and the undriven law. Referees and
reviewers may recompute those values. They may not solve any driven steady
state of a declared world before the run.

## 12. Apparatus

`research/physics/xypher-thaim-reward-proof/`, in Python 3. It uses numpy 2.0.2
and scipy 1.13.1 for training, and python-flint 0.6.0 for exact evaluation,
pinned in `requirements.txt`. It contains:
- the world builder, float and exact;
- the tabular and MLP trainers;
- the exact evaluator;
- the gates, controls and hypothesis decisions;
- a single entry point that prints the report.

This is a measurement apparatus, not a Xypher runtime; no Futuruna
implementation is claimed.

**Embargo.** Until three static reviewers approve the exact commit, the
apparatus may run only:
- `python -m py_compile`;
- unit tests on non-declared worlds: N = 2, N = 3, and the N = 4 `tau = 2`
  pilot worlds;
- an adjoint-versus-finite-difference gradient check, on non-declared worlds
  only.

The section 11 calibration exception is closed. No further training or evaluation
runs before the single run.

**Run.** One command, after the approved commit is pushed and HEAD equals the
remote. The report's SHA-256 is recorded in the result record.

## 13. What the outcomes would mean

- **Theorem 1**, checked as G13: THAIM booked for the whole world carries no
  training signal.
- **H1 and H2 supported:** an agent trained only on its own net THAIM flow acts
  on the world's exchange rate between futures and timber, which is encoded in
  `R_net` through `alpha = 1/ln b_c`. Gradient training recovers the per-pair
  rule despite interacting currents, and the agent acts only where that rate makes
  agency pay.
- **H3 supported:** receipts are an accounting record, not a reward. Paid as a
  reward, they price futures too cheaply and fuel builds the THAIM price would
  not.
- **H4** decides whether the THAIM price guides fuel better than hand-written
  aim.
- **H5** decides whether the value of a map follows from what the rule needs to
  see.
- **Any refutation** names which of these properties THAIM lacks as a training
  signal.

## References

- J. M. Horowitz, K. Zhou, and J. L. England, Phys. Rev. E 95, 042102 (2017).
- A. D. Wissner-Gross and C. E. Freer, Phys. Rev. Lett. 110, 168702 (2013).
- T. Haarnoja, A. Zhou, P. Abbeel, and S. Levine, "Soft Actor-Critic," ICML
  (2018).
- A. Y. Ng, D. Harada, and S. Russell, "Policy invariance under reward
  transformations," ICML (1999).
- A. M. Turner, L. Smith, R. Shah, A. Critch, and P. Tadepalli, "Optimal
  policies tend to seek power," NeurIPS (2021).
- A. S. Klyubin, D. Polani, and C. L. Nehaniv, "Empowerment: a universal
  agent-centric measure of control," IEEE CEC (2005).
- K. Takahashi and Y. Hayashi, "Thermodynamic limits of physical
  intelligence," AGI 2026, LNCS, doi:10.1007/978-3-032-33195-3_24.
- Xyphers research: the committed-futures manuscript (formula I); the
  information law (`xypher-information-law.md`); the CAL-CEF-3 and CAL-CEF-4
  boundaries.
