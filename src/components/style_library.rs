use crate::state::{AppState, CustomPrompt, FONT_HEADING};
use gpui::prelude::FluentBuilder as _;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::scroll::ScrollableElement as _;
use gpui_component::tag::Tag;
use gpui_component::{Icon, Sizable as _, StyledExt as _, h_flex, v_flex};
use gpui_kit::*;
use gpui_kit_assets::IconName;

#[derive(Clone, Debug, PartialEq)]
pub struct RecipeItem {
    pub id: String,
    pub title: String,
    pub category: String,
    pub prompt: String,
    pub width: u32,
    pub height: u32,
    pub steps: u32,
    pub cfg: f32,
    pub is_custom: bool,
    pub created_at: String,
    pub icon: IconName,
}

pub fn get_curated_recipes() -> Vec<RecipeItem> {
    vec![
        RecipeItem {
            id: "curated_silver_knight".into(),
            title: "Silver Knight Spritesheet".into(),
            category: "Characters".into(),
            prompt: "pixel art spritesheet of a valiant knight in shining silver and gold plate armor, holding longsword and crest shield, idle, walk, and sword slash animation frames, 16-bit retro game style, clean solid background".into(),
            width: 512,
            height: 512,
            steps: 24,
            cfg: 4.5,
            is_custom: false,
            created_at: "Built-in".into(),
            icon: IconName::Shield,
        },
        RecipeItem {
            id: "curated_cyber_mech".into(),
            title: "Cyberpunk Bipedal Mech".into(),
            category: "Mecha & Sci-Fi".into(),
            prompt: "16-bit pixel art cyberpunk bipedal combat walker mech with twin gatling guns and missile pods, sleek dark titanium chassis with glowing neon cyan hazard stripes, side profile view, dark solid background".into(),
            width: 1280,
            height: 768,
            steps: 24,
            cfg: 5.0,
            is_custom: false,
            created_at: "Built-in".into(),
            icon: IconName::Cpu,
        },
        RecipeItem {
            id: "curated_arcane_sorceress".into(),
            title: "Celestial Arcane Sorceress".into(),
            category: "Characters".into(),
            prompt: "pixel art spritesheet of an elemental sorceress in flowing celestial robes, levitating magic orb with flame and frost particle trail, casting spell poses, clean dark background, 16-bit SNES JRPG aesthetic".into(),
            width: 512,
            height: 512,
            steps: 24,
            cfg: 4.0,
            is_custom: false,
            created_at: "Built-in".into(),
            icon: IconName::WandSparkles,
        },
        RecipeItem {
            id: "curated_shadow_drake".into(),
            title: "Shadow Drake Boss".into(),
            category: "Monsters".into(),
            prompt: "pixel art sprite of an armored shadow drake dragon, obsidian scales with crimson glowing runes, spread wings, roaring breath attack, 16-bit boss battle style, solid black backdrop".into(),
            width: 768,
            height: 768,
            steps: 28,
            cfg: 5.0,
            is_custom: false,
            created_at: "Built-in".into(),
            icon: IconName::Flame,
        },
        RecipeItem {
            id: "curated_item_grid".into(),
            title: "Legendary RPG Item Grid".into(),
            category: "Items & Gear".into(),
            prompt: "3x3 inventory icon grid of fantasy RPG items: bubbling red health potion, glowing blue mana flask, enchanted dwarven waraxe, mithril tower shield, dragon claw amulet, magic leather boots, spell scroll, pixel art icons, solid background".into(),
            width: 512,
            height: 512,
            steps: 20,
            cfg: 4.0,
            is_custom: false,
            created_at: "Built-in".into(),
            icon: IconName::Grid2x2,
        },
        RecipeItem {
            id: "curated_acid_slime".into(),
            title: "Toxic Acid Slime".into(),
            category: "Monsters".into(),
            prompt: "pixel art spritesheet of a gelatinous acid slime creature, translucent green body with bubbling core, jumping, squashing, and split attack animations, solid background, retro arcade game".into(),
            width: 512,
            height: 512,
            steps: 20,
            cfg: 4.0,
            is_custom: false,
            created_at: "Built-in".into(),
            icon: IconName::Sparkles,
        },
        RecipeItem {
            id: "curated_dungeon_tileset".into(),
            title: "Modular Dungeon Tileset".into(),
            category: "Tilesets".into(),
            prompt: "isometric 16-bit pixel art dungeon tileset: stone brick floor tiles, mossy stone walls, arched doorway with iron portcullis, wall torch with fire flame, wooden treasure chest, modular game asset, solid background".into(),
            width: 768,
            height: 768,
            steps: 24,
            cfg: 4.5,
            is_custom: false,
            created_at: "Built-in".into(),
            icon: IconName::Layers,
        },
        RecipeItem {
            id: "curated_cyber_shinobi".into(),
            title: "Cyber Shinobi Assassin".into(),
            category: "Characters".into(),
            prompt: "pixel art full spritesheet of a cybernetic ninja in matte black carbon suit with glowing emerald visor, dual wakizashi blades, dash, wall-cling, and blade flurry frames, dark solid background".into(),
            width: 512,
            height: 512,
            steps: 24,
            cfg: 4.5,
            is_custom: false,
            created_at: "Built-in".into(),
            icon: IconName::Zap,
        },
        RecipeItem {
            id: "curated_skill_badges".into(),
            title: "RPG Skill Badges".into(),
            category: "UI & Badges".into(),
            prompt: "grid of circular retro game skill badges: fireball icon, lightning strike, iron shield defense, winged speed boost, heart vitality, lockpick stealth, crisp pixel art icons, solid background".into(),
            width: 512,
            height: 512,
            steps: 20,
            cfg: 4.0,
            is_custom: false,
            created_at: "Built-in".into(),
            icon: IconName::Award,
        },
        RecipeItem {
            id: "curated_space_fighter".into(),
            title: "Retro Arcade Space Fighter".into(),
            category: "Mecha & Sci-Fi".into(),
            prompt: "top-down 16-bit pixel art space fighter ship, twin plasma engines with blue exhaust trails, dual laser turrets, wing cannons, retro SHMUP arcade style, solid black space backdrop".into(),
            width: 768,
            height: 768,
            steps: 24,
            cfg: 5.0,
            is_custom: false,
            created_at: "Built-in".into(),
            icon: IconName::Rocket,
        },
    ]
}

