#!/usr/bin/env python3
"""Independent check of CAL-CEF-4 ("Does seeing further ahead buy more open future?").

Written from the boundary alone, sharing no code with the Rust apparatus. It
rebuilds every world of boundary sections 1 and 3, solves each steady state
exactly on the orbits of the island relabellings that fix home (fraction-free
elimination over Python integers), checks the lifted law on the full chain,
compares H and J with the frozen report, and re-decides H0--H4.

    python3 independent_check.py expected-report.txt

Standard library only; the largest worlds then take several minutes each and the
whole check takes one to two hours. If python-flint is installed, its exact
rational solver is used instead and the check takes a few minutes.
"""

import itertools
import math
import re
import sys
from fractions import Fraction as F

PAIRS = ((3, 2), (8, 2), (6, 3))
WORLDS = ((3, 1), (3, 2), (3, 3), (4, 1), (4, 2), (5, 1))


def agents(tau):
    return ["LOCAL"] + [f"AIM-{k}" for k in range(1, tau + 1)] + ["GLOBAL"]


class World:
    def __init__(self, n, tau):
        self.n, self.tau = n, tau
        self.bridges = list(itertools.combinations(range(n), 2))
        self.k = len(self.bridges)
        self.index = {b: i for i, b in enumerate(self.bridges)}
        self.routes = list(itertools.product(range(n), repeat=tau))
        self.states = [(c, r, t) for c in range(1 << self.k) for r in self.routes for t in range(tau + 1)]
        self.perms = [(0,) + q for q in itertools.permutations(range(1, n))]
        self.orbit_of, self.orbits = {}, []
        for z in self.states:
            if z in self.orbit_of:
                continue
            o = len(self.orbits)
            members = sorted({self.image(z, s) for s in self.perms})
            self.orbits.append(members)
            for m in members:
                self.orbit_of[m] = o

    def bit(self, u, w):
        return 1 << self.index[(min(u, w), max(u, w))]

    def image(self, z, s):
        c, r, t = z
        cc = 0
        for i, (u, w) in enumerate(self.bridges):
            if c >> i & 1:
                cc |= self.bit(s[u], s[w])
        return cc, tuple(s[v] for v in r), t

    def walk(self, r):
        return (0,) + r

    def permitted(self, agent, r, t):
        """Bit mask of bridges the agent's fuel may toggle at route r, cursor t."""
        p = self.walk(r)
        if agent == "GLOBAL":
            return (1 << self.k) - 1
        if agent == "LOCAL":
            x = p[t]
            return sum(self.bit(x, y) for y in range(self.n) if y != x)
        depth = int(agent.split("-")[1])
        mask = 0
        for step in range(max(0, t - depth), min(self.tau, t + depth)):
            if p[step] != p[step + 1]:
                mask |= self.bit(p[step], p[step + 1])
        return mask

    def channels(self, z, agent, b_c, b_h):
        """(destination, rate, class) for every channel out of z; class in step/replan/weather/fuel."""
        c, r, t = z
        p = self.walk(r)
        out = []
        for nt, a, b in ((t + 1, p[t], p[min(t + 1, self.tau)]), (t - 1, p[max(t - 1, 0)], p[t])):
            if 0 <= nt <= self.tau and nt != t and (a == b or c & self.bit(a, b)):
                out.append(((c, r, nt), 1, "step"))
        if t == 0:
            out += [((c, q, 0), 1, "replan") for q in self.routes if q != r]
        fuel = self.permitted(agent, r, t) if agent else 0
        for i in range(self.k):
            m = 1 << i
            out.append(((c & ~m, r, t), b_c, "weather") if c & m else ((c | m, r, t), 1, "weather"))
            if fuel & m:
                out.append(((c & ~m, r, t), b_h, "fuel") if c & m else ((c | m, r, t), 1, "fuel"))
        return out

    def two_step(self, c, x):
        near = [v for v in range(self.n) if v == x or c & self.bit(x, v)]
        return sum(1 + sum(1 for w in range(self.n) if w != v and c & self.bit(v, w)) for v in near)


try:
    import flint
except ImportError:
    flint = None


def solve_exact(a, rhs):
    """Exact solution of a x = rhs over the rationals."""
    n = len(a)
    if flint is not None:
        x = flint.fmpq_mat(n, n, [v for row in a for v in row]).solve(flint.fmpq_mat(n, 1, rhs))
        return [F(int(x[i, 0].p), int(x[i, 0].q)) for i in range(n)]
    prev = 1
    for k in range(n):
        piv = next(r for r in range(k, n) if a[r][k] != 0)
        if piv != k:
            a[k], a[piv] = a[piv], a[k]
            rhs[k], rhs[piv] = rhs[piv], rhs[k]
        akk, rowk, bk = a[k][k], a[k], rhs[k]
        for r in range(k + 1, n):
            row, ark = a[r], a[r][k]
            if ark == 0:
                for col in range(k + 1, n):
                    if row[col]:
                        row[col] = akk * row[col] // prev
                rhs[r] = akk * rhs[r] // prev
            else:
                for col in range(k + 1, n):
                    row[col] = (akk * row[col] - ark * rowk[col]) // prev
                rhs[r] = (akk * rhs[r] - ark * bk) // prev
            row[k] = 0
        prev = akk
    x = [F(0)] * n
    for k in range(n - 1, -1, -1):
        s = F(rhs[k]) - sum(a[k][col] * x[col] for col in range(k + 1, n) if a[k][col])
        x[k] = s / a[k][k]
    return x


