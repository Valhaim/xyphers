"""Exact evaluation of a frozen rate table (boundary section 5).

Steady state in rationals (python-flint fmpq_mat). Every logarithmic quantity is a
finite sum of rational multiples of logarithms of positive integers (LogLin);
identities are decided over a pairwise coprime base, magnitudes by certified arb
enclosures at 256 bits. LogLin and the coprime base follow
research/physics/xypher-information-law/infolaw.py.
"""

from fractions import Fraction as Q
from math import gcd

import flint

from thaimworld import UMAX

flint.ctx.prec = 256


class LogLin:
    """sum of c * ln(n) over positive integers n, rational c."""

    def __init__(self, terms=None):
        self.t = dict(terms or {})

    def add_log(self, c, num, den=1):
        if c == 0:
            return self
        for n, s in ((num, c), (den, -c)):
            if n != 1:
                self.t[n] = self.t.get(n, Q(0)) + s
        return self

    def __add__(self, o):
        r = LogLin(self.t)
        for n, c in o.t.items():
            r.t[n] = r.t.get(n, Q(0)) + c
        return r

    def __neg__(self):
        return LogLin({n: -c for n, c in self.t.items()})

    def __sub__(self, o):
        return self + (-o)

    def scale(self, k):
        return LogLin({m: k * c for m, c in self.t.items()})

    def sign(self):
        """exact zero over a coprime base, else the sign of a certified enclosure (precision doubles)."""
        if self.is_zero():
            return 0
        prec = 256
        while prec <= 1 << 16:
            old = flint.ctx.prec
            flint.ctx.prec = prec
            try:
                e = self.enclose()
                if e > 0:
                    return 1
                if e < 0:
                    return -1
            finally:
                flint.ctx.prec = old
            prec *= 2
        raise AssertionError("sign not decided at 65536 bits")

    def is_zero(self):
        nums = [n for n, c in self.t.items() if c != 0]
        base = coprime_base(nums)
        acc = {b: Q(0) for b in base}
        for n in nums:
            r = n
            for b in base:
                while r % b == 0:
                    r //= b
                    acc[b] += self.t[n]
            if r != 1:
                raise AssertionError("coprime base failed")
        return all(v == 0 for v in acc.values())

    def enclose(self):
        s = flint.arb(0)
        for n, c in self.t.items():
            if c != 0:
                s += arbq(c) * flint.arb(n).log()
        return s


