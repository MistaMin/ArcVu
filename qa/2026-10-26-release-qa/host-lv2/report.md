# Plugin QA: LV2: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | LV2: ArcVu (OpenGrid), 60 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1650x431, 0.2% animates by itself |
| every parameter changes the sound | FAIL | Free Wheeling |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | FAIL | 0 VU reference (1.74000001 -> 2.00000000), Peak (0.47999999 -> 0.00000000), Meter type (4.88000011 -> 5.00000000), Face (1.96000004 -> 2.00000000), Single meter (0.34999999 -> 0.00000000), Mid side (0.72000003 -> 1.00000000), AES-17 (0.09000000 -> 0.00000000), Polarity L (0.68000001 -> 1.00000000), Polarity R (0.05000000 -> 0.00000000), Mute L (0.41999999 -> 0.00000000), Mute R (0.79000002 -> 1.00000000), Monitor (0.80000001 -> 1.00000000), High-pass slope (3.59999990 -> 4.00000000), Low-pass slope (2.55999994 -> 3.00000000), Dynamic on (0.49000001 -> 0.00000000), Dynamic type (3.44000006 -> 3.00000000), Dynamic listen (0.23000000 -> 0.00000000), VU model (1.20000005 -> 1.00000000), Overshoot (5.73799992 -> 5.73999977), Analog RMS (0.30000001 -> 0.00000000) |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Free Wheeling |  | DEAD (-240.0 dB) | ok (0.082%) |
| 1 | Enabled | random state 1 | CONDITIONAL (2.9 dB) | ok (0.141%) |
| 2 | 0 VU reference |  | excused (-240.0 dB) | ok (0.250%) |
| 3 | Peak |  | excused (-240.0 dB) | ok (0.348%) |
| 4 | Meter type |  | excused (-240.0 dB) | ok (6.747%) |
| 5 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 6 | Single meter |  | excused (-240.0 dB) | ok (3.753%) |
| 7 | Mid side | random state 1 | CONDITIONAL (6.0 dB) | ok (0.527%) |
| 8 | AES-17 |  | excused (-240.0 dB) | ok (0.327%) |
| 9 | RMS window |  | excused (-240.0 dB) | ok (0.327%) |
| 10 | Clip threshold |  | excused (-240.0 dB) | ok (0.327%) |
| 11 | Trim L |  | ok (-0.6 dB) | ok (0.441%) |
| 12 | Trim R |  | ok (-0.6 dB) | ok (0.435%) |
| 13 | Trim M | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.393%) |
| 14 | Trim S | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.393%) |
| 15 | Polarity L |  | ok (2.3 dB) | ok (0.455%) |
| 16 | Polarity R |  | ok (1.0 dB) | ok (0.460%) |
| 17 | Mute L |  | ok (-3.8 dB) | ok (0.297%) |
| 18 | Mute R |  | ok (-5.0 dB) | ok (0.350%) |
| 19 | Monitor |  | ok (-4.5 dB) | ok (0.361%) |
| 20 | High-pass | random state 1 | CONDITIONAL (0.2 dB) | ok (0.361%) |
| 21 | High-pass slope |  | ok (-8.9 dB) | ok (0.379%) |
| 22 | Low-pass | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.361%) |
| 23 | Low-pass slope |  | ok (-3.2 dB) | ok (0.342%) |
| 24 | Mono frequency | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.361%) |
| 25 | Mono amount |  | ok (-23.4 dB) | ok (0.385%) |
| 26 | Dynamic frequency | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.361%) |
| 27 | Dynamic depth | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.361%) |
| 28 | Dynamic on | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.379%) |
| 29 | Dynamic type | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.361%) |
| 30 | Dynamic listen | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.361%) |
| 31 | VU model |  | excused (-138.5 dB) | ok (0.361%) |
| 32 | Rise |  | excused (-138.5 dB) | ok (0.361%) |
| 33 | Fall |  | excused (-138.5 dB) | ok (0.361%) |
| 34 | Overshoot |  | excused (-138.5 dB) | ok (0.361%) |
| 35 | Peak hold |  | excused (-138.5 dB) | ok (0.361%) |
| 36 | Value hold |  | excused (-138.5 dB) | ok (0.361%) |
| 37 | Clip hold |  | excused (-138.5 dB) | ok (0.361%) |
| 38 | Peak lamp threshold |  | excused (-138.5 dB) | ok (0.361%) |
| 39 | Red zone |  | excused (-138.5 dB) | ok (3.733%) |
| 40 | Comfort zone |  | excused (-138.5 dB) | ok (0.361%) |
| 41 | Analog RMS |  | excused (-138.5 dB) | ok (0.361%) |
| 42 | Weighting |  | excused (-138.5 dB) | ok (0.474%) |
| 43 | Force mono |  | ok (-4.7 dB) | ok (0.599%) |
| 44 | Trim link | random state 3 | CONDITIONAL (0.3 dB) | ok (0.474%) |
| 45 | In place | random state 3 | CONDITIONAL (-10.6 dB) | ok (0.474%) |
| 46 | Show values |  | excused (-138.5 dB) | ok (0.474%) |
| 47 | Show hold |  | excused (-138.5 dB) | ok (0.533%) |
| 48 | Show tools |  | excused (-138.5 dB) | ok (3.291%) |
| 49 | Show label |  | excused (-138.5 dB) | ok (3.948%) |
| 50 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (18.767%) |
| 51 | GUI size |  | excused (-138.5 dB) | ok (3.973%) |
| 52 | Drag sensitivity |  | excused (-138.5 dB) | ok (100.000%) |
| 53 | Meter refresh |  | excused (-138.5 dB) | ok (100.000%) |
| 54 | Bypass hidden processors | random state 3 | CONDITIONAL (-5.3 dB) | ok (100.000%) |
| 55 | Tooltips |  | excused (-138.5 dB) | ok (100.000%) |
| 56 | Stay above windows |  | excused (-138.5 dB) | ok (100.000%) |
| 57 | Bypass | random state 1 | CONDITIONAL (2.9 dB) | ok (100.000%) |
| 58 | Wet | random state 1 | CONDITIONAL (2.9 dB) | ok (100.000%) |
| 59 | Delta |  | ok (-3.8 dB) | ok (100.000%) |
