# Plugin QA: VST3: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: ArcVu (OpenGrid), 59 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x368, 0.0% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | FAIL | 0 VU reference (20 -> 18), Meter type (BBC -> Nordic), Face (Arc -> Vertical), RMS window (1448.40 -> 947.28), Clip threshold (-10.92 -> -0.68), Trim L (-1.92 -> 7.15), Trim R (15.84 -> 19.24), Trim M (-14.40 -> -18.57), Trim S (3.36 -> -1.48), Polarity L (On -> Off), Polarity R (Off -> On), Mute L (On -> Off), Monitor (L -> R), High-pass (399.20 -> 26.29), Low-pass (11070.00 -> 6310.17), Mono frequency (122.60 -> 310.98), Mono amount (0.64 -> 0.16), Dynamic frequency (199.20 -> 9581.99), Dynamic depth (0.38 -> 0.14), Dynamic type (0 -> 1) |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | 0 VU reference |  | excused (-240.0 dB) | ok (1.636%) |
| 1 | Peak |  | excused (-240.0 dB) | ok (1.989%) |
| 2 | Meter type |  | excused (-240.0 dB) | ok (4.141%) |
| 3 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 4 | Single meter |  | excused (-240.0 dB) | ok (100.000%) |
| 5 | Mid side | Trim M=1 | ok (1.5 dB) | ok (100.000%) |
| 6 | AES-17 |  | excused (-240.0 dB) | ok (100.000%) |
| 7 | RMS window |  | excused (-240.0 dB) | ok (100.000%) |
| 8 | Clip threshold |  | excused (-240.0 dB) | ok (100.000%) |
| 9 | Trim L |  | ok (-0.6 dB) | ok (100.000%) |
| 10 | Trim R |  | ok (-0.6 dB) | ok (100.000%) |
| 11 | Trim M | Mid side=1 | ok (1.5 dB) | ok (100.000%) |
| 12 | Trim S | Mid side=1 | ok (2.4 dB) | ok (100.000%) |
| 13 | Polarity L |  | ok (2.3 dB) | ok (100.000%) |
| 14 | Polarity R |  | ok (1.0 dB) | ok (100.000%) |
| 15 | Mute L |  | ok (-3.8 dB) | ok (100.000%) |
| 16 | Mute R |  | ok (-5.0 dB) | ok (100.000%) |
| 17 | Monitor |  | ok (-4.5 dB) | ok (100.000%) |
| 18 | High-pass | High-pass slope=1 | ok (-5.0 dB) | ok (100.000%) |
| 19 | High-pass slope |  | ok (-4.4 dB) | ok (100.000%) |
| 20 | Low-pass | Low-pass slope=1 | ok (-3.8 dB) | ok (100.000%) |
| 21 | Low-pass slope |  | ok (-14.0 dB) | ok (100.000%) |
| 22 | Mono frequency | Mono amount=1 | ok (-15.8 dB) | ok (100.000%) |
| 23 | Mono amount |  | ok (-14.0 dB) | ok (100.000%) |
| 24 | Dynamic frequency | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | ok (100.000%) |
| 25 | Dynamic depth | Dynamic on=1, Dynamic type=0.5 | ok (-5.2 dB) | ok (100.000%) |
| 26 | Dynamic on | Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | ok (100.000%) |
| 27 | Dynamic type | Dynamic on=1, Dynamic depth=1 | ok (-8.2 dB) | ok (100.000%) |
| 28 | Dynamic listen | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-3.6 dB) | ok (100.000%) |
| 29 | VU model |  | excused (-138.5 dB) | ok (100.000%) |
| 30 | Rise |  | excused (-138.5 dB) | ok (100.000%) |
| 31 | Fall |  | excused (-138.5 dB) | ok (100.000%) |
| 32 | Overshoot |  | excused (-138.5 dB) | ok (100.000%) |
| 33 | Peak hold |  | excused (-138.5 dB) | ok (100.000%) |
| 34 | Value hold |  | excused (-138.5 dB) | ok (100.000%) |
| 35 | Clip hold |  | excused (-138.5 dB) | ok (100.000%) |
| 36 | Peak lamp threshold |  | excused (-138.5 dB) | ok (100.000%) |
| 37 | Red zone |  | excused (-138.5 dB) | ok (100.000%) |
| 38 | Comfort zone |  | excused (-138.5 dB) | ok (100.000%) |
| 39 | Analog RMS |  | excused (-138.5 dB) | ok (100.000%) |
| 40 | Weighting |  | excused (-138.5 dB) | ok (100.000%) |
| 41 | Force mono |  | ok (-4.7 dB) | ok (100.000%) |
| 42 | Trim link | Trim L=1 | ok (-0.6 dB) | ok (100.000%) |
| 43 | In place | Monitor=0.4 | ok (-3.8 dB) | ok (100.000%) |
| 44 | Show values |  | excused (-138.5 dB) | ok (100.000%) |
| 45 | Show hold |  | excused (-138.5 dB) | ok (100.000%) |
| 46 | Show tools |  | excused (-138.5 dB) | ok (100.000%) |
| 47 | Show label |  | excused (-138.5 dB) | ok (100.000%) |
| 48 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (100.000%) |
| 49 | GUI size |  | excused (-138.5 dB) | ok (100.000%) |
| 50 | Drag sensitivity |  | excused (-138.5 dB) | ok (100.000%) |
| 51 | Meter refresh |  | excused (-138.5 dB) | ok (100.000%) |
| 52 | Bypass hidden processors | Meters only=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (100.000%) |
| 53 | Tooltips |  | excused (-138.5 dB) | ok (100.000%) |
| 54 | Stay above windows |  | excused (-138.5 dB) | ok (100.000%) |
| 55 | Bypass | random state 1 | CONDITIONAL (-4.0 dB) | ok (100.000%) |
| 56 | Bypass | random state 1 | CONDITIONAL (-4.0 dB) | ok (100.000%) |
| 57 | Wet | random state 1 | CONDITIONAL (-4.0 dB) | ok (100.000%) |
| 58 | Delta |  | ok (-3.8 dB) | ok (100.000%) |
