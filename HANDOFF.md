# Handoff: ArcVu for iOS

Version 1.0.0 of the desktop plug-in is built, signed and tested (macOS VST3, Audio Unit, CLAP, LV2, AAX and Standalone; Windows and Linux builds exist from earlier CI). **The job now is an iOS version**: an Audio Unit extension (AUv3) and a standalone app that meter audio on iPhone and iPad, sharing the same Rust detector and drawing code. The repository is **private** and nothing is published. 1.0.0 has no public release yet.

## Ground rules from the owner

- Keep the repository private. Do not publish a release, tag, or make anything public unless the owner says so in plain words.
- Ask before pushing. Never force-push without being asked. Make commits in coherent chunks.
- No third-party product or brand names in code, comments, docs, commit messages or file names (format and platform names such as iOS, AUv3, AAX and JUCE, and licence texts, are fine).
- No AI tool or assistant names and no co-author or "generated with" lines anywhere, including commit messages.
- Do not change the 55 host parameter IDs or the layout of `ArcVuSetup`. Saved projects depend on them.
- Be honest in the QA ledger: list what ran, what did not, and why. "Skipped" is not "passed".

## What is in the repository

| Path | What it is |
|---|---|
| `dsp/` | Rust static library with a C ABI (`plugin/arcvu.h`). The detector (VU, RMS, K, PPM), the loudness engine (`loudness.rs`), the loudness panel drawing (`loudness_panel.rs`) and the drawing list for every face (`lib.rs`). No JUCE, no allocation on the audio path. |
| `plugin/` | JUCE glue. `PluginProcessor` owns the meter and the 55 parameters, `PluginEditor` replays the drawing list and handles clicks, `ParameterPanel.h` is the paged settings menu. |
| `qa/UiAudit.cpp` | Console app that builds the editor off screen, checks every parameter path and writes screenshots (`--manual-shots` writes the manual images). |
| `installer/`, `tools/` | macOS and Windows packaging, licence collection, the PDF manual builder. |
| `docs/` | `USER_MANUAL.md` (source of the PDF), `RELEASE.md`, images. |
| `qa/<date>-*/` | QA ledgers and logs. Read `qa/PROJECT.md` and `qa/2026-10-28-loudness/ledger.md` first. |

Build layout expected next to the repository: JUCE at `../audiogridder/JUCE` (JUCE 8), the drawing toolkit at `../instruments/GoodLookinDri` (private), CLAP extensions at `../clap-juce-extensions`. Rust (`cargo`) and CMake 3.22 or later are required.

## How it works

- The audio thread calls `arcvu_process` with interleaved samples. Parameters are copied into an `ArcVuSetup` struct (`arcvu_setup`) every block. The loudness meter reads the raw input, the bounce log reads the output.
- The editor does not draw widgets. It calls `arcvu_paint`, which returns a list of drawing commands (`ArcVuPaint`, at most 8192 per frame, text up to 47 bytes) that JUCE replays. Clicks are hit-tested in view coordinates (`PluginEditor::mouseDown`), using the same rectangles the Rust code draws. Face sizes come from `arcvu_view_size`: Arc 1100×252 (440 on the Classic skin), Horizontal 1100×292 (340 Classic) and Vertical 380×700, plus 168 points (252 on the Vertical face) when the loudness panel is on.
- Interface settings (skin, colour, peak scale, loudness panel, bounce log, window zoom) are not host parameters. They live in a small text file (`global.txt`, `ArcVuProcessor::saveGlobalKey`), shared by all instances.
- Offline renders are detected with `AudioProcessor::isNonRealtime()`. The bounce log is written by `ArcVuProcessor::finishBounce`, which runs on the message thread.

## Plan for iOS

1. **Rust targets.** `rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios`. Extend the `arcvu_dsp` rule in `CMakeLists.txt` to build the right slice for the iOS SDK, as it does for universal macOS. Xcode 16 is installed on the owner's Mac.
2. **JUCE formats.** Add `AUv3` and `Standalone` for iOS (`juce_add_plugin` supports both). Choose the deployment target, bundle ids (`com.opengrid.arcvu` for the app, a child id for the extension), the icon set, and the microphone permission text already in `CMakeLists.txt`.
3. **Touch and layout.** JUCE turns touches into mouse events, so the hit-testing works, but the faces are fixed-size desktop layouts. Decide how they fit: Vertical for iPhone portrait, Horizontal and Arc for landscape and iPad, scaling the whole view to the screen. The window constrainer, `setAlwaysOnTop` and tooltips do nothing on iOS; leave them out for that target. Drag-to-set knobs in the Tools menu need a touch check.
4. **Files and menus.** `juce::FileChooser` (preset load and save), `PopupMenu` and the user preset folder behave differently on iOS (document picker, sandbox). `userApplicationDataDirectory` for `global.txt` and `userDocumentsDirectory` for the bounce logs resolve inside the app container: enable file sharing (`UIFileSharingEnabled`) if the logs should be reachable in Files, or drop the log on iOS.
5. **Audio.** Standalone needs the microphone permission and an audio session category. As an AUv3 inside a host, offline rendering is reported through the host's offline flag; check that `isNonRealtime()` is true there before trusting the bounce log.
6. **QA.** Extend `qa/UiAudit.cpp` for the iOS build where it can run, run the simulator, then a real device. Record every step in a new `qa/<date>-ios/ledger.md`. Desktop checks to rerun after any shared change: `cargo test --manifest-path dsp/Cargo.toml` (72 tests), the UI audit (0 failures), and the host suites in `qa/PROJECT.md`.

Do not fork the detector or the drawing code for iOS. Keep one Rust library; add `cfg(target_os = "ios")` only where unavoidable.

## Things that already cost time

- `CMakeLists.txt` lists the Rust source files that trigger a rebuild (`ARCVU_RUST_SOURCES`). A new `.rs` file must be added there or edits to it will not rebuild the library.
- JUCE `FileOutputStream` appends to an existing file, so delete old screenshot files before rerunning the UI audit.
- `ArcVuPaint.text` holds 47 bytes. Longer labels are cut silently; split them.
- `Meter::process` is only the metering step. Test the whole path through `Meter::process_io`.
- The loudness meter is checked against the EBU test signals in `loudness.rs` tests. Do not change its gating or windows without rerunning them; the standards are ITU-R BS.1770-5 and EBU Tech 3341 and 3342.

## Open items on the desktop version

- No 1.0.0 release is published yet. The owner tests it privately first (`dist/release-1.0.0/macos/`, Developer ID signed, notarised, with the AAX PACE-signed).
- The AAX Validator page-table test fails (no page table is shipped); the validator's other tests crash on the owner's Mac. The AAX is not yet verified in Pro Tools for this build.
- The VST3 editor check was skipped in the host suite on the first run after the plug-ins were replaced, twice, and passed on repeats. The cause is not found.
- The offline bounce log is verified only in one host and format.
- Windows and Linux 1.0.0 packages are not built yet (the owner builds Windows on a laptop; the Linux CI is blocked by a GitHub billing limit).
- `qa/2026-10-26-face-qa/ledger.md` and `report.md` carry uncommitted edits that are not part of this work. Leave them for the owner.
