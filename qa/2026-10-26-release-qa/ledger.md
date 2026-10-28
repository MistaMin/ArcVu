# ArcVu release QA ledger, 2026-10-26

Baseline: `main` at `1785216` (release 1.0.0 work, committed locally, not pushed) plus the QA files themselves. Version 1.0.0, developer OpenGrid
(bundle `com.opengrid.arcvu`, manufacturer `OpGr`). This ledger covers what was added or
changed after `qa/2026-10-26-qa-audit/ledger.md` (140 rows: 138 verified, 2 out of scope), which still stands for the
detector, presets and parameters. Evidence paths are relative to `qa/2026-10-26-release-qa/`.

Status counts: skipped 3, unverified 4, verified 27. Total 34 rows (counted from the rows).

| ID | Item | Source | Code | GUI | Test | Evidence | Status |
|---|---|---|---|---|---|---|---|
| N1 | RMS meter reads absolute dBFS (AES-17 adds 3 dB), same as the reference meter | user, reference meter manual | dsp/src/lib.rs reading() | RMS type | rms_meter_reads_absolute_dbfs_whatever_the_reference; live 1 kHz -12 dBFS tone in REAPER next to the reference meter: -15.0 RMS, -12.0 peak on both | logs/rust-debug.log, logs/rust-release.log; live comparison done in the user's REAPER on 2026-10-26 (screenshots in the session, not saved here) | verified |
| N2 | Value hold defaults to 5 s | user | PluginProcessor.cpp valueHoldP, dsp default | Meter settings > Value hold | REAPER param default 0.1935 = 5.0 s on a range of -1..30 | reaper/report.json | verified |
| N3 | Skin selector (Modern, Classic) in Display, remembered between sessions | user | PluginProcessor setSkin, ParameterPanel group 2 | Display > Skin | --categories-global-test restores skin; renders | logs/categories-global.log; shots/classic-*.png | verified |
| N4 | Three colour themes, remembered | user | dsp palette(), classic_face() | Display > Colour | categories-global restores theme; renders in 3 colours x 3 faces | logs/categories-global.log; shots/theme-*.png, shots/classic-*.png | verified |
| N5 | Classic needle dials for all nine meter types | user | dsp needle_meter | all faces | 27 renders (9 types x 3 colours) viewed | shots/classic-kind-<type>-theme-<colour>.png, all 27, from this build | verified |
| N6 | Classic scale numbers are whole numbers; VU prints -20,-16,-12,-8,-4,0,+4; end value always printed; K dials print "0 = -12/-14/-20 dBFS" under the title | user | needle_meter ticks | Classic dials | rendered and viewed | shots/classic-*.png | verified |
| N7 | Caution band joins the red zone as one continuous arc; no loose end ticks | user | needle_meter arc pieces | Classic dials | rendered and viewed | shots/classic-*.png | verified |
| N8 | Window resizes freely at the face aspect ratio; size remembered | user | PluginEditor constrainer, setZoom | all faces | zoom restored by categories-global test; scripted resize of the standalone window held the aspect ratio | logs/categories-global.log | verified (restore, scripted resize) |
| N9 | Dragging the window corner inside a real host | user | host resize | all faces | needs a visible host and a mouse drag | none | unverified (hidden-window rule; not done) |
| N10 | Tools menu is a dropdown of knobs: frequencies show at most three digits (12.5 for 12500, 380 for 380) | user | ParameterPanel group 6 | Tools | UI audit: every Tools row GUI to host and host to GUI | logs/ui-audit.log; shots (panel renders in ui output) | verified |
| N11 | Vertical face is 380 wide with both bars in one window and a centred scale | user | dsp stereo_vertical_meter | Vertical | rendered, viewed; user reviewed | shots/theme-*-face-2.png | verified |
| N12 | Phase correlation is a glowing line, left = out of phase | user | dsp phase_gauge | all faces | rendered, viewed; ui-audit phase checks | shots/theme-0-face-0.png | verified |
| N13 | Digital PEAK and HOLD readout in the header (bars on Vertical); Values and Hold switches hide them | user | dsp digital_panel, digital_row | header | show_values / show_hold unit tests on every face | logs/rust-release.log | verified |
| N14 | Single (sum) mode keeps the stereo interface; the second meter shows OFF | user | dsp inactive() | all faces, both skins | rendered on 3 faces | shots/face-*-single.png, shots/single-sheet.png | verified (user has not yet reviewed the final look) |
| N15 | "Made in Germany" mark | user | dsp paint() | all faces | rendered | shots/theme-0-face-0.png | verified |
| N16 | REAPER host suite on the final build, all four shipped formats, each in a hidden REAPER with one ArcVu instance | plugin-daw-test | n/a | n/a | VST3 and CLAP: all 9 checks PASS. AU and LV2: loads, finite, not silent, deterministic, editor, shown and reopen PASS; the "changes the sound" (AU) and "state" (AU, LV2) checks report FAIL because of harness limits, each re-checked by a second route (contexts, exact values) and passing | logs/reaper-suite-<format>.log; host-<format>/report.md; host-notes.md; exceptions.txt | verified (see host-notes.md for the two explained FAILs) |
| N17 | auval with the OpenGrid identity (aufx ArcV OpGr) | plugin-daw-test | n/a | n/a | auval -v aufx ArcV OpGr | logs/auval-opengrid.log | verified |
| N18 | Unit tests, Debug and Release | plugin-qa | dsp/src | n/a | cargo test x2: 57 passed | logs/rust-debug.log, logs/rust-release.log | verified |
| N19 | UI audit: 55 parameters GUI to host and back, all panels, all types x faces | plugin-gui-coverage | qa/UiAudit.cpp | all | 935 PASS, 0 FAIL | logs/ui-audit.log | verified |
| N20 | Realtime safety, state race, bad state, keys, presets, categories, detector audit | plugin-qa | qa/UiAudit.cpp | n/a | each mode exits 0 | logs/realtime.log, state-race.log, bad-state.log, keys.log, preset-file.log, categories-global.log, detector-audit.log | verified |
| N21 | Static GUI coverage | plugin-gui-coverage | n/a | n/a | gui_coverage.py: 55/55 bound (6 false UNKNOWN explained) | logs/gui-coverage.log | verified |
| N22 | Build warnings | plugin-qa | n/a | n/a | Release build: 0 warnings in the log | logs/build-release.log | verified |
| N23 | pluginval, clap-validator | plugin-daw-test | n/a | n/a | not installed; downloading needs the user's OK | none | skipped |
| N24 | Pro Tools, AAX Validator | plugin-daw-test | n/a | n/a | AAX builds universal and unsigned; the Validator and Pro Tools need the PACE-signed bundle and a system install | logs/universal-archs.txt | skipped (until signed) |
| N25 | Logic Pro (AU) | plugin-daw-test | n/a | n/a | Logic is not installed | none | skipped |
| N26 | Studio One, other hosts | plugin-daw-test | n/a | n/a | installed, but needs a visible host and manual steps | none | unverified |
| N27 | CLAP and LV2 builds | Make-Plugin-Release | CMakeLists.txt | n/a | built and tested in REAPER as part of N16 | host-clap/, host-lv2/ | verified (supersedes the earlier out-of-scope decision) |
| N28 | All six bundles are universal (arm64 + x86_64); the x86_64 code runs (keys test under Rosetta) | Make-Plugin-Release | CMakeLists.txt ARCVU_UNIVERSAL | n/a | lipo -archs on each binary; ArcVu_QA --keys-test under arch -x86_64 | logs/universal-archs.txt, logs/rosetta-keys-test.log | verified (Rosetta, not Intel hardware) |
| N29 | Changing window size, skin or colour leaves the other saved global preferences alone | QA finding | PluginProcessor saveGlobalKey | n/a | categories-global test: drag sensitivity and refresh rate unchanged after setZoom | logs/categories-global.log | verified (bug found and fixed in this run) |
| N30 | Standalone app has a custom icon | Make-Plugin-Release | CMakeLists.txt ICON_BIG/ICON_SMALL, Resources/ | n/a | check_standalone_icon.sh | logs/standalone-icon.log | verified |
| N31 | Unsigned installer, DMG and bundle archive build; payload lists all formats, manual and licences | Make-Plugin-Release | installer/build_macos.sh | n/a | dry run without SIGN | logs/pkg-dryrun-unsigned.log | verified (unsigned) |
| N32 | User manual PDF built from docs/USER_MANUAL.md with screenshots of this build and a parameter reference read from the source (55 parameters) | Make-Plugin-Release | tools/build_user_manual.py | n/a | built, 6 pages, pages viewed | ArcVu-User-Manual.pdf | verified |
| N33 | Developer ID signing, hardened runtime, notarisation and staple; PACE signature of the AAX (`wraptool sign`, `verify`); AAX Validator; Pro Tools | Make-Plugin-Release | installer/build_macos.sh SIGN=1 | n/a | signing environment pre-flight passed (certificates, AC_NOTARY, wraptool 6.0.1, iLok); the signing itself must run in the user's Terminal | logs/signing-env-preflight.log | unverified (user step) |
| N34 | Windows x64 and Linux x86_64 builds | Make-Plugin-Release | installer/build_windows.ps1, build_linux.sh, .github/workflows/build.yml | n/a | needs the OPENGRID_CI_TOKEN secret and a push; no Windows or Linux machine here | none | unverified |

