# AAX page table and VST3 editor check: scope ledger

Build: 1.0.0 sources plus the page-table change (`Resources/ArcVu.xml`, `ArcVuAAXExtensions` in `plugin/PluginProcessor.h`, a post-build copy in `CMakeLists.txt`). No parameter ID, `ArcVuSetup` field or Rust code changed.

| Item | Evidence | Status |
|---|---|---|
| `HANDOFF.md` said no release was published | edited to say a private pre-release v1.0.0 exists, no public release | done |
| AAX page table shipped in `Contents/Resources` | bundle listing after rebuild | verified (unsigned local build) |
| AAX Validator `test.page_table.load` | without the XML: FAIL "Failed to load page tables library" (logs/aax-noxml); with it: PASS (logs/aax-unsigned) | verified, unsigned build |
| AAX Validator `test.page_table.automation_list` | FAIL without the XML, PASS with it. Warning: 56 automatable parameters are not on a page of the table (it lists 8 controls) | passes; warning open |
| AAX Validator `test.describe_validation` | PASS, warning "gaps between registered fields" | passes; warning open |
| Other validator tests (`test.parameters`, `test.parameter_traversal`, `test.load_unload`, DTT-based tests) | not run in this pass | not run |
| PACE-signed AAX still carries the XML and passes | the page table is copied after JUCE's build; the signed release bundle was not rebuilt | not run |
| AAX in Pro Tools | not run | not run |
| Windows page-table layout and plug-in IDs | the IDs `jcbb` and `jccc` come from reading the JUCE source and match the validator's pass on macOS | not run on Windows |
| `cargo test --manifest-path dsp/Cargo.toml` | 72 passed | verified |
| UI audit (`ArcVu_QA`) | FAILURES 0 | verified |
| Hidden REAPER would not start (the cause of the earlier launch timeouts) | Two causes found. 1: a macOS "reopen windows after a crash" dialog, left by REAPER crashing in a plug-in scan, blocked the main thread (stack sample). Fixed by `defaults write com.cockos.reaper ApplePersistenceIgnoreState -bool YES` for the run, removed afterwards. 2: with no seeded config REAPER rescans every plug-in and one third-party plug-in crashes the scan; the seeded copy of the user's REAPER settings fixes it | verified (logs/launch-vst3-e.log) |
| REAPER host suite, VST3, 4 runs on this build | runs 1, 2 and 4: load, finite, not silent, deterministic, every parameter changes the sound, state survives save and reopen, editor reopens 5 cycles all PASS; **"editor opens" SKIPPED** ("no editor window found"). Run 3: all PASS including "editor opens 1375x368". Logs: `logs/reaper-suite-vst3.log`, `-run2`, `-run3`, `-run4` | PASS in all but one check; editor check ran in 1 of 4 runs |
| Cause of the skipped editor check | The harness hides the REAPER process, and the stock window list (`winlist.swift`, on-screen windows only) then returns an empty list whenever REAPER is still hidden when the editor opens, so no window is found. Run 4 showed the empty list for the whole run. Whether REAPER is un-hidden by the time the editor opens varies from run to run, which is why the skip looked random, and why it seemed to follow a reinstall. Evidence is the empty list in run 4 and the result of the fix below | cause found; shown by the fix, not by isolating the un-hide |
| Fix: `qa/2026-10-26-release-qa/hidden_reaper.py` uses `winlist_all.swift` (every window of the process, on-screen or not) | with it, REAPER hidden (hidden-check: true): run 5, run 6 and run 7 each PASS all 9 checks, including "editor opens 1375x368" and "every parameter is shown in the GUI". Runs 6 and 7 followed a fresh copy of the VST3 into the user folder. Logs: `logs/reaper-suite-vst3-run5..7.log` | verified for VST3 in REAPER, 3 of 3 runs |
