use crate::components::diffsynth_controls::{
    render_custom_weight_card, render_diffsynth_aesthetic, render_diffsynth_content_ref,
};
use crate::lora::{LoRAInfo, get_lora_registry};
use crate::state::{AppState, FONT_HEADING};
use gpui::prelude::FluentBuilder as _;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::group_box::{GroupBox, GroupBoxVariants as _};
use gpui_component::input::{Textarea, TextareaState};
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::switch::Switch;
use gpui_component::tag::Tag;
use gpui_component::{Disableable as _, Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;

pub fn render_control_panel(
    state: &AppState,
    prompt_input: &Entity<TextareaState>,
    cx: &mut App,
    on_preset_click: impl Fn(&str, &mut Window, &mut App) + 'static + Clone,
    on_size_click: impl Fn((u32, u32), &mut Window, &mut App) + 'static + Clone,
    on_steps_change: impl Fn(u32, &mut Window, &mut App) + 'static + Clone,
    on_cfg_change: impl Fn(f32, &mut Window, &mut App) + 'static + Clone,
    on_random_seed_toggle: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_generate_click: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_cr_toggle: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_cr_weight: impl Fn(f32, &mut Window, &mut App) + 'static + Clone,
    on_cr_negative: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_cr_pick: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_aes_toggle: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_aes_weight: impl Fn(f32, &mut Window, &mut App) + 'static + Clone,
    on_aes_negative: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_aes_block: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
    on_aes_merge: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
    on_toggle_advanced: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_toggle_weight_picker: impl Fn(bool, &mut Window, &mut App) + 'static + Clone,
    on_lora_toggle: impl Fn(String, bool, &mut Window, &mut App) + 'static + Clone,
    on_lora_weight: impl Fn(String, f32, &mut Window, &mut App) + 'static + Clone,
    on_lora_inject_prompt: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
    on_lora_help: impl Fn(LoRAInfo, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let p = state.palette();
    let registry = get_lora_registry();
    let content_ref_info = registry
        .iter()
        .find(|l| l.id == "diffsynth_content_ref")
        .cloned()
        .unwrap();
    let aesthetic_info = registry
        .iter()
        .find(|l| l.id == "diffsynth_aesthetic")
        .cloned()
        .unwrap();

    let is_generating = state.is_generating;
    let on_gen = on_generate_click.clone();

    let btn_sz_512 = {
        let b = Button::new("sz_512").label("512²").small().flex_1();
        if state.width == 512 && state.height == 512 {
            b.primary()
        } else {
            b.secondary()
        }
    };
    let btn_sz_768 = {
        let b = Button::new("sz_768").label("768²").small().flex_1();
        if state.width == 768 && state.height == 768 {
            b.primary()
        } else {
            b.secondary()
        }
    };
    let btn_sz_1024 = {
        let b = Button::new("sz_1024").label("1024²").small().flex_1();
        if state.width == 1024 && state.height == 1024 {
            b.primary()
        } else {
            b.secondary()
        }
    };
    let btn_sz_wide = {
        let b = Button::new("sz_wide").label("1280×768").small().flex_1();
        if state.width == 1280 && state.height == 768 {
            b.primary()
        } else {
            b.secondary()
        }
    };

    v_flex()
        .w(px(396.0))
        .h_full()
        .border_r_2()
        .border_color(rgb(p.border))
        .bg(rgb(p.bg_sidebar))
        .p_4()
        .gap_3p5()
        .overflow_y_scrollbar()
        // 1. Creative Prompt Section wrapped in GroupBox
        .child(
            GroupBox::new()
                .title(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .items_center()
                        .gap_2()
                        .pr_3()
                        .child(
                            div()
                                .font_family(FONT_HEADING)
                                .font_bold()
                                .text_xs()
                                .text_color(rgb(p.text_primary))
                                .child("Prompt & Concept"),
                        )
                        .child(
                            Tag::secondary().small().flex_shrink_0().child(if state.selected_model.to_lowercase().contains("klein") {
                                "FLUX.2 Klein 4B".to_string()
                            } else {
                                state.selected_model.clone()
                            }),
                        ),
                )
                .outline()
                .child(
                    v_flex()
                        .gap_2p5()
                        .w_full()
                        // Interactive Editable Prompt Box (Multiline Textarea)
                        .child(
                            div()
                                .w_full()
                                .border_2()
                                .border_color(rgb(p.border))
                                .rounded_lg()
                                .bg(rgb(p.bg_input))
                                .p_1()
                                .child(
                                    Textarea::new(prompt_input)
                                        .h(px(96.0)),
                                ),
                        )
                        // Quick Presets Chips
                        .child(
                            v_flex()
                                .gap_1p5()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(rgb(p.text_secondary))
                                        .child("Quick Presets:"),
                                )
                                .child(
                                    h_flex()
                                        .flex_wrap()
                                        .gap_1p5()
                                        .child({
                                            let on_p = on_preset_click.clone();
                                            Button::new("preset_knight")
                                                .icon(IconName::Shield)
                                                .label("Knight")
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_p("pixel art spritesheet of a knight in silver armor, walking and attack animations, solid dark background, 16-bit retro game style", window, cx);
                                                })
                                        })
                                        .child({
                                            let on_p = on_preset_click.clone();
                                            Button::new("preset_witch")
                                                .icon(IconName::WandSparkles)
                                                .label("Witch")
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_p("pixel art spritesheet of a witch in purple robe with wooden staff, casting spell animation, solid black background", window, cx);
                                                })
                                        })
                                        .child({
                                            let on_p = on_preset_click.clone();
                                            Button::new("preset_slime")
                                                .icon(IconName::Sparkles)
                                                .label("Slime")
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_p("pixel art spritesheet of a green glowing slime monster, bouncing and attack animation, solid background", window, cx);
                                                })
                                        })
                                        .child({
                                            let on_p = on_preset_click.clone();
                                            Button::new("preset_mech")
                                                .icon(IconName::Cpu)
                                                .label("Cyber Mech")
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_p("16-bit pixel art cyberpunk bipedal combat mech with laser cannon, side view, solid navy background", window, cx);
                                                })
                                        })
                                        .child({
                                            let on_p = on_preset_click.clone();
                                            Button::new("preset_icons")
                                                .icon(IconName::Grid2x2)
                                                .label("RPG Items")
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_p("grid of RPG item icons: health potion, iron dagger, magic spell scroll, gold coin, pixel art style, solid background", window, cx);
                                                })
                                        }),
                                ),
                        ),
                ),
        )
        // 2. Generation Dimensions & Settings wrapped in GroupBox
        .child(
            GroupBox::new()
                .title(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .items_center()
                        .gap_2()
                        .pr_3()
                        .child(
                            div()
                                .font_family(FONT_HEADING)
                                .font_bold()
                                .text_xs()
                                .text_color(rgb(p.text_primary))
                                .child("Resolution & Sampling"),
                        )
                        .child(
                            Tag::primary().small().flex_shrink_0().child(format!("{}×{}", state.width, state.height)),
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
                                    let on_s = on_size_click.clone();
                                    btn_sz_512.on_click(move |_, window, cx: &mut App| {
                                        on_s((512, 512), window, cx);
                                    })
                                })
                                .child({
                                    let on_s = on_size_click.clone();
                                    btn_sz_768.on_click(move |_, window, cx: &mut App| {
                                        on_s((768, 768), window, cx);
                                    })
                                })
                                .child({
                                    let on_s = on_size_click.clone();
                                    btn_sz_1024.on_click(move |_, window, cx: &mut App| {
                                        on_s((1024, 1024), window, cx);
                                    })
                                })
                                .child({
                                    let on_s = on_size_click.clone();
                                    btn_sz_wide.on_click(move |_, window, cx: &mut App| {
                                        on_s((1280, 768), window, cx);
                                    })
                                }),
                        )
                        // Steps Stepper
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(rgb(p.text_secondary))
                                        .child("Inference Steps:"),
                                )
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .items_center()
                                        .child({
                                            let on_st_down = on_steps_change.clone();
                                            let st = state.steps;
                                            Button::new("st_down")
                                                .icon(IconName::Minus)
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_st_down(st.saturating_sub(4).max(4), window, cx);
                                                })
                                        })
                                        .child(
                                            Tag::secondary().child(format!("{}", state.steps))
                                        )
                                        .child({
                                            let on_st_up = on_steps_change.clone();
                                            let st = state.steps;
                                            Button::new("st_up")
                                                .icon(IconName::Plus)
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_st_up((st + 4).min(50), window, cx);
                                                })
                                        }),
                                ),
                        )
                        // CFG Guidance Stepper
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(rgb(p.text_secondary))
                                        .child("CFG Guidance:"),
                                )
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .items_center()
                                        .child({
                                            let on_cfg_down = on_cfg_change.clone();
                                            let cfg = state.cfg_scale;
                                            Button::new("cfg_down")
                                                .icon(IconName::Minus)
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_cfg_down((cfg - 0.5).max(1.0), window, cx);
                                                })
                                        })
                                        .child(
                                            Tag::secondary().child(format!("{:.1}", state.cfg_scale))
                                        )
                                        .child({
                                            let on_cfg_up = on_cfg_change.clone();
                                            let cfg = state.cfg_scale;
                                            Button::new("cfg_up")
                                                .icon(IconName::Plus)
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_cfg_up((cfg + 0.5).min(20.0), window, cx);
                                                })
                                        }),
                                ),
                        )
                        // Random Seed Switch
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_medium()
                                        .text_color(rgb(p.text_secondary))
                                        .child("Random Seed:"),
                                )
                                .child({
                                    let on_rand = on_random_seed_toggle.clone();
                                    Switch::new("switch_random_seed")
                                        .checked(state.random_seed)
                                        .on_click(move |checked, window, cx: &mut App| {
                                            on_rand(*checked, window, cx);
                                        })
                                }),
                        ),
                ),
        )
        // 3. DiffSynth LoRA Diffusion Templates wrapped in GroupBox
        .child(
            GroupBox::new()
                .title(
                    h_flex()
                        .w_full()
                        .justify_between()
                        .items_center()
                        .gap_2()
                        .pr_3()
                        .child(
                            div()
                                .font_family(FONT_HEADING)
                                .font_bold()
                                .text_xs()
                                .text_color(rgb(p.text_primary))
                                .child("Weights & LoRAs"),
                        )
                        .child(
                            Tag::info().small().flex_shrink_0().child("ModelScope 4B"),
                        ),
                )
                .outline()
                .child(
                    v_flex()
                        .gap_2p5()
                        .w_full()
                        // 1. Dropdown Selection Menu Button
                        .child({
                            let on_pick = on_toggle_weight_picker.clone();
                            let is_open = state.show_weight_picker;
                            let active_count = state.lora_enabled.values().filter(|&&e| e).count()
                                + if state.content_ref.enabled { 1 } else { 0 }
                                + if state.aesthetic.enabled { 1 } else { 0 };

                            Button::new("btn_toggle_weight_dropdown")
                                .icon(if is_open { IconName::ChevronUp } else { IconName::ChevronDown })
                                .label(if is_open {
                                    "Close Weights Menu".to_string()
                                } else if active_count == 0 {
                                    "Select Custom Weight / LoRA...".to_string()
                                } else {
                                    format!("+ Add Weight ({} Active)...", active_count)
                                })
                                .secondary()
                                .w_full()
                                .on_click(move |_, window, cx: &mut App| {
                                    on_pick(!is_open, window, cx);
                                })
                        })
                        // 2. Dropdown Menu List (When Open)
                        .when(state.show_weight_picker, |container| {
                            let available: Vec<_> = registry
                                .iter()
                                .filter(|l| {
                                    if l.id == "diffsynth_content_ref" {
                                        !state.content_ref.enabled
                                    } else if l.id == "diffsynth_aesthetic" {
                                        !state.aesthetic.enabled
                                    } else {
                                        !state.lora_enabled.get(&l.id).copied().unwrap_or(false)
                                    }
                                })
                                .cloned()
                                .collect();

                            let on_cr_t = on_cr_toggle.clone();
                            let on_aes_t = on_aes_toggle.clone();
                            let on_l_t = on_lora_toggle.clone();
                            let on_pick_c = on_toggle_weight_picker.clone();

                            container.child(
                                v_flex()
                                    .w_full()
                                    .p_2()
                                    .rounded_xl()
                                    .border_2()
                                    .border_color(rgb(p.accent))
                                    .bg(rgb(p.bg_card))
                                    .shadow_md()
                                    .gap_1()
                                    .child(
                                        div()
                                            .px_2()
                                            .py_1()
                                            .text_xs()
                                            .font_bold()
                                            .text_color(rgb(p.text_secondary))
                                            .child("Available Weights (Click to Activate):"),
                                    )
                                    .child(
                                        if available.is_empty() {
                                            div()
                                                .px_2()
                                                .py_1p5()
                                                .text_xs()
                                                .text_color(rgb(p.text_muted))
                                                .child("All installed weights are currently active.")
                                                .into_any_element()
                                        } else {
                                            v_flex()
                                                .gap_1()
                                                .children(available.into_iter().map(|lora| {
                                                    let id = lora.id.clone();
                                                    let on_cr = on_cr_t.clone();
                                                    let on_aes = on_aes_t.clone();
                                                    let on_lora = on_l_t.clone();
                                                    let on_close = on_pick_c.clone();

                                                    h_flex()
                                                        .w_full()
                                                        .p_2()
                                                        .rounded_lg()
                                                        .bg(rgb(p.bg_input))
                                                        .hover(|s| s.bg(rgb(p.bg_chip)))
                                                        .justify_between()
                                                        .items_center()
                                                        .gap_2()
                                                        .cursor_pointer()
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
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .text_color(rgb(p.text_muted))
                                                                        .overflow_hidden()
                                                                        .whitespace_nowrap()
                                                                        .child(lora.category.clone()),
                                                                ),
                                                        )
                                                        .child(
                                                            Tag::primary()
                                                                .small()
                                                                .flex_shrink_0()
                                                                .child("+ Activate")
                                                        )
                                                        .on_mouse_down(gpui::MouseButton::Left, move |_, window, cx: &mut App| {
                                                            if id == "diffsynth_content_ref" {
                                                                on_cr(true, window, cx);
                                                            } else if id == "diffsynth_aesthetic" {
                                                                on_aes(true, window, cx);
                                                            } else {
                                                                on_lora(id.clone(), true, window, cx);
                                                            }
                                                            on_close(false, window, cx);
                                                        })
                                                }))
                                                .into_any_element()
                                        }
                                    )
                            )
                        })
                        // 3. Active Weight Cards (Only enabled ones appear!)
                        .when(state.content_ref.enabled, |c| {
                            c.child(render_diffsynth_content_ref(
                                state,
                                cx,
                                on_cr_toggle.clone(),
                                on_cr_weight.clone(),
                                on_cr_negative.clone(),
                                on_cr_pick.clone(),
                                on_lora_help.clone(),
                                content_ref_info.clone(),
                            ))
                        })
                        .when(state.aesthetic.enabled, |c| {
                            c.child(render_diffsynth_aesthetic(
                                state,
                                cx,
                                on_aes_toggle.clone(),
                                on_aes_weight.clone(),
                                on_aes_negative.clone(),
                                on_aes_block.clone(),
                                on_aes_merge.clone(),
                                on_toggle_advanced.clone(),
                                on_lora_help.clone(),
                                aesthetic_info.clone(),
                            ))
                        })
                        .children(
                            registry
                                .into_iter()
                                .filter(|l| l.id != "diffsynth_content_ref" && l.id != "diffsynth_aesthetic")
                                .filter(|l| state.lora_enabled.get(&l.id).copied().unwrap_or(false))
                                .map(|lora| {
                                    render_custom_weight_card(
                                        state,
                                        lora,
                                        on_lora_toggle.clone(),
                                        on_lora_weight.clone(),
                                        on_lora_inject_prompt.clone(),
                                        on_lora_help.clone(),
                                    )
                                })
                        ),
                ),
        )
        // 4. Primary Generate Button
        .child(
            v_flex()
                .pt_1()
                .child(
                    Button::new("btn_generate")
                        .label(if is_generating {
                            "Generating..."
                        } else {
                            "Generate Image"
                        })
                        .icon(if is_generating {
                            IconName::RefreshCw
                        } else {
                            IconName::Sparkles
                        })
                        .primary()
                        .large()
                        .w_full()
                        .disabled(is_generating)
                        .on_click(move |_, window, cx: &mut App| {
                            on_gen(window, cx);
                        }),
                ),
        )
}
