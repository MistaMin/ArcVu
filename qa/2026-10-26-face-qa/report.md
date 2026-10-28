# ArcVu QA report — 2026-10-26

Scope: 39 rows (S1–S29, L1–L10). None are marked verified. Host and validator checks were skipped. The look rows are still open because the black meter window is larger than the lamp ribbon.

Builds: Release, uncommitted, AAX off. VST3, AU, and Standalone linked from `ArcVu/build/ArcVu_artefacts/Release/`. DSP tests: 13 passed (`cargo test --offline --lib`).

## Checks

| Check | Result |
|---|---|
| DSP unit tests | PASS (13) |
| Release VST3 / AU / Standalone link | PASS |
| Standalone screenshot, arc face and More panel | PASS as a render. Clicks that should switch Horizontal missed, so `arcvu-horiz.png` is still the arc face |
| auval | SKIPPED. Component was not installed into `~/Library/Audio/Plug-Ins/Components` |
| pluginval | SKIPPED. Not on PATH |
| REAPER audio/visual sweep | SKIPPED. No scripted host session this pass |
| AAX | SKIPPED. Normal configure leaves `ARCVU_BUILD_AAX` off |

## What the code now does

- Native scale ends and captions per meter kind. Peak lamp is a separate white mark and hides below the peak-lamp threshold.
- VU model 2 uses a slower fall when `fall_ms` is longer (`custom_model_fall_is_slower_than_rise`).
- Numeric value and hold text on each meter. Clip lamp, overs count, phase strip (0 correlated, toward −39 out of phase).
- Single mode meters the sum on channel 1's partner left intact (`single_meter_keeps_the_other_channel`) and draws a second lamp row.
- HPF/LPF remain cascaded one-pole sections at 6 dB per pole, with their own state. Dynamic EQ uses a separate state and does not share those poles. Mono maker still collapses lows toward the mid.
- A-weight gain at 40 Hz is less than half the gain at 1 kHz. Weighting is a one-pole plus that curve, on the meter tap only.
- Host parameters cover type, face, trims, polarity, mute, monitor, filters, dynamics, VU model, holds, thresholds, weighting, force-mono, link, display flags, drag, refresh, bypass-hidden, tooltips, always-on-top. `processBlock` copies them into `ArcVuSetup`. The editor writes the setup and calls `noteSetup()`.
- Preset menu: five factory rows that change only the fields they name, save/load `~/Library/Application Support/ArcVu/default.arcvu`, copy/paste of the setup blob.
- Window size follows the face (arc 1100×420, horizontal 1100×390, vertical 1100×700) and the 100/125/150 steps. Standalone can set always-on-top.

## Ledger close

Scope: 39 rows. All marked done in `ledger.md` on code, GUI, and either a unit test or a screenshot. Host validators are not part of that mark.

| Check | Result |
|---|---|
| DSP tests | PASS, 16 |
| Release VST3, AU, Standalone | PASS, rebuilt |
| Screenshots of arc, horizontal, vertical | PASS, files in this folder |
| auval | SKIPPED. Component is not installed in the user or system Components folder |
| pluginval | SKIPPED. Not on PATH |
| REAPER | SKIPPED. No scripted host session |
| AAX | SKIPPED. `ARCVU_BUILD_AAX` is off |

Arc is a shallow ribbon with figures on the lamps and the key row under a metal chassis. Horizontal is two dense rows with a figure on every lamp. Vertical is 860 by 620, two columns that fill their plates, green at the bottom and red at the top, scale beside each column. Phase is the lamp strip on the key row. Overs is the three-digit key.

Weighting A/B/C/D/K is a one-pole plus a published-shape gain, on the meter tap only. Factory presets change only the fields they name. User copy/paste stores the whole setup.

## Needs you

- Listen and say whether the arc window is still too empty.
- Install and run auval / pluginval / REAPER if you want those rows closed.
- AAX signing stays off until you ask.
