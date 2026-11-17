use crate::state::{
    AccentColorPreset, AppState, CornerRadiusPreset, FONT_BODY, FONT_HEADING, MotionLevel,
    SettingUpdate, SettingsCategory, ThemePreset, UiScale,
};
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::input::{Input, InputState};
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::switch::Switch;
use gpui_component::{ActiveTheme as _, Icon, Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;

pub fn render_settings_dialog(
    state: &AppState,
    server_url_input: &Entity<InputState>,
    api_key_input: &Entity<InputState>,
    cx: &mut App,
    on_close: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_select_category: impl Fn(SettingsCategory, &mut Window, &mut App) + 'static + Clone,
    on_update: impl Fn(SettingUpdate, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let settings = &state.settings;
    let active_cat = state.settings_category;
    let on_close_btn = on_close.clone();
    let p = state.palette();

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
                .w(px(860.0))
                .h(px(640.0))
                .rounded_xl()
                .border_2()
                .border_color(rgb(p.border))
                .bg(rgb(p.bg_card))
                .shadow_2xl()
                .overflow_hidden()
                // Top Header Bar
                .child(
                    h_flex()
                        .w_full()
                        .px_6()
                        .py_3p5()
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
                                        .size(px(30.0))
                                        .rounded_lg()
                                        .bg(rgb(p.accent))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_color(rgb(p.accent_text))
                                        .child(Icon::new(IconName::Settings).size(px(16.0))),
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
                                                .child("Settings & Preferences"),
                                        )
                                        .child(
                                            div()
                                                .font_family(FONT_BODY)
                                                .text_xs()
                                                .font_medium()
                                                .text_color(rgb(p.text_muted))
                                                .child("App Feel, Themes, Engine Defaults & Diagnostics"),
                                        ),
                                ),
                        )
                        .child(
                            Button::new("btn_close_settings")
                                .icon(IconName::Close)
                                .label("Close")
                                .secondary()
                                .small()
                                .tooltip("Close Settings")
                                .on_click(move |_, window, cx: &mut App| {
                                    on_close_btn(window, cx);
                                }),
                        ),
                )
                // Main Body: Left Sidebar + Right Settings Panel
                .child(
                    h_flex()
                        .flex_1()
                        .w_full()
                        .overflow_hidden()
                        // Left Sidebar Tabs
                        .child(render_sidebar_tabs(active_cat, cx, on_select_category))
                        // Right Content Panel
                        .child(
                            v_flex()
                                .flex_1()
                                .h_full()
                                .p_6()
                                .overflow_y_scrollbar()
                                .child(match active_cat {
                                    SettingsCategory::ThemesAndStyle => {
                                        render_themes_panel(settings, cx, on_update.clone())
                                            .into_any_element()
                                    }
                                    SettingsCategory::AppFeel => {
                                        render_feel_panel(settings, cx, on_update.clone())
                                            .into_any_element()
                                    }
                                    SettingsCategory::Server => {
                                        render_server_panel(
                                            settings,
                                            state,
                                            server_url_input,
                                            api_key_input,
                                            cx,
                                            on_update.clone(),
                                        )
                                        .into_any_element()
                                    }
                                    SettingsCategory::StudioDefaults => {
                                        render_studio_panel(settings, cx, on_update.clone())
                                            .into_any_element()
                                    }
                                    SettingsCategory::RmbgAndAseprite => {
                                        render_rmbg_panel(settings, cx, on_update.clone())
                                            .into_any_element()
                                    }
                                    SettingsCategory::Diagnostics => {
                                        render_diagnostics_panel(
                                            settings,
                                            state,
                                            cx,
                                            on_update.clone(),
                                        )
                                        .into_any_element()
                                    }
                                }),
                        ),
                )
                // Footer
                .child(
                    h_flex()
                        .w_full()
                        .px_6()
                        .py_3()
                        .items_center()
                        .justify_between()
                        .border_t_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().secondary.opacity(0.2))
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Settings automatically saved to settings.json"),
                        )
                        .child(
                            Button::new("btn_done_settings")
                                .label("Done")
                                .primary()
                                .small()
                                .on_click(move |_, window, cx: &mut App| {
                                    on_close(window, cx);
                                }),
                        ),
                ),
        )
}

