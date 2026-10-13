"""Independent check of CAL-THAIM-1 from its frozen boundary and recorded outputs.

Inputs (read only):
  ../derivable/xypher-thaim-reward-boundary.md   (hash checked, values transcribed below)
  ../xypher-thaim-reward-proof/report.txt        (hash checked, parsed)
  ../xypher-thaim-reward-proof/tables/<W>/<agent>.txt

Everything is rebuilt from the boundary text.  Every decision is exact:
rationals (python-flint fmpq / Python int) for all rational quantities; linear
forms in logarithms are expanded over the primes (flint fmpz.factor), so an
exact zero is a zero coefficient vector, and a sign is taken from a certified
arb enclosure, doubling precision from 256 bits until it excludes zero.
No float is used anywhere.

Steady states use a solver written here: multi-modular Gaussian elimination
(nmod_mat over 62-bit primes), Chinese remaindering, rational reconstruction,
and an exact integer verification of p Q = 0, sum p = 1 that must succeed
before a solution is accepted.

Run:  /tmp/thaimvenv/bin/python check.py > output.txt
"""

import hashlib
import math
import os
import re
import sys
import time

from flint import arb, ctx, fmpq, fmpz, fmpz_mat, nmod_mat

T0 = time.time()
ctx.prec = 256
HERE = os.path.dirname(os.path.abspath(__file__))
PHYS = os.path.dirname(HERE)
BOUNDARY = os.path.join(PHYS, "derivable", "xypher-thaim-reward-boundary.md")
PROOF = os.path.join(PHYS, "xypher-thaim-reward-proof")
REPORT = os.path.join(PROOF, "report.txt")
TABLES = os.path.join(PROOF, "tables")
BOUNDARY_SHA = "bfac95e0ecefb98e57bc6bc82ce088b38ef79c6a339dca5347979c0a8642744c"
REPORT_SHA = "a419d1cf735e10daea9b77d083ae08535fb3d9c028eaaeeb9e8486a3c7a8f0a4"

OUT = []


def out(s=""):
    OUT.append(s)
    print(s)


# ---------------------------------------------------------------- tallies
TALLY = {}  # category -> [agree, disagree]
DISAGREEMENTS = []
AMBIGUITIES = []


def tally(cat, ok, what):
    t = TALLY.setdefault(cat, [0, 0])
    if ok:
        t[0] += 1
    else:
        t[1] += 1
        DISAGREEMENTS.append(f"{cat}: {what}")
    return "agree" if ok else "DISAGREE"


# ---------------------------------------------------------------- section 1 world
NI = 4
EDGES = [(i, j) for i in range(NI) for j in range(i + 1, NI)]  # lexicographic
K = len(EDGES)
NS = (1 << K) * NI  # 256 states, index G*N + pos
UMAX = fmpq(4)
HALF_UMAX = fmpq(2)
Q24 = 1 << 24

# section 6 table: (name, tau, b_c, b_h, v)
WORLDS = [
    ("W1", 3, 5, 2, 1),
    ("W2", 3, 5, 3, 4),
    ("W3", 3, 9, 2, 1),
    ("W4", 3, 20, 2, 1),
    ("W5", 4, 20, 3, 1),
    ("W6", 4, 60, 2, 2),
]
WARM = ["W1", "W2", "W3", "W5"]
COLD = ["W4", "W6"]

# section 3 key classes, section 7 order
AGENTS = [
    ("LOCAL", "local"),
    ("GLOBAL", "blind"),
    ("ORACLE", "full"),
    ("NET-blind", "blind"),
    ("NET-local", "local"),
    ("NET-full", "full"),
    ("RECEIPT-full", "full"),
    ("WORLD-full", "full"),
    ("NET-full c=3/4", "full"),
    ("NET-full c=3/2", "full"),
    ("NET-full init 1/4", "full"),
    ("MLP-full s0", "full"),
    ("MLP-full s1", "full"),
]
TABULAR_TRAINED = ["NET-blind", "NET-local", "NET-full", "RECEIPT-full", "WORLD-full",
                   "NET-full c=3/4", "NET-full c=3/2", "NET-full init 1/4"]
MLP = ["MLP-full s0", "MLP-full s1"]


def walk_counts(n, edges, present, tau):
    """N_tau(pos) = e_pos^T (I + A)^tau 1 for the bridges in `present`."""
    adj = [[] for _ in range(n)]
    for k in present:
        i, j = edges[k]
        adj[i].append(j)
        adj[j].append(i)
    vec = [1] * n
    for _ in range(tau):
        vec = [vec[i] + sum(vec[j] for j in adj[i]) for i in range(n)]
    return vec


def bits(G):
    return [k for k in range(K) if G >> k & 1]


