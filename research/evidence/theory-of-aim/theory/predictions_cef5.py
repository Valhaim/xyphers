#!/usr/bin/env python3
"""Frozen CAL-CEF-5 predictions of the cavity theory (cavity.py), printed to six decimals."""
from cavity import solve

PAIRS = ((5, 2), (12, 3))
SPEED = [((3, 2), v) for v in (1, 4, 16)] + [((4, 2), v) for v in (1, 4, 16)]
SIZE = [((3, 1), 1), ((4, 1), 1), ((5, 1), 1)]

def worlds():
    for b_c, b_h in PAIRS:
        for (n, tau), v in SPEED + SIZE:
            yield n, tau, b_c, b_h, v, 2
        yield 3, 3, b_c, b_h, 1, 3

if __name__ == "__main__":
    for n, tau, b_c, b_h, v, radius in worlds():
        for agent in ["LOCAL"] + [f"AIM-{k}" for k in range(1, tau + 1)]:
            e = solve(n, tau, b_c, b_h, agent, v=v, radius=radius)["E"]
            print(f"({n},{tau},{b_c},{b_h},v={v},R={radius}) {agent} {e:.6f}")
