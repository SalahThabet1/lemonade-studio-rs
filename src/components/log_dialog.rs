use crate::state::{AppState, FONT_HEADING};
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::tag::Tag;
use gpui_component::{Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;

pub fn render_log_dialog(
    state: &AppState,
    _cx: &mut App,
    on_close: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_clear: impl Fn(&mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let p = state.palette();
    let on_close_btn = on_close.clone();
    let on_clear_btn = on_clear.clone();

    div()
        .absolute()
        .inset_0()
        .bg(rgba(0x000000cc))
        .flex()
        .items_center()
        .justify_center()
        .p_6()
        .child(
            v_flex()
                .w(px(720.0))
                .h(px(540.0))
                .rounded_xl()
                .border_2()
                .border_color(rgb(p.border))
                .bg(rgb(p.bg_card))
                .shadow_2xl()
                .overflow_hidden()
                // Header
                .child(
                    h_flex()
                        .w_full()
                        .px_5()
                        .py_3p5()
                        .items_center()
                        .justify_between()
                        .border_b_2()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.bg_card_header))
                        .child(
                            h_flex()
                                .gap_2p5()
                                .items_center()
                                .child(
                                    div()
                                        .font_family(FONT_HEADING)
                                        .font_bold()
                                        .text_sm()
                                        .text_color(rgb(p.text_primary))
                                        .child("System & Server Logs"),
                                )
                                .child(Tag::secondary().child(state.server_url.clone())),
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .child(
                                    Button::new("btn_clear_logs")
                                        .icon(IconName::Trash)
                                        .label("Clear")
                                        .small()
                                        .secondary()
                                        .on_click(move |_, window, cx: &mut App| {
                                            on_clear_btn(window, cx);
                                        }),
                                )
                                .child(
                                    Button::new("btn_close_logs")
                                        .icon(IconName::Close)
                                        .label("Close")
                                        .small()
                                        .secondary()
                                        .on_click(move |_, window, cx: &mut App| {
                                            on_close_btn(window, cx);
                                        }),
                                ),
                        ),
                )
                // Console output
                .child(
                    v_flex()
                        .flex_1()
                        .w_full()
                        .p_4()
                        .bg(rgb(p.bg_input))
                        .overflow_y_scrollbar()
                        .gap_1p5()
                        .children(state.logs.iter().rev().take(50).map(|log| {
                            let tag = match log.level.as_str() {
                                "ERROR" => Tag::danger().child("ERROR"),
                                "WARN" => Tag::warning().child("WARN"),
                                "SUCCESS" => Tag::success().child("SUCCESS"),
                                _ => Tag::secondary().child(log.level.clone()),
                            };

                            h_flex()
                                .gap_2()
                                .items_start()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(rgb(p.text_muted))
                                        .child(log.timestamp.clone()),
                                )
                                .child(tag)
                                .child(
                                    div()
                                        .flex_1()
                                        .text_xs()
                                        .text_color(rgb(p.text_primary))
                                        .child(log.text.clone()),
                                )
                        })),
                ),
        )
}
