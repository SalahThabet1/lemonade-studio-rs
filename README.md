# Lemonade Studio (Rust/GPUI)

Fast, tactile, offline-first AI image studio for indie game artists: FLUX generation via a local
Lemonade server, neural/chroma background removal, and Aseprite-ready exports. Native Linux
desktop app built with Rust + GPUI. No cloud accounts, no subscriptions.

## What it does

- **Studio**: prompt → FLUX image via Lemonade OpenAI-compatible API (`POST /v1/images/generations`).
- **Background Remover**: Neural AI (BRIA RMBG ONNX via `scripts/ai_rmbg.py`) with chroma-key
  fallback, live tolerance / 1-bit / halo tuning, cached masks, stale-result guards.
- **Library**: curated prompt recipes + your custom recipes (persisted in settings).
- **Exports**: PNG cutouts + Aseprite/Godot JSON spritesheets with configurable frame sizes.

Honesty rules the app follows:

- Unknown model names **fail loudly** with the available list — never silently substituted.
- Server errors show the URL + status — never masked as a successful demo sprite.
- LoRA / ContentRef / Aesthetic controls are **preview-only** until the server API supports
  them; the app warns on generation when they are enabled.

## Install (recommended)

Download `LemonadeStudio-x86_64.AppImage` from the
[latest GitHub release](https://github.com/SalahThabet1/lemonade-studio-rs/releases):

```bash
chmod +x LemonadeStudio-x86_64.AppImage
./LemonadeStudio-x86_64.AppImage
```

The AppImage bundles the binary, fonts, textures, and RMBG helper. It is built on
Ubuntu 22.04 (glibc 2.35) so it runs on Ubuntu 22.04+, Debian 12+, Fedora 38+, Arch,
and other modern x86_64 distros. You still need a Lemonade server (below) and,
for neural cutouts, `uv` + the RMBG ONNX model on the host.

Prefer a plain tarball? Releases also ship
`lemonade-studio-x86_64.tar.gz` (binary + assets + scripts + README).

## Requirements (building from source)

- Linux (tested on Arch), Rust stable, `uv` (for the RMBG helper), `appimagetool` +
  `rsvg-convert` (packaging only).
- Lemonade server (see below) with an `sd-cpp` image model, e.g. the custom
  `FLUX.2-klein-4B-GGUF-Q4_0` build used during development.
- Optional: BRIA RMBG ONNX model at `~/models/rmbg/rmbg-1.4.onnx` (or `rmbg-2.0.onnx`).

## Quick start (dev)

```bash
cargo test
cargo run --release
```

Settings migrate automatically to `$XDG_CONFIG_HOME/lemonade-studio/settings.json`
(`~/.config/lemonade-studio/settings.json`). Previews/cache live under the system temp
dir (`lemonade-studio/` prefix). Your old `./settings.json` is copied once, never deleted.

## Pairing with the Lemonade AI server

1. Install + start Lemonade (Arch example):

   ```bash
   sudo pacman -S lemonade-server
   sudo systemctl enable --now lemond
   curl http://127.0.0.1:13305/v1/models | head -c 500
   ```

2. Pull an image model. The exact dev-tested custom build:

   - Main: `leejet/FLUX.2-klein-4B-GGUF:flux-2-klein-4b-Q4_0.gguf`
   - Text encoder: `unsloth/Qwen3-4B-GGUF:Qwen3-4B-Q4_K_S.gguf`
   - VAE: `Comfy-Org/flux2-klein-4B:split_files/vae/flux2-vae.safetensors`
   - Registered locally as `FLUX.2-klein-4B-GGUF-Q4_0` (`recipe: sd-cpp`,
     labels include `image`/`edit`).

   If `lemonade pull <model>` offers different quant variants, pick the Q4 build that fits
   your VRAM. Verify it appears as an `image` model:

   ```bash
   curl http://127.0.0.1:13305/v1/models | python3 -c \
     "import json,sys; [print(m['id'], m.get('recipe'), m.get('labels')) for m in json.load(sys.stdin)['data']]"
   ```

3. Point Studio at it: open **Settings → Server**, edit the endpoint (default
   `http://127.0.0.1:13305`, no trailing `/v1`), optionally set the API key, then
   **Refresh**. Pick the model in the picker. Klein 4B profile: 512×512, 4 steps, CFG 1.0.

4. Smoke test before opening Studio:

   ```bash
   curl http://127.0.0.1:13305/v1/images/generations \
     -H 'Content-Type: application/json' \
     -d '{"model":"FLUX.2-klein-4B-GGUF-Q4_0","prompt":"16-bit pixel art knight, solid dark background","size":"512x512","steps":4,"response_format":"b64_json"}'
   ```

### Troubleshooting

| Symptom | Fix |
|---|---|
| `Lemonade server unreachable` | `systemctl --no-pager status lemond`; confirm port `13305`. |
| `Model 'X' is not available` | `lemonade pull` the exact id; Refresh in Settings. |
| Slow first generation | Cold load is normal; watch server logs. Warm runs are faster. |
| OOM / 500 from server | Use the Q4 Klein build; reduce to 512×512; close other GPU apps. |
| RMBG neural fails | Falls back to chroma automatically; install `~/models/rmbg/rmbg-1.4.onnx` + `uv` for neural. |

## Background removal models

Neural mode shells out to `uv run --with onnxruntime --with pillow --with numpy
scripts/ai_rmbg.py` with `--model ~/models/rmbg/rmbg-1.4.onnx` (1.4 and 2.0 both probed).
Chroma mode is pure Rust, offline, instant. Masks are cached next to outputs as
`*.mask.png` so sliders retune without re-running inference.

## Packaging

```bash
cargo build --release
./package_appimage.sh
# → build/LemonadeStudio.AppImage (no host files touched)
```

The script is repo-relative (`PROJECT_DIR`, `APP_DIR`, `OUT_APPIMAGE`, `ICON_SVG_SRC`
all overridable). It refuses to package a debug binary unless `ALLOW_DEBUG=1`.
Install/run steps are intentionally **not** part of packaging.

## Repo hygiene

- `target/`, `build/`, `outputs/`, `settings.json`, `graphify-out/` are git-ignored.
- `DESIGN.md` is canonical (lowercase duplicate removed). `PRODUCT.md` holds product spec.
- Fonts: Climate Crisis + Sora (see their OFL licenses in the font projects).
- CI: `cargo fmt --check`, `clippy -D warnings`, `cargo test`, Python compile check,
  `bash -n` packaging check.

## License

MIT — see `LICENSE-MIT`.
