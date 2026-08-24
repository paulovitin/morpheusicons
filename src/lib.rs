//! # MorpheusIcons
//!
//! A universal, physics-based SVG morphing icon library for Rust UI frameworks.
//! Inspired by [morphicons](https://github.com/guillermolg00/morphicons), MorpheusIcons allows any
//! vector stroke icon to morph fluidly into any other icon using 2D Procrustes analysis, polar
//! interpolation, and spring physics.
//!
//! Works seamlessly with **GPUI**, **egui**, **Leptos**, **Dioxus**, **Iced**, **Slint**, **Vello**, or any standard SVG renderer.
//!
//! ## Quick Start
//!
//! ```rust
//! use morpheusicons::prelude::*;
//!
//! fn main() -> Result<(), String> {
//!     // Create a morph controller between Play and Pause icons using preset IconPair
//!     let mut controller = IconPair::PlayPause.create_controller(SpringConfig::BOUNCY)?;

//!     // Morph towards target icon (Pause)
//!     controller.morph_to_end();

//!     // Step physics animation loop (e.g. per frame dt = 0.016s)
//!     let is_animating = controller.update(0.016);
//!
//!     // Get interpolated SVG path string d="..." for rendering
//!     let svg_path_d = controller.current_svg_path();
//!     println!("Render SVG path: {svg_path_d}");
//!
//!     Ok(())
//! }
//! ```

pub mod animation;
pub mod geometry;
pub mod icons;
pub mod integrations;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use animation::{
    sampled_icon_to_draw_commands, sampled_icon_to_svg_path, DrawCommand, MorphController,
    PathMorpher, Spring, SpringConfig,
};
pub use geometry::{IconPath, PathSegment, Point, ProcrustesMorphData, SampledIcon, SubPath};
pub use icons::{
    check_icon, check_morph_compatibility, check_path_data, KnownIconLibrary, MorphCompatibility,
};
pub use icons::{extract_path_from_svg, icon_from_svg, SvgExtraction};
pub use icons::{Icon, IconPair};
pub use icons::{
    IconSource, RawIcon, ValidationError, ValidationResult, ValidationWarning, Viewport,
};
pub use integrations::SvgRenderer;

/// Convenient prelude module for easy importing.
pub mod prelude {
    pub use crate::animation::{DrawCommand, MorphController, PathMorpher, Spring, SpringConfig};
    pub use crate::geometry::{IconPath, Point, SampledIcon};
    pub use crate::icons::{extract_path_from_svg, icon_from_svg};
    pub use crate::icons::{Icon, IconPair};
    pub use crate::icons::{IconSource, RawIcon, Viewport};
    pub use crate::integrations::SvgRenderer;

    #[cfg(feature = "dioxus")]
    pub use crate::integrations::dioxus::MorphIcon as DioxusMorphIcon;
    #[cfg(feature = "gpui")]
    pub use crate::integrations::gpui::MorpheusGpui;
    #[cfg(feature = "iced")]
    pub use crate::integrations::iced::MorpheusIced;
    #[cfg(feature = "leptos")]
    pub use crate::integrations::leptos::MorphIcon as LeptosMorphIcon;
}
