---
name: MorpheusIcons
description: A white lab bench for a physics engine — clinical neutrals, one kinetic green, and a single specimen in motion.
colors:
  kinetic-green: "#15803d"
  kinetic-green-dark: "#166534"
  kinetic-green-deepest: "#14532d"
  kinetic-green-signal: "#16a34a"
  kinetic-green-wash: "#f0fdf4"
  kinetic-green-edge: "#bbf7d0"
  bench-white: "#ffffff"
  bench-ink: "#111317"
  bench-hairline: "#e5e8ec"
  bench-tray: "#f7f8fa"
  graphite: "#111827"
  slate-quiet: "#4b5563"
  slate-mid: "#374151"
  scrollbar-thumb: "#c2c7cf"
  code-keyword: "#4ade80"
  code-accent: "#86efac"
  code-type: "#bbf7d0"
  code-string: "#fcd34d"
  code-number: "#fbbf24"
  code-comment: "#8b96a8"
  code-punct: "#9ca3af"
  rule-gray: "#e5e7eb"
  field-gray: "#d1d5db"
  caution-wash: "#fffbeb"
  caution-edge: "#fde68a"
  caution-ink: "#b45309"
  alert-red: "#dc2626"
typography:
  display:
    fontFamily: "Space Grotesk, system-ui, sans-serif"
    fontSize: "clamp(1.875rem, 5vw, 3rem)"
    fontWeight: 700
    lineHeight: 1.25
    letterSpacing: "-0.025em"
  headline:
    fontFamily: "Space Grotesk, system-ui, sans-serif"
    fontSize: "clamp(1.25rem, 2.5vw, 1.5rem)"
    fontWeight: 700
    lineHeight: 1.33
    letterSpacing: "normal"
  title:
    fontFamily: "Space Grotesk, system-ui, sans-serif"
    fontSize: "1.125rem"
    fontWeight: 700
    lineHeight: 1.4
    letterSpacing: "-0.025em"
  body:
    fontFamily: "Space Grotesk, system-ui, sans-serif"
    fontSize: "0.875rem"
    fontWeight: 400
    lineHeight: 1.43
    letterSpacing: "normal"
  label:
    fontFamily: "Space Grotesk, system-ui, sans-serif"
    fontSize: "0.75rem"
    fontWeight: 700
    lineHeight: 1.33
    letterSpacing: "0.05em"
  mono:
    fontFamily: "JetBrains Mono, ui-monospace, monospace"
    fontSize: "0.75rem"
    fontWeight: 400
    lineHeight: 1.5
    letterSpacing: "normal"
rounded:
  pill: "8px"
  control: "12px"
  card: "16px"
  full: "9999px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "12px"
  lg: "16px"
  xl: "24px"
  2xl: "32px"
  3xl: "48px"
components:
  button-primary:
    backgroundColor: "{colors.kinetic-green}"
    textColor: "{colors.bench-white}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "10px 14px"
    height: "44px"
  button-primary-hover:
    backgroundColor: "{colors.kinetic-green-bright}"
    textColor: "{colors.bench-white}"
  button-secondary:
    backgroundColor: "{colors.bench-white}"
    textColor: "{colors.slate-mid}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "12px 20px"
    height: "44px"
  button-secondary-hover:
    backgroundColor: "{colors.bench-white}"
    textColor: "{colors.graphite}"
  preset-chip:
    backgroundColor: "{colors.bench-white}"
    textColor: "{colors.slate-mid}"
    typography: "{typography.label}"
    rounded: "{rounded.pill}"
    padding: "12px"
    height: "44px"
  preset-chip-active:
    backgroundColor: "{colors.kinetic-green}"
    textColor: "{colors.bench-white}"
  spring-pill:
    backgroundColor: "{colors.bench-white}"
    textColor: "{colors.slate-quiet}"
    typography: "{typography.label}"
    rounded: "{rounded.pill}"
    padding: "10px 16px"
    height: "44px"
  spring-pill-active:
    backgroundColor: "{colors.kinetic-green}"
    textColor: "{colors.bench-white}"
  framework-tab:
    backgroundColor: "transparent"
    textColor: "{colors.slate-quiet}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "10px 14px"
    height: "44px"
  framework-tab-active:
    backgroundColor: "{colors.kinetic-green}"
    textColor: "{colors.bench-white}"
  card:
    backgroundColor: "{colors.bench-white}"
    textColor: "{colors.bench-ink}"
    rounded: "{rounded.card}"
    padding: "24px"
  select-field:
    backgroundColor: "{colors.bench-white}"
    textColor: "{colors.slate-mid}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "8px 32px 8px 12px"
    height: "44px"
  code-panel:
    backgroundColor: "{colors.graphite}"
    textColor: "#e5e7eb"
    typography: "{typography.mono}"
    rounded: "{rounded.card}"
    padding: "24px"
  status-badge:
    backgroundColor: "{colors.kinetic-green-wash}"
    textColor: "{colors.kinetic-green-deep}"
    typography: "{typography.mono}"
    rounded: "{rounded.full}"
    padding: "4px 12px"
