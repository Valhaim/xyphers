"""CAL-THAIM-1: Can THAIM train an agent? Single entry point (boundary section 12).

    python run.py > report.txt

Trains every agent twice (G09), freezes the rates to multiples of 2^-24, evaluates
every frozen table exactly, runs gates G01-G13 in the declared order, the six
controls on their witnesses, and decides H1-H5 on exact values or on ln b_c-scaled
linear forms. No floating-point value decides a gate, a control, or a hypothesis.
"""

import hashlib
import os
import platform
import sys
from fractions import Fraction as Q

import flint
import numpy as np
import scipy

import exact as X
import thaimworld as T

WORLDS = [("W1", 3, 5, 2, 1), ("W2", 3, 5, 3, 4), ("W3", 3, 9, 2, 1),
          ("W4", 3, 20, 2, 1), ("W5", 4, 20, 3, 1), ("W6", 4, 60, 2, 2)]
WARM = ("W1", "W2", "W3", "W5")
COLD = ("W4", "W6")

# section 6, frozen before any declared world was solved
FROZEN = {
    "W1": dict(NT=Q(97, 18), r0=Q(1, 6), distinct=36, maxr=18, paying=96, ambiguous=0),
    "W2": dict(NT=Q(97, 18), r0=Q(1, 6), distinct=36, maxr=18, paying=96, ambiguous=0),
    "W3": dict(NT=Q(853, 250), r0=Q(1, 10), distinct=36, maxr=18, paying=72, ambiguous=12),
    "W4": dict(NT=Q(6388, 3087), r0=Q(1, 21), distinct=36, maxr=18, paying=0, ambiguous=0),
    "W5": dict(NT=Q(8156, 2401), r0=Q(1, 21), distinct=36, maxr=56, paying=72, ambiguous=12),
    "W6": dict(NT=Q(24466996, 13845841), r0=Q(1, 61), distinct=36, maxr=56, paying=0, ambiguous=0),
}
MULT = {
    3: {Q(1): 60, Q(13, 12): 24, Q(9, 8): 12, Q(32, 27): 24, Q(39, 32): 24, Q(21, 17): 24, Q(32, 25): 12, Q(9, 7): 24, Q(25, 19): 24, Q(4, 3): 24, Q(19, 14): 12, Q(18, 13): 24, Q(38, 27): 12, Q(13, 9): 12, Q(3, 2): 24, Q(32, 21): 24, Q(25, 16): 24, Q(27, 17): 12, Q(13, 8): 24, Q(64, 39): 12, Q(28, 17): 12, Q(7, 4): 24, Q(38, 21): 24, Q(50, 27): 12, Q(2): 24, Q(27, 13): 24, Q(17, 8): 24, Q(13, 6): 24, Q(9, 4): 24, Q(32, 13): 24, Q(21, 8): 24, Q(8): 24, Q(12): 24, Q(13): 24, Q(16): 12, Q(18): 12},
    4: {Q(1): 60, Q(34, 29): 24, Q(34, 27): 24, Q(14, 11): 12, Q(55, 41): 24, Q(139, 102): 24, Q(128, 89): 12, Q(81, 55): 24, Q(40, 27): 12, Q(89, 60): 24, Q(44, 29): 24, Q(30, 19): 12, Q(28, 17): 24, Q(139, 81): 12, Q(89, 51): 24, Q(29, 16): 24, Q(256, 139): 12, Q(76, 41): 12, Q(102, 55): 24, Q(55, 29): 24, Q(81, 41): 12, Q(17, 8): 24, Q(24, 11): 24, Q(178, 81): 12, Q(51, 22): 24, Q(81, 34): 24, Q(139, 56): 24, Q(41, 16): 24, Q(81, 29): 24, Q(3): 24, Q(55, 16): 24, Q(16): 24, Q(29): 24, Q(34): 24, Q(44): 12, Q(56): 12},
}
# receipt windows: build ratios r with theta_R < r < b_c, theta_R = b_c^((b_c-b_h)/(b_c+b_h u_max)) (section 2)
WINDOW = {
    "W1": [Q(3, 2), Q(32, 21), Q(25, 16), Q(27, 17), Q(13, 8), Q(64, 39), Q(28, 17), Q(7, 4), Q(38, 21), Q(50, 27), Q(2), Q(27, 13), Q(17, 8), Q(13, 6), Q(9, 4), Q(32, 13), Q(21, 8)],
    "W2": [Q(39, 32), Q(21, 17), Q(32, 25), Q(9, 7), Q(25, 19), Q(4, 3), Q(19, 14), Q(18, 13), Q(38, 27), Q(13, 9), Q(3, 2), Q(32, 21), Q(25, 16), Q(27, 17), Q(13, 8), Q(64, 39), Q(28, 17), Q(7, 4), Q(38, 21), Q(50, 27), Q(2), Q(27, 13), Q(17, 8), Q(13, 6), Q(9, 4), Q(32, 13), Q(21, 8)],
    "W3": [Q(21, 8), Q(8)],
    "W4": [Q(8), Q(12), Q(13), Q(16), Q(18)],
    "W5": [Q(16)],
    "W6": [Q(34), Q(44), Q(56)],
}
for _wn, _tau, *_ in WORLDS:
    FROZEN[_wn].update(mult=MULT[_tau], local_keys=144, window=WINDOW[_wn])

