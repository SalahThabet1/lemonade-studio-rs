---
version: "alpha"
name: "Neubrutalism"
description: "Neubrutalist interface. Ideal for landing pages, saas. AI-ready template."
colors:
  primary: "#FFEB3B"
  secondary: "#FF5252"
  tertiary: "#2196F3"
  neutral: "#000000"
typography:
  h1:
    fontFamily: Climate Crisis
    fontSize: 2.25rem
    fontWeight: 700
  body-md:
    fontFamily: Sora
    fontSize: 1rem
    fontWeight: 400
  label-caps:
    fontFamily: Sora
    fontSize: 0.75rem
    fontWeight: 500
components:
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.neutral}"
    padding: 12px
---

## Overview

Neubrutalist interface. Ideal for landing pages, saas. AI-ready template. Brutalism on the web was always a provocation. Raw HTML, system fonts, zero polish — it said "I don't care about your expectations." Neubrutalism kept the attitude but ditched the hostility. It's brutalism that actually wants you to click the button.

The turning point was Gumroad's 2021 redesign. Sahil Lavingia stripped the product down to thick black borders, flat primary colors, and offset drop shadows that looked like someone dragged a rectangle two pixels southeast. It was loud, playful, and unmistakably intentional. Designers noticed. Within months, the Figma community had dozens of neubrutalist UI kits climbing the charts.

What separates it from classic brutalism is warmth. Where brutalism embraced ugliness as ideology, neubrutalism uses bold geometry as decoration. The borders are thick but the corners can be rounded. The shadows are hard but the palette is candy-colored. It's confrontational aesthetics made approachable — punk rock in a good mood.

- Density: 5/10 — Balanced
- Variance: 4/10 — Moderate
- Motion: 4/10 — Subtle

- **Style:** Bold, Colorful, Raw, Playful
- **Keywords:** Bold borders, black outlines, primary colors, thick shadows, no gradients, flat colors, 45° shadows, playful, Gen Z
- **Era:** 2020s Modern
- **Light/Dark:** ✓ Full / ✓ Full

## Colors & Curated Themes

The 4 official Neubrutalism palettes designed for both Light and Dark modes:

### 1. The Gumroad Classic (Primary Pop)
* **Vibe:** High-energy, iconic creator standard with unmistakable warmth.
* **Light Mode:** Canvas `#FFFDF5`, Header `#FFEB3B`, Cards `#FFFFFF`, 2px Outlines `#000000`, Text `#000000`.
* **Dark Mode:** Canvas `#121212`, Header `#1E1E1E`, Cards `#1E1E1E`, 2px Neon Yellow Outlines `#FFEB3B`, Text `#FFFFFF`.
* **Key Accents:** Electric Yellow (`#FFEB3B`), Coral Red (`#FF5252`), Electric Blue (`#2196F3`).

### 2. Acid Matrix (Developer Cyber)
* **Vibe:** Edgy, high-octane technical brutalism.
* **Light Mode:** Canvas `#F4FBF4`, Header `#CCFF00`, Cards `#FFFFFF`, 2px Outlines `#000000`, Text `#000000`.
* **Dark Mode:** Canvas `#0D0F12`, Header `#15181F`, Cards `#161A22`, 2px Acid Lime Outlines `#CCFF00`, Text `#FFFFFF`.
* **Key Accents:** Acid Lime (`#CCFF00`), Cyber Violet (`#9D00FF`), Cyber Orange (`#FF5F1F`).

### 3. Bubblegum Punk (Gen-Z Pop-Art)
* **Vibe:** Playful, loud, creative studio aesthetic.
* **Light Mode:** Canvas `#FFF5F8`, Header `#FF73FA`, Cards `#FFFFFF`, 2px Outlines `#000000`, Text `#000000`.
* **Dark Mode:** Canvas `#151017`, Header `#201824`, Cards `#221A26`, 2px Neon Magenta Outlines `#FF73FA`, Text `#FFFFFF`.
* **Key Accents:** Hot Magenta (`#FF73FA`), Tangerine (`#FF9F1C`), Mint Cream (`#88D498`).

