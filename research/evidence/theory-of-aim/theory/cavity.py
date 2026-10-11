#!/usr/bin/env python3
"""Cavity mean-field theory of the price of open future (CAL-CEF-4 worlds).

The world is the CAL-CEF-4 archipelago: islands 0..N-1, home 0, candidate
bridges all pairs; a traveller holds a route of tau steps from home as an
intention, walks it one step at a time (a step across a missing bridge is
blocked), and replans at home; a cold store builds and wears every bridge
(rates 1 and b_c); fuel from a hot store builds and returns the bridges in an
agent's permission set (rates 1 and b_h).

The theory keeps two things exact and approximates one:

* Each bridge is a two-state process whose rates depend only on the
  traveller's state, so, given the traveller's path, distinct bridges are
  independent and each one's probability of standing evolves exactly.
* For each bridge e, the joint process (traveller state, bridge e) is solved
  exactly as a Markov chain; a step across e needs e to stand.
* Approximation: a step across any other bridge e' happens at the step rate
  times the conditional mean m_e'(z) = P(e' stands | traveller at z), solved
  self-consistently (cavity mean field). The open future is evaluated with
  products of these conditional means.

There is no fitted parameter. Parameters beyond CAL-CEF-4:

* v       traveller speed: STEP and REPLAN rates are multiplied by v;
* R       measure radius: open future is the number of R-step walks from the
          traveller's island (CAL-CEF-4 used R = 2);
* replan  REPLAN rate per alternative route (before the factor v).

    python3 cavity.py validate ../../../website-xyphers/public/evidence/aimed-foresight/expected-report.txt
    python3 cavity.py scaling
"""

import itertools
import re
import sys
from fractions import Fraction

import numpy as np


def agents(tau):
    return ["LOCAL"] + [f"AIM-{k}" for k in range(1, tau + 1)] + ["GLOBAL"]


def open_future(n, x, radius, density, index):
    """Expected number of radius-step walks from x when bridge e stands with probability density[e], independently."""
    total = 0.0
    for walk in itertools.product(range(n), repeat=radius):
        path = (x,) + walk
        used = {index[(min(a, b), max(a, b))] for a, b in zip(path, path[1:]) if a != b}
        total += float(np.prod([density[e] for e in used])) if used else 1.0
    return total


