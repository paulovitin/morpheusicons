---
inclusion: manual
name: Icon Libraries Reference
description: Detailed reference of all icon libraries compatible with MorpheusIcons, how to detect compatibility, and integration recipes for each library.
---

# Icon Libraries — Compatibility Reference

## Supported Libraries (Stroke-Based = Morphable)

### Tier 1: Perfect Compatibility (24×24, stroke-only)

| Library | Website | Viewport | Style | Notes |
|---------|---------|----------|-------|-------|
| **Lucide** | lucide.dev | 24×24 | Stroke, 2px | Fork of Feather. Most popular modern stroke set. All icons morph perfectly. |
| **Feather** | feathericons.com | 24×24 | Stroke, 2px | Original set. 286 icons. All morph perfectly. |
| **Heroicons Outline** | heroicons.com | 24×24 | Stroke, 1.5px | Only the `outline` variant. ~290 icons. All morph perfectly. |
| **Tabler Icons** | tabler.io/icons | 24×24 | Stroke, 2px | 5000+ icons. Largest stroke set. All morph perfectly. |

### Tier 2: Compatible with Viewport Adjustment

| Library | Website | Viewport | Style | Notes |
|---------|---------|----------|-------|-------|
| **Phosphor Regular** | phosphoricons.com | 256×256 | Stroke | Use `RawIcon::with_viewport(d, Viewport::new(256.0, 256.0))`. Auto-scaled during morph. |
| **Phosphor Light** | phosphoricons.com | 256×256 | Stroke (thin) | Same as Regular, thinner stroke. Works well. |
| **Phosphor Thin** | phosphoricons.com | 256×256 | Stroke (very thin) | Same viewport. Works but thin strokes may feel less visible mid-morph. |
| **Heroicons Mini** | heroicons.com | 20×20 | Stroke | Use `Viewport::STANDARD_20`. Fewer icons available in outline. |
| **Octicons** | primer.style/octicons | 16×16 | Stroke | Use `Viewport::STANDARD_16`. GitHub's icon set. |

### Tier 3: Partially Compatible (Mixed Stroke/Fill)

| Library | Website | Issue | Workaround |
|---------|---------|-------|------------|
| **Material Symbols Outlined** | fonts.google.com/icons | Mix of stroke and fill | Cherry-pick stroke-only icons; validate each with `check_path_data()` |
| **Remix Icon (Line)** | remixicon.com | Mostly stroke, some fill details | Use only the `-line` suffix variants |
| **Ionicons Outline** | ionic.io/ionicons | Mix | Use only `*-outline` variants |

### NOT Compatible (Fill-Based)

| Library | Why |
|---------|-----|
| **Heroicons Solid** | Fill-based geometry — shapes collapse instead of morphing |
| **Font Awesome** | Fill-based, variable viewports |
| **Phosphor Bold/Fill/Duotone** | Fill or compound fills |
| **Bootstrap Icons** | Mostly fill-based |
| **Material Symbols Filled** | Fill-based |
| **Remix Icon (Fill)** | Fill-based |

## How to Identify If an Icon Library Works

### Quick Heuristic (no code needed)

1. **Open any icon's SVG source** from the library
2. **Look at the attributes:**
   - ✅ Has `stroke="currentColor"` and `fill="none"` → **morphable**
   - ❌ Has `fill="currentColor"` and no `stroke` → **not morphable**
   - ⚠️ Has both `fill` and `stroke` → **test individually**

### SVG Attribute Fingerprints

```xml
<!-- ✅ GOOD: Lucide/Feather/Tabler pattern -->
<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
     fill="none" stroke="currentColor" stroke-width="2"
     stroke-linecap="round" stroke-linejoin="round">
  <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
</svg>

<!-- ✅ GOOD: Heroicons outline pattern -->
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
     stroke-width="1.5" stroke="currentColor">
  <path stroke-linecap="round" stroke-linejoin="round" d="M..."/>
</svg>

<!-- ❌ BAD: Fill-based pattern -->
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
  <path fill-rule="evenodd" d="M..." clip-rule="evenodd"/>
</svg>
```

### Programmatic Detection in Rust

```rust
use morpheusicons::icons::validate::*;
use morpheusicons::icons::source::Viewport;

/// Check if a path from an unknown library is morphable
fn is_morphable(svg_path_d: &str) -> bool {
    check_path_data(svg_path_d).is_ok()
}

/// Full quality assessment for a morph pair
fn assess_pair(icon_a: &str, icon_b: &str, viewport: &Viewport) -> String {
    let compat = check_morph_compatibility(icon_a, icon_b, viewport);
    match (compat.is_compatible, compat.quality_score) {
        (false, _) => "❌ Incompatible — cannot morph".into(),
        (true, s) if s >= 0.8 => format!("✅ Excellent ({:.0}%) — smooth morph", s * 100.0),
        (true, s) if s >= 0.6 => format!("⚠️ Good ({:.0}%) — minor artifacts possible", s * 100.0),
        (true, s) => format!("⚠️ Acceptable ({:.0}%) — visible artifacts likely", s * 100.0),
    }
}
```

