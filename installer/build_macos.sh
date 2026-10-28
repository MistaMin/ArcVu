#!/bin/bash
# SPDX-License-Identifier: MIT
# Builds, tests and packages ArcVu for macOS (universal arm64 + x86_64): VST3, AU, CLAP, LV2, AAX and the
# Standalone app in one installer package, a DMG and a bundle archive.
#
#   installer/build_macos.sh                       unsigned (local checks)
#   SIGN=1 installer/build_macos.sh                sign, PACE-wrap the AAX, notarize and staple
#   SIGN=1 NOTARIZE=0 installer/build_macos.sh     sign but do not submit to Apple
#
# Run SIGN=1 from your own Terminal: codesign, wraptool and notarytool can need the keychain or iLok prompt,
# which an unattended shell cannot answer. The Signing iLok must be plugged in for the AAX.
# Identities, the notary profile and the PACE wrap GUID can be overridden with the ARCVU_* variables below.
set -euo pipefail
export COPYFILE_DISABLE=1
cd "$(dirname "$0")/.."
ROOT=$PWD
VERSION=$(sed -n 's/^project(ArcVu VERSION \([0-9.]*\)).*/\1/p' CMakeLists.txt)
BUILD=${BUILD:-build/release-macos}
OUT=${OUT:-dist/release-$VERSION/macos}
ART="$ROOT/$BUILD/ArcVu_artefacts/Release"
APP_ID=${ARCVU_APP_ID:-"Developer ID Application: Marcos Deida (ZQYU67JUBH)"}
INSTALLER_ID=${ARCVU_INSTALLER_ID:-"Developer ID Installer: Marcos Deida (ZQYU67JUBH)"}
NOTARY_PROFILE=${ARCVU_NOTARY_PROFILE:-AC_NOTARY}
PACE_ACCOUNT=${ARCVU_PACE_ACCOUNT:-mistamin}
AAX_WCGUID=${ARCVU_AAX_WCGUID:-"58571580-D15A-11F1-BD18-005056928F3B"}   # ArcVu, Signing Only (PACE Central)
WRAPTOOL=${ARCVU_WRAPTOOL:-/Applications/PACEAntiPiracy/Eden/Fusion/Current/bin/wraptool}
PKG_ID=com.opengrid.arcvu
export PATH="/opt/homebrew/bin:$HOME/.cargo/bin:$PATH"

echo "==> ArcVu $VERSION: configure and build (universal)"
cmake -S . -B "$BUILD" -G Ninja -DCMAKE_BUILD_TYPE=Release -DARCVU_UNIVERSAL=ON -DARCVU_BUILD_QA=ON
cmake --build "$BUILD" --config Release

echo "==> Tests"
CARGO_TARGET_DIR="$ROOT/$BUILD/dsp-test" cargo test --release --manifest-path dsp/Cargo.toml --offline --lib
QA="$BUILD/ArcVu_QA_artefacts/Release/ArcVu_QA"
SCRATCH=$(mktemp -d)
"$QA" "$SCRATCH/ui" > "$SCRATCH/ui.log" 2>&1 || { tail -20 "$SCRATCH/ui.log"; echo "UI audit failed" >&2; exit 1; }
tail -1 "$SCRATCH/ui.log"
if arch -x86_64 /usr/bin/true 2>/dev/null; then
  # The x86_64 slice under Rosetta: exercises the code path, not Intel performance.
  arch -x86_64 "$QA" --keys-test | tail -1
fi

for f in VST3/ArcVu.vst3 AU/ArcVu.component CLAP/ArcVu.clap LV2/ArcVu.lv2 Standalone/ArcVu.app; do
  [ -e "$ART/$f" ] || { echo "missing $f" >&2; exit 1; }
done
AAX=0; [ -d "$ART/AAX/ArcVu.aaxplugin" ] && AAX=1

if [ "${SIGN:-0}" = "1" ]; then
  echo "==> Sign bundles (Developer ID, hardened runtime, timestamp)"
  for b in "$ART/VST3/ArcVu.vst3" "$ART/AU/ArcVu.component" "$ART/CLAP/ArcVu.clap"; do
    codesign --force --deep --timestamp --options runtime --sign "$APP_ID" "$b"
    codesign --verify --deep --strict --verbose=2 "$b"
  done
  find "$ART/LV2/ArcVu.lv2" -type f \( -name '*.dylib' -o -name '*.so' \) -print0 | while IFS= read -r -d '' f; do
    codesign --force --timestamp --options runtime --sign "$APP_ID" "$f"
    codesign --verify --strict --verbose=2 "$f"
  done
  codesign --force --deep --timestamp --options runtime --entitlements installer/standalone.entitlements \
    --sign "$APP_ID" "$ART/Standalone/ArcVu.app"
  codesign --verify --deep --strict --verbose=2 "$ART/Standalone/ArcVu.app"
  if [ "$AAX" = 1 ]; then
    echo "==> AAX: Developer ID, then PACE Eden signature"
    codesign --force --deep --timestamp --options runtime --sign "$APP_ID" "$ART/AAX/ArcVu.aaxplugin"
    "$WRAPTOOL" sign --verbose --account "$PACE_ACCOUNT" --wcguid "$AAX_WCGUID" --signid "$APP_ID" \
      --in "$ART/AAX/ArcVu.aaxplugin" --out "$ART/AAX/ArcVu.aaxplugin" --autoinstall on
    "$WRAPTOOL" verify --verbose --in "$ART/AAX/ArcVu.aaxplugin"
  fi
