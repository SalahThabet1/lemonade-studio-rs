use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LoRAInfo {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub category: String,
    pub author: String,
    pub modelscope_url: String,
    pub huggingface_url: String,
    pub default_weight: f32,
    pub min_weight: f32,
    pub max_weight: f32,
    pub step: f32,
    pub description: String,
    pub trigger_words: Vec<String>,
    pub required_inputs: Option<String>,
    pub recommended_weight_range: (f32, f32),
    pub options_summary: Vec<String>,
    pub usage_notes: String,
    pub cookbook: Vec<CookbookEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CookbookEntry {
    pub title: String,
    pub prompt: String,
    pub recommended_weight: f32,
    pub notes: String,
}

pub fn get_lora_registry() -> Vec<LoRAInfo> {
    static CACHE: std::sync::OnceLock<Vec<LoRAInfo>> = std::sync::OnceLock::new();
    CACHE.get_or_init(build_lora_registry).clone()
}

fn lora_scan_dirs() -> Vec<std::path::PathBuf> {
    let mut dirs = vec![std::path::PathBuf::from("/var/lib/lemonade/loras")];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(std::path::PathBuf::from(home).join("models/loras"));
    }
    dirs
}

fn build_lora_registry() -> Vec<LoRAInfo> {
    let mut registry = vec![
        LoRAInfo {
            id: "diffsynth_content_ref".into(),
            name: "Template-KleinBase4B-ContentRef".into(),
            filename: "Template-KleinBase4B-ContentRef.safetensors".into(),
            category: "DiffSynth Studio / Diffusion Template".into(),
            author: "DiffSynth-Studio".into(),
            modelscope_url: "https://modelscope.cn/models/DiffSynth-Studio/Template-KleinBase4B-ContentRef".into(),
            huggingface_url: "https://huggingface.co/DiffSynth-Studio/Template-KleinBase4B-ContentRef".into(),
            default_weight: 1.0,
            min_weight: -2.0,
            max_weight: 4.0,
            step: 0.05,
            description: "Content Reference LoRA template for FLUX.2 Klein 4B. Designed to enforce character consistency, facial identity, clothing details, and composition from an input reference image.".into(),
            trigger_words: vec![
                "content reference".into(),
                "character consistency".into(),
                "identity preservation".into(),
            ],
            required_inputs: Some("Input Reference Image (Required when weight > 0). The image is injected into the diffusion pipeline via cross-attention to condition generation.".into()),
            recommended_weight_range: (0.7, 1.3),
            options_summary: vec![
                "Content Influence Scale: -2.0 to +4.0 (negative values invert content guidance)".into(),
                "Negative Baseline Switch: Inverts reference guidance to steer away from reference artifacts".into(),
                "Reference Image Dropzone: Load source character or pose".into(),
            ],
            usage_notes: "When using ContentRef, keep CFG between 3.5 and 7.0 for optimal balance between prompt adherence and reference preservation. To prevent reference burn, use weights between 0.8 and 1.1.".into(),
            cookbook: vec![
                CookbookEntry {
                    title: "Character Identity Transfer".into(),
                    prompt: "16-bit pixel art character sprite, walking animation frame, clean silhouette, neutral lighting".into(),
                    recommended_weight: 1.0,
                    notes: "Load character portrait or sprite as reference image".into(),
                },
                CookbookEntry {
                    title: "Subtle Pose Guidance".into(),
                    prompt: "pixel art rogue with dual daggers, dynamic combat pose, sharp outlines".into(),
                    recommended_weight: 0.75,
                    notes: "Lower weight lets prompt drive colors while preserving silhouette".into(),
                },
            ],
        },
        LoRAInfo {
            id: "diffsynth_aesthetic".into(),
            name: "Template-KleinBase4B-Aesthetic".into(),
            filename: "Template-KleinBase4B-Aesthetic.safetensors".into(),
            category: "DiffSynth Studio / Diffusion Template".into(),
            author: "DiffSynth-Studio".into(),
            modelscope_url: "https://modelscope.cn/models/DiffSynth-Studio/Template-KleinBase4B-Aesthetic".into(),
            huggingface_url: "https://huggingface.co/DiffSynth-Studio/Template-KleinBase4B-Aesthetic".into(),
            default_weight: 1.0,
            min_weight: -2.0,
            max_weight: 4.0,
            step: 0.05,
            description: "Aesthetic refinement and quality booster LoRA template for FLUX.2 Klein 4B. Enhances fine details, lighting, contrast, and shading cohesion.".into(),
            trigger_words: vec![
                "masterpiece".into(),
                "high aesthetic quality".into(),
                "fine shading".into(),
                "crisp palette".into(),
            ],
            required_inputs: None,
            recommended_weight_range: (0.6, 1.4),
            options_summary: vec![
                "Aesthetic Scale: -2.0 to +4.0 (negative values reduce over-processed gloss)".into(),
                "Negative Aesthetic Baseline: Counteracts generic AI smoothing".into(),
                "LoRA Block Selector: Mode toggle (All Blocks vs Custom Slice '1:180:2' odd layers)".into(),
                "Merge Strategy: Mean vs Sum LoRA weight accumulation".into(),
            ],
            usage_notes: "Aesthetic LoRA excels at weights 0.85-1.15. Applying it only to odd blocks ('1:180:2') preserves fine pixel grid details without distorting macro anatomy.".into(),
            cookbook: vec![
                CookbookEntry {
                    title: "Crisp Retro Pixel Shading".into(),
                    prompt: "pixel art knight armor, 16-bit palette, vibrant specular highlights, clean dithering".into(),
                    recommended_weight: 0.95,
                    notes: "Use 'mean' merge strategy and odd-block slice for best retro look".into(),
                },
                CookbookEntry {
                    title: "Dark Fantasy Atmosphere".into(),
                    prompt: "dark dungeon corridor, flickering torchlight, pixelated cobblestone floor, high contrast".into(),
                    recommended_weight: 1.1,
                    notes: "Increases shadow depth and contrast cohesion".into(),
                },
            ],
        },
        LoRAInfo {
            id: "pixel_spritesheet".into(),
            name: "Pixel SpriteSheet v1.0".into(),
            filename: "pixel_spritesheet.safetensors".into(),
            category: "Pixel Art / Gamedev".into(),
            author: "Lemonade Community".into(),
            modelscope_url: "https://modelscope.cn/models/Lemonade-Studio/Pixel-SpriteSheet".into(),
            huggingface_url: "https://huggingface.co/Lemonade-Studio/Pixel-SpriteSheet".into(),
            default_weight: 0.85,
            min_weight: 0.0,
            max_weight: 2.0,
            step: 0.05,
            description: "Specialized sprite sheet generator creating modular game characters in multi-directional walk, idle, and attack poses with solid backgrounds for easy background removal.".into(),
            trigger_words: vec![
                "pixel art".into(),
                "spritesheet".into(),
                "character sheet".into(),
                "16-bit".into(),
                "solid background".into(),
            ],
            required_inputs: None,
            recommended_weight_range: (0.75, 1.0),
            options_summary: vec![
                "Weight scale: 0.0 to 2.0".into(),
                "Works best with 512x512 or 1024x1024 resolutions".into(),
            ],
            usage_notes: "Always include 'solid flat background, no dithering on background' in prompt to allow instant 1-click RMBG isolation in the Background Remover tab.".into(),
            cookbook: vec![
                CookbookEntry {
                    title: "8-Directional Hero Sheet".into(),
                    prompt: "pixel art spritesheet of a paladin warrior in shining silver plate armor, walking animation, solid magenta background".into(),
                    recommended_weight: 0.85,
                    notes: "Solid background color makes RMBG removal 100% accurate".into(),
                },
            ],
        },
        LoRAInfo {
            id: "pixel_art_sprite".into(),
            name: "Clean Pixel Art Sprite v2".into(),
            filename: "pixel_art_sprite.safetensors".into(),
            category: "Pixel Art / Gamedev".into(),
            author: "Lemonade Community".into(),
            modelscope_url: "https://modelscope.cn/models/Lemonade-Studio/Clean-Pixel-Sprite-v2".into(),
            huggingface_url: "https://huggingface.co/Lemonade-Studio/Clean-Pixel-Sprite-v2".into(),
            default_weight: 0.75,
            min_weight: 0.0,
            max_weight: 2.0,
            step: 0.05,
            description: "Produces crisp, low-color-palette individual sprites and props with sharp contours and strict pixel boundaries.".into(),
            trigger_words: vec![
                "pixel art".into(),
                "clean sprite".into(),
                "outlined".into(),
                "retro game asset".into(),
            ],
            required_inputs: None,
            recommended_weight_range: (0.6, 0.9),
            options_summary: vec![
                "Weight scale: 0.0 to 2.0".into(),
                "Ideal for weapon icons, potion bottles, monsters, and UI widgets".into(),
            ],
            usage_notes: "Pair with Aesthetic LoRA at 0.9 for ultra-clean pixel art edges without anti-aliasing artifacts.".into(),
            cookbook: vec![
                CookbookEntry {
                    title: "Magical Potion Icon".into(),
                    prompt: "pixel art potion bottle, glowing cyan elixir, glass reflections, black outline, solid white background".into(),
                    recommended_weight: 0.8,
                    notes: "Pair with RMBG 1-bit alpha for game-ready asset".into(),
                },
            ],
        },
    ];

    // Dynamically scan local LoRA directories for any installed weights
    let scan_dirs = lora_scan_dirs();

    for dir in &scan_dirs {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "safetensors" || ext == "gguf" {
                        let filename = path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        let stem = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        // Skip if already in registry
                        if registry
                            .iter()
                            .any(|r| r.filename == filename || r.id == stem)
                        {
                            continue;
                        }

                        // Generate clean title and trigger words from filename
                        let display_name = stem.replace(['_', '-'], " ");
                        let title = display_name
                            .split_whitespace()
                            .map(|word| {
                                let mut chars = word.chars();
                                match chars.next() {
                                    None => String::new(),
                                    Some(first) => {
                                        first.to_uppercase().collect::<String>() + chars.as_str()
                                    }
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(" ");

                        let trigger_words = if stem.contains("walk") {
                            vec![
                                "pixel art".into(),
                                "walk cycle".into(),
                                "spritesheet".into(),
                            ]
                        } else if stem.contains("pixel") {
                            vec!["pixel art".into(), "sprite".into(), "16-bit".into()]
                        } else {
                            vec![display_name.clone()]
                        };

                        registry.push(LoRAInfo {
                            id: stem,
                            name: title,
                            filename,
                            category: "Custom Installed Weights".into(),
                            author: "Local System".into(),
                            modelscope_url: "https://modelscope.cn".into(),
                            huggingface_url: "https://huggingface.co".into(),
                            default_weight: 0.8,
                            min_weight: 0.0,
                            max_weight: 2.0,
                            step: 0.05,
                            description: "Custom local diffusion weight installed on system."
                                .into(),
                            trigger_words,
                            required_inputs: None,
                            recommended_weight_range: (0.5, 1.2),
                            options_summary: vec!["Weight scale: 0.0 to 2.0".into()],
                            usage_notes: "Locally installed custom weight.".into(),
                            cookbook: vec![],
                        });
                    }
                }
            }
        }
    }

    registry
}
