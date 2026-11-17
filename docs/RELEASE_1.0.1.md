# ArcVu 1.0.1

A fix and polish release of the digital analogue meter by OpenGrid. Pre-release: it has been tested with the checks listed below, not yet in every host.

## What changed

- Saved sessions, presets and the copy/paste setup now carry a version number, so later builds can add settings without dropping the ones you saved. Sessions saved by 1.0.0 still load. Sessions saved by 1.0.1 do not restore their ArcVu settings in 1.0.0.
- Fewer rare crashes and glitches: the meter is locked between the audio thread and the editor, a fault inside the meter can no longer take the host down, large host blocks no longer allocate on the audio thread, and the window zoom is saved once a second instead of on every resize step.
- Lit keys and jewel lamps from GoodLookin DRI 0.2.1.
- The source builds without private repositories.

## Formats and requirements

macOS 11 or later, Apple Silicon and Intel: VST3, Audio Unit, CLAP, LV2, AAX and Standalone. Windows and Linux packages are not part of this release.

## Install

Open the `.pkg` (or the DMG that holds it) and choose the formats you want.

## Checks it passed

Rust unit tests (74), the offscreen editor audit (983 checks, 0 failures), the state, real-time, key and detector test modes, `auval` for the Audio Unit, and the REAPER host suite for CLAP (all 9 checks).

## Known limits

- In the REAPER host suite, VST3, Audio Unit and LV2 fail the "state survives save and reopen" check. The signed 1.0.0 release fails the VST3 check the same way in the same REAPER, and the same code passes in CLAP and in a headless round trip, so it is a host-side difference that has not been explained. The Audio Unit and LV2 results match what 1.0.0 showed in October.
- AAX is signed with PACE but has not been verified in Pro Tools. Studio One and Logic were not tested.
- `pluginval` and `clap-validator` were not run.

## Files

`ArcVu-1.0.1-macos.pkg`, `ArcVu-1.0.1-macos.dmg`, `ArcVu-1.0.1-macos.sha256`, `ArcVu-User-Manual.pdf`, `opengrid-manifest.json`.
