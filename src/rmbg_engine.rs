use anyhow::{Context, Result};
use image::GenericImageView;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

use crate::paths as app_paths;
use crate::state::RmbgEngineMode;

#[allow(dead_code)]
pub struct BackgroundRemovalResult {
    pub output_path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub removed_pixels: usize,
}

fn find_ai_rmbg_script() -> Option<PathBuf> {
    app_paths::resolve_ai_script()
}

fn find_rmbg_onnx_model() -> Option<PathBuf> {
    for c in app_paths::rmbg_model_candidates() {
        if c.exists() {
            return Some(c);
        }
    }
    None
}

/// Runs the Neural AI RMBG model (BRIA RMBG-1.4) via onnxruntime.
/// Semantically segments people, characters, animals, and complex objects.
pub fn process_background_removal_ai(
    input_path: &Path,
    output_path: &Path,
    tolerance: f32,
    pixel_mode: bool,
    halo_trim: u32,
) -> Result<BackgroundRemovalResult> {
    let script = find_ai_rmbg_script().context("ai_rmbg.py helper script not found")?;
    let model = find_rmbg_onnx_model()
        .context("RMBG ONNX model (rmbg-1.4.onnx) not found in ~/models/rmbg/")?;
    let mask_path = output_path.with_extension("mask.png");

    let mut cmd = std::process::Command::new("uv");
    cmd.args([
        "run",
        "--with",
        "onnxruntime",
        "--with",
        "pillow",
        "--with",
        "numpy",
        script.to_str().unwrap(),
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        "--model",
        model.to_str().unwrap(),
        "--tolerance",
        &format!("{:.2}", tolerance),
        "--mask-out",
        mask_path.to_str().unwrap(),
    ]);
    if pixel_mode {
        cmd.arg("--pixel-mode");
    }
    if halo_trim > 0 {
        cmd.args(["--halo-trim", &halo_trim.to_string()]);
    }

    let output = cmd
        .output()
        .context("Failed to spawn uv process for AI RMBG")?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Neural AI RMBG execution failed: {}", err);
    }

    let img = image::open(output_path).context("Failed to open AI cutout output")?;
    let (width, height) = img.dimensions();

    Ok(BackgroundRemovalResult {
        output_path: output_path.to_path_buf(),
        width,
        height,
        removed_pixels: 0,
    })
}

