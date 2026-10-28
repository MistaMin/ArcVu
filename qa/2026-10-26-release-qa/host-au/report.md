# Plugin QA: AU: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | AU: ArcVu (OpenGrid), 58 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1375x368, 0.3% animates by itself |
| every parameter changes the sound | FAIL | Dynamic frequency, Dynamic on, High-pass, Low-pass, Trim M, Trim S, Mid side, Mono frequency, Dynamic listen, Bypass hidden processors, Dynamic depth, Trim link, In place, Dynamic type |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | FAIL | Polarity R (off -> on), AES-17 (off -> on), Dynamic on (off -> on), Mute L (off -> on), Show label (off -> on), Show tools (off -> on), Meters only (off -> on), Mid side (off -> on), Trim link (off -> on), Show hold (off -> on) |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 1 | Meter type |  | excused (-240.0 dB) | ok (6.116%) |
| 2 | Peak |  | excused (-240.0 dB) | ok (0.151%) |
| 3 | Polarity L |  | ok (2.3 dB) | ok (0.174%) |
| 4 | Polarity R |  | ok (1.0 dB) | ok (0.176%) |
| 5 | AES-17 |  | excused (-240.0 dB) | ok (0.083%) |
| 6 | Dynamic frequency |  | DEAD (-240.0 dB) | ok (0.083%) |
| 7 | Dynamic on |  | DEAD (-240.0 dB) | ok (0.101%) |
| 8 | High-pass |  | DEAD (-240.0 dB) | ok (0.083%) |
| 9 | Low-pass |  | DEAD (-240.0 dB) | ok (0.083%) |
| 10 | Mute L |  | ok (-3.8 dB) | ok (0.018%) |
| 11 | Mute R |  | ok (-5.0 dB) | ok (0.124%) |
| 12 | Red zone |  | excused (-240.0 dB) | ok (3.391%) |
| 13 | RMS window |  | excused (-240.0 dB) | ok (0.083%) |
| 14 | Trim L |  | ok (-0.6 dB) | ok (0.206%) |
| 15 | Trim M |  | DEAD (-138.5 dB) | ok (0.151%) |
| 16 | Trim R |  | ok (-0.6 dB) | ok (0.190%) |
| 17 | Trim S |  | DEAD (-138.5 dB) | ok (0.151%) |
| 18 | High-pass slope |  | ok (-4.4 dB) | ok (0.136%) |
| 19 | Stay above windows |  | excused (-138.5 dB) | ok (0.119%) |
| 20 | Show label |  | excused (-138.5 dB) | ok (0.145%) |
| 21 | Show tools |  | excused (-138.5 dB) | ok (0.742%) |
| 22 | Analog RMS |  | excused (-138.5 dB) | ok (0.119%) |
| 23 | Low-pass slope |  | ok (-14.0 dB) | ok (0.119%) |
| 24 | VU model |  | excused (-138.5 dB) | ok (0.119%) |
| 25 | Clip threshold |  | excused (-138.5 dB) | ok (0.119%) |
| 26 | GUI size |  | excused (-138.5 dB) | ok (100.000%) |
| 27 | Show values |  | excused (-138.5 dB) | ok (100.000%) |
| 28 | Fall |  | excused (-138.5 dB) | ok (100.000%) |
| 29 | Clip hold |  | excused (-138.5 dB) | ok (100.000%) |
| 30 | Comfort zone |  | excused (-138.5 dB) | ok (100.000%) |
| 31 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (100.000%) |
| 32 | Mid side |  | DEAD (-138.5 dB) | ok (100.000%) |
| 33 | Mono frequency |  | DEAD (-138.5 dB) | ok (100.000%) |
| 34 | Meter refresh |  | excused (-138.5 dB) | ok (100.000%) |
| 35 | Tooltips |  | excused (-138.5 dB) | ok (100.000%) |
| 36 | Dynamic listen |  | DEAD (-138.5 dB) | ok (100.000%) |
| 37 | Rise |  | excused (-138.5 dB) | ok (100.000%) |
| 38 | 0 VU reference |  | excused (-138.5 dB) | ok (100.000%) |
| 39 | Monitor |  | ok (-4.5 dB) | ok (100.000%) |
| 40 | Mono amount |  | ok (-14.0 dB) | ok (100.000%) |
| 41 | Single meter |  | excused (-138.5 dB) | ok (100.000%) |
| 42 | Drag sensitivity |  | excused (-138.5 dB) | ok (100.000%) |
| 43 | Bypass hidden processors |  | DEAD (-138.5 dB) | ok (100.000%) |
| 44 | Weighting |  | excused (-138.5 dB) | ok (100.000%) |
| 45 | Value hold |  | excused (-138.5 dB) | ok (100.000%) |
| 46 | Overshoot |  | excused (-138.5 dB) | ok (100.000%) |
| 47 | Dynamic depth |  | DEAD (-138.5 dB) | ok (100.000%) |
| 48 | Trim link |  | DEAD (-138.5 dB) | ok (100.000%) |
| 49 | Force mono |  | ok (-4.7 dB) | ok (100.000%) |
| 50 | Show hold |  | excused (-138.5 dB) | ok (100.000%) |
| 51 | Peak lamp threshold |  | excused (-138.5 dB) | ok (100.000%) |
| 52 | In place |  | DEAD (-138.5 dB) | ok (100.000%) |
| 53 | Peak hold |  | excused (-138.5 dB) | ok (100.000%) |
| 54 | Dynamic type |  | DEAD (-138.5 dB) | ok (100.000%) |
| 55 | Bypass |  | ok (-3.8 dB) | ok (100.000%) |
| 56 | Wet |  | ok (-3.8 dB) | ok (100.000%) |
| 57 | Delta |  | ok (-3.8 dB) | ok (100.000%) |
