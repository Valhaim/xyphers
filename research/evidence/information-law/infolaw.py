"""Exact ledger of the information law for bipartite world-register jump processes.

A world state x and a register state m form the system state z = (x, m).
Three channel classes act on it (Information Law note, section 1):

  U  undriven world:  (x, m) <-> (x', m), rates w, w' that do not read m,
                      detailed balance with the world's own law pi_c;
  A  act:             (x, m) <-> (x', m), rates that may read x and m;
  R  register:        (x, m) <-> (x, m'), rates that may read x and m.

Every quantity of the ledger is a finite sum of rational coefficients times
logarithms of positive integers. Identities are decided exactly: all integers
are reduced to a pairwise coprime base, whose logarithms are linearly
independent over the rationals, and every coefficient must vanish.
Inequalities are decided by exact sign arguments or by certified arb
enclosures. No floating-point value decides anything.
"""

from fractions import Fraction as Q
from math import gcd

import flint

flint.ctx.prec = 256


# ---------------------------------------------------------------- log-linear forms

class LogLin:
    """sum of c * ln(n) over positive integers n, rational c."""

    __slots__ = ("t",)

    def __init__(self, terms=None):
        self.t = dict(terms or {})

    def add_log(self, c, num, den=1):
        """add c * ln(num / den) for positive integers num, den."""
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
        return LogLin({n: k * c for n, c in self.t.items()})

    def is_zero(self):
        """exact: True iff the form is identically zero over a coprime base."""
        reduced = reduce_to_base(self)
        return all(c == 0 for c in reduced.values())

    def enclose(self):
        s = flint.arb(0)
        for n, c in self.t.items():
            if c != 0:
                s += flint.arb(flint.fmpq(c.numerator, c.denominator)) * flint.arb(n).log()
        return s


def coprime_base(nums):
    base = []
    stack = [n for n in nums if n > 1]
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


def reduce_to_base(form):
    nums = [n for n, c in form.t.items() if c != 0]
    base = coprime_base(nums)
    out = {b: Q(0) for b in base}
    for n in nums:
        c = form.t[n]
        r = n
        for b in base:
            while r % b == 0:
                r //= b
                out[b] += c
        if r != 1:
            raise AssertionError("coprime base did not factor %d" % n)
    return out


# ---------------------------------------------------------------- the system

def frac(v):
    return v if isinstance(v, Q) else Q(v)


