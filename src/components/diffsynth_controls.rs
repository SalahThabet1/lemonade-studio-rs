use crate::lora::LoRAInfo;
use crate::state::AppState;
use gpui::prelude::FluentBuilder as _;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::switch::Switch;
use gpui_component::tag::Tag;
use gpui_component::{Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;

pub fn render_diffsynth_content_ref(
    state: &AppState,
    _cx: &mut App,
    on_toggle: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_weight_change: impl Fn(f32, &mut Window, &mut App) + 'static + Clone,
    on_negative_toggle: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_pick_image: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_help: impl Fn(LoRAInfo, &mut Window, &mut App) + 'static + Clone,
    info: LoRAInfo,
) -> impl IntoElement {
    let p = state.palette();
    let enabled = state.content_ref.enabled;
    let weight = state.content_ref.weight;
    let negative = state.content_ref.negative_baseline;
    let ref_path = state.content_ref.ref_image_path.clone();

    let info_clone = info.clone();
    let on_help_cb = on_help.clone();

    let border_color = if enabled {
        rgb(p.accent)
    } else {
        rgb(p.border)
    };

    let bg_color = if enabled {
        rgb(p.bg_card_header)
    } else {
        rgb(p.bg_card)
    };

    v_flex()
        .w_full()
        .p_3()
        .rounded_xl()
        .border_1()
        .border_color(border_color)
        .bg(bg_color)
        .gap_2()
        // Header row
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(Switch::new("switch_content_ref").checked(enabled).on_click(
                            move |checked, window, cx: &mut App| {
                                on_toggle(*checked, window, cx);
                            },
                        ))
                        .child(
                            v_flex()
                                .gap_0()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_bold()
                                        .text_color(rgb(p.text_primary))
                                        .child("Content Reference (Klein 4B)"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(rgb(p.text_muted))
                                        .child("DiffSynth Diffusion Template"),
                                ),
                        ),
                )
                .child(
                    Button::new("btn_help_content_ref")
                        .icon(IconName::Info)
                        .ghost()
                        .small()
                        .tooltip("LoRA Details & Requirements")
                        .on_click(move |_, window, cx: &mut App| {
                            on_help_cb(info_clone.clone(), window, cx);
                        }),
                ),
        )
        // Controls when enabled
        .when(enabled, |container| {
            let on_w_down = on_weight_change.clone();
            let on_w_up = on_weight_change.clone();
            let on_neg = on_negative_toggle.clone();
            let on_pick = on_pick_image.clone();

            let has_ref = ref_path.is_some();
            let display_ref = ref_path.unwrap_or_else(|| "No reference image chosen".to_string());

            container.child(
                v_flex()
                    .gap_2()
                    .pt_2()
                    .border_t_1()
                    .border_color(rgb(p.border_subtle))
                    // Influence weight scale
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .text_xs()
                                    .font_medium()
                                    .text_color(rgb(p.text_secondary))
                                    .child("Influence Scale:"),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .items_center()
                                    .child(
                                        Button::new("btn_cr_w_down")
                                            .icon(IconName::Minus)
                                            .small()
                                            .secondary()
                                            .on_click(move |_, window, cx: &mut App| {
                                                let new_w = (weight - 0.1).max(-2.0);
                                                on_w_down(new_w, window, cx);
                                            }),
                                    )
                                    .child(Tag::secondary().child(format!("{:.2}", weight)))
                                    .child(
                                        Button::new("btn_cr_w_up")
                                            .icon(IconName::Plus)
                                            .small()
                                            .secondary()
                                            .on_click(move |_, window, cx: &mut App| {
                                                let new_w = (weight + 0.1).min(4.0);
                                                on_w_up(new_w, window, cx);
                                            }),
                                    ),
                            ),
                    )
                    // Input Reference Image selector
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_medium()
                                    .text_color(rgb(p.text_secondary))
                                    .child("Reference Image (Required):"),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .child(
                                        Button::new("btn_pick_cr_img")
                                            .icon(IconName::ImagePlus)
                                            .label(if has_ref {
                                                "Change Image"
                                            } else {
                                                "Select Reference..."
                                            })
                                            .small()
                                            .secondary()
                                            .on_click(move |_, window, cx: &mut App| {
                                                on_pick(window, cx);
                                            }),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .bg(rgb(p.bg_input))
                                            .border_1()
                                            .border_color(rgb(p.border_subtle))
                                            .text_xs()
                                            .truncate()
                                            .text_color(rgb(p.text_muted))
                                            .child(display_ref),
                                    ),
                            ),
                    )
                    // Invert / Negative baseline switch (Progressive Disclosure)
                    .when(state.show_advanced_diffsynth, |c| {
                        c.child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .p_1p5()
                                .rounded_lg()
                                .bg(rgb(p.bg_card))
                                .border_1()
                                .border_color(rgb(p.border_subtle))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(rgb(p.text_secondary))
                                        .child("Negative Reference Baseline:"),
                                )
                                .child(Switch::new("switch_cr_neg").checked(negative).on_click(
                                    move |checked, window, cx: &mut App| {
                                        on_neg(*checked, window, cx);
                                    },
                                )),
                        )
                    }),
            )
        })
}