AGENTS = ["LOCAL", "GLOBAL", "ORACLE", "NET-blind", "NET-local", "NET-full", "RECEIPT-full",
          "WORLD-full", "NET-full c=3/4", "NET-full c=3/2", "NET-full init 1/4", "MLP-full s0", "MLP-full s1"]
TRAINED = {"NET-blind": ("blind", "net", 1.0, 1.0), "NET-local": ("local", "net", 1.0, 1.0),
           "NET-full": ("full", "net", 1.0, 1.0), "RECEIPT-full": ("full", "receipt", 1.0, 1.0),
           "WORLD-full": ("full", "world", 1.0, 1.0), "NET-full c=3/4": ("full", "net", 0.75, 1.0),
           "NET-full c=3/2": ("full", "net", 1.5, 1.0), "NET-full init 1/4": ("full", "net", 1.0, 0.25)}
KEYCLASS = {"LOCAL": "local", "GLOBAL": "blind", "ORACLE": "full", "MLP-full s0": "full", "MLP-full s1": "full"}
KEYCLASS.update({a: v[0] for a, v in TRAINED.items()})

OUT = []
TABLES = os.path.join(os.path.dirname(os.path.abspath(__file__)), "tables")


def say(s=""):
    OUT.append(s)
    print(s, flush=True)


def dec(x, d=6):
    if x is None:
        return "none"
    x = Q(x)
    sgn = "-" if x < 0 else ""
    x = abs(x)
    scaled = (x.numerator * 10 ** d * 2 + x.denominator) // (2 * x.denominator)
    return f"{sgn}{scaled // 10 ** d}.{scaled % 10 ** d:0{d}d}"


def enc(a):
    return a.str(8)


# ---------------------------------------------------------------- tables

def table_from_pairs(w, keyclass, u_pairs):
    key, m = w.keys(keyclass)
    tab = {}
    for i, k in enumerate(key):
        tab.setdefault(int(k), u_pairs[i])
        if tab[int(k)] != u_pairs[i]:
            raise AssertionError("rates differ within one key")
    return tab


def table_text(tab):
    return "".join(f"{k}\t{v.numerator}/{v.denominator}\n" for k, v in sorted(tab.items()))


def read_table(text):
    tab = {}
    for line in text.splitlines():
        k, v = line.split("\t")
        n, d = v.split("/")
        tab[int(k)] = Q(int(n), int(d))
    return tab


def pairs_from_table(w, keyclass, tab):
    key, _ = w.keys(keyclass)
    return [tab[int(k)] for k in key]


# ---------------------------------------------------------------- agents

def build_agents(w):
    """returns {agent: dict(u=frozen per-pair rates, float=float rates or None, tail, text, text2)}"""
    out = {}
    out["LOCAL"] = dict(u=[Q(x) for x in T.policy_local(w)], float=None)
    out["GLOBAL"] = dict(u=[Q(x) for x in T.policy_global(w)], float=None)
    out["ORACLE"] = dict(u=T.policy_oracle_exact(w), float=None)
    for name, (obs, kind, c, u0) in TRAINED.items():
        uf, _, tail = T.train_tabular(w, obs, kind, c=c, T=8000, lr0=0.2, u0=u0)
        uf2, _, _ = T.train_tabular(w, obs, kind, c=c, T=8000, lr0=0.2, u0=u0)
        fz, fz2 = T.freeze(uf), T.freeze(uf2)
        out[name] = dict(u=fz, float=uf, tail=tail,
                         text=table_text(table_from_pairs(w, obs, fz)),
                         text2=table_text(table_from_pairs(w, obs, fz2)))
    for s in (0, 1):
        uf, tail = T.train_mlp(w, "net", seed=s, T=8000)
        out[f"MLP-full s{s}"] = dict(u=T.freeze(uf), float=uf, tail=tail)
    for name, a in out.items():
        a["table"] = table_from_pairs(w, KEYCLASS[name], a["u"])
    return out