fn render_sidebar_tabs(
    active: SettingsCategory,
    cx: &mut App,
    on_select: impl Fn(SettingsCategory, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let categories = [
        (
            SettingsCategory::ThemesAndStyle,
            "Themes & Styling",
            IconName::Palette,
        ),
        (
            SettingsCategory::AppFeel,
            "App Feel & Juice",
            IconName::Sparkles,
        ),
        (
            SettingsCategory::Server,
            "Server & Engine",
            IconName::Server,
        ),
        (
            SettingsCategory::StudioDefaults,
            "Studio Defaults",
            IconName::SlidersHorizontal,
        ),
        (
            SettingsCategory::RmbgAndAseprite,
            "RMBG & Aseprite",
            IconName::Scissors,
        ),
        (
            SettingsCategory::Diagnostics,
            "Diagnostics & Cache",
            IconName::HardDrive,
        ),
    ];

    v_flex()
        .w(px(220.0))
        .h_full()
        .border_r_2()
        .border_color(cx.theme().border)
        .bg(cx.theme().secondary.opacity(0.15))
        .p_3()
        .gap_1p5()
        .overflow_y_scrollbar()
        .children(categories.into_iter().map(|(cat, label, icon)| {
            let on_sel = on_select.clone();
            let is_selected = cat == active;

            let b = Button::new(format!("tab_{:?}", cat))
                .icon(icon)
                .label(label)
                .small()
                .w_full();

            let styled_btn = if is_selected {
                b.primary()
            } else {
                b.secondary()
            };

            styled_btn.on_click(move |_, window, cx: &mut App| {
                on_sel(cat, window, cx);
            })
        }))
}

// -----------------------------------------------------------------------------
// 1. THEMES & STYLING PANEL
// -----------------------------------------------------------------------------
fn render_themes_panel(
    settings: &crate::state::UserSettings,
    cx: &mut App,
    on_update: impl Fn(SettingUpdate, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let theme_presets = [
        ThemePreset::GumroadPop,
        ThemePreset::AcidCyber,
        ThemePreset::BubblegumPunk,
        ThemePreset::BauhausCobalt,
    ];

    let accents = [
        AccentColorPreset::ElectricYellow,
        AccentColorPreset::AcidLime,
        AccentColorPreset::BubblegumPink,
        AccentColorPreset::ElectricCobalt,
        AccentColorPreset::CoralPunch,
        AccentColorPreset::SafetyOrange,
    ];

    let on_up1 = on_update.clone();
    let on_up2 = on_update.clone();
    let on_up3 = on_update.clone();
    let on_up4 = on_update.clone();

    v_flex()
        .gap_6()
        // Header
        .child(
            v_flex()
                .child(
                    div()
                        .font_family(FONT_HEADING)
                        .font_bold()
                        .text_base()
                        .text_color(cx.theme().foreground)
                        .child("Themes & Visual Aesthetics"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("Customize color palettes, accent highlights, dark/light modes, and UI scaling."),
                ),
        )
        // Theme Presets Grid
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("THEME PRESETS"),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_3()
                        .children(theme_presets.into_iter().map(|preset| {
                            let on_up = on_up1.clone();
                            let is_active = settings.theme_preset == preset;
                            let bg_color = rgb(preset.bg_hex());
                            let card_color = rgb(preset.card_hex());
                            let border_color = rgb(preset.border_hex());

                            let border_c = if is_active {
                                rgb(settings.accent_color.hex())
                            } else {
                                rgb(preset.border_hex())
                            };

                            let card_bg = if is_active {
                                rgb(preset.card_hex())
                            } else {
                                rgb(preset.bg_hex())
                            };

                            div()
                                .w(px(260.0))
                                .p_3()
                                .rounded_xl()
                                .border_2()
                                .border_color(border_c)
                                .bg(card_bg)
                                .flex()
                                .flex_col()
                                .gap_2()
                                .cursor_pointer()
                                .on_mouse_down(
                                    gpui::MouseButton::Left,
                                    move |_, window, cx: &mut App| {
                                        on_up(SettingUpdate::ThemePreset(preset), window, cx);
                                    },
                                )
                                // Swatch preview bar
                                .child(
                                    h_flex()
                                        .h(px(18.0))
                                        .rounded_md()
                                        .overflow_hidden()
                                        .border_1()
                                        .border_color(border_color)
                                        .child(div().w_1_3().h_full().bg(bg_color))
                                        .child(div().w_1_3().h_full().bg(card_color))
                                        .child(
                                            div()
                                                .w_1_3()
                                                .h_full()
                                                .bg(rgb(settings.accent_color.hex())),
                                        ),
                                )
                                .child(
                                    h_flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .font_bold()
                                                .text_sm()
                                                .text_color(if settings.is_dark_mode { rgb(0xf1f5f9) } else { rgb(0x0f172a) })
                                                .child(preset.name()),
                                        )
                                        .child(if is_active {
                                            div()
                                                .px_2()
                                                .py_0p5()
                                                .rounded_full()
                                                .bg(rgb(settings.accent_color.hex()))
                                                .text_color(if settings.accent_color.hex() == 0xffeb3b || settings.accent_color.hex() == 0xccff00 { rgb(0x18181b) } else { rgb(0xffffff) })
                                                .text_xs()
                                                .font_bold()
                                                .child("Active")
                                        } else {
                                            div()
                                        }),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(if settings.is_dark_mode { rgb(0x94a3b8) } else { rgb(0x64748b) })
                                        .child(preset.description()),
                                )
                        })),
                ),
        )
        // Dark / Light Mode Switcher
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("APPEARANCE MODE"),
                )
                .child(
                    h_flex()
                        .gap_3()
                        .child({
                            let on_up = on_up2.clone();
                            let b = Button::new("btn_dark_mode")
                                .icon(IconName::Moon)
                                .label("Dark Mode")
                                .small()
                                .flex_1();
                            let styled = if settings.is_dark_mode {
                                b.primary()
                            } else {
                                b.secondary()
                            };
                            styled.on_click(move |_, window, cx: &mut App| {
                                on_up(SettingUpdate::DarkMode(true), window, cx);
                            })
                        })
                        .child({
                            let on_up = on_up2.clone();
                            let b = Button::new("btn_light_mode")
                                .icon(IconName::Sun)
                                .label("Light Mode")
                                .small()
                                .flex_1();
                            let styled = if !settings.is_dark_mode {
                                b.primary()
                            } else {
                                b.secondary()
                            };
                            styled.on_click(move |_, window, cx: &mut App| {
                                on_up(SettingUpdate::DarkMode(false), window, cx);
                            })
                        }),
                ),
        )
        // Accent Colors
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("ACCENT COLOR"),
                )
                .child(
                    h_flex()
                        .flex_wrap()
                        .gap_2p5()
                        .children(accents.into_iter().map(|acc| {
                            let on_up = on_up3.clone();
                            let is_active = settings.accent_color == acc;

                            let border_c: gpui_kit::Hsla = if is_active {
                                rgb(acc.hex()).into()
                            } else {
                                cx.theme().border
                            };

                            let bg_c: gpui_kit::Hsla = if is_active {
                                cx.theme().accent
                            } else {
                                cx.theme().secondary
                            };

                            let text_col: gpui_kit::Hsla = if is_active {
                                rgb(acc.hex()).into()
                            } else {
                                cx.theme().muted_foreground
                            };

                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .px_3()
                                .py_1p5()
                                .rounded_lg()
                                .border_2()
                                .border_color(border_c)
                                .bg(bg_c)
                                .cursor_pointer()
                                .on_mouse_down(
                                    gpui::MouseButton::Left,
                                    move |_, window, cx: &mut App| {
                                        on_up(SettingUpdate::AccentColor(acc), window, cx);
                                    },
                                )
                                .child(
                                    div()
                                        .size(px(16.0))
                                        .rounded_full()
                                        .bg(rgb(acc.hex()))
                                        .border_1()
                                        .border_color(rgba(0xffffff88)),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_bold()
                                        .text_color(text_col)
                                        .child(acc.name()),
                                )
                        })),
                ),
        )
        // Density & Corner Radius
        .child(
            h_flex()
                .gap_6()
                .child(
                    v_flex()
                        .w_1_2()
                        .gap_2()
                        .child(
                            div()
                                .font_bold()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("UI SCALE & DENSITY"),
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .child({
                                    let on_up = on_up4.clone();
                                    let b = Button::new("scale_compact").label("Compact").small().flex_1();
                                    let s = if settings.ui_scale == UiScale::Compact {
                                        b.primary()
                                    } else {
                                        b.secondary()
                                    };
                                    s.on_click(move |_, window, cx: &mut App| {
                                        on_up(SettingUpdate::UiScale(UiScale::Compact), window, cx);
                                    })
                                })
                                .child({
                                    let on_up = on_up4.clone();
                                    let b = Button::new("scale_cozy").label("Cozy").small().flex_1();
                                    let s = if settings.ui_scale == UiScale::Cozy {
                                        b.primary()
                                    } else {
                                        b.secondary()
                                    };
                                    s.on_click(move |_, window, cx: &mut App| {
                                        on_up(SettingUpdate::UiScale(UiScale::Cozy), window, cx);
                                    })
                                })
                                .child({
                                    let on_up = on_up4.clone();
                                    let b = Button::new("scale_spacious").label("Spacious").small().flex_1();
                                    let s = if settings.ui_scale == UiScale::Spacious {
                                        b.primary()
                                    } else {
                                        b.secondary()
                                    };
                                    s.on_click(move |_, window, cx: &mut App| {
                                        on_up(SettingUpdate::UiScale(UiScale::Spacious), window, cx);
                                    })
                                }),
                        ),
                )
                .child(
                    v_flex()
                        .w_1_2()
                        .gap_2()
                        .child(
                            div()
                                .font_bold()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("CORNER RADIUS STYLE"),
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .child({
                                    let on_up = on_up4.clone();
                                    let b = Button::new("radius_sharp")
                                        .label("Retro Sharp")
                                        .small()
                                        .flex_1();
                                    let s = if settings.corner_radius
                                        == CornerRadiusPreset::RetroSharp
                                    {
                                        b.primary()
                                    } else {
                                        b.secondary()
                                    };
                                    s.on_click(move |_, window, cx: &mut App| {
                                        on_up(
                                            SettingUpdate::CornerRadius(
                                                CornerRadiusPreset::RetroSharp,
                                            ),
                                            window,
                                            cx,
                                        );
                                    })
                                })
                                .child({
                                    let on_up = on_up4.clone();
                                    let b = Button::new("radius_modern").label("Modern").small().flex_1();
                                    let s = if settings.corner_radius
                                        == CornerRadiusPreset::ModernSmooth
                                    {
                                        b.primary()
                                    } else {
                                        b.secondary()
                                    };
                                    s.on_click(move |_, window, cx: &mut App| {
                                        on_up(
                                            SettingUpdate::CornerRadius(
                                                CornerRadiusPreset::ModernSmooth,
                                            ),
                                            window,
                                            cx,
                                        );
                                    })
                                })
                                .child({
                                    let on_up = on_up4.clone();
                                    let b = Button::new("radius_pill").label("Soft Pill").small().flex_1();
                                    let s = if settings.corner_radius == CornerRadiusPreset::PillSoft
                                    {
                                        b.primary()
                                    } else {
                                        b.secondary()
                                    };
                                    s.on_click(move |_, window, cx: &mut App| {
                                        on_up(
                                            SettingUpdate::CornerRadius(
                                                CornerRadiusPreset::PillSoft,
                                            ),
                                            window,
                                            cx,
                                        );
                                    })
                                }),
                        ),
                ),
        )
}