---

# Design System: MorpheusIcons

## Overview

**Creative North Star: "The Physics Lab Bench"**

MorpheusIcons is an engine that turns one icon into another using spring physics and Procrustes geometry. The interface that presents it is a bench: a clean white working surface, a faint measurement grid printed on it, calibrated controls at the edges, and one specimen under the light. The specimen is the morph. Everything else is apparatus, and apparatus that draws attention to itself is apparatus that failed.

The philosophy is **precise, warm, generous**. Engineering rigor — exact hairlines, a mono readout for every number, 44px minimum on everything touchable — softened by real breathing room and 8–16px corners rather than hard ones. The audience is a Rust or web developer evaluating in thirty seconds; the surface has to read as credible instrumentation immediately, and it has to be pleasant enough that they stay and drag the slider. Rigor without warmth reads as a spec sheet, and nobody plays with a spec sheet.

Only one color carries energy. **Kinetic Green** marks the things that move or can be made to move — the live dot, the active pair, the running spring, the primary action. Against a neutral bench it does the entire job of directing attention, which is why the neutrals must stay genuinely neutral. Four things are explicitly rejected: the dark neon dev-tool aesthetic, emoji standing in as UI vocabulary, gradient-and-glow SaaS landing energy, and shouty marketing weight.

**Key Characteristics:**

- White bench, hairline separations, near-zero shadow. Depth is a response, not a decoration.
- Exactly one accent family. Green means kinetic; nothing else earns color.
- An unbroken white bench: no texture, no tiled field, nothing printed under the work.
- Mono for every number, code, and machine-readable badge. Sans for everything a human wrote.
- Spring easing (`cubic-bezier(0.34, 1.56, 0.64, 1)`) on every meaningful transition — the interface obeys the same physics as the product.
- Stroke-based iconography at 2px, round caps and joins, 24×24. The icon language *is* the product.

## Colors

A neutral bench with a single live accent: the palette spends everything on white and gray so that one green can mean something.

### Primary

