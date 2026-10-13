import sys
sys.path.insert(0, __import__('os').path.join(__import__('os').path.dirname(__import__('os').path.abspath(__file__)), '..'))
from fractions import Fraction as Q
import thaimworld as T, exact as X
worlds=[(4,2,3,2,1),(4,2,8,2,1),(3,3,9,2,1),(3,4,20,3,1),(3,3,20,2,1),(3,4,60,2,2),(3,3,5,2,1)]
for N,tau,bc,bh,v in worlds:
    w=T.World(N,tau,bc,bh,v)
    th=bc**((bc-bh)/(bc+bh*T.UMAX))
    u,key,tail=T.train_tabular(w,'full','receipt',T=8000,lr0=0.2); fz=T.freeze(u); r=X.evaluate(w,fz)
    rn,_,_=T.train_tabular(w,'full','net',T=8000,lr0=0.2); rnet=X.evaluate(w,T.freeze(rn))
    rl=X.evaluate(w,[Q(x) for x in T.policy_local(w)])
    wsum=agree=Q(0)
    for i,(a,b) in enumerate(zip(w.pa,w.pb)):
        wt=r['p'][a]+r['p'][b]; wsum+=wt
        if (fz[i]>=2)==(float(w.ratio[i])>th): agree+=wt
    print(f"N={N} tau={tau} ({bc},{bh}) v={v}: theta_R={th:.3f} receipt-rule agreement={float(agree/wsum):.3f} "
          f"R_net(REC)={float(r['R_net'].mid()):+.4f} R_net(NET)={float(rnet['R_net'].mid()):+.4f} "
          f"J(REC)/J(NET)={float(r['J']/rnet['J']) if rnet['J']>0 else float('inf'):.1f} J(REC)/J_LOCAL={float(r['J']/rl['J']):.3f} tail={tail:+.1e}", flush=True)
