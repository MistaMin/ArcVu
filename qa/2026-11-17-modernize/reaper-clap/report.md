# Plugin QA: CLAP: ArcVu (OpenGrid)

| Check | Status | Detail |
|---|---|---|
| loads | PASS | CLAP: ArcVu (OpenGrid), 58 parameters |
| finite output | PASS | peak -3.8 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1080x368, 0.0% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | PASS |  |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | 0 VU reference |  | excused (-240.0 dB) | ok (2.053%) |
| 1 | Peak |  | excused (-240.0 dB) | ok (1.010%) |
| 2 | Meter type |  | excused (-240.0 dB) | ok (5.251%) |
| 3 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 4 | Single meter |  | excused (-240.0 dB) | ok (4.532%) |
| 5 | Mid side | Trim M=1 | ok (1.5 dB) | ok (4.652%) |
| 6 | AES-17 |  | excused (-240.0 dB) | ok (0.091%) |
| 7 | RMS window |  | excused (-240.0 dB) | ok (1.425%) |
| 8 | Clip threshold |  | excused (-240.0 dB) | ok (0.044%) |
| 9 | Trim L |  | ok (-0.6 dB) | ok (3.791%) |
| 10 | Trim R |  | ok (-0.6 dB) | ok (1.029%) |
| 11 | Trim M | Mid side=1 | ok (1.5 dB) | ok (4.652%) |
| 12 | Trim S | Mid side=1 | ok (2.4 dB) | ok (4.090%) |
| 13 | Polarity L |  | ok (2.3 dB) | ok (0.257%) |
| 14 | Polarity R |  | ok (1.0 dB) | ok (0.262%) |
| 15 | Mute L |  | ok (-3.8 dB) | ok (3.362%) |
| 16 | Mute R |  | ok (-5.0 dB) | ok (5.675%) |
| 17 | Monitor |  | ok (-4.5 dB) | ok (0.044%) |
| 18 | High-pass | High-pass slope=1 | ok (-5.0 dB) | ok (0.066%) |
| 19 | High-pass slope |  | ok (-4.4 dB) | ok (4.113%) |
| 20 | Low-pass | Low-pass slope=1 | ok (-3.8 dB) | ok (4.005%) |
| 21 | Low-pass slope |  | ok (-14.0 dB) | ok (3.865%) |
| 22 | Mono frequency | Mono amount=1 | ok (-15.8 dB) | ok (4.156%) |
| 23 | Mono amount |  | ok (-14.0 dB) | ok (0.157%) |
| 24 | Dynamic frequency | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | ok (0.077%) |
| 25 | Dynamic depth | Dynamic on=1, Dynamic type=0.5 | ok (-5.2 dB) | ok (3.993%) |
| 26 | Dynamic on | Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | ok (3.154%) |
| 27 | Dynamic type | Dynamic on=1, Dynamic depth=1 | ok (-8.2 dB) | ok (3.993%) |
| 28 | Dynamic listen | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-3.6 dB) | ok (4.130%) |
| 29 | VU model |  | excused (-138.5 dB) | ok (4.090%) |
| 30 | Rise |  | excused (-138.5 dB) | ok (4.090%) |
| 31 | Fall |  | excused (-138.5 dB) | ok (0.044%) |
| 32 | Overshoot |  | excused (-138.5 dB) | ok (0.879%) |
| 33 | Peak hold |  | excused (-138.5 dB) | ok (0.044%) |
| 34 | Value hold |  | excused (-138.5 dB) | ok (0.044%) |
| 35 | Clip hold |  | excused (-138.5 dB) | ok (0.044%) |
| 36 | Peak lamp threshold |  | excused (-138.5 dB) | ok (0.879%) |
| 37 | Red zone |  | excused (-138.5 dB) | ok (1.153%) |
| 38 | Comfort zone |  | excused (-138.5 dB) | ok (0.044%) |
| 39 | Analog RMS |  | excused (-138.5 dB) | ok (0.044%) |
| 40 | Weighting |  | excused (-138.5 dB) | ok (1.720%) |
| 41 | Force mono |  | ok (-4.7 dB) | ok (0.232%) |
| 42 | Trim link | Trim L=1 | ok (-0.6 dB) | ok (4.239%) |
| 43 | In place | Monitor=0.4 | ok (-3.8 dB) | ok (0.044%) |
| 44 | Show values |  | excused (-138.5 dB) | ok (0.044%) |
| 45 | Show hold |  | excused (-138.5 dB) | ok (0.044%) |
| 46 | Show tools |  | excused (-138.5 dB) | ok (0.044%) |
| 47 | Show label |  | excused (-138.5 dB) | ok (0.044%) |
| 48 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (10.575%) |
| 49 | GUI size |  | excused (-138.5 dB) | ok (100.000%) |
| 50 | Drag sensitivity |  | excused (-138.5 dB) | ok (100.000%) |
| 51 | Meter refresh |  | excused (-138.5 dB) | ok (100.000%) |
| 52 | Bypass hidden processors | Meters only=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | ok (100.000%) |
| 53 | Tooltips |  | excused (-138.5 dB) | ok (100.000%) |
| 54 | Stay above windows |  | excused (-138.5 dB) | ok (100.000%) |
| 55 | Bypass | random state 1 | CONDITIONAL (-4.0 dB) | ok (100.000%) |
| 56 | Wet | random state 1 | CONDITIONAL (-4.0 dB) | ok (100.000%) |
| 57 | Delta |  | ok (-3.8 dB) | ok (100.000%) |
