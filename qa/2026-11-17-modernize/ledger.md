# ArcVu QA, branch modernize-dri-0.2-and-fixes (88eddd6 + uncommitted UiAudit round-trip test), 2026-11-17

Scope: the two audit rounds' fixes (thread lock, FFI panic guard, slice metering, versioned state, settings lock, paint buffer, 1.0.1)
and the move to GoodLookin DRI 0.2.1 (lit keys, jewel lamps). No parameter, host id or DSP change.

| Step | Check | Result | Evidence |
|---|---|---|---|
| 3 | cargo test --locked, debug and release | PASS 74/74 each | logs/cargo-test-*.txt |
| 3 | ArcVu_QA UI audit (all panels, 55 params GUI<->host, skins, themes) | PASS 983, 0 FAIL | logs/qa-ui.log |
| 3 | --state-test (incl. new: distinct-value round trip, early processBlock, huge block, block-size independence) | PASS | logs/qa-state-test.log |
| 3 | --realtime-test, --keys-test, --bad-state-test, --state-race-test, --detector-audit, --preset-file-test, --categories-global-test, --default-load-test, --display-test | PASS, 0 FAIL each | logs/qa-*.log |
| 2 | gui_coverage.py (regex route) | 49/49 bound, 0 MISSING (6 ids declared another way are covered by the UI audit) | logs/gui-coverage.txt |
| 4 | Release build VST3, AU, Standalone, CLAP, LV2, QA | PASS | build/ |
| 5 | auval -v aufx ArcV OpGr | PASS (AU VALIDATION SUCCEEDED) | logs/auval.txt |
| 5 | pluginval, clap-validator | SKIPPED (not installed) | |
| 6 | REAPER CLAP | PASS all 9 checks | reaper-clap/report.md |
| 6 | REAPER VST3 | 7/9: "state survives save and reopen" FAIL, see below. First run also failed "every parameter changes the sound" (In place SILENT); rerun PASS, so a flake | reaper-vst3*/report.md |
| 6 | REAPER AU | 7/9: state FAIL (AES-17 and other booleans shown off->on) and "shown in GUI" FAIL (AES-17, RMS window). Same bool-display quirk as the October AU run; the NOT SHOWN pair did not appear in CLAP or VST3 | reaper-au/report.md |
| 6 | REAPER LV2 | 7/9: state FAIL, shown-in-GUI FAIL (Free Wheeling, Enabled = LV2 wrapper ports). October LV2 run failed the same state check | reaper-lv2/report.md |
| 7 | Screenshots reviewed: face 0 classic/modern, vertical face, Tools panel | OK: lit keys read on/off, nothing clipped | /private/tmp/arcvu-qa-run/ui |
| 8 | Pro Tools (AAX), Logic, Studio One | SKIPPED: AAX needs a signed build and system install; Logic absent | |
| - | Meter ballistics (rise, overshoot, fall, hold) | NOT RE-RUN: no detector code changed; Rust tests and --detector-audit pass. Numeric-readout policy not asked this pass | |

## VST3 state failure: not caused by this branch
The 1.0.0 release bundle (dist/release-1.0.0 stage) run through the same suite in the same REAPER fails the identical check with the
identical values (reaper-vst3-baseline100). The same processor code passes in CLAP (REAPER) and in the headless round trip
(--state-test, all 55 parameters at distinct values). October runs of 1.0.0 passed, so something in the host (REAPER version or its
VST3 state handling) changed since. Open: find out why REAPER does not apply the saved VST3 state.

## Exceptions
No new exceptions. exceptions.txt is the October file, copied unchanged.

## Notes
- Installed plug-ins were replaced during the run; afterwards VST3 and AU were put back to the signed 1.0.0 release bundles and the
  CLAP and LV2 copies this run created were removed.
- Another session's REAPER instances were running; they were left alone.
