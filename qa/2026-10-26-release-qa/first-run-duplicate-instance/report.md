# Plugin QA: VST3: ArcVu (Marcos Deida)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: ArcVu (Marcos Deida), 59 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1100x305, 0.6% animates by itself |
| every parameter changes the sound | FAIL | Mid side, Trim M, Trim S, High-pass, Low-pass, Mono frequency, Dynamic frequency, Dynamic depth, Dynamic on, Dynamic type, Dynamic listen, Trim link, In place |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | FAIL | 0 VU reference (20 -> 18), Meter type (BBC -> Nordic), Face (Arc -> Vertical), RMS window (1448.40 -> 947.28), Clip threshold (-10.92 -> -0.68), Trim L (-1.92 -> 7.15), Trim R (15.84 -> 19.24), Trim M (-14.40 -> -18.57), Trim S (3.36 -> -1.48), Polarity L (On -> Off), Polarity R (Off -> On), Mute L (On -> Off), Monitor (L -> R), High-pass (399.20 -> 26.29), Low-pass (11070.00 -> 6310.17), Mono frequency (122.60 -> 310.98), Mono amount (0.64 -> 0.16), Dynamic frequency (199.20 -> 9581.99), Dynamic depth (0.38 -> 0.14), Dynamic type (0 -> 1) |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | 0 VU reference |  | excused (-240.0 dB) | ok (0.282%) |
| 1 | Peak |  | excused (-240.0 dB) | ok (0.235%) |
| 2 | Meter type |  | excused (-240.0 dB) | ok (5.949%) |
| 3 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 4 | Single meter |  | excused (-240.0 dB) | ok (9.865%) |
| 5 | Mid side |  | DEAD (-240.0 dB) | ok (0.236%) |
| 6 | AES-17 |  | excused (-240.0 dB) | ok (0.170%) |
| 7 | RMS window |  | excused (-240.0 dB) | ok (0.170%) |
| 8 | Clip threshold |  | excused (-240.0 dB) | ok (0.170%) |
| 9 | Trim L |  | ok (-0.6 dB) | ok (0.696%) |
| 10 | Trim R |  | ok (-0.6 dB) | ok (0.655%) |
| 11 | Trim M |  | DEAD (-138.5 dB) | ok (0.641%) |
| 12 | Trim S |  | DEAD (-138.5 dB) | ok (0.641%) |
| 13 | Polarity L |  | ok (2.3 dB) | ok (0.712%) |
| 14 | Polarity R |  | ok (1.0 dB) | ok (0.711%) |
| 15 | Mute L |  | ok (-3.8 dB) | ok (0.559%) |
| 16 | Mute R |  | ok (-5.0 dB) | ok (0.482%) |
| 17 | Monitor |  | ok (-4.5 dB) | ok (0.610%) |
| 18 | High-pass |  | DEAD (-138.5 dB) | ok (0.610%) |
| 19 | High-pass slope |  | ok (-4.4 dB) | ok (0.627%) |
| 20 | Low-pass |  | DEAD (-138.5 dB) | ok (0.610%) |
| 21 | Low-pass slope |  | ok (-14.0 dB) | ok (0.615%) |
| 22 | Mono frequency |  | DEAD (-138.5 dB) | ok (0.610%) |
| 23 | Mono amount |  | ok (-14.0 dB) | ok (0.661%) |
| 24 | Dynamic frequency |  | DEAD (-138.5 dB) | ok (0.610%) |
| 25 | Dynamic depth |  | DEAD (-138.5 dB) | ok (0.610%) |
| 26 | Dynamic on |  | DEAD (-138.5 dB) | ok (0.792%) |
| 27 | Dynamic type |  | DEAD (-138.5 dB) | ok (0.774%) |
| 28 | Dynamic listen |  | DEAD (-138.5 dB) | ok (0.208%) |
| 29 | VU model |  | excused (-138.5 dB) | ok (0.208%) |
| 30 | Rise |  | excused (-138.5 dB) | ok (0.208%) |
| 31 | Fall |  | excused (-138.5 dB) | ok (0.208%) |
| 32 | Overshoot |  | excused (-138.5 dB) | ok (0.208%) |
| 33 | Peak hold |  | excused (-138.5 dB) | ok (0.208%) |
| 34 | Value hold |  | excused (-138.5 dB) | ok (0.208%) |
| 35 | Clip hold |  | excused (-138.5 dB) | ok (0.774%) |
| 36 | Peak lamp threshold |  | excused (-138.5 dB) | ok (0.774%) |
| 37 | Red zone |  | excused (-138.5 dB) | ok (2.295%) |
| 38 | Comfort zone |  | excused (-138.5 dB) | ok (0.774%) |
| 39 | Analog RMS |  | excused (-138.5 dB) | ok (0.774%) |
| 40 | Weighting |  | excused (-138.5 dB) | ok (0.790%) |
| 41 | Force mono |  | ok (-4.7 dB) | ok (0.885%) |
| 42 | Trim link |  | DEAD (-138.5 dB) | ok (0.774%) |
| 43 | In place |  | DEAD (-138.5 dB) | ok (0.774%) |
| 44 | Show values |  | excused (-138.5 dB) | ok (0.886%) |
| 45 | Show hold |  | excused (-3.8 dB) | ok (0.209%) |
| 46 | Show tools |  | excused (-3.8 dB) | ok (0.183%) |
| 47 | Show label |  | excused (-3.8 dB) | ok (0.183%) |
| 48 | Meters only |  | ok (-3.8 dB) | ok (10.144%) |
| 49 | GUI size |  | excused (-3.8 dB) | ok (100.000%) |
| 50 | Drag sensitivity |  | excused (-3.8 dB) | ok (0.183%) |
| 51 | Meter refresh |  | excused (-3.8 dB) | ok (0.183%) |
| 52 | Bypass hidden processors |  | ok (-3.8 dB) | ok (0.183%) |
| 53 | Tooltips |  | excused (-3.8 dB) | ok (0.183%) |
| 54 | Stay above windows |  | excused (-3.8 dB) | ok (0.183%) |
| 55 | Bypass |  | ok (-3.8 dB) | ok (0.183%) |
| 56 | Bypass |  | ok (-3.8 dB) | ok (0.802%) |
| 57 | Wet |  | ok (-3.8 dB) | ok (0.187%) |
| 58 | Delta |  | ok (-3.8 dB) | ok (0.256%) |
