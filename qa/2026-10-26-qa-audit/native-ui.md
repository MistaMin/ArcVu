# Native UI observations — 2026-10-26

The built Release standalone was opened through cua_repl using the exact app path under build/.
The actual 420×700 vertical face was visually observed with the two-line header, square lamp
columns, readouts, wrapped hardware keys and independent phase strip. The standalone's own yellow
"Audio input is muted" strip sits above the editor and is not part of the meter face.

The native app menu could be read. Quit was invoked through its AX menu action during restart.
Coordinate attempts to open the custom Type menu returned `noWindowsAvailable` on repeated fresh
app bindings. No crash report was found for ArcVu in the recent DiagnosticReports listing. This
is an automation failure, not proof that the product crashed or that the controls work natively.

The component-level QA harness invokes slider/combo/button callbacks synchronously, changes host
parameters in the opposite direction, pumps the message loop, renders each page, and saves/restores
state. These are valuable integration tests but do not certify real input, popup placement, native
keyboard/focus handling, tooltips, topmost behavior, or DAW-hosted editor operation.
