use crate::state::{ActiveView, AppState, FONT_BODY, FONT_HEADING};
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::tag::Tag;
use gpui_component::{Icon, Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;

pub fn render_header(
    state: &AppState,
    _cx: &mut App,
    on_open_help: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_open_add_recipe: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_save_from_studio: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_pick_rmbg_input: impl Fn(&mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let p = state.palette();
    let current_view = state.active_view;

    let (space_title, space_desc, space_tag) = match current_view {
        ActiveView::Studio => (
            "Diffusion Studio",
            "FLUX.2 Klein 4B · DiffSynth Templates",
            "FLUX.2 Klein",
        ),
        ActiveView::BackgroundRemover => (
            "Background Remover",
            "BRIA RMBG-1.4 Neural AI · 1-Bit Chroma",
            "RMBG Pipeline",
        ),
        ActiveView::Library => (
            "Style & Prompt Library",
            "16-Bit Retro Presets · Custom User Recipes",
            "Recipe Hub",
        ),
    };

    h_flex()
        .w_full()
        .h(px(52.0))
        .px_4()
        .items_center()
        .justify_between()
        .border_b_2()
        .border_color(rgb(p.border))
        .bg(rgb(p.bg_header))
        // Left Space Breadcrumb & Metadata
        .child(
            h_flex()
                .gap_3()
                .items_center()
                .child(
                    v_flex()
                        .gap_0()
                        .child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    div()
                                        .font_family(FONT_HEADING)
                                        .font_bold()
                                        .text_base()
                                        .text_color(rgb(p.text_primary))
                                        .child(space_title),
                                )
                                .child(
                                    Tag::primary().child(space_tag),
                                ),
                        )
                        .child(
                            div()
                                .font_family(FONT_BODY)
                                .text_xs()
                                .font_medium()
                                .text_color(rgb(p.text_muted))
                                .child(space_desc),
                        ),
                )
                // Server Online status pill
                .child(
                    if state.is_server_online {
                        Tag::success()
                            .child(
                                h_flex()
                                    .gap_1()
                                    .items_center()
                                    .child(Icon::new(IconName::Check).size(px(11.0)))
                                    .child("Server Online"),
                            )
                    } else {
                        Tag::danger()
                            .child(
                                h_flex()
                                    .gap_1()
                                    .items_center()
                                    .child(Icon::new(IconName::CircleAlert).size(px(11.0)))
                                    .child("Offline"),
                            )
                    }
                ),
        )
        // Right Space-Specific Contextual Action Utilities + Window Controls Cluster
        .child(
            h_flex()
                .gap_2p5()
                .items_center()
                .child(
                    match current_view {
                        ActiveView::Studio => {
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    Tag::secondary().child(format!("{}×{}", state.width, state.height)),
                                )
                                .child(
                                    Tag::secondary().child(format!("{} Steps", state.steps)),
                                )
                                .child(
                                    Tag::secondary().child(format!("CFG {:.1}", state.cfg_scale)),
                                )
                                .child(
                                    Button::new("btn_help")
                                        .icon(IconName::BookOpen)
                                        .label("Cookbook")
                                        .secondary()
                                        .small()
                                        .tooltip("LoRA Reference & Diffusion Cookbook")
                                        .on_click(move |_, window, cx: &mut App| {
                                            on_open_help(window, cx);
                                        }),
                                )
                                .into_any_element()
                        }
                        ActiveView::Library => {
                            let total_recipes = 10 + state.settings.custom_prompts.len();
                            let on_save = on_save_from_studio.clone();
                            let on_add = on_open_add_recipe.clone();
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    Tag::secondary().child(format!("{} Recipes", total_recipes)),
                                )
                                .child(
                                    Button::new("btn_header_save_studio")
                                        .icon(IconName::Sparkles)
                                        .label("Save Studio Prompt")
                                        .secondary()
                                        .small()
                                        .tooltip("Save current prompt and parameters from Studio as a new recipe")
                                        .on_click(move |_, window, cx: &mut App| {
                                            on_save(window, cx);
                                        }),
                                )
                                .child(
                                    Button::new("btn_header_add_recipe")
                                        .icon(IconName::Plus)
                                        .label("New Recipe")
                                        .primary()
                                        .small()
                                        .tooltip("Create and save a custom prompt recipe to your local library")
                                        .on_click(move |_, window, cx: &mut App| {
                                            on_add(window, cx);
                                        }),
                                )
                                .into_any_element()
                        }
                        ActiveView::BackgroundRemover => {
                            let on_pick = on_pick_rmbg_input.clone();
                            let has_input = state.rmbg_input_path.is_some();
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    Tag::secondary().child(state.rmbg_engine_mode.name()),
                                )
                                .child(
                                    Tag::secondary().child(format!("Tol {:.0}%", state.rmbg_tolerance * 100.0)),
                                )
                                .child(
                                    Tag::secondary().child(format!("Halo {}px", state.rmbg_halo_trim)),
                                )
                                .child(
                                    Button::new("btn_header_pick_rmbg")
                                        .icon(IconName::FolderOpen)
                                        .label(if has_input { "Change Source" } else { "Open Image..." })
                                        .secondary()
                                        .small()
                                        .tooltip("Open a sprite or texture from disk for background removal")
                                        .on_click(move |_, window, cx: &mut App| {
                                            on_pick(window, cx);
                                        }),
                                )
                                .into_any_element()
                        }
                    }
                )
                // Divider
                .child(
                    div()
                        .h(px(22.0))
                        .w(px(2.0))
                        .bg(rgb(p.border_subtle)),
                )
                // Window Management Controls (Minimize, Maximize / Restore, Close)
                .child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .child(
                            Button::new("btn_win_minimize")
                                .icon(IconName::Minus)
                                .ghost()
                                .small()
                                .tooltip("Minimize")
                                .on_click(|_, window, _| {
                                    window.minimize_window();
                                }),
                        )
                        .child(
                            Button::new("btn_win_maximize")
                                .icon(IconName::Square)
                                .ghost()
                                .small()
                                .tooltip("Maximize / Restore")
                                .on_click(|_, window, _| {
                                    window.zoom_window();
                                }),
                        )
                        .child(
                            Button::new("btn_win_close")
                                .icon(IconName::X)
                                .ghost()
                                .small()
                                .tooltip("Close Lemonade Studio")
                                .on_click(|_, _, cx| {
                                    cx.quit();
                                }),
                        ),
                ),
        )
}
