#!/bin/bash
# Runs the REAPER host suite for each shipped format in a fresh hidden REAPER (one instance per run, so the project never
# holds two ArcVu instances).
cd "$(dirname "$0")/../.."
Q=qa/2026-10-26-release-qa
for spec in "vst3|VST3: ArcVu (OpenGrid)" "clap|CLAP: ArcVu (OpenGrid)" "au|AU: ArcVu (OpenGrid)" "lv2|LV2: ArcVu (OpenGrid)"; do
  fmt=${spec%%|*}; name=${spec#*|}
  python3 $Q/hidden_reaper.py quit --qa-dir "$PWD/$Q/reaper" > /dev/null 2>&1; sleep 3
  rm -rf "$Q/reaper"
  python3 $Q/hidden_reaper.py launch --qa-dir "$PWD/$Q/reaper" > "$Q/logs/launch-$fmt.log" 2>&1
  python3 $Q/hidden_reaper.py suite --qa-dir "$PWD/$Q/reaper" --fx "$name" --exceptions "$PWD/$Q/exceptions.txt" --seconds 2 > "$Q/logs/reaper-suite-$fmt.log" 2>&1
  mkdir -p "$Q/host-$fmt"; cp "$Q/reaper/report.md" "$Q/reaper/report.json" "$Q/reaper/hidden-check.txt" "$Q/reaper/editor_base_0.png" "$Q/host-$fmt/" 2>/dev/null
done
python3 $Q/hidden_reaper.py quit --qa-dir "$PWD/$Q/reaper" > /dev/null 2>&1
echo finished > "$Q/logs/run_all_formats.done"
