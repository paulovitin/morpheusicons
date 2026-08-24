#[cfg(feature = "dioxus")]
pub mod dioxus;
#[cfg(feature = "egui")]
pub mod egui;
#[cfg(feature = "gpui")]
pub mod gpui;
#[cfg(feature = "iced")]
pub mod iced;
#[cfg(feature = "leptos")]
pub mod leptos;
pub mod svg;

pub use svg::SvgRenderer;