### 4. Bauhaus Modern (Architectural Cobalt)
* **Vibe:** Modernist graphic poster brutalism with crisp primary geometry.
* **Light Mode:** Canvas `#F6F8FC`, Header `#2979FF`, Cards `#FFFFFF`, 2px Outlines `#000000`, Text `#000000`.
* **Dark Mode:** Canvas `#0D1117`, Header `#161C26`, Cards `#19212D`, 2px Electric Cobalt Outlines `#2979FF`, Text `#FFFFFF`.
* **Key Accents:** Electric Cobalt (`#2979FF`), Safety Orange (`#FF6D00`), Signal Yellow (`#FFD600`).


## Typography

- **Headings / Display:** `Climate Crisis` ([Google Fonts](https://fonts.google.com/specimen/Climate+Crisis)) — Weight 700 / Ultra-bold display font with iconic brutalist block geometry for hero headers, dialog titles, space titles, and section cards.
- **Body & UI Controls:** `Sora` ([Google Fonts](https://fonts.google.com/specimen/Sora)) — Geometric sans-serif with distinct rhythm and high legibility for all body copy, button labels, control sliders, tags, and settings.
- **Monospace:** `JetBrains Mono` / `DejaVu Sans Mono` — Used for code, latents, dimensions, seed metadata, and technical log entries.

Scale:
- Hero / Space Titles: 1.5rem – 2.25rem (Weight 700, Climate Crisis)
- Section / Dialog Titles: 1.125rem – 1.5rem (Weight 700, Climate Crisis)
- Body: 1rem / 1.5 (Weight 400–500, Sora)
- UI Labels & Tags: 0.75rem – 0.875rem (Weight 500–600, Sora)
- Small / Metadata: 0.75rem (Weight 400, Sora / Mono)


## Layout

- **Grid:** CSS Grid primary. Max-width containment: 1280px centered with 1.5rem side padding.
- **Spacing rhythm:** Balanced. Base unit: 0.5rem (8px).
- **Section vertical gaps:** clamp(4rem, 8vw, 8rem).
- **Hero layout:** Split-screen (text left, visual right).
- **Feature sections:** Zig-zag alternating text+image rows. No 3-equal-columns.
- **Mobile collapse:** All multi-column layouts collapse below 768px. No horizontal overflow.
- **z-index contract:** base (0) / sticky-nav (100) / overlay (200) / modal (300) / toast (500).


## Elevation & Depth

box-shadow: 4px 4px 0 #000, border: 3px solid #000, no gradients, sharp corners (0px), bold typography

- **Physics:** Ease-out curves, 200-300ms duration. Smooth and predictable.
- **Entry animations:** Fade + translate-Y (16px → 0) over 420ms ease-out. Staggered cascades for lists: 80ms between items.
- **Hover states:** Subtle color shift + shadow adjustment over 200ms.
- **Page transitions:** Fade only (200ms).
- **Performance:** Only transform and opacity animated. No layout-triggering properties.


## Shapes

Base corner radius: 8px. See rounded tokens in front matter for the full scale.


## Components

- **Primary Button:** Subtly rounded (0.5rem) shape. Accent color fill. Hover: 8% darken + subtle lift shadow. Active: -1px translate tactile press. Font weight 600. No outer glows.
- **Secondary / Ghost Button:** Outline variant. 1.5px border in muted color. Text in primary color. Hover: subtle background fill.
- **Cards:** Subtly rounded (0.5rem) corners. Surface background. Subtle shadow (0 2px 12px rgba(0,0,0,0.06)). 1px border stroke.
- **Inputs:** Label above input. 1px border stroke. Focus ring: 2px accent color offset 2px. Error text below in semantic red. No floating labels.
- **Navigation:** Primary surface background. Active item: accent color indicator. Font weight 500 when active.
- **Skeletons:** Shimmer animation matching component dimensions. No circular spinners.
- **Empty States:** Icon-based composition with descriptive text and action button.


## Do's and Don'ts

- No emojis in UI — use icon system only (Lucide, Heroicons)
- No pure black (#000000) — use off-black or charcoal variants
- No oversaturated accent colors (saturation cap: 80%)
- No 3-column equal-width feature layouts — use zig-zag or asymmetric grid
- No `h-screen` — use `min-h-[100dvh]`
- No AI copywriting clichés: "Elevate", "Seamless", "Unleash", "Next-Gen"
- No broken external image links — use picsum.photos or inline SVG
- No generic lorem ipsum in demos

- Do Hard borders (2-4px)
- Do Hard offset shadows
- Do High saturation colors
- Do Bold typography
- Do No blurs/gradients
- Do Distinctive 'ugly-cute' look


## Use Case

Landing pages, SaaS make this the design.md of the app and implement it
