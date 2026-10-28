# Loudness panel and bounce log: scope ledger

These checks ran on the build then labelled 1.1.1. The release was renumbered 1.0.0 afterwards with no code change, so every row applies to 1.0.0.

Sources read in full for this work: ITU-R BS.1770-5 (K-weighting Tables 1 and 2, gating, Annex 2 true-peak FIR),
EBU Tech 3341 (2023) and EBU Tech 3342. EBU R 128 itself was not read; Tech 3341 carries what the meter needs from it.

| Item | Evidence | Status |
|---|---|---|
| K-weighting at 48 kHz = BS.1770 Tables 1, 2; other rates by the analogue prototype | `loudness::tests::k_weighting_at_other_rates...` (prototype at 48 kHz matches the table), 44.1/88.2/96 kHz read -23.0 | verified |
| Tech 3341 cases 1, 2 (M, S, I of the -23 and -33 dBFS tones) | unit test | verified |
| Cases 3, 4 (relative gate) | unit test | verified |
| Case 9 (S of alternating tones), case 12 (M of alternating tones) | unit test | verified |
| Cases 10, 13 (maximum M and S over bursts at offsets) | unit test, file-based form only | verified |
| Cases 15-19 (true peak, +0.2/-0.4 dB) | unit test with 10 ms tapers | verified |
| Tech 3342 cases 1-4 (LRA 10, 5, 20, 15 +/-1 LU) | unit test | verified |
| Case 11 and 14 (live meters, 20 successive maxima), 7, 8 (the 5.0 channel and the 3-channel cases) | not run; ArcVu measures one or two channels | not run |
| Cases 20-23 (true peak, 4x synthesised then decimated) | not run | not run |
| Authentic programmes 1, 2 (Tech 3341 cases 5/6 and Tech 3342 cases 5/6) | the EBU files were not downloaded | not run |
| Panel draws on every face and both skins; buttons answer clicks; settings persist | UI audit (0 failures), `loudness_panel_paints_on_every_face...` | verified |
| Loudness follows the audio that reaches `process_io` | `loudness_follows_the_audio_that_reaches_the_meter` | verified |
| Window sizing with the panel on and off | UI audit snapshots | verified (rendered, not interacted in a host) |
| Offline bounce log: no file for a real-time render, a summary and a CSV for an offline one, switch off | UI audit with `setNonRealtime`, `bounce_log_measures_the_output_and_writes_a_row_per_second` | verified in the harness |
| Offline flag as reported by REAPER (VST3) | hidden REAPER, offline render of a 12 s step file (-30, -18, -24 dBFS): the summary, CSV and HTML were written; integrated -21.5 LUFS (expected -21.6), maxima -18.0, per-second rows follow the steps | verified for REAPER VST3 only |
| Offline flag in Logic, Pro Tools, Studio One and the AU, CLAP, LV2, AAX formats | not run | not run |
| REAPER host suite, VST3, 1.1.1 build | three runs in a hidden REAPER: run 1 failed (editor window not found, state after reopen, six conditional parameters), runs 2 and 3 passed all nine checks; an earlier release build run in between passed all nine. Run 1's failure was not reproduced and its cause was not found (logs/reaper-suite-vst3.log) | PASS in 2 of 3 runs; open: cause of the first failure |
| REAPER host suite, CLAP, 1.1.1 build | all 9 checks PASS | logs/reaper-suite-clap-f.log | verified |
| REAPER host suite, AU, 1.1.1 build | 8 of 9 PASS; "state survives save and reopen" FAIL lists only switches set to 0.49 that the AU wrapper rounds to on (the known AU limit in qa/2026-10-26-release-qa/host-notes.md); the exact-value second route was not re-run | logs/reaper-suite-au-f.log | PASS with a known harness limit |
| REAPER host suite, LV2, 1.1.1 build | 7 of 9 PASS; "every parameter changes the sound" lists seven conditional controls and "state survives" lists values snapped to the LV2 step, both the known LV2 limits; contexts and step-aligned second routes were not re-run | logs/reaper-suite-lv2-f.log | PASS with known harness limits |
| REAPER host suite, VST3, first run of that batch | editor check SKIPPED ("no editor window found"), the visual sweep not run, everything else PASS. The same skip happened in the first run of the earlier session; later runs passed all 9. It recurs on the first suite after plug-ins are replaced; the cause is not found | logs/reaper-suite-vst3-f.log | open |
| auval (aufx ArcV OpGr) | AU VALIDATION SUCCEEDED | logs/auval.log | verified |
| AAX validator on this build | no signed AAX build was made | n/a | not run |
| Comparison against another loudness meter in a DAW | not done | not done |
