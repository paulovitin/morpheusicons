# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

The primary design surface is the browser showcase. Every page lives in `pages/` — currently `pages/index.html` (the studio) and `pages/get-started.html` (the integration guide) — and is served at the site root, assembled into `_site/` by `make site`. The GPUI, egui, and Iced example windows are desktop-native Rust GUI surfaces that are also in design scope; they are not iOS or Android and do not follow a mobile-native design language.

## Users

Two audiences arrive at the same showcase and must both be satisfied by it:

- **Rust GUI developers** building on GPUI (Zed), egui, Iced, Leptos, or Dioxus. They arrive skeptical about whether automatic shape morphing actually looks good, need to see the motion quality with their own eyes, and then want the integration snippet for their specific framework so they can `cargo add` and move on.
- **Web developers** who would consume the WASM build in a browser app. They need proof the engine runs in the browser at all, and that its output is a plain SVG `d` string they can drop into their own markup.

The showcase must prove the engine in both worlds — native Rust and browser — rather than treating web as a demo of a native-only library.

## Product Purpose

MorpheusIcons is a physics-based icon morphing engine for Rust. It transforms one stroke icon into another with spring physics, so a developer never hand-authors keyframes, point mappings, or sprite sheets for an icon transition.

Success is a developer watching a morph, believing it, and adopting the crate in their own project — for the web audience, adopting the WASM build.

## Positioning

MorpheusIcons is an **engine, not an icon set**. Most icon libraries hand you static SVGs; a few hand you a fixed set of pre-animated pairs. MorpheusIcons computes the transition between *any two* stroke-based icons the developer brings — Lucide, Feather, Heroicons (outline), Tabler, Phosphor, or hand-drawn paths — by parsing the path, resampling it to equidistant points, aligning the two shapes with 2D Procrustes analysis, interpolating in polar space, and driving the result with a damped harmonic oscillator that can be interrupted and retargeted mid-flight without discontinuity.

The bring-your-own-icons claim and the interruptible spring are the two things a neighboring product could not truthfully copy without building the same math.

## Operating Context

- Developers evaluate this the way they evaluate any crate: skim the README on GitHub, poke the live demo, check the framework list for their stack, copy an install line, and read docs.
- Evaluation is fast and skeptical. Motion quality is the claim, so motion quality has to be visible immediately, not behind a build step.
- The showcase is a static page backed by a `wasm-pack` build (`make wasm`, served by `make run-web` on `localhost:8765`). CSS is Tailwind 3.4 compiled to `dist/output.css` via `npm run build:css`.
- The repo already carries a full open-source presence: CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, issue templates, PR template, CHANGELOG, dual MIT/Apache-2.0 licensing.

## Capabilities and Constraints

**Confirmed capabilities**

- Core engine is pure Rust, `std`-only, with no required third-party dependencies. Framework bindings are opt-in Cargo features.
- Built-in catalog of roughly 60 Lucide-style stroke icons plus preset morph pairs, behind the default `catalog` feature.
- Accepts raw `d` path data, full `<svg>` strings (circles, lines, rects converted), or any type implementing the `IconSource` trait.
- Automatic viewport normalization between differently sized icon sets (24×24 ↔ 256×256).
- Pre-morph compatibility checking with a quality score.
- Spring presets (`DEFAULT`, `BOUNCY`, `GENTLE`) plus user-defined stiffness/damping/mass.
- Integrations: GPUI, egui, Iced, Leptos, Dioxus, plus a pure SVG string renderer and WASM bindings.

**Durable constraints future work must preserve**

