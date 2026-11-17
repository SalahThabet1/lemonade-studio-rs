use crate::lora::LoRAInfo;
use crate::state::{AppState, FONT_BODY, FONT_HEADING};
use gpui::prelude::FluentBuilder as _;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::group_box::{GroupBox, GroupBoxVariants as _};
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::tag::Tag;
use gpui_component::{Icon, Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;

pub fn render_lora_help_dialog(
    state: &AppState,
    info: &LoRAInfo,
    _cx: &mut App,
    on_close: impl Fn(&mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let p = state.palette();
    let on_close_btn = on_close.clone();
    let ms_url = info.modelscope_url.clone();
    let hf_url = info.huggingface_url.clone();

    // Modal backdrop and card
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
                .max_h(px(720.0))
                .rounded_xl()
                .border_2()
                .border_color(rgb(p.border))
                .bg(rgb(p.bg_card))
                .shadow_2xl()
                .overflow_hidden()
                // Modal Header
                .child(
                    h_flex()
                        .w_full()
                        .px_6()
                        .py_4()
                        .items_center()
                        .justify_between()
                        .border_b_2()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.bg_card_header))
                        .child(
                            h_flex()
                                .gap_3()
                                .items_center()
                                .child(
                                    div()
                                        .size(px(28.0))
                                        .rounded_lg()
                                        .bg(rgb(p.accent))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_color(rgb(p.accent_text))
                                        .child(Icon::new(IconName::BookOpen).size(px(16.0))),
                                )
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
                                                        .child(info.name.clone()),
                                                )
                                                .child(Tag::info().child(info.category.clone())),
                                        )
                                        .child(
                                            div()
                                                .font_family(FONT_BODY)
                                                .text_xs()
                                                .font_medium()
                                                .text_color(rgb(p.text_muted))
                                                .child(format!("Developed by {}", info.author)),
                                        ),
                                ),
                        )
                        .child(
                            Button::new("btn_close_lora_help")
                                .icon(IconName::Close)
                                .label("Close")
                                .small()
                                .secondary()
                                .on_click(move |_, window, cx: &mut App| {
                                    on_close_btn(window, cx);
                                }),
                        ),
                )
                // Modal Body (Scrollable)
                .child(
                    v_flex()
                        .w_full()
                        .flex_1()
                        .p_6()
                        .gap_4()
                        .bg(rgb(p.bg_card))
                        .overflow_y_scrollbar()
                        // Provenance Links in GroupBox
                        .child(
                            GroupBox::new()
                                .title("Model Provenance & Hubs")
                                .outline()
                                .child(
                                    h_flex()
                                        .gap_3()
                                        .w_full()
                                        .child(
                                            v_flex()
                                                .flex_1()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_bold()
                                                        .text_color(rgb(p.text_secondary))
                                                        .child("ModelScope:"),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(p.accent))
                                                        .truncate()
                                                        .child(ms_url),
                                                ),
                                        )
                                        .child(
                                            v_flex()
                                                .flex_1()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_bold()
                                                        .text_color(rgb(p.text_secondary))
                                                        .child("HuggingFace:"),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(p.accent))
                                                        .truncate()
                                                        .child(hf_url),
                                                ),
                                        ),
                                ),
                        )
                        // Description in GroupBox
                        .child(
                            GroupBox::new().title("Description").outline().child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(p.text_primary))
                                    .child(info.description.clone()),
                            ),
                        )
                        // Required Inputs (if any)
                        .when_some(info.required_inputs.clone(), |container, req| {
                            container.child(
                                GroupBox::new()
                                    .title(Tag::warning().child("Required Inputs"))
                                    .outline()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_medium()
                                            .text_color(rgb(p.text_primary))
                                            .child(req),
                                    ),
                            )
                        })
                        // Parameters & Options
                        .child(
                            GroupBox::new()
                                .title("Supported Parameters & Flags")
                                .outline()
                                .child(v_flex().gap_1p5().w_full().children(
                                    info.options_summary.iter().map(|opt| {
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(
                                                div()
                                                    .size(px(5.0))
                                                    .rounded_full()
                                                    .bg(rgb(p.accent)),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_medium()
                                                    .text_color(rgb(p.text_secondary))
                                                    .child(opt.clone()),
                                            )
                                    }),
                                )),
                        )
                        // Calibration & Weight Guidance
                        .child(
                            GroupBox::new()
                                .title(
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .child(
                                            div()
                                                .font_bold()
                                                .text_xs()
                                                .child("Calibration & Best Practices"),
                                        )
                                        .child(Tag::primary().child(format!(
                                            "Weight: {:.1} – {:.1}",
                                            info.recommended_weight_range.0,
                                            info.recommended_weight_range.1,
                                        ))),
                                )
                                .outline()
                                .child(
                                    v_flex()
                                        .gap_1p5()
                                        .w_full()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(p.text_primary))
                                                .child(info.usage_notes.clone()),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_bold()
                                                .text_color(rgb(p.accent))
                                                .child(format!(
                                                    "Default Weight: {:.2}",
                                                    info.default_weight
                                                )),
                                        ),
                                ),
                        )
                        // Prompt Cookbook
                        .child(
                            GroupBox::new()
                                .title("Prompt Cookbook Recipes")
                                .outline()
                                .child(v_flex().gap_2().w_full().children(
                                    info.cookbook.iter().map(|cb| {
                                        v_flex()
                                            .p_2p5()
                                            .rounded_lg()
                                            .border_1()
                                            .border_color(rgb(p.border_subtle))
                                            .bg(rgb(p.bg_input))
                                            .gap_1p5()
                                            .child(
                                                h_flex()
                                                    .justify_between()
                                                    .items_center()
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_bold()
                                                            .text_color(rgb(p.text_primary))
                                                            .child(cb.title.clone()),
                                                    )
                                                    .child(Tag::secondary().child(format!(
                                                        "Weight: {:.2}",
                                                        cb.recommended_weight
                                                    ))),
                                            )
                                            .child(
                                                div()
                                                    .p_2()
                                                    .rounded_md()
                                                    .bg(rgb(p.bg_card))
                                                    .border_1()
                                                    .border_color(rgb(p.border_subtle))
                                                    .text_xs()
                                                    .text_color(rgb(p.text_primary))
                                                    .child(cb.prompt.clone()),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_medium()
                                                    .text_color(rgb(p.text_muted))
                                                    .child(cb.notes.clone()),
                                            )
                                    }),
                                )),
                        ),
                ),
        )
}
