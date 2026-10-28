# Changelog

## 1.0.0

First release by OpenGrid.

- Meters: VU (IEC 60268-17), RMS (absolute dBFS, AES-17 option), K-12/K-14/K-20, and the DIN, Nordic, BBC and EBU peak programme meters. The EBU meter follows Tech 3205-E and has a slow mode.
- Faces: Arc, Horizontal and Vertical, each with a Modern lamp skin and a Classic analogue needle skin, in three colours (Graphite/Ivory, Amber, Ice).
- Digital PEAK and HOLD readout held for 5 s by default (adjustable); glowing phase correlation line; K dials print their 0 dBFS reference; Classic dials use whole-number scales.
- Peak scale (More > Display): the digital PEAK number can read the sample peak in dBFS, with 0 dBFS as 0 on every meter type. The default is the meter scale.
- Loudness panel (More > Display): momentary, short-term, a sliding window you choose (10 s by default), integrated, loudness range and true peak, measured to ITU-R BS.1770-5 and checked against the EBU Tech 3341 and 3342 test signals, with a selectable target, true-peak ceiling, EBU +9 / +18 scale and LUFS / LU units. It is off by default and adds no host parameters.
- Offline bounce log: when the host renders offline, ArcVu measures its output and writes a summary (integrated loudness, loudness range, maxima, true peak, sample peak, against your target and ceiling), a visual HTML report with loudness and true-peak charts, and a per-second CSV to Documents/ArcVu Bounce Logs. On by default, switch off in More > Display.
- Single (sum) mode keeps the stereo interface with the second meter off.
- Free window resizing at the face aspect ratio, remembered between sessions.
- Tools dropdown of knobs: high-pass, low-pass, mono maker and dynamic filter.
- AAX is filed under Sound Field in Pro Tools.
- Formats: VST3, Audio Unit, CLAP, LV2, AAX (macOS universal) and Standalone on macOS; VST3, CLAP, LV2 and Standalone on Windows and Linux.
- Pre-release history (0.1.x Alpha) was private.