- **Nine-language localization.** EN, PT, ES, FR, DE, ZH, JA, KO, RU. Any new user-facing copy on the showcase needs all nine translations; adding an untranslated string is a regression, not a TODO.
- **Accessibility is a floor.** Skip-to-content link, visible `:focus-visible` rings, `prefers-reduced-motion` honored, ≥44px touch targets, live regions, focus trapping in the loading overlay, and labeled controls. New work does not ship below this line.
- **The site works without WASM.** If the engine fails to load, is blocked, or times out, the page must degrade to a usable static demo. The retry and "View Static Demo" paths are load-bearing, not decoration.
- **Dependency-light by ethos.** The showcase is vanilla JS with no front-end framework; the Rust core stays `std`-only. Keep it that way.

**Terminology**

Morph (the transition), morph pair, spring config, viewport, `IconSource`, Procrustes alignment, polar interpolation, quality score.

**Explicitly undecided**

- The canonical repository URL. `README.md` and `Cargo.toml` point at `github.com/paulo/morpheusicons`; every GitHub link under `pages/` points at `github.com/rust-ui/ui`. One of these is wrong and the conflict is unresolved. Do not treat either as confirmed until the user settles it.
- The `authors` field in `Cargo.toml` is a placeholder (`paulo@example.com`).

## Brand Commitments

- Name: **MorpheusIcons**. Crate name `morpheusicons`.
- Dual-licensed MIT OR Apache-2.0 — stated on the site and in the repo.
- Attribution to the original **morphicons.com** by Guillermo López is a standing, deliberate credit. It appears in the README footer, the hero, and the site footer. Future work keeps the credit.
- An existing logo mark lives at `assets/morpheus_logo.svg` (gear silhouette with an inner "M" vector). It is present, not confirmed as fixed identity.

Note for future visual work: the dark/neon-vs-light drift recorded here originally has been resolved. `src/input.css` is now the single source of hand-written CSS for every page (the inline no-build fallback blocks are generated from it by `npm run sync:styles`), the dark palette is gone, and the whole surface runs on one green ramp built on `#15803d`.

## Evidence on Hand

**Real, in-repo**

- Working morph engine with unit and integration tests (`tests/morph_test.rs`, `cargo test --all-features`).
- Five rendered midpoint stills, SVG and PNG: play→pause, menu→X, sun→moon, check→X, arrow-right→down.
- Six runnable examples: CLI, GPUI, egui, Iced, Leptos/rust-ui, and a web server.
- Built WASM artifacts in `pkg/`.
- A working, interactive showcase page with live spring scrubbing and per-framework code generation.

**Explicitly absent — must not be fabricated**

- The crate is **not published to crates.io**. The crates.io and docs.rs badges in the README and the crates.io/docs.rs links on the site are aspirational. No published version, no download count, no live docs.rs page.
- No users, adopters, testimonials, case studies, benchmarks, press, or star counts exist. Do not invent them, and do not imply adoption the project has not had.
- No performance numbers have been measured. The "60fps" phrasing in the current meta description is an aspiration, not a benchmark.

## Product Principles

1. **Show the morph before you explain it.** The claim is motion quality; a still frame or a paragraph cannot make it. Whatever a visitor sees first should be moving, real, and driven by the actual engine.
2. **Bring-your-own-icons is the position — never quietly become an icon set.** Every surface should make it obvious that the developer's own Lucide/Tabler/hand-drawn paths are first-class, and that the built-in catalog is a convenience, not the product.
3. **Serve the Rust developer and the web developer in the same breath.** Neither audience should feel like the afterthought demo.
4. **Degrade honestly.** WASM can fail. Reduced motion is a real preference. Nine languages are real readers. The design is not finished until it holds up in each of those states.
5. **Claim only what the repo can back.** Until the crate ships, the site sells a working engine and its math — not adoption, not benchmarks, not a published release.

## Accessibility & Inclusion

No external standard has been contractually required, but the incumbent build sets a deliberate and specific floor that future work must hold: keyboard operability throughout, visible focus indication, `prefers-reduced-motion` support on a site whose entire subject is motion, ≥44px touch targets, screen-reader labeling on every control, and live-region announcements for the morph state. Localization into nine languages, including CJK and Cyrillic, is part of the same commitment — layouts and type must survive those scripts.