fn category_icon(category: &str) -> IconName {
    match category {
        "Characters" => IconName::Shield,
        "Monsters" => IconName::Flame,
        "Items & Gear" => IconName::Grid2x2,
        "Mecha & Sci-Fi" => IconName::Cpu,
        "Tilesets" => IconName::Layers,
        "UI & Badges" => IconName::Award,
        _ => IconName::Sparkles,
    }
}

pub fn render_style_library(
    state: &AppState,
    _cx: &mut App,
    on_select_category: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
    on_open_add_recipe: impl Fn(&mut Window, &mut App) + 'static + Clone,
    _on_save_from_studio: impl Fn(&mut Window, &mut App) + 'static + Clone,
    on_load_recipe: impl Fn(RecipeItem, &mut Window, &mut App) + 'static + Clone,
    on_copy_prompt: impl Fn(&str, &mut Window, &mut App) + 'static + Clone,
    on_edit_recipe: impl Fn(RecipeItem, &mut Window, &mut App) + 'static + Clone,
    on_delete_custom_recipe: impl Fn(String, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let p = state.palette();
    let current_cat = state.library_category.as_str();

    // 1. Gather all recipes (curated + custom)
    let curated = get_curated_recipes();
    let custom_items: Vec<RecipeItem> = state
        .settings
        .custom_prompts
        .iter()
        .map(|c: &CustomPrompt| RecipeItem {
            id: c.id.clone(),
            title: c.title.clone(),
            category: c.category.clone(),
            prompt: c.prompt.clone(),
            width: c.width,
            height: c.height,
            steps: c.steps,
            cfg: c.cfg,
            is_custom: true,
            created_at: if c.created_at.is_empty() {
                "Custom".into()
            } else {
                c.created_at.clone()
            },
            icon: category_icon(&c.category),
        })
        .collect();

    let custom_count = custom_items.len();
    let total_count = curated.len() + custom_count;

    let mut all_recipes = custom_items;
    all_recipes.extend(curated);

    // 2. Filter by selected category
    let filtered_recipes: Vec<RecipeItem> = all_recipes
        .into_iter()
        .filter(|r| match current_cat {
            "All" => true,
            "My Recipes" => r.is_custom,
            other => r.category.eq_ignore_ascii_case(other),
        })
        .collect();

    let categories = [
        ("All", format!("All ({})", total_count)),
        ("My Recipes", format!("My Recipes ({})", custom_count)),
        ("Characters", "Characters".into()),
        ("Monsters", "Monsters".into()),
        ("Items & Gear", "Items & Gear".into()),
        ("Mecha & Sci-Fi", "Mecha & Sci-Fi".into()),
        ("Tilesets", "Tilesets".into()),
        ("UI & Badges", "UI & Badges".into()),
    ];

    v_flex()
        .size_full()
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
                    .opacity(if p.is_dark { 0.20 } else { 0.30 }),
            )
        })
        // Category Filter Chips Strip
        .child(
            h_flex()
                .w_full()
                .px_6()
                .py_3()
                .gap_2()
                .items_center()
                .border_b_2()
                .border_color(rgb(p.border))
                .bg(rgb(p.bg_sidebar))
                .children(categories.iter().map(|(cat_id, label)| {
                    let on_sel = on_select_category.clone();
                    let is_active = current_cat == *cat_id;
                    let id_str = cat_id.to_string();

                    if is_active {
                        Button::new(format!("cat_btn_{}", cat_id))
                            .label(label.clone())
                            .primary()
                            .small()
                            .on_click(move |_, window, cx: &mut App| {
                                on_sel(id_str.clone(), window, cx);
                            })
                    } else {
                        Button::new(format!("cat_btn_{}", cat_id))
                            .label(label.clone())
                            .secondary()
                            .small()
                            .on_click(move |_, window, cx: &mut App| {
                                on_sel(id_str.clone(), window, cx);
                            })
                    }
                })),
        )
        // Main Scrollable Grid of Recipes
        .child(
            v_flex()
                .flex_1()
                .h_0()
                .w_full()
                .overflow_y_scrollbar()
                .p_6()
                .child(if filtered_recipes.is_empty() {
                    // Empty State View
                    div()
                        .w_full()
                        .py_16()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap_4()
                        .child(
                            div()
                                .size(px(56.0))
                                .rounded_2xl()
                                .bg(rgb(p.bg_chip))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(rgb(p.accent))
                                .child(Icon::new(IconName::BookOpen).size(px(28.0))),
                        )
                        .child(
                            div()
                                .font_family(FONT_HEADING)
                                .font_bold()
                                .text_base()
                                .text_color(rgb(p.text_primary))
                                .child(if current_cat == "My Recipes" {
                                    "No Custom Recipes Saved Yet"
                                } else {
                                    "No Recipes Found in This Category"
                                }),
                        )
                        .child(div().text_sm().text_color(rgb(p.text_muted)).child(
                            "Create your first custom recipe or save your current Studio prompt.",
                        ))
                        .child({
                            let on_add = on_open_add_recipe.clone();
                            Button::new("btn_empty_create_recipe")
                                .icon(IconName::Plus)
                                .label("Create Custom Recipe")
                                .primary()
                                .on_click(move |_, window, cx: &mut App| {
                                    on_add(window, cx);
                                })
                        })
                        .into_any_element()
                } else {
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_4()
                        .w_full()
                        .children(filtered_recipes.into_iter().map(|recipe| {
                            let on_load = on_load_recipe.clone();
                            let on_cp = on_copy_prompt.clone();
                            let on_edit = on_edit_recipe.clone();
                            let on_del = on_delete_custom_recipe.clone();
                            let recipe_clone = recipe.clone();
                            let prompt_text = recipe.prompt.clone();
                            let recipe_id = recipe.id.clone();
                            let is_custom = recipe.is_custom;

                            div()
                                .w(px(380.0))
                                .p_4()
                                .rounded_xl()
                                .border_2()
                                .border_color(if is_custom {
                                    rgb(p.accent)
                                } else {
                                    rgb(p.border)
                                })
                                .bg(rgb(p.bg_card))
                                .shadow_md()
                                .overflow_hidden()
                                .flex()
                                .flex_col()
                                .gap_2p5()
                                .hover(|s| s.border_color(rgb(p.accent)))
                                // Row 1: Header Category, Status Badges & Date
                                .child(
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            h_flex()
                                                .gap_1p5()
                                                .items_center()
                                                .child(
                                                    div()
                                                        .size(px(24.0))
                                                        .rounded_md()
                                                        .bg(rgb(p.bg_chip))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .text_color(rgb(p.accent))
                                                        .child(
                                                            Icon::new(recipe.icon).size(px(13.0)),
                                                        ),
                                                )
                                                .child(if is_custom {
                                                    Tag::primary().child("Custom")
                                                } else {
                                                    Tag::secondary().child("Curated")
                                                })
                                                .child(
                                                    Tag::secondary().child(recipe.category.clone()),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_medium()
                                                .text_color(rgb(p.text_muted))
                                                .child(recipe.created_at.clone()),
                                        ),
                                )
                                // Row 2: Recipe Title in Climate Crisis Font
                                .child(
                                    div()
                                        .w_full()
                                        .font_family(FONT_HEADING)
                                        .font_bold()
                                        .text_sm()
                                        .text_color(rgb(p.text_primary))
                                        .child(recipe.title.clone()),
                                )
                                // Row 3: Recommended Parameters Badges
                                .child(
                                    h_flex()
                                        .w_full()
                                        .flex_wrap()
                                        .gap_1p5()
                                        .items_center()
                                        .child(
                                            Tag::primary().child(format!(
                                                "{}×{}",
                                                recipe.width, recipe.height
                                            )),
                                        )
                                        .child(
                                            Tag::secondary()
                                                .child(format!("{} Steps", recipe.steps)),
                                        )
                                        .child(
                                            Tag::secondary()
                                                .child(format!("CFG {:.1}", recipe.cfg)),
                                        ),
                                )
                                // Row 4: Prompt Text Preview Box
                                .child(
                                    div()
                                        .w_full()
                                        .h(px(84.0))
                                        .p_2p5()
                                        .rounded_lg()
                                        .border_2()
                                        .border_color(rgb(p.border_subtle))
                                        .bg(rgb(p.bg_input))
                                        .text_xs()
                                        .text_color(rgb(p.text_secondary))
                                        .overflow_hidden()
                                        .child(recipe.prompt.clone()),
                                )
                                // Row 5: Actions Row: Load in Studio, Copy Prompt, Edit, Delete
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .items_center()
                                        .child({
                                            let rc = recipe_clone.clone();
                                            Button::new(format!("btn_load_{}", recipe.id))
                                                .icon(IconName::Sparkles)
                                                .label("Load in Studio")
                                                .primary()
                                                .small()
                                                .flex_1()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_load(rc.clone(), window, cx);
                                                })
                                        })
                                        .child({
                                            let p_str = prompt_text.clone();
                                            Button::new(format!("btn_cp_{}", recipe.id))
                                                .icon(IconName::Copy)
                                                .label("Copy")
                                                .secondary()
                                                .small()
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_cp(&p_str, window, cx);
                                                })
                                        })
                                        .child({
                                            let r_for_edit = recipe_clone.clone();
                                            Button::new(format!("btn_edit_{}", recipe.id))
                                                .icon(IconName::Pencil)
                                                .label("Edit")
                                                .secondary()
                                                .small()
                                                .tooltip("Edit recipe prompt & parameters")
                                                .on_click(move |_, window, cx: &mut App| {
                                                    on_edit(r_for_edit.clone(), window, cx);
                                                })
                                        })
                                        .when(is_custom, |row| {
                                            let del_id = recipe_id.clone();
                                            row.child(
                                                Button::new(format!("btn_del_{}", recipe_id))
                                                    .icon(IconName::Trash)
                                                    .secondary()
                                                    .small()
                                                    .tooltip("Delete custom recipe from library")
                                                    .on_click(move |_, window, cx: &mut App| {
                                                        on_del(del_id.clone(), window, cx);
                                                    }),
                                            )
                                        }),
                                )
                        }))
                        .into_any_element()
                }),
        )
}
