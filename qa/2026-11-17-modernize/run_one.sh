#!/bin/bash
# usage: run_one.sh <fmt> <fx name> <tag>
cd "$(dirname "$0")/../.."
E=qa/2026-11-17-modernize; R=~/.claude/skills/plugin-daw-test/scripts/reaper_qa.py
fmt=$1; name=$2; tag=$3; D="$PWD/$E/reaper-$fmt-$tag"
rm -rf "$D"
python3 $R launch --qa-dir "$D" > "$E/logs/launch-$fmt-$tag.log" 2>&1
python3 $R suite --qa-dir "$D" --fx "$name" --exceptions "$PWD/$E/exceptions.txt" --seconds 2 > "$E/logs/reaper-suite-$fmt-$tag.log" 2>&1
python3 $R quit --qa-dir "$D" > /dev/null 2>&1
echo "$fmt-$tag done $(date +%T)" >> "$E/logs/hosts.progress"
