#!/usr/bin/env python3
"""Independent check of CAL-CEF-5 ("Can a theory predict the price of open future?").

Written from the boundary alone, sharing no code with the Rust apparatus. It
rebuilds every world of boundary sections 1 and 3 (speed v, measure radius R),
solves each steady state exactly on the orbits of the island relabellings that
fix home, checks the lifted law on the full chain, compares H and J with the
frozen report, and re-decides H1--H5 against the frozen theory predictions.

    python3 independent_check.py expected-report.txt predictions_cef5.txt

Standard library only; the largest worlds then take several minutes each and the
whole check takes one to two hours. If python-flint is installed, its exact
rational solver is used instead and the check takes a few minutes.
"""

import itertools
import math
import re
import sys
from fractions import Fraction as F

PAIRS = ((5, 2), (12, 3))
WORLDS = (((3, 2), 1, 2), ((3, 2), 4, 2), ((3, 2), 16, 2), ((4, 2), 1, 2), ((4, 2), 4, 2), ((4, 2), 16, 2),
          ((3, 1), 1, 2), ((4, 1), 1, 2), ((5, 1), 1, 2), ((3, 3), 1, 3))


def agents(tau):
    return ["LOCAL"] + [f"AIM-{k}" for k in range(1, tau + 1)] + ["GLOBAL"]


class World:
    def __init__(self, n, tau, v=1, radius=2):
        self.n, self.tau, self.v, self.radius = n, tau, v, radius
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
                out.append(((c, r, nt), self.v, "step"))
        if t == 0:
            out += [((c, q, 0), self.v, "replan") for q in self.routes if q != r]
        fuel = self.permitted(agent, r, t) if agent else 0
        for i in range(self.k):
            m = 1 << i
            out.append(((c & ~m, r, t), b_c, "weather") if c & m else ((c | m, r, t), 1, "weather"))
            if fuel & m:
                out.append(((c & ~m, r, t), b_h, "fuel") if c & m else ((c | m, r, t), 1, "fuel"))
        return out

    def walks(self, c, x):
        """Number of radius-step walks from x, each step a stay or a crossing of a standing bridge."""
        count = {x: 1}
        for _ in range(self.radius):
            nxt = {}
            for a, k in count.items():
                for b in range(self.n):
                    if b == a or c & self.bit(a, b):
                        nxt[b] = nxt.get(b, 0) + k
            count = nxt
        return sum(count.values())

    def phi(self, r):
        """Expected radius-step walks from home when every bridge stands independently with probability r."""
        total = F(0)
        for walk in itertools.product(range(self.n), repeat=self.radius):
            path = (0,) + walk
            used = {(min(a, b), max(a, b)) for a, b in zip(path, path[1:]) if a != b}
            total += r ** len(used)
        return total


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
    held = F(sum(w[z] * world.walks(z[0], world.walk(z[1])[z[2]]) for z in world.states), total)
    bridges = F(sum(w[z] * bin(z[0]).count("1") for z in world.states), total)
    r0 = F(1, 1 + b_c)
    h = held - world.phi(r0)
    j = F(fuel, total)
    assert j == (b_c + 1) * bridges - world.k, "J identity"
    rho = bridges / world.k
    return h, j, h / (world.phi(rho) - world.phi(r0)), rho


def label(n, tau, b_c, b_h, v, radius):
    return f"({n},{tau},{b_c},{b_h},v={v},R={radius})"


def main():
    report = {}
    for line in open(sys.argv[1]):
        if line.startswith("WORLD "):
            parts = line.split()
            h = F(re.search(r" H=(\S+)", line).group(1))
            j = F(re.search(r" J=(\S+)", line).group(1))
            report[(parts[1], parts[2])] = (h, j)
    theory = {}
    for line in open(sys.argv[2]):
        key, agent, value = line.split()
        theory[(key, agent)] = F(value)
    edge, agree, total = {}, 0, 0
    for b_c, b_h in PAIRS:
        for (n, tau), v, radius in WORLDS:
            world = World(n, tau, v, radius)
            for agent in agents(tau):
                h, j, e, rho = measure(world, agent, b_c, b_h)
                key = label(n, tau, b_c, b_h, v, radius)
                same = report.get((key, agent)) == (h, j)
                agree += same
                total += 1
                edge[(key, agent)] = (h, e, rho)
                print(f"{key} {agent:6} orbits={len(world.orbits):3} H~{float(h):.4f} J~{float(j):.4f} "
                      f"E~{float(e):.4f} {'agrees' if same else 'DISAGREES'}", flush=True)
    print(f"H and J agree with the frozen report in {agree} of {total} steady states")
    E = lambda key, agent: edge[(key, agent)][1]
    verdict = lambda fails: "SUPPORTED failing=none" if not fails else f"REFUTED failing={fails}"
    h1 = [k for k in theory if not F(99, 100) * theory[k] <= E(*k) <= F(101, 100) * theory[k]]
    h2, h3, h4, h5 = [], [], [], []
    for b_c, b_h in PAIRS:
        for n in (3, 4):
            keys = [label(n, 2, b_c, b_h, v, 2) for v in (1, 4, 16)]
            for agent in ("LOCAL", "AIM-1", "AIM-2"):
                if not E(keys[0], agent) > E(keys[1], agent) > E(keys[2], agent):
                    h2.append((keys[0], agent))
        for (n, tau), v, radius in WORLDS:
            key = label(n, tau, b_c, b_h, v, radius)
            if tau >= 2:
                ladder = [E(key, f"AIM-{k}") for k in range(1, tau + 1)]
                if not (all(a > b for a, b in zip(ladder, ladder[1:])) and ladder[-1] < E(key, "LOCAL")):
                    h3.append(key)
            if not E(key, "AIM-1") > E(key, "LOCAL"):
                h4.append(key)
            if tau == 1:
                r0, rho = F(1, 1 + b_c), edge[(key, "LOCAL")][2]
                law = F(n) * (3 + (n - 2) * r0) / (2 * (3 + (n - 2) * (rho + r0)))
                if not E(key, "LOCAL") <= law <= F(103, 100) * E(key, "LOCAL"):
                    h5.append(key)
    for name, fails in (("H1", h1), ("H2", h2), ("H3", h3), ("H4", h4), ("H5", h5)):
        print(name, verdict(fails))
    sys.exit(0 if agree == total else 1)


if __name__ == "__main__":
    main()
