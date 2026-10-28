# Plugin QA: CLAP: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: ArcVu (OpenGrid), 58 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 830x223, 1.2% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | 0 VU reference |  | excused (-240.0 dB) | ok (100.000%) |
| 1 | Peak |  | excused (-240.0 dB) | ok (100.000%) |
| 2 | Meter type |  | excused (-240.0 dB) | ok (100.000%) |
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
| 56 | Wet | random state 1 | CONDITIONAL (-4.0 dB) | ok (100.000%) |
| 57 | Delta |  | ok (-3.8 dB) | ok (100.000%) |