def solve_orbits(world, agent, b_c, b_h):
    n = len(world.orbits)
    a = [[0] * n for _ in range(n)]
    for o, members in enumerate(world.orbits):
        z = members[0]
        for dest, rate, _ in world.channels(z, agent, b_c, b_h):
            d = world.orbit_of[dest]
            a[d][o] += rate
            a[o][o] -= rate
    a[n - 1] = [1] * n
    x = solve_exact(a, [0] * (n - 1) + [1])
    lcm = 1
    for v in x:
        lcm = lcm * v.denominator // math.gcd(lcm, v.denominator)
    totals = [int(v * lcm) for v in x]
    if lcm < 0 or any(v <= 0 for v in totals):
        raise SystemExit("non-positive orbit weight")
    fact = math.factorial(world.n - 1)
    weight = {}
    for o, members in enumerate(world.orbits):
        assert fact % len(members) == 0
        for z in members:
            weight[z] = totals[o] * (fact // len(members))
    return weight


def measure(world, agent, b_c, b_h):
    w = solve_orbits(world, agent, b_c, b_h)
    balance = {z: 0 for z in world.states}
    fuel = 0
    for z in world.states:
        for dest, rate, cls in world.channels(z, agent, b_c, b_h):
            flow = w[z] * rate
            balance[z] -= flow
            balance[dest] += flow
            if cls == "fuel":
                fuel += flow if dest[0] > z[0] else -flow
    if any(balance.values()):
        raise SystemExit(f"lifted law does not balance: {world.n, world.tau, agent}")
    total = sum(w.values())
    held = F(sum(w[z] * world.two_step(z[0], world.walk(z[1])[z[2]]) for z in world.states), total)
    bridges = F(sum(w[z] * bin(z[0]).count("1") for z in world.states), total)
    r0 = F(1, 1 + b_c)
    phi = lambda r: 1 + 3 * (world.n - 1) * r + (world.n - 1) * (world.n - 2) * r * r
    h = held - phi(r0)
    j = F(fuel, total)
    assert j == (b_c + 1) * bridges - world.k, "J identity"
    rho = bridges / world.k
    return h, j, h / (phi(rho) - phi(r0))


def main():
    report = {}
    for line in open(sys.argv[1]):
        if line.startswith("WORLD "):
            parts = line.split()
            h = F(re.search(r" H=(\S+)", line).group(1))
            j = F(re.search(r" J=(\S+)", line).group(1))
            report[(parts[1], parts[2])] = (h, j)
    edge, agree, total = {}, 0, 0
    for b_c, b_h in PAIRS:
        for n, tau in WORLDS:
            world = World(n, tau)
            for agent in agents(tau):
                h, j, e = measure(world, agent, b_c, b_h)
                key = (f"({n},{tau},{b_c},{b_h})", agent)
                same = report.get(key) == (h, j)
                agree += same
                total += 1
                edge[(n, tau, b_c, b_h, agent)] = (h, e)
                print(f"{key[0]} {agent:6} orbits={len(world.orbits):3} H~{float(h):.4f} J~{float(j):.4f} "
                      f"E~{float(e):.4f} {'agrees' if same else 'DISAGREES'}", flush=True)
    print(f"H and J agree with the frozen report in {agree} of {total} steady states")
    worlds = [(n, tau, b_c, b_h) for b_c, b_h in PAIRS for n, tau in WORLDS]
    verdict = lambda fails: "SUPPORTED failing=none" if not fails else f"REFUTED failing={fails}"
    h0 = [(w, a) for w in worlds for a in agents(w[1])[:-1] if edge[(*w, a)][0] <= 0]
    h1 = [w for w in worlds if edge[(*w, "LOCAL")][1] <= 1]
    ladders = [w for w in worlds if w[1] >= 2]
    h2 = [w for w in ladders if not all(edge[(*w, f"AIM-{k}")][1] < edge[(*w, f"AIM-{k + 1}")][1] for k in range(1, w[1]))]
    h3 = [w for w in ladders if not edge[(*w, f"AIM-{w[1]}")][1] > edge[(*w, "LOCAL")][1]]
    h4 = [p for p in PAIRS if not edge[(3, 1, *p, "LOCAL")][1] < edge[(4, 1, *p, "LOCAL")][1] < edge[(5, 1, *p, "LOCAL")][1]]
    for name, fails in (("H0", h0), ("H1", h1), ("H2", h2), ("H3", h3), ("H4", h4)):
        print(name, verdict(fails))
    sys.exit(0 if agree == total else 1)


if __name__ == "__main__":
    main()
