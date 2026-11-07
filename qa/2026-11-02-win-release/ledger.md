# ArcVu 1.0.0 — Windows QA + release build (2026-11-02/03)

Commit: b7b6990 (tag v1.0.0 == main HEAD), clean tree at start. Machine: Windows 11, MSVC (VS 2026), Ninja, Rust 1.99.1, Release x64.

## Fixes made (uncommitted, working tree)

1. `qa/UiAudit.cpp` — MSVC has no POSIX `setenv`/`unsetenv`; added `_putenv_s` wrappers under `#if defined(_WIN32)` (top of file).
2. `CMakeLists.txt` (~line 150) — `ArcVu_QA` did not link the Windows system libs the Rust std needs (`ws2_32 userenv ntdll bcrypt advapi32`); the plugin target had them, the QA target did not. Added under `if(WIN32)`.
3. `qa/UiAudit.cpp` `snapshot()` — blank screenshots on Windows: JUCE 9's software renderer defers drawing until the `Graphics` context is destroyed; the PNG was written while the context was still alive, so every offscreen shot was empty (219-byte PNGs). Scoped the `Graphics` so it is destroyed before `writeImageToStream`. Diagnosed via a temporary `--paint-probe` mode (FFI returned 549 valid commands; plain `fillAll` also produced 0 pixels until the context was scoped). Probe removed after diagnosis.
4. `build_win.bat` — new, modeled on HybridEQ's (vcvars64 + Ninja configure/build). Uncommitted by design.

The plugin's own CMake already used the correct Windows lib name (`arcvu_dsp.lib`); no RadarVU-style fix was needed. GoodLookinDri path (`../../instruments/GoodLookinDri`) matched the existing checkout.

## Tests

- `cargo test --release --offline` (dsp/): 72/72 PASS. Includes the ballistics evidence: VU response time/overswing (IEC 60268-17), 99%-in-300 ms step, EBU Tech 3205 integration/tone-burst/overswing/reversibility, PPM fall times (ITU table), K-scale calibration, EBU 3341/3342 loudness cases, true peak, sample-rate independence.
- ArcVu_QA console (Release): UI audit 971 PASS / 0 FAIL, "coverage count 55", editor lifecycle 10 reopens; 202 screenshots (all faces × meter types × themes × classic skin × every panel page × loudness panel) visually reviewed — all render correctly. `--preset-file-test`, `--categories-global-test`, `--state-race-test`, `--bad-state-test`, `--realtime-test` (0 allocations in 200 blocks), `--keys-test`, `--detector-audit` (K-14 reads -4.00 dB on the -18 dBFS fixture), `--default-load-test`: all FAILURES 0.
- ctest: not wired in this project (no `enable_testing()`); N/A.

## REAPER suite (REAPER 7.80 x64, fx "VST3: ArcVu")

report.md / report.json in this folder. Loads PASS ("VST3: ArcVu (OpenGrid)", 59 params — note: vendor string is OpenGrid on this build, not "Marcos Deida" as the macOS profile says), finite/not-silent/deterministic PASS, audio sweep PASS (with the repo's shipped exceptions.txt), state save/reopen PASS, editor reopens 5 cycles PASS.

Visual sweep: 15 params verified in the host window; 44 reported NOT SHOWN. Confirmed by a second route (visual_resweep2.py, fresh REAPER): the editor is 1.5× larger than the floating window at 150% DPI, so the bottom strip and right readouts are clipped out of the captures. With the window enlarged and the editor live, any parameter change produces a parameter-independent repaint difference of the meter area (~35k px, rows 118–420) — repaint nondeterminism (needle decay), which makes the per-parameter pixel comparison inconclusive in REAPER on this machine. GUI coverage for those 44 params is instead evidenced by the offscreen UI audit (all 55 params checked against the panels + reviewed screenshots) — the profile designates UiAudit as the authoritative path for this plugin. Verdict: PARTIAL, not a defect.

The suite also leaves REAPER's own appended params dirty (Bypass on, Wet 0.22, Delta on) after the run — a bypassed VST3 editor does not repaint in REAPER, which invalidated my first resweep. Tooling note for reaper_qa.py.

## Formats built (build/ArcVu_artefacts/Release/)

VST3, CLAP, LV2, Standalone (ArcVu.exe), AAX. VST3 class IDs valid 32-hex (moduleinfo.json). AAX is UNSIGNED (no PACE on Windows per release spec): build/ArcVu_artefacts/Release/AAX/ArcVu.aaxplugin, set aside, NOT in the zip and not tested in Pro Tools.

## Package

`C:\Users\marco\Developer\Release Binaries\MistaMin\ArcVu\1.0.0\ArcVu-1.0.0-windows-x64-bundles.zip` (+ .installs.json sidecar) and `opengrid-manifest.json` (schema 2, windows/x64, min_os 10.0, build_class modern, sha256 152ababc…). Zip: Common Files/{VST3,CLAP,LV2} + OpenGrid/ArcVu licenses; no AAX, no Standalone (owner decision), no junk files. Note: the VST3 bundle contains a `desktop.ini` from the build tree (harmless Explorer artifact; spec bans Thumbs.db/.DS_Store/._* only). Tools from BlueChannel (make_bundle_zip.ps1, write_opengrid_manifest.py) — provenance noted; the repo has none of its own. No RELEASE-READY, nothing uploaded, nothing committed.

## Not run / needs the user

- Numeric readout checks: not run — skill requires asking the user which readout behaviour they want first.
- Performance checks (same-machine old/new comparison): not run — no previous Windows build exists to compare against; the change is a port, not a perf change.
- Windows code signing: Section 6.6 says ask the user — nothing is signed.
- AAX: unsigned and unverified in Pro Tools.
- LV2/CLAP host runs: built and packaged; not host-tested on Windows (REAPER suite covers VST3 only).
