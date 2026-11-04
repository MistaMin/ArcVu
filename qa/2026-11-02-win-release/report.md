# Plugin QA: VST3: ArcVu

| Check | Status | Detail |
|---|---|---|
| loads | PASS | VST3: ArcVu (OpenGrid), 59 parameters |
| finite output | PASS | peak -3.2 dBFS |
| not silent | PASS | rms -14.9 dBFS |
| deterministic render | PASS | repeat differs by -240.0 dB |
| editor opens | PASS | 1115x315, 0.0% animates by itself |
| every parameter changes the sound | PASS |  |
| every parameter is shown in the GUI | FAIL | Peak, AES-17, RMS window, Clip threshold, Trim R, Polarity L, Polarity R, Mute R, Monitor, High-pass, High-pass slope, Low-pass, Low-pass slope, Mono frequency, Mono amount, Dynamic frequency, Dynamic depth, Dynamic on, Dynamic type, Dynamic listen, VU model, Rise, Fall, Overshoot, Peak hold, Value hold, Clip hold, Peak lamp threshold, Comfort zone, Analog RMS, Weighting, Force mono, In place, Show values, Show hold, Show tools, Show label, Meters only, Drag sensitivity, Meter refresh, Bypass hidden processors, Tooltips, Stay above windows, Bypass, Wet, Delta |
| state survives save and reopen | PASS |  |
| editor reopens | PASS | 5 close/open cycles |

| # | Parameter | Context | Audio | Visual |
|---|---|---|---|---|
| 0 | 0 VU reference |  | excused (-240.0 dB) | ok (0.040%) |
| 1 | Peak |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 2 | Meter type |  | excused (-240.0 dB) | ok (8.390%) |
| 3 | Face |  | excused (-240.0 dB) | ok (100.000%) |
| 4 | Single meter |  | excused (-240.0 dB) | ok (2.331%) |
| 5 | Mid side | Trim M=1 | ok (1.5 dB) | ok (0.122%) |
| 6 | AES-17 |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 7 | RMS window |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 8 | Clip threshold |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 9 | Trim L |  | ok (-0.6 dB) | ok (0.051%) |
| 10 | Trim R |  | ok (-0.6 dB) | NOT SHOWN (0.000%) |
| 11 | Trim M | Mid side=1 | ok (1.5 dB) | ok (0.122%) |
| 12 | Trim S | Mid side=1 | ok (2.6 dB) | ok (0.071%) |
| 13 | Polarity L |  | ok (2.8 dB) | NOT SHOWN (0.000%) |
| 14 | Polarity R |  | ok (1.5 dB) | NOT SHOWN (0.000%) |
| 15 | Mute L |  | ok (-3.2 dB) | ok (0.034%) |
| 16 | Mute R |  | ok (-4.5 dB) | NOT SHOWN (0.000%) |
| 17 | Monitor |  | ok (-3.8 dB) | NOT SHOWN (0.000%) |
| 18 | High-pass | High-pass slope=1 | ok (-6.8 dB) | NOT SHOWN (0.000%) |
| 19 | High-pass slope |  | ok (-4.4 dB) | NOT SHOWN (0.000%) |
| 20 | Low-pass | Low-pass slope=1 | ok (-5.1 dB) | NOT SHOWN (0.000%) |
| 21 | Low-pass slope |  | ok (-14.7 dB) | NOT SHOWN (0.000%) |
| 22 | Mono frequency | Mono amount=1 | ok (-15.9 dB) | NOT SHOWN (0.000%) |
| 23 | Mono amount |  | ok (-14.0 dB) | NOT SHOWN (0.000%) |
| 24 | Dynamic frequency | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | NOT SHOWN (0.000%) |
| 25 | Dynamic depth | Dynamic on=1, Dynamic type=0.5 | ok (-5.2 dB) | NOT SHOWN (0.000%) |
| 26 | Dynamic on | Dynamic depth=1, Dynamic type=0.5 | ok (-5.2 dB) | NOT SHOWN (0.000%) |
| 27 | Dynamic type | Dynamic on=1, Dynamic depth=1 | ok (-7.8 dB) | NOT SHOWN (0.000%) |
| 28 | Dynamic listen | Dynamic on=1, Dynamic depth=1, Dynamic type=0.5 | ok (-3.6 dB) | NOT SHOWN (0.000%) |
| 29 | VU model |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 30 | Rise |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 31 | Fall |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 32 | Overshoot |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 33 | Peak hold |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 34 | Value hold |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 35 | Clip hold |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 36 | Peak lamp threshold |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 37 | Red zone |  | excused (-240.0 dB) | ok (4.463%) |
| 38 | Comfort zone |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 39 | Analog RMS |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 40 | Weighting |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 41 | Force mono |  | ok (-4.5 dB) | NOT SHOWN (0.000%) |
| 42 | Trim link | Trim L=1 | ok (-0.6 dB) | ok (0.051%) |
| 43 | In place | Monitor=0.4 | ok (-3.2 dB) | NOT SHOWN (0.000%) |
| 44 | Show values |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 45 | Show hold |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 46 | Show tools |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 47 | Show label |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 48 | Meters only | Bypass hidden processors=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | NOT SHOWN (0.000%) |
| 49 | GUI size |  | excused (-240.0 dB) | ok (100.000%) |
| 50 | Drag sensitivity |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 51 | Meter refresh |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 52 | Bypass hidden processors | Meters only=1, High-pass slope=1, High-pass=1 | ok (-4.2 dB) | NOT SHOWN (0.000%) |
| 53 | Tooltips |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 54 | Stay above windows |  | excused (-240.0 dB) | NOT SHOWN (0.000%) |
| 55 | Bypass | random state 1 | CONDITIONAL (-3.2 dB) | NOT SHOWN (0.000%) |
| 56 | Bypass | random state 1 | CONDITIONAL (-3.2 dB) | ok (0.776%) |
| 57 | Wet | random state 1 | CONDITIONAL (-3.2 dB) | NOT SHOWN (0.000%) |
| 58 | Delta |  | ok (-3.2 dB) | NOT SHOWN (0.000%) |
