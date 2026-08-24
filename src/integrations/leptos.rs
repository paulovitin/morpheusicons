#[cfg(feature = "leptos")]
use leptos::prelude::*;

#[cfg(feature = "leptos")]
use crate::icons::Icon;

/// Reactive Leptos component for rendering a MorpheusIcons SVG morphing path.
///
/// Fully compatible with Leptos 0.7+ and Tailwind CSS UI component libraries like `rust-ui/ui`.
///
/// # Example with Leptos / rust-ui
/// ```ignore
/// use leptos::prelude::*;
/// use morpheusicons::integrations::leptos::MorphIcon;
///
/// #[component]
/// pub fn PlayPauseButton() -> impl IntoView {
///     let (path_d, set_path_d) = signal("M...".to_string());
///
///     view! {
///         <button class="p-2 rounded-md hover:bg-accent">
///             <MorphIcon d=path_d class="w-6 h-6 text-foreground" />
///         </button>
///     }
/// }
/// ```
#[cfg(feature = "leptos")]
#[component]
pub fn MorphIcon(
    /// Reactive path string `d="..."` (e.g. from `controller.current_svg_path()`)
    #[prop(into)]
    d: Signal<String>,
    /// Icon width and height in pixels or units (default: 24.0)
    #[prop(optional, into)]
    size: Option<f32>,
    /// Stroke width (default: 2.0)
    #[prop(optional, into)]
    stroke_width: Option<f32>,
    /// Stroke color (default: "currentColor")
    #[prop(optional, into)]
    color: Option<String>,
    /// Extra CSS / Tailwind class names (e.g., "w-6 h-6 text-primary transition-all")
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let size = size.unwrap_or(24.0);
    let stroke_width = stroke_width.unwrap_or(2.0);
    let color = color.unwrap_or_else(|| "currentColor".to_string());
    let class = class.unwrap_or_default();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke=color
            stroke-width=stroke_width
            stroke-linecap="round"
            stroke-linejoin="round"
            width=size
            height=size
            class=class
        >
            <path d=move || d.get() />
        </svg>
    }
}

/// Helper component to render a static icon from the MorpheusIcons catalog in Leptos.
#[cfg(feature = "leptos")]
#[component]
pub fn StaticIcon(
    /// Icon catalog enum variant
    icon: Icon,
    /// Icon size in pixels (default: 24.0)
    #[prop(optional, into)]
    size: Option<f32>,
    /// Stroke width (default: 2.0)
    #[prop(optional, into)]
    stroke_width: Option<f32>,
    /// Stroke color (default: "currentColor")
    #[prop(optional, into)]
    color: Option<String>,
    /// Extra CSS / Tailwind class names
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let d = icon.path_data();
    let size = size.unwrap_or(24.0);
    let stroke_width = stroke_width.unwrap_or(2.0);
    let color = color.unwrap_or_else(|| "currentColor".to_string());
    let class = class.unwrap_or_default();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke=color
            stroke-width=stroke_width
            stroke-linecap="round"
            stroke-linejoin="round"
            width=size
            height=size
            class=class
        >
            <path d=d />
        </svg>
    }
}