# ---------------------------------------------------------------- gates (per world, per agent)

def g01_undriven(w, trans):
    pic = {s: Q(1, w.bc ** w.size[s]) for s in range(w.n)}
    rate = {}
    for a, b, k, cls, i in trans:
        if cls in "WK":
            rate[(a, b, cls, i)] = k
    for (a, b, cls, i), k in rate.items():
        back = rate[(b, a, cls, i)]
        if pic[a] * k != pic[b] * back:
            return False
    net = {s: Q(0) for s in range(w.n)}
    for (a, b, cls, i), k in rate.items():
        net[a] -= pic[a] * k
        net[b] += pic[a] * k
    return all(v == 0 for v in net.values())


def g02_no_demon(w, trans, keyclass, tab):
    key, _ = w.keys(keyclass)
    fwd, back = {}, {}
    for a, b, k, cls, i in trans:
        if cls == "F":
            (fwd if a == w.pa[i] else back)[i] = k
    for i in range(len(w.pa)):
        u = fwd[i]
        if back[i] != w.bh * u or u != tab[int(key[i])]:
            return False
        if not (0 <= u <= T.UMAX) or (u * T.FREEZE_DENOM).denominator != 1:
            return False
    return True


def ratio_table_checks(w, frozen):
    ratios = set(w.ratio)
    paying = sum(1 for r in w.ratio if r > w.bc)
    mult = {}
    for r in w.ratio:
        mult[r] = mult.get(r, 0) + 1
    # local-ambiguity: local keys whose members disagree on paying
    key, m = w.keys("local")
    members = {}
    for i, k in enumerate(key):
        members.setdefault(int(k), set()).add(w.ratio[i] > w.bc)
    ambiguous = sum(1 for s in members.values() if len(s) == 2)
    return (len(ratios) == frozen["distinct"] and max(ratios) == frozen["maxr"]
            and paying == frozen["paying"] and Q(w.bc) not in ratios
            and mult == frozen["mult"] and m == frozen["local_keys"] and ambiguous == frozen["ambiguous"]
            and sorted(r for r in ratios if above_theta(r, w) and r < w.bc) == sorted(frozen["window"])
            and (frozen["ambiguous"] == 0
                 or {k for k, s in members.items() if len(s) == 2} == {k for k, s in members.items() if True in s}))


def above_theta(r, w):
    """exact test r > b_c^((b_c-b_h)/(b_c+b_h u_max)): r^den > b_c^num."""
    e = Q(w.bc - w.bh, w.bc + w.bh * T.UMAX)
    return r ** e.denominator > Q(w.bc) ** e.numerator


def g07_global(w, ev):
    rho = Q(2, 2 + w.bc + w.bh)
    p = ev["p"]
    indep = all(p[s] == rho ** w.size[s] * (1 - rho) ** (w.K - w.size[s]) / w.N for s in range(w.n))
    return indep and ev["E"] == 1


def g08_price(name, tau, bc, bh, v, alpha_from=None):
    w2 = T.World(2, tau, bc, bh, v)
    ev = X.evaluate(w2, [Q(1)] * len(w2.pa), alpha_from=alpha_from)
    expect = X.LogLin().add_log(ev["J"] * tau, 2) - X.LogLin().add_log(ev["J"], bc)
    return (ev["R_net_times_ln"] - expect).is_zero()


# ---------------------------------------------------------------- controls

