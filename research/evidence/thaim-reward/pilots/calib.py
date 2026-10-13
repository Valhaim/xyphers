import sys, time
sys.path.insert(0, __import__('os').path.join(__import__('os').path.dirname(__import__('os').path.abspath(__file__)), '..'))
import numpy as np, thaimworld as T, exact as X
analogues=[(3,3,9,2,1),(3,4,20,3,1),(3,3,10,2,1),(4,2,3,2,1),(3,3,5,2,1),(3,4,9,2,3)]
protocols=[(1500,0.05),(1500,0.2),(4000,0.2),(8000,0.2),(8000,0.5)]
for N,tau,bc,bh,v in analogues:
    w=T.World(N,tau,bc,bh,v)
    ro=X.evaluate(w,T.policy_oracle_exact(w)); Ro=float(ro['R_net'].mid())
    line=f"N={N} tau={tau} ({bc},{bh}) v={v}: R_oracle={Ro:+.5f}"
    for TT,lr0 in protocols:
        t0=time.time(); u,key,tail=T.train_tabular(w,'full','net',T=TT,lr0=lr0); r=X.evaluate(w,T.freeze(u))
        res=sum(float(x) for i,x in enumerate(T.freeze(u)) if not (w.ratio[i]>w.bc))
        line+=f" | T={TT},lr={lr0}: ratio={float(r['R_net'].mid())/Ro if Ro>0 else float('nan'):.4f} agr={float(r['agreement']):.3f} tail={tail:+.1e} ({time.time()-t0:.0f}s)"
    print(line, flush=True)
