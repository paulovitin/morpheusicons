# Contributing to MorpheusIcons

Obrigado por querer contribuir! 🎉 Toda contribuição é bem-vinda — de correções de typo até novos framework integrations.

## 📋 Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Architecture](#project-architecture)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Coding Standards](#coding-standards)
- [Adding a New Framework Integration](#adding-a-new-framework-integration)
- [Adding New Icons to the Catalog](#adding-new-icons-to-the-catalog)
- [Issue Guidelines](#issue-guidelines)

---

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/morpheusicons.git
   cd morpheusicons
   ```
3. **Add upstream** remote:
   ```bash
   git remote add upstream https://github.com/paulo/morpheusicons.git
   ```
4. **Create a branch** for your work:
   ```bash
   git checkout -b feat/my-awesome-feature
   ```

---

## Development Setup

### Prerequisites

- **Rust** 1.70+ (install via [rustup](https://rustup.rs))
- **wasm-pack** (optional, for WASM builds): `cargo install wasm-pack`

### Building

```bash
# Check everything compiles
cargo check --all-features

# Build the library
cargo build

# Build WASM package
wasm-pack build --target web --features wasm --no-default-features
```

### Running Examples

```bash
cargo run --example cli_morph
cargo run --example egui_morph_demo --features egui
cargo run --example iced_morph_demo --features iced
```

---

## Project Architecture

```
src/
├── animation/          # Core morphing engine
│   ├── morph.rs        # PathMorpher, MorphController, DrawCommands
│   └── spring.rs       # Spring physics solver (damped harmonic oscillator)
├── geometry/           # Math & SVG path processing
│   ├── path.rs         # SVG path parser (all commands)
│   ├── point.rs        # 2D point/vector math
│   ├── procrustes.rs   # Procrustes alignment & polar interpolation
│   └── sampling.rs     # Arc-length equidistant resampling
├── icons/              # Icon sources & catalog
│   ├── catalog.rs      # Built-in icon definitions
│   ├── pairs.rs        # Preset morph pairs
│   ├── source.rs       # IconSource trait, RawIcon, Viewport
│   ├── svg_extract.rs  # Full SVG → path data extraction
│   └── validate.rs     # Compatibility checking & quality scoring
├── integrations/       # Framework-specific bindings
│   ├── gpui.rs         # GPUI (Zed)
│   ├── egui.rs         # egui
│   ├── iced.rs         # Iced
│   ├── leptos.rs       # Leptos
│   ├── dioxus.rs       # Dioxus
│   └── svg.rs          # Pure SVG renderer
├── lib.rs              # Public API & prelude
└── wasm.rs             # WASM bindings
```

### Key Design Decisions

- **Zero required dependencies** — the core uses only `std` and pure math
- **IconSource trait** — any icon library can integrate by implementing one trait
- **Viewport normalization** — different icon sizes are scaled automatically
- **Spring physics** — all animations are interruptible without discontinuity

---

## Making Changes

### Branch Naming

- `feat/description` — new features
- `fix/description` — bug fixes
- `docs/description` — documentation changes
- `refactor/description` — code refactoring
- `perf/description` — performance improvements

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add Tabler Icons viewport detection
fix: correct arc-to-cubic conversion for large arcs
docs: add Phosphor integration example
refactor: simplify Procrustes rotation computation
perf: reduce allocations in sampled_icon_to_svg_path
test: add edge cases for SVG path parser
```

---

## Testing

```bash
# Run all library tests
cargo test --lib --all-features

# Run integration tests
cargo test --test morph_test

# Run a specific test
cargo test test_extract_circle

# Run tests with output
cargo test -- --nocapture
```

### Writing Tests

- Unit tests go in the same file, inside a `#[cfg(test)] mod tests { }` block
- Integration tests go in `tests/`
- Test SVG parsing edge cases, morph correctness, and spring physics convergence

---

## Submitting a Pull Request

1. **Ensure all checks pass locally:**
   ```bash
   cargo fmt --all
   cargo clippy --all-features -- -D warnings
   cargo test --lib --all-features
   cargo test --test morph_test
   cargo doc --all-features --no-deps
   ```

2. **Update CHANGELOG.md** under `[Unreleased]`

3. **Push your branch** and open a PR against `main`

4. **Fill out the PR template** — describe what, why, and how to test

5. **Respond to review feedback** — we aim for quick, constructive reviews

### What We Look For

- ✅ Code compiles without warnings
- ✅ Tests cover the new behavior
- ✅ Public API has doc comments
- ✅ No unnecessary dependencies added
- ✅ Performance is not regressed for hot paths (morph interpolation)

---

## Coding Standards

### Formatting

- Run `cargo fmt` before committing (enforced in CI)
- Max line width: follow rustfmt defaults (100 chars)

### Style

- Use `///` doc comments on all public items
- Prefer `Result<T, String>` for fallible operations in the public API
- Use `#[inline]` for small hot-path functions (point math, interpolation)
- Avoid `unsafe` — there should be zero `unsafe` in this crate
- Prefer iterators over index loops where readability isn't sacrificed

### Dependencies

- **Core crate must remain dependency-free** (only `std`)
- Framework integrations are gated behind features
- New optional dependencies need justification in the PR description

---

## Adding a New Framework Integration

1. Create `src/integrations/your_framework.rs`
2. Gate it behind a feature in `Cargo.toml`:
   ```toml
   your_framework = ["dep:your-framework-crate"]
   ```
3. Add the module in `src/integrations/mod.rs`:
   ```rust
   #[cfg(feature = "your_framework")]
   pub mod your_framework;
   ```
4. Export in the prelude if it has a primary widget/component
5. Add an example in `examples/`
6. Document in the README under Framework Integrations

---

## Adding New Icons to the Catalog

The built-in catalog uses Lucide-style icons (stroke-based, 24×24 viewport):

1. Add the variant to the `Icon` enum in `src/icons/catalog.rs`
2. Add the `path_data()` match arm with the SVG `d` attribute
3. The path data must be:
   - Stroke-based (no fill-only shapes)
   - In 24×24 coordinate space
   - Using standard SVG path commands

### Where to get path data

- [Lucide](https://lucide.dev) — inspect the SVG, copy the `d` attribute
- [Heroicons](https://heroicons.com) — use the "outline" variants only
- [Tabler](https://tabler.io/icons) — all icons are stroke-based

---

## Issue Guidelines

### Bug Reports

- Include a **minimal reproduction** (smallest code that triggers the bug)
- Include the **SVG path data** if the issue is about specific icons
- Include your Rust version and enabled features

### Feature Requests

- Explain the **use case**, not just the solution
- Show an **ideal API** — how would you want to use this?
- Note if you'd be willing to implement it yourself

---

## 🙏 Thank You

Every contribution makes MorpheusIcons better for the entire Rust UI ecosystem. Whether you fix a typo, improve performance, or add a whole new framework integration — you're awesome.

If you have questions, open a [Discussion](https://github.com/paulo/morpheusicons/discussions) or reach out in the issue tracker!
