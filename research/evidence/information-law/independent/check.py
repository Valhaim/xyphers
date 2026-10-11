#!/usr/bin/env python3
"""Independent exact check of the information law.

Source of every definition: research/physics/derivable/xypher-information-law.md,
sections 1, 2, 3 and 5.  Written without reading the author's code.

Method
  * Stationary laws: own Gauss-Jordan elimination over fractions.Fraction.
  * Every quantity is a finite sum  sum_c r_c ln(s_c)  with r_c, s_c rational.
    Each integer that enters a logarithm is fully factored into primes
    (flint.fmpz.factor, every factor re-proved prime with fmpz.is_prime and the
    product re-multiplied).  A quantity is stored as its exact coefficient
    vector over ln(prime).  An identity holds iff every prime's coefficient of
    (lhs - rhs) is exactly 0.  If all coefficients vanish the difference is 0
    whatever the primes are; if one does not vanish the difference is nonzero
    because logarithms of distinct primes are Q-linearly independent
    (unique factorisation).
  * Inequalities: exact rational sign arguments per term, or, for a nonzero
    coefficient vector, a certified arb enclosure whose precision is raised
    until it excludes 0 (this terminates because the value is nonzero).
  * No floating-point number decides anything.

Usage:  python3 check.py > output.txt   (Python 3 with python-flint 0.6.0)
"""

import random
import sys
from collections import OrderedDict
from fractions import Fraction as Fr

from flint import arb, ctx, fmpq, fmpz

SEED = 271828
ENCL_PREC = 256

# ---------------------------------------------------------------------------
# Exact logarithm algebra over primes
# ---------------------------------------------------------------------------

_FACTOR_CACHE = {}
STATS = {"ints_factored": 0, "max_digits": 0, "primes": set()}


def factor_int(n):
    """Full prime factorisation of a positive int, verified."""
    if n in _FACTOR_CACHE:
        return _FACTOR_CACHE[n]
    if n <= 0:
        raise ValueError("factor_int needs n > 0")
    if n == 1:
        res = ()
    else:
        fac = fmpz(n).factor()
        res = tuple((int(p), int(e)) for p, e in fac)
        prod = 1
        for p, e in res:
            if not fmpz(p).is_prime():
                raise RuntimeError("non-prime factor %d of %d" % (p, n))
            prod *= p ** e
        if prod != n:
            raise RuntimeError("factorisation of %d does not multiply back" % n)
        STATS["primes"].update(p for p, _ in res)
    STATS["ints_factored"] += 1
    STATS["max_digits"] = max(STATS["max_digits"], len(str(n)))
    _FACTOR_CACHE[n] = res
    return res


class LL:
    """Exact element of the Q-span of {ln p : p prime}: dict prime -> Fraction."""

    __slots__ = ("c",)

    def __init__(self, c=None):
        self.c = {} if c is None else c

    @staticmethod
    def ln(q):
        q = Fr(q)
        if q <= 0:
            raise ValueError("log of non-positive %s" % q)
        c = {}
        for p, e in factor_int(q.numerator):
            c[p] = c.get(p, 0) + e
        for p, e in factor_int(q.denominator):
            c[p] = c.get(p, 0) - e
        return LL({p: Fr(v) for p, v in c.items() if v != 0})

    def __add__(self, o):
        c = dict(self.c)
        for p, v in o.c.items():
            w = c.get(p, 0) + v
            if w == 0:
                c.pop(p, None)
            else:
                c[p] = w
        return LL(c)

    def __neg__(self):
        return LL({p: -v for p, v in self.c.items()})

    def __sub__(self, o):
        return self + (-o)

    def scale(self, r):
        r = Fr(r)
        if r == 0:
            return LL()
        return LL({p: v * r for p, v in self.c.items()})

    def is_zero(self):
        return all(v == 0 for v in self.c.values())

    def arb(self):
        s = arb(0)
        for p in sorted(self.c):
            v = self.c[p]
            s += arb(fmpq(v.numerator, v.denominator)) * arb(p).log()
        return s


def lsum(items):
    s = LL()
    for t in items:
        s = s + t
    return s


def sign_ll(x):
    """Exact sign of an LL: 0 by vanishing coefficients, else certified arb."""
    if x.is_zero():
        return 0
    old = ctx.prec
    try:
        for prec in (128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768):
            ctx.prec = prec
            v = x.arb()
            if v > 0:
                return 1
            if v < 0:
                return -1
    finally:
        ctx.prec = old
    raise RuntimeError("sign undecided (should not happen for nonzero form)")


def encl(x, digits=15):
    old = ctx.prec
    ctx.prec = ENCL_PREC
    try:
        return x.arb().str(digits, radius=True)
    finally:
        ctx.prec = old


# ---------------------------------------------------------------------------
# Bookkeeping
# ---------------------------------------------------------------------------

RESULTS = OrderedDict()   # claim -> [agree, disagree, first failure details]
DIAG = OrderedDict()      # diagnostics that do not enter OVERALL


PREFIX = ""              # claim-name prefix for worlds checked under a separate heading


def check(claim, ok, detail=""):
    r = RESULTS.setdefault(PREFIX + claim, [0, 0, []])
    if ok:
        r[0] += 1
    else:
        r[1] += 1
        if len(r[2]) < 3:
            r[2].append(detail)


def diag(name, flag):
    r = DIAG.setdefault(name, [0, 0])
    r[0 if flag else 1] += 1


# ---------------------------------------------------------------------------
# Worlds
# ---------------------------------------------------------------------------

class World:
    """Bipartite chain on X x M.

    U: families (x, x2, w, w2, tag) present at every m with the same rates.
    A: instances (x, x2, m, k, k2, fam) ; fam = family id or None.
    R: instances (x, m, m2, k, k2, fam) ; fam = family id or None.
    pi: integer weights of the single detailed-balance law (None if absent).
    """

    def __init__(self, nX, nM, kind):
        self.nX, self.nM, self.kind = nX, nM, kind
        self.pi = None
        self.U, self.A, self.R = [], [], []
        self.Ufam_pi = []      # per U family the law it balances (probe worlds)
        self.Afam = {}         # fam id -> (x, x2, k, k2)
        self.Rfam = {}         # fam id -> (m, m2, k, k2)
        self.Umdep = False

    def z(self, x, m):
        return x * self.nM + m

    def instances(self):
        """List of (cls, z, z2, k, k2, ref)."""
        out = []
        for f, (x, x2, w, w2, tag) in enumerate(self.U):
            for m in range(self.nM):
                if self.Umdep:
                    ww, ww2 = tag[m]
                else:
                    ww, ww2 = w, w2
                out.append(("U", self.z(x, m), self.z(x2, m), Fr(ww), Fr(ww2), f))
        for (x, x2, m, k, k2, fam) in self.A:
            out.append(("A", self.z(x, m), self.z(x2, m), Fr(k), Fr(k2), fam))
        for (x, m, m2, k, k2, fam) in self.R:
            out.append(("R", self.z(x, m), self.z(x, m2), Fr(k), Fr(k2), fam))
        return out


def connected(n, inst):
    par = list(range(n))

    def find(a):
        while par[a] != a:
            par[a] = par[par[a]]
            a = par[a]
        return a

    for (_, z, z2, k, k2, _) in inst:
        assert k > 0 and k2 > 0
        a, b = find(z), find(z2)
        if a != b:
            par[a] = b
    return len({find(i) for i in range(n)}) == 1