pub fn render_diffsynth_aesthetic(
    state: &AppState,
    _cx: &mut App,
    on_toggle: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_weight_change: impl Fn(f32, &mut Window, &mut App) + 'static + Clone,
    on_negative_toggle: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_block_slice_toggle: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
    on_merge_type_toggle: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
    on_toggle_advanced: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_help: impl Fn(LoRAInfo, &mut Window, &mut App) + 'static + Clone,
    info: LoRAInfo,
) -> impl IntoElement {
    let p = state.palette();
    let enabled = state.aesthetic.enabled;
    let weight = state.aesthetic.weight;
    let negative = state.aesthetic.negative_baseline;
    let block_slice = state.aesthetic.block_slice.clone();
    let merge_type = state.aesthetic.merge_type.clone();

    let info_clone = info.clone();
    let on_help_cb = on_help.clone();

    let btn_block_all = {
        let b = Button::new("btn_block_all").label("All Blocks").small();
        if block_slice == "all" {
            b.primary()
        } else {
            b.ghost()
        }
    };

    let btn_block_odd = {
        let b = Button::new("btn_block_odd").label("1:180:2 (Odd)").small();
        if block_slice == "1:180:2" {
            b.primary()
        } else {
            b.ghost()
        }
    };

    let btn_merge_mean = {
        let b = Button::new("btn_merge_mean").label("Mean").small();
        if merge_type == "mean" {
            b.primary()
        } else {
            b.ghost()
        }
    };

    let btn_merge_sum = {
        let b = Button::new("btn_merge_sum").label("Sum").small();
        if merge_type == "sum" {
            b.primary()
        } else {
            b.ghost()
        }
    };

    let border_color = if enabled {
        rgb(p.accent)
    } else {
        rgb(p.border)
    };

    let bg_color = if enabled {
        rgb(p.bg_card_header)
    } else {
        rgb(p.bg_card)
    };

    v_flex()
        .w_full()
        .p_3()
        .rounded_xl()
        .border_1()
        .border_color(border_color)
        .bg(bg_color)
        .gap_2()
        // Header row
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(Switch::new("switch_aesthetic").checked(enabled).on_click(
                            move |checked, window, cx: &mut App| {
                                on_toggle(*checked, window, cx);
                            },
                        ))
                        .child(
                            v_flex()
                                .gap_0()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_bold()
                                        .text_color(rgb(p.text_primary))
                                        .child("Aesthetic Booster (Klein 4B)"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(rgb(p.text_muted))
                                        .child("DiffSynth Diffusion Template"),
                                ),
                        ),
                )
                .child(
                    Button::new("btn_help_aesthetic")
                        .icon(IconName::Info)
                        .ghost()
                        .small()
                        .tooltip("LoRA Details & Requirements")
                        .on_click(move |_, window, cx: &mut App| {
                            on_help_cb(info_clone.clone(), window, cx);
                        }),
                ),
        )
        // Controls when enabled
        .when(enabled, |container| {
            let on_w_down = on_weight_change.clone();
            let on_w_up = on_weight_change.clone();
            let on_neg = on_negative_toggle.clone();
            let on_block_a = on_block_slice_toggle.clone();
            let on_block_o = on_block_slice_toggle.clone();
            let on_merge_m = on_merge_type_toggle.clone();
            let on_merge_s = on_merge_type_toggle.clone();

            container.child(
                v_flex()
                    .gap_2()
                    .pt_2()
                    .border_t_1()
                    .border_color(rgb(p.border_subtle))
                    // Aesthetic weight scale
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .text_xs()
                                    .font_medium()
                                    .text_color(rgb(p.text_secondary))
                                    .child("Aesthetic Scale:"),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .items_center()
                                    .child(
                                        Button::new("btn_aes_w_down")
                                            .icon(IconName::Minus)
                                            .small()
                                            .secondary()
                                            .on_click(move |_, window, cx: &mut App| {
                                                let new_w = (weight - 0.1).max(-2.0);
                                                on_w_down(new_w, window, cx);
                                            }),
                                    )
                                    .child(Tag::secondary().child(format!("{:.2}", weight)))
                                    .child(
                                        Button::new("btn_aes_w_up")
                                            .icon(IconName::Plus)
                                            .small()
                                            .secondary()
                                            .on_click(move |_, window, cx: &mut App| {
                                                let new_w = (weight + 0.1).min(4.0);
                                                on_w_up(new_w, window, cx);
                                            }),
                                    ),
                            ),
                    )
                    // Advanced Parameters Accordion Toggle
                    .child(h_flex().justify_between().items_center().pt_1().child({
                        let on_adv = on_toggle_advanced.clone();
                        let is_adv = state.show_advanced_diffsynth;
                        Button::new("btn_toggle_adv_diffsynth")
                            .label(if is_adv {
                                "Hide Advanced Architecture ▲"
                            } else {
                                "Advanced Architecture ▼"
                            })
                            .ghost()
                            .small()
                            .on_click(move |_, window, cx: &mut App| {
                                on_adv(!is_adv, window, cx);
                            })
                    }))
                    // Advanced Parameters Box (Progressive Disclosure)
                    .when(state.show_advanced_diffsynth, |c| {
                        c.child(
                            v_flex()
                                .gap_2()
                                .p_2()
                                .rounded_lg()
                                .bg(rgb(p.bg_card))
                                .border_1()
                                .border_color(rgb(p.border_subtle))
                                // Negative baseline switch
                                .child(
                                    h_flex()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_medium()
                                                .text_color(rgb(p.text_secondary))
                                                .child("Negative Baseline:"),
                                        )
                                        .child(
                                            Switch::new("switch_aes_neg")
                                                .checked(negative)
                                                .on_click(move |checked, window, cx: &mut App| {
                                                    on_neg(*checked, window, cx);
                                                }),
                                        ),
                                )
                                // LoRA Block Slice Selection
                                .child(
                                    h_flex()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_medium()
                                                .text_color(rgb(p.text_secondary))
                                                .child("Block Selection:"),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_1()
                                                .child(btn_block_all.on_click(
                                                    move |_, window, cx: &mut App| {
                                                        on_block_a("all".into(), window, cx);
                                                    },
                                                ))
                                                .child(btn_block_odd.on_click(
                                                    move |_, window, cx: &mut App| {
                                                        on_block_o("1:180:2".into(), window, cx);
                                                    },
                                                )),
                                        ),
                                )
                                // Merge Strategy Selection
                                .child(
                                    h_flex()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_medium()
                                                .text_color(rgb(p.text_secondary))
                                                .child("Merge Strategy:"),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_1()
                                                .child(btn_merge_mean.on_click(
                                                    move |_, window, cx: &mut App| {
                                                        on_merge_m("mean".into(), window, cx);
                                                    },
                                                ))
                                                .child(btn_merge_sum.on_click(
                                                    move |_, window, cx: &mut App| {
                                                        on_merge_s("sum".into(), window, cx);
                                                    },
                                                )),
                                        ),
                                ),
                        )
                    }),
            )
        })
}

