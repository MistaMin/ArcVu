# Plugin QA: VST3: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: ArcVu (OpenGrid), 59 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1100x305, 0.0% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | 0 VU reference |  | excused (-240.0 dB) | ok (0.237%) |
| 1 | Peak |  | excused (-240.0 dB) | ok (0.135%) |
| 2 | Meter type |  | excused (-240.0 dB) | ok (5.907%) |
| 3 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 4 | Single meter |  | excused (-240.0 dB) | ok (3.424%) |
| 5 | Mid side | random state 1 | CONDITIONAL (-10.5 dB) | ok (0.317%) |
| 6 | AES-17 |  | excused (-240.0 dB) | ok (0.069%) |
| 7 | RMS window |  | excused (-240.0 dB) | ok (0.069%) |
| 8 | Clip threshold |  | excused (-240.0 dB) | ok (0.069%) |
| 9 | Trim L |  | ok (-0.6 dB) | ok (0.195%) |
| 10 | Trim R |  | ok (-0.6 dB) | ok (0.198%) |
| 11 | Trim M | random state 5 | CONDITIONAL (-0.5 dB) | ok (0.139%) |
| 12 | Trim S | random state 5 | CONDITIONAL (-0.5 dB) | ok (0.139%) |
| 13 | Polarity L |  | ok (2.3 dB) | ok (0.208%) |
| 14 | Polarity R |  | ok (1.0 dB) | ok (0.205%) |
| 15 | Mute L |  | ok (-3.8 dB) | ok (0.057%) |
| 16 | Mute R |  | ok (-5.0 dB) | ok (0.236%) |
| 17 | Monitor |  | ok (-4.5 dB) | ok (0.108%) |
| 18 | High-pass | random state 1 | CONDITIONAL (-30.6 dB) | ok (0.108%) |
| 19 | High-pass slope |  | ok (-4.4 dB) | ok (0.126%) |
| 20 | Low-pass | random state 3 | CONDITIONAL (6.0 dB) | ok (0.108%) |
| 21 | Low-pass slope |  | ok (-14.0 dB) | ok (0.114%) |
| 22 | Mono frequency | random state 3 | CONDITIONAL (-22.2 dB) | ok (0.108%) |
| 23 | Mono amount |  | ok (-14.0 dB) | ok (0.160%) |
| 24 | Dynamic frequency | random state 5 | CONDITIONAL (-25.7 dB) | ok (0.108%) |
| 25 | Dynamic depth | random state 5 | CONDITIONAL (-76.5 dB) | ok (0.108%) |
| 26 | Dynamic on | random state 1 | CONDITIONAL (-52.7 dB) | ok (0.126%) |
| 27 | Dynamic type | random state 5 | CONDITIONAL (-35.2 dB) | ok (0.108%) |
| 28 | Dynamic listen | random state 5 | CONDITIONAL (-7.7 dB) | ok (0.108%) |
| 29 | VU model |  | excused (-138.5 dB) | ok (0.108%) |
| 30 | Rise |  | excused (-138.5 dB) | ok (0.108%) |
| 31 | Fall |  | excused (-138.5 dB) | ok (0.108%) |
| 32 | Overshoot |  | excused (-138.5 dB) | ok (0.108%) |
| 33 | Peak hold |  | excused (-138.5 dB) | ok (0.108%) |
| 34 | Value hold |  | excused (-138.5 dB) | ok (0.108%) |
| 35 | Clip hold |  | excused (-138.5 dB) | ok (0.108%) |
| 36 | Peak lamp threshold |  | excused (-138.5 dB) | ok (0.108%) |
| 37 | Red zone |  | excused (-138.5 dB) | ok (3.287%) |
| 38 | Comfort zone |  | excused (-138.5 dB) | ok (0.108%) |
| 39 | Analog RMS |  | excused (-138.5 dB) | ok (0.108%) |
| 40 | Weighting |  | excused (-138.5 dB) | ok (0.204%) |
| 41 | Force mono |  | ok (-4.7 dB) | ok (0.287%) |
| 42 | Trim link | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.108%) |
| 43 | In place | random state 5 | CONDITIONAL (-7.7 dB) | ok (0.108%) |
| 44 | Show values |  | excused (-138.5 dB) | ok (100.000%) |
| 45 | Show hold |  | excused (-138.5 dB) | ok (100.000%) |
| 46 | Show tools |  | excused (-138.5 dB) | ok (0.740%) |
| 47 | Show label |  | excused (-138.5 dB) | ok (0.132%) |
| 48 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (15.164%) |
| 49 | GUI size |  | excused (-138.5 dB) | ok (100.000%) |
| 50 | Drag sensitivity |  | excused (-138.5 dB) | ok (0.108%) |
| 51 | Meter refresh |  | excused (-138.5 dB) | ok (0.108%) |
| 52 | Bypass hidden processors | random state 1 | CONDITIONAL (-29.2 dB) | ok (0.108%) |
| 53 | Tooltips |  | excused (-138.5 dB) | ok (0.108%) |
| 54 | Stay above windows |  | excused (-138.5 dB) | ok (0.108%) |
| 55 | Bypass | random state 1 | CONDITIONAL (-4.0 dB) | ok (0.160%) |
| 56 | Bypass | random state 1 | CONDITIONAL (-4.0 dB) | ok (0.665%) |
| 57 | Wet | random state 1 | CONDITIONAL (-4.0 dB) | ok (0.108%) |
| 58 | Delta |  | ok (-3.8 dB) | ok (0.108%) |
