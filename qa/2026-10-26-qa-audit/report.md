# ArcVu QA audit — 2026-10-26

Scope: **126 rows: 83 verified, 18 failed, 23 unverified, 2 out of scope.**
The ledger accounts for the previous S1–S29 and L1–L10 scope, splits partial claims, and adds one UI/state
row for each of the unchanged 55 parameters. **This is an accurate audit, not a claim of 100% product completion.**

Build: uncommitted changes over private `main` at `53e22d4` / `v0.1.1-alpha`. Version stays 0.1.1 Alpha.
No commit, push, tag, public visibility change, signing, system install or release upload.

## What changed

- `plugin/ParameterPanel.h` provides direct JUCE slider/combo/button attachments, grouped into the
  existing Type, Face, Reference, Meter, I/O, Tools and More entry points. Settings are paged within
  the face, and scale uniformly at 100/125/150%. All 55 existing parameters have visible routes.
- `dsp/src/lib.rs` draws a tight 1100×220 shallow arc ribbon, dense 1100×248 horizontal face and
  compact 420×700 vertical face with square lamps, a wrapped header/key row, neutral hardware caps,
  numeric peak readouts, dim coloured lamps and a separate phase strip. Scale captions use enough
  precision to avoid repeated rounded labels. No third-party marks or skins are introduced.
- `plugin/PluginEditor.cpp:118` limits labels by UTF-8 bytes rather than characters, avoiding the
  previous possibility of copying more than 31 bytes. Native typing/focus still needs verification.
- `plugin/PluginProcessor.cpp:347` snapshots current host values before saving and replaces reused
  state buffers. Restore closes stale menu state and terminates the label buffer.
- `dsp/src/lib.rs:208` resets numeric hold along with peak. Two new Rust regression tests cover
  numeric reset and unique captions. The detector's signal processing and all host IDs/setup layout
  are otherwise preserved, including the measured defects below.
- `qa/UiAudit.cpp` and opt-in CMake target `ArcVu_QA` make the component-level checks repeatable.

## Checks

| Check | Result | Evidence / limit |
|---|---|---|
| Rust unit tests, Debug | PASS — 18/18 | rust-tests.log |
| Rust unit tests, Release | PASS — 18/18 | rust-tests-release.log |
| Release Standalone, VST3 and AU builds | PASS | build.log |
| C++ Debug QA build | PASS | build-debug.log |
| Release UI and state harness | PASS — 55/55 | ui-audit.log: FAILURES 0 |
| Debug UI and state harness | PASS — 55/55 | ui-audit-debug.log: FAILURES 0 |
| Two-way GUI/parameter binding | PASS | Every slider/combo/button changed the correct parameter; opposite direction also checked |
| Parameter state roundtrip | PASS — 55/55 | Legal discrete values; tests use the actual ArcVuSetup save/restore path |
| Save while audio stopped / reused buffer | PASS | Explicit regression cases in both harness configurations |
| Editor lifecycle | PASS — ten reopenings/configuration | Component harness, not a DAW lifecycle claim |
| Parameter IDs and C/Rust setup layout | PASS — unchanged | contract-check.json; C header unchanged |
| AU published parameter names | PASS — all 55 | Final auval lists 55 parameters matching the inventory |
| AU validation | PASS | Final auval.log: AU VALIDATION SUCCEEDED |
| UI visual review | PASS for scoped renders | 78 PNGs: all faces at 100/125/150%; every settings page at 100/150%; single/M/S/meters-only states |
| Native mouse/drag/keyboard/popup acceptance | UNVERIFIED | CUA repeatedly returned noWindowsAvailable; native-ui.md |
| Nine-type × three-face visual matrix | UNVERIFIED | Captions tested; full rendered matrix not captured |
| Detector diagnostics | FAIL — three reproduced defects | detector-audit.log; intentionally exits 1 |
| REAPER host suite | SKIPPED / unverified | New test instance failed hidden-state check; stopped only that instance |
| pluginval / VST3 validator | SKIPPED | pluginval absent on PATH; no validator download performed |
| AAX | OUT OF SCOPE | HANDOFF.md excludes it from the normal build |

UI review used the final build's images, with contact sheets and full-size checks of dense/compact
pages and the vertical single-state failure. Rendered pages are distinguished from native input.
Not captured: lit Overs state, every visibility combination, all nine type/face combinations,
and native popup placement. These omissions remain unverified in the ledger.

## Confirmed remaining failures

The detailed list is `findings.md`. Two signal tests still reproduce:

1. **Stereo weighting crosstalk:** silent R reads -5.35976 dB with A weighting, versus -158.99 dB
   unweighted, because the meter filter memory is shared between channels.
2. **Analog K-14 calibration:** a -18 dBFS sine reads +31.9952 rather than -4 dB: 36 dB high.
3. **Fixed in iteration 1 — dynamic HPF/LPF reversal:** HPF listen now passes highs and rejects lows; LPF does the inverse. Both pass 50 Hz/8 kHz signal tests in Debug and Release. See iteration-01-full-rust.log.

The source audit also finds incomplete weighting/reference fidelity, preset/file/global operations,
M/S trim-link semantics, some visibility/activity/monitor gestures and the vertical single secondary
lamp layout. Timing, routing and concurrent/realtime safety need further evidence. Failed and
unverified rows remain open; no blanket exceptions or thresholds were used to hide them.

## Test corrections and limitations

The first UI/state run's 21 boolean state failures came from the harness supplying fractional,
non-canonical normalized values to JUCE boolean parameters, then comparing raw floats after the
boolean state was serialized. JUCE's bool getter canonicalizes those to On/Off. The final harness
sets legal stepped values before testing exact restoration; the original log is retained as
`ui-audit-initial.log`. This fixes invalid test inputs, not a product threshold or a missing check.

The caption uniqueness regression initially caught the EBU ruler producing two "0" labels around
zero. The formatter was corrected, and all 40 captions are now distinct for all nine types.
Existing unused-code/unused-mut Rust warnings remain; no warning suppression was added.
The Debug C++ harness links the CMake-built Release Rust library; Rust's own tests separately run
in Debug and Release. No TSan or real-time allocation trace was run.

## Environment and cleanup

See `qa/PROJECT.md` for repeat commands and host constraints. Temporary user AU/VST3 copies created
for validation were removed after the final AU check; the built bundles remain under `build/`.
No existing user plugin copy was replaced (neither path existed before this pass). The REAPER test
instance was isolated and stopped when it did not stay hidden. No user DAW session was touched.

An automatic approval-review usage-limit error interrupted the final validation/cleanup command
once. That command did not run. After the user's "continue", validation and cleanup succeeded.

## Next work

Read `ledger.md` and `findings.md`. Repair the confirmed detector defects and remaining feature/UI
semantics with focused regression tests; then repeat native input, hidden host, audio/visual sweep,
state/project lifecycle and timing/scale matrices. The present face pass and accurate ledger must
not be described as a fully verified release.