// -----------------------------------------------------------------------------
// 2. APP FEEL & FEEDBACK PANEL
// -----------------------------------------------------------------------------
fn render_feel_panel(
    settings: &crate::state::UserSettings,
    cx: &mut App,
    on_update: impl Fn(SettingUpdate, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let on_up1 = on_update.clone();
    let on_up2 = on_update.clone();
    let on_up3 = on_update.clone();
    let on_up4 = on_update.clone();
    let on_up_tex = on_update.clone();
    let on_up5 = on_update.clone();

    v_flex()
        .gap_6()
        // Header
        .child(
            v_flex()
                .child(
                    div()
                        .font_family(FONT_HEADING)
                        .font_bold()
                        .text_base()
                        .text_color(cx.theme().foreground)
                        .child("App Feel & Sensory Feedback"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("Juice, kinetic animations, audio cues, and retro visual overlays."),
                ),
        )
        // Motion Level
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("UI MOTION & ANIMATION SPEED"),
                )
                .child(
                    h_flex()
                        .gap_3()
                        .child({
                            let on_up = on_up1.clone();
                            let b = Button::new("motion_snappy")
                                .label("Snappy (Instant)")
                                .small()
                                .flex_1();
                            let s = if settings.motion_level == MotionLevel::Snappy {
                                b.primary()
                            } else {
                                b.secondary()
                            };
                            s.on_click(move |_, window, cx: &mut App| {
                                on_up(SettingUpdate::Motion(MotionLevel::Snappy), window, cx);
                            })
                        })
                        .child({
                            let on_up = on_up1.clone();
                            let b = Button::new("motion_smooth")
                                .label("Smooth (Animated)")
                                .small()
                                .flex_1();
                            let s = if settings.motion_level == MotionLevel::Smooth {
                                b.primary()
                            } else {
                                b.secondary()
                            };
                            s.on_click(move |_, window, cx: &mut App| {
                                on_up(SettingUpdate::Motion(MotionLevel::Smooth), window, cx);
                            })
                        })
                        .child({
                            let on_up = on_up1.clone();
                            let b = Button::new("motion_reduced")
                                .label("Reduced Motion")
                                .small()
                                .flex_1();
                            let s = if settings.motion_level == MotionLevel::ReducedMotion {
                                b.primary()
                            } else {
                                b.secondary()
                            };
                            s.on_click(move |_, window, cx: &mut App| {
                                on_up(
                                    SettingUpdate::Motion(MotionLevel::ReducedMotion),
                                    window,
                                    cx,
                                );
                            })
                        }),
                ),
        )
        // Game Feel & Juice Switches
        .child(
            v_flex()
                .gap_3()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("TACTILE JUICE & VISUAL HAPTICS"),
                )
                // Screen Shake
                .child(
                    h_flex()
                        .p_3()
                        .rounded_xl()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().secondary.opacity(0.25))
                        .items_center()
                        .justify_between()
                        .child(
                            v_flex()
                                .child(
                                    div()
                                        .font_bold()
                                        .text_sm()
                                        .text_color(cx.theme().foreground)
                                        .child("Tactile Screen Shake / Punch"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("Provides satisfying kinetic recoil feedback upon successful generation."),
                                ),
                        )
                        .child({
                            let on_up = on_up2.clone();
                            Switch::new("switch_shake")
                                .checked(settings.screen_shake)
                                .on_click(move |checked, window, cx: &mut App| {
                                    on_up(SettingUpdate::ScreenShake(*checked), window, cx);
                                })
                        }),
                )
                // CRT Scanlines
                .child(
                    h_flex()
                        .p_3()
                        .rounded_xl()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().secondary.opacity(0.25))
                        .items_center()
                        .justify_between()
                        .child(
                            v_flex()
                                .child(
                                    div()
                                        .font_bold()
                                        .text_sm()
                                        .text_color(cx.theme().foreground)
                                        .child("Retro CRT Scanline Overlay"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("Simulates vintage arcade monitor phosphor scanlines across pixel art canvas."),
                                ),
                        )
                        .child({
                            let on_up = on_up3.clone();
                            Switch::new("switch_crt")
                                .checked(settings.crt_scanlines)
                                .on_click(move |checked, window, cx: &mut App| {
                                    on_up(SettingUpdate::CrtScanlines(*checked), window, cx);
                                })
                        }),
                )
                // Energetic Glow
                .child(
                    h_flex()
                        .p_3()
                        .rounded_xl()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().secondary.opacity(0.25))
                        .items_center()
                        .justify_between()
                        .child(
                            v_flex()
                                .child(
                                    div()
                                        .font_bold()
                                        .text_sm()
                                        .text_color(cx.theme().foreground)
                                        .child("Active Synthesis Glow"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("Emits a pulsing neon energy aura while the AI diffusion model is computing."),
                                ),
                        )
                        .child({
                            let on_up = on_up4.clone();
                            Switch::new("switch_glow")
                                .checked(settings.haptic_glow)
                                .on_click(move |checked, window, cx: &mut App| {
                                    on_up(SettingUpdate::HapticGlow(*checked), window, cx);
                                })
                        }),
                )
                // Paper Noise Texture
                .child(
                    h_flex()
                        .p_3()
                        .rounded_xl()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().secondary.opacity(0.25))
                        .items_center()
                        .justify_between()
                        .child(
                            v_flex()
                                .child(
                                    div()
                                        .font_bold()
                                        .text_sm()
                                        .text_color(cx.theme().foreground)
                                        .child("Tactile Paper Noise Texture"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("Subtle organic risograph paper fiber noise across all workspace surfaces."),
                                ),
                        )
                        .child({
                            let on_up = on_up_tex.clone();
                            Switch::new("switch_texture")
                                .checked(settings.textured_background)
                                .on_click(move |checked, window, cx: &mut App| {
                                    on_up(SettingUpdate::TexturedBackground(*checked), window, cx);
                                })
                        }),
                ),
        )
        // Audio Cues
        .child(
            v_flex()
                .gap_3()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("AUDIO CUES & SOUND EFFECTS"),
                )
                .child(
                    h_flex()
                        .p_3()
                        .rounded_xl()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().secondary.opacity(0.25))
                        .items_center()
                        .justify_between()
                        .child(
                            v_flex()
                                .child(
                                    div()
                                        .font_bold()
                                        .text_sm()
                                        .text_color(cx.theme().foreground)
                                        .child("Generation Complete Sound"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("Plays a crisp retro chime when images or background cutouts finish."),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_3()
                                .items_center()
                                .child({
                                    let on_up = on_up5.clone();
                                    let vol = settings.sound_volume;
                                    h_flex()
                                        .gap_1()
                                        .items_center()
                                        .child(
                                            Button::new("btn_vol_down")
                                                .icon(IconName::Minus)
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    let new_vol = (vol - 0.1).max(0.1);
                                                    on_up(
                                                        SettingUpdate::SoundVolume(new_vol),
                                                        window,
                                                        cx,
                                                    );
                                                }),
                                        )
                                        .child(
                                            div()
                                                .w(px(40.0))
                                                .text_center()
                                                .text_xs()
                                                .font_bold()
                                                .text_color(cx.theme().foreground)
                                                .child(format!("{:.0}%", vol * 100.0)),
                                        )
                                        .child({
                                            let on_up = on_up5.clone();
                                            Button::new("btn_vol_up")
                                                .icon(IconName::Plus)
                                                .small()
                                                .secondary()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    let new_vol = (vol + 0.1).min(1.0);
                                                    on_up(
                                                        SettingUpdate::SoundVolume(new_vol),
                                                        window,
                                                        cx,
                                                    );
                                                })
                                        })
                                })
                                .child({
                                    let on_up = on_up5.clone();
                                    Switch::new("switch_sound")
                                        .checked(settings.sound_effects)
                                        .on_click(move |checked, window, cx: &mut App| {
                                            on_up(
                                                SettingUpdate::SoundEffects(*checked),
                                                window,
                                                cx,
                                            );
                                        })
                                }),
                        ),
                ),
        )
}

