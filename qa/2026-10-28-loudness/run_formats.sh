#!/bin/bash
# Installs the freshly built formats into the user plug-in folders (the originals are backed up by the caller),
# then runs the REAPER host suite once per format in a fresh hidden REAPER and auval for AU.
cd "$(dirname "$0")/../.."
Q=qa/2026-10-26-release-qa; L=qa/2026-10-28-loudness; B=build/ArcVu_artefacts/Release; P=$HOME/Library/Audio/Plug-Ins
rm -rf $P/VST3/ArcVu.vst3 $P/CLAP/ArcVu.clap $P/Components/ArcVu.component $P/LV2/ArcVu.lv2
cp -R $B/VST3/ArcVu.vst3 $P/VST3/; cp -R $B/CLAP/ArcVu.clap $P/CLAP/; cp -R $B/AU/ArcVu.component $P/Components/; cp -R $B/LV2/ArcVu.lv2 $P/LV2/
killall -9 AudioComponentRegistrar 2>/dev/null; sleep 2
for spec in "vst3|VST3: ArcVu (OpenGrid)" "clap|CLAP: ArcVu (OpenGrid)" "au|AU: ArcVu (OpenGrid)" "lv2|LV2: ArcVu (OpenGrid)"; do
  fmt=${spec%%|*}; name=${spec#*|}
  python3 $Q/hidden_reaper.py quit --qa-dir /private/tmp/arcvu-q/reaper > /dev/null 2>&1; sleep 4
  rm -rf /private/tmp/arcvu-q/reaper
  python3 $Q/hidden_reaper.py launch --qa-dir /private/tmp/arcvu-q/reaper > $L/logs/launch-$fmt-f.log 2>&1
  python3 $Q/hidden_reaper.py suite --qa-dir /private/tmp/arcvu-q/reaper --fx "$name" --exceptions "$PWD/$Q/exceptions.txt" --seconds 2 > $L/logs/reaper-suite-$fmt-f.log 2>&1
  mkdir -p $L/host-$fmt; cp /private/tmp/arcvu-q/reaper/report.md /private/tmp/arcvu-q/reaper/report.json $L/host-$fmt/ 2>/dev/null
done
python3 $Q/hidden_reaper.py quit --qa-dir /private/tmp/arcvu-q/reaper > /dev/null 2>&1
auval -v aufx ArcV OpGr > $L/logs/auval.log 2>&1
echo finished > /private/tmp/arcvu-q/formats.done
