use crate::api::LemonadeClient;
use crate::components::add_recipe_dialog::render_add_recipe_dialog;
use crate::components::control_panel::render_control_panel;
use crate::components::header::render_header;
use crate::components::log_dialog::render_log_dialog;
use crate::components::lora_help_dialog::render_lora_help_dialog;
use crate::components::nav_rail::render_nav_rail;
use crate::components::preview_panel::render_preview_panel;
use crate::components::settings_dialog::render_settings_dialog;
use crate::components::standalone_rmbg::render_standalone_rmbg;
use crate::components::style_library::render_style_library;
use crate::rmbg_engine::{export_aseprite_spritesheet, process_background_removal};
use crate::state::{
    ActiveModal, ActiveView, AppState, CustomPrompt, HistoryEntry, RmbgEngineMode, SettingUpdate,
};
use gpui::Subscription;
use gpui::prelude::FluentBuilder as _;
use gpui_base::slider::{SliderEvent, SliderState};
use gpui_component::input::{InputEvent, InputState, TextareaState};
use gpui_component::{h_flex, v_flex};
use gpui_kit::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

pub struct StudioApp {
    pub state: AppState,
    pub api_client: Arc<LemonadeClient>,
    pub output_dir: PathBuf,
    pub settings_file: PathBuf,
    pub prompt_input: Entity<TextareaState>,
    pub recipe_title_input: Entity<InputState>,
    pub recipe_prompt_input: Entity<TextareaState>,
    pub server_url_input: Entity<InputState>,
    pub api_key_input: Entity<InputState>,
    pub rmbg_tol_slider: Entity<SliderState>,
    pub rmbg_halo_slider: Entity<SliderState>,
    pub focus_handle: gpui::FocusHandle,
    save_generation: Arc<AtomicU64>,
    poll_generation: Arc<AtomicU64>,
    _subscription: Option<Subscription>,
    _sub_tol: Option<Subscription>,
    _sub_halo: Option<Subscription>,
}