// -----------------------------------------------------------------------------
// 3. SERVER & ENGINE PANEL
// -----------------------------------------------------------------------------
fn render_server_panel(
    settings: &crate::state::UserSettings,
    state: &AppState,
    server_url_input: &Entity<InputState>,
    api_key_input: &Entity<InputState>,
    cx: &mut App,
    on_update: impl Fn(SettingUpdate, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let on_up1 = on_update.clone();
    let on_up2 = on_update.clone();
    let on_up3 = on_update.clone();
    let on_up4 = on_update.clone();
    let on_up5 = on_update.clone();
    let url_entity = server_url_input.clone();
    let key_entity = api_key_input.clone();
    let key_entity2 = api_key_input.clone();

    v_flex()
        .gap_6()
        .child(
            v_flex()
                .child(
                    div()
                        .font_family(FONT_HEADING)
                        .font_bold()
                        .text_base()
                        .text_color(cx.theme().foreground)
                        .child("Server & Engine Connection"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child(
                            "Configure the local Lemonade API HTTP service and network heartbeat.",
                        ),
                ),
        )
        // Endpoint Status
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("LEMONADE API ENDPOINT"),
                )
                .child(
                    h_flex()
                        .p_3()
                        .rounded_xl()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().secondary.opacity(0.3))
                        .items_center()
                        .justify_between()
                        .child(
                            h_flex()
                                .gap_3()
                                .items_center()
                                .child(div().size(px(10.0)).rounded_full().bg(
                                    if state.is_server_online {
                                        rgb(0x10b981)
                                    } else {
                                        rgb(0xef4444)
                                    },
                                ))
                                .child(
                                    div()
                                        .font_bold()
                                        .text_sm()
                                        .text_color(cx.theme().foreground)
                                        .child(settings.server_url.clone()),
                                ),
                        )
                        .child(
                            div()
                                .px_2p5()
                                .py_1()
                                .rounded_full()
                                .bg(if state.is_server_online {
                                    rgba(0x10b98122)
                                } else {
                                    rgba(0xef444422)
                                })
                                .border_1()
                                .border_color(if state.is_server_online {
                                    rgba(0x10b98166)
                                } else {
                                    rgba(0xef444466)
                                })
                                .child(
                                    div()
                                        .text_xs()
                                        .font_bold()
                                        .text_color(if state.is_server_online {
                                            rgb(0x10b981)
                                        } else {
                                            rgb(0xef4444)
                                        })
                                        .child(if state.is_server_online {
                                            "Connected (Ready)".to_string()
                                        } else if let Some(err) = state.server_error.clone() {
                                            err.chars().take(80).collect::<String>()
                                        } else {
                                            "Offline (check server)".to_string()
                                        }),
                                ),
                        ),
                ),
        )
        // Endpoint editor
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("EDIT ENDPOINT"),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .flex_1()
                                .border_1()
                                .border_color(cx.theme().border)
                                .rounded_lg()
                                .bg(cx.theme().secondary.opacity(0.25))
                                .p_1()
                                .child(Input::new(&url_entity).w_full()),
                        )
                        .child({
                            let on_up = on_up3.clone();
                            let ent = url_entity.clone();
                            Button::new("apply_endpoint")
                                .label("Apply")
                                .small()
                                .primary()
                                .on_click(move |_, window, cx: &mut App| {
                                    let val = ent.read(cx).value().to_string();
                                    on_up(SettingUpdate::ServerUrl(val), window, cx);
                                })
                        }),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("Example: http://127.0.0.1:13305 — no trailing /v1."),
                ),
        )
        // API key (optional)
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("API KEY (OPTIONAL)"),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            div()
                                .flex_1()
                                .border_1()
                                .border_color(cx.theme().border)
                                .rounded_lg()
                                .bg(cx.theme().secondary.opacity(0.25))
                                .p_1()
                                .child(Input::new(&key_entity).w_full()),
                        )
                        .child({
                            let on_up = on_up4.clone();
                            let ent = key_entity2.clone();
                            Button::new("apply_apikey")
                                .label("Set")
                                .small()
                                .secondary()
                                .on_click(move |_, window, cx: &mut App| {
                                    let val = ent.read(cx).value().to_string();
                                    let opt = if val.trim().is_empty() {
                                        None
                                    } else {
                                        Some(val.trim().to_string())
                                    };
                                    on_up(SettingUpdate::ApiKey(opt), window, cx);
                                })
                        }),
                ),
        )
        // Model picker
        .child(
            v_flex()
                .gap_2()
                .child(
                    h_flex()
                        .justify_between()
                        .items_center()
                        .child(
                            div()
                                .font_bold()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("IMAGE MODEL"),
                        )
                        .child({
                            let on_up = on_up5.clone();
                            Button::new("refresh_models")
                                .label(if state.models_refreshing {
                                    "Refreshing..."
                                } else {
                                    "Refresh"
                                })
                                .small()
                                .ghost()
                                .on_click(move |_, window, cx: &mut App| {
                                    on_up(SettingUpdate::RefreshModels, window, cx);
                                })
                        }),
                )
                .child(
                    h_flex()
                        .gap_2()
                        .flex_wrap()
                        .children(state.available_models.iter().map(|m| {
                            let on_up = on_update.clone();
                            let model = m.clone();
                            let is_active = state.selected_model == *m;
                            let b = Button::new(format!("model_{}", m)).label(m.clone()).small();
                            let s = if is_active {
                                b.primary()
                            } else {
                                b.secondary()
                            };
                            s.on_click(move |_, window, cx: &mut App| {
                                on_up(SettingUpdate::SelectModel(model.clone()), window, cx);
                            })
                        })),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child(format!(
                            "Selected: {} — unknown names fail loudly instead of substituting.",
                            state.selected_model
                        )),
                ),
        )
        // Request Timeout
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("REQUEST TIMEOUT"),
                )
                .child(
                    h_flex()
                        .gap_3()
                        .children([15, 30, 60, 120].into_iter().map(|t| {
                            let on_up = on_up1.clone();
                            let is_active = settings.request_timeout_secs == t;
                            let b = Button::new(format!("timeout_{}", t))
                                .label(format!("{}s", t))
                                .small()
                                .flex_1();
                            let s = if is_active {
                                b.primary()
                            } else {
                                b.secondary()
                            };
                            s.on_click(move |_, window, cx: &mut App| {
                                on_up(SettingUpdate::ServerTimeout(t), window, cx);
                            })
                        })),
                ),
        )
        // Auto Ping Interval
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("HEALTH CHECK POLLING FREQUENCY"),
                )
                .child(
                    h_flex().gap_3().children(
                        [(5, "5s"), (15, "15s"), (30, "30s"), (0, "Disabled")]
                            .into_iter()
                            .map(|(p, lbl)| {
                                let on_up = on_up2.clone();
                                let is_active = settings.auto_ping_interval_secs == p;
                                let b = Button::new(format!("ping_{}", p))
                                    .label(lbl)
                                    .small()
                                    .flex_1();
                                let s = if is_active {
                                    b.primary()
                                } else {
                                    b.secondary()
                                };
                                s.on_click(move |_, window, cx: &mut App| {
                                    on_up(SettingUpdate::AutoPing(p), window, cx);
                                })
                            }),
                    ),
                ),
        )
}

