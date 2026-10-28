# Handoff: ArcVu GUI

Version: 0.1.2 Alpha. The 0.1.2 build is the QA-complete detector and plug-in (ledger: 138 verified, 2 out of scope). The next work is the GUI.

## QA continuation — 2026-10-26 (uncommitted)

The current ledger is `qa/2026-10-26-qa-audit/ledger.md`; it supersedes the old unsupported "done"
claims. Read `report.md`, `findings.md` and `qa/PROJECT.md` before continuing.

The face work now has an original tight arc ribbon (1100×220), dense horizontal face (1100×248),
and compact vertical face (420×700) with wrapped keys/header. C++ `ParameterPanel.h` provides paged,
direct JUCE parameter bindings for all 55 existing IDs; no IDs or ArcVuSetup fields changed. The Rust
face still draws the meter and keys. Settings pages scale with the editor and remain reachable in
meters-only mode. Preset drawing/hits remain in Rust/editor code; the other header menus open the
new C++ parameter panels.

The UI/state harness passes Debug and Release, with 78 Release PNGs. This does not mean native
mouse/popup or DAW QA passed. Final auval passes. Three actual detector diagnostic failures and
remaining feature/UI gaps are listed in findings.md. Do not call the product 100% verified.

New regression fixes: UTF-8 label byte limit, save captures host changes while audio is stopped,
state replaces a reused MemoryBlock, restored state closes stale panels, and reset clears numeric
hold. Detector signal behavior/host IDs/layout remain as handed off, apart from that reset fix.
No version bump, commit, push, tag, signing, public exposure or system install occurred.

## Historical 0.1.1 baseline handoff (superseded where noted above)

Version: 0.1.1 Alpha. Repo: private `https://github.com/MistaMin/ArcVu`. Do not make it public. Do not decompile, or copy the wordmarks, logos or skins of, any third-party meter product. One metal finish is the look. Eight copied skins stay out.

## What you are taking over

The detector and the host parameters are in place. The job is the face. A person comparing ArcVu with a hardware lamp loudness meter should recognise the instrument: brushed metal, a black window only as big as the lamps, short square lamps, dense scale figures, and a hardware key row. The current faces are the right structure and still look like a software meter.

Screenshots from this version are in `qa/2026-10-26-face-qa/`:

- `arcvu-arc.png` — shallow ribbon, figures on the lamps, empty black inside the window beside the ribbon, metal under the window.
- `arcvu-horiz.png` — the closest. Two dense rows, a figure on every lamp, keys underneath.
- `arcvu-vert.png` — 860×620. Two columns fill their plates. The lamps are wide bars, not a narrow hardware column. Scale figures repeat because they are rounded.

## Where the drawing lives

The face is not drawn in C++. `dsp/src/lib.rs` `paint()` builds a GoodLookinDri `Scene`. `plugin/PluginEditor.cpp` replays those commands. Change the look in Rust, then rebuild.

- `view_size(face)` is the window. Arc 1100×300, horizontal 1100×248, vertical 860×620. The editor reads this through `arcvu_view_size` and resizes. GUI scale 100/125/150 multiplies that.
- Arc geometry is `well()`. Horizontal is `horizontal_meter()`. Vertical is `vertical_meter()`.
- Keys are `hardware_row()`. Hit testing is a second copy of those x positions in `PluginEditor.cpp` `mouseDown`. If you move a key, change both.
- Menu hit rectangles are also duplicated between `paint()` and `mouseDown`. Keep them in sync.
- Text size ≥ 15 uses Avenir Next, otherwise Avenir Next Condensed, in the editor replay of command kind 10. `box` is a Rust keyword. Do not name a local that.
- Colour is `0xAARRGGBB`. Lit lamps: red, yellow, green. Dim lamps stay visible in the same hue.

## Build

```text
export PATH="/opt/homebrew/bin:$PATH"
cargo test --manifest-path dsp/Cargo.toml --offline --lib
cmake --build build --config Release --target ArcVu_Standalone ArcVu_VST3 ArcVu_AU
```

JUCE is `/Volumes/Developer/Dev Projects/audiogridder/JUCE`. GoodLookinDri is `../../instruments/GoodLookinDri`. A normal configure leaves `ARCVU_BUILD_AAX` off. Do not sign, notarize, or install system-wide unless asked. Standalone binary: `build/ArcVu_artefacts/Release/Standalone/ArcVu.app`.

Screenshot the window with `screencapture -l <CGWindowID>`, not a rectangle. Window id comes from `CGWindowListCopyWindowInfo` for owner ArcVu. Clicks that the editor must see go through `CGEvent` after activating ArcVu. The standalone shows a yellow “Audio input is muted” banner, so editor y=0 is about 64 px below the window top when the window is at (60, 80).

## What to improve

1. Arc: the ribbon is short and the black window is wider than the lamps. Figures collide. Make the lamps a shallow hardware ribbon and put the figures on the curve without overlap.
2. Vertical: a tall narrow column of square lamps, green at the bottom, red at the top, scale beside it. The column should be the product. The current slot is 58% of a wide plate, so the lamps read as thick bars.
3. Horizontal: tighten the gap between the two rows and the key row. Keep a figure on every lamp.
4. Keys should read as hardware buttons, not rounded software chips. Overs is the three-digit key. Phase is the strip to its right: 0 correlated, −20 uncorrelated, −39 out of phase, red when poor.
5. Menus (Type, Face, Preset, Meter, I/O, Tools, More) are how the deluxe controls are reached. Restyle them so they do not fight the meter. Do not remove a control to make the face prettier. Every host parameter needs a visible control.
6. Look at the screenshot yourself before calling a face done. The ledger is `qa/2026-10-26-face-qa/ledger.md`. Look rows are L1–L10.

## Do not redo

Company Marcos Deida, bundle `com.marcosdeida.arcvu`, codes Mdei/ArcV. State is the `ArcVuSetup` blob plus the host parameters. `processBlock` calls `pullSetup()` from the parameters. The editor writes `setup` and then `noteSetup()`. If you add a control, add the parameter and both directions. The Rust struct and `plugin/arcvu.h` must stay the same layout.