## Integration Recipes by Library

### Lucide (most common case)

```rust
use morpheusicons::prelude::*;

// Get the `d` attribute from any Lucide SVG file or the lucide.dev website
let home = RawIcon::new("M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2zM9 22V12h6v10");
let settings = RawIcon::new("M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25...");

let ctrl = MorphController::from_sources(&home, &settings, SpringConfig::SMOOTH)?;
```

### Heroicons Outline

```rust
use morpheusicons::prelude::*;

// Copy the d="" from heroicons.com (make sure to select "Outline" style)
let magnifying_glass = RawIcon::new("m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196...");
let x_mark = RawIcon::new("M6 18 18 6M6 6l12 12");

let ctrl = MorphController::from_sources(&magnifying_glass, &x_mark, SpringConfig::SNAPPY)?;
```

### Tabler Icons

```rust
use morpheusicons::prelude::*;

// Tabler uses 24×24 viewport, same as Lucide — direct compatibility
let tabler_heart = RawIcon::new("M19.5 12.572l-7.5 7.428l-7.5-7.428...");
let tabler_star = RawIcon::new("M12 17.75l-6.172 3.245l1.179-6.873...");

let ctrl = MorphController::from_sources(&tabler_heart, &tabler_star, SpringConfig::BOUNCY)?;
```

### Phosphor (256×256 viewport)

```rust
use morpheusicons::prelude::*;

// Phosphor uses 256×256 — must declare viewport explicitly
let phosphor_house = RawIcon::with_viewport(
    "M218.83 103.77l-80-75.48a1.14 1.14 0 0 1-.05-.05l-3.95-3.72...",
    Viewport::new(256.0, 256.0),
);

// Mixing with a 24×24 icon — viewport auto-normalizes!
let ctrl = MorphController::from_sources(
    &Icon::Home,         // Built-in 24×24
    &phosphor_house,     // 256×256 → auto-scaled
    SpringConfig::GENTLE,
)?;
```

### Full SVG Document (any library)

```rust
use morpheusicons::prelude::*;

// When you have the complete <svg> element (e.g. downloaded from a library website)
let svg_string = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none"
    stroke="currentColor" stroke-width="2">
  <circle cx="12" cy="12" r="10"/>
  <path d="M12 8v8M8 12h8"/>
</svg>"#;

// Extracts all elements (circle, path, line, etc.) → combined path data
let icon = icon_from_svg(svg_string)?;
let ctrl = MorphController::from_sources(&Icon::X, &icon, SpringConfig::BOUNCY)?;
```

### Cross-Library Morphing

```rust
use morpheusicons::prelude::*;

// You can morph between icons from DIFFERENT libraries
// as long as both are stroke-based
let lucide_home = RawIcon::new("M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z");
let tabler_settings = RawIcon::new("M10.325 4.317c.426-1.756 2.924-1.756 3.35 0...");

// Both are 24×24 stroke icons — works perfectly
let ctrl = MorphController::from_sources(&lucide_home, &tabler_settings, SpringConfig::SMOOTH)?;
```

## Quality Score Interpretation

The `quality_score` from `check_morph_compatibility()` means:

| Score | Rating | What to Expect |
|-------|--------|----------------|
| 0.9–1.0 | Excellent | Silky smooth morph, no artifacts |
| 0.7–0.9 | Good | Smooth morph, possible minor wobble |
| 0.5–0.7 | Acceptable | Works but some expand/collapse visible |
| 0.3–0.5 | Poor | Noticeable artifacts, consider different pair |
| < 0.3 | Bad | Choose a different icon pair |

### What Lowers the Score

- Large difference in subpath count between source and target (−0.1 per subpath diff)
- Either icon having > 6 subpaths (−0.05 per extra)
- Explicit subpath count mismatch warning (−0.15)
- High subpath count warning (−0.10)
- Coordinates outside viewport (−0.05)

## Tips for Best Results

1. **Same library = best results** — icons from the same set share stroke weight, grid alignment, and design language
2. **Similar complexity = smoother morph** — a 2-stroke arrow morphing to a 2-stroke check looks better than to a 10-stroke settings gear
3. **Semantically related pairs** — play↔pause, sun↔moon, menu↔x — these tend to have similar geometry
4. **When mixing libraries** — validate with `check_morph_compatibility()` first
5. **For Phosphor** — always use Regular/Light/Thin variants, never Bold/Fill/Duotone