// -----------------------------------------------------------------------------
// 4. STUDIO DEFAULTS PANEL
// -----------------------------------------------------------------------------
fn render_studio_panel(
    settings: &crate::state::UserSettings,
    cx: &mut App,
    on_update: impl Fn(SettingUpdate, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let on_up1 = on_update.clone();
    let on_up2 = on_update.clone();
    let on_up3 = on_update.clone();
    let on_up4 = on_update.clone();
    let on_up5 = on_update.clone();

    let resolutions = [
        (512, 512, "512²"),
        (768, 768, "768²"),
        (1024, 1024, "1024²"),
        (1280, 768, "1280×768"),
    ];

    v_flex()
        .gap_6()
        .child(
            v_flex()
                .child(
                    div()
                        .font_family(FONT_HEADING)
                        .font_bold()
                        .text_base()
                        .text_color(cx.theme().foreground)
                        .child("Studio Generation Defaults"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("Presets applied automatically when starting new creative sessions."),
                ),
        )
        // Resolution
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("DEFAULT CANVAS RESOLUTION"),
                )
                .child(
                    h_flex()
                        .gap_3()
                        .children(resolutions.into_iter().map(|(w, h, lbl)| {
                            let on_up = on_up1.clone();
                            let is_active = settings.default_width == w && settings.default_height == h;
                            let b = Button::new(format!("res_{}_{}", w, h)).label(lbl).small().flex_1();
                            let s = if is_active { b.primary() } else { b.secondary() };
                            s.on_click(move |_, window, cx: &mut App| {
                                on_up(SettingUpdate::DefaultResolution(w, h), window, cx);
                            })
                        })),
                ),
        )
        // Steps & CFG
        .child(
            h_flex()
                .gap_6()
                .child(
                    v_flex()
                        .w_1_2()
                        .gap_2()
                        .child(
                            div()
                                .font_bold()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("DEFAULT STEPS"),
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child({
                                    let on_up = on_up2.clone();
                                    let cur = settings.default_steps;
                                    Button::new("step_down")
                                        .icon(IconName::Minus)
                                        .small()
                                        .secondary()
                                        .on_click(move |_, window, cx: &mut App| {
                                            let n = (cur.saturating_sub(5)).max(5);
                                            on_up(SettingUpdate::DefaultSteps(n), window, cx);
                                        })
                                })
                                .child(
                                    div()
                                        .w(px(50.0))
                                        .text_center()
                                        .font_bold()
                                        .text_sm()
                                        .text_color(cx.theme().foreground)
                                        .child(settings.default_steps.to_string()),
                                )
                                .child({
                                    let on_up = on_up2.clone();
                                    let cur = settings.default_steps;
                                    Button::new("step_up")
                                        .icon(IconName::Plus)
                                        .small()
                                        .secondary()
                                        .on_click(move |_, window, cx: &mut App| {
                                            let n = (cur + 5).min(50);
                                            on_up(SettingUpdate::DefaultSteps(n), window, cx);
                                        })
                                }),
                        ),
                )
                .child(
                    v_flex()
                        .w_1_2()
                        .gap_2()
                        .child(
                            div()
                                .font_bold()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("DEFAULT CFG SCALE"),
                        )
                        .child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child({
                                    let on_up = on_up3.clone();
                                    let cur = settings.default_cfg_scale;
                                    Button::new("cfg_down")
                                        .icon(IconName::Minus)
                                        .small()
                                        .secondary()
                                        .on_click(move |_, window, cx: &mut App| {
                                            let n = (cur - 0.5).max(1.0);
                                            on_up(SettingUpdate::DefaultCfg(n), window, cx);
                                        })
                                })
                                .child(
                                    div()
                                        .w(px(50.0))
                                        .text_center()
                                        .font_bold()
                                        .text_sm()
                                        .text_color(cx.theme().foreground)
                                        .child(format!("{:.1}", settings.default_cfg_scale)),
                                )
                                .child({
                                    let on_up = on_up3.clone();
                                    let cur = settings.default_cfg_scale;
                                    Button::new("cfg_up")
                                        .icon(IconName::Plus)
                                        .small()
                                        .secondary()
                                        .on_click(move |_, window, cx: &mut App| {
                                            let n = (cur + 0.5).min(20.0);
                                            on_up(SettingUpdate::DefaultCfg(n), window, cx);
                                        })
                                }),
                        ),
                ),
        )
        // Auto Save Switch
        .child(
            h_flex()
                .p_3()
                .rounded_xl()
                .border_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().secondary.opacity(0.25))
                .items_center()
                .justify_between()
                .child(
                    v_flex()
                        .child(
                            div()
                                .font_bold()
                                .text_sm()
                                .text_color(cx.theme().foreground)
                                .child("Auto-Save Generations to Disk"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Saves output PNGs automatically to /outputs without manual prompt."),
                        ),
                )
                .child({
                    let on_up = on_up4.clone();
                    Switch::new("switch_autosave")
                        .checked(settings.auto_save_to_disk)
                        .on_click(move |checked, window, cx: &mut App| {
                            on_up(SettingUpdate::AutoSave(*checked), window, cx);
                        })
                }),
        )
        // Pixel-Art Crisp Zoom Switch
        .child(
            h_flex()
                .p_3()
                .rounded_xl()
                .border_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().secondary.opacity(0.25))
                .items_center()
                .justify_between()
                .child(
                    v_flex()
                        .child(
                            div()
                                .font_bold()
                                .text_sm()
                                .text_color(cx.theme().foreground)
                                .child("Crisp Nearest-Neighbor Zoom"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Preserves razor-sharp pixel edges without bilinear smoothing."),
                        ),
                )
                .child({
                    let on_up = on_up5.clone();
                    Switch::new("switch_crisp")
                        .checked(settings.pixel_art_crisp_zoom)
                        .on_click(move |checked, window, cx: &mut App| {
                            on_up(SettingUpdate::PixelCrisp(*checked), window, cx);
                        })
                }),
        )
}

