use crate::state::{ActiveView, AppState};
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::{Icon, Sizable as _, StyledExt as _, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;

pub fn render_nav_rail(
    state: &AppState,
    _cx: &mut App,
    on_select_space: impl Fn(ActiveView, &mut Window, &mut App) + 'static + Clone,
    on_toggle_theme: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_open_logs: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_open_settings: impl Fn(&mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let p = state.palette();
    let current_view = state.active_view;

    let spaces = [
        (
            ActiveView::Studio,
            IconName::Sparkles,
            "Studio",
            "Image Generation (FLUX.2 Klein)",
        ),
        (
            ActiveView::BackgroundRemover,
            IconName::Scissors,
            "RMBG",
            "Background Remover (BRIA AI & Chroma)",
        ),
        (
            ActiveView::Library,
            IconName::BookOpen,
            "Library",
            "Prompt & Style Library",
        ),
    ];

    v_flex()
        .w(px(64.0))
        .h_full()
        .items_center()
        .justify_between()
        .py_3p5()
        .px_2()
        .border_r_2()
        .border_color(rgb(p.border))
        .bg(rgb(p.bg_header))
        // Top section: Brand mark + Space Navigation Icons
        .child(
            v_flex()
                .items_center()
                .gap_3()
                .w_full()
                // App Logo Badge
                .child(
                    div()
                        .size(px(40.0))
                        .rounded_lg()
                        .border_2()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.accent))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(p.accent_text))
                        .child(Icon::new(IconName::Zap).size(px(20.0))),
                )
                // Divider
                .child(div().w(px(36.0)).h(px(2.0)).bg(rgb(p.border)).my_1())
                // Navigation Space Buttons
                .children(
                    spaces
                        .into_iter()
                        .map(|(space, icon, label, _tooltip_text)| {
                            let on_sel = on_select_space.clone();
                            let is_active = space == current_view;

                            let bg_color = if is_active {
                                rgb(p.accent)
                            } else {
                                rgba(0x00000000)
                            };

                            let text_color = if is_active {
                                rgb(p.accent_text)
                            } else {
                                rgb(p.text_secondary)
                            };

                            let border_color = if is_active {
                                rgb(p.border)
                            } else {
                                rgba(0x00000000)
                            };

                            div()
                                .w_full()
                                .h(px(52.0))
                                .rounded_lg()
                                .border_2()
                                .border_color(border_color)
                                .bg(bg_color)
                                .text_color(text_color)
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .gap_1()
                                .cursor_pointer()
                                .hover(|style| {
                                    if !is_active {
                                        style.bg(rgb(p.bg_chip)).text_color(rgb(p.text_primary))
                                    } else {
                                        style
                                    }
                                })
                                .child(Icon::new(icon).size(px(18.0)))
                                .child(div().text_xs().font_bold().child(label))
                                .on_mouse_down(
                                    gpui::MouseButton::Left,
                                    move |_, window, cx: &mut App| {
                                        on_sel(space, window, cx);
                                    },
                                )
                        }),
                ),
        )
        // Bottom section: Quick Theme Toggle, Logs, Settings & Server Pulse
        .child(
            v_flex()
                .items_center()
                .gap_2()
                .w_full()
                // Divider
                .child(div().w(px(36.0)).h(px(2.0)).bg(rgb(p.border)).my_1())
                // Dark / Light Quick Toggle
                .child({
                    let on_theme = on_toggle_theme.clone();
                    let icon = if state.settings.is_dark_mode {
                        IconName::Moon
                    } else {
                        IconName::Sun
                    };
                    Button::new("rail_theme_toggle")
                        .icon(icon)
                        .small()
                        .secondary()
                        .tooltip(if state.settings.is_dark_mode {
                            "Switch to Light Theme"
                        } else {
                            "Switch to Dark Theme"
                        })
                        .on_click(move |_, window, cx: &mut App| {
                            on_theme(window, cx);
                        })
                })
                // Server Logs Button
                .child({
                    let on_logs = on_open_logs.clone();
                    Button::new("rail_logs")
                        .icon(IconName::SquareTerminal)
                        .small()
                        .secondary()
                        .tooltip("System & Diffusion Logs")
                        .on_click(move |_, window, cx: &mut App| {
                            on_logs(window, cx);
                        })
                })
                // Settings Button
                .child({
                    let on_settings = on_open_settings.clone();
                    Button::new("rail_settings")
                        .icon(IconName::Settings)
                        .small()
                        .secondary()
                        .tooltip("Preferences & Styling")
                        .on_click(move |_, window, cx: &mut App| {
                            on_settings(window, cx);
                        })
                })
                // Server Online Pulse Dot
                .child(
                    div()
                        .mt_1()
                        .size(px(8.0))
                        .rounded_full()
                        .bg(if state.is_server_online {
                            rgb(0x10b981)
                        } else {
                            rgb(0xef4444)
                        }),
                ),
        )
}
