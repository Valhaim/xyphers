#!/usr/bin/env python3
"""Exact checks for "Committed futures as microstates".

The CAL-CEF-1 verifier (Rust, preregistered) checked the configuration law,
the odds, the degree law, and same-temperature contact on eight cases. This
script reproduces the manuscript's additional exact checks, which were not
preregistered: local replanning, the case N = 2, the flux criterion and the
protection bijection, the non-Markov holding time, many travellers, the
degree law on more islands, the build-only dynamics, the size-biased
identity, the long-horizon sweep, and the destination law of open problem Q1.

Dependency-free Python 3.8+. Integer and rational arithmetic only
(fractions.Fraction); no floating-point value enters any check.

    python3 checks.py

prints one line per check and ends with ALL CHECKS PASS (exit status 0).
"""

import itertools
import sys
from collections import deque
from fractions import Fraction

HOME = 0


def bridges_of(n):
    return list(itertools.combinations(range(n), 2))


def size(config):
    return bin(config).count("1")


def adjacency(n, pairs, config):
    adj = {v: {v} for v in range(n)}
    for i, (u, w) in enumerate(pairs):
        if config >> i & 1:
            adj[u].add(w)
            adj[w].add(u)
    return adj


def plans(n, pairs, config, tau, home=HOME):
    """All tau-step walks from home, a stay allowed at every step."""
    adj = adjacency(n, pairs, config)
    out = [(home,)]
    for _ in range(tau):
        out = [p + (w,) for p in out for w in sorted(adj[p[-1]])]
    return out


def count_by_matrix(n, pairs, config, tau, home=HOME):
    """e_h^T (I + A_G)^tau 1 by integer matrix-vector products."""
    adj = adjacency(n, pairs, config)
    vec = [1] * n
    for _ in range(tau):
        vec = [sum(vec[j] for j in adj[i]) for i in range(n)]
    return vec[home]


def uses(plan, pair):
    u, w = pair
    return any({plan[k], plan[k + 1]} == {u, w} for k in range(len(plan) - 1))


class World:
    """System-state chain: reservoir lumped (Theorem 1(ii)), so a Build or
    Dismantle into configuration G' has rate b^(K-|G'|); Step and Replan
    have rate 1. A state is (config, ((plan, cursor), ...)), one pair per
    traveller."""

    def __init__(self, n, tau, b, homes=(HOME,), replan="uniform", dismantle=True):
        self.n, self.tau, self.b = n, tau, b
        self.homes, self.replan, self.dismantle = homes, replan, dismantle
        self.pairs = bridges_of(n)
        self.k = len(self.pairs)
        self.plans = {}
        self.plan_sets = {}
        for config in range(1 << self.k):
            for home in set(homes):
                ps = plans(n, self.pairs, config, tau, home)
                self.plans[config, home] = ps
                self.plan_sets[config, home] = set(ps)
        self.states = []
        for config in range(1 << self.k):
            per = [[(p, t) for p in self.plans[config, h] for t in range(tau + 1)] for h in homes]
            self.states += [(config, combo) for combo in itertools.product(*per)]
        self.rates = {s: self._moves(s) for s in self.states}

    def weight(self, state):
        return Fraction(1, self.b ** size(state[0]))

    def _replans(self, config, home, plan):
        if self.replan == "uniform":
            return [q for q in self.plans[config, home] if q != plan]
        targets = []
        for k in range(1, self.tau + 1):
            for v in range(self.n):
                if v != plan[k]:
                    q = plan[:k] + (v,) + plan[k + 1:]
                    if q in self.plan_sets[config, home]:
                        targets.append(q)
        return targets

    def _moves(self, state):
        config, travellers = state
        out = {}

        def add(target, rate):
            out[target] = out.get(target, 0) + rate

        for j, (plan, t) in enumerate(travellers):
            def put(p, c):
                moved = list(travellers)
                moved[j] = (p, c)
                return (config, tuple(moved))

            if t < self.tau:
                add(put(plan, t + 1), 1)
            if t > 0:
                add(put(plan, t - 1), 1)
            if t == 0:
                for q in self._replans(config, self.homes[j], plan):
                    add(put(q, 0), 1)
            here = plan[t]
            for i, pair in enumerate(self.pairs):
                if here not in pair:
                    continue
                if not config >> i & 1:
                    target = config | 1 << i
                    add((target, travellers), self.b ** (self.k - size(target)))
                elif self.dismantle and not any(uses(p, pair) for p, _ in travellers):
                    target = config & ~(1 << i)
                    add((target, travellers), self.b ** (self.k - size(target)))
        return out

    def closed(self):
        valid = set(self.states)
        return all(t in valid for s in self.states for t in self.rates[s])

    def detailed_balance(self):
        edges = 0
        for s, out in self.rates.items():
            for t, r in out.items():
                edges += 1
                if self.weight(s) * r != self.weight(t) * self.rates[t].get(s, 0):
                    return False, edges
        return True, edges

    def reachable(self, start, within=None):
        seen = {start}
        queue = deque([start])
        while queue:
            for t in self.rates[queue.popleft()]:
                if t not in seen and (within is None or t in within):
                    seen.add(t)
                    queue.append(t)
        return seen

    def connected(self):
        return len(self.reachable(self.states[0])) == len(self.states)

    def marginal(self):
        law = [Fraction(0)] * (1 << self.k)
        for s in self.states:
            law[s[0]] += self.weight(s)
        total = sum(law)
        return [x / total for x in law], total