def control_gates(w, tab_u, keyclass, demon=None, weather_reads_agent=False, receipts_as_net=False,
                  alpha_from=None, frozen=None, price_world=None):
    """run gates G01..G08 in order on a witness; return the first failing gate or None."""
    tab = table_from_pairs(w, keyclass, tab_u)
    trans = X.transitions(w, tab_u, demon=demon, weather_reads_agent=weather_reads_agent)
    if not g01_undriven(w, trans):
        return "G01"
    if not g02_no_demon(w, trans, keyclass, tab):
        return "G02"
    p = X.solve_exact(w.n, trans)
    if not X.is_stationary(w.n, trans, p):
        return "G03"
    ev = X.evaluate(w, tab_u)
    bal = (ev["receipt_flow"] + ev["S_U"]) if receipts_as_net else (ev["S_A"] + ev["S_U"])
    if not bal.is_zero():
        return "G04"
    if not (ev["J_identity"] and ev["sigma_Lc"] == ev["J"] and ev["sigma_Lh"] == -ev["J"] and w.bc > w.bh):
        return "G05"
    if not ratio_table_checks(w, frozen):
        return "G06"
    if not g07_global(w, X.evaluate(w, [Q(1)] * len(w.pa))):
        return "G07"
    tau, bc, bh, v = price_world
    if not g08_price("C", tau, bc, bh, v, alpha_from=alpha_from):
        return "G08"
    return None


def run_controls(N, tau, bc, bh, v, frozen):
    """the six controls of boundary section 8 on the witness world (N, tau, bc, bh, v) with the GLOBAL table."""
    w = T.World(N, tau, bc, bh, v)
    glob = [Q(1)] * len(w.pa)
    pw = (tau, bc, bh, v)
    return [
        ("C01 demon fuel", "G02", control_gates(w, glob, "blind", demon=0, frozen=frozen, price_world=pw)),
        ("C02 weather reads the agent", "G01", control_gates(w, glob, "blind", weather_reads_agent=True, frozen=frozen, price_world=pw)),
        ("C03 receipts booked as net flow", "G04", control_gates(w, glob, "blind", receipts_as_net=True, frozen=frozen, price_world=pw)),
        ("C04 price from the hot store", "G08", control_gates(w, glob, "blind", alpha_from=bh, frozen=frozen, price_world=pw)),
        ("C05 equal stores", "G05", control_gates(T.World(N, tau, bc, bc, v), glob, "blind", frozen=frozen, price_world=(tau, bc, bc, v))),
        ("C06 corrupted ratio table", "G06", control_gates(w, glob, "blind", frozen=dict(frozen, paying=frozen["paying"] + 1), price_world=pw)),
    ]


def frozen_from_world(w):
    """frozen record computed from a world itself; used only by tests on non-declared worlds."""
    from fractions import Fraction
    key, m = w.keys("local")
    members = {}
    for i, k in enumerate(key):
        members.setdefault(int(k), set()).add(w.ratio[i] > w.bc)
    mult = {}
    for r in w.ratio:
        mult[r] = mult.get(r, 0) + 1
    window = sorted(r for r in set(w.ratio) if above_theta(r, w) and r < w.bc)
    return dict(NT=X.NT_pic(w), r0=Fraction(1, 1 + w.bc), distinct=len(set(w.ratio)), maxr=max(w.ratio),
                paying=sum(1 for r in w.ratio if r > w.bc), ambiguous=sum(1 for s in members.values() if len(s) == 2),
                mult=mult, local_keys=m, window=window)


# ---------------------------------------------------------------- main

