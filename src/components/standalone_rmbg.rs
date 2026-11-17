use crate::state::{AppState, FONT_HEADING, RmbgEngineMode};
use gpui::prelude::FluentBuilder as _;
use gpui_base::slider::SliderState;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle,
};
use gpui_component::group_box::{GroupBox, GroupBoxVariants as _};
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::slider::Slider;
use gpui_component::spinner::Spinner;
use gpui_component::switch::Switch;
use gpui_component::tag::Tag;
use gpui_component::{Disableable as _, Icon, Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;
use std::path::PathBuf;

pub fn render_standalone_rmbg(
    state: &AppState,
    rmbg_tol_slider: &Entity<SliderState>,
    rmbg_halo_slider: &Entity<SliderState>,
    _cx: &mut App,
    on_pick_input: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_select_input: impl Fn(&str, &mut Window, &mut App) + 'static + Clone,
    on_mode_change: impl Fn(RmbgEngineMode, &mut Window, &mut App) + 'static + Clone,
    _on_tolerance_change: impl Fn(f32, &mut Window, &mut App) + 'static + Clone,
    on_pixel_mode_toggle: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    _on_halo_trim_change: impl Fn(u32, &mut Window, &mut App) + 'static + Clone,
    on_process_rmbg: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_export_png: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_export_aseprite: impl Fn(&mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let p = state.palette();
    let has_input = state.rmbg_input_path.is_some();
    let has_output = state.rmbg_output_path.is_some();
    let is_processing = state.rmbg_is_processing;
    let tolerance = state.rmbg_tolerance;
    let pixel_mode = state.rmbg_pixel_mode;
    let halo_trim = state.rmbg_halo_trim;

    let on_process = on_process_rmbg.clone();
    let on_exp_png = on_export_png.clone();
    let on_exp_ase = on_export_aseprite.clone();
    let on_pick = on_pick_input.clone();

    v_flex()
        .size_full()
        .relative()
        .bg(rgb(p.bg_viewport))
        .when(state.settings.textured_background, |container| {
            let tex = crate::state::get_asset_path("textures/paper_noise.png");
            container.child(
                img(tex)
                    .absolute()
                    .inset_0()
                    .size_full()
                    .opacity(if p.is_dark { 0.18 } else { 0.28 })
            )
        })
        // Main Content Area: Side Controls + Dual Canvas
        .child(
            h_flex()
                .flex_1()
                .size_full()
                // Left Controls Column
                .child(
                    v_flex()
                        .w(px(340.0))
                        .h_full()
                        .p_4()
                        .gap_3p5()
                        .border_r_2()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.bg_sidebar))
                        .overflow_y_scrollbar()
                        // 1. Engine Selection GroupBox
                        .child(
                            GroupBox::new()
                                .title(
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .font_family(FONT_HEADING)
                                                .font_bold()
                                                .text_xs()
                                                .text_color(rgb(p.text_primary))
                                                .child("Segmentation Engine"),
                                        )
                                        .child(
                                            if state.rmbg_engine_mode == RmbgEngineMode::NeuralAi {
                                                Tag::primary().child("BRIA RMBG-1.4")
                                            } else {
                                                Tag::secondary().child("Chroma Key")
                                            }
                                        ),
                                )
                                .outline()
                                .child(
                                    v_flex()
                                        .gap_2p5()
                                        .w_full()
                                        .child(
                                            h_flex()
                                                .gap_1p5()
                                                .child({
                                                    let on_m = on_mode_change.clone();
                                                    let is_ai = state.rmbg_engine_mode == RmbgEngineMode::NeuralAi;
                                                    Button::new("btn_mode_ai")
                                                        .icon(IconName::Brain)
                                                        .label("Neural AI")
                                                        .small()
                                                        .when(is_ai, |b| b.primary())
                                                        .when(!is_ai, |b| b.secondary())
                                                        .tooltip("Deep learning model — recognizes people, faces, hair, pets & complex objects against any background")
                                                        .on_click(move |_, window, cx: &mut App| {
                                                            on_m(RmbgEngineMode::NeuralAi, window, cx);
                                                        })
                                                })
                                                .child({
                                                    let on_m = on_mode_change.clone();
                                                    let is_chroma = state.rmbg_engine_mode == RmbgEngineMode::ChromaKey;
                                                    Button::new("btn_mode_chroma")
                                                        .icon(IconName::Palette)
                                                        .label("Pixel Chroma")
                                                        .small()
                                                        .when(is_chroma, |b| b.primary())
                                                        .when(!is_chroma, |b| b.secondary())
                                                        .tooltip("Corner color distance — for retro pixel art sprites on solid-color backdrops")
                                                        .on_click(move |_, window, cx: &mut App| {
                                                            on_m(RmbgEngineMode::ChromaKey, window, cx);
                                                        })
                                                }),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_medium()
                                                .text_color(rgb(p.text_muted))
                                                .child(if state.rmbg_engine_mode == RmbgEngineMode::NeuralAi {
                                                    "Recognizes people, subjects, and objects from photos or artwork."
                                                } else {
                                                    "Subtracts backdrop color matching image corners (retro sprites)."
                                                }),
                                        ),
                                ),
                        )
                        // 2. Settings GroupBox with Live Sliders
                        .child(
                            GroupBox::new()
                                .title(
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .font_family(FONT_HEADING)
                                                .font_bold()
                                                .text_xs()
                                                .text_color(rgb(p.text_primary))
                                                .child("Alpha Mask Parameters"),
                                        )
                                        .child(
                                            if pixel_mode {
                                                Tag::success().child("1-Bit Alpha")
                                            } else {
                                                Tag::secondary().child("Smooth Alpha")
                                            }
                                        ),
                                )
                                .outline()
                                .child(
                                    v_flex()
                                        .gap_3()
                                        .w_full()
                                        // 1. Alpha Tolerance Live Slider
                                        .child(
                                            v_flex()
                                                .gap_1p5()
                                                .w_full()
                                                .child(
                                                    h_flex()
                                                        .justify_between()
                                                        .items_center()
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .font_medium()
                                                                .text_color(rgb(p.text_secondary))
                                                                .child("Alpha Tolerance (Live):"),
                                                        )
                                                        .child(
                                                            Tag::secondary()
                                                                .child(format!("{:.0}%", tolerance * 100.0)),
                                                        ),
                                                )
                                                .child(
                                                    Slider::new(rmbg_tol_slider)
                                                        .w_full()
                                                ),
                                        )
                                        // 2. 1-Bit Retro Alpha Switch
                                        .child(
                                            h_flex()
                                                .justify_between()
                                                .items_center()
                                                .child(
                                                    v_flex()
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .font_medium()
                                                                .text_color(rgb(p.text_secondary))
                                                                .child("1-Bit Retro Alpha:"),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .font_medium()
                                                                .text_color(rgb(p.text_muted))
                                                                .child("Hard binary 0/255 for pixel art"),
                                                        ),
                                                )
                                                .child({
                                                    let on_pix = on_pixel_mode_toggle.clone();
                                                    Switch::new("switch_rmbg_pixel")
                                                        .checked(pixel_mode)
                                                        .on_click(move |checked, window, cx: &mut App| {
                                                            on_pix(*checked, window, cx);
                                                        })
                                                }),
                                        )
                                        // 3. Halo Clean-up Live Slider
                                        .child(
                                            v_flex()
                                                .gap_1p5()
                                                .w_full()
                                                .child(
                                                    h_flex()
                                                        .justify_between()
                                                        .items_center()
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .font_medium()
                                                                .text_color(rgb(p.text_secondary))
                                                                .child("Halo Clean-up (Live):"),
                                                        )
                                                        .child(
                                                            Tag::secondary()
                                                                .child(if halo_trim == 0 { "Off".to_string() } else { format!("{}px", halo_trim) }),
                                                        ),
                                                )
                                                .child(
                                                    Slider::new(rmbg_halo_slider)
                                                        .w_full()
                                                ),
                                        )
                                        // Execute Button inside Card
                                        .child(
                                            Button::new("btn_run_rmbg")
                                                .label(if is_processing {
                                                    "Analyzing with RMBG..."
                                                } else if has_output {
                                                    "Re-run Full Analysis"
                                                } else {
                                                    "Remove Background (RMBG)"
                                                })
                                                .icon(if is_processing {
                                                    IconName::RefreshCw
                                                } else {
                                                    IconName::Scissors
                                                })
                                                .primary()
                                                .large()
                                                .w_full()
                                                .disabled(!has_input || is_processing)
                                                .tooltip("Runs full semantic inference to build base mask. Once generated, sliders adjust output live.")
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_process(window, cx);
                                                }),
                                        ),
                                ),
                        )
                        // 3. Export Actions GroupBox
                        .child(
                            GroupBox::new()
                                .title(
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .font_family(FONT_HEADING)
                                                .font_bold()
                                                .text_xs()
                                                .text_color(rgb(p.text_primary))
                                                .child("Export & Packaging"),
                                        )
                                        .child(
                                            Tag::secondary().child("PNG · JSON")
                                        ),
                                )
                                .outline()
                                .child(
                                    v_flex()
                                        .gap_2()
                                        .w_full()
                                        .child(
                                            Button::new("btn_export_png")
                                                .label("Save Clean PNG")
                                                .icon(IconName::Download)
                                                .secondary()
                                                .w_full()
                                                .disabled(!has_output)
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_exp_png(window, cx);
                                                }),
                                        )
                                        .child(
                                            Button::new("btn_export_aseprite")
                                                .label("Export Aseprite JSON")
                                                .icon(IconName::FileCode)
                                                .secondary()
                                                .w_full()
                                                .disabled(!has_output)
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_exp_ase(window, cx);
                                                }),
                                        ),
                                ),
                        ),
                )
                // Right Dual Canvases
                .child(
                    h_flex()
                        .flex_1()
                        .w_0()
                        .min_w_0()
                        .h_full()
                        .p_4()
                        .gap_4()
                        .overflow_hidden()
                        .bg(rgb(p.bg_viewport))
                        // Original Image Viewport
                        .child(
                            v_flex()
                                .flex_1()
                                .w_0()
                                .min_w_0()
                                .h_full()
                                .rounded_2xl()
                                .border_2()
                                .border_color(rgb(p.border))
                                .bg(rgb(p.bg_card))
                                .p_3p5()
                                .gap_2()
                                .overflow_hidden()
                                .child(
                                    h_flex()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            Tag::secondary().child("ORIGINAL INPUT")
                                        ),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .w_full()
                                        .h_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .overflow_hidden()
                                        .child(if let Some(ref in_path) = state.rmbg_input_path {
                                            div()
                                                .relative()
                                                .max_w_full()
                                                .max_h_full()
                                                .rounded_xl()
                                                .border_2()
                                                .border_color(rgb(p.border))
                                                .overflow_hidden()
                                                .shadow_md()
                                                .child(
                                                    img(crate::state::get_asset_path("textures/checkerboard.png"))
                                                        .absolute()
                                                        .inset_0()
                                                        .size_full()
                                                        .object_fit(gpui::ObjectFit::Cover)
                                                )
                                                .child(
                                                    img(PathBuf::from(in_path))
                                                        .relative()
                                                        .max_w_full()
                                                        .max_h_full()
                                                )
                                                .into_any_element()
                                        } else {
                                            let on_p2 = on_pick.clone();
                                            v_flex()
                                                .gap_3()
                                                .items_center()
                                                .justify_center()
                                                .p_4()
                                                .child(
                                                    Empty::new()
                                                        .header(
                                                            EmptyHeader::new()
                                                                .media(
                                                                    EmptyMedia::new()
                                                                        .child(Icon::new(IconName::FolderOpen).size_8()),
                                                                )
                                                                .title(
                                                                    EmptyTitle::new()
                                                                        .child("No Input Sprite Loaded"),
                                                                )
                                                                .description(
                                                                    EmptyDescription::new()
                                                                        .child("Open a local sprite image or select a recent generation below"),
                                                                ),
                                                        )
                                                        .content(
                                                            EmptyContent::new()
                                                                .child(
                                                                    Button::new("btn_choose_sprite_file")
                                                                        .icon(IconName::FolderOpen)
                                                                        .label("Choose Image File...")
                                                                        .primary()
                                                                        .on_click(move |_, window, cx: &mut App| {
                                                                            on_p2(window, cx);
                                                                        }),
                                                                ),
                                                        ),
                                                )
                                                .when(!state.history.is_empty(), |dz| {
                                                    let on_sel = on_select_input.clone();
                                                    dz.child(
                                                        v_flex()
                                                            .gap_1p5()
                                                            .items_center()
                                                            .pt_2()
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_medium()
                                                                    .text_color(rgb(p.text_secondary))
                                                                    .child("Or load from recent Studio outputs:"),
                                                            )
                                                            .child(
                                                                h_flex()
                                                                    .gap_2()
                                                                    .items_center()
                                                                    .children(state.history.iter().rev().take(3).map(|hist| {
                                                                        let path = hist.image_path.clone();
                                                                        let on_s = on_sel.clone();
                                                                        div()
                                                                            .size(px(52.0))
                                                                            .rounded_lg()
                                                                            .border_1()
                                                                            .border_color(rgb(p.border))
                                                                            .overflow_hidden()
                                                                            .cursor_pointer()
                                                                            .hover(|s| s.border_color(rgb(p.accent)))
                                                                            .child(img(PathBuf::from(path.clone())).size_full())
                                                                            .on_mouse_down(gpui::MouseButton::Left, move |_, window, cx: &mut App| {
                                                                                on_s(&path, window, cx);
                                                                            })
                                                                    })),
                                                            ),
                                                    )
                                                })
                                                .into_any_element()
                                        }),
                                ),
                        )
                        // Cutout Result Viewport (with checkerboard)
                        .child(
                            v_flex()
                                .flex_1()
                                .w_0()
                                .min_w_0()
                                .h_full()
                                .rounded_2xl()
                                .border_2()
                                .border_color(rgb(p.border))
                                .bg(rgb(p.bg_card))
                                .p_3p5()
                                .gap_2()
                                .overflow_hidden()
                                .child(
                                    h_flex()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            Tag::success().child("TRANSPARENT CUTOUT (LIVE)")
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_medium()
                                                .text_color(rgb(p.text_muted))
                                                .child("Live alpha & halo applied"),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .w_full()
                                        .h_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .overflow_hidden()
                                        .child(if let Some(ref out_path) = state.rmbg_output_path {
                                            div()
                                                .relative()
                                                .max_w_full()
                                                .max_h_full()
                                                .rounded_xl()
                                                .border_2()
                                                .border_color(rgb(p.border))
                                                .overflow_hidden()
                                                .shadow_md()
                                                // Alpha transparency checkerboard background
                                                .child(
                                                    img(crate::state::get_asset_path("textures/checkerboard.png"))
                                                        .absolute()
                                                        .inset_0()
                                                        .size_full()
                                                        .object_fit(gpui::ObjectFit::Cover)
                                                )
                                                // Cutout image on top with crisp pixel art rendering
                                                .child(
                                                    img(PathBuf::from(out_path))
                                                        .relative()
                                                        .max_w_full()
                                                        .max_h_full()
                                                )
                                                .into_any_element()
                                        } else if is_processing {
                                            Spinner::new().large().into_any_element()
                                        } else {
                                            Empty::new()
                                                .header(
                                                    EmptyHeader::new()
                                                        .media(
                                                            EmptyMedia::new()
                                                                .child(Icon::new(IconName::Scissors).size_8()),
                                                        )
                                                        .title(
                                                            EmptyTitle::new()
                                                                .child("Awaiting Processing"),
                                                        )
                                                        .description(
                                                            EmptyDescription::new()
                                                                .child("Processed transparent cutout will appear here after clicking 'Remove Background'."),
                                                        ),
                                                )
                                                .into_any_element()
                                        }),
                                ),
                        ),
                ),
        )
}
