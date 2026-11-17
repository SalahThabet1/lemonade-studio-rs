use crate::state::{AppState, FONT_HEADING};
use gpui::prelude::FluentBuilder as _;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::empty::{Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};
use gpui_component::spinner::Spinner;
use gpui_component::tag::Tag;
use gpui_component::{Disableable as _, Icon, Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;
use std::path::PathBuf;

pub fn render_preview_panel(
    state: &AppState,
    _cx: &mut App,
    on_send_to_rmbg: impl Fn(&str, &mut Window, &mut App) + 'static + Clone,
    on_open_external: impl Fn(&str, &mut Window, &mut App) + 'static + Clone,
    on_select_history: impl Fn(&str, &mut Window, &mut App) + 'static + Clone,
    on_cancel: impl Fn(&mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let p = state.palette();
    let has_image = state.current_image_path.is_some();
    let current_path = state.current_image_path.clone().unwrap_or_default();
    let is_generating = state.is_generating;

    let path_for_rmbg = current_path.clone();
    let path_for_open = current_path.clone();
    let on_rmbg = on_send_to_rmbg.clone();
    let on_open = on_open_external.clone();

    v_flex()
        .flex_1()
        .w_0()
        .min_w_0()
        .h_full()
        .overflow_hidden()
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
        // Top Toolbar
        .child(
            h_flex()
                .w_full()
                .h(px(46.0))
                .px_4()
                .items_center()
                .justify_between()
                .border_b_2()
                .border_color(rgb(p.border))
                .bg(rgb(p.bg_header))
                // Left canvas viewport label & status
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            Tag::primary()
                                .child("VIEWPORT"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_medium()
                                .text_color(rgb(p.text_muted))
                                .child(state.status_message.clone()),
                        ),
                )
                // Right Action Toolbar
                .child(
                    h_flex()
                        .gap_1p5()
                        .items_center()
                        .when(is_generating, |actions| {
                            let on_c = on_cancel.clone();
                            actions.child(
                                Button::new("btn_cancel_gen")
                                    .icon(IconName::Close)
                                    .label("Cancel")
                                    .small()
                                    .secondary()
                                    .tooltip("Cancel in-flight generation")
                                    .on_click(move |_, window, cx: &mut App| {
                                        on_c(window, cx);
                                    }),
                            )
                        })
                        .when(has_image, |actions| {
                            actions.child(
                                Tag::secondary().child(
                                    if state.settings.pixel_art_crisp_zoom {
                                        "Nearest-Neighbor"
                                    } else {
                                        "Smooth Zoom"
                                    }
                                )
                            )
                        })
                        .child(
                            Button::new("btn_send_rmbg")
                                .icon(IconName::Scissors)
                                .label("Remove Background")
                                .small()
                                .secondary()
                                .disabled(!has_image || is_generating)
                                .tooltip("Send this image to Background Remover studio")
                                .on_click(move |_, window, cx: &mut App| {
                                    if !path_for_rmbg.is_empty() {
                                        on_rmbg(&path_for_rmbg, window, cx);
                                    }
                                }),
                        )
                        .child(
                            Button::new("btn_open_ext")
                                .icon(IconName::ExternalLink)
                                .label("Open Externally")
                                .small()
                                .ghost()
                                .disabled(!has_image)
                                .tooltip("Open current output in default viewer")
                                .on_click(move |_, window, cx: &mut App| {
                                    if !path_for_open.is_empty() {
                                        on_open(&path_for_open, window, cx);
                                    }
                                }),
                        ),
                ),
        )
        // Center Image Canvas Viewport
        .child(
            div()
                .flex_1()
                .w_full()
                .relative()
                .flex()
                .items_center()
                .justify_center()
                .overflow_hidden()
                .bg(rgb(p.bg_viewport))
                .child(
                    div()
                        .size_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(if let Some(ref img_path) = state.current_image_path {
                            let is_glow = state.settings.haptic_glow && is_generating;
                            let border_col = if is_glow {
                                rgb(p.accent)
                            } else {
                                rgb(p.border)
                            };
                            div()
                                .max_w_full()
                                .max_h_full()
                                .p_2p5()
                                .rounded_xl()
                                .border_2()
                                .border_color(border_col)
                                .bg(rgb(p.bg_card))
                                .shadow_lg()
                                .relative()
                                .flex()
                                .items_center()
                                .justify_center()
                                .overflow_hidden()
                                .child(
                                    img(crate::state::get_asset_path("textures/checkerboard.png"))
                                        .absolute()
                                        .inset_0()
                                        .size_full()
                                        .object_fit(gpui::ObjectFit::Cover)
                                )
                                .child(
                                    img(PathBuf::from(img_path))
                                        .relative()
                                        .max_w_full()
                                        .max_h_full()
                                        .rounded_lg(),
                                )
                                .when(state.settings.crt_scanlines, |d| {
                                    d.child(
                                        div()
                                            .absolute()
                                            .inset_0()
                                            .rounded_lg()
                                            .border_1()
                                            .border_color(rgba(0x10b98133))
                                            .bg(rgba(0x00000018)),
                                    )
                                })
                                .into_any_element()
                        } else {
                            // Official gpui-component Empty state
                            div()
                                .w(px(420.0))
                                .child(
                                    Empty::new()
                                        .header(
                                            EmptyHeader::new()
                                                .media(
                                                    EmptyMedia::new()
                                                        .child(Icon::new(IconName::Sparkles).size_8()),
                                                )
                                                .title(
                                                    EmptyTitle::new()
                                                        .child("Ready to Generate"),
                                                )
                                                .description(
                                                    EmptyDescription::new()
                                                        .child("Configure prompt and parameters on the left sidebar, then click 'Generate Image' or pick a Quick Preset to synthesize retro game sprites."),
                                                ),
                                        ),
                                )
                                .into_any_element()
                        }),
                )
                // Loading Overlay when generating
                .when(is_generating, |viewport| {
                    viewport.child(
                        div()
                            .absolute()
                            .inset_0()
                            .bg(rgba(0x000000cc))
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .gap_3()
                            .child(Spinner::new().large())
                            .child(
                                div()
                                    .text_sm()
                                    .font_bold()
                                    .text_color(rgb(0xffffff))
                                    .child("Diffusion Pipeline In Progress..."),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x9ca3af))
                                    .child("Sampling noise latents via FLUX.2 Klein 4B"),
                            ),
                    )
                }),
        )
        // Bottom History Filmstrip Reel
        .child(
            v_flex()
                .w_full()
                .h(px(124.0))
                .border_t_2()
                .border_color(rgb(p.border))
                .bg(rgb(p.bg_card))
                .p_2p5()
                .gap_1p5()
                .child(
                    h_flex()
                        .justify_between()
                        .items_center()
                        .px_1()
                        .child(
                            div()
                                .font_family(FONT_HEADING)
                                .text_xs()
                                .font_bold()
                                .text_color(rgb(p.text_secondary))
                                .child("Generation History Reel"),
                        )
                        .child(
                            Tag::secondary()
                                .child(format!("{} items", state.history.len())),
                        ),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .overflow_hidden()
                        .p_0p5()
                        .children(state.history.iter().rev().take(12).map(|entry| {
                            let entry_path = entry.image_path.clone();
                            let is_selected = state.current_image_path.as_deref() == Some(&entry.image_path);
                            let on_sel = on_select_history.clone();

                            let border_color = if is_selected {
                                rgb(p.accent)
                            } else {
                                rgb(p.border)
                            };

                            div()
                                .w(px(76.0))
                                .h(px(76.0))
                                .rounded_lg()
                                .overflow_hidden()
                                .border_2()
                                .border_color(border_color)
                                .bg(rgb(p.bg_input))
                                .cursor_pointer()
                                .hover(|style| style.border_color(rgb(p.accent)))
                                .child(img(PathBuf::from(&entry.image_path)).size_full())
                                .on_mouse_down(
                                    gpui::MouseButton::Left,
                                    move |_, _window, cx: &mut App| {
                                        on_sel(&entry_path, _window, cx);
                                    },
                                )
                        })),
                ),
        )
}