// -----------------------------------------------------------------------------
// 5. RMBG & ASEPRITE PANEL
// -----------------------------------------------------------------------------
fn render_rmbg_panel(
    settings: &crate::state::UserSettings,
    cx: &mut App,
    on_update: impl Fn(SettingUpdate, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let on_up1 = on_update.clone();
    let on_up2 = on_update.clone();
    let on_up3 = on_update.clone();
    let on_up4 = on_update.clone();
    let on_up5 = on_update.clone();

    v_flex()
        .gap_6()
        .child(
            v_flex()
                .child(
                    div()
                        .font_family(FONT_HEADING)
                        .font_bold()
                        .text_base()
                        .text_color(cx.theme().foreground)
                        .child("Background Remover & Aseprite"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("Sprite isolation thresholds, 1-bit alpha snapping, and export workflows."),
                ),
        )
        // Tolerance Stepper
        .child(
            h_flex()
                .p_3()
                .rounded_xl()
                .border_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().secondary.opacity(0.25))
                .items_center()
                .justify_between()
                .child(
                    v_flex()
                        .child(
                            div()
                                .font_bold()
                                .text_sm()
                                .text_color(cx.theme().foreground)
                                .child("Default Color Distance Tolerance"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Controls background color keying sensitivity (5% to 95%)."),
                        ),
                )
                .child({
                    let on_up = on_up1.clone();
                    let tol = settings.rmbg_default_tolerance;
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(
                            Button::new("tol_down")
                                .icon(IconName::Minus)
                                .small()
                                .secondary()
                                .on_click(move |_, window, cx: &mut App| {
                                    let n = (tol - 0.05).max(0.05);
                                    on_up(SettingUpdate::RmbgTolerance(n), window, cx);
                                }),
                        )
                        .child(
                            div()
                                .w(px(50.0))
                                .text_center()
                                .font_bold()
                                .text_sm()
                                .text_color(cx.theme().foreground)
                                .child(format!("{:.0}%", tol * 100.0)),
                        )
                        .child({
                            let on_up = on_up1.clone();
                            Button::new("tol_up")
                                .icon(IconName::Plus)
                                .small()
                                .secondary()
                                .on_click(move |_, window, cx: &mut App| {
                                    let n = (tol + 0.05).min(0.95);
                                    on_up(SettingUpdate::RmbgTolerance(n), window, cx);
                                })
                        })
                }),
        )
        // 1-Bit Retro Alpha Switch
        .child(
            h_flex()
                .p_3()
                .rounded_xl()
                .border_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().secondary.opacity(0.25))
                .items_center()
                .justify_between()
                .child(
                    v_flex()
                        .child(
                            div()
                                .font_bold()
                                .text_sm()
                                .text_color(cx.theme().foreground)
                                .child("1-Bit Retro Alpha Mode by Default"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Clamps opacity strictly to binary 0 or 255 for classic retro sprites."),
                        ),
                )
                .child({
                    let on_up = on_up2.clone();
                    Switch::new("switch_default_pixel")
                        .checked(settings.rmbg_default_pixel_mode)
                        .on_click(move |checked, window, cx: &mut App| {
                            on_up(SettingUpdate::RmbgPixelMode(*checked), window, cx);
                        })
                }),
        )
        // Halo Trim
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("DEFAULT HALO EDGE EROSION"),
                )
                .child(
                    h_flex()
                        .gap_3()
                        .children([(0, "Off (0px)"), (1, "1px (Recommended)"), (2, "2px (Deep)")].into_iter().map(|(trim, lbl)| {
                            let on_up = on_up3.clone();
                            let is_active = settings.rmbg_default_halo_trim == trim;
                            let b = Button::new(format!("trim_{}", trim)).label(lbl).small().flex_1();
                            let s = if is_active { b.primary() } else { b.secondary() };
                            s.on_click(move |_, window, cx: &mut App| {
                                on_up(SettingUpdate::RmbgHaloTrim(trim), window, cx);
                            })
                        })),
                ),
        )
        // Auto-Generate Aseprite JSON
        .child(
            h_flex()
                .p_3()
                .rounded_xl()
                .border_1()
                .border_color(cx.theme().border)
                .bg(cx.theme().secondary.opacity(0.25))
                .items_center()
                .justify_between()
                .child(
                    v_flex()
                        .child(
                            div()
                                .font_bold()
                                .text_sm()
                                .text_color(cx.theme().foreground)
                                .child("Auto-Generate Aseprite JSON Manifest"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(cx.theme().muted_foreground)
                                .child("Writes spritesheet slice data and color palette indices beside exported cutouts."),
                        ),
                )
                .child({
                    let on_up = on_up4.clone();
                    Switch::new("switch_aseprite_json")
                        .checked(settings.auto_generate_aseprite_json)
                        .on_click(move |checked, window, cx: &mut App| {
                            on_up(SettingUpdate::AsepriteJson(*checked), window, cx);
                        })
                }),
        )
        // Aseprite frame size
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("ASEPRITE FRAME SIZE"),
                )
                .child(
                    h_flex()
                        .gap_3()
                        .children([(16, 16), (32, 32), (64, 64), (128, 128)].into_iter().map(|(w, h)| {
                            let on_up = on_up5.clone();
                            let is_active = settings.aseprite_frame_width == w
                                && settings.aseprite_frame_height == h;
                            let b = Button::new(format!("frame_{}_{}", w, h))
                                .label(format!("{}×{}", w, h))
                                .small()
                                .flex_1();
                            let s = if is_active { b.primary() } else { b.secondary() };
                            s.on_click(move |_, window, cx: &mut App| {
                                on_up(SettingUpdate::AsepriteFrame(w, h), window, cx);
                            })
                        })),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child(format!(
                            "Current: {}×{} — oversized frames are rejected with an error.",
                            settings.aseprite_frame_width, settings.aseprite_frame_height
                        )),
                ),
        )
}

