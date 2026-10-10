#!/usr/bin/env python3
"""Independent check of CAL-CEF-3 ("How much open future can a unit of energy buy?").

Written after the frozen run, from the boundary alone, sharing no code with the
Rust apparatus. It rebuilds every driven world of boundary section 1, solves
each steady state exactly (fraction-free elimination over Python integers),
and compares H and J with the frozen report. It then re-decides H0--H3.

    python3 independent_check.py expected-report.txt

Standard library only. About ninety seconds on a laptop.
"""

import itertools
import re
import sys
from fractions import Fraction as F

PAIRS = ((3, 2), (8, 2), (6, 3))
WORLDS = [(3, tau, b) for b in PAIRS for tau in (1, 2, 3)] + [(4, 1, b) for b in PAIRS]
R = 2


def solve_stationary(n, channels):
    """Exact p with p Q = 0, sum p = 1: rows are balance equations, the last replaced by normalisation."""
    a = [[0] * n for _ in range(n)]
    for i, j, rate in channels:
        a[j][i] += rate
        a[i][i] -= rate
    a[n - 1] = [1] * n
    rhs = [0] * (n - 1) + [1]
    prev = 1
    for k in range(n):
        piv = next(r for r in range(k, n) if a[r][k] != 0)
        if piv != k:
            a[k], a[piv] = a[piv], a[k]
            rhs[k], rhs[piv] = rhs[piv], rhs[k]
        akk, rowk, bk = a[k][k], a[k], rhs[k]
        for r in range(k + 1, n):
            row, ark = a[r], a[r][k]
            for c in range(k + 1, n):
                row[c] = (akk * row[c] - ark * rowk[c]) // prev
            rhs[r] = (akk * rhs[r] - ark * bk) // prev
            row[k] = 0
        prev = akk
    x = [F(0)] * n
    for k in range(n - 1, -1, -1):
        s = F(rhs[k]) - sum(a[k][c] * x[c] for c in range(k + 1, n) if a[k][c])
        x[k] = s / a[k][k]
    return x


def world(n_islands, tau, b_c, b_h, agent):
    bridges = list(itertools.combinations(range(n_islands), 2))
    k_bridges = len(bridges)
    plan_cache = {}

    def plans(config, steps):
        key = (config, steps)
        if key not in plan_cache:
            near = {v: {v} for v in range(n_islands)}
            for i, (u, w) in enumerate(bridges):
                if config >> i & 1:
                    near[u].add(w)
                    near[w].add(u)
            out = []

            def walk(path):
                if len(path) == steps + 1:
                    out.append(tuple(path))
                    return
                for v in sorted(near[path[-1]]):
                    walk(path + [v])

            walk([0])
            plan_cache[key] = out
        return plan_cache[key]

    def uses(plan, i):
        u, w = bridges[i]
        return any({plan[s], plan[s + 1]} == {u, w} for s in range(len(plan) - 1))

    states = [(c, p, t) for c in range(1 << k_bridges) for p in plans(c, tau) for t in range(tau + 1)]
    index = {s: i for i, s in enumerate(states)}
    channels, fuel = [], []
    for s in states:
        c, p, t = s
        moves = []
        if t < tau:
            moves.append(((c, p, t + 1), 1, None))
        if t > 0:
            moves.append(((c, p, t - 1), 1, None))
        if t == 0:
            moves += [((c, q, 0), 1, None) for q in plans(c, tau) if q != p]
        for i, (u, w) in enumerate(bridges):
            stores = [("cold", b_c)]
            if agent == "GLOBAL" or p[t] in (u, w):
                stores.append(("hot", b_h))
            for store, ratio in stores:
                if not c >> i & 1:
                    moves.append(((c | 1 << i, p, t), 1, (store, +1)))
                elif not uses(p, i):
                    moves.append(((c & ~(1 << i), p, t), ratio, (store, -1)))
        for dest, rate, packet in moves:
            channels.append((index[s], index[dest], rate))
            if packet and packet[0] == "hot":
                fuel.append((index[s], rate * packet[1]))
    p = solve_stationary(len(states), channels)
    open_future = lambda c: len(plans(c, R))
    held = sum(p[index[s]] * open_future(s[0]) for s in states)
    weights = [F(len(plans(c, tau)), b_c ** bin(c).count("1")) for c in range(1 << k_bridges)]
    cold = sum(w * open_future(c) for c, w in enumerate(weights)) / sum(weights)
    flow = sum(p[i] * signed_rate for i, signed_rate in fuel)
    return len(states), held - cold, flow


def main():
    report = {}
    for line in open(sys.argv[1]):
        if line.startswith("WORLD "):
            parts = line.split()
            h = F(re.search(r" H=(\S+)", line).group(1))
            j = F(re.search(r" J=(\S+)", line).group(1))
            report[(parts[1], parts[2])] = (h, j)
    yields, agree = {}, 0
    for n_islands, tau, (b_c, b_h) in WORLDS:
        for agent in ("LOCAL", "GLOBAL"):
            count, h, j = world(n_islands, tau, b_c, b_h, agent)
            key = (f"({n_islands},{tau},{R},{b_c},{b_h})", agent)
            same = report.get(key) == (h, j)
            agree += same
            yields[(n_islands, tau, b_c, b_h, agent)] = (h, h / j)
            print(f"{key[0]} {agent:6} states={count:3} H~{float(h):.4f} J~{float(j):.4f} "
                  f"Y~{float(h / j):.4f} {'agrees' if same else 'DISAGREES'}", flush=True)
    print(f"H and J agree with the frozen report in {agree} of {2 * len(WORLDS)} steady states")
    local = lambda n, t, b: yields[(n, t, *b, "LOCAL")]
    glob = lambda n, t, b: yields[(n, t, *b, "GLOBAL")]
    h0 = all(local(n, t, b)[0] > 0 for n, t, b in WORLDS)
    h1 = all(local(n, t, b)[1] > glob(n, t, b)[1] for n, t, b in WORLDS)
    h2 = [b for b in PAIRS if not local(3, 1, b)[1] < local(3, 2, b)[1] < local(3, 3, b)[1]]
    ratio = lambda t, b: local(3, t, b)[1] / glob(3, t, b)[1]
    h3 = [b for b in PAIRS if not ratio(1, b) < ratio(2, b) < ratio(3, b)]
    verdict = lambda ok: "SUPPORTED" if ok else "REFUTED"
    print(f"H0 {verdict(h0)}")
    print(f"H1 {verdict(h1)}")
    print(f"H2 {verdict(not h2)} failing={h2 or 'none'}")
    print(f"H3 {verdict(not h3)} failing={h3 or 'none'}")
    sys.exit(0 if agree == 2 * len(WORLDS) else 1)


if __name__ == "__main__":
    main()
