mod api;
mod app;
mod components;
mod lora;
mod paths;
mod rmbg_engine;
mod state;

use app::StudioApp;
use gpui_component::input::{InputState, TextareaState};
use gpui_kit::*;
use state::{UserSettings, apply_theme_to_gpui};
use std::borrow::Cow;

fn main() {
    let _tokio_guard = crate::api::TOKIO_RT.enter();
    gpui_kit::application()
        .with_assets(gpui_kit_assets::AllAssets)
        .run(|cx| {
            let font_bytes: Vec<Cow<'static, [u8]>> = vec![
                Cow::Borrowed(include_bytes!("../assets/fonts/ClimateCrisis.ttf")),
                Cow::Borrowed(include_bytes!("../assets/fonts/Sora.ttf")),
            ];
            if let Err(err) = cx.text_system().add_fonts(font_bytes) {
                eprintln!("Warning: Failed to load embedded fonts: {err:?}");
            }

            gpui_kit::init(cx);
            gpui_component::init(cx);

        let settings_path = crate::paths::settings_file();
        let settings = UserSettings::load_or_default(&settings_path);
        apply_theme_to_gpui(&settings, cx);

        let bounds = Bounds::centered(None, size(px(1280.0), px(840.0)), cx);
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("Lemonade Studio — Diffusion & Pixel Art Engine (Rust/GPUI)".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            ..Default::default()
        };

        gpui_kit::open_window(window_options, cx, |window, cx| {
            let prompt_input = cx.new(|cx| {
                let mut input = TextareaState::new(window, cx);
                input.set_value(
                    "pixel art spritesheet of a knight in silver armor, walking and attack animations, solid dark background, 16-bit retro game style",
                    window,
                    cx,
                );
                input
            });
            let recipe_title_input = cx.new(|cx| {
                let mut input = InputState::new(window, cx);
                input.set_placeholder("e.g. Cyberpunk Hoverbike Spritesheet", window, cx);
                input
            });
            let recipe_prompt_input = cx.new(|cx| {
                let mut input = TextareaState::new(window, cx);
                input.set_placeholder("Enter prompt for custom recipe...", window, cx);
                input
            });
            let server_url_input = cx.new(|cx| {
                let mut input = InputState::new(window, cx);
                input.set_value(settings.server_url.clone(), window, cx);
                input.set_placeholder("http://127.0.0.1:13305", window, cx);
                input
            });
            let api_key_input = cx.new(|cx| {
                let mut input = InputState::new(window, cx);
                if let Some(k) = settings.api_key.clone() {
                    input.set_value(k, window, cx);
                }
                input.set_placeholder("Optional API key", window, cx);
                input
            });
            cx.new(|cx| {
                StudioApp::new(
                    prompt_input,
                    recipe_title_input,
                    recipe_prompt_input,
                    server_url_input,
                    api_key_input,
                    cx,
                )
            })
        })
        .expect("Failed to open Lemonade Studio window");
    });
}