failures = []


def report(ok, name, detail):
    print(("PASS " if ok else "FAIL ") + name + " -- " + detail)
    if not ok:
        failures.append(name)


def fmt(values):
    return ",".join(str(v) for v in values)


def decimal(x, places=4):
    scaled = (x.numerator * 10 ** places * 2 + x.denominator) // (2 * x.denominator)
    whole, frac = divmod(scaled, 10 ** places)
    return f"{whole}.{frac:0{places}d}"


# 1. Theorem 1 and Proposition 1 (local replanning), including N = 2.
for (n, tau, b) in [(2, 1, 2), (2, 3, 2), (3, 1, 2), (3, 2, 2), (3, 3, 2), (3, 2, 3),
                    (4, 1, 2), (4, 2, 2), (4, 2, 3), (4, 3, 2)]:
    for rule in ("uniform", "edit"):
        world = World(n, tau, b, replan=rule)
        balanced, edges = world.detailed_balance()
        law, _ = world.marginal()
        counts = [len(world.plans[c, HOME]) for c in range(1 << world.k)]
        weights = [Fraction(counts[c], b ** size(c)) for c in range(1 << world.k)]
        z = sum(weights)
        law_ok = all(law[c] == weights[c] / z for c in range(1 << world.k))
        mean = sum(size(c) * law[c] for c in range(1 << world.k))
        report(world.closed() and balanced and world.connected() and law_ok,
               f"configuration-law {rule} (N,tau,b)=({n},{tau},{b})",
               f"system states={len(world.states)} edges={edges} detailed balance with b^-|G|, "
               f"connected, pi(G)=N_tau b^-|G|/Z with Z={z}, mean bridges={mean}")

# 2. Primary example (Table I, Fig. 1).
primary = World(3, 2, 2)
law, _ = primary.marginal()
counts = [len(primary.plans[c, HOME]) for c in range(8)]
report(counts == [1, 4, 4, 7, 1, 5, 5, 9] and law == [Fraction(x, 87) for x in (8, 16, 16, 14, 4, 10, 10, 9)],
       "primary (3,2,2)", f"N_2={fmt(counts)} pi={fmt(law)}")

# 3. Plan counts: depth-first enumeration equals e_h^T (I+A)^tau 1.
ok = True
for n in (3, 4, 5):
    pairs = bridges_of(n)
    for tau in (1, 2, 3, 4):
        for config in range(1 << len(pairs)):
            ok &= len(plans(n, pairs, config, tau)) == count_by_matrix(n, pairs, config, tau)
report(ok, "plan-count", "enumerated walks equal matrix-power counts for N=3..5, tau=1..4, every configuration")

