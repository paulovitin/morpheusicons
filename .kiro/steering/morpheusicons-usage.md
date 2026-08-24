---
inclusion: auto
name: MorpheusIcons Usage Guide
description: How to use the MorpheusIcons morphing engine, supported icon libraries, compatibility rules, and integration patterns for any Rust GUI framework or WASM target.
---

# MorpheusIcons — AI Reference Guide

## What This Is

MorpheusIcons is a **physics-based icon morphing engine** in Rust. It transforms one stroke-based SVG icon into another using spring physics, Procrustes alignment, and polar interpolation. No keyframes, no sprite sheets — just math.

It is an **engine, not an icon set**. Developers bring their own icons from any stroke-based library.

## Quick Mental Model

```
Icon A (SVG path) + Icon B (SVG path) + SpringConfig → MorphController → animated SVG path string
```

## Core API Pattern

```rust
use morpheusicons::prelude::*;

// 1. Create controller from two icon sources
let ctrl = MorphController::from_sources(&source, &target, SpringConfig::BOUNCY)?;

// 2. Trigger morph
ctrl.morph_to_end();   // A → B
ctrl.morph_to_start(); // B → A

// 3. Update each frame (dt in seconds)
let still_animating = ctrl.update(0.016);
let svg_d = ctrl.current_svg_path(); // → "M6.00 4.00C..."
```

## How to Provide Icons

### Option 1: Built-in catalog (feature `catalog`)

```rust
use morpheusicons::prelude::*;
let ctrl = MorphController::new(Icon::Play.path_data(), Icon::Pause.path_data(), SpringConfig::BOUNCY)?;
```

### Option 2: Raw path data from any library

```rust
let icon = RawIcon::new("M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z");
```

### Option 3: Full SVG string (auto-extracts paths, circles, lines, etc.)

```rust
let icon = icon_from_svg(r#"<svg viewBox="0 0 24 24">...</svg>"#)?;
```

### Option 4: Implement `IconSource` trait

```rust
impl IconSource for MyIcon {
    fn path_data(&self) -> &str { &self.d }
    fn viewport(&self) -> Viewport { Viewport::STANDARD_24 }
}
```

## Supported Icon Libraries

#[[file:.kiro/steering/icon-libraries-reference.md]]

## Compatibility Rules — How to Know If an Icon Will Morph Well

### Hard Requirements (must pass or morph fails)

| Rule | Why |
|------|-----|
| Must be **stroke-based** geometry | Fill-based icons collapse, they don't morph |
| Must contain valid SVG path commands (M, L, C, Q, A, H, V, S, T, Z) | Parser needs to understand geometry |
| Viewport must have positive dimensions | Coordinate system must be valid |

### Quality Indicators (affect animation smoothness)

| Factor | Impact | Guidance |
|--------|--------|----------|
| Subpath count difference between A and B | High | Keep within ±4 subpaths for smooth results |
| Total subpath count > 8 | Medium | Complex icons still work but may feel busy |
| Coordinates outside viewport | Low | Usually fine, minor clipping possible |
| Different viewport sizes | None | Auto-normalized (e.g. 24×24 ↔ 256×256) |

### Programmatic Compatibility Check

```rust
use morpheusicons::icons::validate::*;

// Quick check: is this path valid?
let ok = check_path_data("M5 12h14").is_ok();

// Full compatibility report with quality score (0.0–1.0):
let compat = check_morph_compatibility(path_a, path_b, &Viewport::STANDARD_24);
println!("Compatible: {}, Quality: {:.0}%", compat.is_compatible, compat.quality_score * 100.0);
```

## Spring Configurations

| Preset | Behavior | Use Case |
|--------|----------|----------|
| `SpringConfig::DEFAULT` / `SMOOTH` | Fluid, no overshoot | General UI transitions |
| `SpringConfig::BOUNCY` | Elastic overshoot | Toggles, playful micro-interactions |
| `SpringConfig::GENTLE` | Slow, elegant | Page transitions, ambient motion |
| `SpringConfig::SNAPPY` | Fast, immediate | Direct feedback buttons |

Custom: `SpringConfig { stiffness: 200.0, damping: 15.0, mass: 1.0, precision: 0.001 }`

## Framework Integrations (Cargo features)

| Feature flag | Framework | Output |
|-------------|-----------|--------|
| `gpui` | GPUI (Zed editor) | Native element via `MorpheusGpui::morph_svg()` |
| `egui` | egui | Painter call via `paint_morph_icon()` |
| `iced` | Iced | SVG widget via `MorpheusIced::morph_svg()` |
| `leptos` | Leptos | `<MorphIcon>` component |
| `dioxus` | Dioxus | `MorphIcon {}` component |
| `wasm` | Browser/JS | WASM bindings, returns SVG path string |
| *(none)* | Any | `ctrl.current_svg_path()` → raw `d="..."` string |

## SVG Element Extraction

When given a full `<svg>` document, `extract_path_from_svg()` handles:

- `<path d="...">` — used directly
- `<circle>` — converted to arc-based path
- `<ellipse>` — converted to arc-based path
- `<rect>` — converted with optional rounded corners
- `<line>` — converted to M/L
- `<polyline>` — converted to M/L chain
- `<polygon>` — converted to M/L/Z

## Viewport Handling

- `Viewport::STANDARD_24` — Lucide, Feather, Heroicons, Tabler (24×24)
- `Viewport::STANDARD_20` — Heroicons mini (20×20)
- `Viewport::STANDARD_16` — Octicons (16×16)
- `Viewport::new(256.0, 256.0)` — Phosphor
- Auto-scaling happens during morphing if source/target viewports differ

## Project Layout

```
src/
├── animation/morph.rs      — MorphController (the main API)
├── animation/spring.rs     — Spring physics solver + presets
├── geometry/path.rs        — SVG path parser
├── geometry/procrustes.rs  — Procrustes alignment + polar interpolation
├── geometry/sampling.rs    — Arc-length resampling
├── icons/catalog.rs        — Built-in ~60 icons
├── icons/source.rs         — IconSource trait, RawIcon, Viewport
├── icons/svg_extract.rs    — Full SVG → path extraction
├── icons/validate.rs       — Compatibility checking + quality score
├── integrations/           — Framework-specific widgets
└── wasm.rs                 — WASM bindings
```

## Decision Flowchart for AI

When a user wants to morph icons:

1. **Identify the icon source** — Is it a library name (Lucide, Tabler, etc.), a raw `d` string, or a full SVG?
2. **Check stroke vs fill** — Only stroke-based icons morph. Solid/fill variants (Heroicons solid, Font Awesome) won't work.
3. **Determine viewport** — Most libs are 24×24. Phosphor is 256×256. Use `RawIcon::with_viewport()` if non-standard.
4. **Pick integration** — Match to the user's framework (GPUI/egui/Iced/Leptos/Dioxus/WASM/raw string).
5. **Choose spring** — `BOUNCY` for playful, `SMOOTH` for professional, `SNAPPY` for instant feedback.
6. **Validate if unsure** — Use `check_morph_compatibility()` to get a quality score before committing.
