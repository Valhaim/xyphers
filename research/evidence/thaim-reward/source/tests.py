"""Unit tests on NON-declared worlds only (boundary section 12): N = 2, N = 3, and
the N = 4, tau = 2 pilot worlds. No declared world (N = 4, tau in {3, 4}) is built here.

Run:  python tests.py     (needs numpy, scipy, python-flint)
"""

import math
import sys
from fractions import Fraction as Q

import numpy as np

import exact as X
import thaimworld as T

FAIL = []


def check(name, ok):
    print(("PASS " if ok else "FAIL ") + name)
    if not ok:
        FAIL.append(name)


def test_float_exact_agree():
    w = T.World(3, 2, 3, 2, 1)
    rng = np.random.default_rng(7)
    u = T.freeze(rng.uniform(0, T.UMAX, len(w.pa)))
    r = X.evaluate(w, u)
    p_float, _ = w.solve(np.array([float(x) for x in u]))
    err = max(abs(float(r["p"][s]) - p_float[s]) for s in range(w.n))
    check("N=3 tau=2: float and exact steady states agree within 1e-12", err < 1e-12)
    check("N=3 tau=2: exact steady state is stationary", r["stationary"])
    check("N=3 tau=2: J identity", r["J_identity"])
    check("N=3 tau=2: THAIM balance S_A + S_U = 0 exactly", r["balance_zero"])
    val, _, _ = w.reward(p_float, np.array([float(x) for x in u]), "net")
    check("N=3 tau=2: float R_net matches certified exact R_net within 1e-10",
          abs(val - float(r["R_net"].mid())) < 1e-10)
    val, _, _ = w.reward(p_float, np.array([float(x) for x in u]), "receipt")
    check("N=3 tau=2: float R_receipt matches certified exact R_receipt within 1e-10",
          abs(val - float(r["R_receipt"].mid())) < 1e-10)


def test_price_calibration():
    for tau, bc, bh, v in ((3, 5, 2, 1), (4, 20, 3, 1), (2, 3, 2, 2)):
        w = T.World(2, tau, bc, bh, v)
        r = X.evaluate(w, [Q(1)] * len(w.pa))
        expect = X.LogLin().add_log(r["J"] * tau, 2) - X.LogLin().add_log(r["J"], bc)
        check(f"N=2 tau={tau} b_c={bc}: R_net ln b_c = J (tau ln 2 - ln b_c) exactly",
              (r["R_net_times_ln"] - expect).is_zero())
        wrong = X.evaluate(w, [Q(1)] * len(w.pa), alpha_from=bh)
        check(f"N=2 tau={tau}: hot-store price is detected", not (wrong["R_net_times_ln"] - expect).is_zero())
        sign_ok = (r["R_net"] > 0) == (tau * math.log(2) > math.log(bc))
        check(f"N=2 tau={tau} b_c={bc}: sign of R_net is sign(tau ln 2 - ln b_c)", sign_ok)


def test_global_calibration():
    w = T.World(4, 2, 3, 2, 1)
    u = [Q(1)] * len(w.pa)
    r = X.evaluate(w, u)
    rho = Q(2, 2 + w.bc + w.bh)
    indep = all(r["p"][s] == rho ** w.size[s] * (1 - rho) ** (w.K - w.size[s]) / w.N for s in range(w.n))
    check("N=4 tau=2 (pilot world): GLOBAL bridges independent, traveller uniform", indep)
    check("N=4 tau=2 (pilot world): GLOBAL edge E = 1 exactly", r["E"] == 1)


def test_training_pilot():
    w = T.World(4, 2, 3, 2, 1)
    import run as R
    u, key, _ = T.train_tabular(w, "full", "net", T=1500, lr0=0.05)
    r = X.evaluate(w, T.freeze(u))
    check("N=4 tau=2 (pilot world): tabular NET-full learns the rule (agreement >= 0.98)", r["agreement"] >= Q(98, 100))
    u2, _, _ = T.train_tabular(w, "full", "net", T=1500, lr0=0.05)
    t1 = R.table_text(R.table_from_pairs(w, "full", T.freeze(u)))
    t2 = R.table_text(R.table_from_pairs(w, "full", T.freeze(u2)))
    check("N=4 tau=2 (pilot world): tabular training is deterministic", t1 == t2)
    uw, _, _ = T.train_tabular(w, "full", "world", T=1500, lr0=0.05)
    rw = X.evaluate(w, T.freeze(uw))
    rl = X.evaluate(w, [Q(x) for x in T.policy_local(w)])
    check("N=4 tau=2 (pilot world): WORLD-full idles (J <= J_LOCAL/20)", rw["J"] <= rl["J"] / 20)


def test_controls_pilot():
    import run as R
    w = T.World(4, 2, 3, 2, 1)
    for name, expect, got in R.run_controls(4, 2, 3, 2, 1, R.frozen_from_world(w)):
        check(f"pilot witness N=4 tau=2 (3,2,1): {name} fails first at {expect}", got == expect)


def test_pipeline_N3():
    import run as R
    w = T.World(3, 3, 9, 2, 1)
    agents = R.build_agents(w)
    for name in R.AGENTS:
        a = agents[name]
        trans = X.transitions(w, a["u"])
        ok = R.g01_undriven(w, trans) and R.g02_no_demon(w, trans, R.KEYCLASS[name], a["table"])
        ev = X.evaluate(w, a["u"])
        ok = ok and ev["stationary"] and ev["balance_zero"] and ev["J_identity"]
        back = R.pairs_from_table(w, R.KEYCLASS[name], R.read_table(R.table_text(a["table"])))
        ok = ok and back == a["u"]
        if name in R.TRAINED:
            ok = ok and a["text"] == a["text2"]
        check(f"N=3 tau=3 (9,2,1): pipeline gates hold for {name}", ok)
    check("N=3 tau=3 (9,2,1): ratio-table check passes on its own frozen record",
          R.ratio_table_checks(w, R.frozen_from_world(w)))


def test_gradient_fd():
    w = T.World(3, 3, 9, 2, 1)
    rng = np.random.default_rng(3)
    u = rng.uniform(0.2, 3.0, len(w.pa))
    for kind in ("net", "receipt", "world"):
        p, lu = w.solve(u)
        val, g, direct = w.reward(p, u, kind)
        grad = w.adjoint_du(p, lu, g, u) + direct
        worst = 0.0
        for i in rng.choice(len(u), 8, replace=False):
            h = 1e-6
            up, um = u.copy(), u.copy()
            up[i] += h
            um[i] -= h
            fp = w.reward(w.solve(up)[0], up, kind)[0]
            fm = w.reward(w.solve(um)[0], um, kind)[0]
            worst = max(worst, abs((fp - fm) / (2 * h) - grad[i]))
        check(f"N=3 tau=3: adjoint gradient of R_{kind} matches finite differences (max err {worst:.1e})", worst < 1e-6)


if __name__ == "__main__":
    test_float_exact_agree()
    test_price_calibration()
    test_global_calibration()
    test_training_pilot()
    test_controls_pilot()
    test_pipeline_N3()
    test_gradient_fd()
    print("failed:", len(FAIL))
    sys.exit(1 if FAIL else 0)
