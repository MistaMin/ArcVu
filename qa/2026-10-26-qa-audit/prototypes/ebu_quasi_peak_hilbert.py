import math
import numpy as np
sr=96000.0
rows=[(100,8,0.5),(10,6,0.5),(5,4,0.75),(1.5,-1,1.0),(0.5,-9,2.0)]
TAPS=127; HALF=TAPS//2
n=np.arange(-HALF,HALF+1)
h=np.where(n%2!=0, 2.0/(np.pi*np.where(n==0,1,n)), 0.0)*np.hamming(TAPS)
def envelope_power(x):
    # causal: quadrature from FIR, input delayed by HALF
    q=np.convolve(x,h,mode='full')[:len(x)]
    xd=np.concatenate([np.zeros(HALF),x])[:len(x)]
    return xd*xd+q*q
release=10**(-24/2.8/10/sr)
def ppm_reading(p, ts, tf):
    cs=1-math.exp(-1/(ts*sr)); cf=1-math.exp(-1/(tf*sr))
    ps=pf=0.0; out=np.empty(len(p))
    for i,v in enumerate(p):
        if v>ps: ps+=cs*(v-ps)
        else: ps*=release
        if ps>pf: pf+=cf*(ps-pf)
        else: pf*=release
        out[i]=pf
    return out
def burst_max(dur_ms, amp, ts, tf):
    nb=int(dur_ms*0.001*sr); tot=nb+int(0.25*sr)
    x=np.zeros(tot); x[:nb]=amp*np.sin(2*np.pi*5000*np.arange(nb)/sr)
    p=envelope_power(x)
    return 10*math.log10(ppm_reading(p,ts,tf).max())
