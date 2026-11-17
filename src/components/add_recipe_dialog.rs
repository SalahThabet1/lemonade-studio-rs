use crate::state::{AppState, FONT_BODY, FONT_HEADING};
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::input::{Input, InputState, Textarea, TextareaState};
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::{Icon, Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;

pub fn render_add_recipe_dialog(
    state: &AppState,
    title_input: &Entity<InputState>,
    prompt_input: &Entity<TextareaState>,
    _cx: &mut App,
    on_close: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_set_category: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
    on_set_resolution: impl Fn((u32, u32), &mut Window, &mut App) + 'static + Clone,
    on_set_steps: impl Fn(u32, &mut Window, &mut App) + 'static + Clone,
    on_set_cfg: impl Fn(f32, &mut Window, &mut App) + 'static + Clone,
    on_paste_studio_prompt: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_save_recipe: impl Fn(&mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let p = state.palette();
    let current_cat = &state.new_recipe_category;
    let current_w = state.new_recipe_width;
    let current_h = state.new_recipe_height;
    let current_steps = state.new_recipe_steps;
    let current_cfg = state.new_recipe_cfg;
    let is_editing = state.editing_recipe_id.is_some();

    let categories = [
        "Characters",
        "Monsters",
        "Items & Gear",
        "Mecha & Sci-Fi",
        "Tilesets",
        "UI & Badges",
        "Custom",
    ];

    let resolutions = [
        (512, 512, "512×512 (Square)"),
        (768, 768, "768×768 (Hi-Res)"),
        (1024, 1024, "1024×1024 (HD)"),
        (1280, 768, "1280×768 (Wide)"),
        (768, 1280, "768×1280 (Portrait)"),
    ];

    let steps_options = [20, 24, 28, 32];
    let cfg_options = [3.5, 4.0, 4.5, 5.0, 7.0];

    // Backdrop
    div()
        .absolute()
        .inset_0()
        .bg(rgba(0x000000a0))
        .flex()
        .items_center()
        .justify_center()
        .p_6()
        // Modal Card
        .child(
            v_flex()
                .w(px(640.0))
                .max_h(px(720.0))
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
                        .h(px(56.0))
                        .px_6()
                        .items_center()
                        .justify_between()
                        .border_b_2()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.bg_header))
                        .child(
                            h_flex()
                                .gap_3()
                                .items_center()
                                .child(
                                    div()
                                        .size(px(32.0))
                                        .rounded_lg()
                                        .bg(rgb(p.accent))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_color(rgb(p.accent_text))
                                        .child(Icon::new(IconName::BookOpen).size(px(18.0))),
                                )
                                .child(
                                    v_flex()
                                        .gap_0()
                                        .child(
                                            div()
                                                .font_family(FONT_HEADING)
                                                .font_bold()
                                                .text_base()
                                                .text_color(rgb(p.text_primary))
                                                .child(if is_editing { "Edit Prompt Recipe" } else { "Create Custom Recipe" }),
                                        )
                                        .child(
                                            div()
                                                .font_family(FONT_BODY)
                                                .text_xs()
                                                .font_medium()
                                                .text_color(rgb(p.text_muted))
                                                .child(if is_editing {
                                                    "Update prompt, category & tuned parameters in your library"
                                                } else {
                                                    "Save prompt & tuned parameters to your local library"
                                                }),
                                        ),
                                ),
                        )
                        .child({
                            let on_c = on_close.clone();
                            Button::new("btn_close_add_recipe")
                                .icon(IconName::X)
                                .secondary()
                                .small()
                                .on_click(move |_, window, cx: &mut App| {
                                    on_c(window, cx);
                                })
                        }),
                )
                // Form Body
                .child(
                    v_flex()
                        .flex_1()
                        .w_full()
                        .p_6()
                        .gap_4()
                        .overflow_y_scrollbar()
                        // Field 1: Title
                        .child(
                            v_flex()
                                .gap_1p5()
                                .w_full()
                                .child(
                                    div()
                                        .font_semibold()
                                        .text_xs()
                                        .text_color(rgb(p.text_primary))
                                        .child("Recipe Title / Name"),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .h(px(36.0))
                                        .rounded_lg()
                                        .border_2()
                                        .border_color(rgb(p.border))
                                        .bg(rgb(p.bg_input))
                                        .px_2()
                                        .flex()
                                        .items_center()
                                        .child(Input::new(title_input).w_full()),
                                ),
                        )
                        // Field 2: Category Pills
                        .child(
                            v_flex()
                                .gap_1p5()
                                .w_full()
                                .child(
                                    div()
                                        .font_semibold()
                                        .text_xs()
                                        .text_color(rgb(p.text_primary))
                                        .child("Category"),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .flex_wrap()
                                        .children(categories.iter().map(|cat| {
                                            let on_sel = on_set_category.clone();
                                            let cat_str = cat.to_string();
                                            let is_sel = current_cat.eq_ignore_ascii_case(cat);

                                            if is_sel {
                                                Button::new(format!("btn_cat_sel_{}", cat))
                                                    .label(*cat)
                                                    .primary()
                                                    .small()
                                                    .on_click(move |_, window, cx: &mut App| {
                                                        on_sel(cat_str.clone(), window, cx);
                                                    })
                                            } else {
                                                Button::new(format!("btn_cat_sel_{}", cat))
                                                    .label(*cat)
                                                    .secondary()
                                                    .small()
                                                    .on_click(move |_, window, cx: &mut App| {
                                                        on_sel(cat_str.clone(), window, cx);
                                                    })
                                            }
                                        })),
                                ),
                        )
                        // Field 3: Prompt Textarea
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
                                                .font_semibold()
                                                .text_xs()
                                                .text_color(rgb(p.text_primary))
                                                .child("Prompt Text"),
                                        )
                                        .child({
                                            let on_paste = on_paste_studio_prompt.clone();
                                            Button::new("btn_paste_from_studio")
                                                .icon(IconName::Sparkles)
                                                .label("Copy from Current Studio")
                                                .secondary()
                                                .small()
                                                .tooltip("Load prompt and parameter settings currently active in Studio")
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_paste(window, cx);
                                                })
                                        }),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .h(px(110.0))
                                        .rounded_lg()
                                        .border_2()
                                        .border_color(rgb(p.border))
                                        .bg(rgb(p.bg_input))
                                        .p_2()
                                        .child(Textarea::new(prompt_input).size_full()),
                                ),
                        )
                        // Field 4: Recommended Resolution
                        .child(
                            v_flex()
                                .gap_1p5()
                                .w_full()
                                .child(
                                    div()
                                        .font_semibold()
                                        .text_xs()
                                        .text_color(rgb(p.text_primary))
                                        .child("Target Dimensions"),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .flex_wrap()
                                        .children(resolutions.iter().map(|(w, h, label)| {
                                            let on_res = on_set_resolution.clone();
                                            let is_sel = current_w == *w && current_h == *h;
                                            let pair = (*w, *h);

                                            if is_sel {
                                                Button::new(format!("btn_res_{}x{}", w, h))
                                                    .label(*label)
                                                    .primary()
                                                    .small()
                                                    .on_click(move |_, window, cx: &mut App| {
                                                        on_res(pair, window, cx);
                                                    })
                                            } else {
                                                Button::new(format!("btn_res_{}x{}", w, h))
                                                    .label(*label)
                                                    .secondary()
                                                    .small()
                                                    .on_click(move |_, window, cx: &mut App| {
                                                        on_res(pair, window, cx);
                                                    })
                                            }
                                        })),
                                ),
                        )
                        // Field 5: Steps & CFG Row
                        .child(
                            h_flex()
                                .gap_4()
                                .w_full()
                                // Steps
                                .child(
                                    v_flex()
                                        .flex_1()
                                        .gap_1p5()
                                        .child(
                                            div()
                                                .font_semibold()
                                                .text_xs()
                                                .text_color(rgb(p.text_primary))
                                                .child("Inference Steps"),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_1p5()
                                                .children(steps_options.iter().map(|s| {
                                                    let on_st = on_set_steps.clone();
                                                    let is_sel = current_steps == *s;
                                                    let val = *s;

                                                    if is_sel {
                                                        Button::new(format!("btn_st_{}", s))
                                                            .label(format!("{}", s))
                                                            .primary()
                                                            .small()
                                                            .flex_1()
                                                            .on_click(move |_, window, cx: &mut App| {
                                                                on_st(val, window, cx);
                                                            })
                                                    } else {
                                                        Button::new(format!("btn_st_{}", s))
                                                            .label(format!("{}", s))
                                                            .secondary()
                                                            .small()
                                                            .flex_1()
                                                            .on_click(move |_, window, cx: &mut App| {
                                                                on_st(val, window, cx);
                                                            })
                                                    }
                                                })),
                                        ),
                                )
                                // CFG
                                .child(
                                    v_flex()
                                        .flex_1()
                                        .gap_1p5()
                                        .child(
                                            div()
                                                .font_semibold()
                                                .text_xs()
                                                .text_color(rgb(p.text_primary))
                                                .child("Guidance CFG"),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_1p5()
                                                .children(cfg_options.iter().map(|c| {
                                                    let on_c = on_set_cfg.clone();
                                                    let is_sel = (current_cfg - *c).abs() < 0.05;
                                                    let val = *c;

                                                    if is_sel {
                                                        Button::new(format!("btn_cfg_{:.1}", c))
                                                            .label(format!("{:.1}", c))
                                                            .primary()
                                                            .small()
                                                            .flex_1()
                                                            .on_click(move |_, window, cx: &mut App| {
                                                                on_c(val, window, cx);
                                                            })
                                                    } else {
                                                        Button::new(format!("btn_cfg_{:.1}", c))
                                                            .label(format!("{:.1}", c))
                                                            .secondary()
                                                            .small()
                                                            .flex_1()
                                                            .on_click(move |_, window, cx: &mut App| {
                                                                on_c(val, window, cx);
                                                            })
                                                    }
                                                })),
                                        ),
                                ),
                        ),
                )
                // Footer Actions
                .child(
                    h_flex()
                        .w_full()
                        .h(px(56.0))
                        .px_6()
                        .items_center()
                        .justify_end()
                        .gap_3()
                        .border_t_2()
                        .border_color(rgb(p.border))
                        .bg(rgb(p.bg_header))
                        .child({
                            let on_c = on_close.clone();
                            Button::new("btn_cancel_add_recipe")
                                .label("Cancel")
                                .secondary()
                                .small()
                                .on_click(move |_, window, cx: &mut App| {
                                    on_c(window, cx);
                                })
                        })
                        .child({
                            let on_save = on_save_recipe.clone();
                            Button::new("btn_save_custom_recipe")
                                .icon(IconName::Check)
                                .label(if is_editing { "Save Recipe Changes" } else { "Save Recipe to Library" })
                                .primary()
                                .small()
                                .on_click(move |_, window, cx: &mut App| {
                                    on_save(window, cx);
                                })
                        }),
                ),
        )
}
