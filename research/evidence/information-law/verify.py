"""Verification of the information law (Information Law note, sections 3-6).

Run:  python verify.py > verification.txt
Needs python-flint (tested with 0.6.0). Deterministic: fixed seeds, exact
rationals, certified arb enclosures; no floating-point value decides a check.
"""

import random
from fractions import Fraction as Q

import flint

from infolaw import LogLin, System, arbq, theorem_checks

KAPPAS = [1, 10, 100, 1000, 10**4, 10**6]
FAILURES = []


def check(name, ok):
    if not ok:
        FAILURES.append(name)
    return "PASS" if ok else "FAIL"


def fmt(x, d=12):
    return x.str(d, radius=False)


def lt0(x):
    return bool(x < 0)


# ---------------------------------------------------------------- 1. random ensembles

def random_system(rng, memory="general", act="general"):
    nx, nm = rng.randint(2, 5), rng.randint(2, 4)
    X, M = list(range(nx)), list(range(nm))
    g = {x: rng.randint(1, 7) for x in X}
    pairs = [(i, i + 1) for i in range(nx - 1)]
    pairs += [(a, b) for a in X for b in X if b > a + 1 and rng.random() < 0.4]
    undriven = []
    for a, b in pairs:
        for _ in range(1 + (rng.random() < 0.25)):  # sometimes two parallel undriven channels
            kap = rng.randint(1, 4)
            undriven.append((a, b, kap * g[b], kap * g[a]))
    world_pairs = [(a, b) for a in X for b in X if a < b]
    acts = []
    if act == "unused":
        chosen = [(a, b, rng.randint(1, 9), rng.randint(1, 9)) for a, b in world_pairs if rng.random() < 0.5]
        for m in M:
            acts += [(m, a, b, u, v) for a, b, u, v in chosen]
    else:
        for m in M:
            acts += [(m, a, b, rng.randint(1, 9), rng.randint(1, 9)) for a, b in world_pairs if rng.random() < 0.5]
    register = []
    if memory == "blind":
        rr = [(rng.randint(1, 9), rng.randint(1, 9)) for _ in range(nm - 1)]
        for x in X:
            register += [(x, i, i + 1, rr[i][0], rr[i][1]) for i in range(nm - 1)]
    else:
        for x in X:
            register += [(x, i, i + 1, rng.randint(1, 9), rng.randint(1, 9)) for i in range(nm - 1)]
            register += [(x, 0, nm - 1, rng.randint(1, 9), rng.randint(1, 9))] if nm > 2 and rng.random() < 0.3 else []
    return System(X, M, undriven, acts, register, g)


