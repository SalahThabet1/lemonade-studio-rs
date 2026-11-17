use anyhow::{Context, Result};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct GenerationRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    size: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
    steps: u32,
    cfg_scale: f32,
    response_format: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
}

#[derive(Deserialize)]
struct GenerationResponse {
    data: Vec<GenerationData>,
}

#[derive(Deserialize)]
struct GenerationData {
    b64_json: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ModelsResponse {
    data: Vec<ModelItem>,
}

#[derive(Deserialize)]
pub(crate) struct ModelItem {
    id: String,
    #[serde(default)]
    labels: Option<Vec<String>>,
    #[serde(default)]
    recipe: Option<String>,
}

pub static TOKIO_RT: std::sync::LazyLock<tokio::runtime::Runtime> =
    std::sync::LazyLock::new(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to initialize Tokio runtime")
    });

pub struct LemonadeClient {
    client: reqwest::Client,
    pub base_url: String,
}

fn build_http_client(timeout_secs: u64, api_key: Option<&str>) -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(key) = api_key {
        if !key.trim().is_empty() {
            if let Ok(v) = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", key.trim()))
            {
                headers.insert(reqwest::header::AUTHORIZATION, v);
            }
        }
    }
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs.clamp(5, 600)))
        .default_headers(headers)
        .build()
        .unwrap_or_default()
}

impl LemonadeClient {
    pub fn new(base_url: String) -> Self {
        Self::with_options(base_url, 180, None)
    }

    pub fn with_options(base_url: String, timeout_secs: u64, api_key: Option<&str>) -> Self {
        let _guard = TOKIO_RT.enter();
        Self {
            client: build_http_client(timeout_secs, api_key),
            base_url: crate::state::normalize_server_url(&base_url),
        }
    }

    /// Rebuild the inner HTTP client after server settings change.
    #[allow(dead_code)]
    pub fn reconfigure(&mut self, base_url: &str, timeout_secs: u64, api_key: Option<&str>) {
        self.base_url = crate::state::normalize_server_url(base_url);
        self.client = build_http_client(timeout_secs, api_key);
    }

    #[allow(dead_code)]
    pub async fn check_health(&self) -> bool {
        let url = format!("{}/v1/health", self.base_url);
        match self.client.get(&url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => {
                let models_url = format!("{}/v1/models", self.base_url);
                match self.client.get(&models_url).send().await {
                    Ok(resp) => resp.status().is_success(),
                    Err(_) => false,
                }
            }
        }
    }

    pub async fn list_models(&self) -> Vec<String> {
        // Discovery failure is explicit: empty list means offline/unknown.
        // Callers decide whether to keep the previous list or show offline UI.
        match self.fetch_models().await {
            Ok(models) => {
                let kept: Vec<String> = models
                    .iter()
                    .filter(|m| is_image_model(m))
                    .map(|m| m.id.clone())
                    .collect();
                kept
            }
            Err(_) => Vec::new(),
        }
    }

    async fn fetch_models(&self) -> Result<Vec<ModelItem>> {
        let url = format!("{}/v1/models", self.base_url);
        let resp = self.client.get(&url).send().await?;
        Ok(resp.json::<ModelsResponse>().await?.data)
    }

    pub async fn generate_image(
        &self,
        model: &str,
        prompt: &str,
        width: u32,
        height: u32,
        steps: u32,
        cfg_scale: f32,
        seed: Option<i64>,
        output_dir: &Path,
    ) -> Result<PathBuf> {
        let actual_model =
            resolve_model_name(&self.list_models().await, model).map_err(|available| {
                anyhow::anyhow!(
                    "Model '{}' is not available on the Lemonade server at {}. Available: {}",
                    model,
                    self.base_url,
                    if available.is_empty() {
                        "(none — server offline or no image models)".to_string()
                    } else {
                        available.join(", ")
                    }
                )
            })?;
        let size_str = format!("{}x{}", width, height);
        let req_body = GenerationRequest {
            model: &actual_model,
            prompt,
            size: &size_str,
            width: Some(width),
            height: Some(height),
            steps,
            cfg_scale,
            response_format: "b64_json",
            seed,
        };

        let url = format!("{}/v1/images/generations", self.base_url);
        let res = self.client.post(&url).json(&req_body).send().await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let parsed: GenerationResponse =
                    resp.json().await.context("Failed to parse JSON response")?;
                if let Some(first) = parsed.data.first() {
                    let image_bytes = BASE64
                        .decode(&first.b64_json)
                        .context("Invalid base64 in response")?;
                    let filename = unique_png_name("gen", seed);
                    let out_path = output_dir.join(filename);
                    fs::create_dir_all(output_dir).ok();
                    fs::write(&out_path, image_bytes)?;
                    return Ok(out_path);
                }
                anyhow::bail!("No image data returned from server");
            }
            Ok(resp) => {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                let snippet: String = text.chars().take(500).collect();
                anyhow::bail!("Server returned error {}: {}", status, snippet);
            }
            Err(e) => {
                anyhow::bail!(
                    "Lemonade server unreachable at {}: {}. Start Lemonade and pull an image model first.",
                    self.base_url,
                    e
                )
            }
        }
    }
}

/// Collision-safe output name without millisecond collisions.
pub fn unique_png_name(prefix: &str, seed: Option<i64>) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!(
        "{}_{}_{}_{}.png",
        prefix,
        chrono::Local::now().format("%Y%m%d_%H%M%S"),
        seed.unwrap_or(0),
        n % 100000
    )
}

/// Pure helpers below are unit-tested without network access.
pub fn is_image_model(m: &ModelItem) -> bool {
    m.recipe.as_deref() == Some("sd-cpp")
        || m.labels
            .as_ref()
            .map_or(false, |l| l.iter().any(|s| s == "image"))
        || m.id.to_lowercase().contains("flux")
        || m.id.to_lowercase().contains("diffusion")
}