- **Kinetic Green** (#15803d): The accent, and the only one. It measures 5.02:1 against white **and** 5.02:1 under white, which is why it can be both a text color and a fill color without a second token. It marks motion and state — the pulsing live dot on the stage, the active preset chip, the selected spring config, the selected framework tab, the primary button, and every inline link. If an element neither moves nor represents a live/selected state, it does not get this color.
- **Kinetic Green Dark** (#166534) and **Deepest** (#14532d): Hover state and gradient end. Hover goes *darker*, never brighter — brightening a green fill drops white text below the floor.
- **Kinetic Green Bright** (#22c55e): Non-text decoration only — pulsing status dots and gradient starts. It is 2.28:1 on white and must never sit behind a label.
- **Kinetic Green Wash** (#f0fdf4) and **Kinetic Green Edge** (#bbf7d0): The badge and pill substrate — wash for fill, edge for its 1px border. Used for the license chip, the hero pill, and the live pair label.

### Secondary

- **Kinetic Green Signal** (#16a34a): Non-text only — the live pulse dot, decorative gradients, tinted shadows. It is 3.30:1 on white, so it may never carry text or be a fill behind text.

### Neutral

- **Bench White** (#ffffff): The page ground and every card surface. Not off-white, not tinted. The bench is white.
- **Bench Ink** (#111317): The document's default text color, set on `body`.
- **Graphite** (#111827): Doing two jobs — the color of headings and strong text, and the background of every code panel. Code is a dark object resting on a white bench; that inversion is the point.
- **Slate Quiet** (#4b5563): Secondary and supporting copy — descriptions, captions, labels at rest, the footer. The most-used text color after graphite.
- **Slate Mid** (#374151): Control labels and chip text at rest, one step stronger than supporting copy because it must stay legible at 12px.
- **Bench Hairline** (#e5e8ec): The 1px border on every card and pill. The primary separator in the entire system.
- **Rule Gray** (#e5e7eb) and **Field Gray** (#d1d5db): Section dividers and input strokes respectively.
- **Scrollbar Thumb** (#c2c7cf): The 6px custom scrollbar thumb inside the code panel. Non-text chrome; the only gray outside the main ramp.

**Code panel tokens.** The dark code panel is the one place the palette inverts, so it carries its own seven tokens — all measured against the #111827 panel, all clearing 4.5:1:

| Token | Value | On panel | Carries |
| --- | --- | --- | --- |
| Code Keyword | #4ade80 | 10.18:1 | `fn`, `let`, `use`, `const`, `import` |
| Code Accent | #86efac | 12.63:1 | Macros (`view!`), attributes (`#[component]`), flags |
| Code Type | #bbf7d0 | 14.64:1 | `PascalCase` types, TOML sections |
| Code String | #fcd34d | 12.30:1 | String and char literals |
| Code Number | #fbbf24 | 10.63:1 | Numeric literals |
| Code Comment | #8b96a8 | 5.94:1 | Comments, italic |
| Code Punct | #9ca3af | 6.99:1 | Operators and delimiters |

Two hues do all the work: **green carries structure**, **amber carries literals**. Everything else is neutral. There is no blue, no purple, no red — a code panel is not an excuse to reintroduce a palette the rest of the system refused.
- **Bench Tray** (#f7f8fa): Recessed surfaces — the scrollbar track, and the stage's inner well.

### Semantic

- **Caution Wash / Edge / Ink** (#fffbeb / #fde68a / #b45309): The static-mode notice shown when the WASM engine is unavailable. This is a load-bearing state, not an edge case — see PRODUCT.md.
- **Alert Red** (#dc2626): Loader failure copy only.

### Named Rules

**The One Accent Rule.** Green is the only color that carries meaning. If a new element needs to stand out and it is not live, selected, or in motion, the answer is weight, size, or space — never a new hue. A second accent family is how this system dies.

**The One Hairline Rule.** Every border in the system is 1px and one of two near-identical grays. Card and pill chrome uses Bench Hairline (#e5e8ec); Tailwind's border utilities resolve to Rule Gray (#e5e7eb). They are 1/255 apart and read as one line. Do not add a third border color, and do not exceed 1px.

**The Inversion Rule.** Dark surfaces exist for exactly one purpose: code. A graphite panel signals "this is machine text you will copy." Nothing else in the system goes dark, ever.

## Typography

**Display / Body Font:** Space Grotesk (fallback: system-ui, sans-serif)
**Mono Font:** JetBrains Mono (fallback: ui-monospace, monospace)

**Character:** Space Grotesk is a geometric grotesque with just enough eccentricity — the angled terminals, the flat-sided round forms — to read as engineered rather than generic. It carries technical authority without the coldness of a neo-grotesque. JetBrains Mono answers it directly: a coding face for a page selling a crate. The pairing says *built by developers, for developers* without a single word of copy doing that work.

### Hierarchy

- **Display** (700, 30px → 48px at 640px, 1.25, -0.025em): The hero statement. Once per page, never twice.
- **Headline** (700, 20px → 24px at 640px, 1.33): Section titles — "Multi-Framework Rust Integration", "Get Started".
- **Title** (700, 18px → 20px at 640px, 1.4, -0.025em): The product wordmark in the header, and dialog-level headings.
- **Body** (400, 14px, 1.43): Descriptions and supporting copy, in Slate Quiet. 16px is available for the hero paragraph at ≥640px. Cap measure at 65–75ch; the hero currently holds a `max-w-2xl` (42rem) container, which is the right instinct.
- **Label** (700, 12px, 0.05em, often uppercase): The workhorse — 12px is the single most-used size in the system. Every control, chip, tab, badge, and section eyebrow. Uppercase with `0.05em` tracking for eyebrows and category headers; sentence case for controls.
- **Mono** (400, 12–14px): Every number, path string, code block, install command, and machine-readable badge. The percentage readout, the license chip, the pair label.

### Named Rules

**The 700 Ceiling Rule.** Space Grotesk ships nothing above 700, and the project loads 400/500/600/700. Anything asking for 800 gets a browser-synthesized fake bold — smeared stems, broken optical weight. Use `font-bold` (700) as the maximum.

**The Mono Means Machine Rule.** If a human wrote it, it is Space Grotesk. If a machine produced it or a machine will consume it — a coordinate, a percentage, a `d=` string, a cargo command, a license identifier — it is JetBrains Mono. There is no aesthetic use of mono in this system.

**The Small-Type Rule.** 12px is a legitimate primary size here, and every gray in this system clears 4.5:1 at 12px — that is why the ramp starts at Slate Quiet (#4b5563) and not at #6b7280. Gray 500 and lighter are reserved for icons and rules, never for text.

## Layout

A single centered column at `max-w-7xl` (1280px), gutters of 16px rising to 24px at 640px, with vertical section rhythm of 40px rising to 48px. Sections stack; nothing is full-bleed except the header's bottom rule.

The one structural grid is the studio row: a 12-column grid at ≥1024px splitting **7 columns for the stage / 5 columns for the presets**, collapsing to a single stacked column below that. Preset chips sit in a fixed 2-column grid inside their card at every size. Card padding is 20px → 24px for the preset cards and 24px → 32px for the major panels.

Spacing runs on a 4px base, and the system genuinely uses only a narrow band of it: 8px between inline items (by far the most common), 12px between related controls, 16–24px between groups inside a card, 32–48px between sections. Anything larger is section rhythm; anything smaller is optical correction.

**Responsive behavior is deliberately two-stage.** The build uses `sm:` (640px) in 37 places and `lg:` (1024px) in 3; `md:` and `xl:` are unused. That is a feature, not an oversight: one typographic/spacing step up at 640px, one structural change at 1024px. Ambient decoration is suppressed below 640px outright.

Every interactive element carries `min-height: 44px`. This is not negotiable and it constrains the design — 12px labels sit in 44px targets with generous vertical padding, and that padding is where the system's warmth comes from.

Because 44px targets are wide, every horizontal group of controls must be allowed to wrap (`flex-wrap`), and any group label takes its own line below 640px. A control row that cannot wrap will be clipped by its card's `overflow-hidden` and the control becomes unreachable — which is a functional failure, not a layout blemish.

### Named Rules

**The Two-Breakpoint Rule.** Design for 640px and 1024px. If a layout needs a third breakpoint to work, the layout is wrong.

**The 44px Floor Rule.** Nothing clickable is shorter than 44px, including 12px-label pills and icon-only buttons. Pad, don't shrink. When the visual element must stay small — an 8px slider track — grow a transparent hit area around it instead of growing the visual.

**The Feedback-Survives-Reduced-Motion Rule.** `prefers-reduced-motion` stops looping, ambient, and travel-and-fade motion. It does **not** kill the 150ms state transitions on colour, border, and shadow. A blanket `animation-duration: 0.01ms !important` on `*` is forbidden: it removes feedback along with decoration and makes the interface read as broken rather than calm.

## Elevation & Depth

**Flat by default; shadow is a response, not a property.** Surfaces sit directly on the bench, separated by a 1px hairline. Depth appears when the interface is answering the user — hover, focus, selection — and disappears when it isn't. This is why the palette can be almost entirely white without the page collapsing into mush: the hairlines do the structural work that shadows do elsewhere.

### Shadow Vocabulary

- **Card rest** (`box-shadow: 0 2px 6px rgba(17, 19, 23, 0.06)`): Barely there. Enough to lift a white card off a white bench, not enough to read as a shadow. This is the only shadow allowed at rest.
- **Action affirmation** (`box-shadow: 0 4px 14px rgba(21, 128, 61, 0.24)`): A green-tinted shadow under primary buttons. The shadow inherits the accent's hue, so the button appears to be emitting rather than casting.
- **State lift** (`shadow-sm` / `shadow-md` / `shadow-lg`): Applied on selection to chips, spring pills, and tabs. The selected item lifts; its siblings stay flat.
- **Specimen glow** (`drop-shadow(0 0 30px rgba(22, 163, 74, 0.3))`): The one signature effect, on the morphing SVG itself. The specimen under the light. It is a glow, not a shadow, and it exists nowhere else.

### Named Rules

**The Flat-At-Rest Rule.** A surface nobody is interacting with casts nothing beyond the 2px/6% card shadow. If you find yourself adding a resting shadow to create hierarchy, use a hairline or space instead.

**The Singular Glow Rule.** The specimen glow belongs to the morphing icon alone. The moment a second element glows, neither one means anything. The stage card is an ordinary card: hairline, 16px, resting shadow. Its authority comes from size and position, never from a heavier shadow than its neighbours.

## Shapes

Corners are consistently soft but never pill-round except where roundness is semantic. The vocabulary is four steps: **8px** for pills and small controls (preset chips, spring pills, the copy button), **12px** for standard controls (buttons, tabs, the language select, the install panel), **16px** for cards and code panels, and **fully round** for badges, status dots, and the progress track.

Borders are the defining form element: 1px, always, in a near-white gray. Nothing in this system is defined by a heavy stroke or a filled block except the primary button and the code panel.

The iconography has its own hard-set language, and it matters more here than in most systems because icons are the product: **24×24 viewBox, `fill="none"`, `stroke="currentColor"`, `stroke-width="2"`, `stroke-linecap="round"`, `stroke-linejoin="round"`.** Every icon on every surface obeys this — it is also the exact contract the morphing engine requires of input icons, so a filled or non-stroke icon anywhere in the UI is both an aesthetic break and a statement the engine contradicts.

### Named Rules

**The Two-Hue Code Rule.** Syntax highlighting uses green for structure and amber for literals, and nothing else. Adding a third hue to a code block reintroduces the rainbow the rest of the system spent its discipline avoiding. Highlighting is generated by `pages/syntax.js` — no highlight.js, no Prism, no CDN, because the product ships zero runtime dependencies and its own showcase must not be the exception.

**The Four Radii Rule.** 8, 12, 16, full. There is no 4px, no 20px, no 24px. Cards render at 16px via `.glass-card`, pills at 8px via `.glass-pill`; markup must not restate a radius those classes already own.

**The Stroke Contract Rule.** Every icon is stroke-based at 2px with round caps and joins on a 24×24 viewport. Nothing filled, ever. The product cannot morph fill-based icons; the interface must not display what the engine cannot accept.

## Components

### Buttons

- **Shape:** Softly rounded (12px), minimum 44px tall, horizontal padding 14–20px.
- **Primary:** Kinetic Green fill (#15803d), white 12px bold label, with a green-tinted shadow (`0 4px 14px rgba(21,128,61,0.24)`). Icon + label, 6px gap, icon at 16–20px.
- **Hover / Focus:** Fill *darkens* to #166534 over 200ms; focus shows the global 2px #15803d ring at 2px offset. No transform, no scale — calibrated, not bouncy.
- **Secondary:** White fill, 1px Field Gray border, Slate Mid label. On hover the label darkens to Graphite and the border shifts to a light green (#4ade80). Used for "Read the Docs", "crates.io".
- **Icon button on dark:** Sits on the graphite code panel — `#374151` fill, `#4b5563` border, `#d1d5db` label, brightening on hover. The only button variant that lives on a dark surface.

### Chips

- **Preset chip:** White, 1px hairline, 8px radius, 12px semibold Slate Mid label left-aligned with a 14px chevron-right marker at 50% opacity right-aligned, `justify-content: space-between`, 44px tall. The marker inherits `currentColor` so it reads correctly on both the white and the green-filled state. Hover darkens the label to Graphite and greens the border.
- **Selected:** Solid Kinetic Green fill, white label, 1px green border, `shadow-lg`. The border width is explicit on both states so the corner and edge never shift on selection. Exactly one chip is selected at a time across both categories.
- **Category variance:** None. Both preset categories use the same chip and the same green; they are told apart by their heading, their icon, and their contents — not by a second hue.

### Segmented Controls (Spring Settings, Framework Tabs)

- **Spring pills:** A `role="radiogroup"` of three. Rest is white with a hairline and Slate Quiet label; selected is Kinetic Green fill, white label, `shadow-md`. 8px radius, 44px tall.
- **Framework tabs:** A `role="tablist"` of five inside a `#f3f4f6` tray with a 1px border and 16px radius, 6px inner padding. Rest is transparent with a Slate Quiet label; selected is Kinetic Green fill, white label, `shadow-sm`, 12px radius.
- Both express the same idea with different housings: **the selected item is the only filled thing in the group.**

### Cards / Containers

- **Corner Style:** 16px.
- **Background:** Bench White, flat.
- **Border:** 1px Bench Hairline (#e5e8ec).
- **Shadow:** `0 2px 6px rgba(17,19,23,0.06)` — the resting shadow, and the only one (see Elevation).
- **Internal Padding:** 20→24px for secondary cards, 24→32px for primary panels. Internal vertical rhythm `space-y` 16–32px.

### Inputs / Fields

- **Language picker:** Chrome, not a control. No border, no fill: a 16px globe, the two-letter code, and a 12px chevron at 60% opacity, in Slate Quiet, inside a 44px target. Hover fills with Bench Tray and darkens the label to Graphite. The native `<select>` sits invisibly on top, so the OS picker and assistive technology behave normally and the focus ring is drawn on the face via `:focus-within`. Identical on every page, backed by one stored preference.
- **Range slider (spring progress):** An 8px-tall Gray 200 track inside a **44px-tall transparent hit area** (`.range-control`), with a 20px white-ringed Kinetic Green thumb. `accent-color` is not used, because it cannot separate the visual track from the touch target. Paired with a right-aligned mono percentage readout in Kinetic Green. The label and readout sit above the track in a `space-between` row.
- **Focus (global):** `outline: 2px solid #15803d; outline-offset: 2px; border-radius: 8px` on `:focus-visible`. One ring, everywhere, no exceptions. The radius is 8px so the ring stays inside the Four Radii Rule.

### Navigation

- Sticky header at 97%-opaque white with a 1px bottom rule, 14px vertical padding, no shadow.
- Left cluster: the 44px gear logo (slow 20s rotation), the wordmark at Title weight, a mono license chip in green wash, and a Slate Quiet tagline that hides below 640px.
- Right cluster: the language select and a primary-green GitHub button whose label collapses to icon-only below 640px.
- A visually-hidden skip link precedes everything and reveals on focus as a green pill at the top-left.

### The Stage (signature component)

The bench's centerpiece and the thing every other decision defers to. A recessed well — Bench Tray fill, 1px Rule Gray border, 16px radius, minimum 260→300px tall — containing the morphing SVG at 112→144px, in Kinetic Green, wearing the specimen glow. The whole well is a `role="button"` with `tabindex="0"` that toggles the morph on click, Enter, or Space, and lifts 1% on hover over 300ms.

Above it: a `tracking-widest` uppercase eyebrow with a pulsing green dot, and an `aria-live="polite"` mono badge naming the current pair. Below it: a bold Slate Quiet instruction line that names the *next* target ("Click or press Space to morph → Moon") and darkens on hover. The stage announces its own state in three registers — visual, textual, and to a screen reader — because it is the only element on the page that is genuinely dynamic.

### Status & Feedback

- **Live dot:** An 8px fully-round Kinetic Green Bright dot with a slow pulse, prefixed to any label describing something running.
- **Badge / pill:** Green wash fill, green edge border, fully round, 4px/12px padding, 12px mono label in Kinetic Green (4.79:1 on the wash).
- **Static-mode notice:** Caution Wash fill, Caution Edge border, Caution Ink text, 12px radius, centered, inserted above the studio when the WASM engine is unavailable. It informs without alarming — the page still works.
- **Code panel:** Graphite background, `#1f2937` border, 16px radius, 24px padding, `#e5e7eb` mono text, horizontal overflow scroll with a custom 6px scrollbar (Bench Tray track, `#c2c7cf` thumb, fully round).

### Motion

Every transition in the system uses the spring curve **`cubic-bezier(0.34, 1.56, 0.64, 1)`** — an overshoot easing that mirrors the damped oscillator inside the product. Section reveals run 600ms with a 150ms stagger; entrance animations 600–800ms; the logo gear rotates once per 20s; live dots pulse; the static fallback crossfades two icons on a 4s loop. State transitions (color, border, shadow) run 200–300ms. The stage SVG itself transitions at 75ms because it is being driven frame-by-frame by the engine and must not lag behind it.

`prefers-reduced-motion: reduce` collapses every animation and transition to 0.01ms globally, and the static fallback switches from a crossfade to showing both icons side by side. Motion is never the only carrier of information.

### Named Rules

**The Same Physics Rule.** The interface eases the way the engine eases. `cubic-bezier(0.34, 1.56, 0.64, 1)` is the house curve; a linear or ease-in-out transition on a meaningful state change is off-brand in a literal sense.

**The Quiet Chrome Rule.** Controls that switch context rather than act — the language picker, and anything like it — carry no border and no fill at rest. They are wayfinding, not calls to action, and a bordered pill next to a primary button reads as competition. Give them the 44px target and the hover fill; withhold the weight.

**The One Selected Thing Rule.** In every group — presets, spring configs, framework tabs — exactly one member is filled and lifted, and the rest are flat and white. Never zero, never two.

## Do's and Don'ts

### Do:

- **Do** treat Kinetic Green (#15803d) as a state color. It marks live, selected, and in-motion. Reach for weight, size, or space when something merely needs emphasis.
- **Do** separate surfaces with a 1px hairline (#e5e8ec) and let them sit flat. The resting shadow is `0 2px 6px rgba(17,19,23,0.06)` and nothing heavier.
- **Do** cap font weight at 700 (`font-bold`). Space Grotesk has nothing above it and the project loads nothing above it.
- **Do** set every icon at 24×24, `fill="none"`, `stroke-width="2"`, round caps and joins — the same contract the engine requires of input icons.
- **Do** use JetBrains Mono for every number, path, command, and machine-readable label, and Space Grotesk for everything a person wrote.
- **Do** hold the 44px minimum on every interactive element, and get there with padding rather than by growing the 12px label.
- **Do** design at two breakpoints — 640px and 1024px — and make each one earn its change.
- **Do** ship every state in all nine languages and verify the layout survives CJK and Cyrillic (see PRODUCT.md).
- **Do** give every state a non-motion carrier: text, color, and an ARIA announcement, so `prefers-reduced-motion` loses nothing but the animation.

### Don't:

- **Don't** reintroduce the dark neon world. `src/input.css` and `tailwind.config.js` once described a `#09090b` ground with `rgba(15,23,42,0.75)` backdrop-blur glass and a `0 0 40px rgba(59,130,246,0.4)` blue bloom; both have been rewritten to the bench. Nothing in this system is dark except a code panel, and no surface uses `backdrop-filter`.
- **Don't** use emoji or Unicode glyphs as UI vocabulary. This is a page whose entire product is icons; every marker, category badge, caret, checkmark, and status symbol is a drawn stroke icon obeying the Stroke Contract Rule. A `⟷` inside a pair name is text, not an icon, and is the one permitted exception.
- **Don't** add a second accent family. There is one green ramp and it does every job — category distinction included. Emerald is not part of this system.
- **Don't** let a second element glow, and don't give the stage card a heavier shadow than the cards beside it. The specimen glow is the morphing SVG's alone; every card in the system shares one resting shadow.
- **Don't** reach for gradient-and-glow SaaS energy — mesh backgrounds, floating orbs, purple-blue washes, oversized bloom. The `.ambient-orb` rule left in the stylesheet has no markup using it; leave it that way.
- **Don't** write shouty. No `font-extrabold`, no exclamation copy, no ALL-CAPS beyond 12px eyebrows and control labels at `0.05em`.
- **Don't** invent a fifth radius. 8, 12, 16, full.
- **Don't** put a dark surface anywhere except a code panel.
- **Don't** assume bold rescues small text. WCAG counts text as "large" only at ≥24px, or ≥18.66px when bold — 12px bold is normal text and needs the full 4.5:1. This is why #16a34a (3.30:1 on white) cannot carry text at any weight.
- **Don't** invent a bespoke active treatment for a toggle. Every on/selected control in this system resolves to the same thing: solid Kinetic Green fill, white label, 1px border, `shadow-md`. Status dots inside a control inherit `currentColor` so they stay legible in both states.
