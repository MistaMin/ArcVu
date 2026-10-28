# EBU quasi-peak prototypes (not in the plugin)

`ebu_quasi_peak_hilbert.py` simulates a Hilbert-envelope power detector at 96 kHz and compares
its 5 kHz burst responses with Tech 3205-E Table 2.

Best two-stage fit (127-tap FIR, stages 12 ms and 3.0 ms): error 1.46 x tolerance.
Results: 100 ms +8.00 (pass), 10 ms +5.42 (needs +6 ±0.5, fail), 5 ms +3.20 (needs +4 ±0.75, borderline fail),
1.5 ms -1.45 (needs -1 ±1, pass), 0.5 ms -6.09 (needs -9 ±2, fail).
A third fast stage (0.05-0.4 ms) did not change the 0.5 ms row, so the Hilbert envelope itself
spreads very short bursts too much. The plugin detector is unchanged by this prototype.