pub fn render_custom_weight_card(
    state: &AppState,
    lora: LoRAInfo,
    on_toggle: impl Fn(String, bool, &mut Window, &mut App) + 'static + Clone,
    on_weight_change: impl Fn(String, f32, &mut Window, &mut App) + 'static + Clone,
    on_inject_prompt: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
    on_help: impl Fn(LoRAInfo, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let p = state.palette();
    let id = lora.id.clone();
    let weight = *state.lora_weights.get(&id).unwrap_or(&lora.default_weight);
    let lora_info = lora.clone();

    let id_remove = id.clone();
    let on_rem = on_toggle.clone();
    let on_hlp = on_help.clone();

    let id_w_down = id.clone();
    let id_w_up = id.clone();
    let on_w_down = on_weight_change.clone();
    let on_w_up = on_weight_change.clone();
    let on_inj = on_inject_prompt.clone();
    let triggers = lora.trigger_words.join(", ");
    let has_triggers = !lora.trigger_words.is_empty();

    v_flex()
        .w_full()
        .p_3()
        .rounded_xl()
        .border_2()
        .border_color(rgb(p.accent))
        .bg(rgb(p.bg_card_header))
        .gap_2p5()
        // Header: Title & Badges on Left, Info & Remove on Right
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .gap_2()
                .child(
                    v_flex()
                        .flex_1()
                        .min_w_0()
                        .gap_0p5()
                        .child(
                            div()
                                .text_xs()
                                .font_bold()
                                .text_color(rgb(p.text_primary))
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .child(lora.name.clone()),
                        )
                        .child(Tag::secondary().small().child(lora.category.clone())),
                )
                .child(
                    h_flex()
                        .flex_shrink_0()
                        .gap_1()
                        .items_center()
                        .child(
                            Button::new(format!("btn_help_lora_{}", id))
                                .icon(IconName::Info)
                                .ghost()
                                .small()
                                .tooltip("LoRA Details & Cookbook")
                                .on_click(move |_, window, cx: &mut App| {
                                    on_hlp(lora_info.clone(), window, cx);
                                }),
                        )
                        .child(
                            Button::new(format!("btn_rem_lora_{}", id))
                                .icon(IconName::X)
                                .ghost()
                                .small()
                                .tooltip("Deactivate this weight")
                                .on_click(move |_, window, cx: &mut App| {
                                    on_rem(id_remove.clone(), false, window, cx);
                                }),
                        ),
                ),
        )
        // Weight Scale Stepper
        .child(
            h_flex()
                .justify_between()
                .items_center()
                .pt_1()
                .border_t_1()
                .border_color(rgb(p.border_subtle))
                .child(
                    div()
                        .text_xs()
                        .font_medium()
                        .text_color(rgb(p.text_secondary))
                        .child("Weight Scale:"),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .items_center()
                        .child(
                            Button::new(format!("btn_lora_w_down_{}", id))
                                .icon(IconName::Minus)
                                .small()
                                .secondary()
                                .on_click(move |_, window, cx: &mut App| {
                                    on_w_down(
                                        id_w_down.clone(),
                                        (weight - 0.05).max(0.0),
                                        window,
                                        cx,
                                    );
                                }),
                        )
                        .child(Tag::secondary().child(format!("{:.2}", weight)))
                        .child(
                            Button::new(format!("btn_lora_w_up_{}", id))
                                .icon(IconName::Plus)
                                .small()
                                .secondary()
                                .on_click(move |_, window, cx: &mut App| {
                                    on_w_up(id_w_up.clone(), (weight + 0.05).min(2.5), window, cx);
                                }),
                        ),
                ),
        )
        // Trigger Words Box with full width and right-aligned Inject button
        .when(has_triggers, |c| {
            let trig_str = triggers.clone();
            c.child(
                v_flex()
                    .gap_1p5()
                    .p_2()
                    .rounded_lg()
                    .bg(rgb(p.bg_card))
                    .border_1()
                    .border_color(rgb(p.border_subtle))
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(rgb(p.text_secondary))
                                    .child("Trigger Words:"),
                            )
                            .child(
                                Button::new(format!("btn_inject_lora_{}", id))
                                    .icon(IconName::Sparkles)
                                    .label("Inject into Prompt")
                                    .primary()
                                    .small()
                                    .tooltip("Append trigger words to the prompt textarea")
                                    .on_click(move |_, window, cx: &mut App| {
                                        on_inj(trig_str.clone(), window, cx);
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_medium()
                            .text_color(rgb(p.text_muted))
                            .child(triggers),
                    ),
            )
        })
}
