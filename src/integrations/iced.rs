#[cfg(feature = "iced")]
use iced::widget::svg::{Handle, Svg};

#[cfg(feature = "iced")]
use crate::animation::MorphController;
#[cfg(feature = "iced")]
use crate::icons::Icon;

/// Iced Integration for MorpheusIcons.
///
/// Converts MorpheusIcons morph controllers and vector icons into native Iced `Svg` widgets.
#[cfg(feature = "iced")]
pub struct MorpheusIced;

#[cfg(feature = "iced")]
impl MorpheusIced {
    /// Render an SVG path string `d="..."` as an Iced `Svg` widget.
    pub fn icon_svg<Theme: iced::widget::svg::Catalog>(
        d_path: impl Into<String>,
    ) -> Svg<'static, Theme> {
        let full_svg = crate::integrations::svg::SvgRenderer::render_svg_document(
            &d_path.into(),
            24.0,
            24.0,
            "#3b82f6",
            2.0,
        );
        let handle = Handle::from_memory(full_svg.into_bytes());
        Svg::new(handle)
    }

    /// Renders the current state of a `MorphController` as an Iced `Svg` widget.
    pub fn morph_svg<Theme: iced::widget::svg::Catalog>(
        controller: &MorphController,
    ) -> Svg<'static, Theme> {
        Self::icon_svg(controller.current_svg_path())
    }

    /// Convenience method to create a static Iced icon from the catalog.
    pub fn catalog_svg<Theme: iced::widget::svg::Catalog>(icon: Icon) -> Svg<'static, Theme> {
        Self::icon_svg(icon.path_data())
    }
}