class System:
    """undriven: [(x, x2, w, w2)], applied in every register state;
    act: [(m, x, x2, a, a2)]; register: [(x, m, m2, r, r2)]; pi_c: {x: weight}."""

    def __init__(self, X, M, undriven, act, register, pi_c):
        self.X, self.M = list(X), list(M)
        self.states = [(x, m) for x in self.X for m in self.M]
        self.index = {z: i for i, z in enumerate(self.states)}
        self.pi_c = {x: frac(v) for x, v in pi_c.items()}
        self.undriven = [(x, x2, frac(w), frac(w2)) for x, x2, w, w2 in undriven]
        self.channels = []
        for x, x2, w, w2 in self.undriven:
            if self.pi_c[x] * w != self.pi_c[x2] * w2:
                raise ValueError("undriven pair %r-%r violates detailed balance" % (x, x2))
            for m in self.M:
                self.channels.append(("U", (x, m), (x2, m), w, w2))
        for m, x, x2, a, a2 in act:
            self.channels.append(("A", (x, m), (x2, m), frac(a), frac(a2)))
        for x, m, m2, r, r2 in register:
            self.channels.append(("R", (x, m), (x, m2), frac(r), frac(r2)))
        for cls, z, z2, k, k2 in self.channels:
            if k <= 0 or k2 <= 0:
                raise ValueError("every channel must be reciprocal with positive rates")
            if z == z2:
                raise ValueError("self loop")
            if cls in "UA" and z[1] != z2[1]:
                raise ValueError("world channel changes the register")
            if cls == "R" and z[0] != z2[0]:
                raise ValueError("register channel changes the world")
        self.P = None

    # -- exact steady state
    def solve(self):
        n = len(self.states)
        rows = [[flint.fmpq(0)] * n for _ in range(n)]
        for _, z, z2, k, k2 in self.channels:
            i, j = self.index[z], self.index[z2]
            kf, kb = flint.fmpq(k.numerator, k.denominator), flint.fmpq(k2.numerator, k2.denominator)
            rows[j][i] += kf
            rows[i][i] -= kf
            rows[i][j] += kb
            rows[j][j] -= kb
        rows[-1] = [flint.fmpq(1)] * n
        A = flint.fmpq_mat(n, n, [v for row in rows for v in row])
        b = flint.fmpq_mat(n, 1, [0] * (n - 1) + [1])
        sol = A.solve(b)
        P = {}
        for z, i in self.index.items():
            v = sol[i, 0]
            P[z] = Q(int(v.p), int(v.q))
        self.set_law(P)
        return self

    def set_law(self, P):
        if sum(P.values()) != 1 or any(v <= 0 for v in P.values()):
            raise ValueError("law must be positive and normalised")
        self.P = dict(P)
        D = 1
        for v in P.values():
            D = D * v.denominator // gcd(D, v.denominator)
        self.D = D
        self.n = {z: int(v * D) for z, v in P.items()}
        self.nx = {x: sum(self.n[(x, m)] for m in self.M) for x in self.X}
        self.nm = {m: sum(self.n[(x, m)] for x in self.X) for m in self.M}
        self.p = {x: Q(self.nx[x], D) for x in self.X}
        self.q = {m: Q(self.nm[m], D) for m in self.M}

    def is_stationary(self):
        out = {z: Q(0) for z in self.states}
        for _, z, z2, k, k2 in self.channels:
            j = self.P[z] * k - self.P[z2] * k2
            out[z] -= j
            out[z2] += j
        return all(v == 0 for v in out.values())

    # -- the ledger
    def ledger(self):
        if self.P is None:
            self.solve()
        L = {key: LogLin() for key in (
            "flow_U", "flow_A", "flow_R", "S_U", "S_A", "S_R", "I_U", "I_A", "I_R", "floor")}
        sign_ok = True
        currents = []
        for cls, z, z2, k, k2 in self.channels:
            u, v = self.P[z] * k, self.P[z2] * k2
            J = u - v
            currents.append((cls, z, z2, J))
            if (J > 0) != (u > v) or (J == 0) != (u == v):
                sign_ok = False
            # flow: J ln(k/k')
            L["flow_" + cls].add_log(J, k.numerator * k2.denominator, k2.numerator * k.denominator)
            # entropy production: J ln(P(z)k / (P(z')k'))
            L["S_" + cls].add_log(J, self.n[z] * k.numerator * k2.denominator,
                                  self.n[z2] * k2.numerator * k.denominator)
            # information: J [i(z') - i(z)], i(x,m) = ln(n_xm D / (n_x n_m))
            (x, m), (x2, m2) = z, z2
            L["I_" + cls].add_log(J, self.n[z2] * self.nx[x] * self.nm[m],
                                  self.n[z] * self.nx[x2] * self.nm[m2])
        floor_terms = []
        for x, x2, w, w2 in self.undriven:
            u, v = self.p[x] * w, self.p[x2] * w2
            jx = u - v
            if (jx > 0) != (u > v) or (jx == 0) != (u == v):
                sign_ok = False
            L["floor"].add_log(jx, w.numerator * w2.denominator * self.nx[x],
                               w2.numerator * w.denominator * self.nx[x2])
            floor_terms.append((x, x2, w, w2, jx))
        L["sigma_world"] = L["flow_U"] + L["flow_A"]
        L["sigma_reg"] = L["flow_R"]
        L["I_act"] = -L["I_A"]
        L["I_learn"] = L["I_R"]
        L["I_noise"] = -L["I_U"]
        self.L, self.currents, self.floor_terms, self.sign_ok = L, currents, floor_terms, sign_ok
        return L

    # -- certified log-sum gaps: I_noise and (for blind memory) -I_learn as sums of
    #    per-pair gaps sum_m (a_m - b_m) ln(a_m/b_m) - (A - B) ln(A/B) >= 0
    def logsum_gaps(self, cls):
        groups = {}
        for c, z, z2, k, k2 in self.channels:
            if c != cls:
                continue
            if cls == "U":
                key, member = (z[0], z2[0], k, k2), z[1]
            else:
                key, member = (z[1], z2[1], k, k2), z[0]
            groups.setdefault(key, []).append((self.P[z] * k, self.P[z2] * k2))
        worst = None
        for pairs in groups.values():
            A = sum(a for a, _ in pairs)
            B = sum(b for _, b in pairs)
            if all(a * B == b * A for a, b in pairs):
                continue  # equal ratios: the gap is exactly zero
            g = flint.arb(0)
            for a, b in pairs:
                g += arbq(a - b) * (arbq(a) / arbq(b)).log()
            g -= arbq(A - B) * (arbq(A) / arbq(B)).log()
            if not g > 0:
                raise AssertionError("log-sum gap not certified positive: %s" % g)
            worst = g if worst is None or g.mid() < worst.mid() else worst
        return worst


def arbq(v):
    return flint.arb(flint.fmpq(v.numerator, v.denominator))


def theorem_checks(S):
    """the four exact identities and the sign facts of the theorem, decided exactly."""
    L = S.ledger()
    out = {}
    out["stationary"] = S.is_stationary()
    out["(1) sigma_world = floor - I_act + S_A"] = (L["sigma_world"] - (L["floor"] - L["I_act"] + L["S_A"])).is_zero()
    out["(2) sigma_reg = I_learn + S_R"] = (L["sigma_reg"] - (L["I_learn"] + L["S_R"])).is_zero()
    out["(3) I_learn = I_act + I_noise"] = (L["I_learn"] - (L["I_act"] + L["I_noise"])).is_zero()
    out["(3') S_U = floor + I_noise"] = (L["S_U"] - (L["floor"] + L["I_noise"])).is_zero()
    out["(4) term signs exact"] = S.sign_ok
    S.logsum_gaps("U")  # raises unless every nonzero gap is certified positive
    out["(4) I_noise >= 0 certified"] = True
    return out
