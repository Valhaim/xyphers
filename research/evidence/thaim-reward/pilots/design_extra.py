"""Design pilots on tau = 2 worlds only (boundary section 11): oracle rule, MLP vs tabular,
distance to the formula-I-form law, and the price scan. Rerun to regenerate design_extra.txt."""
import math, sys, os
import numpy as np
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import design_check as D, tab as TB
import xeval as X

def KL(p, q):
    m = p > 0
    return float(np.sum(p[m] * np.log(p[m] / q[m])))

for bc, bh, v in ((3, 2, 1), (3, 2, 4), (8, 2, 1)):
    w = X.World(4, bc, bh, v, R=2); D.setup(w)
    prof = w.dS_pair > math.log(bc)
    print(f"world N=4 tau=2 ({bc},{bh}) v={v}: share of builds with ratio > b_c = {prof.mean():.3f}")
    for umax in (1.0, 4.0, 16.0):
        u = umax * prof.astype(float); p, _ = w.solve(u); t = D.thaim_terms(w, p, u); m = w.measures(p, u)
        print(f"  ORACLE u_max={umax}: net THAIM={w.alpha*t['S_A']-t['J']:+.5f} J={t['J']:.4f} E={m['E']:.4f}")
    for obs in ("local", "full"):
        for kind in ("net", "receipt"):
            u, p = TB.train_tab(w, obs, kind); t = D.thaim_terms(w, p, u); m = w.measures(p, u)
            wt = p[w.pa] + p[w.pb]; agr = float(((u > 2.0) == prof)[wt > 1e-9].mean())
            print(f"  tabular {kind}/{obs}: net THAIM={w.alpha*t['S_A']-t['J']:+.4f} J={t['J']:.3f} E={m['E']:.3f} rule agreement={agr:.3f}")
    if bc == 3:
        piI = w.NR * w.pic; piI /= piI.sum()
        for c in (0.6, 0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 2.0, 3.0):
            u, p = TB.train_tab(w, "full", "net", c=c); m = w.measures(p, u)
            print(f"  price c={c}: E={m['E']:.4f} J={m['J']:.3f} KL(p||pi_I)={KL(p, piI):.4f}")