// -----------------------------------------------------------------------------
// 6. DIAGNOSTICS & STORAGE PANEL
// -----------------------------------------------------------------------------
fn render_diagnostics_panel(
    settings: &crate::state::UserSettings,
    state: &AppState,
    cx: &mut App,
    on_update: impl Fn(SettingUpdate, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let on_up1 = on_update.clone();
    let on_up2 = on_update.clone();
    let on_up3 = on_update.clone();

    v_flex()
        .gap_6()
        .child(
            v_flex()
                .child(
                    div()
                        .font_family(FONT_HEADING)
                        .font_bold()
                        .text_base()
                        .text_color(cx.theme().foreground)
                        .child("Diagnostics, Cache & Maintenance"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("System hardware capabilities, history reel buffer limits, and reset actions."),
                ),
        )
        // Hardware Card
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("HARDWARE ACCELERATION PROFILE"),
                )
                .child(
                    v_flex()
                        .p_4()
                        .rounded_xl()
                        .border_1()
                        .border_color(cx.theme().border)
                        .bg(cx.theme().secondary.opacity(0.3))
                        .gap_2()
                        .child(
                            h_flex()
                                .justify_between()
                                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("Host Operating System"))
                                .child(div().text_xs().font_bold().text_color(cx.theme().foreground).child("Arch Linux (x86_64)")),
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("GPU Acceleration"))
                                .child(div().text_xs().font_bold().text_color(rgb(0x10b981)).child("NVIDIA RTX 3050 Laptop (4GB VRAM)")),
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("UI Engine"))
                                .child(div().text_xs().font_bold().text_color(cx.theme().foreground).child("GPUI Kit v0.9.1 (Vulkan / Wayland)")),
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .child(div().text_xs().text_color(cx.theme().muted_foreground).child("History Reel Cached"))
                                .child(div().text_xs().font_bold().text_color(cx.theme().foreground).child(format!("{} entries", state.history.len()))),
                        ),
                ),
        )
        // History Limit
        .child(
            v_flex()
                .gap_2()
                .child(
                    div()
                        .font_bold()
                        .text_xs()
                        .text_color(cx.theme().muted_foreground)
                        .child("MAX HISTORY FILMSTRIP ITEMS"),
                )
                .child(
                    h_flex()
                        .gap_3()
                        .children([10, 25, 50, 100].into_iter().map(|limit| {
                            let on_up = on_up1.clone();
                            let is_active = settings.max_history_items == limit;
                            let b = Button::new(format!("hist_{}", limit)).label(format!("{} items", limit)).small().flex_1();
                            let s = if is_active { b.primary() } else { b.secondary() };
                            s.on_click(move |_, window, cx: &mut App| {
                                on_up(SettingUpdate::HistoryLimit(limit), window, cx);
                            })
                        })),
                ),
        )
        // Reset and Maintenance Buttons
        .child(
            h_flex()
                .gap_4()
                .child(
                    Button::new("btn_clear_cache")
                        .icon(IconName::Trash)
                        .label("Clear Filmstrip Cache")
                        .small()
                        .secondary()
                        .on_click(move |_, window, cx: &mut App| {
                            on_up2(SettingUpdate::ClearCache, window, cx);
                        }),
                )
                .child(
                    Button::new("btn_reset_defaults")
                        .icon(IconName::RotateCcw)
                        .label("Reset All Settings to Factory Default")
                        .small()
                        .secondary()
                        .on_click(move |_, window, cx: &mut App| {
                            on_up3(SettingUpdate::ResetDefaults, window, cx);
                        }),
                ),
        )
}
