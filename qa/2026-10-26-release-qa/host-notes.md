# Host suite notes (final build, OpenGrid identity)

Each format ran in its own hidden REAPER (own settings folder, one ArcVu instance, `run_all_formats.sh`). Logs: `logs/reaper-suite-<format>.log`,
reports in `host-<format>/`.

| Format | loads | finite | not silent | deterministic | editor | params change sound | params shown | state | editor reopen |
|---|---|---|---|---|---|---|---|---|---|
| VST3 | PASS | PASS | PASS | PASS | PASS | PASS | PASS | PASS | PASS |
| CLAP | PASS | PASS | PASS | PASS | PASS | PASS | PASS | PASS | PASS |
| AU | PASS | PASS | PASS | PASS | PASS | see below | PASS | see below | PASS |
| LV2 | PASS | PASS | PASS | PASS | PASS | see below | PASS | see below | PASS |

## AU: "every parameter changes the sound" FAIL
Fourteen conditional controls (M/S trims, filters, mono maker, dynamic filter, Trim link, In place, Bypass hidden processors) were DEAD
in the default state and, unlike VST3 and CLAP, not found in the random states. They act only while another control is set. Re-run with an explicit
context for each (see `exceptions.txt`, `logs/reaper-suite-au-contexts.log`): every one changes the sound (PASS); Bypass hidden processors needs
Meters only and a filter (context added after that run).

## AU: "state survives save and reopen" FAIL
The suite sets every parameter to a distinct in-between value. The AU wrapper rounds boolean parameters to 0 or 1 when the project is reopened, so
a boolean set to 0.49 reads "off" before and "on" after. Second route with exact values (Mute L on, Meters only off, Dynamic on on, Trim link off,
Dynamic listen on, Single meter on, Show hold off): all identical after save and reopen. Not a plug-in defect; users cannot set a switch to 0.49.

## LV2
- "Free Wheeling" is a port added by the LV2 wrapper (transport state), not an ArcVu parameter: excused in `exceptions.txt`.
- State: LV2 exposes choices as numbers and the suite's values are off-step (1.74 comes back as 2.0, which is the plug-in snapping to the step).
  Second route with step-aligned values (Mute L, Dynamic on, Meter type 3, Face 2, Monitor 4, Trim L 6.5, High-pass slope 3, Single meter, Meters only):
  all identical after save and reopen.

## First run, two instances
`first-run-duplicate-instance/`: an earlier run had two ArcVu instances in the project because the agent also called add_fx by hand. Its state check
failed for that reason; clean runs pass.
