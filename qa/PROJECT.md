# ArcVu QA profile

Baseline private MistaMin/ArcVu main 53e22d4 / v0.1.1-alpha. Normal formats: Standalone, VST3, AU.
AAX is an opt-in later signed build; excluded by HANDOFF.md. One original metal finish. Detector
architecture, 55 existing host IDs, ArcVuSetup layout, manufacturer and bundle IDs stay unchanged.

## Build and repeat

```sh
export PATH="/opt/homebrew/bin:$PATH"
cargo test --manifest-path dsp/Cargo.toml --offline --lib
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DARCVU_BUILD_QA=ON
cmake --build build --config Release --target ArcVu_Standalone ArcVu_VST3 ArcVu_AU ArcVu_QA -j 8
build/ArcVu_QA_artefacts/Release/ArcVu_QA "$PWD/qa/2026-10-26-qa-audit/screenshots"
build/ArcVu_QA_artefacts/Release/ArcVu_QA --detector-audit
cmake -S . -B build/qa-debug -DCMAKE_BUILD_TYPE=Debug -DARCVU_BUILD_QA=ON
cmake --build build/qa-debug --target ArcVu_QA -j 6
build/qa-debug/ArcVu_QA_artefacts/Debug/ArcVu_QA /private/tmp/arcvu-debug-ui
```

`--detector-audit` currently exits 1 with three real failures. Preserve these diagnostics until
corresponding detector fixes have independent evidence. The UI/state mode exits 0 only when its
checks pass. It is not a DAW or native input test.

JUCE: ../audiogridder/JUCE; GoodLookinDri: ../instruments/GoodLookinDri. No ArcVu global skill profile
existed at the beginning of this pass; this repository profile records the observed environment.
Installed hosts include REAPER, Pro Tools and Studio One; Logic is absent. pluginval is absent on PATH.

## Host safety and exceptions

Follow plugin-daw-test hidden-window rules. Existing user DAW sessions are never touched. Stop only
the launched QA instance if it is not hidden. No system install, signing, upload or release.
For auval, temporary user AU installation and AudioComponentRegistrar refresh are needed. This pass
removed only its newly created user AU/VST3 copies after final validation; built bundles remain in build/.

Meter/display/preference controls deliberately do not change audio. Audio or conditional controls
are identified in parameter-inventory.json. A future audio sweep needs explicit context for M/S
trims, dynamic enable/type/depth/listen, mono amount, filter slopes, and meters-only/bypass-hidden.
No broad "dead parameter" exception should hide these conditional cases.