# 4. Lemma 3 (which configurations exchange) and Remark 3 (protection).
for (n, tau) in [(3, 1), (3, 2), (3, 3), (4, 1), (4, 2), (4, 3), (5, 2), (5, 3)]:
    pairs = bridges_of(n)
    cases = zero = 0
    ok = True
    for config in range(1 << len(pairs)):
        adj = adjacency(n, pairs, config)
        dist = {HOME: 0}
        queue = deque([HOME])
        while queue:
            v = queue.popleft()
            for w in adj[v]:
                if w not in dist:
                    dist[w] = dist[v] + 1
                    queue.append(w)
        before = plans(n, pairs, config, tau)
        for i, pair in enumerate(pairs):
            if config >> i & 1:
                continue
            after = plans(n, pairs, config | 1 << i, tau)
            build = {(p, t) for p in before for t in range(tau + 1) if p[t] in pair}
            dismantle = {(p, t) for p in after for t in range(tau + 1) if p[t] in pair and not uses(p, pair)}
            crossing = sum(1 for p in after if uses(p, pair))
            reachable = min(dist.get(u, n + tau + 1) for u in pair) <= tau
            ok &= build == dismantle and crossing == len(after) - len(before) and bool(build) == reachable
            cases += 1
            zero += not build
    report(ok, f"flux-and-protection (N,tau)=({n},{tau})",
           f"{cases} (G, unbuilt e) pairs: dismantle-capable states of G+e = build-capable states of G; "
           f"plans crossing e = N(G+e)-N(G); positive flux iff an endpoint within distance tau "
           f"(zero-flux pairs: {zero})")

# 5. Proposition 2: stationary configuration process is not Markov.
for (n, tau, b) in [(2, 1, 2), (3, 2, 2), (4, 1, 3), (4, 3, 2)]:
    for rule in ("uniform", "edit"):
        world = World(n, tau, b, replan=rule)
        g = 1
        inside = [s for s in world.states if s[0] == g]

        def exit_rate(s):
            return sum(r for t, r in world.rates[s].items() if t[0] != g)

        def second(s):
            total = sum(world.rates[s].values())
            return sum(-r * exit_rate(t) for t, r in world.rates[s].items() if t[0] == g) + total * exit_rate(s)

        m = len(inside)
        s1 = Fraction(sum(-exit_rate(s) for s in inside), m)
        s2 = Fraction(sum(second(s) for s in inside), m)
        e1 = Fraction(sum(exit_rate(s) for s in inside), m)
        e2 = Fraction(sum(exit_rate(s) ** 2 for s in inside), m)
        sigma = (g, (((HOME,) * (tau + 1), 0),))
        crossing = (g, (((HOME,) * tau + (1,), 0),))
        into_empty = [sum(r for t, r in world.rates[s].items() if t[0] == 0) for s in (sigma, crossing)]
        ok = s1 == -e1 and s2 == e2 and s2 - s1 ** 2 > 0 and into_empty == [b ** world.k, 0]
        report(ok, f"non-Markov {rule} (N,tau,b)=({n},{tau},{b})",
               f"G={{0,1}}: S'(0)={s1}=-<eps>, S''(0)={s2}=<eps^2>, Var(eps)={s2 - s1 ** 2}>0; "
               f"rates into empty from (sigma,0) and ((0,..,0,1),0) = {into_empty[0]}, {into_empty[1]}")

# 6. Proposition 3: many travellers.
for homes in [(0, 0), (0, 1), (0, 1, 2)]:
    tau = 2 if len(homes) == 2 else 1
    world = World(3, tau, 2, homes=homes)
    balanced, _ = world.detailed_balance()
    law, _ = world.marginal()
    product = []
    for c in range(8):
        value = Fraction(1, 2 ** size(c))
        for h in homes:
            value *= len(world.plans[c, h])
        product.append(value)
    z = sum(product)
    ok = world.closed() and balanced and world.connected() and law == [x / z for x in product]
    report(ok, f"many-travellers homes={homes} (N,tau,b)=(3,{tau},2)",
           f"system states={len(world.states)}, detailed balance, connected, "
           f"pi(G) = b^-|G| prod_i N^(h_i)(G) / Z = {fmt(law)}")

