# Plugin QA: CLAP: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: ArcVu (OpenGrid), 58 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1375x368, 0.0% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | 0 VU reference |  | excused (-240.0 dB) | ok (0.258%) |
| 1 | Peak |  | excused (-240.0 dB) | ok (0.151%) |
| 2 | Meter type |  | excused (-240.0 dB) | ok (6.116%) |
| 3 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 4 | Single meter |  | excused (-240.0 dB) | ok (3.567%) |
| 5 | Mid side | random state 1 | CONDITIONAL (-10.5 dB) | ok (0.347%) |
| 6 | AES-17 |  | excused (-240.0 dB) | ok (0.083%) |
| 7 | RMS window |  | excused (-240.0 dB) | ok (0.083%) |
| 8 | Clip threshold |  | excused (-240.0 dB) | ok (0.083%) |
| 9 | Trim L |  | ok (-0.6 dB) | ok (0.206%) |
| 10 | Trim R |  | ok (-0.6 dB) | ok (0.209%) |
| 11 | Trim M | random state 5 | CONDITIONAL (-0.5 dB) | ok (0.151%) |
| 12 | Trim S | random state 5 | CONDITIONAL (-0.5 dB) | ok (0.151%) |
| 13 | Polarity L |  | ok (2.3 dB) | ok (0.210%) |
| 14 | Polarity R |  | ok (1.0 dB) | ok (0.211%) |
| 15 | Mute L |  | ok (-3.8 dB) | ok (0.053%) |
| 16 | Mute R |  | ok (-5.0 dB) | ok (0.241%) |
| 17 | Monitor |  | ok (-4.5 dB) | ok (0.119%) |
| 18 | High-pass | random state 1 | CONDITIONAL (-30.6 dB) | ok (0.119%) |
| 19 | High-pass slope |  | ok (-4.4 dB) | ok (0.136%) |
| 20 | Low-pass | random state 3 | CONDITIONAL (6.0 dB) | ok (0.119%) |
| 21 | Low-pass slope |  | ok (-14.0 dB) | ok (0.119%) |
| 22 | Mono frequency | random state 3 | CONDITIONAL (-22.2 dB) | ok (0.119%) |
| 23 | Mono amount |  | ok (-14.0 dB) | ok (0.159%) |
| 24 | Dynamic frequency | random state 5 | CONDITIONAL (-25.7 dB) | ok (0.119%) |
| 25 | Dynamic depth | random state 5 | CONDITIONAL (-76.5 dB) | ok (0.119%) |
| 26 | Dynamic on | random state 1 | CONDITIONAL (-52.7 dB) | ok (0.136%) |
| 27 | Dynamic type | random state 5 | CONDITIONAL (-35.2 dB) | ok (0.119%) |
| 28 | Dynamic listen | random state 5 | CONDITIONAL (-7.7 dB) | ok (0.119%) |
| 29 | VU model |  | excused (-138.5 dB) | ok (0.119%) |
| 30 | Rise |  | excused (-138.5 dB) | ok (0.119%) |
| 31 | Fall |  | excused (-138.5 dB) | ok (0.119%) |
| 32 | Overshoot |  | excused (-138.5 dB) | ok (0.119%) |
| 33 | Peak hold |  | excused (-138.5 dB) | ok (0.119%) |
| 34 | Value hold |  | excused (-138.5 dB) | ok (0.119%) |
| 35 | Clip hold |  | excused (-138.5 dB) | ok (0.119%) |
| 36 | Peak lamp threshold |  | excused (-138.5 dB) | ok (0.119%) |
| 37 | Red zone |  | excused (-138.5 dB) | ok (3.426%) |
| 38 | Comfort zone |  | excused (-138.5 dB) | ok (0.119%) |
| 39 | Analog RMS |  | excused (-138.5 dB) | ok (0.119%) |
| 40 | Weighting |  | excused (-138.5 dB) | ok (0.219%) |
| 41 | Force mono |  | ok (-4.7 dB) | ok (0.294%) |
| 42 | Trim link | random state 1 | CONDITIONAL (-0.1 dB) | ok (0.119%) |
| 43 | In place | random state 5 | CONDITIONAL (-7.7 dB) | ok (0.119%) |
| 44 | Show values |  | excused (-138.5 dB) | ok (0.371%) |
| 45 | Show hold |  | excused (-138.5 dB) | ok (0.280%) |
| 46 | Show tools |  | excused (-138.5 dB) | ok (0.742%) |
| 47 | Show label |  | excused (-138.5 dB) | ok (0.145%) |
| 48 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (15.709%) |
| 49 | GUI size |  | excused (-138.5 dB) | ok (100.000%) |
| 50 | Drag sensitivity |  | excused (-138.5 dB) | ok (100.000%) |
| 51 | Meter refresh |  | excused (-138.5 dB) | ok (100.000%) |
| 52 | Bypass hidden processors | random state 1 | CONDITIONAL (-29.2 dB) | ok (100.000%) |
| 53 | Tooltips |  | excused (-138.5 dB) | ok (100.000%) |
| 54 | Stay above windows |  | excused (-138.5 dB) | ok (100.000%) |
| 55 | Bypass | random state 1 | CONDITIONAL (-4.0 dB) | ok (100.000%) |
| 56 | Wet | random state 1 | CONDITIONAL (-4.0 dB) | ok (100.000%) |
| 57 | Delta |  | ok (-3.8 dB) | ok (100.000%) |
