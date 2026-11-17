# Plugin QA: LV2: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | LV2: ArcVu (OpenGrid), 60 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x431, 0.0% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | FAIL | Free Wheeling, Enabled |
| state survives save and reopen | FAIL | Free Wheeling (On -> Off), 0 VU reference (1.74000001 -> ), Peak (0.47999999 -> ), Meter type (4.88000011 -> ), Face (1.96000004 -> ), Single meter (0.34999999 -> ), Mid side (0.72000003 -> ), AES-17 (0.09000000 -> ), RMS window (936.20001221 -> ), Clip threshold (-2.03999996 -> ), Trim L (-14.39999962 -> ), Trim R (3.35999990 -> ), Trim M (21.12000084 -> ), Trim S (-9.11999989 -> ), Polarity L (0.68000001 -> ), Polarity R (0.05000000 -> ), Mute L (0.41999999 -> ), Mute R (0.79000002 -> ), Monitor (0.80000001 -> ), High-pass (274.39999390 -> ) |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Free Wheeling |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 1 | Enabled | random state 1 | CONDITIONAL (2.9 dB) | NOT SHOWN (0.000%) |
| 2 | 0 VU reference |  | excused (-240.0 dB) | ok (1.663%) |
| 3 | Peak |  | excused (-240.0 dB) | ok (0.865%) |
| 4 | Meter type |  | excused (-240.0 dB) | ok (4.979%) |
| 5 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 6 | Single meter |  | excused (-240.0 dB) | ok (48.888%) |
| 7 | Mid side | Trim M=1 | ok (1.5 dB) | ok (51.289%) |
| 8 | AES-17 |  | excused (-240.0 dB) | ok (51.201%) |
| 9 | RMS window |  | excused (-240.0 dB) | ok (51.200%) |
| 10 | Clip threshold |  | excused (-240.0 dB) | ok (51.200%) |
| 11 | Trim L |  | ok (-0.6 dB) | ok (51.495%) |
| 12 | Trim R |  | ok (-0.6 dB) | ok (51.576%) |
| 13 | Trim M | Mid side=1 | ok (1.5 dB) | ok (51.799%) |
| 14 | Trim S | Mid side=1 | ok (2.4 dB) | ok (51.684%) |
| 15 | Polarity L |  | ok (2.3 dB) | ok (51.225%) |
| 16 | Polarity R |  | ok (1.0 dB) | ok (51.223%) |
| 17 | Mute L |  | ok (-3.8 dB) | ok (51.152%) |
| 18 | Mute R |  | ok (-5.0 dB) | ok (50.939%) |
| 19 | Monitor |  | ok (-4.5 dB) | ok (51.200%) |
| 20 | High-pass | High-pass slope=1 | ok (-5.0 dB) | ok (51.503%) |
| 21 | High-pass slope |  | ok (-8.9 dB) | ok (51.207%) |
| 22 | Low-pass | Low-pass slope=1 | ok (-3.8 dB) | ok (51.205%) |
| 23 | Low-pass slope |  | ok (-3.2 dB) | ok (51.228%) |
| 24 | Mono frequency | Mono amount=1 | ok (-15.8 dB) | ok (51.205%) |
| 25 | Mono amount |  | ok (-23.4 dB) | ok (51.206%) |
| 26 | Dynamic frequency | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | ok (51.205%) |
| 27 | Dynamic depth | Dynamic on=1, Dynamic type=0.5 | ok (-12.7 dB) | ok (51.233%) |
| 28 | Dynamic on | Dynamic depth=1, Dynamic type=0.5 | ok (-12.7 dB) | ok (51.205%) |
| 29 | Dynamic type | Dynamic on=1, Dynamic depth=1 | ok (-7.5 dB) | ok (51.813%) |
| 30 | Dynamic listen | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-4.4 dB) | ok (51.202%) |
| 31 | VU model |  | excused (-138.5 dB) | ok (51.200%) |
| 32 | Rise |  | excused (-138.5 dB) | ok (51.203%) |
| 33 | Fall |  | excused (-138.5 dB) | ok (51.200%) |
| 34 | Overshoot |  | excused (-138.5 dB) | ok (51.200%) |
| 35 | Peak hold |  | excused (-138.5 dB) | ok (51.200%) |
| 36 | Value hold |  | excused (-138.5 dB) | ok (51.200%) |
| 37 | Clip hold |  | excused (-138.5 dB) | ok (51.200%) |
| 38 | Peak lamp threshold |  | excused (-138.5 dB) | ok (51.200%) |
| 39 | Red zone |  | excused (-138.5 dB) | ok (50.913%) |
| 40 | Comfort zone |  | excused (-138.5 dB) | ok (51.200%) |
| 41 | Analog RMS |  | excused (-138.5 dB) | ok (51.200%) |
| 42 | Weighting |  | excused (-138.5 dB) | ok (51.809%) |
| 43 | Force mono |  | ok (-4.7 dB) | ok (51.196%) |
| 44 | Trim link | Trim L=1 | ok (-0.6 dB) | ok (51.858%) |
| 45 | In place | Monitor=0.4 | ok (-3.8 dB) | ok (51.239%) |
| 46 | Show values |  | excused (-138.5 dB) | ok (51.847%) |
| 47 | Show hold |  | excused (-138.5 dB) | ok (51.198%) |
| 48 | Show tools |  | excused (-138.5 dB) | ok (50.634%) |
| 49 | Show label |  | excused (-138.5 dB) | ok (50.655%) |
| 50 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (50.363%) |
| 51 | GUI size |  | excused (-138.5 dB) | ok (1.293%) |
| 52 | Drag sensitivity |  | excused (-138.5 dB) | ok (100.000%) |
| 53 | Meter refresh |  | excused (-138.5 dB) | ok (100.000%) |
| 54 | Bypass hidden processors | Meters only=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (100.000%) |
| 55 | Tooltips |  | excused (-138.5 dB) | ok (100.000%) |
| 56 | Stay above windows |  | excused (-138.5 dB) | ok (100.000%) |
| 57 | Bypass | random state 1 | CONDITIONAL (2.9 dB) | ok (100.000%) |
| 58 | Wet | random state 1 | CONDITIONAL (2.9 dB) | ok (100.000%) |
| 59 | Delta |  | ok (-3.8 dB) | ok (100.000%) |
