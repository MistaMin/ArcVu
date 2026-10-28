# SPDX-License-Identifier: MIT
# Builds, tests and packages ArcVu for Windows x64: VST3, CLAP, LV2 and the Standalone application in a zip.
#   powershell -ExecutionPolicy Bypass -File installer\build_windows.ps1
# Requires Visual Studio 2022 (C++), CMake 3.22+, Rust (MSVC toolchain) and Python 3. AAX is never built here.
# JUCE is expected at ..\audiogridder\JUCE and GoodLookinDri at ..\instruments\GoodLookinDri.
# Optional: SIGN_CERT_THUMBPRINT signs the binaries with Authenticode.
$ErrorActionPreference = 'Stop'
Set-Location (Split-Path $PSScriptRoot -Parent)
$Version = [regex]::Match((Get-Content CMakeLists.txt -Raw), 'project\(ArcVu VERSION ([0-9.]+)\)').Groups[1].Value
$Build = if ($env:BUILD) { $env:BUILD } else { "build\release-windows" }
$Out = "dist\windows"
$Art = "$Build\ArcVu_artefacts\Release"

$env:CARGO_TARGET_DIR = "$Build-test"
cargo test --release --manifest-path dsp\Cargo.toml --lib
if ($LASTEXITCODE -ne 0) { throw "Rust tests failed" }
cmake -S . -B $Build -G "Visual Studio 17 2022" -A x64 -DARCVU_BUILD_AAX=OFF
cmake --build $Build --config Release --parallel
if ($LASTEXITCODE -ne 0) { throw "build failed" }
python -m pip install --quiet reportlab pillow
if ($LASTEXITCODE -ne 0) { throw "pip install failed" }
python tools\build_user_manual.py

foreach ($required in @("$Art\VST3\ArcVu.vst3", "$Art\CLAP\ArcVu.clap", "$Art\LV2\ArcVu.lv2", "$Art\Standalone\ArcVu.exe")) {
    if (-not (Test-Path $required)) { throw "Missing build artifact: $required" }
}
if (Test-Path "$Art\AAX") { throw "AAX found in the Windows package" }

if ($env:SIGN_CERT_THUMBPRINT) {
    Get-ChildItem -Recurse "$Art\VST3\ArcVu.vst3", "$Art\CLAP\ArcVu.clap", "$Art\LV2\ArcVu.lv2", "$Art\Standalone\ArcVu.exe" -Include *.vst3, *.clap, *.dll, *.exe |
        Where-Object { -not $_.PSIsContainer } | ForEach-Object {
        signtool sign /sha1 $env:SIGN_CERT_THUMBPRINT /fd sha256 /tr http://timestamp.digicert.com /td sha256 $_.FullName
    }
}

$Name = "ArcVu-$Version-windows-x86_64"
if (Test-Path $Out) { Remove-Item -Recurse -Force $Out }
New-Item -ItemType Directory -Force -Path "$Out\$Name\plugins" | Out-Null
Copy-Item -Recurse "$Art\VST3\ArcVu.vst3", "$Art\CLAP\ArcVu.clap", "$Art\LV2\ArcVu.lv2" "$Out\$Name\plugins\"
Copy-Item "$Art\Standalone\ArcVu.exe" "$Out\$Name\"
python tools\collect_licenses.py $Build "$Out\$Name\Licenses\third-party" win
Copy-Item LICENSE, BINARY_LICENSE.txt, THIRD_PARTY_LICENSES.md, CHANGELOG.md "$Out\$Name\"
Copy-Item output\pdf\ArcVu-User-Manual.pdf "$Out\$Name\"
@"
ArcVu $Version for Windows (x64)

Copy plugins\ArcVu.vst3 to C:\Program Files\Common Files\VST3,
plugins\ArcVu.clap to C:\Program Files\Common Files\CLAP and
plugins\ArcVu.lv2 to C:\Program Files\Common Files\LV2.
ArcVu.exe is the standalone application. AAX is not built for Windows.
"@ | Set-Content "$Out\$Name\INSTALL.txt"
Compress-Archive -Path "$Out\$Name" -DestinationPath "$Out\$Name.zip"
"{0}  {1}" -f (Get-FileHash "$Out\$Name.zip" -Algorithm SHA256).Hash.ToLower(), "$Name.zip" | Set-Content "$Out\$Name.zip.sha256"
Get-Content "$Out\$Name.zip.sha256"
