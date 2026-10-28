# Releasing ArcVu

Developer: OpenGrid. Product: ArcVu. Bundle id `com.opengrid.arcvu`, AU/VST3 codes `OpGr` / `ArcV`, LV2 `urn:opengrid:arcvu`,
CLAP `com.opengrid.arcvu`.

## What ships

| Platform | Formats | Package |
|---|---|---|
| macOS 11+ (arm64 + x86_64) | VST3, AU, CLAP, LV2, AAX, Standalone | signed and notarised `.pkg`, `.dmg`, bundles `.zip` |
| Windows 10+ x64 | VST3, CLAP, LV2, Standalone | `.zip` (built by GitHub Actions) |
| Linux x86_64 | VST3, CLAP, LV2, Standalone | `.tar.gz` (built by GitHub Actions) |

## Order of work

1. QA first (`qa/<date>-release-qa/ledger.md`): unit tests, UI audit, REAPER suite per format (hidden REAPER), auval. A release of an
   unverified build is not made.
2. Version: `project(ArcVu VERSION x.y.z)` in `CMakeLists.txt` and `dsp/Cargo.toml`; update `README.md`, `CHANGELOG.md`.
3. Manual: `python3 tools/build_user_manual.py` (needs `reportlab` and `pillow`). Screenshots in `docs/images` come from
   `ArcVu_QA --manual-shots docs/images` plus the menu images of the UI audit.
4. Commit with a clean tree, then build and sign on the Mac.

## macOS (run in your own Terminal)

`codesign`, `wraptool` and `notarytool` need the keychain and the Signing iLok, which an unattended shell cannot answer. Check first:

```bash
~/.agent/skills/Make-Plugin-Release/scripts/verify_signing_env.sh
SIGN=1 installer/build_macos.sh
```

The script builds universal, tests, signs every bundle with the Developer ID Application certificate (hardened runtime, timestamp),
PACE-wraps the AAX (`wraptool sign`, account `mistamin`, product wrap GUID in `ARCVU_AAX_WCGUID`, "Signing Only"), builds and signs the
installer with the Developer ID Installer certificate, notarises and staples the package and the DMG, and writes SHA-256 files to
`dist/release-<version>/macos`. `SIGN=1 NOTARIZE=0` signs without submitting. Without `SIGN=1` it produces unsigned packages for checks.

AAX: built when the Avid AAX SDK is at `../aax-sdk-2-8-1` (not committed). Test the signed AAX with the AAX Validator and in Pro Tools
before the release is published.

## Windows and Linux (GitHub Actions)

`.github/workflows/build.yml` builds both on every push and on tags `v*`, and attaches the archives to the release. The job checks out
JUCE (public `MistaMin/JUCE`, pinned commit) and GoodLookinDri (private `MistaMin/GoodLookinDri`, pinned commit) beside the ArcVu
checkout. Add the repository secret `OPENGRID_CI_TOKEN` (a fine-grained token with read access to GoodLookinDri) before the first run.
The pinned commits are in the workflow's `env`; update them when the dependencies change.

## Publishing

Tag `v<version>`, push the tag, check that the Windows and Linux assets appear, upload the macOS files from `dist`, and compare
every SHA-256 with the file on the release page. State in the notes which hosts and platforms were tested and which were not.
The ArcVu repository is private; a release on it is visible only to its members until the repository is made public.

## Checklist

```text
Version / tag / source commit:
Clean tree: yes / no
Unit tests (Debug and Release), UI audit, auval:
Host suite per format (VST3, AU, CLAP, LV2):
AAX: PACE signed (wraptool verify), AAX Validator, Pro Tools:
Developer ID signatures verified, notarisation Accepted, staple validated, Gatekeeper assessment:
Windows and Linux CI green:
Licence inventory (tools/collect_licenses.py INDEX.txt):
Checksums computed after stapling:
Known limits stated in the notes:
```
