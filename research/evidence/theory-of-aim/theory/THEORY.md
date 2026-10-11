---
title: Cavity Theory of the Price of Open Future
aliases:
  - Agency Cavity Theory
  - Alignment Theory of Aim
tags:
  - domain/physics
  - type/theory
  - topic/thermodynamics
  - topic/xypher
  - topic/nonequilibrium
domain: Physics
type: theory
status: retrospective-validation; prospective test CAL-CEF-5
td: td-2670e3
created: 2026-10-11
updated: 2026-10-11
related:
  - "[[Aimed Foresight Result]]"
  - "[[Open Future Price Result]]"
---

# Cavity theory of the price of open future

## 0. Claim

CAL-CEF-4 measured, exactly, how much open future an agent holds per unit of
fuel flow, relative to random placement of the same fuel. A theory with no
fitted parameter reproduces all 48 of its non-trivial edges `E` (every LOCAL
and AIM agent in all 18 worlds) within 0.62%. The theory explains the three
patterns of that experiment:
- the edge of aim grows with the size of the world;
- aiming further along a route lowers the edge;
- the agent that fuels only its next crossing leads.

It makes new predictions at parameters never computed exactly. The theory was
built after the CAL-CEF-4 run, from its data; this note is a retrospective
validation. Its first prospective test is CAL-CEF-5, which freezes its
predictions before any exact computation.

## 1. The theory

The world is the CAL-CEF-4 archipelago (Aimed Foresight Boundary, section 1).
The traveller state `z = (route, cursor)` moves by STEP and REPLAN; each bridge
`e` is a two-state process built at rate `1 + f_e(z)` and worn at rate
`b_c + b_h f_e(z)`, where `f_e(z)` is 1 when the agent's fuel may touch `e`.

Three facts make the problem nearly solvable:

1. A bridge's rates depend only on the traveller's state. Given the
   traveller's path, distinct bridges are independent, and each one's
   probability of standing evolves exactly.
2. For any one bridge `e`, the joint process (traveller state, bridge `e`)
   is a small Markov chain, solvable exactly. A step across `e` needs `e` to
   stand.
3. The traveller also needs the other bridges to stand when it crosses them.
   The theory's single approximation replaces each such bridge `e'` by its
   conditional probability `m_e'(z)` of standing, given the traveller's state
   (a cavity mean field). It solves the `K` joint chains self-consistently.

The open future of the traveller is then evaluated with products of the
conditional means. The fuel flow is
`J = sum_z pi(z) sum_e f_e(z) [(1 - m_e(z)) - b_h m_e(z)]`, and the edge is
`E = H / [Phi_N(rho) - Phi_N(r_0)]`, with `rho` the mean bridge density, as in
the boundary. Implementation:
`research/physics/xypher-agency-theory/cavity.py` (floating point; theory
only, never an acceptance path).

## 2. Retrospective validation

`cavity.py validate` against the frozen CAL-CEF-4 report
(`validation.txt`):

| Agents | Worlds | Steady states | Largest relative error | Sign |
|---|---:|---:|---:|---|
| LOCAL, AIM-1, AIM-2, AIM-3 | 18 | 48 | 0.62% (LOCAL at `(4, 2, 3, 2)`) | theory below exact in all 48 |

The theory reproduces every ordering in the report: AIM-1 above LOCAL in every
world, every foresight ladder falling, and the edge of LOCAL rising from three
to five islands. The worlds of CAL-CEF-3 (plans protected by the weather,
open future counted from home) are outside this theory.

## 3. What it explains

### 3.1 The value of a bridge to the traveller

At radius 2, the traveller's open future is
`N_2 = 1 + 2 deg(x) + sum over neighbours y of deg(y)`. One extra bridge at the
traveller's island adds `3 + (N - 2) r` to its expectation. One extra bridge
elsewhere adds about `2 r`, through the walks that reach it in one step. Here
`r` is the background density, between 1/9 and 1/4 in these worlds. A bridge
away from the traveller is therefore worth 7% to 15% of one at its island.

The foresight ladder falls for this reason. AIM-`k` with `k >= 2` spends part
of its fuel on route bridges away from the traveller's island. A bridge also
forgets: it relaxes at rate `1 + b_c`, four to nine times faster than the
traveller steps, so fuel placed ahead decays before the traveller arrives.

### 3.2 The edge of aim grows with the world

At weak drive, with all fuel on the traveller's `N - 1` bridges, the edge of
LOCAL is

```text
E_LOCAL  ->  N (3 + (N - 2) r_0) / (2 (3 + 2 (N - 2) r_0))  ->  N / 4   (N large).
```

At the exact density, the same formula with `2 r_0` replaced by `rho + r_0`
lies within 3% of all 18 exact LOCAL edges of CAL-CEF-4, always above them.
The full theory extends the law to worlds no exact solver can reach
(`scaling.txt`). At stores x8 and x2 the edge of LOCAL is 2.58 at six
islands, 3.94 at ten, and 6.94 at twenty. It keeps growing linearly while
`E/N` falls slowly toward 1/4. Random placement spreads bridges over all
`N (N - 1)/2` pairs, while the traveller uses those near it, and the share
that touches its island, `2/N`, keeps shrinking.

### 3.3 Why the nearest aim leads

From the exact CAL-CEF-4 values, AIM-1's lead over LOCAL (1% to 3%) splits
into two factors. One is the open future per extra bridge held (AIM-1's bridges
are worth more to the traveller). The other is the random rival's value per
extra bridge at each agent's own density. Random bridges reinforce one another,
so the rival does better at the higher density of LOCAL. Placement accounts for
29% to 73% of the lead, depending on the stores; the rival's density effect
accounts for the rest. The edge `E` therefore keeps a small bias in favour of
agents that spend less. It is about 1% here, far below the 15% to 25% fall
along the foresight ladders.

## 4. Predictions

1. **Faster travellers gain less from aim.** Multiplying the traveller's step
   and replanning rates by `v` lowers every aimed agent's edge: a traveller
   much faster than the world's memory averages its fuel over everything it
   visits. In the theory the edge levels off above 1, because a blocked step
   ties the traveller's island to the standing bridges at any speed.
2. **Blind foresight never pays.** No combination tried makes the ladder rise:
   - a longer measure (`R = 3`);
   - faster travellers;
   - slower replanning;
   - asymmetric step rates.
   The route is chosen at random, so aiming further along it means aiming at
   bridges the traveller may never use while they still stand. Foresight can
   pay only if the plan carries information about where value lies (CAL-CEF-6).
3. **The edge of aim grows linearly with the size of the world** (section
   3.2).

CAL-CEF-5 freezes the theory's numbers for 56 new steady states and tests them
exactly: `predictions_cef5.py` and `predictions_cef5.txt` in the same
directory.

## 5. Scope

The theory is mean field in one place (section 1, item 3). It treats the
CAL-CEF-4 world class: unprotected routes held as intentions, weather on every
bridge, and fuel on a permission set that depends on the traveller's state. It
does not cover plans constrained by the bridges (CAL-CEF-1 to 3), plans chosen
by what they open, or several travellers.
