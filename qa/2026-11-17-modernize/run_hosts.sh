#!/bin/bash
# REAPER host suite for each format, one fresh hidden REAPER per format (never two ArcVu instances in one project).
cd "$(dirname "$0")/../.."
E=qa/2026-11-17-modernize
R=~/.claude/skills/plugin-daw-test/scripts/reaper_qa.py
for spec in "vst3|VST3: ArcVu (OpenGrid)" "clap|CLAP: ArcVu (OpenGrid)" "au|AU: ArcVu (OpenGrid)" "lv2|LV2: ArcVu (OpenGrid)"; do
  fmt=${spec%%|*}; name=${spec#*|}
  python3 $R quit --qa-dir "$PWD/$E/reaper-$fmt" > /dev/null 2>&1; sleep 3
  rm -rf "$E/reaper-$fmt"
  python3 $R launch --qa-dir "$PWD/$E/reaper-$fmt" > "$E/logs/launch-$fmt.log" 2>&1
  python3 $R suite --qa-dir "$PWD/$E/reaper-$fmt" --fx "$name" --exceptions "$PWD/$E/exceptions.txt" --seconds 2 > "$E/logs/reaper-suite-$fmt.log" 2>&1
  python3 $R quit --qa-dir "$PWD/$E/reaper-$fmt" > /dev/null 2>&1
  echo "$fmt done $(date +%T)" >> "$E/logs/hosts.progress"
done
echo finished >> "$E/logs/hosts.progress"