#[allow(dead_code)]
pub fn filter_image_models(models: &[ModelItem]) -> Vec<String> {
    let kept: Vec<String> = models
        .iter()
        .filter(|m| is_image_model(m))
        .map(|m| m.id.clone())
        .collect();
    if kept.is_empty() {
        default_image_models()
    } else {
        kept
    }
}

#[allow(dead_code)]
pub fn default_image_models() -> Vec<String> {
    vec![
        "FLUX.2-klein-4B-GGUF-Q4_0".into(),
        "FLUX.2-klein-base-4b".into(),
    ]
}

pub fn normalize_model_name(name: &str) -> String {
    name.to_lowercase().replace(['-', '_', '.'], "")
}

pub fn resolve_model_name(models: &[String], requested: &str) -> Result<String, Vec<String>> {
    if models.iter().any(|m| m == requested) {
        return Ok(requested.to_string());
    }
    let req_clean = normalize_model_name(requested);
    for m in models {
        let m_clean = normalize_model_name(m);
        if m_clean.contains(&req_clean) || req_clean.contains(&m_clean) {
            return Ok(m.clone());
        }
    }
    Err(models.to_vec())
}
/// Creates a crisp procedural retro pixel-art character sprite with a solid background
/// for standalone testing and RMBG verification when server is offline.
#[allow(dead_code)]
pub fn create_sample_pixel_art(width: u32, height: u32, out_path: &Path) -> Result<()> {
    use image::{Rgb, RgbImage};

    let mut img = RgbImage::new(width, height);
    let bg_color = Rgb([16, 18, 27]); // Dark slate solid background
    let armor_color = Rgb([190, 210, 240]);
    let gold_trim = Rgb([240, 190, 40]);
    let visor_glow = Rgb([60, 220, 255]);
    let shadow_color = Rgb([70, 80, 110]);
    let cape_color = Rgb([200, 40, 60]);

    // Fill background
    for pixel in img.pixels_mut() {
        *pixel = bg_color;
    }

    // Draw procedural 16-bit retro knight in center
    let center_x = (width / 2) as i32;
    let center_y = (height / 2) as i32;
    let scale = (width.min(height) / 32) as i32;

    for dy in -12..12 {
        for dx in -8..8 {
            let px = center_x + dx * scale;
            let py = center_y + dy * scale;
            if px < 0 || py < 0 || px >= width as i32 || py >= height as i32 {
                continue;
            }

            let color = match (dx, dy) {
                // Visor glow
                (-2..=2, -6) => visor_glow,
                // Helmet
                (-4..=4, -8..=-4) => armor_color,
                // Cape behind
                (-7..=-5, -3..=7) | (5..=7, -3..=7) => cape_color,
                // Chestplate & Gold Trim
                (-1..=1, -2..=1) => gold_trim,
                (-4..=4, -3..=3) => armor_color,
                // Belt
                (-4..=4, 4) => gold_trim,
                // Legs & Boots
                (-4..=-1, 5..=10) | (1..=4, 5..=10) => shadow_color,
                // Shield / Sword
                (-8..=-5, -2..=5) => armor_color,
                (6..=7, -7..=8) => gold_trim,
                _ => continue,
            };

            // Paint scaled pixel block
            for by in 0..scale {
                for bx in 0..scale {
                    let rx = (px + bx) as u32;
                    let ry = (py + by) as u32;
                    if rx < width && ry < height {
                        img.put_pixel(rx, ry, color);
                    }
                }
            }
        }
    }

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }
    img.save(out_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &str, recipe: Option<&str>, labels: Option<Vec<&str>>) -> ModelItem {
        ModelItem {
            id: id.into(),
            labels: labels.map(|v| v.into_iter().map(|s| s.to_string()).collect()),
            recipe: recipe.map(|s| s.to_string()),
        }
    }

    #[test]
    fn filters_image_models_without_network() {
        let models = vec![
            item("Qwen3-0.6B-GGUF", Some("llamacpp"), Some(vec!["chat"])),
            item("Flux-2-Klein-4B", Some("sd-cpp"), Some(vec!["image"])),
            item("SD-Turbo", None, Some(vec!["image"])),
        ];
        assert_eq!(
            filter_image_models(&models),
            vec!["Flux-2-Klein-4B".to_string(), "SD-Turbo".to_string()]
        );
    }

    #[test]
    fn falls_back_when_no_image_model_present() {
        let models = vec![item(
            "Qwen3-0.6B-GGUF",
            Some("llamacpp"),
            Some(vec!["chat"]),
        )];
        assert_eq!(filter_image_models(&models), default_image_models());
    }

    #[test]
    fn resolves_close_model_names_deterministically() {
        let models = vec![
            "FLUX.2-klein-4B-GGUF-Q4_0".to_string(),
            "SD-Turbo".to_string(),
        ];
        assert_eq!(
            resolve_model_name(&models, "flux-2-klein-4b"),
            Ok("FLUX.2-klein-4B-GGUF-Q4_0".to_string())
        );
        assert_eq!(
            resolve_model_name(&models, "SD-Turbo"),
            Ok("SD-Turbo".to_string())
        );
        assert!(resolve_model_name(&[], "anything").is_err());
        // No silent substitution: unknown names fail instead of guessing.
        assert!(resolve_model_name(&models, "definitely-not-a-model").is_err());
    }

    #[test]
    fn unique_names_do_not_collide() {
        let a = unique_png_name("gen", Some(7));
        let b = unique_png_name("gen", Some(7));
        assert_ne!(a, b);
        assert!(a.ends_with(".png") && b.ends_with(".png"));
    }
}