/// Removes background from an image file using color distance from corners.
/// Designed for retro pixel art game sprites on solid-color backdrops.
pub fn process_background_removal_chroma(
    input_path: &Path,
    output_path: &Path,
    tolerance: f32,
    pixel_mode: bool,
    halo_trim: u32,
) -> Result<BackgroundRemovalResult> {
    let img = image::open(input_path).context("Failed to open input image")?;
    let (width, height) = img.dimensions();
    let mut rgba_img = img.to_rgba8();

    // 1. Sample corners to determine the dominant background color
    let corners = [
        rgba_img.get_pixel(0, 0),
        rgba_img.get_pixel(width.saturating_sub(1), 0),
        rgba_img.get_pixel(0, height.saturating_sub(1)),
        rgba_img.get_pixel(width.saturating_sub(1), height.saturating_sub(1)),
    ];

    let avg_bg_r = corners.iter().map(|p| p[0] as f32).sum::<f32>() / 4.0;
    let avg_bg_g = corners.iter().map(|p| p[1] as f32).sum::<f32>() / 4.0;
    let avg_bg_b = corners.iter().map(|p| p[2] as f32).sum::<f32>() / 4.0;

    let max_dist = (255.0_f32.powi(2) * 3.0).sqrt();
    let thresh_dist = (tolerance * max_dist).max(5.0);

    let mut removed_count = 0;
    let w = width as usize;
    let h = height as usize;
    let mut dist_map = vec![0f32; w * h];
    let mut mask = vec![false; w * h];

    for y in 0..height {
        for x in 0..width {
            let p = rgba_img.get_pixel(x, y);
            let dr = p[0] as f32 - avg_bg_r;
            let dg = p[1] as f32 - avg_bg_g;
            let db = p[2] as f32 - avg_bg_b;
            let dist = (dr * dr + dg * dg + db * db).sqrt();
            dist_map[y as usize * w + x as usize] = dist;

            if dist < thresh_dist {
                mask[y as usize * w + x as usize] = true;
                removed_count += 1;
            }
        }
    }

    // 2. Halo trim / erosion: separable max-filter dilation of the mask.
    if halo_trim > 0 {
        let r = halo_trim as i32;
        let width_i = width as i32;
        let height_i = height as i32;
        let mut tmp = vec![false; w * h];
        for y in 0..height_i {
            for x in 0..width_i {
                let start_x = (x - r).max(0) as usize;
                let end_x = (x + r).min(width_i - 1) as usize;
                let row = y as usize * w;
                let mut hit = false;
                for nx in start_x..=end_x {
                    if mask[row + nx] {
                        hit = true;
                        break;
                    }
                }
                tmp[row + x as usize] = hit;
            }
        }
        let mut dilated = vec![false; w * h];
        for y in 0..height_i {
            let start_y = (y - r).max(0) as usize;
            let end_y = (y + r).min(height_i - 1) as usize;
            for x in 0..width_i {
                let mut hit = false;
                for ny in start_y..=end_y {
                    if tmp[ny * w + x as usize] {
                        hit = true;
                        break;
                    }
                }
                dilated[y as usize * w + x as usize] = hit;
            }
        }
        mask = dilated;
    }

    // 3. Apply alpha mask
    for y in 0..height {
        for x in 0..width {
            let p = rgba_img.get_pixel_mut(x, y);
            if mask[y as usize * w + x as usize] {
                p[3] = 0; // Transparent
            } else if pixel_mode {
                p[3] = 255; // 1-bit crisp binary alpha for retro pixel art
            }
        }
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    rgba_img
        .save(output_path)
        .context("Failed to save background-removed PNG")?;

    // Also cache raw distance mask for instant live slider tuning (reuses pass-1 distances).
    let mask_path = output_path.with_extension("mask.png");
    let mut raw_mask_img = image::GrayImage::new(width, height);
    for (i, d) in dist_map.iter().enumerate() {
        let norm = ((d / max_dist) * 255.0).clamp(0.0, 255.0) as u8;
        raw_mask_img.put_pixel((i % w) as u32, (i / w) as u32, image::Luma([norm]));
    }
    raw_mask_img.save(&mask_path).ok();

    Ok(BackgroundRemovalResult {
        output_path: output_path.to_path_buf(),
        width,
        height,
        removed_pixels: removed_count,
    })
}

/// Interactively applies tolerance, 1-bit mode, and halo trim to a cached raw mask (~1-2ms).
/// Updates the cutout output LIVE without re-running neural network inference.
pub fn apply_live_mask_tuning(
    input_path: &Path,
    mask_path: &Path,
    output_path: &Path,
    tolerance: f32,
    pixel_mode: bool,
    halo_trim: u32,
) -> Result<BackgroundRemovalResult> {
    let img = image::open(input_path).context("Failed to open input image")?;
    let mask_raw = image::open(mask_path)
        .context("Failed to open cached mask")?
        .to_luma8();
    let (width, height) = img.dimensions();
    let mask_img = if mask_raw.dimensions() != (width, height) {
        image::imageops::resize(
            &mask_raw,
            width,
            height,
            image::imageops::FilterType::Triangle,
        )
    } else {
        mask_raw
    };
    let mut rgba_img = img.to_rgba8();

    let thresh = (tolerance * 255.0).clamp(0.0, 255.0) as u8;

    let mut alpha_plane: Vec<u8> = Vec::with_capacity((width * height) as usize);
    if pixel_mode {
        for p in mask_img.pixels() {
            alpha_plane.push(if p[0] >= thresh { 255 } else { 0 });
        }
    } else {
        let t = tolerance.clamp(0.01, 0.99);
        let low = (t - 0.15).max(0.0);
        let high = (t + 0.15).min(1.0);
        for p in mask_img.pixels() {
            let val = p[0] as f32 / 255.0;
            let a = if val >= high {
                1.0
            } else if val <= low {
                0.0
            } else {
                (val - low) / (high - low)
            };
            alpha_plane.push((a * 255.0) as u8);
        }
    }

    if halo_trim > 0 {
        let radius = halo_trim as i32;
        let mut temp = vec![0u8; (width * height) as usize];
        // 1. Separable Horizontal min-filter pass
        for y in 0..height as i32 {
            let row_offset = (y as u32 * width) as usize;
            for x in 0..width as i32 {
                let mut min_val = 255u8;
                let start_x = (x - radius).max(0);
                let end_x = (x + radius).min(width as i32 - 1);
                for nx in start_x..=end_x {
                    min_val = min_val.min(alpha_plane[row_offset + nx as usize]);
                }
                temp[row_offset + x as usize] = min_val;
            }
        }
        // 2. Separable Vertical min-filter pass
        let mut eroded = vec![0u8; (width * height) as usize];
        for y in 0..height as i32 {
            let start_y = (y - radius).max(0);
            let end_y = (y + radius).min(height as i32 - 1);
            let out_row_offset = (y as u32 * width) as usize;
            for x in 0..width as i32 {
                let mut min_val = 255u8;
                for ny in start_y..=end_y {
                    let in_idx = (ny as u32 * width + x as u32) as usize;
                    min_val = min_val.min(temp[in_idx]);
                }
                eroded[out_row_offset + x as usize] = min_val;
            }
        }
        alpha_plane = eroded;
    }

    for (i, p) in rgba_img.pixels_mut().enumerate() {
        p[3] = alpha_plane[i];
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    rgba_img
        .save(output_path)
        .context("Failed to save live tuned cutout PNG")?;

    Ok(BackgroundRemovalResult {
        output_path: output_path.to_path_buf(),
        width,
        height,
        removed_pixels: 0,
    })
}

/// Unified entry point for background removal: routes to Neural AI or Chroma Key based on mode.
pub fn process_background_removal(
    input_path: &Path,
    output_path: &Path,
    mode: RmbgEngineMode,
    tolerance: f32,
    pixel_mode: bool,
    halo_trim: u32,
) -> Result<BackgroundRemovalResult> {
    match mode {
        RmbgEngineMode::NeuralAi => {
            match process_background_removal_ai(
                input_path,
                output_path,
                tolerance,
                pixel_mode,
                halo_trim,
            ) {
                Ok(res) => Ok(res),
                Err(e) => {
                    eprintln!(
                        "Neural AI RMBG failed ({}), falling back to Chroma Key...",
                        e
                    );
                    process_background_removal_chroma(
                        input_path,
                        output_path,
                        tolerance,
                        pixel_mode,
                        halo_trim,
                    )
                }
            }
        }
        RmbgEngineMode::ChromaKey => process_background_removal_chroma(
            input_path,
            output_path,
            tolerance,
            pixel_mode,
            halo_trim,
        ),
    }
}

/// Exports an Aseprite / Godot compatible JSON metadata manifest alongside the PNG spritesheet.
pub fn export_aseprite_spritesheet(
    png_path: &Path,
    json_path: &Path,
    frame_width: u32,
    frame_height: u32,
) -> Result<()> {
    let img = image::open(png_path).context("Failed to read image for Aseprite export")?;
    let (width, height) = img.dimensions();

    let fw = if frame_width == 0 { width } else { frame_width };
    let fh = if frame_height == 0 {
        height
    } else {
        frame_height
    };
    anyhow::ensure!(fw > 0 && fh > 0, "Frame dimensions must be non-zero");
    anyhow::ensure!(
        fw <= width && fh <= height,
        "Frame {}x{} exceeds image {}x{}",
        fw,
        fh,
        width,
        height
    );

    let cols = (width / fw).max(1);
    let rows = (height / fh).max(1);
    let total_frames = cols * rows;

    let mut frames_json = serde_json::Map::new();

    for i in 0..total_frames {
        let col = i % cols;
        let row = i / cols;
        let x = col * fw;
        let y = row * fh;

        let frame_key = format!("frame_{:03}.aseprite", i);
        frames_json.insert(
            frame_key,
            json!({
                "frame": { "x": x, "y": y, "w": fw, "h": fh },
                "rotated": false,
                "trimmed": false,
                "spriteSourceSize": { "x": 0, "y": 0, "w": fw, "h": fh },
                "sourceSize": { "w": fw, "h": fh },
                "duration": 100
            }),
        );
    }

    let manifest = json!({
        "frames": frames_json,
        "meta": {
            "app": "Lemonade Studio Rust (GPUI)",
            "version": "1.0.0",
            "image": png_path.file_name().unwrap_or_default().to_string_lossy(),
            "format": "RGBA8888",
            "size": { "w": width, "h": height },
            "scale": "1",
            "frameTags": [
                { "name": "all_frames", "from": 0, "to": total_frames - 1, "direction": "forward" }
            ],
            "layers": [
                { "name": "Spritesheet Layer", "opacity": 255, "blendMode": "normal" }
            ]
        }
    });

    if let Some(parent) = json_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(json_path, serde_json::to_string_pretty(&manifest)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    fn solid_sprite(path: &Path) {
        let mut img = RgbaImage::new(16, 16);
        for p in img.pixels_mut() {
            *p = Rgba([10, 10, 10, 255]);
        }
        // Red foreground square in the middle.
        for y in 6..10 {
            for x in 6..10 {
                img.put_pixel(x, y, Rgba([200, 30, 30, 255]));
            }
        }
        img.save(path).unwrap();
    }

    #[test]
    fn chroma_removes_background_and_caches_mask() {
        let dir = std::env::temp_dir().join("lemonade-rmbg-test");
        std::fs::create_dir_all(&dir).unwrap();
        let input = dir.join("in.png");
        let output = dir.join("out.png");
        solid_sprite(&input);

        let res = process_background_removal_chroma(&input, &output, 0.15, true, 0).unwrap();
        assert_eq!((res.width, res.height), (16, 16));
        assert!(res.removed_pixels > 100);
        assert!(output.exists());
        assert!(output.with_extension("mask.png").exists());

        let out = image::open(&output).unwrap().to_rgba8();
        assert_eq!(out.get_pixel(0, 0)[3], 0);
        assert_eq!(out.get_pixel(7, 7)[3], 255);
    }

    #[test]
    fn halo_trim_expands_transparency() {
        let dir = std::env::temp_dir().join("lemonade-rmbg-test");
        std::fs::create_dir_all(&dir).unwrap();
        let input = dir.join("in.png");
        let plain = dir.join("halo0.png");
        let halo = dir.join("halo2.png");
        solid_sprite(&input);

        process_background_removal_chroma(&input, &plain, 0.15, true, 0).unwrap();
        process_background_removal_chroma(&input, &halo, 0.15, true, 2).unwrap();
        let a = image::open(&plain).unwrap().to_rgba8();
        let b = image::open(&halo).unwrap().to_rgba8();
        let count = |img: &RgbaImage| img.pixels().filter(|p| p[3] == 0).count();
        assert!(count(&b) >= count(&a));
    }

    #[test]
    fn aseprite_export_rejects_oversize_frames() {
        let dir = std::env::temp_dir().join("lemonade-rmbg-test");
        std::fs::create_dir_all(&dir).unwrap();
        let png = dir.join("sheet.png");
        RgbaImage::new(64, 64).save(&png).unwrap();
        assert!(export_aseprite_spritesheet(&png, &dir.join("ok.json"), 32, 32).is_ok());
        assert!(export_aseprite_spritesheet(&png, &dir.join("bad.json"), 128, 32).is_err());
    }
}
