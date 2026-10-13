import math, numpy as np
import design_check as D, xeval as X
def obs_keys(w, obs):
    G=w.Gidx[w.pa]; pos=w.pos[w.pa]; k=w.pk
    if obs=='full': key=(G*w.N+pos)*w.K+k
    elif obs=='local':
        X_=w.features('local')[w.pa]; code=((X_>0).astype(int)*(1<<np.arange(X_.shape[1]))).sum(1); key=code*w.K+k
    else: key=k.copy()
    _,inv=np.unique(key,return_inverse=True); return inv
def train_tab(w, obs, kind, umax=4.0, iters=1500, lr=0.05, u0=1.0, c=1.0):
    inv=obs_keys(w,obs); m=inv.max()+1; z=np.full(m, math.log(u0/(umax-u0))); mt=np.zeros(m); vt=np.zeros(m)
    a0=w.alpha; w.alpha=a0*c
    for it in range(1,iters+1):
        s=1/(1+np.exp(-z[inv])); u=umax*s; p,lu=w.solve(u); val,g,direct=D.value_grad(w,p,u,kind)
        du=w.adjoint_du(p,lu,g,u)+direct; gz=np.bincount(inv,weights=du*u*(1-s),minlength=m)
        mt=0.9*mt+0.1*gz; vt=0.999*vt+0.001*gz*gz
        z+=lr*0.1**(it/iters)*(mt/(1-0.9**it))/(np.sqrt(vt/(1-0.999**it))+1e-12)
    w.alpha=a0; u=umax/(1+np.exp(-z[inv])); p,_=w.solve(u); return u,p