def phi_A(S):
    """Phi_A, one term per m-independent act channel family (a family is present in every register state)."""
    multiplicity, form = {}, LogLin()
    for cls, z, z2, k, k2 in S.channels:
        if cls == "A":
            key = (z[0], z2[0], k, k2)
            multiplicity[key] = multiplicity.get(key, 0) + 1
    for (x, x2, k, k2), count in multiplicity.items():
        if count % len(S.M):
            raise ValueError("act channel family not present in every register state")
        j = S.p[x] * k - S.p[x2] * k2
        form.add_log(j * (count // len(S.M)), k.numerator * k2.denominator * S.nx[x], k2.numerator * k.denominator * S.nx[x2])
    return form


def section_random():
    print("== 1. Random worlds ==")
    print("Each world: 2-5 world states, 2-4 register states, random detailed-balance")
    print("undriven channels (some parallel), random act and register channels.")
    print()
    for label, seed, n, memory, act in (
        ("general", 20261011, 300, "general", "general"),
        ("blind memory (register rates do not read x)", 20261012, 100, "blind", "general"),
        ("unused memory (act rates do not read m)", 20261013, 100, "general", "unused"),
    ):
        rng = random.Random(seed)
        tally = {}
        below = 0
        biggest_saving = None
        states = 0
        for _ in range(n):
            S = random_system(rng, memory, act)
            states += len(S.states)
            res = theorem_checks(S)
            L = S.L
            if memory == "blind":
                S.logsum_gaps("R")
                res["(V4) I_learn <= 0 certified"] = True
            if act == "unused":
                res["(V5) sigma_world = floor + Phi_A"] = (L["sigma_world"] - (L["floor"] + phi_A(S))).is_zero()
            for k, v in res.items():
                tally[k] = tally.get(k, 0) + bool(v)
            gap = (L["sigma_world"] - L["floor"]).enclose()
            if lt0(gap):
                below += 1
                saving = -gap
                if biggest_saving is None or saving.mid() > biggest_saving[0].mid():
                    biggest_saving = (saving, L["I_act"].enclose())
            elif not gap >= 0:
                raise AssertionError("sign of sigma_world - floor not certified")
        print("-- %s: %d worlds, %d system states, seed %d" % (label, n, states, seed))
        for k, v in tally.items():
            print("   %-44s %3d/%d  %s" % (k, v, n, check("%s %s" % (label, k), v == n)))
        print("   world fuel certified below its floor:       %3d/%d" % (below, n))
        if memory == "blind" or act == "unused":
            print("   corollary: never below the floor            %s" % check(label + " floor", below == 0))
        elif biggest_saving:
            s, i = biggest_saving
            print("   largest saving floor - sigma_world = %s with I_act = %s" % (fmt(s, 8), fmt(i, 8)))
        print()


# ---------------------------------------------------------------- tight construction

def build_tight(X, M, undriven, pi_c, Pstar, act_currents, register_tree, kappa):
    """rates that hold Pstar exactly, carry act_currents [(m, x, x2, j)], and dissipate
    at most sum j^2 / kappa in the act and register channels (note, section 5)."""
    out = {(x, m): Q(0) for x in X for m in M}
    for x, x2, w, w2 in undriven:
        for m in M:
            j = Pstar[(x, m)] * w - Pstar[(x2, m)] * w2
            out[(x, m)] += j
            out[(x2, m)] -= j
    for m, x, x2, j in act_currents:
        out[(x, m)] += j
        out[(x2, m)] -= j
    reg_currents = []
    for x in X:
        if sum(out[(x, m)] for m in M) != 0:
            raise ValueError("act currents do not conserve the world marginal at %r" % (x,))
        edges = register_tree[x]
        adj = {m: [] for m in M}
        for m, m2 in edges:
            adj[m].append(m2)
            adj[m2].append(m)
        root = M[0]
        order, parent, seen = [], {root: None}, {root}
        stack = [root]
        while stack:
            v = stack.pop()
            order.append(v)
            for u in adj[v]:
                if u not in seen:
                    seen.add(u)
                    parent[u] = v
                    stack.append(u)
        if len(seen) != len(M):
            raise ValueError("register tree does not span M")
        resid = {m: out[(x, m)] for m in M}
        for v in reversed(order):
            if parent[v] is None:
                continue
            j = resid[v]  # current from parent to v: an inflow at v that cancels v's net outflow
            reg_currents.append((x, parent[v], v, j))
            resid[parent[v]] += j
            resid[v] -= j
        assert all(resid[m] == 0 for m in M)
    k = Q(kappa)

    def rates(za, zb, j):
        return (k + max(j, Q(0))) / Pstar[za], (k + max(-j, Q(0))) / Pstar[zb]

    acts = [(m, x, x2) + rates((x, m), (x2, m), j) for m, x, x2, j in act_currents]
    regs = [(x, m, m2) + rates((x, m), (x, m2), j) for x, m, m2, j in reg_currents]
    S = System(X, M, undriven, acts, regs, pi_c)
    S.solve()
    if S.P != Pstar:
        raise AssertionError("tight construction does not hold Pstar")
    sq = sum(j * j for *_, j in act_currents) + sum(j * j for *_, j in reg_currents)
    sqA = sum(j * j for *_, j in act_currents)
    return S, sqA / k, sq / k


def kappa_sweep(title, make, closed=None):
    print("   kappa   sigma_world                floor - sigma_world        (floor - sigma_world)/I_act   S_A <= sum j_A^2/kappa")
    last = None
    for kappa in KAPPAS:
        S, boundA, bound = make(kappa)
        res = theorem_checks(S)
        for k_, v in res.items():
            check("%s kappa=%d %s" % (title, kappa, k_), v)
        L = S.L
        sw = L["sigma_world"].enclose()
        saving = (L["floor"] - L["sigma_world"]).enclose()
        ratio = saving / L["I_act"].enclose()
        SA = L["S_A"].enclose()
        SR = L["S_R"].enclose()
        okA = bool(SA <= arbq(boundA)) and bool(SA + SR <= arbq(bound))
        check("%s kappa=%d dissipation bound" % (title, kappa), okA)
        print("   %-7s %-26s %-26s %-29s %s" % (kappa, fmt(sw, 10), fmt(saving, 10), fmt(ratio, 10), "PASS" if okA else "FAIL"))
        last = S
    return last


# ---------------------------------------------------------------- 2. the demon world

def section_demon():
    print("== 2. The demon world: closed form and the exchange rate ==")
    p0, p1, a = Q(3, 4), Q(1, 4), Q(9, 10)
    X, M = [0, 1], [0, 1]
    undriven = [(0, 1, 1, 1)]
    pi_c = {0: 1, 1: 1}
    Pstar = {(x, m): (p0 if x == 0 else p1) * (a if m == x else 1 - a) for x in X for m in M}
    j = p0 - p1
    print("World {0,1}, undriven flips at rate 1 (pi_c uniform); register copies x with")
    print("accuracy a = %s; held law p = (%s, %s); one act channel, in register state 1," % (a, p0, p1))
    print("moves x from 1 to 0 and carries the whole holding current %s." % j)

    def make(kappa):
        return build_tight(X, M, undriven, pi_c, Pstar, [(1, 1, 0, j)], {0: [(0, 1)], 1: [(0, 1)]}, kappa)

    S = kappa_sweep("demon", make)
    L = S.L
    lr = lambda c, r: LogLin().add_log(c, r.numerator, r.denominator)
    odds = a / (1 - a)
    cf = {
        "floor": lr(p0 - p1, p0 / p1),
        "I_act": lr(p0 - p1, odds),
        "I_noise": lr(2 * a - 1, odds),
        "I_learn": lr(p0 - p1 + 2 * a - 1, odds),
    }
    print("   closed forms, decided exactly (kappa-independent):")
    print("     floor   = (p0 - p1) ln(p0/p1)              %s  %s" % (fmt(cf["floor"].enclose(), 10), check("demon floor closed form", (L["floor"] - cf["floor"]).is_zero())))
    print("     I_act   = (p0 - p1) ln(a/(1-a))            %s  %s" % (fmt(cf["I_act"].enclose(), 10), check("demon I_act closed form", (L["I_act"] - cf["I_act"]).is_zero())))
    print("     I_noise = (2a - 1) ln(a/(1-a))              %s  %s" % (fmt(cf["I_noise"].enclose(), 10), check("demon I_noise closed form", (L["I_noise"] - cf["I_noise"]).is_zero())))
    print("     I_learn = (p0 - p1 + 2a - 1) ln(a/(1-a))    %s  %s" % (fmt(cf["I_learn"].enclose(), 10), check("demon I_learn closed form", (L["I_learn"] - cf["I_learn"]).is_zero())))
    lim_world = cf["floor"] - cf["I_act"]
    lim_total = cf["floor"] + cf["I_noise"]
    print("   limits as kappa grows: sigma_world -> floor - I_act = %s (the act channel returns fuel)" % fmt(lim_world.enclose(), 10))
    print("                          sigma_reg   -> I_learn       = %s" % fmt(cf["I_learn"].enclose(), 10))
    print("                          total       -> floor + I_noise = %s" % fmt(lim_total.enclose(), 10))
    print()


# ---------------------------------------------------------------- 3. archipelago worlds

BRIDGES = ["01", "02", "12"]
HOME = 0b011


def popcount(g):
    return bin(g).count("1")


def archipelago_undriven(bc):
    K = len(BRIDGES)
    configs = list(range(1 << K))
    undriven = []
    for g in configs:
        for e in range(K):
            if not g >> e & 1:
                undriven.append((g, g | 1 << e, 1, bc))
    pi_c = {g: bc ** (K - popcount(g)) for g in configs}
    return configs, undriven, pi_c


def section_archipelago_tight():
    print("== 3. Archipelago, three islands: the exchange rate reached in a Xypher world ==")
    bc, t, a = 3, 2, Q(4, 5)
    configs, undriven, pi_c = archipelago_undriven(bc)
    K = len(BRIDGES)
    w = {g: Q(t ** popcount(g & HOME) * bc ** (K - popcount(g))) for g in configs}
    Z = sum(w.values())
    p = {g: w[g] / Z for g in configs}
    M = [0, 1, 2]
    cond = lambda m, g: a if m == popcount(g & HOME) else (1 - a) / 2
    Pstar = {(g, m): p[g] * cond(m, g) for g in configs for m in M}
    toggles = [(g, g | 1 << e) for g in configs for e in (0, 1) if not g >> e & 1]
    hold = {(g, g2): -(p[g] * 1 - p[g2] * bc) for g, g2 in toggles}
    print("Weather builds every bridge at rate 1 and wears it at rate b_c = %d (cold store)." % bc)
    print("Held world law p(G) ~ %d^h(G) b_c^-|G|, h = home bridges standing (islands 0-1, 0-2)." % t)
    print("Register: a sensor reading h(G) with accuracy %s. Act channels toggle home bridges." % a)
    print()
    print("-- blind agent (no register), act currents matched to cancel the weather (Horowitz-Zhou-England):")

    def make_blind(kappa):
        Pb = {(g, 0): p[g] for g in configs}
        return build_tight(configs, [0], undriven, pi_c, Pb,
                           [(0, g, g2, j) for (g, g2), j in hold.items()], {g: [] for g in configs}, kappa)

    for kappa in KAPPAS:
        S, bA, b = make_blind(kappa)
        res = theorem_checks(S)
        for k_, v in res.items():
            check("blind archipelago kappa=%d %s" % (kappa, k_), v)
    L = S.L
    print("   at kappa = 10^6: sigma_world = %s, floor = %s, I_act = %s" % (
        fmt(L["sigma_world"].enclose(), 10), fmt(L["floor"].enclose(), 10), fmt(L["I_act"].enclose(), 3)))
    print("   with no register there is no information (I_act = 0): the floor is its limit.")
    print()
    print("-- informed agent: each home build is carried in the register state that reads the")
    print("   pre-build count, so every build consumes ln(2a/(1-a)) = ln 8 of information.")

    def make(kappa):
        acts = [(popcount(g & HOME), g, g2, j) for (g, g2), j in hold.items()]
        return build_tight(configs, M, undriven, pi_c, Pstar, acts, {g: [(0, 1), (1, 2)] for g in configs}, kappa)

    S = kappa_sweep("archipelago", make)
    L = S.L
    print("   held p is the same in both agents; floor(p) = %s" % fmt(L["floor"].enclose(), 10))
    print("   I_act = %s, I_noise = %s, I_learn = %s" % (fmt(L["I_act"].enclose(), 10), fmt(L["I_noise"].enclose(), 10), fmt(L["I_learn"].enclose(), 10)))
    print("   as kappa grows: sigma_world -> floor - I_act = %s;  total -> floor + I_noise = %s" % (
        fmt((L["floor"] - L["I_act"]).enclose(), 10), fmt((L["floor"] + L["I_noise"]).enclose(), 10)))
    print()


def section_archipelago_natural():
    print("== 4. Archipelago, three islands: a sensing agent with fixed-force fuel ==")
    bc, bh, gamma = 3, 2, 4
    configs, undriven, pi_c = archipelago_undriven(bc)
    M = [0, 1, 2]
    register = []
    for g in configs:
        h = popcount(g & HOME)
        for m in (0, 1):
            up_is_toward = h >= m + 1
            register.append((g, m, m + 1, gamma if up_is_toward else 1, 1 if up_is_toward else gamma))
    toggles = [(g, g | 1 << e) for g in configs for e in (0, 1) if not g >> e & 1]
    print("Same weather (b_c = %d). Fuel from a hot store (b_h = %d): build a home bridge at rate 1," % (bc, bh))
    print("return it at rate b_h. Register: sensor of h(G), steps toward h at rate %d, away at rate 1." % gamma)
    print("Identity check only: these agents are not compared; that is the next experiment.")
    for label, M_, acts, reg in (
        ("LOCAL fuel, no register", [0], [(0, g, g2, 1, bh) for g, g2 in toggles], []),
        ("informed fuel, only while the sensor reads a home bridge down",
         M, [(m, g, g2, 1, bh) for m in (0, 1) for g, g2 in toggles], register),
    ):
        S = System(configs, M_, undriven, acts, reg, pi_c)
        S.solve()
        res = theorem_checks(S)
        for k_, v in res.items():
            check("natural %s %s" % (label, k_), v)
        L = S.L
        hot = sum(J for cls, z, z2, J in S.currents if cls == "A")
        flowA = LogLin().add_log(-hot, bh)  # each net build from the hot store: flow -ln b_h
        print("-- %s" % label)
        print("   theorem identities (1)-(4): %s" % ("PASS" if all(res.values()) else "FAIL"))
        print("   fuel flow J = %s (exact);  act-channel entropy flow = -J ln b_h: %s" % (hot, check("natural flow %s" % label, (L["flow_A"] - flowA).is_zero())))
        print("   sigma_world = J ln(b_c/b_h): %s" % check("natural two-store %s" % label, (L["sigma_world"] - LogLin().add_log(hot, bc, bh)).is_zero()))
        for key in ("sigma_world", "floor", "I_act", "I_noise", "I_learn", "S_A", "S_R"):
            print("   %-12s %s" % (key, fmt(L[key].enclose(), 10)))
    print()


if __name__ == "__main__":
    print("Information law verification")
    print("python-flint %s, arb precision %d bits" % (flint.__version__, flint.ctx.prec))
    print()
    section_random()
    section_demon()
    section_archipelago_tight()
    section_archipelago_natural()
    print("== Summary ==")
    print("checks failed: %d" % len(FAILURES))
    for f in FAILURES:
        print("  FAIL", f)
    print("OVERALL", "PASS" if not FAILURES else "FAIL")
