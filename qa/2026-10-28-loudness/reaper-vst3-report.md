# Plugin QA: VST3: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: ArcVu (OpenGrid), 59 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1375x368, 0.3% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | 0 VU reference |  | excused (-240.0 dB) | ok (0.267%) |
| 1 | Peak |  | excused (-240.0 dB) | ok (0.162%) |
| 2 | Meter type |  | excused (-240.0 dB) | ok (6.086%) |
| 3 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 4 | Single meter |  | excused (-240.0 dB) | ok (3.407%) |
| 5 | Mid side | Trim M=1 | ok (1.5 dB) | ok (0.589%) |
| 6 | AES-17 |  | excused (-240.0 dB) | ok (0.127%) |
| 7 | RMS window |  | excused (-240.0 dB) | ok (0.127%) |
| 8 | Clip threshold |  | excused (-240.0 dB) | ok (0.127%) |
| 9 | Trim L |  | ok (-0.6 dB) | ok (0.219%) |
| 10 | Trim R |  | ok (-0.6 dB) | ok (0.201%) |
| 11 | Trim M | Mid side=1 | ok (1.5 dB) | ok (0.563%) |
| 12 | Trim S | Mid side=1 | ok (2.4 dB) | ok (0.608%) |
| 13 | Polarity L |  | ok (2.3 dB) | ok (0.209%) |
| 14 | Polarity R |  | ok (1.0 dB) | ok (0.213%) |
| 15 | Mute L |  | ok (-3.8 dB) | ok (0.052%) |
| 16 | Mute R |  | ok (-5.0 dB) | ok (0.161%) |
| 17 | Monitor |  | ok (-4.5 dB) | ok (0.127%) |
| 18 | High-pass | High-pass slope=1 | ok (-5.0 dB) | ok (0.145%) |
| 19 | High-pass slope |  | ok (-4.4 dB) | ok (0.145%) |
| 20 | Low-pass | Low-pass slope=1 | ok (-3.8 dB) | ok (0.136%) |
| 21 | Low-pass slope |  | ok (-14.0 dB) | ok (0.131%) |
| 22 | Mono frequency | Mono amount=1 | ok (-15.8 dB) | ok (0.198%) |
| 23 | Mono amount |  | ok (-14.0 dB) | ok (0.167%) |
| 24 | Dynamic frequency | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | ok (0.201%) |
| 25 | Dynamic depth | Dynamic on=1, Dynamic type=0.5 | ok (-5.2 dB) | ok (0.145%) |
| 26 | Dynamic on | Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | ok (0.204%) |
| 27 | Dynamic type | Dynamic on=1, Dynamic depth=1 | ok (-8.2 dB) | ok (0.146%) |
| 28 | Dynamic listen | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-3.6 dB) | ok (0.152%) |
| 29 | VU model |  | excused (-138.5 dB) | ok (0.127%) |
| 30 | Rise |  | excused (-138.5 dB) | ok (0.127%) |
| 31 | Fall |  | excused (-138.5 dB) | ok (0.127%) |
| 32 | Overshoot |  | excused (-138.5 dB) | ok (0.127%) |
| 33 | Peak hold |  | excused (-138.5 dB) | ok (0.127%) |
| 34 | Value hold |  | excused (-138.5 dB) | ok (0.127%) |
| 35 | Clip hold |  | excused (-138.5 dB) | ok (0.127%) |
| 36 | Peak lamp threshold |  | excused (-138.5 dB) | ok (0.127%) |
| 37 | Red zone |  | excused (-138.5 dB) | ok (3.331%) |
| 38 | Comfort zone |  | excused (-138.5 dB) | ok (0.127%) |
| 39 | Analog RMS |  | excused (-138.5 dB) | ok (0.127%) |
| 40 | Weighting |  | excused (-138.5 dB) | ok (0.196%) |
| 41 | Force mono |  | ok (-4.7 dB) | ok (0.262%) |
| 42 | Trim link | Trim L=1 | ok (-0.6 dB) | ok (0.275%) |
| 43 | In place | Monitor=0.4 | ok (-3.8 dB) | ok (0.127%) |
| 44 | Show values |  | excused (-138.5 dB) | ok (0.368%) |
| 45 | Show hold |  | excused (-138.5 dB) | ok (0.303%) |
| 46 | Show tools |  | excused (-138.5 dB) | ok (0.599%) |
| 47 | Show label |  | excused (-138.5 dB) | ok (0.154%) |
| 48 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (11.790%) |
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
