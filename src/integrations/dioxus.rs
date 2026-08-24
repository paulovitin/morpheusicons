#[cfg(feature = "dioxus")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus")]
use crate::icons::Icon;

/// Reactive Dioxus component for rendering a MorpheusIcons SVG morphing path.
///
/// Designed for Dioxus 0.6+ and cross-platform UI frameworks.
#[cfg(feature = "dioxus")]
#[component]
pub fn MorphIcon(
    /// SVG path string `d="..."` (e.g. from `controller.current_svg_path()`)
    d: String,
    /// Icon width and height in pixels or units (default: 24.0)
    size: Option<f32>,
    /// Stroke width (default: 2.0)
    stroke_width: Option<f32>,
    /// Stroke color (default: "currentColor")
    color: Option<String>,
    /// Extra CSS / Tailwind class names
    class: Option<String>,
) -> Element {
    let size = size.unwrap_or(24.0);
    let stroke_width = stroke_width.unwrap_or(2.0);
    let color = color.unwrap_or_else(|| "currentColor".to_string());
    let class = class.unwrap_or_default();

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "{stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            width: "{size}",
            height: "{size}",
            class: "{class}",
            path { d: "{d}" }
        }
    }
}

/// Helper component to render a static icon from the MorpheusIcons catalog in Dioxus.
#[cfg(feature = "dioxus")]
#[component]
pub fn StaticIcon(
    /// Icon catalog enum variant
    icon: Icon,
    /// Icon size in pixels (default: 24.0)
    size: Option<f32>,
    /// Stroke width (default: 2.0)
    stroke_width: Option<f32>,
    /// Stroke color (default: "currentColor")
    color: Option<String>,
    /// Extra CSS / Tailwind class names
    class: Option<String>,
) -> Element {
    let d = icon.path_data();
    let size = size.unwrap_or(24.0);
    let stroke_width = stroke_width.unwrap_or(2.0);
    let color = color.unwrap_or_else(|| "currentColor".to_string());
    let class = class.unwrap_or_default();

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "{stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            width: "{size}",
            height: "{size}",
            class: "{class}",
            path { d: "{d}" }
        }
    }
}
