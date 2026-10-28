#!/bin/bash
# SPDX-License-Identifier: MIT
# Builds, tests and packages ArcVu for Linux (x86_64 or aarch64): VST3, CLAP, LV2 and Standalone.
#   installer/build_linux.sh
# Build dependencies (Debian/Ubuntu): ninja-build, ALSA, X11 and related -dev packages, Freetype, fontconfig, GL, Rust.
# JUCE is expected at ../audiogridder/JUCE and GoodLookinDri at ../instruments/GoodLookinDri (see README).
set -euo pipefail
cd "$(dirname "$0")/.."
VERSION=$(sed -n 's/^project(ArcVu VERSION \([0-9.]*\)).*/\1/p' CMakeLists.txt)
ARCH=$(uname -m)
BUILD=${BUILD:-build/release-linux}
OUT=${OUT:-dist/linux}
ART="$BUILD/ArcVu_artefacts/Release"
export PATH="$HOME/.cargo/bin:$PATH"

CARGO_TARGET_DIR="$BUILD-test" cargo test --release --manifest-path dsp/Cargo.toml --lib
cmake -S . -B "$BUILD" -G Ninja -DCMAKE_BUILD_TYPE=Release -DARCVU_BUILD_AAX=OFF
cmake --build "$BUILD" --config Release
python3 -c 'import reportlab, PIL' 2>/dev/null || python3 -m pip install --quiet reportlab pillow
python3 tools/build_user_manual.py

NAME="ArcVu-$VERSION-linux-$ARCH"
rm -rf "$OUT"; mkdir -p "$OUT/$NAME/plugins"
cp -R "$ART/VST3/ArcVu.vst3" "$ART/CLAP/ArcVu.clap" "$ART/LV2/ArcVu.lv2" "$OUT/$NAME/plugins/"
cp "$ART/Standalone/ArcVu" "$OUT/$NAME/ArcVu"
python3 tools/collect_licenses.py "$BUILD" "$OUT/$NAME/Licenses/third-party" linux
cp LICENSE BINARY_LICENSE.txt THIRD_PARTY_LICENSES.md CHANGELOG.md "$OUT/$NAME/"
cp output/pdf/ArcVu-User-Manual.pdf "$OUT/$NAME/"
cat > "$OUT/$NAME/INSTALL.txt" <<TXT
ArcVu $VERSION for Linux ($ARCH)

Per-user install:
  mkdir -p ~/.vst3 ~/.clap ~/.lv2
  cp -R plugins/ArcVu.vst3 ~/.vst3/
  cp -R plugins/ArcVu.clap ~/.clap/
  cp -R plugins/ArcVu.lv2 ~/.lv2/
The standalone application is ./ArcVu. AAX is not built for Linux.
TXT
if find "$OUT/$NAME" -iname '*.aaxplugin' | grep -q .; then echo "AAX found in package" >&2; exit 1; fi
(cd "$OUT" && tar -czf "$NAME.tar.gz" "$NAME" && sha256sum "$NAME.tar.gz" > "$NAME.tar.gz.sha256" && cat "$NAME.tar.gz.sha256")
