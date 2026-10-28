# Plugin QA: VST3: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: ArcVu (OpenGrid), 59 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | SKIPPED | no editor window found (no Screen Recording permission, or no editor) |
| every parameter changes the sound | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | 0 VU reference |  | excused (-240.0 dB) | -  |
| 1 | Peak |  | excused (-240.0 dB) | -  |
| 2 | Meter type |  | excused (-240.0 dB) | -  |
| 3 | Face |  | excused (-240.0 dB) | -  |
| 4 | Single meter |  | excused (-240.0 dB) | -  |
| 5 | Mid side | Trim M=1 | ok (1.5 dB) | -  |
| 6 | AES-17 |  | excused (-240.0 dB) | -  |
| 7 | RMS window |  | excused (-240.0 dB) | -  |
| 8 | Clip threshold |  | excused (-240.0 dB) | -  |
| 9 | Trim L |  | ok (-0.6 dB) | -  |
| 10 | Trim R |  | ok (-0.6 dB) | -  |
| 11 | Trim M | Mid side=1 | ok (1.5 dB) | -  |
| 12 | Trim S | Mid side=1 | ok (2.4 dB) | -  |
| 13 | Polarity L |  | ok (2.3 dB) | -  |
| 14 | Polarity R |  | ok (1.0 dB) | -  |
| 15 | Mute L |  | ok (-3.8 dB) | -  |
| 16 | Mute R |  | ok (-5.0 dB) | -  |
| 17 | Monitor |  | ok (-4.5 dB) | -  |
| 18 | High-pass | High-pass slope=1 | ok (-5.0 dB) | -  |
| 19 | High-pass slope |  | ok (-4.4 dB) | -  |
| 20 | Low-pass | Low-pass slope=1 | ok (-3.8 dB) | -  |
| 21 | Low-pass slope |  | ok (-14.0 dB) | -  |
| 22 | Mono frequency | Mono amount=1 | ok (-15.8 dB) | -  |
| 23 | Mono amount |  | ok (-14.0 dB) | -  |
| 24 | Dynamic frequency | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | -  |
| 25 | Dynamic depth | Dynamic on=1, Dynamic type=0.5 | ok (-5.2 dB) | -  |
| 26 | Dynamic on | Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | -  |
| 27 | Dynamic type | Dynamic on=1, Dynamic depth=1 | ok (-8.2 dB) | -  |
| 28 | Dynamic listen | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-3.6 dB) | -  |
| 29 | VU model |  | excused (-138.5 dB) | -  |
| 30 | Rise |  | excused (-138.5 dB) | -  |
| 31 | Fall |  | excused (-138.5 dB) | -  |
| 32 | Overshoot |  | excused (-138.5 dB) | -  |
| 33 | Peak hold |  | excused (-138.5 dB) | -  |
| 34 | Value hold |  | excused (-138.5 dB) | -  |
| 35 | Clip hold |  | excused (-138.5 dB) | -  |
| 36 | Peak lamp threshold |  | excused (-138.5 dB) | -  |
| 37 | Red zone |  | excused (-138.5 dB) | -  |
| 38 | Comfort zone |  | excused (-138.5 dB) | -  |
| 39 | Analog RMS |  | excused (-138.5 dB) | -  |
| 40 | Weighting |  | excused (-138.5 dB) | -  |
| 41 | Force mono |  | ok (-4.7 dB) | -  |
| 42 | Trim link | Trim L=1 | ok (-0.6 dB) | -  |
| 43 | In place | Monitor=0.4 | ok (-3.8 dB) | -  |
| 44 | Show values |  | excused (-138.5 dB) | -  |
| 45 | Show hold |  | excused (-138.5 dB) | -  |
| 46 | Show tools |  | excused (-138.5 dB) | -  |
| 47 | Show label |  | excused (-138.5 dB) | -  |
| 48 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | -  |
| 49 | GUI size |  | excused (-138.5 dB) | -  |
| 50 | Drag sensitivity |  | excused (-138.5 dB) | -  |
| 51 | Meter refresh |  | excused (-138.5 dB) | -  |
| 52 | Bypass hidden processors | Meters only=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | -  |
| 53 | Tooltips |  | excused (-138.5 dB) | -  |
| 54 | Stay above windows |  | excused (-138.5 dB) | -  |
| 55 | Bypass | random state 1 | CONDITIONAL (-4.0 dB) | -  |
| 56 | Bypass | random state 1 | CONDITIONAL (-4.0 dB) | -  |
| 57 | Wet | random state 1 | CONDITIONAL (-4.0 dB) | -  |
| 58 | Delta |  | ok (-3.8 dB) | -  |
