# Plugin QA: LV2: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | LV2: ArcVu (OpenGrid), 60 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1650x431, 0.2% animates by itself |
| every parameter changes the sound | FAIL | Dynamic frequency, Dynamic depth, Dynamic on, Dynamic type, Dynamic listen, In place, Meters only |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | FAIL | 0 VU reference (1.74000001 -> 2.00000000), Peak (0.47999999 -> 0.00000000), Meter type (4.88000011 -> 5.00000000), Face (1.96000004 -> 2.00000000), Single meter (0.34999999 -> 0.00000000), Mid side (0.72000003 -> 1.00000000), AES-17 (0.09000000 -> 0.00000000), Polarity L (0.68000001 -> 1.00000000), Polarity R (0.05000000 -> 0.00000000), Mute L (0.41999999 -> 0.00000000), Mute R (0.79000002 -> 1.00000000), Monitor (0.80000001 -> 1.00000000), High-pass slope (3.59999990 -> 4.00000000), Low-pass slope (2.55999994 -> 3.00000000), Dynamic on (0.49000001 -> 0.00000000), Dynamic type (3.44000006 -> 3.00000000), Dynamic listen (0.23000000 -> 0.00000000), VU model (1.20000005 -> 1.00000000), Overshoot (5.73799992 -> 5.73999977), Analog RMS (0.30000001 -> 0.00000000) |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Free Wheeling |  | excused (-240.0 dB) | ok (0.138%) |
| 1 | Enabled | random state 5 | CONDITIONAL (4.3 dB) | ok (0.150%) |
| 2 | 0 VU reference |  | excused (-240.0 dB) | ok (0.198%) |
| 3 | Peak |  | excused (-240.0 dB) | ok (0.405%) |
| 4 | Meter type |  | excused (-240.0 dB) | ok (6.867%) |
| 5 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 6 | Single meter |  | excused (-240.0 dB) | ok (100.000%) |
| 7 | Mid side | Trim M=1 | ok (1.5 dB) | ok (100.000%) |
| 8 | AES-17 |  | excused (-240.0 dB) | ok (100.000%) |
| 9 | RMS window |  | excused (-240.0 dB) | ok (100.000%) |
| 10 | Clip threshold |  | excused (-240.0 dB) | ok (100.000%) |
| 11 | Trim L |  | ok (-0.6 dB) | ok (100.000%) |
| 12 | Trim R |  | ok (-0.6 dB) | ok (100.000%) |
| 13 | Trim M | Mid side=1 | ok (1.5 dB) | ok (100.000%) |
| 14 | Trim S | Mid side=1 | ok (2.4 dB) | ok (100.000%) |
| 15 | Polarity L |  | ok (2.3 dB) | ok (100.000%) |
| 16 | Polarity R |  | ok (1.0 dB) | ok (100.000%) |
| 17 | Mute L |  | ok (-3.8 dB) | ok (100.000%) |
| 18 | Mute R |  | ok (-5.0 dB) | ok (100.000%) |
| 19 | Monitor |  | ok (-4.5 dB) | ok (100.000%) |
| 20 | High-pass | High-pass slope=1 | ok (-4.2 dB) | ok (100.000%) |
| 21 | High-pass slope |  | ok (-3.8 dB) | ok (100.000%) |
| 22 | Low-pass | Low-pass slope=1random state 5 | CONDITIONAL (6.0 dB) | ok (100.000%) |
| 23 | Low-pass slope |  | ok (-3.8 dB) | ok (100.000%) |
| 24 | Mono frequency | Mono amount=1random state 5 | CONDITIONAL (-2.2 dB) | ok (100.000%) |
| 25 | Mono amount |  | ok (-3.8 dB) | ok (100.000%) |
| 26 | Dynamic frequency | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | DEAD (-240.0 dB) | ok (100.000%) |
| 27 | Dynamic depth | Dynamic on=1, Dynamic type=0.5 | DEAD (-240.0 dB) | ok (100.000%) |
| 28 | Dynamic on | Dynamic depth=1, Dynamic type=0.5 | DEAD (-240.0 dB) | ok (100.000%) |
| 29 | Dynamic type | Dynamic on=1, Dynamic depth=1 | DEAD (-240.0 dB) | ok (100.000%) |
| 30 | Dynamic listen | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | DEAD (-240.0 dB) | ok (100.000%) |
| 31 | VU model |  | excused (-3.8 dB) | ok (100.000%) |
| 32 | Rise |  | excused (-3.8 dB) | ok (100.000%) |
| 33 | Fall |  | excused (-3.8 dB) | ok (100.000%) |
| 34 | Overshoot |  | excused (-3.8 dB) | ok (100.000%) |
| 35 | Peak hold |  | excused (-3.8 dB) | ok (100.000%) |
| 36 | Value hold |  | excused (-3.8 dB) | ok (100.000%) |
| 37 | Clip hold |  | excused (-3.8 dB) | ok (100.000%) |
| 38 | Peak lamp threshold |  | excused (-3.8 dB) | ok (100.000%) |
| 39 | Red zone |  | excused (-3.8 dB) | ok (100.000%) |
| 40 | Comfort zone |  | excused (-3.8 dB) | ok (100.000%) |
| 41 | Analog RMS |  | excused (-3.8 dB) | ok (100.000%) |
| 42 | Weighting |  | excused (-3.8 dB) | ok (100.000%) |
| 43 | Force mono |  | ok (-3.8 dB) | ok (100.000%) |
| 44 | Trim link | Trim L=1random state 5 | CONDITIONAL (2.4 dB) | ok (100.000%) |
| 45 | In place | Monitor=0.4 | DEAD (-240.0 dB) | ok (100.000%) |
| 46 | Show values |  | excused (-3.8 dB) | ok (100.000%) |
| 47 | Show hold |  | excused (-3.8 dB) | ok (100.000%) |
| 48 | Show tools |  | excused (-3.8 dB) | ok (100.000%) |
| 49 | Show label |  | excused (-3.8 dB) | ok (100.000%) |
| 50 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | DEAD (-240.0 dB) | ok (100.000%) |
| 51 | GUI size |  | excused (-3.8 dB) | ok (3.224%) |
| 52 | Drag sensitivity |  | excused (-3.8 dB) | ok (100.000%) |
| 53 | Meter refresh |  | excused (-3.8 dB) | ok (100.000%) |
| 54 | Bypass hidden processors | Meters only=1, High-pass slope=1, High-pass=1random state 5 | CONDITIONAL (6.0 dB) | ok (100.000%) |
| 55 | Tooltips |  | excused (-3.8 dB) | ok (100.000%) |
| 56 | Stay above windows |  | excused (-3.8 dB) | ok (100.000%) |
| 57 | Bypass |  | ok (-3.8 dB) | ok (100.000%) |
| 58 | Wet |  | ok (-3.8 dB) | ok (100.000%) |
| 59 | Delta |  | ok (-3.8 dB) | ok (100.000%) |