# 7. Proposition 4: degree law at tau = 2.
for n in (3, 4, 5, 6):
    pairs = bridges_of(n)
    count = [count_by_matrix(n, pairs, c, 2) for c in range(1 << len(pairs))]
    ok = True
    cases = 0
    for config in range(1 << len(pairs)):
        adj = adjacency(n, pairs, config)
        degree = {v: len(adj[v]) - 1 for v in range(n)}
        ok &= count[config] == sum(degree[v] + 1 for v in adj[HOME])
        for i, (u, w) in enumerate(pairs):
            if config >> i & 1:
                continue
            delta = count[config | 1 << i] - count[config]
            if u == HOME:
                predicted = degree[w] + 3
            else:
                predicted = (u in adj[HOME]) + (w in adj[HOME])
            ok &= delta == predicted
            cases += 1
    report(ok, f"degree-law N={n}", f"N_2 = sum over N[h] of (d_v+1) and Delta N_2 on all {cases} (G, unbuilt e) pairs")

# 8. Proposition 5: a build-only dynamics is absorbed by the complete configuration.
for (n, tau) in [(3, 1), (3, 2), (4, 2)]:
    world = World(n, tau, 2, dismantle=False)
    full = (1 << world.k) - 1
    top = {s for s in world.states if s[0] == full}
    absorbed = all(any(t[0] == full for t in world.reachable(s)) for s in world.states)
    closed_top = all(t in top for s in top for t in world.rates[s])
    one_class = len(world.reachable(next(iter(top)), within=top)) == len(top)
    irreversible = any(world.rates[t].get(s, 0) == 0 for s in world.states for t in world.rates[s])
    report(absorbed and closed_top and one_class and irreversible, f"build-only (N,tau)=({n},{tau})",
           f"complete configuration reachable from all {len(world.states)} states, closed, one class of {len(top)} states; "
           f"some move has no reverse")

# 9. Remark 2: Z = (1 + 1/b)^K E_q[N_tau] with q = 1/(1+b).
ok = True
for (n, tau, b) in [(3, 1, 2), (3, 2, 2), (3, 3, 2), (3, 2, 3), (4, 1, 2), (4, 2, 2), (4, 2, 3), (4, 3, 2)]:
    pairs = bridges_of(n)
    k = len(pairs)
    q = Fraction(1, 1 + b)
    z = sum(Fraction(count_by_matrix(n, pairs, c, tau), b ** size(c)) for c in range(1 << k))
    mean = sum(count_by_matrix(n, pairs, c, tau) * q ** size(c) * (1 - q) ** (k - size(c)) for c in range(1 << k))
    ok &= z == (1 + Fraction(1, b)) ** k * mean
    if (n, tau, b) == (3, 2, 2):
        primary_mean = mean
report(ok and primary_mean == Fraction(29, 9), "size-biased-identity",
       f"Z = (1+1/b)^K E_q[N_tau] on the eight family cases; primary E_q[N_2] = {primary_mean}")

# 10. Exploratory long-horizon sweep, N = 5, b = 2.
pairs = bridges_of(5)
means = []
for tau in range(1, 13):
    z = bridges = home_bridges = Fraction(0)
    for config in range(1 << len(pairs)):
        weight = Fraction(count_by_matrix(5, pairs, config, tau), 2 ** size(config))
        z += weight
        bridges += weight * size(config)
        home_bridges += weight * sum(1 for i, (u, _) in enumerate(pairs) if u == HOME and config >> i & 1)
    means.append((bridges / z, home_bridges / z))
rising = all(means[i][0] < means[i + 1][0] and means[i][1] < means[i + 1][1] for i in range(11))
report(rising and means[0] == (Fraction(26, 7), Fraction(12, 7)), "long-horizon N=5 b=2",
       "mean bridges tau=1..12: " + " ".join(decimal(m[0], 3) for m in means)
       + "; mean home bridges: " + " ".join(decimal(m[1], 3) for m in means)
       + f"; strictly increasing; tau=1 exact {means[0][0]}, {means[0][1]}")

# 11. Open problem Q1: destination law at {0,1},{0,2}, tau = 2.
endpoints = [p[-1] for p in plans(3, bridges_of(3), 0b011, 2)]
law = [Fraction(endpoints.count(v), len(endpoints)) for v in range(3)]
report(law == [Fraction(3, 7), Fraction(2, 7), Fraction(2, 7)], "destination-law",
       f"uniform plans at {{0,1}},{{0,2}} end on islands 0,1,2 with probabilities {fmt(law)}")

if failures:
    print(f"{len(failures)} CHECK(S) FAILED")
    sys.exit(1)
print("ALL CHECKS PASS")