fi

echo "==> Stage the installer payload"
rm -rf "$OUT"; mkdir -p "$OUT"
STAGE="$OUT/stage"; ROOTFS="$STAGE/root"; mkdir -p "$STAGE/scripts" "$ROOTFS"
P="$ROOTFS/Library/Audio/Plug-Ins"
mkdir -p "$P/VST3" "$P/Components" "$P/CLAP" "$P/LV2" "$ROOTFS/Applications"
ditto "$ART/VST3/ArcVu.vst3" "$P/VST3/ArcVu.vst3"
ditto "$ART/AU/ArcVu.component" "$P/Components/ArcVu.component"
ditto "$ART/CLAP/ArcVu.clap" "$P/CLAP/ArcVu.clap"
ditto "$ART/LV2/ArcVu.lv2" "$P/LV2/ArcVu.lv2"
ditto "$ART/Standalone/ArcVu.app" "$ROOTFS/Applications/ArcVu.app"
if [ "$AAX" = 1 ]; then
  mkdir -p "$ROOTFS/Library/Application Support/Avid/Audio/Plug-Ins"
  ditto "$ART/AAX/ArcVu.aaxplugin" "$ROOTFS/Library/Application Support/Avid/Audio/Plug-Ins/ArcVu.aaxplugin"
fi
SUPPORT="$ROOTFS/Library/Application Support/OpenGrid/ArcVu"
mkdir -p "$SUPPORT/Licenses" "$SUPPORT/Manual/images"
python3 tools/collect_licenses.py "$BUILD" "$SUPPORT/Licenses/third-party" mac
cp LICENSE BINARY_LICENSE.txt THIRD_PARTY_LICENSES.md CHANGELOG.md "$SUPPORT/Licenses/"
python3 tools/build_user_manual.py
cp output/pdf/ArcVu-User-Manual.pdf "$SUPPORT/Manual/"
cp docs/USER_MANUAL.md "$SUPPORT/Manual/"; cp docs/images/*.png "$SUPPORT/Manual/images/"
printf '#!/bin/bash\n/usr/bin/killall -9 AudioComponentRegistrar 2>/dev/null || true\nexit 0\n' > "$STAGE/scripts/postinstall"
chmod +x "$STAGE/scripts/postinstall"
cat BINARY_LICENSE.txt THIRD_PARTY_LICENSES.md LICENSE > "$STAGE/license.txt"
AAX_LINE=""; [ "$AAX" = 1 ] && AAX_LINE='<li><b>AAX</b> &rarr; /Library/Application Support/Avid/Audio/Plug-Ins/</li>'
cat > "$STAGE/welcome.html" <<HTML
<html><body style="font-family:-apple-system,Helvetica,Arial,sans-serif;padding:20px;">
<h1>ArcVu $VERSION</h1><p>This installer places:</p><ul>
<li><b>VST3</b> &rarr; /Library/Audio/Plug-Ins/VST3/</li>
<li><b>Audio Unit</b> &rarr; /Library/Audio/Plug-Ins/Components/</li>
<li><b>CLAP</b> &rarr; /Library/Audio/Plug-Ins/CLAP/</li>
<li><b>LV2</b> &rarr; /Library/Audio/Plug-Ins/LV2/</li>
$AAX_LINE
<li><b>Standalone app</b> &rarr; /Applications/</li></ul>
<p>macOS universal (Apple silicon and Intel). The manual and licence texts go to /Library/Application Support/OpenGrid/ArcVu.</p>
</body></html>
HTML
cat > "$STAGE/conclusion.html" <<'HTML'
<html><body style="font-family:-apple-system,Helvetica,Arial,sans-serif;padding:20px;">
<h1>Installation complete</h1><p>ArcVu is installed. Restart your DAW to load it. The user manual is in /Library/Application Support/OpenGrid/ArcVu/Manual.</p>
</body></html>
HTML
# AAX and bundles that a host or DMG may relocate are kept in place by the installer.
cat > "$STAGE/distribution.xml" <<DIST
<?xml version="1.0" encoding="utf-8" standalone="no"?>
<installer-gui-script minSpecVersion="2">
    <title>ArcVu $VERSION</title>
    <welcome file="welcome.html" mime-type="text/html"/>
    <license file="license.txt" mime-type="text/plain"/>
    <conclusion file="conclusion.html" mime-type="text/html"/>
    <options customize="never" require-scripts="false" hostArchitectures="arm64,x86_64"/>
    <domains enable_anywhere="false" enable_currentUserHome="false" enable_localSystem="true"/>
    <choices-outline><line choice="default"><line choice="$PKG_ID.pkg"/></line></choices-outline>
    <choice id="default"/>
    <choice id="$PKG_ID.pkg" visible="false"><pkg-ref id="$PKG_ID.pkg"/></choice>
    <pkg-ref id="$PKG_ID.pkg" version="$VERSION" auth="Root" onConclusion="none">ArcVu.pkg</pkg-ref>
</installer-gui-script>
DIST
xattr -cr "$ROOTFS" 2>/dev/null || true
echo "==> Build the package"
pkgbuild --root "$ROOTFS" --identifier "$PKG_ID.pkg" --version "$VERSION" --install-location / \
  --scripts "$STAGE/scripts" "$STAGE/ArcVu.pkg"
UNSIGNED="$STAGE/ArcVu-unsigned.pkg"
productbuild --distribution "$STAGE/distribution.xml" --resources "$STAGE" --package-path "$STAGE" "$UNSIGNED"
PKG="$OUT/ArcVu-$VERSION-macos.pkg"
if [ "${SIGN:-0}" = "1" ]; then
  productsign --sign "$INSTALLER_ID" "$UNSIGNED" "$PKG"
  pkgutil --check-signature "$PKG"
  if [ "${NOTARIZE:-1}" = "1" ]; then
    xcrun notarytool submit "$PKG" --keychain-profile "$NOTARY_PROFILE" --wait
    xcrun stapler staple "$PKG"
    xcrun stapler validate "$PKG"
    spctl --assess --type install -v "$PKG" || true
  fi
else
  cp "$UNSIGNED" "$PKG"
fi

echo "==> DMG and bundle archive"
DMGSTAGE="$OUT/dmg"; mkdir -p "$DMGSTAGE/Documentation"
cp "$PKG" "$DMGSTAGE/"; cp output/pdf/ArcVu-User-Manual.pdf "$DMGSTAGE/Documentation/"
cp LICENSE BINARY_LICENSE.txt THIRD_PARTY_LICENSES.md CHANGELOG.md "$DMGSTAGE/Documentation/"
printf 'ArcVu %s\n\nDouble-click ArcVu-%s-macos.pkg to install VST3, Audio Unit, CLAP, LV2, AAX and the Standalone app.\n' "$VERSION" "$VERSION" > "$DMGSTAGE/INSTALL.txt"
DMG="$OUT/ArcVu-$VERSION-macos.dmg"
if [ "${SIGN:-0}" = "1" ]; then
  NOTARIZE=${NOTARIZE:-1} "$HOME/.agent/skills/Make-Plugin-Release/scripts/build_notarized_dmg.sh" "ArcVu $VERSION" "$DMGSTAGE" "$DMG" "$APP_ID" "$NOTARY_PROFILE"
else
  hdiutil create -volname "ArcVu $VERSION" -srcfolder "$DMGSTAGE" -ov -format UDZO "$DMG" > /dev/null
fi
ZIP="$OUT/ArcVu-$VERSION-macos-bundles.zip"
BUNDLES="$OUT/bundles"; mkdir -p "$BUNDLES"
for f in VST3/ArcVu.vst3 AU/ArcVu.component CLAP/ArcVu.clap LV2/ArcVu.lv2 Standalone/ArcVu.app; do ditto "$ART/$f" "$BUNDLES/$(basename "$f")"; done
[ "$AAX" = 1 ] && ditto "$ART/AAX/ArcVu.aaxplugin" "$BUNDLES/ArcVu.aaxplugin"
cp -R "$SUPPORT/Licenses" "$BUNDLES/Licenses"; cp output/pdf/ArcVu-User-Manual.pdf "$BUNDLES/"
(cd "$BUNDLES" && ditto -c -k --sequesterRsrc . "$ROOT/$ZIP")
rm -rf "$STAGE" "$DMGSTAGE" "$BUNDLES"
(cd "$OUT" && shasum -a 256 "ArcVu-$VERSION-macos.pkg" "ArcVu-$VERSION-macos.dmg" "ArcVu-$VERSION-macos-bundles.zip" > "ArcVu-$VERSION-macos.sha256")
cat "$OUT/ArcVu-$VERSION-macos.sha256"
echo "done: $OUT"