def coprime_base(nums):
    base, stack = [], [n for n in nums if n > 1]
    while stack:
        a = stack.pop()
        if a == 1:
            continue
        for i, b in enumerate(base):
            g = gcd(a, b)
            if g > 1:
                base.pop(i)
                stack.extend(c for c in (g, b // g, a // g) if c > 1)
                break
        else:
            base.append(a)
    return base


def arbq(x):
    return flint.arb(flint.fmpq(x.numerator, x.denominator))


def fq(x):
    return flint.fmpq(x.numerator, x.denominator)


def tq(x):
    return Q(int(x.p), int(x.q))


# ---------------------------------------------------------------- generator and steady state

def transitions(w, u, demon=None, weather_reads_agent=False):
    """all transitions (from, to, rate) as Fractions. demon/weather_reads_agent build controls C01/C02."""
    out = []
    for i, (a, b) in enumerate(zip(w.pa, w.pb)):
        wear = Q(w.bc) * ((1 + u[i]) if weather_reads_agent else 1)
        out.append((a, b, Q(1), "W", i))
        out.append((b, a, wear, "W", i))
        back = Q(w.bh) * u[i] if demon is None or i != demon else Q(w.bh) * u[i] + 1
        out.append((a, b, u[i], "F", i))
        out.append((b, a, back, "F", i))
    for i, (a, b) in enumerate(zip(w.wa, w.wb)):
        out.append((a, b, Q(w.v), "K", i))
        out.append((b, a, Q(w.v), "K", i))
    return out


def solve_exact(n, trans):
    rows = [[flint.fmpq(0)] * n for _ in range(n)]
    for a, b, k, _, _ in trans:
        if k == 0:
            continue
        kk = fq(k)
        rows[b][a] += kk
        rows[a][a] -= kk
    rows[-1] = [flint.fmpq(1)] * n
    A = flint.fmpq_mat(n, n, [x for r in rows for x in r])
    rhs = flint.fmpq_mat(n, 1, [0] * (n - 1) + [1])
    sol = A.solve(rhs)
    return [tq(sol[i, 0]) for i in range(n)]


def is_stationary(n, trans, p):
    net = [Q(0)] * n
    for a, b, k, _, _ in trans:
        f = p[a] * k
        net[a] -= f
        net[b] += f
    return all(x == 0 for x in net) and sum(p) == 1 and all(x > 0 for x in p)


# ---------------------------------------------------------------- warming rival

def phi_poly(w):
    """coefficients of Phi_tau(rho) = sum_G rho^|G| (1-rho)^(K-|G|) mean_pos N_tau(G, pos)."""
    by_size = {}
    for s in range(w.n):
        by_size[w.size[s]] = by_size.get(w.size[s], Q(0)) + Q(w.NT[s], w.N)
    return by_size


def phi(w, rho, poly=None):
    poly = poly or phi_poly(w)
    return sum(c * rho ** m * (1 - rho) ** (w.K - m) for m, c in poly.items())


def NT_pic(w):
    Z = sum(Q(1, w.bc ** w.size[s]) for s in range(w.n))
    return sum(Q(w.NT[s], w.bc ** w.size[s]) for s in range(w.n)) / Z


# ---------------------------------------------------------------- evaluation

def evaluate(w, u, alpha_from=None):
    """exact measures of the frozen table u (list of Fractions). alpha_from builds control C04."""
    trans = transitions(w, u)
    p = solve_exact(w.n, trans)
    r = {}
    r["stationary"] = is_stationary(w.n, trans, p)
    jf = [p[a] * u[i] - w.bh * u[i] * p[b] for i, (a, b) in enumerate(zip(w.pa, w.pb))]
    jw = [p[a] - w.bc * p[b] for a, b in zip(w.pa, w.pb)]
    jk = [w.v * (p[a] - p[b]) for a, b in zip(w.wa, w.wb)]
    J = sum(jf)
    Gm = sum(p[s] * w.size[s] for s in range(w.n))
    r["J"], r["Gmean"] = J, Gm
    r["J_identity"] = J == (w.bc + 1) * Gm - w.K
    S_A, S_U, rec = LogLin(), LogLin(), LogLin()
    for i, (a, b) in enumerate(zip(w.pa, w.pb)):
        S_A.add_log(jf[i], w.NT[b], w.NT[a])
        S_U.add_log(jw[i], w.NT[b], w.NT[a])
        rec.add_log(u[i] * p[a], w.NT[b], w.NT[a])
    for i, (a, b) in enumerate(zip(w.wa, w.wb)):
        S_U.add_log(jk[i], w.NT[b], w.NT[a])
    r["S_A"], r["S_U"], r["receipt_flow"] = S_A, S_U, rec
    r["balance_zero"] = (S_A + S_U).is_zero()
    lnb = flint.arb(alpha_from if alpha_from is not None else w.bc).log()
    r["R_net"] = S_A.enclose() / lnb - arbq(J)
    r["R_receipt"] = rec.enclose() / lnb - arbq(J)
    r["R_world"] = (S_A + S_U).enclose() / lnb - arbq(J)
    # R_net * ln b as a LogLin, for the price calibration (G08)
    r["R_net_times_ln"] = S_A - LogLin().add_log(J, alpha_from if alpha_from is not None else w.bc)
    # sigma_world as formal symbols: coefficient of L_c and L_h
    r["sigma_Lc"] = -sum(jw)   # weather builds take a packet from the cold store: flow -ln b_c per build
    r["sigma_Lh"] = -J         # fuel builds take a packet from the hot store
    H = sum(p[s] * w.NT[s] for s in range(w.n)) - NT_pic(w)
    r["H"] = H
    rho_star = (J + w.K) / (w.K * (w.bc + 1))
    r0 = Q(1, 1 + w.bc)
    poly = phi_poly(w)
    Hw = phi(w, rho_star, poly) - phi(w, r0, poly)
    r["H_warm"] = Hw
    r["E"] = H / Hw if Hw != 0 else None
    # floor over undriven pairs, and eta
    floor = flint.arb(0)
    for i, (a, b) in enumerate(zip(w.pa, w.pb)):
        x, y = p[a], p[b] * w.bc
        if x != y:
            floor += arbq(x - y) * (arbq(x) / arbq(y)).log()
    for i, (a, b) in enumerate(zip(w.wa, w.wb)):
        x, y = p[a] * w.v, p[b] * w.v
        if x != y:
            floor += arbq(x - y) * (arbq(x) / arbq(y)).log()
    r["floor"] = floor
    sigma = arbq(J) * arbq(Q(w.bc, w.bh)).log()
    r["sigma"] = sigma
    r["eta"] = floor / sigma if J > 0 else None
    # rule agreement, aim
    half = Q(UMAX, 2)
    wsum, agree = Q(0), Q(0)
    for i, (a, b) in enumerate(zip(w.pa, w.pb)):
        wt = p[a] + p[b]
        wsum += wt
        if (u[i] >= half) == (w.ratio[i] > w.bc):
            agree += wt
    r["agreement"] = agree / wsum
    r["aim"] = sum(jf[i] for i in range(len(jf)) if w.touch[i]) / J if J != 0 else None
    r["p"] = p
    return r