class World:
    def __init__(self, name, tau, bc, bh, v):
        self.name, self.tau, self.bc, self.bh, self.v = name, tau, bc, bh, v
        self.N = [0] * NS
        for G in range(1 << K):
            wc = walk_counts(NI, EDGES, bits(G), tau)
            for pos in range(NI):
                self.N[G * NI + pos] = wc[pos]
        # fuel / weather pairs: a (e absent) <-> b (e present)
        self.pairs = []
        for a in range(NS):
            G, pos = divmod(a, NI)
            for e in range(K):
                if not G >> e & 1:
                    b = a + (1 << e) * NI
                    self.pairs.append((a, b, e))
        self.r = [fmpq(self.N[b], self.N[a]) for a, b, e in self.pairs]
        self.touch = [a % NI in EDGES[e] for a, b, e in self.pairs]
        # walk pairs, each unordered pair once
        self.walks = []
        for x in range(NS):
            G, pos = divmod(x, NI)
            for k in bits(G):
                i, j = EDGES[k]
                if pos == i:
                    y = G * NI + j
                    self.walks.append((x, y))
        # keys
        self.rawkey = {"blind": [], "local": [], "full": []}
        for a, b, e in self.pairs:
            G, pos = divmod(a, NI)
            at = tuple(G >> k & 1 for k in range(K) if pos in EDGES[k])
            self.rawkey["blind"].append((e,))
            self.rawkey["local"].append((pos, at, e))
            self.rawkey["full"].append((a, e))
        self.keyindex = {}
        self.nkeys = {}
        for cls, raws in self.rawkey.items():
            order = sorted(set(raws))
            rank = {k: i for i, k in enumerate(order)}
            self.keyindex[cls] = [rank[k] for k in raws]
            self.nkeys[cls] = len(order)
        # Phi_tau polynomial coefficients: c_k = sum over |G| = k of sum_pos N
        self.ck = [0] * (K + 1)
        for G in range(1 << K):
            self.ck[bin(G).count("1")] += sum(self.N[G * NI + p] for p in range(NI))
        self.r0 = fmpq(1, 1 + bc)

    def phi(self, rho):
        s = fmpq(0)
        for k in range(K + 1):
            s += self.ck[k] * rho ** k * (1 - rho) ** (K - k)
        return s / NI

    def pi_c(self, bc=None):
        bc = self.bc if bc is None else bc
        w = [fmpq(bc) ** (K - bin(x // NI).count("1")) for x in range(NS)]
        z = sum(w)
        return [x / z for x in w]


# ---------------------------------------------------------------- exact logs
FACT = {}


def logvec(n):
    """prime -> exponent of the integer n >= 1 (flint factorisation)."""
    n = int(n)
    if n not in FACT:
        FACT[n] = {int(p): int(e) for p, e in fmpz(n).factor()} if n > 1 else {}
    return FACT[n]


def form_add(f, g, c=fmpq(1)):
    h = dict(f)
    for p, e in g.items():
        h[p] = h.get(p, fmpq(0)) + c * e
        if h[p] == 0:
            del h[p]
    return h


def form_scale(f, c):
    return {p: c * e for p, e in f.items() if c * e != 0}


def form_from_weights(wts):
    """sum over integers n of wts[n] * ln n."""
    f = {}
    for n in sorted(wts):
        if wts[n] != 0:
            f = form_add(f, logvec(n), wts[n])
    return f


def form_arb(f, prec):
    old = ctx.prec
    ctx.prec = prec
    try:
        s = arb(0)
        for p in sorted(f):
            s += arb(f[p]) * arb(p).log()
        return s
    finally:
        ctx.prec = old


def form_sign(f):
    if not f:
        return 0
    prec = 256
    while True:
        a = form_arb(f, prec)
        if a > 0:
            return 1
        if a < 0:
            return -1
        prec *= 2


# ---------------------------------------------------------------- own exact solver
PRIMES = []


def primes_upto(count):
    q = PRIMES[-1] if PRIMES else (1 << 62)
    while len(PRIMES) < count:
        q -= 1
        if fmpz(q).is_prime():
            PRIMES.append(q)
    return PRIMES[:count]


def ratrec(a, m, bound):
    r0, r1 = m, a % m
    s0, s1 = 0, 1
    while r1 > bound:
        q = r0 // r1
        r0, r1 = r1, r0 - q * r1
        s0, s1 = s1, s0 - q * s1
    if s1 == 0 or abs(s1) > bound:
        return None
    if s1 < 0:
        r1, s1 = -r1, -s1
    if math.gcd(r1, s1) != 1:
        return None
    return r1, s1


SOLVER_STATS = []


def stationary(n, out_rates):
    """out_rates[x] = {y: fmpq rate}; returns exact stationary law as fmpq list.

    Multi-modular elimination on M^T with the last equation replaced by the
    normalisation, CRT, rational reconstruction, exact integer verification.
    """
    S = 1
    for row in out_rates:
        for k in row.values():
            S = S * int(k.q) // math.gcd(S, int(k.q))
    rows = [{y: int(k * S) for y, k in row.items() if k != 0} for row in out_rates]
    mt = [[0] * n for _ in range(n)]
    for x in range(n):
        tot = 0
        for y, k in rows[x].items():
            mt[y][x] += k
            tot += k
        mt[x][x] -= tot
    mt[n - 1] = [1] * n
    A = fmpz_mat(mt)
    vals = [0] * n
    mod = 1
    used = 0
    target = 16
    while True:
        for p in primes_upto(target)[used:]:
            used += 1
            try:
                xs = nmod_mat(A, p).solve(nmod_mat(n, 1, [0] * (n - 1) + [1], p))
            except (ZeroDivisionError, ValueError):
                continue
            inv = pow(mod % p, -1, p)
            for i in range(n):
                r = int(xs[i, 0])
                vals[i] = vals[i] + mod * ((r - vals[i]) * inv % p)
            mod *= p
        bound = math.isqrt(mod // 2)
        cand = []
        den = 1
        ok = True
        for i in range(n):
            t = vals[i] * den % mod
            if t <= bound:
                cand.append((t, den))
                continue
            rr = ratrec(vals[i], mod, bound)
            if rr is None:
                ok = False
                break
            cand.append(rr)
            den = den * rr[1] // math.gcd(den, rr[1])
        if ok:
            D = 1
            for num, d in cand:
                D = D * d // math.gcd(D, d)
            nums = [num * (D // d) for num, d in cand]
            # exact verification of nums M = 0, sum nums = D
            acc = [0] * n
            for x in range(n):
                tot = 0
                for y, k in rows[x].items():
                    acc[y] += nums[x] * k
                    tot += k
                acc[x] -= nums[x] * tot
            if all(a == 0 for a in acc) and sum(nums) == D:
                SOLVER_STATS.append((used, D.bit_length()))
                return [fmpq(v, D) for v in nums]
        target *= 2


# ---------------------------------------------------------------- chain
class Chain:
    """Channels: (kind, x, y, k_xy, k_yx, info). kind W weather, K walk, F fuel."""

    def __init__(self, world, u, bh=None, weather_mod=False, demon=False, n=NS):
        self.world = world
        self.bc = world.bc
        self.bh = world.bh if bh is None else bh
        self.u = u
        self.ch = []
        for i, (a, b, e) in enumerate(world.pairs):
            wear = fmpq(self.bc) * ((1 + u[i]) if weather_mod else 1)
            self.ch.append(("W", a, b, fmpq(1), wear, i))
        for x, y in world.walks:
            self.ch.append(("K", x, y, fmpq(world.v), fmpq(world.v), None))
        for i, (a, b, e) in enumerate(world.pairs):
            back = self.bh * u[i]
            if demon and i == 0:
                back = back + 1
            self.ch.append(("F", a, b, u[i], back, i))
        self.n = n

    def solve(self):
        rates = [dict() for _ in range(self.n)]
        for kind, x, y, kxy, kyx, info in self.ch:
            rates[x][y] = rates[x].get(y, fmpq(0)) + kxy
            rates[y][x] = rates[y].get(x, fmpq(0)) + kyx
        self.p = stationary(self.n, rates)
        p = self.p
        self.cur = [p[x] * kxy - p[y] * kyx for kind, x, y, kxy, kyx, info in self.ch]
        return p


# ---------------------------------------------------------------- tables
def table_file(world, agent):
    return os.path.join(TABLES, world, agent.replace(" ", "_").replace("/", "-") + ".txt")


def read_table(path):
    raw = open(path, "rb").read()
    text = raw.decode("ascii")
    vals = []
    for i, line in enumerate(text.splitlines()):
        idx, frac = line.split("\t")
        num, den = frac.split("/")
        assert int(idx) == i
        vals.append(fmpq(int(num), int(den)))
    canon = "".join(f"{i}\t{int(v.p)}/{int(v.q)}\n" for i, v in enumerate(vals))
    return vals, raw, canon.encode("ascii") == raw, hashlib.sha256(raw).hexdigest()


# ---------------------------------------------------------------- evaluation
def evaluate(world, chain, balpha=None):
    """Exact quantities and scaled linear forms for one solved chain."""
    w = world
    balpha = w.bc if balpha is None else balpha
    p = chain.p
    ev = {}
    J = fmpq(0)
    resid = fmpq(0)
    aimJ = fmpq(0)
    sA = {}
    sU = {}
    rec = {}
    sigma_c = fmpq(0)
    sigma_h = fmpq(0)
    sigma_ok = True
    for (kind, x, y, kxy, kyx, info), c in zip(chain.ch, chain.cur):
        nx, ny = w.N[x], w.N[y]
        if kind == "F":
            J += c
            if kxy < HALF_UMAX:
                resid += c
            if w.touch[info]:
                aimJ += c
            if nx != ny:
                sA[nx] = sA.get(nx, fmpq(0)) - c
                sA[ny] = sA.get(ny, fmpq(0)) + c
                g = kxy * p[x]
                rec[nx] = rec.get(nx, fmpq(0)) - g
                rec[ny] = rec.get(ny, fmpq(0)) + g
            if kxy != 0:
                if kyx != chain.bh * kxy:
                    sigma_ok = False
                sigma_h -= c
        else:
            if nx != ny:
                sU[nx] = sU.get(nx, fmpq(0)) - c
                sU[ny] = sU.get(ny, fmpq(0)) + c
            if kind == "W":
                if kyx != chain.bc * kxy:
                    sigma_ok = False
                sigma_c -= c
            elif kxy != kyx:
                sigma_ok = False
    G = sum(p[x] * bin(x // NI).count("1") for x in range(NS))
    ev["J"] = J
    ev["G"] = G
    ev["resid"] = resid
    ev["Jsum_identity"] = J == (w.bc + 1) * G - K
    ev["sigma_formal"] = sigma_ok and sigma_c == J and sigma_h == -J
    ev["SdotA"] = form_from_weights(sA)
    ev["SdotU"] = form_from_weights(sU)
    ev["receipt_flow"] = form_from_weights(rec)
    lnb = logvec(balpha)
    ev["Fnet"] = form_add(ev["SdotA"], lnb, -J)
    ev["Freceipt"] = form_add(ev["receipt_flow"], lnb, -J)
    ev["Fworld"] = form_add(form_add(ev["SdotA"], ev["SdotU"]), lnb, -J)
    Hp = sum(p[x] * w.N[x] for x in range(NS))
    ev["Nbar"] = Hp
    pic = w.pi_c()
    ev["H"] = Hp - sum(pic[x] * w.N[x] for x in range(NS))
    ev["rho"] = (J + K) / (K * (w.bc + 1))
    ev["Hwarm"] = w.phi(ev["rho"]) - w.phi(w.r0)
    ev["E"] = None if J == 0 else ev["H"] / ev["Hwarm"]
    ev["aim"] = None if J == 0 else aimJ / J
    # rule agreement
    tot = fmpq(0)
    agr = fmpq(0)
    fnp = fmpq(0)
    for i, (a, b, e) in enumerate(w.pairs):
        wt = p[a] + p[b]
        tot += wt
        fuelled = chain.u[i] >= HALF_UMAX
        paying = w.r[i] > w.bc
        if fuelled == paying:
            agr += wt
        if fuelled and not paying:
            fnp += wt
    ev["agreement"] = agr / tot
    ev["fuelled_nonpaying"] = fnp / tot
    ev["pairweight_total"] = tot
    return ev


def theta_exponent(bc, bh):
    return fmpq(bc - bh, bc + bh * 4)


def above_theta(r, bc, ex):
    """r > bc^(num/den)  <=>  r^den > bc^num, exact integers."""
    num, den = int(ex.p), int(ex.q)
    return int(r.p) ** den > bc ** num * int(r.q) ** den


def theta_agreement(world, chain):
    ex = theta_exponent(world.bc, world.bh)
    tot = fmpq(0)
    agr = fmpq(0)
    p = chain.p
    for i, (a, b, e) in enumerate(world.pairs):
        wt = p[a] + p[b]
        tot += wt
        if (chain.u[i] >= HALF_UMAX) == above_theta(world.r[i], world.bc, ex):
            agr += wt
    return agr / tot


# ---------------------------------------------------------------- rounding / parsing
def round_dec(x, d):
    """Exact half-even rounding of a rational to d decimals; also flags near ties."""
    s = 10 ** d
    y = x * s
    fl = int(y.floor())
    frac = y - fl
    if frac > fmpq(1, 2) or (frac == fmpq(1, 2) and fl % 2 == 1):
        fl += 1
    near = abs(frac - fmpq(1, 2)) < fmpq(1, 10 ** 9)
    neg = fl < 0
    fl = abs(fl)
    txt = str(fl).rjust(d + 1, "0")
    txt = txt[:-d] + "." + txt[-d:] if d else txt
    return ("-" if neg else "") + txt, near


def dec(s):
    """Decimal / scientific string -> exact fmpq."""
    m = re.fullmatch(r"([+-]?)(\d+)(?:\.(\d*))?(?:e([+-]?\d+))?", s)
    sign, ip, fp, ex = m.groups()
    fp = fp or ""
    v = fmpq(int(ip + fp), 10 ** len(fp))
    if ex:
        e = int(ex)
        v = v * fmpq(10) ** e if e >= 0 else v / fmpq(10) ** (-e)
    return -v if sign == "-" else v


def enclosure(s):
    m = re.fullmatch(r"\[(\S+) \+/- (\S+)\]", s)
    mid, rad = dec(m.group(1)), dec(m.group(2))
    return mid - rad, mid + rad


def form_in(f, base, lo, hi):
    """Certified lo < f / ln(base) < hi (exact rational bounds)."""
    prec = 256
    while prec <= 8192:
        old = ctx.prec
        ctx.prec = prec
        v = form_arb(f, prec) / arb(base).log()
        a = (v - arb(lo)) > 0 and (arb(hi) - v) > 0
        b = (v - arb(lo)) < 0 or (arb(hi) - v) < 0
        ctx.prec = old
        if a:
            return True
        if b:
            return False
        prec *= 2
    return False


def value_str(f, base):
    old = ctx.prec
    ctx.prec = 256
    v = form_arb(f, 256) / arb(base).log()
    s = v.str(12, radius=False)
    ctx.prec = old
    return s


def parse_report(text):
    rep = {"worlds": {}, "gates": {}, "controls": {}, "hyp": {}, "h3w": {}, "theta": {}}
    cur = None
    for line in text.splitlines():
        m = re.match(r"^WORLD (W\d) N=(\d+) tau=(\d+) b_c=(\d+) b_h=(\d+) v=(\d+)$", line)
        if m:
            cur = m.group(1)
            rep["worlds"][cur] = {"params": tuple(int(g) for g in m.groups()[1:]), "agents": {}}
            continue
        m = re.match(r"^  (\S.*?)\s+key=(blind|local|full)\s+(.*)$", line)
        if m and cur:
            fields = dict(re.findall(r"(\w+)=(\[[^\]]*\]|\S+)", m.group(3)))
            fields["key"] = m.group(2)
            rep["worlds"][cur]["agents"][m.group(1)] = fields
            continue
        m = re.match(r"^  reported: RECEIPT-full agreement with theta_R=(\S+): (\S+)$", line)
        if m:
            rep["theta"][cur] = (m.group(1), m.group(2))
            continue
        m = re.match(r"^  (G\d\d) (PASS|FAIL) \((\d+)/(\d+)\)$", line)
        if m:
            rep["gates"][m.group(1)] = (m.group(2), int(m.group(3)), int(m.group(4)))
            continue
        m = re.match(r"^  FIRST FAILURE: (.*)$", line)
        if m:
            rep["first_failure"] = m.group(1)
            continue
        m = re.match(r"^  (C\d\d) .*: failed first at (\S+) \(expected (\S+)\) (PASS|FAIL)$", line)
        if m:
            rep["controls"][m.group(1)] = (m.group(2), m.group(3), m.group(4))
            continue
        m = re.match(r"^  H3 (W\d): fuelled non-paying weight=(\S+)$", line)
        if m:
            rep["h3w"][m.group(1)] = m.group(2)
            continue
        m = re.match(r"^  (H\d) (SUPPORTED|REFUTED|UNDECIDED) failing=(.*)$", line)
        if m:
            rep["hyp"][m.group(1)] = (m.group(2), m.group(3))
            continue
        m = re.match(r"^OVERALL (\S+)$", line)
        if m:
            rep["overall"] = m.group(1)
        if line.startswith("SLOT READING (G12):"):
            rep["slot"] = line
    return rep


# ---------------------------------------------------------------- section 6 frozen record
MULT_TEXT = {
    3: """1:60, 13/12:24, 9/8:12, 32/27:24, 39/32:24, 21/17:24, 32/25:12, 9/7:24, 25/19:24, 4/3:24,
         19/14:12, 18/13:24, 38/27:12, 13/9:12, 3/2:24, 32/21:24, 25/16:24, 27/17:12, 13/8:24, 64/39:12,
         28/17:12, 7/4:24, 38/21:24, 50/27:12, 2:24, 27/13:24, 17/8:24, 13/6:24, 9/4:24, 32/13:24, 21/8:24,
         8:24, 12:24, 13:24, 16:12, 18:12""",
    4: """1:60, 34/29:24, 34/27:24, 14/11:12, 55/41:24, 139/102:24, 128/89:12, 81/55:24, 40/27:12, 89/60:24,
         44/29:24, 30/19:12, 28/17:24, 139/81:12, 89/51:24, 29/16:24, 256/139:12, 76/41:12, 102/55:24,
         55/29:24, 81/41:12, 17/8:24, 24/11:24, 178/81:12, 51/22:24, 81/34:24, 139/56:24, 41/16:24, 81/29:24,
         3:24, 55/16:24, 16:24, 29:24, 34:24, 44:12, 56:12""",
}


def fq(s):
    a, _, b = s.partition("/")
    return fmpq(int(a), int(b or 1))


def parse_mult(t):
    d = {}
    for item in t.replace("\n", " ").split(","):
        r, c = item.strip().split(":")
        d[fq(r)] = int(c)
    return d


W1_WINDOW = "3/2, 32/21, 25/16, 27/17, 13/8, 64/39, 28/17, 7/4, 38/21, 50/27, 2, 27/13, 17/8, 13/6, 9/4, 32/13, 21/8"
WINDOWS = {
    "W1": W1_WINDOW,
    "W2": "39/32, 21/17, 32/25, 9/7, 25/19, 4/3, 19/14, 18/13, 38/27, 13/9, " + W1_WINDOW,
    "W3": "21/8, 8",
    "W4": "8, 12, 13, 16, 18",
    "W5": "16",
    "W6": "34, 44, 56",
}
FROZEN = {
    # paying count, paying r set (None = not stated), max r, local ambiguous keys (count of 144)
    "W1": dict(count=96, rset="8, 12, 13, 16, 18", maxr=18, amb=0, theta="1.45", nbar="97/18", r0="1/6"),
    "W2": dict(count=96, rset=None, maxr=18, amb=0, theta="1.21", nbar="97/18", r0="1/6"),
    "W3": dict(count=72, rset="12, 13, 16, 18", maxr=18, amb=12, theta="2.47", nbar="853/250", r0="1/10",
               amb_np_r=8, amb_np_share="0.810"),
    "W4": dict(count=0, rset="", maxr=18, amb=0, theta="6.86", nbar="6388/3087", r0="1/21"),
    "W5": dict(count=72, rset="29, 34, 44, 56", maxr=56, amb=12, theta="4.91", nbar="8156/2401", r0="1/21",
               amb_np_r=16, amb_np_share="0.907"),
    "W6": dict(count=0, rset="", maxr=56, amb=0, theta="32.86", nbar="24466996/13845841", r0="1/61"),
}
POSTHOC = {"W1": "21/8", "W2": "21/8", "W3": "8", "W5": "16"}


def fset(s):
    return set() if not s else {fq(x.strip()) for x in s.split(",")}


def arb_round_str(x, d, prec=256):
    """Certified d-decimal rounding of a real enclosed by arb x (None if ambiguous)."""
    z = (x * 10 ** d + arb(fmpq(1, 2))).floor().unique_fmpz()
    if z is None:
        return None
    txt = str(int(z)).rjust(d + 1, "0")
    return txt[:-d] + "." + txt[-d:]


def theta_value(bc, bh, prec=256):
    old = ctx.prec
    ctx.prec = prec
    ex = theta_exponent(bc, bh)
    v = (arb(ex) * arb(bc).log()).exp()
    ctx.prec = old
    return v


def g06_world(world, frozen, verbose=False):
    """All section 6 combinatorics for one world; returns list of (check, ok)."""
    w = world
    res = []
    pay = [i for i in range(len(w.pairs)) if w.r[i] > w.bc]
    res.append(("paying count", len(pay) == frozen["count"]))
    if frozen["rset"] is not None:
        res.append(("paying ratio set", {w.r[i] for i in pay} == fset(frozen["rset"])))
    rs = set(w.r)
    res.append(("max build ratio", max(rs) == frozen["maxr"]))
    res.append(("36 distinct build ratios", len(rs) == 36))
    res.append(("no ratio equals b_c", all(r != w.bc for r in rs)))
    res.append(("all ratios >= 1", all(r >= 1 for r in rs)))
    mult = {}
    for r in w.r:
        mult[r] = mult.get(r, 0) + 1
    res.append(("multiplicity table tau=%d" % w.tau, mult == parse_mult(MULT_TEXT[w.tau])))
    # local ambiguity
    li = w.keyindex["local"]
    members = {}
    for i in range(len(w.pairs)):
        members.setdefault(li[i], []).append(i)
    holding = {k for k, m in members.items() if any(w.r[i] > w.bc for i in m)}
    amb = {k for k, m in members.items()
           if any(w.r[i] > w.bc for i in m) and any(w.r[i] <= w.bc for i in m)}
    res.append(("local keys = 144", len(members) == 144))
    res.append(("ambiguous local keys", len(amb) == frozen["amb"]))
    if frozen["amb"]:
        res.append(("ambiguous keys are all keys holding a paying pair", amb == holding))
        res.append(("each ambiguous key has 8 members, 2 non-paying at r=%d" % frozen["amb_np_r"],
                    all(len(members[k]) == 8 and
                        sorted(w.r[i] for i in members[k] if w.r[i] <= w.bc) == [fmpq(frozen["amb_np_r"])] * 2
                        for k in amb)))
        pic = w.pi_c()
        tot = fmpq(0)
        np_ = fmpq(0)
        for k in amb:
            for i in members[k]:
                a, b, e = w.pairs[i]
                wt = pic[a] + pic[b]
                tot += wt
                if w.r[i] <= w.bc:
                    np_ += wt
        s, near = round_dec(np_ / tot, 3)
        res.append(("non-paying pi_c share of ambiguous keys = %s (computed %s)" % (frozen["amb_np_share"], s),
                    s == frozen["amb_np_share"]))
    # every paying pair is a build at an isolated traveller's island
    res.append(("paying pairs at isolated traveller", all(
        not any((w.pairs[i][0] // NI) >> k & 1 for k in range(K) if w.pairs[i][0] % NI in EDGES[k])
        for i in pay)))
    # receipt windows
    ex = theta_exponent(w.bc, w.bh)
    win = {r for r in rs if above_theta(r, w.bc, ex) and r < w.bc}
    res.append(("receipt window", win == fset(WINDOWS[w.name])))
    th = theta_value(w.bc, w.bh)
    res.append(("theta_R printed %s" % frozen["theta"], arb_round_str(th, 2) == frozen["theta"]))
    if w.name in POSTHOC:
        mx = max(win)
        old = ctx.prec
        ctx.prec = 256
        ok = mx == fq(POSTHOC[w.name]) and (arb(mx) - arb(fmpq(18, 10)) * th) > 0
        ctx.prec = old
        res.append(("post-hoc: max window ratio %s >= 1.8 theta_R" % POSTHOC[w.name], ok))
    return res


# ---------------------------------------------------------------- gates on one table
def g01(world, chain):
    """detailed balance of WEATHER/WALK with pi_c, and pi_c stationary under them."""
    pic = world.pi_c()
    net = [fmpq(0)] * NS
    for kind, x, y, kxy, kyx, info in chain.ch:
        if kind == "F":
            continue
        if pic[x] * kxy != pic[y] * kyx:
            return False
        f = pic[x] * kxy - pic[y] * kyx
        net[x] -= f
        net[y] += f
    return all(v == 0 for v in net)


def g02(world, chain, table, cls):
    ki = world.keyindex[cls]
    for kind, x, y, kxy, kyx, info in chain.ch:
        if kind != "F":
            continue
        u = table[ki[info]]
        if kxy != u or kyx != chain.bh * u:
            return False
        if not (0 <= u <= UMAX) or (u * Q24).q != 1:
            return False
    return True


def g03(chain):
    p = chain.p
    if not all(v > 0 for v in p) or sum(p) != 1:
        return False
    net = [fmpq(0)] * chain.n
    for c, (kind, x, y, kxy, kyx, info) in zip(chain.cur, chain.ch):
        net[x] -= c
        net[y] += c
    return all(v == 0 for v in net)


def g04(ev, receipts_as_net=False):
    a = ev["receipt_flow"] if receipts_as_net else ev["SdotA"]
    return not form_add(a, ev["SdotU"])


def g05(ev, bc, bh):
    return ev["Jsum_identity"] and ev["sigma_formal"] and bc > bh


def g07(world, chain, ev):
    rho = fmpq(2, 2 + world.bc + world.bh)
    for x in range(NS):
        k = bin(x // NI).count("1")
        if chain.p[x] != rho ** k * (1 - rho) ** (K - k) / NI:
            return False
    return (ev["rho"] == rho and ev["H"] == world.phi(ev["rho"]) - world.phi(world.r0)
            and ev["E"] == 1)


def g08(tau, bc, bh, v, balpha):
    """Reference world N=2, one bridge, fuel u=1."""
    edges = [(0, 1)]
    n = 4  # index G*2 + pos
    Nv = walk_counts(2, edges, [], tau) + walk_counts(2, edges, [0], tau)
    rates = [dict() for _ in range(n)]
    ch = []
    for pos in (0, 1):
        a, b = pos, 2 + pos
        ch.append(("W", a, b, fmpq(1), fmpq(bc)))
        ch.append(("F", a, b, fmpq(1), fmpq(bh)))
    ch.append(("K", 2, 3, fmpq(v), fmpq(v)))
    for kind, x, y, kxy, kyx in ch:
        rates[x][y] = rates[x].get(y, fmpq(0)) + kxy
        rates[y][x] = rates[y].get(x, fmpq(0)) + kyx
    p = stationary(n, rates)
    J = fmpq(0)
    sA = {}
    for kind, x, y, kxy, kyx in ch:
        if kind == "F":
            c = p[x] * kxy - p[y] * kyx
            J += c
            sA[Nv[x]] = sA.get(Nv[x], fmpq(0)) - c
            sA[Nv[y]] = sA.get(Nv[y], fmpq(0)) + c
    lhs = form_add(form_from_weights(sA), logvec(balpha), -J)
    rhs = form_add(form_scale(logvec(2), J * tau), logvec(bc), -J)
    eq = not form_add(lhs, rhs, fmpq(-1))
    # section 2 extras (decided only with the declared price): isolated-channel current and sign rule
    pa = p[0] + p[1]
    iso = J == pa * (bc - bh) / (bc + bh)
    sign_rule = (form_sign(lhs) > 0) == (2 ** tau > bc) if balpha == bc else True
    return eq and iso and sign_rule and Nv == [1, 1, 2 ** tau, 2 ** tau]


# ---------------------------------------------------------------- main run
def main():
    bsha = hashlib.sha256(open(BOUNDARY, "rb").read()).hexdigest()
    rtxt = open(REPORT, "rb").read()
    rsha = hashlib.sha256(rtxt).hexdigest()
    out("CAL-THAIM-1 independent check (from the frozen boundary and recorded outputs only)")
    out(f"boundary sha256 {bsha} {'frozen value' if bsha == BOUNDARY_SHA else 'MISMATCH'}")
    out(f"report   sha256 {rsha} {'recorded value' if rsha == REPORT_SHA else 'MISMATCH'}")
    tally("input hashes", bsha == BOUNDARY_SHA, "boundary hash")
    tally("input hashes", rsha == REPORT_SHA, "report hash")
    rep = parse_report(rtxt.decode("utf-8"))
    out("arithmetic: exact rationals; logs expanded over primes (flint fmpz.factor), signs from certified arb")
    out("steady states: own multi-modular solver + exact verification")
    out()

    worlds = {}
    results = {}  # (world, agent) -> dict
    gate_checks = {g: [] for g in ["G01", "G02", "G03", "G04", "G05", "G06", "G07", "G08",
                                   "G11", "G12", "G13"]}

    for (wn, tau, bc, bh, v) in WORLDS:
        w = World(wn, tau, bc, bh, v)
        worlds[wn] = w
        rw = rep["worlds"].get(wn)
        out(f"WORLD {wn} N={NI} tau={tau} b_c={bc} b_h={bh} v={v} "
            f"keys blind/local/full={w.nkeys['blind']}/{w.nkeys['local']}/{w.nkeys['full']} "
            f"fuel pairs={len(w.pairs)} walk pairs={len(w.walks)}")
        tally("world parameters", rw is not None and rw["params"] == (NI, tau, bc, bh, v), f"{wn} header")
        for agent, cls in AGENTS:
            path = table_file(wn, agent)
            table, raw, canon_ok, tsha = read_table(path)
            ki = w.keyindex[cls]
            u = [table[ki[i]] for i in range(len(w.pairs))]
            ch = Chain(w, u)
            ch.solve()
            ev = evaluate(w, ch)
            res = dict(ev=ev, chain=ch, table=table, cls=cls)
            results[(wn, agent)] = res
            # per-table gates
            gate_checks["G01"].append(((wn, agent), g01(w, ch)))
            gate_checks["G02"].append(((wn, agent), len(table) == w.nkeys[cls] and g02(w, ch, table, cls)))
            gate_checks["G03"].append(((wn, agent), g03(ch)))
            gate_checks["G04"].append(((wn, agent), g04(ev)))
            gate_checks["G05"].append(((wn, agent), g05(ev, bc, bh)))
            ra = rw["agents"].get(agent, {}) if rw else {}
            # G12: canonical text regenerates the evaluated rate of every fuel pair
            regen = read_table(path)[0]
            g12 = (canon_ok and len(table) == w.nkeys[cls]
                   and all(regen[ki[i]] == ch.ch[len(w.pairs) + len(w.walks) + i][3]
                           for i in range(len(w.pairs))))
            if agent == "ORACLE":
                g12 = g12 and all(u[i] == (UMAX if w.r[i] > bc else 0) for i in range(len(w.pairs)))
            if agent == "LOCAL":
                g12 = g12 and all(u[i] == (1 if w.touch[i] else 0) for i in range(len(w.pairs)))
            if agent == "GLOBAL":
                g12 = g12 and all(x == 1 for x in u)
            gate_checks["G12"].append(((wn, agent), g12))
            res["sha"] = tsha
            # print own values
            Rn = value_str(ev["Fnet"], bc)
            Rr = value_str(ev["Freceipt"], bc)
            Ew = "none" if ev["E"] is None else round_dec(ev["E"], 4)[0]
            Am = "none" if ev["aim"] is None else round_dec(ev["aim"], 4)[0]
            out(f"  {agent:<17} key={cls:<5} J={round_dec(ev['J'], 6)[0]} resid={round_dec(ev['resid'], 6)[0]} "
                f"<|G|>={round_dec(ev['G'], 6)[0]} H={round_dec(ev['H'], 6)[0]} rho*={round_dec(ev['rho'], 6)[0]} "
                f"H_warm={round_dec(ev['Hwarm'], 6)[0]} E={Ew} aim={Am} agreement={round_dec(ev['agreement'], 4)[0]} "
                f"sign(R_net)={form_sign(ev['Fnet']):+d} R_net={Rn} R_receipt={Rr} "
                f"R_world=-J:{not form_add(ev['Fworld'], logvec(bc), ev['J'])}")
            # comparison with the report
            cmp = []
            if not ra:
                tally("report rows", False, f"{wn} {agent} missing in report")
                continue
            cmp.append(("key class", tally("key class", ra["key"] == cls, f"{wn} {agent} key")))
            for fld, val, d in [("J", ev["J"], 6), ("residual_J", ev["resid"], 6), ("H", ev["H"], 6),
                                ("E", ev["E"], 4), ("aim", ev["aim"], 4), ("agreement", ev["agreement"], 4)]:
                if val is None:
                    ok = ra.get(fld) == "none"
                else:
                    s, near = round_dec(val, d)
                    ok = ra.get(fld) == s
                    if near:
                        AMBIGUITIES.append(f"{wn} {agent} {fld} within 1e-9 of a rounding tie")
                        ok = ok or abs(dec(ra.get(fld)) - val) <= fmpq(1, 2 * 10 ** d)
                cmp.append((fld, tally(fld, ok, f"{wn} {agent} {fld} mine={val if val is None else round_dec(val, d)[0]} report={ra.get(fld)}")))
            for fld, F in [("R_net", ev["Fnet"]), ("R_receipt", ev["Freceipt"])]:
                rv = ra.get(fld)
                if rv == "0":
                    ok = not F
                elif rv and rv.startswith("["):
                    lo, hi = enclosure(rv)
                    ok = form_in(F, bc, lo, hi)
                else:
                    ok = False
                cmp.append((fld, tally(fld, ok, f"{wn} {agent} {fld} mine={value_str(F, bc)} report={rv}")))
            # sigma = floor / eta consistency of the recorded enclosures, sigma = J ln(b_c/b_h)
            fl, et = ra.get("floor"), ra.get("eta")
            sig = form_add(form_scale(logvec(bc), ev["J"]), logvec(bh), -ev["J"])
            if fl == "0" and et == "none":
                ok = not sig and ev["J"] == 0
            else:
                flo, fhi = enclosure(fl)
                elo, ehi = enclosure(et)
                ok = elo > 0 and flo > 0 and _between(sig, flo / ehi, fhi / elo)
            cmp.append(("floor/eta", tally("sigma = floor/eta (recorded enclosures)", ok, f"{wn} {agent} floor={fl} eta={et}")))
            cmp.append(("table sha", tally("table sha256", ra.get("table_sha256") == tsha, f"{wn} {agent} table sha")))
            bad = [c for c, s in cmp if s != "agree"]
            out(f"      vs report: {len(cmp) - len(bad)}/{len(cmp)} agree" + (f"; DISAGREE {bad}" if bad else ""))
        # receipt theta_R agreement
        th = theta_value(bc, bh)
        thstr = arb_round_str(th, 4)
        tagr = round_dec(theta_agreement(w, results[(wn, "RECEIPT-full")]["chain"]), 4)[0]
        out(f"  RECEIPT-full agreement with theta_R={thstr}: {tagr}")
        rt = rep["theta"].get(wn)
        tally("theta_R and receipt agreement", rt is not None and rt[0] == thstr, f"{wn} theta_R")
        tally("theta_R and receipt agreement", rt is not None and rt[1] == tagr, f"{wn} theta agreement")
        out()

    # per-world gates
    for wn, tau, bc, bh, v in WORLDS:
        w = worlds[wn]
        checks = g06_world(w, FROZEN[wn])
        for name, ok in checks:
            if not ok:
                out(f"  G06 {wn} failing check: {name}")
        gate_checks["G06"].append((wn, all(ok for _, ok in checks)))
        out(f"  G06 {wn}: {sum(ok for _, ok in checks)}/{len(checks)} section 6 checks hold")
        rg = results[(wn, "GLOBAL")]
        gate_checks["G07"].append((wn, g07(w, rg["chain"], rg["ev"])))
        gate_checks["G08"].append((wn, g08(tau, bc, bh, v, bc)))
        pic = w.pi_c()
        nbar = sum(pic[x] * w.N[x] for x in range(NS))
        gate_checks["G11"].append((wn, nbar == fq(FROZEN[wn]["nbar"]) and w.r0 == fq(FROZEN[wn]["r0"])
                                   and w.phi(w.r0) == nbar))
        ew, el = results[(wn, "WORLD-full")]["ev"], results[(wn, "LOCAL")]["ev"]
        gate_checks["G13"].append((wn, ew["J"] * 20 <= el["J"]
                                   and not form_add(ew["Fworld"], logvec(bc), ew["J"])))
    out()

    out("GATES (mine vs report)")
    first = None
    for g in ["G01", "G02", "G03", "G04", "G05", "G06", "G07", "G08", "G09", "G10", "G11", "G12", "G13"]:
        rg = rep["gates"].get(g)
        if g in ("G09", "G10"):
            out(f"  {g} not reproducible from boundary and tables (needs training / float rates); report {rg}")
            continue
        cks = gate_checks[g]
        npass = sum(ok for _, ok in cks)
        verdict = "PASS" if npass == len(cks) else "FAIL"
        if verdict == "FAIL" and first is None:
            first = f"{g} {[k for k, ok in cks if not ok][0]}"
        st = tally("gates", rg == (verdict, npass, len(cks)), f"{g} mine={verdict} {npass}/{len(cks)} report={rg}")
        out(f"  {g} {verdict} ({npass}/{len(cks)})  report {rg[0]} ({rg[1]}/{rg[2]})  {st}")
    first = first or "none"
    st = tally("gates", rep.get("first_failure") == first, f"first failure mine={first} report={rep.get('first_failure')}")
    out(f"  FIRST FAILURE: {first}  report {rep.get('first_failure')}  {st}")
    out(f"  slot reading line printed in report: {tally('gates', 'slot' in rep, 'slot reading missing')}")
    out()

    # controls
    out("CONTROLS (witness W1 GLOBAL; G08 on reference world with W1 tau, stores, v)")
    w1 = worlds["W1"]
    _, t1, bc1, bh1, v1 = WORLDS[0]
    glob = [fmpq(1)] * len(w1.pairs)
    expected = {"C01": "G02", "C02": "G01", "C03": "G04", "C04": "G08", "C05": "G05", "C06": "G06"}
    for cname in ["C01", "C02", "C03", "C04", "C05", "C06"]:
        bh = bc1 if cname == "C05" else bh1
        wctl = w1 if cname != "C05" else World("W1", t1, bc1, bc1, v1)
        balpha = bh1 if cname == "C04" else bc1
        frozen = dict(FROZEN["W1"])
        if cname == "C06":
            frozen["count"] = 97
        ch = Chain(wctl, glob, bh=bh, weather_mod=(cname == "C02"), demon=(cname == "C01"))
        fail = None
        seq = ["G01", "G02", "G03", "G04", "G05", "G06", "G07", "G08"]
        ev = None
        for g in seq:
            if g == "G01":
                ok = g01(wctl, ch)
            elif g == "G02":
                ok = g02(wctl, ch, [fmpq(1)] * 6, "blind")
            elif g == "G03":
                ch.solve()
                ok = g03(ch)
            elif g == "G04":
                ev = evaluate(wctl, ch, balpha)
                ok = g04(ev, receipts_as_net=(cname == "C03"))
            elif g == "G05":
                ok = g05(ev, bc1, bh)
            elif g == "G06":
                ok = all(o for _, o in g06_world(wctl, frozen))
            elif g == "G07":
                ok = g07(wctl, ch, ev)
            elif g == "G08":
                ok = g08(t1, bc1, bh, v1, balpha)
            if not ok:
                fail = g
                break
        verdict = "PASS" if fail == expected[cname] else "FAIL"
        rc = rep["controls"].get(cname)
        st = tally("controls", rc == (fail, expected[cname], verdict),
                   f"{cname} mine=({fail},{verdict}) report={rc}")
        out(f"  {cname}: failed first at {fail} (expected {expected[cname]}) {verdict}  report {rc}  {st}")
    out()

    # hypotheses
    out("HYPOTHESES")
    R = lambda wn, a: results[(wn, a)]["ev"]
    h = {}
    fails = []
    for (wn, *_r) in WORLDS:
        if not R(wn, "NET-full")["agreement"] >= fmpq(98, 100):
            fails.append(wn)
    h["H1"] = fails
    fails = []
    for wn in WARM:
        fo, fn = R(wn, "ORACLE")["Fnet"], R(wn, "NET-full")["Fnet"]
        ok = form_sign(fo) > 0 and form_sign(fn) > 0 and form_sign(form_add(fn, fo, fmpq(-98, 100))) >= 0
        if not ok:
            fails.append(wn)
    for wn in COLD:
        if not R(wn, "NET-full")["J"] * 20 <= R(wn, "LOCAL")["J"]:
            fails.append(wn)
    h["H2"] = fails
    fails = []
    decided3 = []
    for wn in WARM:
        fn = R(wn, "NET-full")["Fnet"]
        if form_sign(fn) <= 0:
            out(f"  H3 {wn}: undecided (R_net(NET-full) <= 0)")
            continue
        decided3.append(wn)
        wt = R(wn, "RECEIPT-full")["fuelled_nonpaying"]
        s = round_dec(wt, 4)[0]
        rr = rep["h3w"].get(wn)
        st = tally("H3 weights", rr == s, f"{wn} mine={s} report={rr}")
        out(f"  H3 {wn}: fuelled non-paying weight={s}  report {rr}  {st}")
        fr = R(wn, "RECEIPT-full")["Fnet"]
        ok = wt >= fmpq(1, 100) and form_sign(form_add(form_add(fn, fr, fmpq(-1)), fn, fmpq(-1, 100))) > 0
        if not ok:
            fails.append(wn)
    h["H3"] = fails
    fails = []
    for wn in WARM:
        en, el = R(wn, "NET-full")["E"], R(wn, "LOCAL")["E"]
        if not (en is not None and el is not None and en > el):
            fails.append(wn)
    h["H4"] = fails
    fails = []
    decided5 = []
    for wn in ["W1", "W2", "W3", "W5"]:
        fn = R(wn, "NET-full")["Fnet"]
        if form_sign(fn) <= 0:
            out(f"  H5 {wn}: undecided (R_net(NET-full) <= 0)")
            continue
        decided5.append(wn)
        diff = form_add(form_add(fn, R(wn, "NET-local")["Fnet"], fmpq(-1)), fn, fmpq(-1, 100))
        sg = form_sign(diff)
        ok = sg > 0 if wn in ("W3", "W5") else sg <= 0
        if not ok:
            fails.append(wn)
    h["H5"] = fails
    decided = {"H1": 6, "H2": 6, "H3": len(decided3), "H4": 4, "H5": len(decided5)}
    for hn in ["H1", "H2", "H3", "H4", "H5"]:
        if decided[hn] == 0:
            v = "UNDECIDED"
        else:
            v = "SUPPORTED" if not h[hn] else "REFUTED"
        fl = "none" if not h[hn] else str(h[hn])
        rh = rep["hyp"].get(hn)
        st = tally("hypotheses", rh == (v, fl), f"{hn} mine=({v},{fl}) report={rh}")
        out(f"  {hn} {v} failing={fl}  report {rh}  {st}")
    # supporting margins, printed exactly where useful
    for wn in WARM:
        en, el = R(wn, "NET-full")["E"], R(wn, "LOCAL")["E"]
        out(f"  H4 {wn}: E(NET-full)-E(LOCAL) = {round_dec(en - el, 6)[0]}")
    allpass = all(rep["gates"][g][0] == "PASS" for g in rep["gates"])
    out(f"  report OVERALL {rep.get('overall')}  (requires all gates PASS and all controls as stated)")
    ctl_ok = all(rep['controls'][c][2] == 'PASS' for c in rep['controls'])
    tally("gates", rep.get("overall") == ("PASS" if allpass and ctl_ok else "FAIL"), "overall line")
    out()

    out("SOLVER: primes used per steady state (min/max) %d/%d; common denominator bits (min/max) %d/%d" % (
        min(s[0] for s in SOLVER_STATS), max(s[0] for s in SOLVER_STATS),
        min(s[1] for s in SOLVER_STATS), max(s[1] for s in SOLVER_STATS)))
    out("NOT CHECKED: G09, G10 (training and float rates); tail (training); sigma_floor and eta individually "
        "(defined in the information-law note, not in the boundary) - only sigma = floor/eta is checked")
    out()
    out("COUNTS (agree/disagree)")
    ta = td = 0
    for cat in sorted(TALLY):
        a, d = TALLY[cat]
        ta += a
        td += d
        out(f"  {cat:<40} {a:>5} / {d}")
    out(f"  {'TOTAL':<40} {ta:>5} / {td}")
    for d in DISAGREEMENTS:
        out(f"  DISAGREEMENT {d}")
    for a in AMBIGUITIES:
        out(f"  AMBIGUITY {a}")
    out("OVERALL AGREE" if td == 0 else "OVERALL DISAGREE")
    print(f"runtime {time.time() - T0:.1f} s", file=sys.stderr)


def _between(f, lo, hi):
    """certified lo < f < hi for a linear form f in logs (exact rational bounds)."""
    prec = 256
    while prec <= 8192:
        v = form_arb(f, prec)
        old = ctx.prec
        ctx.prec = prec
        a = (v - arb(lo)) > 0 and (arb(hi) - v) > 0
        b = (v - arb(lo)) < 0 or (arb(hi) - v) < 0
        ctx.prec = old
        if a:
            return True
        if b:
            return False
        prec *= 2
    return False


if __name__ == "__main__":
    main()