def main():
    say("CAL-THAIM-1: Can THAIM train an agent?")
    say(f"python {platform.python_version()}, numpy {np.__version__}, scipy {scipy.__version__}, "
        f"python-flint {flint.__version__}, platform {platform.system()} {platform.machine()}")
    say("training: tabular Adam, lr 0.2*0.1^(t/T), t=1..T, T=8000; MLP lr 0.03*0.1^(t/T), t=0..T-1, T=8000 (pilot schedule); "
        "rates frozen to multiples of 2^-24, ties to even")
    say()
    gates = {f"G{i:02d}": [] for i in range(1, 14)}   # list of (label, ok)
    results = {}
    for wname, tau, bc, bh, v in WORLDS:
        w = T.World(4, tau, bc, bh, v)
        say(f"WORLD {wname} N=4 tau={tau} b_c={bc} b_h={bh} v={v}")
        agents = build_agents(w)
        evs = {}
        for name in AGENTS:
            a = agents[name]
            trans = X.transitions(w, a["u"])
            gates["G01"].append((f"{wname} {name}", g01_undriven(w, trans)))
            gates["G02"].append((f"{wname} {name}", g02_no_demon(w, trans, KEYCLASS[name], a["table"])))
            ev = X.evaluate(w, a["u"])
            evs[name] = ev
            gates["G03"].append((f"{wname} {name}", ev["stationary"]))
            gates["G04"].append((f"{wname} {name}", ev["balance_zero"]))
            gates["G05"].append((f"{wname} {name}", ev["J_identity"] and ev["sigma_Lc"] == ev["J"]
                                 and ev["sigma_Lh"] == -ev["J"] and w.bc > w.bh))
            if name in TRAINED:
                h1 = hashlib.sha256(a["text"].encode()).hexdigest()
                h2 = hashlib.sha256(a["text2"].encode()).hexdigest()
                gates["G09"].append((f"{wname} {name}", h1 == h2))
            if a["float"] is not None:
                err = max(abs(Q(float(f)) - fz) for f, fz in zip(a["float"], a["u"]))
                gates["G10"].append((f"{wname} {name}", err <= Q(1, 1 << 25)))
            text = table_text(a["table"])
            path = os.path.join(TABLES, wname, name.replace(" ", "_").replace("/", "-") + ".txt")
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w", encoding="utf-8", newline="\n") as fh:
                fh.write(text)
            with open(path, encoding="utf-8") as fh:
                back = pairs_from_table(w, KEYCLASS[name], read_table(fh.read()))
            gates["G12"].append((f"{wname} {name}", back == a["u"]))
            digest = hashlib.sha256(text.encode()).hexdigest()
            residual = sum(ev["p"][aa] * uu - w.bh * uu * ev["p"][bb]
                           for aa, bb, uu in zip(w.pa, w.pb, a["u"]) if uu < Q(T.UMAX, 2))
            tail = a.get("tail")
            say(f"  {name:15s} key={KEYCLASS[name]:5s} J={dec(ev['J'])} residual_J={dec(residual)} H={dec(ev['H'])} "
                f"E={dec(ev['E'], 4)} aim={dec(ev['aim'], 4)} agreement={dec(ev['agreement'], 4)} "
                f"R_net={enc(ev['R_net'])} R_receipt={enc(ev['R_receipt'])} floor={enc(ev['floor'])} "
                f"eta={enc(ev['eta']) if ev['eta'] is not None else 'none'}"
                + (f" tail={tail:+.1e}" if tail is not None else "")
                + f" table_sha256={digest}")
        gates["G06"].append((wname, ratio_table_checks(w, FROZEN[wname])))
        gates["G07"].append((wname, g07_global(w, evs["GLOBAL"])))
        gates["G08"].append((wname, g08_price(wname, tau, bc, bh, v)))
        gates["G11"].append((wname, X.NT_pic(w) == FROZEN[wname]["NT"] and Q(1, 1 + bc) == FROZEN[wname]["r0"]))
        gates["G13"].append((wname, evs["WORLD-full"]["J"] <= evs["LOCAL"]["J"] / 20))
        # reported: receipt agreement with the isolated-channel threshold
        theta = bc ** ((bc - bh) / (bc + bh * T.UMAX))
        rec = evs["RECEIPT-full"]
        wsum = agr = Q(0)
        for i, (aa, bb) in enumerate(zip(w.pa, w.pb)):
            wt = rec["p"][aa] + rec["p"][bb]
            wsum += wt
            if (agents["RECEIPT-full"]["u"][i] >= Q(T.UMAX, 2)) == above_theta(w.ratio[i], w):
                agr += wt
        say(f"  reported: RECEIPT-full agreement with theta_R={theta:.4f}: {dec(agr / wsum, 4)}")
        results[wname] = (w, evs, agents)
        say()
    say("SLOT READING (G12): Graph Substrate = (G, pos); Environment = WEATHER, WALK (cold store); "
        "Praxion = fuel channels; Crystal = observation key, no memory (register empty); Thermo = stores b_c, b_h, "
        "alpha = 1/ln b_c, U = lambda|G|; Ruby empty, Opal absent. Rates read only the key; rewards enter training only; "
        "ORACLE's table is precomputed from section 6 combinatorics.")
    say()
    say("GATES")
    gate_ok = {}
    for g, checks in gates.items():
        ok = all(c for _, c in checks)
        gate_ok[g] = ok
        first = next((lbl for lbl, c in checks if not c), None)
        say(f"  {g} {'PASS' if ok else 'FAIL'} ({sum(c for _, c in checks)}/{len(checks)})"
            + (f" first failing check: {first}" if first else ""))
    first_gate = next((g for g in gates if not gate_ok[g]), None)
    say("  FIRST FAILURE: " + (f"{first_gate} {next(l for l, c in gates[first_gate] if not c)}" if first_gate else "none"))
    say()
    say("CONTROLS (witness W1 with the GLOBAL table; stop at first failing check)")
    ctl_ok = True
    for name, expect, got in run_controls(4, 3, 5, 2, 1, FROZEN["W1"]):
        ok = got == expect
        ctl_ok &= ok
        say(f"  {name}: failed first at {got} (expected {expect}) {'PASS' if ok else 'FAIL'}")
    say()
    say("HYPOTHESES")

    def rl(ev):
        return ev["R_net_times_ln"]

    verdicts = {}
    # H1
    fail = [wn for wn in results if results[wn][1]["NET-full"]["agreement"] < Q(98, 100)]
    verdicts["H1"] = fail
    # H2
    fail = []
    for wn in WARM:
        ev = results[wn][1]
        if not (rl(ev["ORACLE"]).sign() > 0 and rl(ev["NET-full"]).sign() > 0
                and (rl(ev["NET-full"]) - rl(ev["ORACLE"]).scale(Q(98, 100))).sign() >= 0):
            fail.append(wn)
    for wn in COLD:
        ev = results[wn][1]
        if not ev["NET-full"]["J"] <= ev["LOCAL"]["J"] / 20:
            fail.append(wn)
    verdicts["H2"] = fail
    # H3: receipts fuel builds that do not pay, and lose net THAIM by more than 1%
    fail, undecided3 = [], []
    for wn in WARM:
        w, ev, _ = results[wn]
        if rl(ev["NET-full"]).sign() <= 0:
            undecided3.append(wn)
            continue
        rec = ev["RECEIPT-full"]
        u_rec = results[wn][2]["RECEIPT-full"]["u"]
        wsum = bad = Q(0)
        for i, (aa, bb) in enumerate(zip(w.pa, w.pb)):
            wt = rec["p"][aa] + rec["p"][bb]
            wsum += wt
            if u_rec[i] >= Q(T.UMAX, 2) and not w.ratio[i] > w.bc:
                bad += wt
        margin = (rl(ev["NET-full"]).scale(Q(99, 100)) - rl(rec)).sign() > 0
        say(f"  H3 {wn}: fuelled non-paying weight={dec(bad / wsum, 4)}")
        if not (bad / wsum >= Q(1, 100) and margin):
            fail.append(wn)
    verdicts["H3"] = fail
    # H4
    verdicts["H4"] = [wn for wn in WARM
                      if not (results[wn][1]["NET-full"]["E"] is not None and results[wn][1]["LOCAL"]["E"] is not None
                              and results[wn][1]["NET-full"]["E"] > results[wn][1]["LOCAL"]["E"])]
    # H5
    fail, undecided = [], []
    for wn in ("W1", "W2", "W3", "W5"):
        ev = results[wn][1]
        if rl(ev["NET-full"]).sign() <= 0:
            undecided.append(wn)
            continue
        form = rl(ev["NET-full"]).scale(Q(99, 100)) - rl(ev["NET-local"])  # > 0  <=>  full - local > 0.01 full
        if wn in ("W3", "W5"):
            if not form.sign() > 0:
                fail.append(wn)
        else:
            if not form.sign() <= 0:
                fail.append(wn)
    verdicts["H5"] = fail
    ranges = {"H3": list(WARM), "H5": ["W1", "W2", "W3", "W5"]}
    for h in ("H1", "H2", "H3", "H4", "H5"):
        und = {"H3": undecided3, "H5": undecided}.get(h, [])
        extra = f" undecided={und}" if und else ""
        label = "REFUTED" if verdicts[h] else ("UNDECIDED" if h in ranges and sorted(und) == sorted(ranges[h]) else "SUPPORTED")
        say(f"  {h} {label} failing={verdicts[h] or 'none'}{extra}")
    say()
    overall = all(gate_ok.values()) and ctl_ok
    say(f"OVERALL {'PASS' if overall else 'FAIL'}")
    return 0 if overall else 1


if __name__ == "__main__":
    sys.exit(main())