impl StudioApp {
    pub fn new(
        prompt_input: Entity<TextareaState>,
        recipe_title_input: Entity<InputState>,
        recipe_prompt_input: Entity<TextareaState>,
        server_url_input: Entity<InputState>,
        api_key_input: Entity<InputState>,
        cx: &mut Context<Self>,
    ) -> Self {
        let state = AppState::default();
        let api_client = Arc::new(LemonadeClient::new(state.server_url.clone()));
        let output_dir = PathBuf::from(state.settings.output_dir_path.clone());
        let settings_file = crate::paths::settings_file();
        std::fs::create_dir_all(&output_dir).ok();

        let sub = cx.subscribe(&prompt_input, |this, entity, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                let text = entity.read(cx).value().to_string();
                this.state.prompt = text;
            }
        });

        let rmbg_tol_slider = cx.new(|_| {
            SliderState::new()
                .min(0.01)
                .max(1.0)
                .step(0.01)
                .default_value(state.rmbg_tolerance)
        });

        let rmbg_halo_slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(10.0)
                .step(1.0)
                .default_value(state.rmbg_halo_trim as f32)
        });

        let sub_tol = cx.subscribe(
            &rmbg_tol_slider,
            |this, _entity, event: &SliderEvent, cx| {
                let val = match event {
                    SliderEvent::Change(val) | SliderEvent::Release(val) => val,
                };
                let t = val.start().clamp(0.01, 1.0);
                this.state.rmbg_tolerance = t;
                this.apply_live_rmbg_tuning(cx);
                cx.notify();
            },
        );

        let sub_halo = cx.subscribe(
            &rmbg_halo_slider,
            |this, _entity, event: &SliderEvent, cx| {
                let val = match event {
                    SliderEvent::Change(val) | SliderEvent::Release(val) => val,
                };
                let h = val.start().round().clamp(0.0, 10.0) as u32;
                this.state.rmbg_halo_trim = h;
                this.apply_live_rmbg_tuning(cx);
                cx.notify();
            },
        );

        // Auto-check health and discover loaded models on server startup
        let client_discovery = api_client.clone();
        cx.spawn(async move |this, cx| {
            let (is_online, models) = crate::api::TOKIO_RT
                .spawn(async move {
                    let online = client_discovery.check_health().await;
                    let models = if online {
                        client_discovery.list_models().await
                    } else {
                        vec![]
                    };
                    (online, models)
                })
                .await
                .unwrap_or((false, vec![]));

            this.update(cx, |app, cx| {
                app.state.is_server_online = is_online;
                if !models.is_empty() {
                    app.state.available_models = models;
                    if !app
                        .state
                        .available_models
                        .contains(&app.state.selected_model)
                    {
                        if let Some(first) = app.state.available_models.first() {
                            app.state.selected_model = first.clone();
                        }
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();

        let app = Self {
            state,
            api_client: api_client.clone(),
            output_dir,
            settings_file,
            prompt_input,
            recipe_title_input,
            recipe_prompt_input,
            server_url_input,
            api_key_input,
            rmbg_tol_slider,
            rmbg_halo_slider,
            focus_handle: cx.focus_handle(),
            save_generation: Arc::new(AtomicU64::new(0)),
            poll_generation: Arc::new(AtomicU64::new(0)),
            _subscription: Some(sub),
            _sub_tol: Some(sub_tol),
            _sub_halo: Some(sub_halo),
        };
        app.start_health_polling(cx);
        app
    }

    /// Start (or restart) periodic server health polling.
    /// Only one poll loop runs at a time; older loops exit on generation mismatch.
    pub fn start_health_polling(&self, cx: &mut Context<Self>) {
        let generation = self.poll_generation.fetch_add(1, Ordering::Relaxed) + 1;
        let interval = self.state.settings.auto_ping_interval_secs;
        if interval == 0 {
            return;
        }
        let client = self.api_client.clone();
        let poll_gen = self.poll_generation.clone();
        let view = cx.entity().clone();
        cx.spawn(async move |_, cx| {
            loop {
                tokio::time::sleep(Duration::from_secs(interval)).await;
                if poll_gen.load(Ordering::Relaxed) != generation {
                    break;
                }
                let online = client.check_health().await;
                let models = if online {
                    client.list_models().await
                } else {
                    Vec::new()
                };
                let _ = view.update(cx, |app: &mut StudioApp, cx| {
                    app.state.is_server_online = online;
                    app.state.models_last_refresh = Some(std::time::Instant::now());
                    if online {
                        app.state.server_error = None;
                        if !models.is_empty() {
                            app.state.available_models = models;
                        }
                    }
                    cx.notify();
                });
            }
        })
        .detach();
    }

    /// Coalesced settings persistence: rapid changes collapse into one write.
    pub fn schedule_settings_save(&self) {
        let generation = self.save_generation.fetch_add(1, Ordering::Relaxed) + 1;
        let snapshot = self.state.settings.clone();
        let path = self.settings_file.clone();
        let gen_handle = self.save_generation.clone();
        crate::api::TOKIO_RT.spawn(async move {
            tokio::time::sleep(Duration::from_millis(300)).await;
            if gen_handle.load(Ordering::Relaxed) != generation {
                return;
            }
            snapshot.save_to_disk(&path);
        });
    }

    /// Write settings immediately (reset, shutdown paths).
    pub fn flush_settings(&self) {
        self.save_generation.fetch_add(1, Ordering::Relaxed);
        self.state.settings.save_to_disk(&self.settings_file);
    }

    /// Rebuild the HTTP client after endpoint/timeout/key changes.
    pub fn rebuild_client(&mut self) {
        let timeout = self.state.settings.request_timeout_secs;
        let key = self.state.settings.api_key.clone();
        self.api_client = Arc::new(LemonadeClient::with_options(
            self.state.settings.server_url.clone(),
            timeout,
            key.as_deref(),
        ));
        self.state.server_url = self.state.settings.server_url.clone();
    }

    /// Explicit model discovery refresh with 60s cache unless forced.
    pub fn refresh_models(&mut self, cx: &mut Context<Self>, force: bool) {
        if self.state.models_refreshing {
            return;
        }
        if !force {
            if let Some(t) = self.state.models_last_refresh {
                if t.elapsed() < Duration::from_secs(60) {
                    return;
                }
            }
        }
        self.state.models_refreshing = true;
        cx.notify();
        let client = self.api_client.clone();
        cx.spawn(async move |this, cx| {
            let online = client.check_health().await;
            let models = if online {
                client.list_models().await
            } else {
                Vec::new()
            };
            this.update(cx, |app, cx| {
                app.state.models_refreshing = false;
                app.state.is_server_online = online;
                app.state.models_last_refresh = Some(std::time::Instant::now());
                if online {
                    app.state.server_error = None;
                    if !models.is_empty() {
                        app.state.available_models = models;
                        if !app
                            .state
                            .available_models
                            .contains(&app.state.selected_model)
                        {
                            if let Some(first) = app.state.available_models.first().cloned() {
                                app.state.selected_model = first;
                            }
                        }
                    }
                } else {
                    app.state.server_error = Some(format!(
                        "Lemonade server offline at {}",
                        app.api_client.base_url
                    ));
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Cancel in-flight generation; stale results are ignored by epoch.
    pub fn cancel_generation(&mut self, cx: &mut Context<Self>) {
        self.state.generation_epoch = self.state.generation_epoch.wrapping_add(1);
        if self.state.is_generating {
            self.state.is_generating = false;
            self.state.status_message = "Generation cancelled.".into();
            self.state.add_log("WARN", "Generation cancelled by user");
            cx.notify();
        }
    }

    /// Interactively updates the background removal cutout in real time (~1-2ms)
    /// without re-running heavy neural network inference.
    pub fn apply_live_rmbg_tuning(&mut self, cx: &mut Context<Self>) {
        if let Some(ref in_str) = self.state.rmbg_input_path.clone() {
            let in_p = PathBuf::from(in_str);
            let tol = self.state.rmbg_tolerance;
            let pix = self.state.rmbg_pixel_mode;
            let halo = self.state.rmbg_halo_trim;

            // If we have a cached raw mask:
            let mask_opt = self.state.rmbg_mask_path.clone().or_else(|| {
                self.state.rmbg_output_path.as_ref().map(|p| {
                    PathBuf::from(p)
                        .with_extension("mask.png")
                        .to_string_lossy()
                        .to_string()
                })
            });

            if let Some(mask_str) = mask_opt {
                let mask_p = PathBuf::from(&mask_str);
                if mask_p.exists() {
                    self.state.rmbg_preview_epoch += 1;
                    // Write live slider preview to the app cache dir to prevent spamming outputs/
                    let cache = crate::paths::cache_dir();
                    std::fs::create_dir_all(&cache).ok();
                    let out_path = cache.join(format!(
                        "lemonade_rmbg_live_{}.png",
                        self.state.rmbg_preview_epoch % 2
                    ));
                    if crate::rmbg_engine::apply_live_mask_tuning(
                        &in_p, &mask_p, &out_path, tol, pix, halo,
                    )
                    .is_ok()
                    {
                        self.state.rmbg_output_path = Some(out_path.to_string_lossy().to_string());
                        self.state.rmbg_mask_path = Some(mask_str);
                        cx.notify();
                        return;
                    }
                }
            }

            // In ChromaKey mode, we can compute live without a neural mask in ~2ms:
            if self.state.rmbg_engine_mode == RmbgEngineMode::ChromaKey {
                self.state.rmbg_preview_epoch += 1;
                // Write live slider preview to the app cache dir to prevent spamming outputs/
                let cache = crate::paths::cache_dir();
                std::fs::create_dir_all(&cache).ok();
                let out_path = cache.join(format!(
                    "lemonade_rmbg_live_chroma_{}.png",
                    self.state.rmbg_preview_epoch % 2
                ));
                if let Ok(res) = crate::rmbg_engine::process_background_removal_chroma(
                    &in_p, &out_path, tol, pix, halo,
                ) {
                    self.state.rmbg_output_path =
                        Some(res.output_path.to_string_lossy().to_string());
                    self.state.rmbg_mask_path = Some(
                        res.output_path
                            .with_extension("mask.png")
                            .to_string_lossy()
                            .to_string(),
                    );
                    cx.notify();
                }
            }
        }
    }

    pub fn trigger_generation(&mut self, cx: &mut Context<Self>) {
        if self.state.is_generating {
            return;
        }

        self.state.generation_epoch = self.state.generation_epoch.wrapping_add(1);
        let epoch = self.state.generation_epoch;
        self.state.is_generating = true;
        self.state.status_message = "Connecting to Lemonade server...".into();
        self.state
            .add_log("INFO", "Initiating image generation task...");
        // Honesty gate: LoRA/reference controls currently adjust local state only.
        // The Lemonade image API request below does not yet carry them.
        if self.state.content_ref.enabled
            || self.state.aesthetic.enabled
            || self.state.lora_enabled.values().any(|&e| e)
        {
            self.state.add_log(
                "WARN",
                "LoRA/ContentRef/Aesthetic controls are preview-only: current server request sends model, prompt, size, steps, CFG, and seed.",
            );
        }
        cx.notify();

        let model = self.state.selected_model.clone();
        let prompt = self.state.prompt.clone();
        let width = self.state.width;
        let height = self.state.height;
        let steps = self.state.steps;
        let cfg = self.state.cfg_scale;
        let seed = if self.state.random_seed {
            chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
        } else {
            self.state.seed
        };

        let client = self.api_client.clone();
        // Honor auto-save: preview-only runs stay in the cache dir.
        let out_dir = if self.state.settings.auto_save_to_disk {
            self.output_dir.clone()
        } else {
            let cache = crate::paths::cache_dir();
            std::fs::create_dir_all(&cache).ok();
            cache
        };
        let prompt_clone = prompt.clone();

        cx.spawn(async move |this, cx| {
            let result = crate::api::TOKIO_RT
                .spawn(async move {
                    client
                        .generate_image(
                            &model,
                            &prompt_clone,
                            width,
                            height,
                            steps,
                            cfg,
                            Some(seed),
                            &out_dir,
                        )
                        .await
                })
                .await
                .unwrap_or_else(|e| Err(anyhow::anyhow!("Tokio runtime error: {}", e)));

            this.update(cx, |app, cx| {
                // Ignore stale completions after cancel/new generation.
                if app.state.generation_epoch != epoch {
                    return;
                }
                app.state.is_generating = false;
                match result {
                    Ok(path) => {
                        let path_str = path.to_string_lossy().to_string();
                        app.state.current_image_path = Some(path_str.clone());
                        app.state.status_message = "Generation complete!".into();
                        app.state
                            .add_log("SUCCESS", &format!("Generated: {}", path_str));

                        if app.state.settings.screen_shake {
                            app.state
                                .add_log("INFO", "Juice: Screen shake recoil triggered");
                        }
                        if app.state.settings.sound_effects {
                            app.state
                                .add_log("INFO", "Audio: Chime cue played on complete");
                        }

                        app.state.history.push(HistoryEntry {
                            id: uuid_string(),
                            image_path: path_str,
                            prompt: prompt.clone(),
                            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                            width,
                            height,
                            seed,
                        });

                        let max_hist = app.state.settings.max_history_items;
                        if app.state.history.len() > max_hist {
                            app.state.history.remove(0);
                        }
                    }
                    Err(e) => {
                        app.state.status_message = format!("Error: {}", e);
                        app.state
                            .add_log("ERROR", &format!("Generation failed: {}", e));
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    pub fn apply_setting_update(&mut self, update: SettingUpdate, cx: &mut App) {
        // Computed before the by-value match below (which moves `update`).
        let needs_client_rebuild = matches!(
            update,
            SettingUpdate::ServerTimeout(_)
                | SettingUpdate::ServerUrl(_)
                | SettingUpdate::ApiKey(_)
        );
        match update {
            SettingUpdate::ThemePreset(preset) => {
                self.state.settings.theme_preset = preset;
                self.state.settings.accent_color = preset.signature_accent();
                self.state.settings.is_dark_mode = preset.is_dark();
                crate::state::apply_theme_to_gpui(&self.state.settings, cx);
                self.state.add_log(
                    "INFO",
                    &format!(
                        "Switched to {} theme (accent: {})",
                        preset.name(),
                        self.state.settings.accent_color.name()
                    ),
                );
            }
            SettingUpdate::DarkMode(dark) => {
                self.state.settings.is_dark_mode = dark;
                crate::state::apply_theme_to_gpui(&self.state.settings, cx);
                self.state.add_log(
                    "INFO",
                    if dark {
                        "Dark mode active"
                    } else {
                        "Light mode active"
                    },
                );
            }
            SettingUpdate::AccentColor(acc) => {
                self.state.settings.accent_color = acc;
                crate::state::apply_theme_to_gpui(&self.state.settings, cx);
                self.state
                    .add_log("INFO", &format!("Accent color set to {}", acc.name()));
            }
            SettingUpdate::UiScale(scale) => {
                self.state.settings.ui_scale = scale;
                self.state
                    .add_log("INFO", &format!("UI scale set to {}", scale.name()));
            }
            SettingUpdate::CornerRadius(radius) => {
                self.state.settings.corner_radius = radius;
                self.state
                    .add_log("INFO", &format!("Corner radius set to {}", radius.name()));
            }
            SettingUpdate::Motion(m) => {
                self.state.settings.motion_level = m;
                self.state
                    .add_log("INFO", &format!("Motion speed set to {}", m.name()));
            }
            SettingUpdate::ScreenShake(shake) => {
                self.state.settings.screen_shake = shake;
                self.state
                    .add_log("INFO", &format!("Tactile screen shake: {}", shake));
            }
            SettingUpdate::CrtScanlines(crt) => {
                self.state.settings.crt_scanlines = crt;
                self.state
                    .add_log("INFO", &format!("Retro CRT scanlines: {}", crt));
            }
            SettingUpdate::HapticGlow(glow) => {
                self.state.settings.haptic_glow = glow;
                self.state
                    .add_log("INFO", &format!("Active model glow: {}", glow));
            }
            SettingUpdate::TexturedBackground(tex) => {
                self.state.settings.textured_background = tex;
                self.state
                    .add_log("INFO", &format!("Tactile paper noise texture: {}", tex));
            }
            SettingUpdate::SoundEffects(snd) => {
                self.state.settings.sound_effects = snd;
                self.state.add_log("INFO", &format!("Audio cues: {}", snd));
            }
            SettingUpdate::SoundVolume(vol) => {
                self.state.settings.sound_volume = vol;
            }
            SettingUpdate::ServerTimeout(t) => {
                self.state.settings.request_timeout_secs = t;
                self.rebuild_client();
                self.state
                    .add_log("INFO", &format!("Request timeout: {}s", t));
            }
            SettingUpdate::AutoPing(p) => {
                self.state.settings.auto_ping_interval_secs = p;
                self.state
                    .add_log("INFO", &format!("Health polling: {}s", p));
            }
            SettingUpdate::ServerUrl(url) => {
                let normalized = crate::state::normalize_server_url(&url);
                self.state.settings.server_url = normalized.clone();
                self.rebuild_client();
                self.state.models_last_refresh = None;
                self.state
                    .add_log("INFO", &format!("Lemonade endpoint: {}", normalized));
            }
            SettingUpdate::ApiKey(key) => {
                self.state.settings.api_key = key.clone();
                self.rebuild_client();
                self.state.add_log(
                    "INFO",
                    if key
                        .as_deref()
                        .map(|k| !k.trim().is_empty())
                        .unwrap_or(false)
                    {
                        "API key configured"
                    } else {
                        "API key cleared"
                    },
                );
            }
            SettingUpdate::SelectModel(model) => {
                self.state.selected_model = model.clone();
                self.state.settings.default_model = model.clone();
                // Apply per-model generation defaults so Klein profiles stay fast
                // and other models get a usable quality baseline.
                let (w, h, steps, cfg) = crate::state::model_defaults(&model);
                self.state.width = w;
                self.state.height = h;
                self.state.steps = steps;
                self.state.cfg_scale = cfg;
                self.state.settings.default_width = w;
                self.state.settings.default_height = h;
                self.state.settings.default_steps = steps;
                self.state.settings.default_cfg_scale = cfg;
                self.state
                    .add_log("INFO", &format!("Selected model: {}", model));
            }
            SettingUpdate::RefreshModels => {
                self.state.models_last_refresh = None;
                self.state.add_log("INFO", "Refreshing Lemonade models...");
            }
            SettingUpdate::DefaultResolution(w, h) => {
                self.state.settings.default_width = w;
                self.state.settings.default_height = h;
                self.state.width = w;
                self.state.height = h;
                self.state
                    .add_log("INFO", &format!("Default resolution set to {}×{}", w, h));
            }
            SettingUpdate::DefaultSteps(s) => {
                self.state.settings.default_steps = s;
                self.state.steps = s;
                self.state
                    .add_log("INFO", &format!("Default steps set to {}", s));
            }
            SettingUpdate::DefaultCfg(cfg) => {
                self.state.settings.default_cfg_scale = cfg;
                self.state.cfg_scale = cfg;
                self.state
                    .add_log("INFO", &format!("Default CFG set to {:.1}", cfg));
            }
            SettingUpdate::AutoSave(b) => {
                self.state.settings.auto_save_to_disk = b;
                self.state
                    .add_log("INFO", &format!("Auto-save to disk: {}", b));
            }
            SettingUpdate::PixelCrisp(b) => {
                self.state.settings.pixel_art_crisp_zoom = b;
                self.state
                    .add_log("INFO", &format!("Crisp pixel zoom: {}", b));
            }
            SettingUpdate::RmbgTolerance(tol) => {
                self.state.settings.rmbg_default_tolerance = tol;
                self.state.rmbg_tolerance = tol;
                self.state.add_log(
                    "INFO",
                    &format!("Default RMBG tolerance: {:.0}%", tol * 100.0),
                );
            }
            SettingUpdate::RmbgPixelMode(b) => {
                self.state.settings.rmbg_default_pixel_mode = b;
                self.state.rmbg_pixel_mode = b;
                self.state
                    .add_log("INFO", &format!("Default 1-bit alpha: {}", b));
            }
            SettingUpdate::RmbgHaloTrim(t) => {
                self.state.settings.rmbg_default_halo_trim = t;
                self.state.rmbg_halo_trim = t;
                self.state
                    .add_log("INFO", &format!("Default halo trim: {}px", t));
            }
            SettingUpdate::AsepriteJson(b) => {
                self.state.settings.auto_generate_aseprite_json = b;
                self.state
                    .add_log("INFO", &format!("Auto-generate Aseprite JSON: {}", b));
            }
            SettingUpdate::AsepriteFrame(w, h) => {
                let w = w.clamp(1, 2048);
                let h = h.clamp(1, 2048);
                self.state.settings.aseprite_frame_width = w;
                self.state.settings.aseprite_frame_height = h;
                self.state
                    .add_log("INFO", &format!("Aseprite frame size: {}x{}", w, h));
            }
            SettingUpdate::HistoryLimit(lim) => {
                let lim = lim.clamp(1, 200);
                self.state.settings.max_history_items = lim;
                // History is newest-last: drop oldest entries first.
                while self.state.history.len() > lim {
                    self.state.history.remove(0);
                }
                self.state
                    .add_log("INFO", &format!("History limit set to {} items", lim));
            }
            SettingUpdate::ClearCache => {
                self.state.history.clear();
                self.state.set_rmbg_source(None);
                self.state.current_image_path = None;
                self.state
                    .add_log("INFO", "Cleared history, preview, and RMBG state");
            }
            SettingUpdate::ResetDefaults => {
                self.state.settings = crate::state::UserSettings::default();
                self.state.sync_from_settings();
                self.rebuild_client();
                // Sync RMBG sliders with restored defaults.
                self.rmbg_tol_slider.update(cx, |slider, cx| {
                    *slider = SliderState::new()
                        .min(0.01)
                        .max(1.0)
                        .step(0.01)
                        .default_value(self.state.rmbg_tolerance);
                    cx.notify();
                });
                self.rmbg_halo_slider.update(cx, |slider, cx| {
                    *slider = SliderState::new()
                        .min(0.0)
                        .max(10.0)
                        .step(1.0)
                        .default_value(self.state.rmbg_halo_trim as f32);
                    cx.notify();
                });
                crate::state::apply_theme_to_gpui(&self.state.settings, cx);
                self.flush_settings();
                self.state
                    .add_log("WARN", "Reset all application settings to defaults");
                return;
            }
            SettingUpdate::AddCustomPrompt(prompt) => {
                let title = prompt.title.clone();
                self.state.settings.custom_prompts.push(prompt);
                self.state.add_log(
                    "SUCCESS",
                    &format!("Saved custom recipe to library: {}", title),
                );
                self.state.status_message = format!("Saved recipe '{}' to library", title);
            }
            SettingUpdate::UpdateCustomPrompt(prompt) => {
                let title = prompt.title.clone();
                if let Some(pos) = self
                    .state
                    .settings
                    .custom_prompts
                    .iter()
                    .position(|p| p.id == prompt.id)
                {
                    self.state.settings.custom_prompts[pos] = prompt;
                    self.state
                        .add_log("SUCCESS", &format!("Updated custom recipe: {}", title));
                    self.state.status_message = format!("Updated recipe '{}'", title);
                } else {
                    self.state.settings.custom_prompts.push(prompt);
                    self.state.add_log(
                        "SUCCESS",
                        &format!("Saved custom recipe to library: {}", title),
                    );
                    self.state.status_message = format!("Saved recipe '{}' to library", title);
                }
            }
            SettingUpdate::DeleteCustomPrompt(id) => {
                if let Some(pos) = self
                    .state
                    .settings
                    .custom_prompts
                    .iter()
                    .position(|p| p.id == id)
                {
                    let title = self.state.settings.custom_prompts[pos].title.clone();
                    self.state.settings.custom_prompts.remove(pos);
                    self.state
                        .add_log("INFO", &format!("Deleted recipe from library: {}", title));
                    self.state.status_message = format!("Deleted recipe '{}'", title);
                }
            }
        }
        // Normalize ranges, then coalesce rapid writes into one disk flush.
        self.state.settings.validate_in_place();
        if needs_client_rebuild {
            self.rebuild_client();
        }
        self.schedule_settings_save();
    }
}

impl Render for StudioApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_view = self.state.active_view;
        let active_modal = self.state.active_modal.clone();
        let view = cx.entity().clone();
        let p = self.state.palette();

        let v_key = view.clone();
        h_flex()
            .track_focus(&self.focus_handle)
            .on_key_down(move |event: &gpui::KeyDownEvent, _window, cx: &mut App| {
                let key = &event.keystroke.key;
                let ctrl = event.keystroke.modifiers.control || event.keystroke.modifiers.platform;
                if key == "escape" {
                    v_key.update(cx, |app, cx| {
                        app.state.active_modal = ActiveModal::None;
                        cx.notify();
                    });
                } else if ctrl && key == "1" {
                    v_key.update(cx, |app, cx| {
                        app.state.active_view = ActiveView::Studio;
                        cx.notify();
                    });
                } else if ctrl && key == "2" {
                    v_key.update(cx, |app, cx| {
                        app.state.active_view = ActiveView::BackgroundRemover;
                        cx.notify();
                    });
                } else if ctrl && key == "3" {
                    v_key.update(cx, |app, cx| {
                        app.state.active_view = ActiveView::Library;
                        cx.notify();
                    });
                } else if ctrl && (key == "enter" || key == "return") {
                    v_key.update(cx, |app, cx| {
                        if app.state.active_view == ActiveView::Studio {
                            app.trigger_generation(cx);
                        }
                    });
                }
            })
            .size_full()
            .bg(rgb(p.bg_app))
            .text_color(rgb(p.text_primary))
            .relative()
            .when(self.state.settings.textured_background, |app_div| {
                let tex = crate::state::get_asset_path("textures/paper_noise.png");
                app_div.child(
                    img(tex)
                        .absolute()
                        .inset_0()
                        .size_full()
                        .opacity(if p.is_dark { 0.16 } else { 0.25 })
                )
            })
            // Left Space Navigation Rail
            .child(render_nav_rail(
                &self.state,
                cx,
                // on_select_space
                {
                    let v = view.clone();
                    move |space, _window, cx: &mut App| {
                        v.update(cx, |app, cx| {
                            app.state.active_view = space;
                            cx.notify();
                        });
                    }
                },
                // on_toggle_theme
                {
                    let v = view.clone();
                    move |_window, cx: &mut App| {
                        v.update(cx, |app, cx| {
                            let next = !app.state.settings.is_dark_mode;
                            app.apply_setting_update(SettingUpdate::DarkMode(next), cx);
                            cx.notify();
                        });
                    }
                },
                // on_open_logs
                {
                    let v = view.clone();
                    move |_window, cx: &mut App| {
                        v.update(cx, |app, cx| {
                            app.state.active_modal = ActiveModal::Logs;
                            cx.notify();
                        });
                    }
                },
                // on_open_settings
                {
                    let v = view.clone();
                    move |_window, cx: &mut App| {
                        v.update(cx, |app, cx| {
                            app.state.active_modal = ActiveModal::Settings;
                            cx.notify();
                        });
                    }
                },
            ))
            // Right Main Content Area (Header + Workspace)
            .child(
                v_flex()
                    .flex_1()
                    .h_full()
                    .overflow_hidden()
                    .child(render_header(
                        &self.state,
                        cx,
                        // on_open_help
                        {
                            let v = view.clone();
                            move |_window, cx: &mut App| {
                                let registry = crate::lora::get_lora_registry();
                                let first = registry[0].clone();
                                v.update(cx, |app, cx| {
                                    app.state.active_modal = ActiveModal::LoRAHelp(Box::new(first));
                                    cx.notify();
                                });
                            }
                        },
                        // on_open_add_recipe
                        {
                            let v = view.clone();
                            move |_window, cx: &mut App| {
                                v.update(cx, |app, cx| {
                                    app.state.editing_recipe_id = None;
                                    app.state.active_modal = ActiveModal::AddCustomRecipe;
                                    cx.notify();
                                });
                            }
                        },
                        // on_save_from_studio
                        {
                            let v = view.clone();
                            move |window, cx: &mut App| {
                                v.update(cx, |app, cx| {
                                    let cur_prompt = app.state.prompt.clone();
                                    let w = app.state.width;
                                    let h = app.state.height;
                                    let s = app.state.steps;
                                    let cfg = app.state.cfg_scale;

                                    app.state.new_recipe_width = w;
                                    app.state.new_recipe_height = h;
                                    app.state.new_recipe_steps = s;
                                    app.state.new_recipe_cfg = cfg;
                                    app.recipe_title_input.update(cx, |input, cx| {
                                        input.set_value("Studio Custom Recipe", window, cx);
                                    });
                                    app.recipe_prompt_input.update(cx, |input, cx| {
                                        input.set_value(cur_prompt, window, cx);
                                    });
                                    app.state.active_modal = ActiveModal::AddCustomRecipe;
                                    cx.notify();
                                });
                            }
                        },
                        // on_pick_rmbg_input
                        {
                            let v = view.clone();
                            move |_window, cx: &mut App| {
                                v.update(cx, |_app, cx| {
                                    cx.spawn(async move |this, cx| {
                                        if let Some(file) = rfd::AsyncFileDialog::new()
                                            .add_filter("Images (*.png, *.jpg, *.jpeg, *.webp, *.bmp)", &["png", "jpg", "jpeg", "webp", "bmp"])
                                            .set_title("Open Sprite Image for Background Removal")
                                            .pick_file()
                                            .await
                                        {
                                            let path_str = file.path().to_string_lossy().to_string();
                                            this.update(cx, |app, cx| {
                                                app.state.set_rmbg_source(Some(path_str.clone()));
                                                app.state.status_message = format!("Loaded sprite: {}", path_str);
                                                app.state.add_log("INFO", &format!("Loaded sprite image for RMBG: {}", path_str));
                                                if app.state.rmbg_engine_mode == RmbgEngineMode::ChromaKey {
                                                    app.apply_live_rmbg_tuning(cx);
                                                }
                                                cx.notify();
                                            })
                                            .ok();
                                        }
                                    })
                                    .detach();
                                });
                            }
                        },
                    ))
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .w_full()
                            .relative()
                            .overflow_hidden()
                            .child(match active_view {
                        ActiveView::Studio => h_flex()
                            .size_full()
                            // Left Control Panel Sidebar
                            .child(render_control_panel(
                                &self.state,
                                &self.prompt_input,
                                cx,
                                // Preset click
                                {
                                    let v = view.clone();
                                    move |preset_text, window, cx: &mut App| {
                                        let text = preset_text.to_string();
                                        v.update(cx, |app, cx| {
                                            app.state.prompt = text.clone();
                                            app.prompt_input.update(cx, |input, cx| {
                                                input.set_value(text.clone(), window, cx);
                                            });
                                            app.state.add_log("INFO", &format!("Applied preset: {}", text));
                                            cx.notify();
                                        });
                                    }
                                },
                                // Size click
                                {
                                    let v = view.clone();
                                    move |(w, h), _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.width = w;
                                            app.state.height = h;
                                            cx.notify();
                                        });
                                    }
                                },
                                // Steps
                                {
                                    let v = view.clone();
                                    move |steps, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.steps = steps;
                                            cx.notify();
                                        });
                                    }
                                },
                                // CFG
                                {
                                    let v = view.clone();
                                    move |cfg, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.cfg_scale = cfg;
                                            cx.notify();
                                        });
                                    }
                                },
                                // Random seed toggle
                                {
                                    let v = view.clone();
                                    move |rnd, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.random_seed = rnd;
                                            cx.notify();
                                        });
                                    }
                                },
                                // Generate trigger
                                {
                                    let v = view.clone();
                                    move |_window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.trigger_generation(cx);
                                        });
                                    }
                                },
                                // ContentRef toggle
                                {
                                    let v = view.clone();
                                    move |enabled, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.content_ref.enabled = enabled;
                                            cx.notify();
                                        });
                                    }
                                },
                                // ContentRef weight
                                {
                                    let v = view.clone();
                                    move |w, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.content_ref.weight = w;
                                            cx.notify();
                                        });
                                    }
                                },
                                // ContentRef negative baseline
                                {
                                    let v = view.clone();
                                    move |neg, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.content_ref.negative_baseline = neg;
                                            cx.notify();
                                        });
                                    }
                                },
                                // ContentRef pick image
                                {
                                    let v = view.clone();
                                    move |_window, cx: &mut App| {
                                        v.update(cx, |_app, cx| {
                                            cx.spawn(async move |this, cx| {
                                                if let Some(file) = rfd::AsyncFileDialog::new()
                                                    .add_filter("Images (*.png, *.jpg, *.jpeg, *.webp, *.bmp)", &["png", "jpg", "jpeg", "webp", "bmp"])
                                                    .set_title("Select Reference Image for ContentRef LoRA")
                                                    .pick_file()
                                                    .await
                                                {
                                                    let path_str = file.path().to_string_lossy().to_string();
                                                    this.update(cx, |app, cx| {
                                                        app.state.content_ref.ref_image_path = Some(path_str.clone());
                                                        app.state.add_log("INFO", &format!("Selected reference image: {}", path_str));
                                                        cx.notify();
                                                    })
                                                    .ok();
                                                }
                                            })
                                            .detach();
                                        });
                                    }
                                },
                                // Aesthetic toggle
                                {
                                    let v = view.clone();
                                    move |enabled, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.aesthetic.enabled = enabled;
                                            cx.notify();
                                        });
                                    }
                                },
                                // Aesthetic weight
                                {
                                    let v = view.clone();
                                    move |w, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.aesthetic.weight = w;
                                            cx.notify();
                                        });
                                    }
                                },
                                // Aesthetic negative baseline
                                {
                                    let v = view.clone();
                                    move |neg, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.aesthetic.negative_baseline = neg;
                                            cx.notify();
                                        });
                                    }
                                },
                                // Aesthetic block slice
                                {
                                    let v = view.clone();
                                    move |block, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.aesthetic.block_slice = block;
                                            cx.notify();
                                        });
                                    }
                                },
                                // Aesthetic merge
                                {
                                    let v = view.clone();
                                    move |merge, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.aesthetic.merge_type = merge;
                                            cx.notify();
                                        });
                                    }
                                },
                                // Toggle advanced architecture
                                {
                                    let v = view.clone();
                                    move |show_adv, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.show_advanced_diffsynth = show_adv;
                                            cx.notify();
                                        });
                                    }
                                },
                                // Toggle weight picker dropdown
                                {
                                    let v = view.clone();
                                    move |is_open, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.show_weight_picker = is_open;
                                            cx.notify();
                                        });
                                    }
                                },
                                // Custom LoRA toggle
                                {
                                    let v = view.clone();
                                    move |id, enabled, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.lora_enabled.insert(id.clone(), enabled);
                                            app.state.add_log(
                                                "INFO",
                                                &format!(
                                                    "Custom weight '{}' {}",
                                                    id,
                                                    if enabled { "enabled" } else { "disabled" }
                                                ),
                                            );
                                            cx.notify();
                                        });
                                    }
                                },
                                // Custom LoRA weight
                                {
                                    let v = view.clone();
                                    move |id, weight, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.lora_weights.insert(id, weight);
                                            cx.notify();
                                        });
                                    }
                                },
                                // Custom LoRA prompt trigger words injection
                                {
                                    let v = view.clone();
                                    move |triggers, window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            let current = app.state.prompt.trim();
                                            let new_prompt = if current.is_empty() {
                                                triggers.clone()
                                            } else if current.contains(&triggers) {
                                                current.to_string()
                                            } else {
                                                format!("{}, {}", current.trim_end_matches(','), triggers)
                                            };
                                            app.state.prompt = new_prompt.clone();
                                            app.prompt_input.update(cx, |input, cx| {
                                                input.set_value(new_prompt, window, cx);
                                            });
                                            app.state.add_log("INFO", &format!("Injected trigger words: {}", triggers));
                                            cx.notify();
                                        });
                                    }
                                },
                                // LoRA help modal
                                {
                                    let v = view.clone();
                                    move |info, _window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.state.active_modal = ActiveModal::LoRAHelp(Box::new(info));
                                            cx.notify();
                                        });
                                    }
                                },
                            ))
                            // Right Preview Canvas & History Reel
                            .child(render_preview_panel(
                                &self.state,
                                cx,
                                // Send to RMBG
                                {
                                    let v = view.clone();
                                    move |img_path, _window, cx: &mut App| {
                                        let path_str = img_path.to_string();
                                        v.update(cx, |app, cx| {
                                            app.state.set_rmbg_source(Some(path_str.clone()));
                                            app.state.active_view = ActiveView::BackgroundRemover;
                                            app.state.add_log("INFO", &format!("Sent image to Background Remover: {}", path_str));
                                            cx.notify();
                                        });
                                    }
                                },
                                // Open externally
                                |_img_path, _window, _cx: &mut App| {
                                    open::that(_img_path).ok();
                                },
                                // Select history item
                                {
                                    let v = view.clone();
                                    move |img_path, _window, cx: &mut App| {
                                        let path_str = img_path.to_string();
                                        v.update(cx, |app, cx| {
                                            app.state.current_image_path = Some(path_str);
                                            cx.notify();
                                        });
                                    }
                                },
                                // Cancel generation
                                {
                                    let v = view.clone();
                                    move |_window, cx: &mut App| {
                                        v.update(cx, |app, cx| {
                                            app.cancel_generation(cx);
                                        });
                                    }
                                },
                            ))
                            .into_any_element(),

                        ActiveView::BackgroundRemover => render_standalone_rmbg(
                            &self.state,
                            &self.rmbg_tol_slider,
                            &self.rmbg_halo_slider,
                            cx,
                            // Pick input
                            {
                                let v = view.clone();
                                move |_window, cx: &mut App| {
                                    v.update(cx, |_app, cx| {
                                        cx.spawn(async move |this, cx| {
                                            if let Some(file) = rfd::AsyncFileDialog::new()
                                                .add_filter("Images (*.png, *.jpg, *.jpeg, *.webp, *.bmp)", &["png", "jpg", "jpeg", "webp", "bmp"])
                                                .set_title("Open Sprite Image for Background Removal")
                                                .pick_file()
                                                .await
                                            {
                                                let path_str = file.path().to_string_lossy().to_string();
                                                this.update(cx, |app, cx| {
                                                    app.state.set_rmbg_source(Some(path_str.clone()));
                                                    app.state.status_message = format!("Loaded sprite: {}", path_str);
                                                    app.state.add_log("INFO", &format!("Loaded sprite image for RMBG: {}", path_str));
                                                    if app.state.rmbg_engine_mode == RmbgEngineMode::ChromaKey {
                                                        app.apply_live_rmbg_tuning(cx);
                                                    }
                                                    cx.notify();
                                                })
                                                .ok();
                                            }
                                        })
                                        .detach();
                                    });
                                }
                            },
                            // Select input directly (from recent history)
                            {
                                let v = view.clone();
                                move |img_path, _window, cx: &mut App| {
                                    let path_str = img_path.to_string();
                                    v.update(cx, |app, cx| {
                                        app.state.set_rmbg_source(Some(path_str.clone()));
                                        app.state.status_message = format!("Loaded recent sprite: {}", path_str);
                                        app.state.add_log("INFO", &format!("Loaded recent sprite: {}", path_str));
                                        if app.state.rmbg_engine_mode == RmbgEngineMode::ChromaKey {
                                            app.apply_live_rmbg_tuning(cx);
                                        }
                                        cx.notify();
                                    });
                                }
                            },
                            // Engine mode change
                            {
                                let v = view.clone();
                                move |mode, _window, cx: &mut App| {
                                    v.update(cx, |app, cx| {
                                        app.state.rmbg_engine_mode = mode;
                                        app.state.add_log("INFO", &format!("Switched RMBG engine to: {}", mode.name()));
                                        app.apply_live_rmbg_tuning(cx);
                                        cx.notify();
                                    });
                                }
                            },
                            // Tolerance (Live)
                            {
                                let v = view.clone();
                                move |tol, _window, cx: &mut App| {
                                    v.update(cx, |app, cx| {
                                        app.state.rmbg_tolerance = tol;
                                        app.apply_live_rmbg_tuning(cx);
                                        cx.notify();
                                    });
                                }
                            },
                            // Pixel mode (Live)
                            {
                                let v = view.clone();
                                move |pix, _window, cx: &mut App| {
                                    v.update(cx, |app, cx| {
                                        app.state.rmbg_pixel_mode = pix;
                                        app.apply_live_rmbg_tuning(cx);
                                        cx.notify();
                                    });
                                }
                            },
                            // Halo trim (Live)
                            {
                                let v = view.clone();
                                move |trim, _window, cx: &mut App| {
                                    v.update(cx, |app, cx| {
                                        app.state.rmbg_halo_trim = trim;
                                        app.apply_live_rmbg_tuning(cx);
                                        cx.notify();
                                    });
                                }
                            },
                            // Process RMBG
                            {
                                let v = view.clone();
                                move |_window, cx: &mut App| {
                                    let snapshot = {
                                        let s = &v.read(cx).state;
                                        match s.rmbg_input_path.clone() {
                                            Some(p) => Some((
                                                p,
                                                s.rmbg_engine_mode,
                                                s.rmbg_tolerance,
                                                s.rmbg_pixel_mode,
                                                s.rmbg_halo_trim,
                                            )),
                                            None => None,
                                        }
                                    };
                                    let Some((in_path, mode, tol, pix, trim)) = snapshot else {
                                        return;
                                    };
                                    v.update(cx, |app, cx| {
                                        app.state.rmbg_is_processing = true;
                                        app.state.add_log("INFO", "Processing background removal...");
                                        cx.notify();
                                    });
                                    let out_dir = v.read(cx).output_dir.clone();
                                    let out_filename = format!("rmbg_{}.png", chrono::Local::now().format("%Y%m%d_%H%M%S"));
                                    let out_path = out_dir.join(out_filename);
                                    let in_path_buf = PathBuf::from(in_path.clone());

                                    v.update(cx, |_app, cx| {
                                        cx.spawn(async move |this, cx| {
                                            let res = crate::api::TOKIO_RT
                                                .spawn_blocking(move || {
                                                    process_background_removal(
                                                        &in_path_buf, &out_path, mode, tol, pix, trim,
                                                    )
                                                })
                                                .await
                                                .unwrap_or_else(|e| {
                                                    Err(anyhow::anyhow!("RMBG worker failed: {}", e))
                                                });
                                            this.update(cx, |app, cx| {
                                                app.state.rmbg_is_processing = false;
                                                // Drop stale results if the user picked another source meanwhile.
                                                if app.state.rmbg_input_path.as_deref() != Some(in_path.as_str()) {
                                                    app.state.add_log("INFO", "Ignored stale RMBG result (source changed)");
                                                    cx.notify();
                                                    return;
                                                }
                                                match res {
                                                    Ok(r) => {
                                                        let out_str = r.output_path.to_string_lossy().to_string();
                                                        let mask_str = r.output_path.with_extension("mask.png").to_string_lossy().to_string();
                                                        app.state.rmbg_output_path = Some(out_str.clone());
                                                        app.state.rmbg_mask_path = Some(mask_str);
                                                        app.state.add_log("SUCCESS", &format!("Background removed: {} ({} px made transparent)", out_str, r.removed_pixels));
                                                        // Honor auto-generate: write sidecar JSON off-thread.
                                                        if app.state.settings.auto_generate_aseprite_json {
                                                            let png = r.output_path.clone();
                                                            let json = r.output_path.with_extension("json");
                                                            let fw = app.state.settings.aseprite_frame_width;
                                                            let fh = app.state.settings.aseprite_frame_height;
                                                            cx.spawn(async move |this, cx| {
                                                                let res = crate::api::TOKIO_RT
                                                                    .spawn_blocking(move || {
                                                                        export_aseprite_spritesheet(&png, &json, fw, fh)
                                                                    })
                                                                    .await;
                                                                let ok = matches!(&res, Ok(Ok(())));
                                                                this.update(cx, |app, cx| {
                                                                    if ok {
                                                                        app.state.add_log("SUCCESS", "Auto-generated Aseprite JSON sidecar");
                                                                    } else {
                                                                        app.state.add_log("WARN", "Auto Aseprite JSON failed (see frame size)");
                                                                    }
                                                                    cx.notify();
                                                                })
                                                                .ok();
                                                            })
                                                            .detach();
                                                        }
                                                    }
                                                    Err(e) => {
                                                        app.state.add_log("ERROR", &format!("Background removal failed: {}", e));
                                                    }
                                                }
                                                cx.notify();
                                            })
                                            .ok();
                                        })
                                        .detach();
                                    });
                                }
                            },
                            // Export PNG
                            {
                                let v = view.clone();
                                move |_window, cx: &mut App| {
                                    let live = v.read(cx).state.rmbg_output_path.clone();
                                    let Some(live_path) = live else { return };
                                    let src_path = PathBuf::from(&live_path);
                                    if !src_path.exists() {
                                        return;
                                    }
                                    let out_dir = v.read(cx).output_dir.clone();
                                    let filename = crate::api::unique_png_name("rmbg_cutout", None);
                                    let dest_path = out_dir.join(&filename);
                                    v.update(cx, |app, cx| {
                                        app.state.status_message = "Exporting cutout...".into();
                                        cx.notify();
                                    });
                                    v.update(cx, |_app, cx| {
                                        cx.spawn(async move |this, cx| {
                                            let res = crate::api::TOKIO_RT
                                                .spawn_blocking(move || std::fs::copy(&src_path, &dest_path))
                                                .await;
                                            this.update(cx, |app, cx| {
                                                match res {
                                                    Ok(Ok(_)) => {
                                                        app.state.add_log("SUCCESS", &format!("Saved clean cutout PNG to outputs/{}", filename));
                                                        app.state.status_message = format!("Saved: outputs/{}", filename);
                                                    }
                                                    _ => {
                                                        app.state.add_log("ERROR", "PNG export failed");
                                                    }
                                                }
                                                cx.notify();
                                            })
                                            .ok();
                                        })
                                        .detach();
                                    });
                                }
                            },
                            // Export Aseprite
                            {
                                let v = view.clone();
                                move |_window, cx: &mut App| {
                                    let st = v.read(cx).state.clone();
                                    let Some(live_path) = st.rmbg_output_path.clone() else { return };
                                    let src_path = PathBuf::from(&live_path);
                                    if !src_path.exists() {
                                        return;
                                    }
                                    let out_dir = v.read(cx).output_dir.clone();
                                    let base = format!(
                                        "rmbg_cutout_{}",
                                        chrono::Local::now().format("%Y%m%d_%H%M%S")
                                    );
                                    let dest_png = out_dir.join(format!("{}.png", base));
                                    let dest_json = out_dir.join(format!("{}.json", base));
                                    let fw = st.settings.aseprite_frame_width;
                                    let fh = st.settings.aseprite_frame_height;
                                    v.update(cx, |app, cx| {
                                        app.state.status_message = "Exporting Aseprite...".into();
                                        cx.notify();
                                    });
                                    v.update(cx, |_app, cx| {
                                        cx.spawn(async move |this, cx| {
                                            let res = crate::api::TOKIO_RT
                                                .spawn_blocking(move || {
                                                    std::fs::copy(&src_path, &dest_png)?;
                                                    export_aseprite_spritesheet(&dest_png, &dest_json, fw, fh)
                                                })
                                                .await;
                                            let ok = matches!(&res, Ok(Ok(())));
                                            let msg = if ok {
                                                format!("Exported Aseprite manifest: {}.json ({}x{} frames)", base, fw, fh)
                                            } else {
                                                format!("Aseprite export failed (frame {}x{} invalid?)", fw, fh)
                                            };
                                            this.update(cx, |app, cx| {
                                                if ok {
                                                    app.state.add_log("SUCCESS", &msg);
                                                    app.state.status_message = format!("Exported: outputs/{}.json", base);
                                                } else {
                                                    app.state.add_log("ERROR", &msg);
                                                    app.state.status_message = msg;
                                                }
                                                cx.notify();
                                            })
                                            .ok();
                                        })
                                        .detach();
                                    });
                                }
                            },
                        )
                        .into_any_element(),
                        ActiveView::Library => render_style_library(
                            &self.state,
                            cx,
                            // on_select_category
                            {
                                let v = view.clone();
                                move |cat, _window, cx: &mut App| {
                                    v.update(cx, |app, cx| {
                                        app.state.library_category = cat;
                                        cx.notify();
                                    });
                                }
                            },
                            // on_open_add_recipe
                            {
                                let v = view.clone();
                                move |_window, cx: &mut App| {
                                    v.update(cx, |app, cx| {
                                        app.state.editing_recipe_id = None;
                                        app.state.active_modal = ActiveModal::AddCustomRecipe;
                                        cx.notify();
                                    });
                                }
                            },
                            // on_save_from_studio
                            {
                                let v = view.clone();
                                move |window, cx: &mut App| {
                                    v.update(cx, |app, cx| {
                                        let cur_prompt = app.state.prompt.clone();
                                        let w = app.state.width;
                                        let h = app.state.height;
                                        let s = app.state.steps;
                                        let cfg = app.state.cfg_scale;

                                        app.state.new_recipe_width = w;
                                        app.state.new_recipe_height = h;
                                        app.state.new_recipe_steps = s;
                                        app.state.new_recipe_cfg = cfg;
                                        app.recipe_title_input.update(cx, |input, cx| {
                                            input.set_value("Studio Custom Recipe", window, cx);
                                        });
                                        app.recipe_prompt_input.update(cx, |input, cx| {
                                            input.set_value(cur_prompt, window, cx);
                                        });
                                        app.state.active_modal = ActiveModal::AddCustomRecipe;
                                        cx.notify();
                                    });
                                }
                            },
                            // on_load_recipe
                            {
                                let v = view.clone();
                                move |recipe, window, cx: &mut App| {
                                    v.update(cx, |app, cx| {
                                        app.state.prompt = recipe.prompt.clone();
                                        app.prompt_input.update(cx, |input, cx| {
                                            input.set_value(recipe.prompt.clone(), window, cx);
                                        });
                                        app.state.width = recipe.width;
                                        app.state.height = recipe.height;
                                        app.state.steps = recipe.steps;
                                        app.state.cfg_scale = recipe.cfg;
                                        app.state.active_view = ActiveView::Studio;
                                        app.state.add_log("INFO", &format!("Loaded library recipe: {}", recipe.title));
                                        app.state.status_message = format!("Loaded recipe '{}' into Studio", recipe.title);
                                        cx.notify();
                                    });
                                }
                            },
                            // on_copy_prompt
                            {
                                let v = view.clone();
                                move |prompt_text, _window, cx: &mut App| {
                                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(prompt_text.to_string()));
                                    v.update(cx, |app, cx| {
                                        app.state.add_log("INFO", "Copied prompt to clipboard");
                                        app.state.status_message = "Recipe prompt copied to clipboard!".to_string();
                                        cx.notify();
                                    });
                                }
                            },
                            // on_edit_recipe
                            {
                                let v = view.clone();
                                move |recipe, window, cx: &mut App| {
                                    v.update(cx, |app, cx| {
                                        app.state.editing_recipe_id = Some(recipe.id.clone());
                                        app.state.new_recipe_category = recipe.category.clone();
                                        app.state.new_recipe_width = recipe.width;
                                        app.state.new_recipe_height = recipe.height;
                                        app.state.new_recipe_steps = recipe.steps;
                                        app.state.new_recipe_cfg = recipe.cfg;
                                        let title_val = if recipe.is_custom {
                                            recipe.title.clone()
                                        } else {
                                            format!("[Custom] {}", recipe.title)
                                        };
                                        app.recipe_title_input.update(cx, |input, cx| {
                                            input.set_value(title_val, window, cx);
                                        });
                                        app.recipe_prompt_input.update(cx, |input, cx| {
                                            input.set_value(recipe.prompt.clone(), window, cx);
                                        });
                                        app.state.active_modal = ActiveModal::AddCustomRecipe;
                                        cx.notify();
                                    });
                                }
                            },
                            // on_delete_custom_recipe
                            {
                                let v = view.clone();
                                move |id, _window, cx: &mut App| {
                                    v.update(cx, |app, cx| {
                                        app.apply_setting_update(SettingUpdate::DeleteCustomPrompt(id), cx);
                                        cx.notify();
                                    });
                                }
                            },
                        )
                        .into_any_element(),
                    }),
            ))
            // Modals Overlay (LoRA Help, Logs, or Settings)
            .when(
                matches!(active_modal, ActiveModal::LoRAHelp(_)),
                |app_div| {
                    if let ActiveModal::LoRAHelp(ref info) = active_modal {
                        let v = view.clone();
                        app_div.child(render_lora_help_dialog(
                            &self.state,
                            info,
                            cx,
                            move |_window, cx: &mut App| {
                                v.update(cx, |app, cx| {
                                    app.state.active_modal = ActiveModal::None;
                                    cx.notify();
                                });
                            },
                        ))
                    } else {
                        app_div
                    }
                },
            )
            .when(
                matches!(active_modal, ActiveModal::Logs),
                |app_div| {
                    let v = view.clone();
                    let v_clear = view.clone();
                    app_div.child(render_log_dialog(
                        &self.state,
                        cx,
                        move |_window, cx: &mut App| {
                            v.update(cx, |app, cx| {
                                app.state.active_modal = ActiveModal::None;
                                cx.notify();
                            });
                        },
                        move |_window, cx: &mut App| {
                            v_clear.update(cx, |app, cx| {
                                app.state.logs.clear();
                                cx.notify();
                            });
                        },
                    ))
                },
            )
            .when(
                matches!(active_modal, ActiveModal::Settings),
                |app_div| {
                    let v_close = view.clone();
                    let v_cat = view.clone();
                    let v_up = view.clone();
                    let server_url_entity = self.server_url_input.clone();
                    let api_key_entity = self.api_key_input.clone();
                    app_div.child(render_settings_dialog(
                        &self.state,
                        &server_url_entity,
                        &api_key_entity,
                        cx,
                        move |_window, cx: &mut App| {
                            v_close.update(cx, |app, cx| {
                                app.state.active_modal = ActiveModal::None;
                                cx.notify();
                            });
                        },
                        move |cat, _window, cx: &mut App| {
                            v_cat.update(cx, |app, cx| {
                                app.state.settings_category = cat;
                                cx.notify();
                            });
                        },
                        move |update, _window, cx: &mut App| {
                            v_up.update(cx, |app, cx| {
                                if matches!(update, SettingUpdate::RefreshModels) {
                                    app.refresh_models(cx, true);
                                } else {
                                    app.apply_setting_update(update, cx);
                                }
                                cx.notify();
                            });
                        },
                    ))
                },
            )
            .when(
                matches!(active_modal, ActiveModal::AddCustomRecipe),
                |app_div| {
                    let v_close = view.clone();
                    let v_cat = view.clone();
                    let v_res = view.clone();
                    let v_st = view.clone();
                    let v_cfg = view.clone();
                    let v_paste = view.clone();
                    let v_save = view.clone();

                    app_div.child(render_add_recipe_dialog(
                        &self.state,
                        &self.recipe_title_input,
                        &self.recipe_prompt_input,
                        cx,
                        // on_close
                        move |_window, cx: &mut App| {
                            v_close.update(cx, |app, cx| {
                                app.state.active_modal = ActiveModal::None;
                                cx.notify();
                            });
                        },
                        // on_set_category
                        move |cat, _window, cx: &mut App| {
                            v_cat.update(cx, |app, cx| {
                                app.state.new_recipe_category = cat;
                                cx.notify();
                            });
                        },
                        // on_set_resolution
                        move |(w, h), _window, cx: &mut App| {
                            v_res.update(cx, |app, cx| {
                                app.state.new_recipe_width = w;
                                app.state.new_recipe_height = h;
                                cx.notify();
                            });
                        },
                        // on_set_steps
                        move |steps, _window, cx: &mut App| {
                            v_st.update(cx, |app, cx| {
                                app.state.new_recipe_steps = steps;
                                cx.notify();
                            });
                        },
                        // on_set_cfg
                        move |cfg, _window, cx: &mut App| {
                            v_cfg.update(cx, |app, cx| {
                                app.state.new_recipe_cfg = cfg;
                                cx.notify();
                            });
                        },
                        // on_paste_studio_prompt
                        move |window, cx: &mut App| {
                            v_paste.update(cx, |app, cx| {
                                let cur_prompt = app.state.prompt.clone();
                                let w = app.state.width;
                                let h = app.state.height;
                                let s = app.state.steps;
                                let cfg = app.state.cfg_scale;
                                app.state.new_recipe_width = w;
                                app.state.new_recipe_height = h;
                                app.state.new_recipe_steps = s;
                                app.state.new_recipe_cfg = cfg;
                                app.recipe_prompt_input.update(cx, |input, cx| {
                                    input.set_value(cur_prompt, window, cx);
                                });
                                cx.notify();
                            });
                        },
                        // on_save_recipe
                        move |_window, cx: &mut App| {
                            v_save.update(cx, |app, cx| {
                                let raw_title = app.recipe_title_input.read(cx).value().to_string();
                                let title = if raw_title.trim().is_empty() {
                                    "Custom Recipe".to_string()
                                } else {
                                    raw_title.trim().to_string()
                                };
                                let raw_prompt = app.recipe_prompt_input.read(cx).value().to_string();
                                let prompt = if raw_prompt.trim().is_empty() {
                                    app.state.prompt.clone()
                                } else {
                                    raw_prompt.trim().to_string()
                                };

                                let (prompt_id, is_existing) = match &app.state.editing_recipe_id {
                                    Some(id) if id.starts_with("custom_") => (id.clone(), true),
                                    _ => (format!("custom_{}", uuid_string()), false),
                                };

                                let prompt_item = CustomPrompt {
                                    id: prompt_id,
                                    title,
                                    category: app.state.new_recipe_category.clone(),
                                    prompt,
                                    width: app.state.new_recipe_width,
                                    height: app.state.new_recipe_height,
                                    steps: app.state.new_recipe_steps,
                                    cfg: app.state.new_recipe_cfg,
                                    created_at: chrono::Local::now().format("%Y-%m-%d").to_string(),
                                };

                                if is_existing {
                                    app.apply_setting_update(SettingUpdate::UpdateCustomPrompt(prompt_item), cx);
                                } else {
                                    app.apply_setting_update(SettingUpdate::AddCustomPrompt(prompt_item), cx);
                                }
                                app.state.editing_recipe_id = None;
                                app.state.active_modal = ActiveModal::None;
                                cx.notify();
                            });
                        },
                    ))
                },
            )
    }
}

fn uuid_string() -> String {
    format!(
        "{:x}",
        chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
    )
}
