"""CAL-THAIM-1 world, rewards, and trainers (boundary sections 1-4).

World x = (G, pos) on N islands: WEATHER (build 1, wear b_c), WALK (rate v across
built bridges), FUEL pairs (build u, return b_h u, u read from the absent
endpoint's observation key). Open future S(x) = ln N_tau(G, pos).

Floats appear only in training. Exact evaluation lives in exact.py.
"""

import itertools
import math
import warnings
from fractions import Fraction as Q

import numpy as np
import scipy.sparse as sp
import scipy.sparse.linalg as spla

UMAX = 4
FREEZE_DENOM = 1 << 24

# numpy 2.0 with Accelerate on arm64 macOS raises spurious floating-point warnings inside
# matmul (documented in the frontier pilot); train_mlp asserts that every rate is finite.
warnings.filterwarnings("ignore", message=".*encountered in matmul")


class World:
    def __init__(self, N, tau, bc, bh, v):
        self.N, self.tau, self.bc, self.bh, self.v = N, tau, bc, bh, v
        self.edges = list(itertools.combinations(range(N), 2))
        K = self.K = len(self.edges)
        nG = self.nG = 1 << K
        self.n = nG * N
        self.G = [s // N for s in range(self.n)]
        self.pos = [s % N for s in range(self.n)]
        self.size = [bin(g).count("1") for g in self.G]
        self.NT = [self._walks(self.G[s], self.pos[s]) for s in range(self.n)]
        # fuel pairs (also the weather pairs): a = e absent, b = e present
        self.pa, self.pb, self.pk = [], [], []
        for g in range(nG):
            for p in range(N):
                for k in range(K):
                    if not g >> k & 1:
                        self.pa.append(g * N + p)
                        self.pb.append((g | 1 << k) * N + p)
                        self.pk.append(k)
        self.ratio = [Q(self.NT[b], self.NT[a]) for a, b in zip(self.pa, self.pb)]
        self.touch = [self.pos[a] in self.edges[k] for a, k in zip(self.pa, self.pk)]
        # walk pairs: same G, pos i <-> j across built bridge (i, j)
        self.wa, self.wb = [], []
        for g in range(nG):
            for k, (i, j) in enumerate(self.edges):
                if g >> k & 1:
                    self.wa.append(g * N + i)
                    self.wb.append(g * N + j)
        self.alpha = 1.0 / math.log(bc)
        self._float_static()

    def _walks(self, g, pos):
        adj = {v: {v} for v in range(self.N)}
        for k, (i, j) in enumerate(self.edges):
            if g >> k & 1:
                adj[i].add(j)
                adj[j].add(i)
        vec = [1] * self.N
        for _ in range(self.tau):
            vec = [sum(vec[j] for j in adj[i]) for i in range(self.N)]
        return vec[pos]

    # ------------------------------------------------------------ observation keys
    def keys(self, obs):
        """integer key per fuel pair, read from the absent endpoint (boundary 1.3)."""
        raw = []
        for a, k in zip(self.pa, self.pk):
            g, p = self.G[a], self.pos[a]
            if obs == "blind":
                raw.append((k,))
            elif obs == "local":
                inc = tuple(g >> kk & 1 for kk, e in enumerate(self.edges) if p in e)
                raw.append((p, inc, k))
            elif obs == "full":
                raw.append((a, k))
            else:
                raise ValueError(obs)
        index = {key: i for i, key in enumerate(sorted(set(raw)))}
        return np.array([index[key] for key in raw]), len(index)

    # ------------------------------------------------------------ float model
    def _float_static(self):
        self.fpa = np.array(self.pa)
        self.fpb = np.array(self.pb)
        self.fwa = np.array(self.wa, dtype=int)
        self.fwb = np.array(self.wb, dtype=int)
        self.fsize = np.array(self.size, dtype=float)
        self.S = np.log(np.array(self.NT, dtype=float))
        self.dS_pair = self.S[self.fpb] - self.S[self.fpa]
        self.dS_walk = self.S[self.fwb] - self.S[self.fwa] if len(self.wa) else np.zeros(0)
        fr = np.concatenate([self.fpa, self.fpb, self.fwa, self.fwb])
        to = np.concatenate([self.fpb, self.fpa, self.fwb, self.fwa])
        rt = np.concatenate([np.ones(len(self.fpa)), np.full(len(self.fpa), float(self.bc)),
                             np.full(len(self.fwa), float(self.v)), np.full(len(self.fwa), float(self.v))])
        self.U_fr, self.U_to, self.U_rt = fr, to, rt
        self.U_out = np.bincount(fr, weights=rt, minlength=self.n)

    def solve(self, u):
        n, bh = self.n, self.bh
        fr = np.concatenate([self.U_fr, self.fpa, self.fpb])
        to = np.concatenate([self.U_to, self.fpb, self.fpa])
        rt = np.concatenate([self.U_rt, u, bh * u])
        out = self.U_out + np.bincount(self.fpa, weights=u, minlength=n) + np.bincount(self.fpb, weights=bh * u, minlength=n)
        rows = np.concatenate([to, np.arange(n)])
        cols = np.concatenate([fr, np.arange(n)])
        vals = np.concatenate([rt, -out])
        keep = rows != n - 1
        rows = np.concatenate([rows[keep], np.full(n, n - 1)])
        cols = np.concatenate([cols[keep], np.arange(n)])
        vals = np.concatenate([vals[keep], np.ones(n)])
        A = sp.csc_matrix((vals, (rows, cols)), shape=(n, n))
        lu = spla.splu(A)
        e = np.zeros(n)
        e[-1] = 1.0
        return lu.solve(e), lu

    def adjoint_du(self, p, lu, g, u):
        """dF/du per fuel pair through p, given g = dF/dp (bordered-system adjoint)."""
        mu = lu.solve(g, trans="T")
        mu[-1] = 0.0
        return (mu[self.fpa] - mu[self.fpb]) * (p[self.fpa] - self.bh * p[self.fpb])

    # ------------------------------------------------------------ rewards (floats, training only)
    def reward(self, p, u, kind, c=1.0):
        """value, dF/dp, direct dF/du for kind in {net, receipt, world}."""
        a = self.alpha
        J = (self.bc + 1) * float(self.fsize @ p) - self.K
        gJ = (self.bc + 1) * self.fsize
        if kind == "world":
            return -J, -gJ, np.zeros_like(u)
        if kind == "net":
            # Sdot_A = -Sdot_U, Sdot_U a function of p only
            g = np.zeros(self.n)
            np.add.at(g, self.fpa, -c * a * self.dS_pair)
            np.add.at(g, self.fpb, c * a * self.bc * self.dS_pair)
            if len(self.fwa):
                np.add.at(g, self.fwa, -c * a * self.v * self.dS_walk)
                np.add.at(g, self.fwb, c * a * self.v * self.dS_walk)
            jw = p[self.fpa] - self.bc * p[self.fpb]
            jwalk = self.v * (p[self.fwa] - p[self.fwb]) if len(self.fwa) else np.zeros(0)
            S_U = float(jw @ self.dS_pair + jwalk @ self.dS_walk)
            return -c * a * S_U - J, g - gJ, np.zeros_like(u)
        if kind == "receipt":
            pos = np.maximum(0.0, self.dS_pair)
            val = a * float((u * p[self.fpa]) @ pos) - J
            g = np.zeros(self.n)
            np.add.at(g, self.fpa, a * u * pos)
            return val, g - gJ, a * p[self.fpa] * pos
        raise ValueError(kind)


# ---------------------------------------------------------------- trainers

def train_tabular(w, obs, kind, c=1.0, T=8000, lr0=0.2, u0=1.0, log_tail=100):
    """boundary section 4: one parameter per key, Adam per parameter, no random numbers."""
    key, m = w.keys(obs)
    z = np.full(m, math.log(u0 / (UMAX - u0)))
    mt = np.zeros(m)
    vt = np.zeros(m)
    ref = None
    for t in range(1, T + 1):
        s = 1.0 / (1.0 + np.exp(-z[key]))
        u = UMAX * s
        p, lu = w.solve(u)
        val, g, direct = w.reward(p, u, kind, c)
        if t == T - log_tail + 1:
            ref = val          # objective before the last log_tail updates
        du = w.adjoint_du(p, lu, g, u) + direct
        gz = np.bincount(key, weights=du * u * (1.0 - s), minlength=m)
        mt = 0.9 * mt + 0.1 * gz
        vt = 0.999 * vt + 0.001 * gz * gz
        z = z + lr0 * 0.1 ** (t / T) * (mt / (1 - 0.9 ** t)) / (np.sqrt(vt / (1 - 0.999 ** t)) + 1e-12)
    u = UMAX / (1.0 + np.exp(-z[key]))
    final = w.reward(w.solve(u)[0], u, kind, c)[0]
    return u, key, final - ref


class MLP:
    """The frontier pilot's network (xypher-eval/xeval.py): o -> tanh(8) -> K logits."""

    def __init__(self, d, h, K, rng, u0=2.0, w2std=0.1):
        self.P = {
            "W1": rng.normal(0, 1 / math.sqrt(d), (d, h)),
            "b1": np.zeros(h),
            "W2": rng.normal(0, w2std, (h, K)),
            "b2": np.full(K, math.log(u0 / (UMAX - u0))),
        }
        self.m = {k: np.zeros_like(v) for k, v in self.P.items()}
        self.s = {k: np.zeros_like(v) for k, v in self.P.items()}
        self.t = 0

    def forward(self, X):
        H1 = np.tanh(X @ self.P["W1"] + self.P["b1"])
        return H1 @ self.P["W2"] + self.P["b2"], (X, H1)

    def backward(self, cache, dout):
        X, H1 = cache
        g = {"W2": H1.T @ dout, "b2": dout.sum(0)}
        dZ = (dout @ self.P["W2"].T) * (1 - H1 ** 2)
        g["W1"] = X.T @ dZ
        g["b1"] = dZ.sum(0)
        return g

    def ascend(self, g, lr, b1=0.9, b2=0.999, eps=1e-8):
        self.t += 1
        for k in self.P:
            self.m[k] = b1 * self.m[k] + (1 - b1) * g[k]
            self.s[k] = b2 * self.s[k] + (1 - b2) * g[k] ** 2
            mh = self.m[k] / (1 - b1 ** self.t)
            sh = self.s[k] / (1 - b2 ** self.t)
            self.P[k] = self.P[k] + lr * mh / (np.sqrt(sh) + eps)


def mlp_features(w):
    onepos = np.eye(w.N)[np.array(w.pos)]
    bits = ((np.array(w.G)[:, None] >> np.arange(w.K)) & 1).astype(float)
    return np.hstack([2 * onepos - 1, 2 * bits - 1])


def train_mlp(w, kind, seed, T=8000, lr=0.03, hidden=8, log_tail=100):
    rng = np.random.default_rng(seed)
    X = mlp_features(w)
    net = MLP(X.shape[1], hidden, w.K, rng)
    pk = np.array(w.pk)
    ref = None
    for t in range(T):
        out, cache = net.forward(X)
        zz = out[w.fpa, pk]
        u = UMAX / (1.0 + np.exp(-zz))
        p, lu = w.solve(u)
        val, g, direct = w.reward(p, u, kind)
        if t == T - log_tail:
            ref = val          # objective before the last log_tail updates
        du = w.adjoint_du(p, lu, g, u) + direct
        dz = du * u * (1 - u / UMAX)
        dout = np.zeros((w.n, w.K))
        np.add.at(dout, (w.fpa, pk), dz)
        net.ascend(net.backward(cache, dout), lr * 0.1 ** (t / T))
    out, _ = net.forward(X)
    u = UMAX / (1.0 + np.exp(-out[w.fpa, pk]))
    if not np.all(np.isfinite(u)):
        raise FloatingPointError("MLP produced a non-finite rate")
    final = w.reward(w.solve(u)[0], u, kind)[0]
    return u, final - ref


# ---------------------------------------------------------------- fixed policies and freezing

def policy_local(w):
    return np.array([1.0 if t else 0.0 for t in w.touch])


def policy_global(w):
    return np.ones(len(w.pa))


def policy_oracle_exact(w):
    return [Q(UMAX) if r > w.bc else Q(0) for r in w.ratio]


def freeze(u):
    """round each float rate to the nearest multiple of 2^-24 (boundary section 4)."""
    return [Q(int(round(float(x) * FREEZE_DENOM)), FREEZE_DENOM) for x in u]


