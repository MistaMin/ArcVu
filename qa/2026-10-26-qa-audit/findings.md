# ArcVu remaining findings

The face and parameter IDs are preserved. These are genuine gaps retained from the handoff scope,
not new release requirements. A failed row is not excused because its control now exists.

## F1 — Weighting leaks across stereo channels (confirmed)

`dsp/src/lib.rs process_io` processes the interleaved metering scratch buffer with a single
`weight_z` history. Left and right therefore share filter memory. The independent diagnostic
in `qa/UiAudit.cpp --detector-audit` drives a 1 kHz sine into L and exact silence into R:
unweighted right = -158.99 dB; A-weighted right = -5.35976 dB. Expected: right remains silent.
The current A/B/C/D/K code is also explicitly a one-pole/published-shape approximation, not a
standards-compliant set of cascades. The old ledger's S8 "done" was unsupported.

## F2 — Analog K-scale calibration is 36 dB high (confirmed)

With a -18 dBFS sample-peak sine and K-14, the ideal analog path reads +31.9952 dB instead of
-4 dB. `reading()` adds `14 - reference_db` to an already reference-aligned VU value;
reference_db is -18, so that adds +32 rather than the required -4 adjustment. This diagnostic
checks the actual processor-to-Rust path, not just the scale-caption helper. It remains failed.

## F3 — Dynamic HPF/LPF selection was reversed (fixed and regression-tested)

The branch named HPF (`dyn_type == 1`) calls the cascade with `highpass=false`; the LPF branch
uses `true`. In listen mode at 1 kHz, HPF output RMS is 0.141258 at 50 Hz and 0.0023885 at 8 kHz.
The filter flags are now correct. New signal tests exercise both modes at 50 Hz and 8 kHz in
Debug and Release. Narrow/wide BPF shape still needs its own response matrix; do not infer those
modes are correct from this test.

## F4 — Scale/reference and weighting fidelity incomplete (source audit)

Nordic/EBU TEST/reference marks are absent from `scale_caption()`. The helper linearly interpolates
40 captions. Unique fractional labels fix rounded duplicates but do not prove native PPM alignment.
The nine-type/three-face visual matrix has not been captured. Weighting compliance is not proved
by the existing test that merely compares a gain formula at two frequencies.

## F5 — Hold and linked-trim semantics incomplete (source audit)

Numeric peak follows the peak-lamp envelope; it has no independent numeric hold path. The numeric
meter hold reset is fixed and regression-tested in this pass. Linked trims currently only copy
L into R; a right/master gesture or M/S link is not implemented. Custom timing is tested for one
rise/fall case, not for every model and duration/infinity combination.

## F6 — Remaining UI behavior gaps (source audit and rendered evidence)

- Vertical single mode still uses the generic horizontal `second_lamps()` helper in a tall skinny
  rectangle; it becomes a bundle of vertical stripes, not a second vertical lamp column. See
  `screenshots/face-2-single.png`.
- The compact hardware row returns before the hidden M/S trim hint. A complete tools/trims/channel
  activity indication is also absent.
- `show_tools` is not a full channel-control visibility switch.
- The Monitor combo can select every mode but lacks the old click-active-mode-to-LR gesture.
- All 55 parameters have controls and pass callback binding/state tests. Real mouse, drag,
  keyboard, popup, focus and screen-reader interaction remain separate unverified requirements.
- The red Overs (>0) state and the complete visibility-toggle matrix have not been captured.

## F7 — Preset/global/standalone parity incomplete (source audit)

Preset menu supports five factory/reset actions, save/load default, and clipboard copy/paste.
It does not implement previous/next, categories, user preset browser, or arbitrary state-file
load/save. Reset changes only a subset of settings. The constructor never loads the saved default.
There is no selectable drag-mode setting or independent persistent global settings file.
Standalone full restore/topmost/file operations were not accepted merely because the app opens.

## F8 — Concurrent/realtime access needs a dedicated follow-up (unverified runtime risk)

`processBlock()` pulls/writes shared `setup` and mutates the Rust Meter. The UI also changes setup,
paints the same Meter and resets it without a coherent cross-thread snapshot/command mechanism.
There are per-block buffer allocations. Single-threaded harness tests and auval are not TSan,
allocation tracing, or a sustained concurrent editor/audio stress test. This concern is recorded
as unverified (B1), not a demonstrated crash or proof of realtime safety.

## Host/native limits

- Final AU validator passes. This does not establish DAW automation, save/reopen or VST3 loading.
- REAPER QA PID 3512 was launched separately with `open -n -g -j`, isolated reaper.ini and bridge.
  NSRunningApplication reported `hidden=false`; only this instance was stopped immediately.
- pluginval is not on PATH. No validator download was performed.
- The standalone was opened and its compact vertical face inspected. Native coordinate actions
  repeatedly failed with `noWindowsAvailable`. Offscreen callbacks are not relabelled as native
  mouse interaction. See `native-ui.md`.
