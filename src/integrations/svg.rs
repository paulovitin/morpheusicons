/// Generic SVG document generator.
pub struct SvgRenderer;

impl SvgRenderer {
    /// Renders an SVG path `d="..."` into a full `<svg>` XML string.
    pub fn render_svg_document(
        d_path: &str,
        viewbox_width: f32,
        viewbox_height: f32,
        stroke_color: &str,
        stroke_width: f32,
    ) -> String {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {viewbox_width} {viewbox_height}" fill="none" stroke="{stroke_color}" stroke-width="{stroke_width}" stroke-linecap="round" stroke-linejoin="round"><path d="{d_path}" /></svg>"#
        )
    }
}