def solve(n, tau, b_c, b_h, agent, v=1.0, radius=2, replan=1.0, tol=1e-13, max_iter=2000):
    bridges = list(itertools.combinations(range(n), 2))
    index = {b: i for i, b in enumerate(bridges)}
    k_bridges = len(bridges)
    routes = list(itertools.product(range(n), repeat=tau))
    states = [(r, t) for r in routes for t in range(tau + 1)]
    where = {z: i for i, z in enumerate(states)}
    m_states = len(states)

    def permitted(r, t):
        p = (0,) + r
        if agent == "GLOBAL":
            return set(range(k_bridges))
        if agent == "LOCAL":
            return {index[(min(p[t], y), max(p[t], y))] for y in range(n) if y != p[t]}
        depth = int(agent.split("-")[1])
        return {index[(min(p[s], p[s + 1]), max(p[s], p[s + 1]))]
                for s in range(max(0, t - depth), min(tau, t + depth)) if p[s] != p[s + 1]}

    fuel = np.zeros((k_bridges, m_states))
    for j, (r, t) in enumerate(states):
        for e in permitted(r, t):
            fuel[e, j] = 1.0
    moves = []
    for j, (r, t) in enumerate(states):
        p = (0,) + r
        out = []
        for nt in (t + 1, t - 1):
            if 0 <= nt <= tau:
                a, b = p[t], p[nt]
                out.append((where[(r, nt)], None if a == b else index[(min(a, b), max(a, b))], v))
        if t == 0:
            out += [(where[(q, 0)], None, v * replan) for q in routes if q != r]
        moves.append(out)

    r0 = 1.0 / (1 + b_c)
    mean = np.full((k_bridges, m_states), r0)
    occupancy = None
    for _ in range(max_iter):
        new = np.empty_like(mean)
        laws = []
        for e in range(k_bridges):
            q = np.zeros((2 * m_states, 2 * m_states))
            for j in range(m_states):
                for standing in (0, 1):
                    s = 2 * j + standing
                    for j2, e2, rate in moves[j]:
                        if e2 is None:
                            q[s, 2 * j2 + standing] += rate
                        elif e2 == e:
                            q[s, 2 * j2 + standing] += rate * standing
                        else:
                            q[s, 2 * j2 + standing] += rate * mean[e2, j]
                    if standing:
                        q[s, s - 1] += b_c + b_h * fuel[e, j]
                    else:
                        q[s, s + 1] += 1 + fuel[e, j]
            np.fill_diagonal(q, -q.sum(1))
            a = q.T.copy()
            a[-1, :] = 1.0
            rhs = np.zeros(2 * m_states)
            rhs[-1] = 1.0
            law = np.linalg.solve(a, rhs).reshape(m_states, 2)
            laws.append(law.sum(1))
            new[e] = law[:, 1] / law.sum(1)
        change = float(np.max(np.abs(new - mean)))
        mean = 0.5 * mean + 0.5 * new
        occupancy = np.mean(laws, axis=0)
        if change < tol:
            break
    held = sum(occupancy[j] * open_future(n, ((0,) + states[j][0])[states[j][1]], radius, mean[:, j], index)
               for j in range(m_states))
    cold = open_future(n, 0, radius, np.full(k_bridges, r0), index)
    flow = float(sum(occupancy[j] * fuel[e, j] * ((1 - mean[e, j]) - b_h * mean[e, j])
                     for e in range(k_bridges) for j in range(m_states)))
    rho = float(sum(occupancy[j] * mean[e, j] for e in range(k_bridges) for j in range(m_states)) / k_bridges)
    h = held - cold
    rival = open_future(n, 0, radius, np.full(k_bridges, rho), index) - cold
    return {"H": h, "J": flow, "rho": rho, "E": h / rival}


def static_law(n, rho, r0):
    """Closed form: equal-power edge of LOCAL at radius 2, all fuel on the traveller's bridges."""
    return n * (3 + (n - 2) * r0) / (2 * (3 + (n - 2) * (rho + r0)))


def validate(report):
    worst = 0.0
    for line in open(report):
        if not line.startswith("WORLD "):
            continue
        parts = line.split()
        n, tau, b_c, b_h = (int(x) for x in parts[1].strip("()").split(","))
        agent = parts[2]
        if agent == "GLOBAL":
            continue
        exact = float(Fraction(re.search(r" E=(\S+)", line).group(1)))
        theory = solve(n, tau, b_c, b_h, agent)["E"]
        error = theory / exact - 1
        worst = max(worst, abs(error))
        print(f"({n},{tau},{b_c},{b_h}) {agent:6} exact {exact:.4f} theory {theory:.4f} error {100 * error:+.2f}%")
    print(f"worst relative error {100 * worst:.2f}%")


def scaling():
    for b_c, b_h in ((8, 2), (3, 2)):
        r0 = 1 / (1 + b_c)
        for n in (3, 4, 5, 6, 8, 10, 14, 20):
            local = solve(n, 1, b_c, b_h, "LOCAL")
            print(f"({b_c},{b_h}) N={n:2d} E_LOCAL {local['E']:.3f}  E/N {local['E'] / n:.4f}  "
                  f"weak-drive law {n * (3 + (n - 2) * r0) / (2 * (3 + 2 * (n - 2) * r0)):.3f}")


if __name__ == "__main__":
    if sys.argv[1] == "validate":
        validate(sys.argv[2])
    elif sys.argv[1] == "scaling":
        scaling()
