"""Design sanity check for THAIM reward forms, on pilot worlds only (N=4, R=2).

Rewards (energy per unit time, packet energy lambda = 1, alpha = 1/ln b_c):
  receipt : alpha * sum_builds u p(a) max(0, dS) - J      one-sided THAIM receipts
  net     : alpha * Sdot_A - J,  Sdot_A = - sum_U J_l dS_l  agent's own net THAIM flow
  world   : alpha * sum_all J_l dS_l - J = -J               world's net THAIM flow
with S = ln N_R(G, pos).
"""
import math
import os
import sys

import numpy as np

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../xypher-frontier-pilots/xypher-eval"))
import xeval as X  # noqa: E402


def setup(w):
    S = np.log(w.NR)
    w.S = S
    w.alpha = 1.0 / math.log(w.bc)
    # undriven flows: weather pairs (pa->pb build rate 1, wear bc) and walk pairs (rate v)
    w.dS_pair = S[w.pb] - S[w.pa]
    w.dS_walk = S[w.wb] - S[w.wa]


def thaim_terms(w, p, u):
    jw = p[w.pa] * 1.0 - p[w.pb] * w.bc
    jwalk = (p[w.wa] - p[w.wb]) * w.v
    S_U = float(jw @ w.dS_pair + jwalk @ w.dS_walk)
    jf = p[w.pa] * u - w.bh * p[w.pb] * u
    S_A = float(jf @ w.dS_pair)
    J = float(jf.sum())
    receipts = float((u * p[w.pa]) @ np.maximum(0.0, w.dS_pair))
    return dict(S_A=S_A, S_U=S_U, J=J, receipts=receipts)


def value_grad(w, p, u, kind):
    """value and (dF/dp, direct dF/du)."""
    a = w.alpha
    gJ = (w.bc + 1) * w.size.astype(float)
    J = (w.bc + 1) * float(w.size @ p) - w.K
    if kind == "net":
        # Sdot_A = -S_U, S_U depends on p only
        g = np.zeros(w.n)
        cw = w.dS_pair
        np.add.at(g, w.pa, -a * cw * 1.0)
        np.add.at(g, w.pb, a * cw * w.bc)
        cwalk = w.dS_walk * w.v
        np.add.at(g, w.wa, -a * cwalk)
        np.add.at(g, w.wb, a * cwalk)
        t = thaim_terms(w, p, u)
        return a * t["S_A"] - J, g - gJ, np.zeros_like(u)
    if kind == "receipt":
        pos = np.maximum(0.0, w.dS_pair)
        val = a * float((u * p[w.pa]) @ pos) - J
        g = np.zeros(w.n)
        np.add.at(g, w.pa, a * u * pos)
        return val, g - gJ, a * p[w.pa] * pos
    if kind == "world":
        return -J, -gJ, np.zeros_like(u)
    raise ValueError(kind)


def train(w, obs, kind, seed=0, hidden=8, umax=4.0, iters=800, lr=0.03):
    rng = np.random.default_rng(seed)
    pol = X.Policy(w, obs, hidden, umax, rng, 0.3, 0.1)
    for it in range(iters):
        u, cache = pol.rates()
        p, lu = w.solve(u)
        val, g, direct = value_grad(w, p, u, kind)
        du = w.adjoint_du(p, lu, g, u) + direct
        grads = pol.grad_from_du(du, u, cache)
        pol.net.adam_ascend(grads, lr * 0.1 ** (it / iters))
    u, _ = pol.rates()
    p, _ = w.solve(u)
    return u, p


def report(w, name, u, p):
    m = w.measures(p, u)
    t = thaim_terms(w, p, u)
    val_net = w.alpha * t["S_A"] - t["J"]
    print(f"  {name:22s} J={m['J']:.4f} H={m['H']:+.4f} E={m['E']:.4f} eta={m['eta']:.3f} aim={m['aim']:.3f} "
          f"u_inc={m['u_inc']:.2f} u_non={m['u_non']:.2f} | netTHAIM={val_net:+.5f} receipts={w.alpha*t['receipts']:.4f} "
          f"S_A+S_U={t['S_A']+t['S_U']:+.1e}")


if __name__ == "__main__":
    for bc, bh, v in ((8, 2, 1), (3, 2, 1), (3, 2, 4)):
        w = X.World(4, bc, bh, v, R=2)
        setup(w)
        print(f"world N=4 R=2 b_c={bc} b_h={bh} v={v}; ln b_c={math.log(bc):.4f}; max dS={w.dS_pair.max():.4f}; "
              f"share of builds with dS>ln b_c={float(np.mean(w.dS_pair > math.log(bc))):.3f}")
        report(w, "LOCAL u=1", w.u_local(1.0), w.solve(w.u_local(1.0))[0])
        report(w, "GLOBAL u=1", w.u_global(1.0), w.solve(w.u_global(1.0))[0])
        for obs in ("blind", "local", "full"):
            for kind in ("receipt", "net", "world"):
                u, p = train(w, obs, kind)
                report(w, f"{kind}/{obs}", u, p)
        sys.stdout.flush()