def rand_pair(rng, n):
    a, b = rng.sample(range(n), 2)
    return a, b


def gen_world(rng, kind):
    """kind in general, V4, V5, V45, V7, multilaw, Ureads."""
    while True:
        if kind == "V7":
            w = gen_v7(rng)
        else:
            nX = rng.randint(2, 5)
            nM = rng.randint(2, 4)
            w = World(nX, nM, kind)
            npairs = nX * (nX - 1) // 2
            if kind == "multilaw":
                # outside the class: each U family balances its own law
                for _ in range(rng.randint(1, npairs + 2)):
                    x, x2 = rand_pair(rng, nX)
                    lawf = [rng.randint(1, 5) for _ in range(nX)]
                    c = rng.randint(1, 3)
                    w.U.append((x, x2, c * lawf[x2], c * lawf[x], None))
                    w.Ufam_pi.append(lawf)
                # force at least one parallel pair of families with distinct ratios
                x, x2 = rand_pair(rng, nX)
                for lawf in ([1, 2, 3, 4, 5][:nX], [5, 1, 4, 2, 3][:nX]):
                    w.U.append((x, x2, lawf[x2], lawf[x], None))
                    w.Ufam_pi.append(lawf)
            else:
                w.pi = [rng.randint(1, 5) for _ in range(nX)]
                for _ in range(rng.randint(1, npairs + 2)):
                    x, x2 = rand_pair(rng, nX)
                    c = rng.randint(1, 3)
                    if kind == "Ureads":
                        tag = []
                        for m in range(nM):
                            cm = rng.randint(1, 4)
                            tag.append((cm * w.pi[x2] * rng.randint(1, 2), cm * w.pi[x]))
                        w.U.append((x, x2, None, None, tag))
                    elif kind == "nonDB":
                        # m-independent U rates with no detailed-balance law (sec. 1.2 remark)
                        w.U.append((x, x2, rng.randint(1, 5), rng.randint(1, 5), None))
                    else:
                        w.U.append((x, x2, c * w.pi[x2], c * w.pi[x], None))
                if kind == "Ureads":
                    w.Umdep = True
                if kind == "nonDB":
                    w.pi = None
            # actions
            if kind in ("V5", "V45"):
                nf = rng.randint(0 if kind == "V45" else 1, npairs + 1)
                fams = []
                for _ in range(nf):
                    x, x2 = rand_pair(rng, nX)
                    fams.append((x, x2, rng.randint(1, 5), rng.randint(1, 5)))
                    if rng.random() < 0.4:   # parallel family, other ratio
                        fams.append((x, x2, rng.randint(1, 5), rng.randint(1, 5)))
                for fid, (x, x2, k, k2) in enumerate(fams):
                    w.Afam[fid] = (x, x2, k, k2)
                    for m in range(nM):
                        w.A.append((x, x2, m, k, k2, fid))
            else:
                for _ in range(rng.randint(0 if kind != "general" else 1, nX * nM)):
                    x, x2 = rand_pair(rng, nX)
                    m = rng.randrange(nM)
                    w.A.append((x, x2, m, rng.randint(1, 5), rng.randint(1, 5), None))
            # register
            if kind in ("V4", "V45"):
                nf = rng.randint(1, nM * (nM - 1) // 2 + 1)
                fams = []
                for _ in range(nf):
                    m, m2 = rand_pair(rng, nM)
                    fams.append((m, m2, rng.randint(1, 5), rng.randint(1, 5)))
                    if rng.random() < 0.4:
                        fams.append((m, m2, rng.randint(1, 5), rng.randint(1, 5)))
                for fid, (m, m2, k, k2) in enumerate(fams):
                    w.Rfam[fid] = (m, m2, k, k2)
                    for x in range(nX):
                        w.R.append((x, m, m2, k, k2, fid))
            else:
                for _ in range(rng.randint(nX, 2 * nX * nM)):
                    x = rng.randrange(nX)
                    m, m2 = rand_pair(rng, nM)
                    w.R.append((x, m, m2, rng.randint(1, 5), rng.randint(1, 5), None))
        if connected(w.nX * w.nM, w.instances()):
            return w


def gen_v7(rng):
    """Two-store world: every U and A move changes the energy by one packet."""
    while True:
        nX = rng.randint(3, 5)
        nM = rng.randint(2, 3)
        E = [rng.randint(0, 2) for _ in range(nX)]
        cand = [(x, x2) for x in range(nX) for x2 in range(nX) if E[x2] == E[x] + 1]
        if not cand:
            continue
        bc = rng.randint(3, 6)
        bh = rng.randint(2, bc - 1)
        w = World(nX, nM, "V7")
        w.bc, w.bh, w.E = bc, bh, E
        Emax = max(E)
        w.pi = [bc ** (Emax - E[x]) * 1 for x in range(nX)]
        for _ in range(rng.randint(1, len(cand) + 1)):
            x, x2 = rng.choice(cand)       # build x -> x2
            c = rng.randint(1, 3)
            w.U.append((x, x2, c * w.pi[x2], c * w.pi[x], None))
        for _ in range(rng.randint(1, len(cand) * nM)):
            x, x2 = rng.choice(cand)
            m = rng.randrange(nM)
            s = rng.randint(1, 4)
            w.A.append((x, x2, m, s, s * bh, None))
        for _ in range(rng.randint(nX, 2 * nX * nM)):
            x = rng.randrange(nX)
            m, m2 = rand_pair(rng, nM)
            w.R.append((x, m, m2, rng.randint(1, 5), rng.randint(1, 5), None))
        if connected(nX * nM, w.instances()):
            return w


# ---------------------------------------------------------------------------
# Exact stationary law
# ---------------------------------------------------------------------------

def stationary(n, inst):
    """Solve P L = 0, sum P = 1 by Gauss-Jordan over Fractions."""
    # balance rows: for each state y: sum_z P(z) W(z->y) - P(y) out(y) = 0
    M = [[Fr(0)] * n for _ in range(n)]
    for (_, z, z2, k, k2, _) in inst:
        M[z2][z] += k
        M[z][z] -= k
        M[z][z2] += k2
        M[z2][z2] -= k2
    rhs = [Fr(0)] * n
    M[n - 1] = [Fr(1)] * n
    rhs[n - 1] = Fr(1)
    rows = [M[i] + [rhs[i]] for i in range(n)]
    for col in range(n):
        piv = None
        for r in range(col, n):
            if rows[r][col] != 0:
                piv = r
                break
        if piv is None:
            raise RuntimeError("singular system: stationary law not unique")
        rows[col], rows[piv] = rows[piv], rows[col]
        pv = rows[col][col]
        rows[col] = [v / pv for v in rows[col]]
        for r in range(n):
            if r != col and rows[r][col] != 0:
                f = rows[r][col]
                rows[r] = [a - f * b for a, b in zip(rows[r], rows[col])]
    return [rows[i][n] for i in range(n)]


def is_stationary(P, inst):
    n = len(P)
    flow = [Fr(0)] * n
    for (_, z, z2, k, k2, _) in inst:
        J = P[z] * k - P[z2] * k2
        flow[z] -= J
        flow[z2] += J
    return all(f == 0 for f in flow) and sum(P) == 1 and all(v > 0 for v in P)


# ---------------------------------------------------------------------------
# Section 2 quantities
# ---------------------------------------------------------------------------

def quantities(w, P=None):
    nX, nM = w.nX, w.nM
    n = nX * nM
    inst = w.instances()
    if P is None:
        P = stationary(n, inst)
    Q = {"P": P, "inst": inst}
    Q["stationary"] = is_stationary(P, inst)
    p = [sum(P[w.z(x, m)] for m in range(nM)) for x in range(nX)]
    q = [sum(P[w.z(x, m)] for x in range(nX)) for m in range(nM)]
    Q["p"], Q["q"] = p, q
    lnP = [LL.ln(v) for v in P]
    lnp = [LL.ln(v) for v in p]
    lnq = [LL.ln(v) for v in q]
    ii = [lnP[w.z(x, m)] - lnp[x] - lnq[m] for x in range(nX) for m in range(nM)]
    Q["i"] = ii
    rows = []
    for (cls, z, z2, k, k2, ref) in inst:
        J = P[z] * k - P[z2] * k2
        F = LL.ln(k / k2).scale(J)
        S = (lnP[z] + LL.ln(k) - lnP[z2] - LL.ln(k2)).scale(J)
        Id = (ii[z2] - ii[z]).scale(J)
        rows.append(dict(cls=cls, z=z, z2=z2, k=k, k2=k2, ref=ref, J=J, F=F, S=S,
                         Id=Id, u=P[z] * k, v=P[z2] * k2))
    Q["rows"] = rows
    by = lambda c, key: lsum(r[key] for r in rows if r["cls"] in c)
    Q["sigma_world"] = by("UA", "F")
    Q["sigma_reg"] = by("R", "F")
    Q["Sigma_A"] = by("A", "S")
    Q["Sigma_R"] = by("R", "S")
    Q["SU"] = by("U", "S")
    Q["Idot_act"] = -by("A", "Id")
    Q["Idot_learn"] = by("R", "Id")
    Q["Idot_noise"] = -by("U", "Id")
    Q["Id_U"] = by("U", "Id")
    Q["Id_A"] = by("A", "Id")
    Q["S_UA"] = by("UA", "S")
    # floor: one term per U channel (family over m)
    fl_terms = []
    flD = LL()
    for f, (x, x2, ww, ww2, tag) in enumerate(w.U):
        if w.Umdep:
            ww, ww2 = tag[0]           # meaningless outside the class; uses m=0
        j = p[x] * ww - p[x2] * ww2
        fl_terms.append((j, p[x] * ww, p[x2] * ww2,
                         LL.ln(Fr(p[x] * ww) / (p[x2] * ww2)).scale(j)))
        if w.pi is not None:
            flD = flD + (lnp[x] - LL.ln(w.pi[x]) - lnp[x2] + LL.ln(w.pi[x2])).scale(j)
    Q["floor_terms"] = fl_terms
    Q["floor"] = lsum(t[3] for t in fl_terms)
    Q["floor_D"] = flD
    # floor aggregated per unordered world pair (parallel U channels merged)
    agg = {}
    for (x, x2, ww, ww2, tag) in w.U:
        if w.Umdep:
            ww, ww2 = tag[0]
        key = (min(x, x2), max(x, x2))
        a = agg.setdefault(key, [Fr(0), Fr(0)])
        if x < x2:
            a[0] += ww; a[1] += ww2
        else:
            a[0] += ww2; a[1] += ww
    fa = LL()
    for (x, x2), (W, W2) in agg.items():
        j = p[x] * W - p[x2] * W2
        fa = fa + LL.ln(p[x] * W / (p[x2] * W2)).scale(j)
    Q["floor_agg"] = fa
    # Lemma 1 sums
    Q["L1_f"] = lsum((lnp[r["z2"] // nM] - lnp[r["z"] // nM]).scale(r["J"])
                     for r in rows if r["cls"] in "UA")
    Q["L1_g"] = lsum((lnq[r["z2"] % nM] - lnq[r["z"] % nM]).scale(r["J"])
                     for r in rows if r["cls"] == "R")
    Q["L1_i"] = lsum(r["Id"] for r in rows)
    # proof-of-(4) per-U-channel noise terms
    noise_terms = []
    for f in range(len(w.U)):
        rs = [r for r in rows if r["cls"] == "U" and r["ref"] == f]
        Uu = sum(r["u"] for r in rs)
        Vv = sum(r["v"] for r in rs)
        t = lsum(LL.ln(r["u"] / r["v"]).scale(r["J"]) for r in rs) \
            - LL.ln(Uu / Vv).scale(Uu - Vv)
        noise_terms.append(t)
    Q["noise_terms"] = noise_terms
    # V5: Phi_A per A family (channel) and aggregated per world pair
    if w.Afam:
        Q["Phi_A"] = lsum(LL.ln(Fr(k) * p[x] / (Fr(k2) * p[x2])).scale(k * p[x] - k2 * p[x2])
                          for (x, x2, k, k2) in w.Afam.values())
        Q["Phi_A_terms"] = [(k * p[x] - k2 * p[x2], Fr(k) * p[x] / (Fr(k2) * p[x2]))
                            for (x, x2, k, k2) in w.Afam.values()]
        Q["Phi_A_agg"] = phi_agg(w.Afam.values(), p)
    else:
        Q["Phi_A"], Q["Phi_A_terms"], Q["Phi_A_agg"] = LL(), [], LL()
    if w.Rfam:
        Q["Phi_R"] = lsum(LL.ln(Fr(k) * q[m] / (Fr(k2) * q[m2])).scale(k * q[m] - k2 * q[m2])
                          for (m, m2, k, k2) in w.Rfam.values())
        Q["Phi_R_agg"] = phi_agg(w.Rfam.values(), q)
    return Q


def phi_agg(fams, law):
    agg = {}
    for (a, b, k, k2) in fams:
        key = (min(a, b), max(a, b))
        s = agg.setdefault(key, [Fr(0), Fr(0)])
        if a < b:
            s[0] += k; s[1] += k2
        else:
            s[0] += k2; s[1] += k
    out = LL()
    for (a, b), (K, K2) in agg.items():
        out = out + LL.ln(K * law[a] / (K2 * law[b])).scale(K * law[a] - K2 * law[b])
    return out


def nonneg_term(J, u, v):
    """J ln(u/v) >= 0 decided exactly: sign(J) * sign(u/v - 1) >= 0."""
    sj = (J > 0) - (J < 0)
    sl = (u > v) - (u < v)
    return sj * sl >= 0


def has_parallel(fams):
    seen = {}
    for (a, b, k, k2) in fams:
        key = (min(a, b), max(a, b))
        r = Fr(k, k2) if a < b else Fr(k2, k)
        seen.setdefault(key, set()).add(r)
    return any(len(s) > 1 for s in seen.values())


# ---------------------------------------------------------------------------
# Per-world checks of sections 2 and 3
# ---------------------------------------------------------------------------

def check_world(w, label):
    Q = quantities(w)
    nX, nM = w.nX, w.nM
    tag = "%s(%s)" % (label, w.kind)
    check("exact stationarity of P", Q["stationary"], tag)
    sw, sr = Q["sigma_world"], Q["sigma_reg"]
    SA, SR, fl = Q["Sigma_A"], Q["Sigma_R"], Q["floor"]
    Ia, Il, In = Q["Idot_act"], Q["Idot_learn"], Q["Idot_noise"]
    check("(1) sigma_world = floor - Idot_act + Sigma_A", (sw - (fl - Ia + SA)).is_zero(), tag)
    check("(2) sigma_reg = Idot_learn + Sigma_R", (sr - (Il + SR)).is_zero(), tag)
    check("(3) Idot_learn = Idot_act + Idot_noise", (Il - (Ia + In)).is_zero(), tag)
    check("(3') sum_U S_c = floor + Idot_noise", (Q["SU"] - (fl + In)).is_zero(), tag)
    check("Lemma 1: sum_UA J[ln p(x')-ln p(x)] = 0", Q["L1_f"].is_zero(), tag)
    check("Lemma 1: sum_R J[ln q(m')-ln q(m)] = 0", Q["L1_g"].is_zero(), tag)
    check("Lemma 1: sum_all Idot_c = 0", Q["L1_i"].is_zero(), tag)
    check("Lemma 2(a): sum_UA S = sigma_world - sum_UA Idot",
          (Q["S_UA"] - (sw - Q["Id_U"] - Q["Id_A"])).is_zero(), tag)
    if w.kind != "nonDB":
        check("sec.2 floor = -d/dt D(p||pi_c) under U alone", (fl - Q["floor_D"]).is_zero(), tag)
        check("floor per U channel = floor per merged world pair (single pi_c)",
              (fl - Q["floor_agg"]).is_zero(), tag)
    else:
        diag("non-DB U worlds where the floor per U channel equals the floor per merged pair",
             (fl - Q["floor_agg"]).is_zero())
    # (4) by exact term signs
    check("(4) Sigma_A >= 0 (every term, exact sign)",
          all(nonneg_term(r["J"], r["u"], r["v"]) for r in Q["rows"] if r["cls"] == "A"), tag)
    check("(4) Sigma_R >= 0 (every term, exact sign)",
          all(nonneg_term(r["J"], r["u"], r["v"]) for r in Q["rows"] if r["cls"] == "R"), tag)
    check("(4) floor >= 0 (every term, exact sign)",
          all(nonneg_term(j, a, b) for (j, a, b, _) in Q["floor_terms"]), tag)
    sIn = sign_ll(In)
    check("(4) Idot_noise >= 0 (exact zero or certified)", sIn >= 0, tag)
    check("proof of (4): every per-U-channel noise term >= 0",
          all(sign_ll(t) >= 0 for t in Q["noise_terms"]), tag)
    cond = True
    for (x, x2, *_r) in w.U:
        for m in range(nM):
            if Q["P"][w.z(x, m)] / Q["p"][x] != Q["P"][w.z(x2, m)] / Q["p"][x2]:
                cond = False
    check("Idot_noise = 0 iff P(m|x) = P(m|x') on U pairs", (sIn == 0) == cond, tag)
    diag("worlds with Idot_noise exactly 0", sIn == 0)
    # corollaries
    check("V1 sigma_world >= floor - Idot_act", sign_ll(sw - fl + Ia) >= 0, tag)
    check("V1 Idot_act <= Idot_learn", sign_ll(Il - Ia) >= 0, tag)
    check("V1 sigma_world >= floor - Idot_learn", sign_ll(sw - fl + Il) >= 0, tag)
    a_cur = any(r["J"] != 0 for r in Q["rows"] if r["cls"] == "A")
    if a_cur:
        check("V1: some A transition carries current => Sigma_A > 0 (certified)", sign_ll(SA) > 0, tag)
    else:
        check("V1: no A current => Sigma_A = 0 and Idot_act = 0 (exact)", SA.is_zero() and Ia.is_zero(), tag)
    if not Ia.is_zero():
        check("V1 proof: Idot_act != 0 => some A transition has J != 0", a_cur, tag)
        check("V1 strict: Idot_act != 0 => sigma_world > floor - Idot_act (certified)",
              sign_ll(sw - fl + Ia) > 0, tag)
    diag("%s worlds with Idot_act != 0 (V1 strictness applies)" % w.kind, not Ia.is_zero())
    check("V2 sigma_reg >= Idot_learn", sign_ll(sr - Il) >= 0, tag)
    check("V3 sigma_world + sigma_reg = floor + Idot_noise + Sigma_A + Sigma_R",
          (sw + sr - (fl + In + SA + SR)).is_zero(), tag)
    check("V3 sigma_world + sigma_reg >= floor + Idot_noise", sign_ll(sw + sr - fl - In) >= 0, tag)
    below = sign_ll(sw - fl) < 0
    diag("%s worlds below the floor (sigma_world < floor)" % w.kind, below)
    if w.kind in ("V4", "V45"):
        check("V4 Idot_learn <= 0", sign_ll(Il) <= 0, tag)
        check("V4 proof: Sigma_R = Phi_R - Idot_learn (Phi_R per R channel family)",
              (SR - (Q["Phi_R"] - Il)).is_zero(), tag)
        check("V4 proof: Sigma_R >= Phi_R", sign_ll(SR - Q["Phi_R"]) >= 0, tag)
        check("V4 proof: sigma_world = floor + Idot_noise - Idot_learn + Sigma_A",
              (sw - (fl + In - Il + SA)).is_zero(), tag)
        check("V4 sigma_world >= floor + Idot_noise - Idot_learn", sign_ll(sw - fl - In + Il) >= 0, tag)
        check("V4 floor + Idot_noise - Idot_learn >= floor", sign_ll(In - Il) >= 0, tag)
        check("V4 sigma_world >= floor", not below, tag)
        if has_parallel(w.Rfam.values()):
            diag("V4 parallel-R worlds where Phi_R per merged register pair also fits",
                 (SR - (Q["Phi_R_agg"] - Il)).is_zero())
    if w.kind in ("V5", "V45"):
        check("V5 sigma_world = floor + Phi_A (Phi_A per A channel family, each counted once)",
              (sw - (fl + Q["Phi_A"])).is_zero(), tag)
        check("V5 Phi_A >= 0 (every term, exact sign)",
              all(nonneg_term(j, r, Fr(1)) for (j, r) in Q["Phi_A_terms"]), tag)
        check("V5 proof: Sigma_A = Phi_A + Idot_act", (SA - (Q["Phi_A"] + Ia)).is_zero(), tag)
        check("V5 sigma_world >= floor", not below, tag)
        p = Q["p"]
        flowX = [Fr(0)] * nX
        for (x, x2, ww, ww2, _t) in w.U:
            J = p[x] * ww - p[x2] * ww2
            flowX[x] -= J
            flowX[x2] += J
        for (x, x2, k, k2) in w.Afam.values():
            J = p[x] * k - p[x2] * k2
            flowX[x] -= J
            flowX[x2] += J
        check("V5: p is stationary for the autonomous world chain (U + A families)",
              all(f == 0 for f in flowX), tag)
        if has_parallel(w.Afam.values()):
            diag("V5 parallel-A worlds where Phi_A per merged world pair also fits",
                 (sw - (fl + Q["Phi_A_agg"])).is_zero())
    if w.kind == "V45":
        check("V4+V5 worlds: Idot_noise = 0 (product law)", sIn == 0, tag)
    if w.kind == "V7":
        Jb_U = Fr(0)
        Jb_A = Fr(0)
        check("V7 hypothesis b_c > b_h", w.bc > w.bh, tag)
        pk = True
        for r in Q["rows"]:
            if r["cls"] in "UA":
                d = w.E[r["z2"] // nM] - w.E[r["z"] // nM]
                b = w.bc if r["cls"] == "U" else w.bh
                pk = pk and (LL.ln(r["k"] / r["k2"]) - LL.ln(b).scale(-d)).is_zero()
        check("V7 hypothesis: packet relations ln(k/k') = -Delta|G| ln b (exact, every U and A jump)",
              pk, tag)
        for r in Q["rows"]:
            x, x2 = r["z"] // nM, r["z2"] // nM
            d = w.E[x2] - w.E[x]
            assert d in ((1, -1) if r["cls"] in "UA" else (0,))
            if r["cls"] == "U":
                Jb_U += d * r["J"]
            elif r["cls"] == "A":
                Jb_A += d * r["J"]
        check("V7 steady energy: cold build current = - hot build current", Jb_U + Jb_A == 0, tag)
        L = LL.ln(Fr(w.bc, w.bh))
        check("V7 sigma_world = J ln(b_c/b_h)", (sw - L.scale(Jb_A)).is_zero(), tag)
        old = ctx.prec
        ok = False
        if (L.scale(Jb_A) - (fl - Ia)).is_zero():
            ok = True
        else:
            for prec in (128, 256, 512, 1024, 2048, 4096):
                ctx.prec = prec
                val = arb(fmpq(Jb_A.numerator, Jb_A.denominator)) - (fl - Ia).arb() / L.arb()
                if val > 0:
                    ok = True
                    break
                if val < 0:
                    break
        ctx.prec = old
        check("V7 J >= [floor - Idot_act] / ln(b_c/b_h)", ok, tag)
        # V6 with alpha = lambda/ln b_c, lambda = 1: alpha [floor - sigma_world] <= alpha Idot_act
        s = sign_ll(Ia - fl + sw)
        old = ctx.prec
        ctx.prec = ENCL_PREC
        alpha_pos = (1 / arb(w.bc).log()) > 0
        ctx.prec = old
        check("V6 (alpha = 1/ln b_c): W_floor - W <= alpha Idot_act (exact sign x certified alpha > 0)",
              s >= 0 and alpha_pos, tag)
    if w.kind == "general":
        check_blind(w, Q, tag)
    return Q


def check_probe(w, label):
    """Diagnostics outside the class; do not enter OVERALL."""
    Q = quantities(w)
    sw, fl = Q["sigma_world"], Q["floor"]
    if w.kind == "multilaw":
        diag("multi-law probe: (1) holds with floor per U channel",
             (sw - (fl - Q["Idot_act"] + Q["Sigma_A"])).is_zero())
        diag("multi-law probe: (3') holds with floor per U channel",
             (Q["SU"] - (fl + Q["Idot_noise"])).is_zero())
        diag("multi-law probe: (1) holds with floor per merged world pair",
             (sw - (Q["floor_agg"] - Q["Idot_act"] + Q["Sigma_A"])).is_zero())
    else:
        diag("negative control (U reads register): checker detects (3') failure",
             not (Q["SU"] - (fl + Q["Idot_noise"])).is_zero())
        diag("negative control (U reads register): checker detects (1) failure",
             not (sw - (fl - Q["Idot_act"] + Q["Sigma_A"])).is_zero())


# ---------------------------------------------------------------------------
# Section 5: the construction of theorem T
# ---------------------------------------------------------------------------

def tree_currents(nodes, edges, inflow):
    """Currents on tree edges (a, b) [positive = a -> b] cancelling inflow."""
    adj = {v: [] for v in nodes}
    for idx, (a, b) in enumerate(edges):
        adj[a].append((b, idx))
        adj[b].append((a, idx))
    root = nodes[0]
    order, parent = [root], {root: (None, None)}
    for v in order:
        for (u, idx) in adj[v]:
            if u not in parent:
                parent[u] = (v, idx)
                order.append(u)
    assert len(order) == len(nodes), "not a spanning tree"
    sub = {v: inflow[v] for v in nodes}
    cur = [None] * len(edges)
    for v in reversed(order[1:]):
        par, idx = parent[v]
        # subtree of v has net inflow sub[v]; push it out to the parent
        a, b = edges[idx]
        cur[idx] = sub[v] if (a == v) else -sub[v]
        sub[par] += sub[v]
    return cur, sub[root]


def construct_T(nX, nM, Pstar, Ufams, Aspec, Rtree, Rextra, kappa):
    """Rates of section 5 for a given P*, U families, A currents.

    Aspec: list of (x, x2, m, j).  Rtree[x]: list of (m, m2) spanning M.
    Rextra: list of (x, m, m2) carrying zero.  Returns world and channel currents.
    """
    kappa = Fr(kappa)
    zf = lambda x, m: x * nM + m
    inflow = {(x, m): Fr(0) for x in range(nX) for m in range(nM)}
    for (x, x2, ww, ww2) in Ufams:
        for m in range(nM):
            J = Pstar[zf(x, m)] * ww - Pstar[zf(x2, m)] * ww2
            inflow[(x, m)] -= J
            inflow[(x2, m)] += J
    for (x, x2, m, j) in Aspec:
        inflow[(x, m)] -= j
        inflow[(x2, m)] += j
    world_ok = all(sum(inflow[(x, m)] for m in range(nM)) == 0 for x in range(nX))
    w = World(nX, nM, "T")
    for (x, x2, ww, ww2) in Ufams:
        w.U.append((x, x2, ww, ww2, None))

    def rates(z, z2, j):
        return ((kappa + max(j, 0)) / Pstar[z], (kappa + max(-j, 0)) / Pstar[z2])

    jlist = []
    for (x, x2, m, j) in Aspec:
        k, k2 = rates(zf(x, m), zf(x2, m), j)
        w.A.append((x, x2, m, k, k2, None))
        jlist.append(("A", j))
    roots_ok = True
    for x in range(nX):
        cur, rootres = tree_currents(list(range(nM)), Rtree[x],
                                     {m: inflow[(x, m)] for m in range(nM)})
        roots_ok = roots_ok and rootres == 0
        for (m, m2), j in zip(Rtree[x], cur):
            k, k2 = rates(zf(x, m), zf(x, m2), j)
            w.R.append((x, m, m2, k, k2, None))
            jlist.append(("R", j))
    for (x, m, m2) in Rextra:
        k, k2 = rates(zf(x, m), zf(x, m2), Fr(0))
        w.R.append((x, m, m2, k, k2, None))
        jlist.append(("R", Fr(0)))
    return w, jlist, world_ok and roots_ok


def x_connected(nX, w):
    """Graph on X with an edge per U channel and per A channel in any register state."""
    par = list(range(nX))

    def find(a):
        while par[a] != a:
            a = par[a]
        return a

    for (x, x2, *_r) in w.U:
        par[find(x)] = find(x2)
    for (x, x2, *_r) in w.A:
        par[find(x)] = find(x2)
    return len({find(x) for x in range(nX)}) == 1


def check_T(w, jlist, Pstar, label, kappa):
    """Checks of theorem T for one constructed world.  Returns Q."""
    inst = w.instances()
    check("T hypothesis: X connected by U channels and A channels", x_connected(w.nX, w), label)
    P = stationary(len(Pstar), inst)
    check("T: P* is the unique stationary law (solver = P*, balance exact)",
          P == list(Pstar) and is_stationary(list(Pstar), inst), label)
    Q = quantities(w, P=list(Pstar))
    rowsAR = [r for r in Q["rows"] if r["cls"] in "AR"]
    check("T: constructed channel currents equal the prescribed currents",
          [r["J"] for r in rowsAR] == [j for (_, j) in jlist], label)
    kap = Fr(kappa)
    per = True
    for r in rowsAR:
        aj = abs(r["J"])
        if not (r["S"] - LL.ln((kap + aj) / kap).scale(aj)).is_zero():
            per = False
    check("T: each A/R channel dissipates |j| ln[(kappa+|j|)/kappa] exactly", per, label)
    bound = sum(j * j for (_, j) in jlist) / kap
    diss = Q["Sigma_A"] + Q["Sigma_R"]
    ok = sign_ll(LL() - diss) >= 0 if bound == 0 else certify_le(diss, bound)
    check("T: Sigma_A + Sigma_R <= sum j^2 / kappa (certified)", ok, label)
    perb = all(certify_le(LL.ln((kap + abs(r["J"])) / kap).scale(abs(r["J"])), r["J"] ** 2 / kap)
               for r in rowsAR)
    check("T: each channel |j| ln[(kappa+|j|)/kappa] <= j^2/kappa (certified)", perb, label)
    # identities still hold in the constructed world
    sw, fl = Q["sigma_world"], Q["floor"]
    check("T worlds: identities (1),(2),(3),(3') exact",
          (sw - (fl - Q["Idot_act"] + Q["Sigma_A"])).is_zero()
          and (Q["sigma_reg"] - (Q["Idot_learn"] + Q["Sigma_R"])).is_zero()
          and (Q["Idot_learn"] - (Q["Idot_act"] + Q["Idot_noise"])).is_zero()
          and (Q["SU"] - (fl + Q["Idot_noise"])).is_zero(), label)
    check("T: |sigma_world - (floor - Idot_act)| <= sum j^2/kappa (certified)",
          certify_le(sw - (fl - Q["Idot_act"]), bound) and sign_ll(sw - (fl - Q["Idot_act"])) >= 0,
          label)
    check("T: |sigma_reg - Idot_learn| <= sum j^2/kappa (certified)",
          certify_le(Q["sigma_reg"] - Q["Idot_learn"], bound)
          and sign_ll(Q["sigma_reg"] - Q["Idot_learn"]) >= 0, label)
    if any(r["J"] != 0 for r in rowsAR if r["cls"] == "A"):
        check("T / sec.0: bound never attained: sigma_world > floor - Idot_act strictly (certified)",
              sign_ll(sw - (fl - Q["Idot_act"])) > 0, label)
    Q["bound"] = bound
    return Q


def certify_le(x, r):
    """Certify LL x <= rational r (strictly, or exact equality when r is 0 and x is 0)."""
    r = Fr(r)
    if r == 0:
        return sign_ll(x) <= 0
    old = ctx.prec
    try:
        for prec in (128, 256, 512, 1024, 2048, 4096, 8192):
            ctx.prec = prec
            v = arb(fmpq(r.numerator, r.denominator)) - x.arb()
            if v >= 0:
                return True
            if v < 0:
                return False
    finally:
        ctx.prec = old
    return False


def ratio_encl(Q):
    old = ctx.prec
    ctx.prec = ENCL_PREC
    try:
        return (Q["floor"] - Q["sigma_world"]).arb() / Q["Idot_act"].arb()
    finally:
        ctx.prec = old


def demon_world(p0, a, kappa):
    p1 = 1 - p0
    Pstar = [p0 * a, p0 * (1 - a), p1 * (1 - a), p1 * a]   # z = x*2 + m
    Ufams = [(0, 1, Fr(1), Fr(1))]
    Aspec = [(1, 0, 1, p0 - p1)]                            # in register state 1, x: 1 -> 0
    Rtree = {0: [(0, 1)], 1: [(0, 1)]}
    w, jlist, ok = construct_T(2, 2, Pstar, Ufams, Aspec, Rtree, [], kappa)
    return w, jlist, ok, Pstar


def demon_closed(p0, a):
    p1 = 1 - p0
    L = LL.ln(a / (1 - a))
    return {
        "floor": LL.ln(p0 / p1).scale(p0 - p1),
        "Idot_act": L.scale(p0 - p1),
        "Idot_noise": L.scale(2 * a - 1),
        "Idot_learn": L.scale(p0 - p1 + 2 * a - 1),
    }


def demon(out):
    p0, p1, a = Fr(3, 4), Fr(1, 4), Fr(9, 10)
    Pstar = demon_world(p0, a, 1)[3]
    out.append("")
    out.append("== Section 5: demon world  p = (3/4, 1/4),  a = 9/10 ==")
    out.append("P* = %s  (z = (x,m) in order (0,0),(0,1),(1,0),(1,1))" % [str(v) for v in Pstar])
    closed = demon_closed(p0, a)
    info_ref = None
    for kappa in (1, 10, 100, 1000, 10 ** 6):
        w, jlist, ok, _ = demon_world(p0, a, kappa)
        lab = "demon kappa=%d" % kappa
        check("T: construction hypotheses hold (world marginal conserved, tree roots balance)", ok, lab)
        Q = check_T(w, jlist, Pstar, lab, kappa)
        for name, form in closed.items():
            check("demon closed form: %s" % name, (Q[name] - form).is_zero(), lab)
        info = (Q["Idot_act"], Q["Idot_learn"], Q["Idot_noise"], Q["floor"])
        if info_ref is None:
            info_ref = info
            SU_ref = Q["SU"]
        else:
            check("T: information sums and floor do not depend on kappa",
                  all((u - v).is_zero() for u, v in zip(info, info_ref)), lab)
            check("T: U dissipation sum_U S_l does not depend on kappa", (Q["SU"] - SU_ref).is_zero(), lab)
        check("V3 in demon world: sigma_world + sigma_reg >= floor + Idot_noise",
              sign_ll(Q["sigma_world"] + Q["sigma_reg"] - Q["floor"] - Q["Idot_noise"]) >= 0, lab)
        r = ratio_encl(Q)
        out.append("kappa=%-8d rates: A %s ; R %s" % (
            kappa, [(str(k), str(k2)) for (_, _, _, k, k2, _) in w.A],
            [(str(k), str(k2)) for (_, _, _, k, k2, _) in w.R]))
        out.append("   currents j: %s   sum j^2/kappa = %s" % ([str(j) for _, j in jlist], Q["bound"]))
        out.append("   sigma_world          = %s" % encl(Q["sigma_world"]))
        out.append("   Sigma_A + Sigma_R    = %s" % encl(Q["Sigma_A"] + Q["Sigma_R"]))
        out.append("   (floor - sigma_world)/Idot_act = %s" % r.str(15, radius=True))
        if kappa == 1:
            check("sec.10 quoted ratio 0.815 at kappa=1 (certified |r-0.815|<5e-4)",
                  abs(r - arb(fmpq(815, 1000))) < arb(fmpq(5, 10000)), lab)
        if kappa == 10 ** 6:
            check("sec.10 quoted ratio 0.99999977 at kappa=1e6 (certified |r-q|<5e-9)",
                  abs(r - arb(fmpq(99999977, 10 ** 8))) < arb(fmpq(5, 10 ** 9)), lab)
            Qlast = Q
    Q = Qlast
    lim = Q["floor"] - Q["Idot_act"]          # T: sigma_world -> floor - Idot_act
    out.append("closed forms (exact, in ln 3 units via prime vector):")
    for name in ("floor", "Idot_act", "Idot_noise", "Idot_learn"):
        out.append("   %-10s = %s   prime vector %s" % (name, encl(Q[name]),
                   {p: str(v) for p, v in Q[name].c.items()}))
    out.append("   limit sigma_world (floor - Idot_act) = %s  prime vector %s" % (
        encl(lim), {p: str(v) for p, v in lim.c.items()}))
    check("sec.10: sigma_world converges to -(1/2) ln 3", (lim - LL.ln(3).scale(Fr(-1, 2))).is_zero(), "demon")
    corr_lim = LL.ln(p0 * (1 - a) / (p1 * a)).scale(p0 - p1)
    check("sec.5 text: limit sigma_world = (p0-p1) ln[p0 (1-a) / (p1 a)] = floor - Idot_act (exact)",
          (lim - corr_lim).is_zero(), "demon")
    check("sec.5 text: the limit is negative (a/(1-a) > p0/p1, i.e. a > p0)",
          a / (1 - a) > p0 / p1 and a > p0 and sign_ll(corr_lim) < 0, "demon")
    run1 = LL.ln(p1 * a / (p0 * (1 - a))).scale(p0 - p1)
    out.append("   run-1 text formula (p0-p1) ln[p1 a/(p0(1-a))], now withdrawn = %s" % encl(run1))
    diag("run-1 text formula (now withdrawn) is certified positive in the demon world", sign_ll(run1) > 0)


def demon_grid(out):
    """Section 5 demon statements on a grid of (p0, a), kappa = 1000."""
    p0s = [Fr(3, 5), Fr(2, 3), Fr(3, 4), Fr(4, 5), Fr(9, 10)]
    As = [Fr(11, 20), Fr(3, 5), Fr(2, 3), Fr(3, 4), Fr(4, 5), Fr(9, 10), Fr(19, 20)]
    kappa = 1000
    tally = {-1: 0, 0: 0, 1: 0}
    for p0 in p0s:
        for a in As:
            p1 = 1 - p0
            lab = "demon grid p0=%s a=%s" % (p0, a)
            w, jlist, ok, Pstar = demon_world(p0, a, kappa)
            check("demon grid: construction hypotheses hold", ok, lab)
            Q = check_T(w, jlist, Pstar, lab, kappa)
            closed = demon_closed(p0, a)
            check("demon grid: four closed forms exact",
                  all((Q[n] - f).is_zero() for n, f in closed.items()), lab)
            lim = Q["floor"] - Q["Idot_act"]
            form = LL.ln(p0 * (1 - a) / (p1 * a)).scale(p0 - p1)
            check("demon grid: limit (p0-p1) ln[p0 (1-a)/(p1 a)] = floor - Idot_act (exact)",
                  (lim - form).is_zero(), lab)
            s = sign_ll(form)
            tally[s] += 1
            check("demon grid: limit < 0  <=>  a/(1-a) > p0/p1  <=>  a > p0 (exact)",
                  (s < 0) == (a / (1 - a) > p0 / p1) == (a > p0), lab)
            check("demon grid: V3 sigma_world + sigma_reg >= floor + Idot_noise",
                  sign_ll(Q["sigma_world"] + Q["sigma_reg"] - Q["floor"] - Q["Idot_noise"]) >= 0, lab)
    out.append("demon grid (%d worlds, kappa = 1000): limit negative %d, zero %d (a = p0), positive %d"
               % (len(p0s) * len(As), tally[-1], tally[0], tally[1]))


def check_blind(w, Q, tag):
    """Section 6: blind agent (M a single state) holding the same p, built by the T recipe."""
    nX = w.nX
    p = Q["p"]
    Ufams = [(x, x2, Fr(ww), Fr(ww2)) for (x, x2, ww, ww2, _t) in w.U]
    inflowX = {x: Fr(0) for x in range(nX)}
    for (x, x2, ww, ww2) in Ufams:
        J = p[x] * ww - p[x2] * ww2
        inflowX[x] -= J
        inflowX[x2] += J
    tedges = [(x - 1, x) for x in range(1, nX)]
    cur, rootres = tree_currents(list(range(nX)), tedges, inflowX)
    Aspec = [(x, x2, 0, j) for (x, x2), j in zip(tedges, cur)]
    total_inf = Q["sigma_world"] + Q["sigma_reg"]
    In, SA, SR = Q["Idot_noise"], Q["Sigma_A"], Q["Sigma_R"]
    for kappa in (1, 1000):
        wb, _jl, ok = construct_T(nX, 1, list(p), Ufams, Aspec, {x: [] for x in range(nX)}, [], kappa)
        Pb = stationary(nX, wb.instances())
        Qb = quantities(wb, P=Pb)
        check("sec.6 blind agent: holds the same p (unique stationary law = p, exact)",
              ok and rootres == 0 and Pb == list(p) and Qb["stationary"], tag)
        total_b = Qb["sigma_world"] + Qb["sigma_reg"]
        check("sec.6 blind total = sigma_floor(p) + Sigma_A(blind), same floor (exact)",
              (total_b - (Q["floor"] + Qb["Sigma_A"])).is_zero() and (Qb["floor"] - Q["floor"]).is_zero()
              and Qb["Idot_act"].is_zero() and Qb["Idot_noise"].is_zero(), tag)
        cheaper = sign_ll(total_inf - total_b) < 0
        crit = sign_ll(In + SA + SR - Qb["Sigma_A"]) < 0
        check("sec.6: informed cheaper in total <=> Idot_noise + Sigma_A + Sigma_R < Sigma_A(blind)",
              cheaper == crit, tag)
        diag("sec.6 informed agent cheaper in total than blind agent (kappa_blind=%d)" % kappa, cheaper)


def random_T(rng, count, out):
    """General theorem T on random P*, U channels and A currents."""
    shown = 0
    for t in range(count):
        nX = rng.randint(2, 4)
        nM = rng.randint(2, 3)
        n = nX * nM
        wts = [rng.randint(1, 9) for _ in range(n)]
        tot = sum(wts)
        Pstar = [Fr(v, tot) for v in wts]
        pi = [rng.randint(1, 5) for _ in range(nX)]
        Ufams = []
        for _ in range(rng.randint(1, nX + 1)):
            x, x2 = rand_pair(rng, nX)
            c = rng.randint(1, 3)
            Ufams.append((x, x2, Fr(c * pi[x2]), Fr(c * pi[x])))
        # extra A channels with random currents, then a spanning tree of A channels on X
        Aspec = []
        for _ in range(rng.randint(0, 2)):
            x, x2 = rand_pair(rng, nX)
            Aspec.append((x, x2, rng.randrange(nM), Fr(rng.randint(-3, 3), 10)))
        inflowX = {x: Fr(0) for x in range(nX)}
        for (x, x2, ww, ww2) in Ufams:
            for m in range(nM):
                J = Pstar[x * nM + m] * ww - Pstar[x2 * nM + m] * ww2
                inflowX[x] -= J
                inflowX[x2] += J
        for (x, x2, m, j) in Aspec:
            inflowX[x] -= j
            inflowX[x2] += j
        perm = list(range(nX))
        rng.shuffle(perm)
        tedges = [(perm[rng.randrange(i)], perm[i]) for i in range(1, nX)]
        cur, rootres = tree_currents(list(range(nX)), tedges, inflowX)
        assert rootres == 0
        for (x, x2), j in zip(tedges, cur):
            Aspec.append((x, x2, rng.randrange(nM), j))
        Rtree = {}
        Rextra = []
        for x in range(nX):
            pm = list(range(nM))
            rng.shuffle(pm)
            Rtree[x] = [(pm[rng.randrange(i)], pm[i]) for i in range(1, nM)]
            if rng.random() < 0.5:
                m, m2 = rand_pair(rng, nM)
                Rextra.append((x, m, m2))
        ref = None
        for kappa in (1, 1000):
            w, jlist, ok = construct_T(nX, nM, Pstar, Ufams, Aspec, Rtree, Rextra, kappa)
            lab = "randomT#%d kappa=%d" % (t, kappa)
            check("T: construction hypotheses hold (world marginal conserved, tree roots balance)", ok, lab)
            Q = check_T(w, jlist, Pstar, lab, kappa)
            info = (Q["Idot_act"], Q["Idot_learn"], Q["Idot_noise"], Q["floor"])
            if ref is None:
                ref = info
                SU_ref = Q["SU"]
            else:
                check("T: information sums and floor do not depend on kappa",
                      all((u - v).is_zero() for u, v in zip(info, ref)), lab)
                check("T: U dissipation sum_U S_l does not depend on kappa", (Q["SU"] - SU_ref).is_zero(), lab)
            if kappa == 1000 and sign_ll(Q["Idot_act"]) > 0:
                diag("random T worlds with Idot_act > 0", True)
                if shown < 4:
                    shown += 1
                    out.append("randomT#%d |X|=%d |M|=%d kappa=1000: (floor-sigma_world)/Idot_act = %s"
                               % (t, nX, nM, ratio_encl(Q).str(12, radius=True)))
            elif kappa == 1000:
                diag("random T worlds with Idot_act > 0", False)


# ---------------------------------------------------------------------------

def main():
    rng = random.Random(SEED)
    out = []
    out.append("Independent exact check of the information law (sections 1, 2, 3, 5; run 2 adds 3, 5, 6 revisions)")
    out.append("seed = %d ; identities: full prime factorisation ; inequalities: exact signs / arb" % SEED)
    plan = [("general", 260), ("V4", 70), ("V5", 70), ("V45", 30), ("V7", 50)]
    probes = [("multilaw", 20), ("Ureads", 20)]
    nworld = 0
    shown = 0
    out.append("")
    out.append("== Random worlds in the class ==")
    for kind, cnt in plan:
        for i in range(cnt):
            w = gen_world(rng, kind)
            nworld += 1
            label = "W%03d" % nworld
            Q = check_world(w, label)
            if shown < 4 and kind in ("general", "V7"):
                shown += 1
                out.append("%s kind=%s |X|=%d |M|=%d U=%d A=%d R=%d  P=%s" % (
                    label, kind, w.nX, w.nM, len(w.U), len(w.A), len(w.R),
                    [str(v) for v in Q["P"]]))
                sw, fl, Ia, SA = Q["sigma_world"], Q["floor"], Q["Idot_act"], Q["Sigma_A"]
                rows = [("(1) lhs sigma_world", sw), ("(1) rhs floor-Idot_act+Sigma_A", fl - Ia + SA),
                        ("(2) lhs sigma_reg", Q["sigma_reg"]),
                        ("(2) rhs Idot_learn+Sigma_R", Q["Idot_learn"] + Q["Sigma_R"]),
                        ("(3) lhs Idot_learn", Q["Idot_learn"]),
                        ("(3) rhs Idot_act+Idot_noise", Q["Idot_act"] + Q["Idot_noise"]),
                        ("(3') lhs sum_U S_c", Q["SU"]),
                        ("(3') rhs floor+Idot_noise", fl + Q["Idot_noise"])]
                for name, val in rows:
                    out.append("   %-32s %s" % (name, encl(val)))
                basis = set(sw.c) | set(fl.c) | set(Ia.c) | set(SA.c)
                out.append("   (1): %d primes carry nonzero coefficients in its four terms; "
                           "all coefficients of lhs - rhs vanish: %s"
                           % (len(basis), (sw - (fl - Ia + SA)).is_zero()))
    out.append("worlds in the class checked: %d" % nworld)
    nprobe = 0
    for kind, cnt in probes:
        for i in range(cnt):
            w = gen_world(rng, kind)
            nprobe += 1
            check_probe(w, "P%03d" % nprobe)
    out.append("probe worlds outside the class (diagnostics only): %d" % nprobe)
    demon(out)
    out.append("")
    out.append("== Section 5: theorem T on random P*, U channels and A currents ==")
    random_T(rng, 40, out)
    # ---- run 2 additions (after all run-1 draws, so run-1 worlds are unchanged) ----
    global PREFIX
    out.append("")
    out.append("== Run 2: sec.1.2 remark, (1)-(3') for m-independent U rates without detailed balance ==")
    PREFIX = "[sec.1.2 non-DB U] "
    for i in range(60):
        w = gen_world(rng, "nonDB")
        check_world(w, "N%03d" % (i + 1))
    PREFIX = ""
    out.append("non-DB U worlds checked: 60")
    out.append("")
    out.append("== Run 2: demon world on a grid of (p0, a) ==")
    demon_grid(out)
    # T without its connectivity hypothesis: X = {0,1,2}, U joins 0-1 only, no A channel
    nX, nM = 3, 2
    Pstar = [Fr(1, 6)] * 6
    w, jl, ok = construct_T(nX, nM, Pstar, [(0, 1, Fr(1), Fr(1))], [],
                            {x: [(0, 1)] for x in range(nX)}, [], 1)
    try:
        stationary(nX * nM, w.instances())
        unique = True
    except RuntimeError:
        unique = False
    diag("T without X-connectivity: P* balances exactly yet the stationary law is not unique",
         ok and is_stationary(Pstar, w.instances()) and not x_connected(nX, w) and not unique)
    out.append("")
    out.append("factorisation: %d integers factored, largest %d digits, %d distinct primes" % (
        len(_FACTOR_CACHE), STATS["max_digits"], len(STATS["primes"])))
    out.append("")
    out.append("== Claims: agree / disagree ==")
    total_a = total_d = 0
    for claim, (a, d, det) in RESULTS.items():
        total_a += a
        total_d += d
        out.append("%-5s %5d / %-3d  %s" % ("PASS" if d == 0 else "FAIL", a, d, claim))
        for x in det:
            out.append("              first failure: %s" % x)
    out.append("")
    out.append("== Diagnostics (do not enter OVERALL): true / false ==")
    for name, (a, b) in DIAG.items():
        out.append("      %5d / %-3d  %s" % (a, b, name))
    out.append("")
    out.append("checks: %d agree, %d disagree, %d claims" % (total_a, total_d, len(RESULTS)))
    out.append("OVERALL PASS" if total_d == 0 else "OVERALL FAIL")
    sys.stdout.write("\n".join(out) + "\n")


if __name__ == "__main__":
    main()
