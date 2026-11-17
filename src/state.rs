use crate::lora::LoRAInfo;
use crate::paths as app_paths;
use gpui_kit::rgb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const FONT_HEADING: &str = "Climate Crisis";
pub const FONT_BODY: &str = "Sora";

fn default_true() -> bool {
    true
}

fn default_frame_width() -> u32 {
    64
}

fn default_frame_height() -> u32 {
    64
}

pub fn get_asset_path(subpath: &str) -> PathBuf {
    app_paths::resolve_asset(subpath)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActiveView {
    #[default]
    Studio,
    BackgroundRemover,
    Library,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemePreset {
    #[default]
    #[serde(alias = "Neubrutalism")]
    GumroadPop,
    AcidCyber,
    BubblegumPunk,
    BauhausCobalt,
}

impl ThemePreset {
    pub fn name(&self) -> &'static str {
        match self {
            ThemePreset::GumroadPop => "The Gumroad Classic",
            ThemePreset::AcidCyber => "Acid Matrix",
            ThemePreset::BubblegumPunk => "Bubblegum Punk",
            ThemePreset::BauhausCobalt => "Bauhaus Modern",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ThemePreset::GumroadPop => {
                "Iconic Gumroad primary pop: Electric Yellow (#FFEB3B), Coral (#FF5252), 2px solid outlines & warm cream canvas"
            }
            ThemePreset::AcidCyber => {
                "High-octane developer brutalism: Toxic Acid Lime (#CCFF00), Cyber Violet, and deep void contrast"
            }
            ThemePreset::BubblegumPunk => {
                "Playful Gen-Z pop-art brutalism: Hot Neon Magenta (#FF73FA), Tangerine (#FF9F1C), and crisp geometry"
            }
            ThemePreset::BauhausCobalt => {
                "Modernist architectural brutalism: Electric Cobalt (#2979FF), Safety Orange (#FF6D00), and pure white cards"
            }
        }
    }

    pub fn bg_hex(&self) -> u32 {
        match self {
            ThemePreset::GumroadPop => 0xfffdf5,
            ThemePreset::AcidCyber => 0xf4fbf4,
            ThemePreset::BubblegumPunk => 0xfff5f8,
            ThemePreset::BauhausCobalt => 0xf6f8fc,
        }
    }

    pub fn card_hex(&self) -> u32 {
        match self {
            ThemePreset::GumroadPop => 0xffffff,
            ThemePreset::AcidCyber => 0xffffff,
            ThemePreset::BubblegumPunk => 0xffffff,
            ThemePreset::BauhausCobalt => 0xffffff,
        }
    }

    pub fn border_hex(&self) -> u32 {
        match self {
            ThemePreset::GumroadPop => 0x000000,
            ThemePreset::AcidCyber => 0x000000,
            ThemePreset::BubblegumPunk => 0x000000,
            ThemePreset::BauhausCobalt => 0x000000,
        }
    }

    pub fn is_dark(&self) -> bool {
        false
    }

    pub fn signature_accent(&self) -> AccentColorPreset {
        match self {
            ThemePreset::GumroadPop => AccentColorPreset::ElectricYellow,
            ThemePreset::AcidCyber => AccentColorPreset::AcidLime,
            ThemePreset::BubblegumPunk => AccentColorPreset::BubblegumPink,
            ThemePreset::BauhausCobalt => AccentColorPreset::ElectricCobalt,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AccentColorPreset {
    #[default]
    #[serde(alias = "NeubrutalYellow", alias = "LemonadeAmber")]
    ElectricYellow,
    #[serde(alias = "EmeraldMatrix")]
    AcidLime,
    #[serde(alias = "ElectricViolet")]
    BubblegumPink,
    #[serde(alias = "NeubrutalBlue", alias = "CyberCyan")]
    ElectricCobalt,
    #[serde(alias = "NeubrutalCoral", alias = "CrimsonRed")]
    CoralPunch,
    SafetyOrange,
}

impl AccentColorPreset {
    pub fn hex(&self) -> u32 {
        match self {
            AccentColorPreset::ElectricYellow => 0xffeb3b,
            AccentColorPreset::AcidLime => 0xccff00,
            AccentColorPreset::BubblegumPink => 0xff73fa,
            AccentColorPreset::ElectricCobalt => 0x2979ff,
            AccentColorPreset::CoralPunch => 0xff5252,
            AccentColorPreset::SafetyOrange => 0xff6d00,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            AccentColorPreset::ElectricYellow => "Electric Yellow (#FFEB3B)",
            AccentColorPreset::AcidLime => "Toxic Acid Lime (#CCFF00)",
            AccentColorPreset::BubblegumPink => "Bubblegum Pink (#FF73FA)",
            AccentColorPreset::ElectricCobalt => "Electric Cobalt (#2979FF)",
            AccentColorPreset::CoralPunch => "Coral Punch (#FF5252)",
            AccentColorPreset::SafetyOrange => "Safety Orange (#FF6D00)",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UiScale {
    Compact,
    #[default]
    Cozy,
    Spacious,
}

impl UiScale {
    pub fn name(&self) -> &'static str {
        match self {
            UiScale::Compact => "Compact (14px)",
            UiScale::Cozy => "Cozy (16px)",
            UiScale::Spacious => "Spacious (18px)",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CornerRadiusPreset {
    RetroSharp,
    #[default]
    ModernSmooth,
    PillSoft,
}

impl CornerRadiusPreset {
    pub fn name(&self) -> &'static str {
        match self {
            CornerRadiusPreset::RetroSharp => "Sharp (0px)",
            CornerRadiusPreset::ModernSmooth => "Modern (8px)",
            CornerRadiusPreset::PillSoft => "Soft (16px)",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MotionLevel {
    Snappy,
    #[default]
    Smooth,
    ReducedMotion,
}

impl MotionLevel {
    pub fn name(&self) -> &'static str {
        match self {
            MotionLevel::Snappy => "Snappy (Instant, 0ms)",
            MotionLevel::Smooth => "Smooth (Balanced)",
            MotionLevel::ReducedMotion => "Reduced Motion",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RmbgEngineMode {
    #[default]
    NeuralAi,
    ChromaKey,
}

impl RmbgEngineMode {
    pub fn name(&self) -> &'static str {
        match self {
            RmbgEngineMode::NeuralAi => "Neural AI (People & Objects)",
            RmbgEngineMode::ChromaKey => "Chroma Key (Solid Color)",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SettingsCategory {
    #[default]
    ThemesAndStyle,
    AppFeel,
    Server,
    StudioDefaults,
    RmbgAndAseprite,
    Diagnostics,
}

impl SettingsCategory {
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            SettingsCategory::ThemesAndStyle => "Themes & Styling",
            SettingsCategory::AppFeel => "App Feel & Feedback",
            SettingsCategory::Server => "Server & Connection",
            SettingsCategory::StudioDefaults => "Studio Defaults",
            SettingsCategory::RmbgAndAseprite => "RMBG & Aseprite",
            SettingsCategory::Diagnostics => "Diagnostics & Storage",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CustomPrompt {
    pub id: String,
    pub title: String,
    pub category: String,
    pub prompt: String,
    pub width: u32,
    pub height: u32,
    pub steps: u32,
    pub cfg: f32,
    #[serde(default)]
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSettings {
    // Themes & Styling
    pub theme_preset: ThemePreset,
    pub is_dark_mode: bool,
    pub accent_color: AccentColorPreset,
    pub ui_scale: UiScale,
    pub corner_radius: CornerRadiusPreset,

    // App Feel & Sensory Feedback
    pub motion_level: MotionLevel,
    pub screen_shake: bool,
    pub crt_scanlines: bool,
    pub haptic_glow: bool,
    #[serde(default = "default_true")]
    pub textured_background: bool,
    pub sound_effects: bool,
    pub sound_volume: f32,

    // Server & Connection
    pub server_url: String,
    pub request_timeout_secs: u64,
    pub auto_ping_interval_secs: u64,
    pub api_key: Option<String>,

    // Studio Defaults
    pub default_model: String,
    pub default_width: u32,
    pub default_height: u32,
    pub default_steps: u32,
    pub default_cfg_scale: f32,
    pub auto_save_to_disk: bool,
    pub output_dir_path: String,
    pub filename_pattern: String,
    pub pixel_art_crisp_zoom: bool,

    // RMBG & Tooling Defaults
    pub rmbg_default_tolerance: f32,
    pub rmbg_default_pixel_mode: bool,
    pub rmbg_default_halo_trim: u32,
    pub aseprite_executable_path: String,
    pub auto_generate_aseprite_json: bool,
    #[serde(default = "default_frame_width")]
    pub aseprite_frame_width: u32,
    #[serde(default = "default_frame_height")]
    pub aseprite_frame_height: u32,

    // Cache & Diagnostics
    pub max_history_items: usize,
    pub low_vram_optimization: bool,

    // Custom Saved Prompts & Recipes
    #[serde(default)]
    pub custom_prompts: Vec<CustomPrompt>,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            theme_preset: ThemePreset::GumroadPop,
            is_dark_mode: false,
            accent_color: AccentColorPreset::ElectricYellow,
            ui_scale: UiScale::Cozy,
            corner_radius: CornerRadiusPreset::ModernSmooth,

            motion_level: MotionLevel::Smooth,
            screen_shake: true,
            crt_scanlines: false,
            haptic_glow: true,
            textured_background: true,
            sound_effects: true,
            sound_volume: 0.8,

            server_url: "http://127.0.0.1:13305".into(),
            request_timeout_secs: 180,
            auto_ping_interval_secs: 15,
            api_key: None,

            default_model: "FLUX.2-klein-4B-GGUF-Q4_0".into(),
            default_width: 512,
            default_height: 512,
            default_steps: 4,
            default_cfg_scale: 1.0,
            auto_save_to_disk: true,
            output_dir_path: app_paths::default_output_dir().to_string_lossy().to_string(),
            filename_pattern: "lemonade_{date}_{seed}.png".into(),
            pixel_art_crisp_zoom: true,

            rmbg_default_tolerance: 0.15,
            rmbg_default_pixel_mode: true,
            rmbg_default_halo_trim: 1,
            aseprite_executable_path: "aseprite".into(),
            auto_generate_aseprite_json: true,
            aseprite_frame_width: 64,
            aseprite_frame_height: 64,

            max_history_items: 25,
            low_vram_optimization: true,

            custom_prompts: vec![
                CustomPrompt {
                    id: "custom_init_1".into(),
                    title: "Neon Cyberpunk Hoverbike".into(),
                    category: "Mecha & Sci-Fi".into(),
                    prompt: "16-bit pixel art cyberpunk hoverbike with glowing neon magenta light strips, carbon-fiber body, dual plasma exhaust, side view, clean dark solid background, arcade racing game style".into(),
                    width: 1024,
                    height: 512,
                    steps: 24,
                    cfg: 4.5,
                    created_at: "Pre-installed".into(),
                },
                CustomPrompt {
                    id: "custom_init_2".into(),
                    title: "Arcane Runestone Golem".into(),
                    category: "Monsters".into(),
                    prompt: "pixel art spritesheet of an ancient mossy stone golem with glowing azure glyph runes, stone fist smash and laser beam attack frames, 16-bit SNES JRPG boss, clean solid backdrop".into(),
                    width: 768,
                    height: 768,
                    steps: 28,
                    cfg: 5.0,
                    created_at: "Pre-installed".into(),
                },
            ],
        }
    }
}

impl UserSettings {
    pub fn load_or_default(path: &Path) -> Self {
        if path.exists() {
            if let Ok(data) = std::fs::read_to_string(path) {
                if let Ok(mut settings) = serde_json::from_str::<UserSettings>(&data) {
                    if settings.accent_color == AccentColorPreset::ElectricYellow
                        && settings.theme_preset != ThemePreset::GumroadPop
                    {
                        settings.accent_color = settings.theme_preset.signature_accent();
                    }
                    if settings.default_model == "flux-2-klein-4b" {
                        settings.default_model = "FLUX.2-klein-4B-GGUF-Q4_0".into();
                        settings.default_width = 512;
                        settings.default_height = 512;
                        settings.default_steps = 4;
                        settings.default_cfg_scale = 1.0;
                    }
                    settings.validate_in_place();
                    // Persist migrated/normalized settings best-effort.
                    settings.save_to_disk(path);
                    return settings;
                }
            }
        }
        let default_settings = Self::default();
        default_settings.save_to_disk(path);
        default_settings
    }

    /// Clamp/normalize every persisted setting so corrupt configs degrade gracefully.
    pub fn validate_in_place(&mut self) {
        self.server_url = normalize_server_url(&self.server_url);
        self.request_timeout_secs = self.request_timeout_secs.clamp(5, 600);
        self.auto_ping_interval_secs = self.auto_ping_interval_secs.clamp(0, 300);
        self.sound_volume = self.sound_volume.clamp(0.0, 1.0);
        self.default_width = self.default_width.clamp(64, 2048);
        self.default_height = self.default_height.clamp(64, 2048);
        self.default_steps = self.default_steps.clamp(1, 100);
        self.default_cfg_scale = self.default_cfg_scale.clamp(0.0, 20.0);
        self.max_history_items = self.max_history_items.clamp(1, 200);
        self.rmbg_default_tolerance = self.rmbg_default_tolerance.clamp(0.01, 1.0);
        self.rmbg_default_halo_trim = self.rmbg_default_halo_trim.min(10);
        if self.aseprite_frame_width == 0 {
            self.aseprite_frame_width = 64;
        }
        if self.aseprite_frame_height == 0 {
            self.aseprite_frame_height = 64;
        }
        self.aseprite_frame_width = self.aseprite_frame_width.min(2048);
        self.aseprite_frame_height = self.aseprite_frame_height.min(2048);
        if self.output_dir_path.trim().is_empty() {
            self.output_dir_path = app_paths::default_output_dir()
                .to_string_lossy()
                .to_string();
        }
        if self.filename_pattern.trim().is_empty() {
            self.filename_pattern = "lemonade_{date}_{seed}.png".into();
        }
        if self.default_model.trim().is_empty() {
            self.default_model = "FLUX.2-klein-4B-GGUF-Q4_0".into();
        }
    }

    pub fn save_to_disk(&self, path: &Path) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            // Atomic write: never leave a truncated settings.json behind.
            if let Some(parent) = path.parent() {
                let tmp = parent.join(format!(".settings.{}.tmp", std::process::id()));
                if std::fs::write(&tmp, &json).is_ok() {
                    std::fs::rename(&tmp, path).ok();
                    return;
                }
            }
            std::fs::write(path, json).ok();
        }
    }
}

pub fn normalize_server_url(raw: &str) -> String {
    let trimmed = raw.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return "http://127.0.0.1:13305".into();
    }
    // Accept bare host:port and upgrade to http.
    if !trimmed.contains("://") {
        return format!("http://{}", trimmed);
    }
    trimmed.to_string()
}

/// Recommended generation defaults per model family.
pub fn model_defaults(model: &str) -> (u32, u32, u32, f32) {
    let lower = model.to_lowercase();
    if lower.contains("klein") {
        (512, 512, 4, 1.0)
    } else {
        (512, 512, 20, 4.5)
    }
}

#[derive(Clone, Debug)]
pub enum SettingUpdate {
    ThemePreset(ThemePreset),
    DarkMode(bool),
    AccentColor(AccentColorPreset),
    UiScale(UiScale),
    CornerRadius(CornerRadiusPreset),
    Motion(MotionLevel),
    ScreenShake(bool),
    CrtScanlines(bool),
    HapticGlow(bool),
    TexturedBackground(bool),
    SoundEffects(bool),
    SoundVolume(f32),
    ServerTimeout(u64),
    AutoPing(u64),
    DefaultResolution(u32, u32),
    DefaultSteps(u32),
    DefaultCfg(f32),
    AutoSave(bool),
    PixelCrisp(bool),
    RmbgTolerance(f32),
    RmbgPixelMode(bool),
    RmbgHaloTrim(u32),
    AsepriteJson(bool),
    AsepriteFrame(u32, u32),
    ServerUrl(String),
    ApiKey(Option<String>),
    SelectModel(String),
    RefreshModels,
    HistoryLimit(usize),
    ClearCache,
    ResetDefaults,
    AddCustomPrompt(CustomPrompt),
    UpdateCustomPrompt(CustomPrompt),
    DeleteCustomPrompt(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiffSynthContentRefConfig {
    pub enabled: bool,
    pub weight: f32,
    pub negative_baseline: bool,
    pub ref_image_path: Option<String>,
}

impl Default for DiffSynthContentRefConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            weight: 1.0,
            negative_baseline: false,
            ref_image_path: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiffSynthAestheticConfig {
    pub enabled: bool,
    pub weight: f32,
    pub negative_baseline: bool,
    pub block_slice: String,
    pub merge_type: String,
}

impl Default for DiffSynthAestheticConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            weight: 1.0,
            negative_baseline: false,
            block_slice: "all".into(),
            merge_type: "mean".into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub image_path: String,
    pub prompt: String,
    pub timestamp: String,
    pub width: u32,
    pub height: u32,
    pub seed: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogMessage {
    pub timestamp: String,
    pub level: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActiveModal {
    None,
    LoRAHelp(Box<LoRAInfo>),
    Logs,
    Settings,
    AddCustomRecipe,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct AppState {
    pub active_view: ActiveView,
    pub active_modal: ActiveModal,

    // Settings
    pub settings: UserSettings,
    pub settings_category: SettingsCategory,

    // Server
    pub server_url: String,
    pub is_server_online: bool,
    pub available_models: Vec<String>,
    pub selected_model: String,

    // Studio Generation
    pub prompt: String,
    pub negative_prompt: String,
    pub width: u32,
    pub height: u32,
    pub steps: u32,
    pub cfg_scale: f32,
    pub denoise_strength: f32,
    pub seed: i64,
    pub random_seed: bool,

    // LoRA states
    pub lora_weights: HashMap<String, f32>,
    pub lora_enabled: HashMap<String, bool>,
    pub show_weight_picker: bool,
    pub content_ref: DiffSynthContentRefConfig,
    pub aesthetic: DiffSynthAestheticConfig,
    pub show_advanced_diffsynth: bool,

    // Current Generation Result
    pub current_image_path: Option<String>,
    pub is_generating: bool,
    pub generation_epoch: u64,
    pub status_message: String,

    // Server discovery cache
    pub models_last_refresh: Option<std::time::Instant>,
    pub models_refreshing: bool,
    pub server_error: Option<String>,

    // History Strip
    pub history: Vec<HistoryEntry>,

    // Standalone RMBG
    pub rmbg_input_path: Option<String>,
    pub rmbg_output_path: Option<String>,
    pub rmbg_mask_path: Option<String>,
    pub rmbg_preview_epoch: u64,
    pub rmbg_engine_mode: RmbgEngineMode,
    pub rmbg_tolerance: f32,
    pub rmbg_pixel_mode: bool,
    pub rmbg_halo_trim: u32,
    pub rmbg_is_processing: bool,

    // Style & Prompt Library State
    pub library_category: String,
    pub new_recipe_category: String,
    pub new_recipe_width: u32,
    pub new_recipe_height: u32,
    pub new_recipe_steps: u32,
    pub new_recipe_cfg: f32,
    pub editing_recipe_id: Option<String>,

    // Logs
    pub logs: Vec<LogMessage>,
}

impl Default for AppState {
    fn default() -> Self {
        let settings_path = app_paths::settings_file();
        let settings = UserSettings::load_or_default(&settings_path);

        let mut lora_weights = HashMap::new();
        let mut lora_enabled = HashMap::new();

        lora_weights.insert("diffsynth_content_ref".into(), 1.0);
        lora_weights.insert("diffsynth_aesthetic".into(), 1.0);
        lora_weights.insert("pixel_spritesheet".into(), 0.85);
        lora_weights.insert("pixel_art_sprite".into(), 0.75);

        lora_enabled.insert("diffsynth_content_ref".into(), false);
        lora_enabled.insert("diffsynth_aesthetic".into(), false);
        lora_enabled.insert("pixel_spritesheet".into(), true);
        lora_enabled.insert("pixel_art_sprite".into(), false);

        let default_w = settings.default_width;
        let default_h = settings.default_height;
        let default_steps = settings.default_steps;
        let default_cfg = settings.default_cfg_scale;
        let default_model = settings.default_model.clone();
        let server_url = settings.server_url.clone();
        let rmbg_tol = settings.rmbg_default_tolerance;
        let rmbg_pix = settings.rmbg_default_pixel_mode;
        let rmbg_trim = settings.rmbg_default_halo_trim;

        let selected_model = if default_model == "flux-2-klein-4b" {
            "FLUX.2-klein-4B-GGUF-Q4_0".to_string()
        } else {
            default_model
        };

        Self {
            active_view: ActiveView::Studio,
            active_modal: ActiveModal::None,

            settings,
            settings_category: SettingsCategory::ThemesAndStyle,

            server_url,
            is_server_online: true,
            available_models: vec![
                "FLUX.2-klein-4B-GGUF-Q4_0".into(),
                "FLUX.2-klein-base-4b".into(),
            ],
            selected_model,

            prompt: "pixel art spritesheet of a knight in silver armor, walking and attack animations, solid dark background, 16-bit retro game style".into(),
            negative_prompt: "blurry, photographic, low quality, noisy background".into(),
            width: default_w,
            height: default_h,
            steps: default_steps,
            cfg_scale: default_cfg,
            denoise_strength: 0.75,
            seed: 42,
            random_seed: true,

            lora_weights,
            lora_enabled,
            show_weight_picker: false,
            content_ref: DiffSynthContentRefConfig::default(),
            aesthetic: DiffSynthAestheticConfig::default(),
            show_advanced_diffsynth: false,

            current_image_path: None,
            is_generating: false,
            generation_epoch: 0,
            status_message: "Ready to generate".into(),
            models_last_refresh: None,
            models_refreshing: false,
            server_error: None,

            history: Vec::new(),

            rmbg_input_path: None,
            rmbg_output_path: None,
            rmbg_mask_path: None,
            rmbg_preview_epoch: 0,
            rmbg_engine_mode: RmbgEngineMode::NeuralAi,
            rmbg_tolerance: rmbg_tol,
            rmbg_pixel_mode: rmbg_pix,
            rmbg_halo_trim: rmbg_trim,
            rmbg_is_processing: false,

            library_category: "All".into(),
            new_recipe_category: "Characters".into(),
            new_recipe_width: 512,
            new_recipe_height: 512,
            new_recipe_steps: 24,
            new_recipe_cfg: 4.5,
            editing_recipe_id: None,

            logs: vec![
                LogMessage {
                    timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                    level: "INFO".into(),
                    text: "Lemonade Studio GPUI initialized".into(),
                },
            ],
        }
    }
}

impl AppState {
    /// Single entry point for changing the RMBG source image.
    /// Clears stale mask/output state so a new source can never reuse an old result.
    pub fn set_rmbg_source(&mut self, path: Option<String>) {
        self.rmbg_input_path = path;
        self.rmbg_output_path = None;
        self.rmbg_mask_path = None;
        self.rmbg_is_processing = false;
    }

    /// Re-apply persisted settings to all mirrored runtime state.
    /// Used after ResetDefaults and after loading settings.
    pub fn sync_from_settings(&mut self) {
        self.server_url = self.settings.server_url.clone();
        self.selected_model = self.settings.default_model.clone();
        self.width = self.settings.default_width;
        self.height = self.settings.default_height;
        self.steps = self.settings.default_steps;
        self.cfg_scale = self.settings.default_cfg_scale;
        self.rmbg_tolerance = self.settings.rmbg_default_tolerance;
        self.rmbg_pixel_mode = self.settings.rmbg_default_pixel_mode;
        self.rmbg_halo_trim = self.settings.rmbg_default_halo_trim;
        self.set_rmbg_source(self.rmbg_input_path.clone());
        // History newest-first: drop oldest entries beyond the limit.
        while self.history.len() > self.settings.max_history_items {
            self.history.remove(0);
        }
    }

    pub fn add_log(&mut self, level: &str, text: &str) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        self.logs.push(LogMessage {
            timestamp,
            level: level.to_string(),
            text: text.to_string(),
        });
        if self.logs.len() > 200 {
            self.logs.remove(0);
        }
    }

    pub fn palette(&self) -> ThemePalette {
        ThemePalette::new(
            self.settings.theme_preset,
            self.settings.accent_color,
            self.settings.is_dark_mode,
        )
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct ThemePalette {
    pub is_dark: bool,
    pub bg_app: u32,
    pub bg_header: u32,
    pub bg_sidebar: u32,
    pub bg_viewport: u32,
    pub bg_card: u32,
    pub bg_card_header: u32,
    pub bg_input: u32,
    pub bg_chip: u32,
    pub bg_chip_active: u32,

    pub border: u32,
    pub border_strong: u32,
    pub border_subtle: u32,

    pub text_primary: u32,
    pub text_secondary: u32,
    pub text_muted: u32,

    pub accent: u32,
    pub accent_hover: u32,
    pub accent_subtle: u32,
    pub accent_text: u32,

    pub success: u32,
    pub warning: u32,
    pub danger: u32,
}

#[allow(dead_code)]
impl ThemePalette {
    pub fn new(preset: ThemePreset, accent: AccentColorPreset, is_dark: bool) -> Self {
        let (
            bg_app,
            bg_header,
            bg_sidebar,
            bg_viewport,
            bg_card,
            bg_card_header,
            bg_input,
            bg_chip,
            bg_chip_active,
            border,
            border_strong,
            border_subtle,
            text_primary,
            text_secondary,
            text_muted,
        ) = if is_dark {
            match preset {
                ThemePreset::GumroadPop => (
                    0x121212, 0x1e1e1e, 0x181818, 0x0e0e0e, 0x1e1e1e, 0x262626, 0x141414, 0x2b2b2b,
                    0xffeb3b, 0xffeb3b, 0xffeb3b, 0x383838, 0xffffff, 0xe4e4e4, 0x9e9e9e,
                ),
                ThemePreset::AcidCyber => (
                    0x0d0f12, 0x15181f, 0x121419, 0x0a0b0e, 0x161a22, 0x1f2430, 0x101217, 0x222735,
                    0xccff00, 0xccff00, 0xccff00, 0x2c352c, 0xffffff, 0xe0e6e0, 0x8fa38f,
                ),
                ThemePreset::BubblegumPunk => (
                    0x151017, 0x201824, 0x1b141e, 0x0f0b11, 0x221a26, 0x2d2233, 0x18121b, 0x322438,
                    0xff73fa, 0xff73fa, 0xff73fa, 0x422b48, 0xffffff, 0xf5e5f2, 0xb092ab,
                ),
                ThemePreset::BauhausCobalt => (
                    0x0d1117, 0x161c26, 0x131821, 0x0a0d12, 0x19212d, 0x222c3d, 0x11161e, 0x253142,
                    0x2979ff, 0x2979ff, 0x2979ff, 0x27364d, 0xffffff, 0xe1e9f5, 0x8a9eba,
                ),
            }
        } else {
            // Light variations (Clean Neubrutalism surfaces with 2px hard outlines)
            match preset {
                ThemePreset::GumroadPop => (
                    0xfffdf5, 0xffffff, 0xfffdf5, 0xfffdf0, 0xffffff, 0xfff8d6, 0xffffff, 0xf0f0f0,
                    0xffeb3b, 0x000000, 0x000000, 0x000000, 0x000000, 0x181818, 0x555555,
                ),
                ThemePreset::AcidCyber => (
                    0xf4fbf4, 0xffffff, 0xf8fdf8, 0xf0f9f0, 0xffffff, 0xeaffd0, 0xffffff, 0xedf7ed,
                    0xccff00, 0x000000, 0x000000, 0x000000, 0x000000, 0x141414, 0x4a554a,
                ),
                ThemePreset::BubblegumPunk => (
                    0xfff5f8, 0xffffff, 0xfff8fa, 0xfff0f5, 0xffffff, 0xffe6fc, 0xffffff, 0xfdecf4,
                    0xff73fa, 0x000000, 0x000000, 0x000000, 0x000000, 0x20151c, 0x5e4c56,
                ),
                ThemePreset::BauhausCobalt => (
                    0xf6f8fc, 0xffffff, 0xf8faff, 0xf0f4f9, 0xffffff, 0xe3eeff, 0xffffff, 0xeef3fa,
                    0x2979ff, 0x000000, 0x000000, 0x000000, 0x000000, 0x141c28, 0x4e5a6c,
                ),
            }
        };

        let effective_accent =
            if accent == AccentColorPreset::ElectricYellow && preset != ThemePreset::GumroadPop {
                preset.signature_accent()
            } else {
                accent
            };

        let (accent_val, accent_hover, accent_subtle, accent_text) = match effective_accent {
            AccentColorPreset::ElectricYellow => (
                0xffeb3b,
                0xfdd835,
                if is_dark { 0x332e00 } else { 0xfff9c4 },
                0x000000,
            ),
            AccentColorPreset::AcidLime => (
                0xccff00,
                0xb8e600,
                if is_dark { 0x253300 } else { 0xf2ffcc },
                0x000000,
            ),
            AccentColorPreset::BubblegumPink => (
                0xff73fa,
                0xe65be0,
                if is_dark { 0x3e113c } else { 0xffe3fd },
                0x000000,
            ),
            AccentColorPreset::ElectricCobalt => (
                0x2979ff,
                0x1c64e0,
                if is_dark { 0x0b2654 } else { 0xdbeaff },
                0xffffff,
            ),
            AccentColorPreset::CoralPunch => (
                0xff5252,
                0xff1744,
                if is_dark { 0x3d1212 } else { 0xffebee },
                0xffffff,
            ),
            AccentColorPreset::SafetyOrange => (
                0xff6d00,
                0xe65100,
                if is_dark { 0x3d1c00 } else { 0xffeedb },
                0xffffff,
            ),
        };

        Self {
            is_dark,
            bg_app,
            bg_header,
            bg_sidebar,
            bg_viewport,
            bg_card,
            bg_card_header,
            bg_input,
            bg_chip,
            bg_chip_active,
            border,
            border_strong,
            border_subtle,
            text_primary,
            text_secondary,
            text_muted,
            accent: accent_val,
            accent_hover,
            accent_subtle,
            accent_text,
            success: 0x10b981,
            warning: 0xf59e0b,
            danger: 0xef4444,
        }
    }

    pub fn border_rgba(&self) -> gpui_kit::Hsla {
        rgb(self.border).into()
    }

    pub fn accent_rgba(&self) -> gpui_kit::Hsla {
        rgb(self.accent).into()
    }
}

pub fn apply_theme_to_gpui(settings: &UserSettings, cx: &mut gpui_kit::App) {
    let mode = if settings.is_dark_mode {
        gpui_component::ThemeMode::Dark
    } else {
        gpui_component::ThemeMode::Light
    };
    gpui_component::Theme::change(mode, None, cx);

    let pal = ThemePalette::new(
        settings.theme_preset,
        settings.accent_color,
        settings.is_dark_mode,
    );

    gpui_component::Theme::update(cx, |theme| {
        theme.font_family = FONT_BODY.into();
        theme.colors.background = rgb(pal.bg_app).into();
        theme.colors.border = rgb(pal.border).into();
        theme.colors.input = rgb(pal.bg_input).into();
        theme.colors.foreground = rgb(pal.text_primary).into();
        theme.colors.muted = rgb(pal.bg_chip).into();
        theme.colors.muted_foreground = rgb(pal.text_muted).into();
        theme.colors.sidebar = rgb(pal.bg_sidebar).into();
        theme.colors.sidebar_border = rgb(pal.border).into();
        theme.colors.sidebar_foreground = rgb(pal.text_primary).into();
        theme.colors.sidebar_accent = rgb(pal.accent_subtle).into();
        theme.colors.primary = rgb(pal.accent).into();
        theme.colors.primary_hover = rgb(pal.accent_hover).into();
        theme.colors.primary_foreground = rgb(pal.accent_text).into();
        theme.colors.accent = rgb(pal.accent_subtle).into();
        theme.colors.accent_foreground = rgb(pal.text_primary).into();
        theme.colors.ring = rgb(pal.accent).into();
        theme.colors.secondary = rgb(pal.bg_chip).into();
        theme.colors.secondary_hover = rgb(pal.bg_chip_active).into();
        theme.colors.secondary_active = rgb(pal.bg_chip_active).into();
        theme.colors.secondary_foreground = rgb(pal.text_primary).into();
        theme.colors.button = rgb(pal.bg_chip).into();
        theme.colors.button_hover = rgb(pal.bg_chip_active).into();
        theme.colors.button_foreground = rgb(pal.text_primary).into();
        theme.colors.button_primary = rgb(pal.accent).into();
        theme.colors.button_primary_hover = rgb(pal.accent_hover).into();
        theme.colors.button_primary_foreground = rgb(pal.accent_text).into();
        theme.colors.button_secondary = rgb(pal.bg_chip).into();
        theme.colors.button_secondary_hover = rgb(pal.bg_chip_active).into();
        theme.colors.button_secondary_foreground = rgb(pal.text_primary).into();
        theme.colors.title_bar = rgb(pal.bg_header).into();
        theme.colors.title_bar_border = rgb(pal.border).into();
        theme.colors.status_bar = rgb(pal.bg_header).into();
        theme.colors.status_bar_border = rgb(pal.border).into();
        theme.colors.tab_bar = rgb(pal.bg_header).into();
        theme.colors.tab = rgb(pal.bg_header).into();
        theme.colors.tab_active = rgb(pal.accent).into();
        theme.colors.tab_active_foreground = rgb(pal.accent_text).into();
        theme.colors.popover = rgb(pal.bg_card).into();
        theme.colors.popover_foreground = rgb(pal.text_primary).into();
    });
}
