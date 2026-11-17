#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="${PROJECT_DIR:-$SCRIPT_DIR}"
APP_DIR="${APP_DIR:-${PROJECT_DIR}/build/LemonadeStudio.AppDir}"
OUT_APPIMAGE="${OUT_APPIMAGE:-${PROJECT_DIR}/build/LemonadeStudio.AppImage}"
ICON_SVG_SRC="${ICON_SVG_SRC:-${PROJECT_DIR}/assets/icons/ai.lemonadestudio.LemonadeStudio.svg}"

echo "=== Packaging Lemonade Studio AppImage ==="
echo "Project: ${PROJECT_DIR}"
echo "Output:  ${OUT_APPIMAGE}"

command -v appimagetool >/dev/null 2>&1 || { echo "Missing appimagetool in PATH" >&2; exit 1; }
command -v rsvg-convert >/dev/null 2>&1 || { echo "Missing rsvg-convert in PATH" >&2; exit 1; }
[ -f "${ICON_SVG_SRC}" ] || { echo "Missing icon: ${ICON_SVG_SRC}" >&2; exit 1; }

BIN_SRC=""
if [ -f "${PROJECT_DIR}/target/release/lemonade-studio-rs" ]; then
    BIN_SRC="${PROJECT_DIR}/target/release/lemonade-studio-rs"
    echo "Using release binary..."
elif [ "${ALLOW_DEBUG:-0}" = "1" ] && [ -f "${PROJECT_DIR}/target/debug/lemonade-studio-rs" ]; then
    BIN_SRC="${PROJECT_DIR}/target/debug/lemonade-studio-rs"
    echo "WARNING: packaging debug binary (ALLOW_DEBUG=1)" >&2
else
    echo "Missing release binary. Run: cargo build --release" >&2
    exit 1
fi

rm -rf "${APP_DIR}"
mkdir -p "${APP_DIR}/usr/bin"
mkdir -p "${APP_DIR}/usr/share/applications"
mkdir -p "${APP_DIR}/usr/share/lemonade-studio/scripts"
mkdir -p "${APP_DIR}/usr/share/icons/hicolor/scalable/apps"
mkdir -p "${APP_DIR}/usr/share/icons/hicolor/256x256/apps"
mkdir -p "${APP_DIR}/usr/share/icons/hicolor/512x512/apps"
mkdir -p "${APP_DIR}/usr/share/fonts/lemonade-studio"
mkdir -p "${APP_DIR}/usr/share/lemonade-studio/textures"

cp -f "${PROJECT_DIR}/assets/fonts/"* "${APP_DIR}/usr/share/fonts/lemonade-studio/"
cp -f "${PROJECT_DIR}/assets/textures/"* "${APP_DIR}/usr/share/lemonade-studio/textures/"
cp -f "${PROJECT_DIR}/scripts/ai_rmbg.py" "${APP_DIR}/usr/share/lemonade-studio/scripts/ai_rmbg.py"
chmod +x "${APP_DIR}/usr/share/lemonade-studio/scripts/ai_rmbg.py"
cp "${BIN_SRC}" "${APP_DIR}/usr/bin/lemonade-studio-rs"
chmod +x "${APP_DIR}/usr/bin/lemonade-studio-rs"

rsvg-convert -w 256 -h 256 "${ICON_SVG_SRC}" -o "${APP_DIR}/ai.lemonadestudio.LemonadeStudio.png"
rsvg-convert -w 256 -h 256 "${ICON_SVG_SRC}" -o "${APP_DIR}/usr/share/icons/hicolor/256x256/apps/ai.lemonadestudio.LemonadeStudio.png"
rsvg-convert -w 512 -h 512 "${ICON_SVG_SRC}" -o "${APP_DIR}/usr/share/icons/hicolor/512x512/apps/ai.lemonadestudio.LemonadeStudio.png"
cp "${ICON_SVG_SRC}" "${APP_DIR}/ai.lemonadestudio.LemonadeStudio.svg"
cp "${ICON_SVG_SRC}" "${APP_DIR}/usr/share/icons/hicolor/scalable/apps/ai.lemonadestudio.LemonadeStudio.svg"
ln -sf "ai.lemonadestudio.LemonadeStudio.png" "${APP_DIR}/.DirIcon"

cat << 'EOF' > "${APP_DIR}/ai.lemonadestudio.LemonadeStudio.desktop"
[Desktop Entry]
Name=Lemonade Studio
GenericName=AI Image Studio & Pixel Art Editor
Comment=GPU-Accelerated AI Diffusion Studio and Pixel-Art RMBG Tool (Rust/GPUI)
Exec=lemonade-studio-rs %U
Icon=ai.lemonadestudio.LemonadeStudio
Terminal=false
Type=Application
Categories=Graphics;2DGraphics;RasterGraphics;
StartupWMClass=lemonade_studio_rs
Keywords=AI;Image;Generation;Diffusion;Flux;Spritesheet;Lemonade;PixelArt;RMBG;
EOF

cp "${APP_DIR}/ai.lemonadestudio.LemonadeStudio.desktop" "${APP_DIR}/usr/share/applications/ai.lemonadestudio.LemonadeStudio.desktop"
ln -sf "ai.lemonadestudio.LemonadeStudio.desktop" "${APP_DIR}/lemonade-studio.desktop"

cat << 'EOF' > "${APP_DIR}/AppRun"
#!/bin/bash
HERE="$(dirname "$(readlink -f "${0}")")"
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
export XDG_DATA_DIRS="${HERE}/usr/share:${XDG_DATA_DIRS}"

exec "${HERE}/usr/bin/lemonade-studio-rs" "$@"
EOF
chmod +x "${APP_DIR}/AppRun"

echo "Running appimagetool..."
ARCH=x86_64 appimagetool --appimage-extract-and-run "${APP_DIR}" "${OUT_APPIMAGE}"
chmod +x "${OUT_APPIMAGE}"

echo "=== Successfully built: ${OUT_APPIMAGE} ==="
echo "Install is a separate step; this script no longer modifies ~/.local, desktop files, or icon caches."
