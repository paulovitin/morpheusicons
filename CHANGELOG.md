# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2026-08-24

### Changed

- Bump `egui` 0.29 → 0.36 and `eframe` 0.29 → 0.36
- Bump `leptos` 0.7 → 0.8
- Bump `dioxus` 0.6 → 0.7
- Bump `iced` 0.13 → 0.14
- Bump `tailwindcss` 3.4.17 → 4.3.3
- Bump GitHub Actions: checkout v7, setup-node v7, upload-pages-artifact v5, deploy-pages v5

### Fixed

- Adapt egui examples for 0.36 API (`Frame::new()`, `corner_radius()`, `App::ui()`)
- Adapt iced example for 0.14 API (boot function)
- Pin `termcolor <1.4` to fix naga 27 compile error (upstream breaking change)
- Allow `Ubuntu-font-1.0` license in cargo-deny (new epaint_default_fonts dep)

## [0.1.1] - 2026-08-24

### Added

- `IconSource` trait — integrate any external icon library with one trait implementation
- `RawIcon` wrapper for using raw SVG path data strings from any source
- `Viewport` struct with standard sizes (24×24, 20×20, 16×16) and auto-scaling
- `extract_path_from_svg()` — extract path data from full SVG markup (converts `<circle>`, `<rect>`, `<line>`, `<polyline>`, `<polygon>`, `<ellipse>` to path commands)
- `icon_from_svg()` — convenience function to get an `IconSource` from SVG markup
- `MorphController::from_sources()` — create morph controllers from any `IconSource` pair
- `PathMorpher::from_sources()` — create morphers from any `IconSource` pair
- `check_morph_compatibility()` — validate icon pairs with quality scoring
- `check_path_data()` — quick validation of SVG path strings
- `KnownIconLibrary` enum documenting compatible icon libraries
- `IconPath::scale()` for viewport normalization

## [0.1.0] - 2024-01-01

### Added

- Core morphing engine with Procrustes 2D alignment and polar interpolation
- Spring physics animation driver (`SpringConfig::DEFAULT`, `BOUNCY`, `GENTLE`)
- SVG path parser supporting all standard commands (M, L, C, Q, A, H, V, S, T, Z)
- Arc-length equidistant resampling for smooth interpolation
- Built-in icon catalog with 60+ stroke-based icons (Lucide-style)
- Preset icon pairs (`IconPair::PlayPause`, `SunMoon`, `MenuX`, etc.)
- GPUI integration (`MorpheusGpui`)
- egui integration (`paint_morph_icon`)
- Iced integration (`MorpheusIced`)
- Leptos component (`MorphIcon`)
- Dioxus component (`MorphIcon`)
- Pure SVG string renderer (`SvgRenderer`)
- WebAssembly support via `wasm-bindgen`
- CLI morph example
- Web server example with WASM demo page

[Unreleased]: https://github.com/paulovitin/morpheusicons/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/paulovitin/morpheusicons/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/paulovitin/morpheusicons/releases/tag/v0.1.0