## Exceptions (see exceptions.txt for reasons)
- 29 `audio:` exceptions for metering, display and preference parameters (they do not alter the audio path).
- `Meters only @ Bypass hidden processors=1.0, High-pass slope=1.0, High-pass=1.0`: its audio effect needs the hidden processors
  to be active. NEW this run.
- Conditional controls (Mid side, Trim M/S, High-pass, Low-pass, Mono frequency, Dynamic *, Trim link, In place, Bypass, Wet) were
  DEAD in the default state and passed as CONDITIONAL in random states 1, 3 and 5: they act only with other settings.

## Notes
- First host run had two ArcVu instances because the agent also called add_fx by hand; its state check failed. Cause confirmed by a
  clean run and a manual round trip, kept in `first-run-duplicate-instance/`. Not a plugin defect.
- Host suite ran in a hidden REAPER with its own settings folder; the user's REAPER session was not touched.
- Screenshots in shots/ come from the final build's UI audit; reaper/editor_base_*.png are the editor inside the hidden host.

## Exceptions added after the first run (NEW)
- Context lines for every conditional control, `audio:Free Wheeling` (LV2 wrapper port), and `Bypass hidden processors @ Meters only=1.0, High-pass slope=1.0, High-pass=1.0`.
- The AU and LV2 "state survives save and reopen" FAILs are not excused in the file: they are reported as FAIL with the second-route evidence in `host-notes.md`.
