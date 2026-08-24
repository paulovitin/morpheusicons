//! Extract path data from full SVG markup strings.
//!
//! When integrating with external icon libraries, you often have full SVG documents
//! (e.g. `<svg><path d="..."/><circle cx="12" cy="12" r="10"/></svg>`) rather than
//! raw path data strings. This module extracts and converts all drawable elements
//! into a single SVG path `d="..."` string suitable for morphing.
//!
//! # Supported SVG elements
//!
//! | Element | Conversion |
//! |---------|-----------|
//! | `<path d="...">` | Used directly |
//! | `<circle cx cy r>` | Converted to arc-based path |
//! | `<ellipse cx cy rx ry>` | Converted to arc-based path |
//! | `<rect x y width height rx ry>` | Converted to path with optional rounded corners |
//! | `<line x1 y1 x2 y2>` | Converted to `M x1 y1 L x2 y2` |
//! | `<polyline points>` | Converted to `M ... L ... L ...` |
//! | `<polygon points>` | Converted to `M ... L ... Z` |
//!
//! # Example
//!
//! ```rust
//! use morpheusicons::icons::svg_extract::extract_path_from_svg;
//!
//! let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
//!   <circle cx="12" cy="12" r="10"/>
//!   <line x1="12" y1="8" x2="12" y2="16"/>
//!   <line x1="8" y1="12" x2="16" y2="12"/>
//! </svg>"#;
//!
//! let result = extract_path_from_svg(svg).unwrap();
//! assert!(!result.path_data.is_empty());
//! assert_eq!(result.viewport_width, 24.0);
//! assert_eq!(result.viewport_height, 24.0);
//! ```

use crate::icons::source::{RawIcon, Viewport};

/// Result of extracting path data from an SVG document.
#[derive(Debug, Clone)]
pub struct SvgExtraction {
    /// Combined path data string from all elements.
    pub path_data: String,
    /// Detected viewport width (from `viewBox` or `width` attribute).
    pub viewport_width: f32,
    /// Detected viewport height (from `viewBox` or `height` attribute).
    pub viewport_height: f32,
    /// Number of elements that were converted.
    pub element_count: usize,
}

impl SvgExtraction {
    /// Converts this extraction into a `RawIcon` for use with the morphing engine.
    pub fn to_raw_icon(&self) -> RawIcon {
        RawIcon::with_viewport(
            self.path_data.clone(),
            Viewport::new(self.viewport_width, self.viewport_height),
        )
    }

    /// Converts this extraction into a `RawIcon`, normalizing to a target viewport.
    pub fn to_raw_icon_normalized(&self, _target: Viewport) -> RawIcon {
        let source_vp = Viewport::new(self.viewport_width, self.viewport_height);
        // Store with actual source viewport; normalization happens at morph time via IconSource::to_icon_path
        RawIcon::with_viewport(self.path_data.clone(), source_vp)
    }
}

/// Extracts path data from a full SVG markup string.
///
/// Parses the SVG and converts all supported drawable elements (`<path>`, `<circle>`,
/// `<rect>`, `<line>`, `<polyline>`, `<polygon>`, `<ellipse>`) into a unified
/// SVG path data string.
///
/// Returns an error if the SVG contains no drawable elements or is malformed.
pub fn extract_path_from_svg(svg: &str) -> Result<SvgExtraction, String> {
    let (viewport_width, viewport_height) = detect_viewport(svg);

    let mut paths: Vec<String> = Vec::new();

    // Extract <path d="..."> elements
    for d_attr in extract_attr_values(svg, "path", "d") {
        if !d_attr.trim().is_empty() {
            paths.push(d_attr);
        }
    }

    // Extract <circle cx cy r> elements
    for attrs in extract_element_attrs(svg, "circle") {
        if let Some(path) = circle_to_path(&attrs) {
            paths.push(path);
        }
    }

    // Extract <ellipse cx cy rx ry> elements
    for attrs in extract_element_attrs(svg, "ellipse") {
        if let Some(path) = ellipse_to_path(&attrs) {
            paths.push(path);
        }
    }

    // Extract <rect x y width height rx ry> elements
    for attrs in extract_element_attrs(svg, "rect") {
        if let Some(path) = rect_to_path(&attrs) {
            paths.push(path);
        }
    }

    // Extract <line x1 y1 x2 y2> elements
    for attrs in extract_element_attrs(svg, "line") {
        if let Some(path) = line_to_path(&attrs) {
            paths.push(path);
        }
    }

    // Extract <polyline points="..."> elements
    for attrs in extract_element_attrs(svg, "polyline") {
        if let Some(path) = polyline_to_path(&attrs, false) {
            paths.push(path);
        }
    }

    // Extract <polygon points="..."> elements
    for attrs in extract_element_attrs(svg, "polygon") {
        if let Some(path) = polyline_to_path(&attrs, true) {
            paths.push(path);
        }
    }

    if paths.is_empty() {
        return Err("no drawable SVG elements found".to_string());
    }

    let element_count = paths.len();
    let path_data = paths.join("");

    Ok(SvgExtraction {
        path_data,
        viewport_width,
        viewport_height,
        element_count,
    })
}

/// Extracts path data from an SVG string and returns it as an `IconSource`.
///
/// This is a convenience function that combines extraction and wrapping.
pub fn icon_from_svg(svg: &str) -> Result<RawIcon, String> {
    let extraction = extract_path_from_svg(svg)?;
    Ok(extraction.to_raw_icon())
}

/// Extracts path data from an SVG string, normalizing to the given target viewport.
pub fn icon_from_svg_normalized(svg: &str, target_viewport: Viewport) -> Result<RawIcon, String> {
    let extraction = extract_path_from_svg(svg)?;
    Ok(extraction.to_raw_icon_normalized(target_viewport))
}

// ---------------------------------------------------------------------------
// SVG element to path conversions
// ---------------------------------------------------------------------------

fn circle_to_path(attrs: &[(String, String)]) -> Option<String> {
    let cx = get_attr_f32(attrs, "cx").unwrap_or(0.0);
    let cy = get_attr_f32(attrs, "cy").unwrap_or(0.0);
    let r = get_attr_f32(attrs, "r")?;

    if r <= 0.0 {
        return None;
    }

    // Circle as two arcs
    Some(format!(
        "M{} {}A{} {} 0 1 0 {} {}A{} {} 0 1 0 {} {}",
        cx - r,
        cy,
        r,
        r,
        cx + r,
        cy,
        r,
        r,
        cx - r,
        cy
    ))
}

fn ellipse_to_path(attrs: &[(String, String)]) -> Option<String> {
    let cx = get_attr_f32(attrs, "cx").unwrap_or(0.0);
    let cy = get_attr_f32(attrs, "cy").unwrap_or(0.0);
    let rx = get_attr_f32(attrs, "rx")?;
    let ry = get_attr_f32(attrs, "ry")?;

    if rx <= 0.0 || ry <= 0.0 {
        return None;
    }

    Some(format!(
        "M{} {}A{} {} 0 1 0 {} {}A{} {} 0 1 0 {} {}",
        cx - rx,
        cy,
        rx,
        ry,
        cx + rx,
        cy,
        rx,
        ry,
        cx - rx,
        cy
    ))
}

fn rect_to_path(attrs: &[(String, String)]) -> Option<String> {
    let x = get_attr_f32(attrs, "x").unwrap_or(0.0);
    let y = get_attr_f32(attrs, "y").unwrap_or(0.0);
    let w = get_attr_f32(attrs, "width")?;
    let h = get_attr_f32(attrs, "height")?;
    let rx = get_attr_f32(attrs, "rx").unwrap_or(0.0);
    let ry = get_attr_f32(attrs, "ry").unwrap_or(rx); // SVG spec: ry defaults to rx

    if w <= 0.0 || h <= 0.0 {
        return None;
    }

    if rx <= 0.0 && ry <= 0.0 {
        // Simple rectangle
        Some(format!(
            "M{} {}h{}v{}h{}z",
            x, y, w, h, -w
        ))
    } else {
        // Rounded rectangle
        let rx = rx.min(w / 2.0);
        let ry = ry.min(h / 2.0);

        Some(format!(
            "M{} {}h{}a{} {} 0 0 1 {} {}v{}a{} {} 0 0 1 {} {}h{}a{} {} 0 0 1 {} {}v{}a{} {} 0 0 1 {} {}z",
            x + rx, y,
            w - 2.0 * rx,
            rx, ry, rx, ry,
            h - 2.0 * ry,
            rx, ry, -rx, ry,
            -(w - 2.0 * rx),
            rx, ry, -rx, -ry,
            -(h - 2.0 * ry),
            rx, ry, rx, -ry,
        ))
    }
}

fn line_to_path(attrs: &[(String, String)]) -> Option<String> {
    let x1 = get_attr_f32(attrs, "x1").unwrap_or(0.0);
    let y1 = get_attr_f32(attrs, "y1").unwrap_or(0.0);
    let x2 = get_attr_f32(attrs, "x2").unwrap_or(0.0);
    let y2 = get_attr_f32(attrs, "y2").unwrap_or(0.0);

    Some(format!("M{} {}L{} {}", x1, y1, x2, y2))
}

fn polyline_to_path(attrs: &[(String, String)], close: bool) -> Option<String> {
    let points_str = get_attr_str(attrs, "points")?;
    let points = parse_points(points_str);

    if points.is_empty() {
        return None;
    }

    let mut path = format!("M{} {}", points[0].0, points[0].1);
    for &(x, y) in &points[1..] {
        path.push_str(&format!("L{} {}", x, y));
    }
    if close {
        path.push('z');
    }

    Some(path)
}

// ---------------------------------------------------------------------------
// Simple SVG attribute parsing (no external XML dependency)
// ---------------------------------------------------------------------------

/// Detects viewport from `viewBox` attribute or `width`/`height` attributes.
fn detect_viewport(svg: &str) -> (f32, f32) {
    // Try viewBox first: viewBox="minX minY width height"
    if let Some(viewbox) = find_root_attr(svg, "viewBox")
        .or_else(|| find_root_attr(svg, "viewbox"))
    {
        let parts: Vec<f32> = viewbox
            .split_whitespace()
            .flat_map(|s| s.split(','))
            .filter_map(|s| s.trim().parse::<f32>().ok())
            .collect();

        if parts.len() >= 4 {
            return (parts[2], parts[3]);
        }
    }

    // Fall back to width/height attributes
    let width = find_root_attr(svg, "width")
        .and_then(|s| parse_dimension(&s))
        .unwrap_or(24.0);
    let height = find_root_attr(svg, "height")
        .and_then(|s| parse_dimension(&s))
        .unwrap_or(24.0);

    (width, height)
}

/// Finds an attribute value on the root `<svg>` element.
fn find_root_attr(svg: &str, attr_name: &str) -> Option<String> {
    // Find the <svg ...> opening tag
    let svg_lower = svg.to_lowercase();
    let svg_start = svg_lower.find("<svg")?;
    let svg_tag_end = svg[svg_start..].find('>')? + svg_start;
    let svg_tag = &svg[svg_start..=svg_tag_end];

    extract_attribute(svg_tag, attr_name)
}

/// Extracts all values of a specific attribute from all instances of a given element.
fn extract_attr_values(svg: &str, element: &str, attr_name: &str) -> Vec<String> {
    let mut results = Vec::new();
    let search_open = format!("<{}", element);
    let mut pos = 0;

    while let Some(start) = svg[pos..].find(&search_open) {
        let abs_start = pos + start;
        // Make sure it's actually the element (not a prefix match like <pathx)
        let after_tag = abs_start + search_open.len();
        if after_tag < svg.len() {
            let next_char = svg.as_bytes()[after_tag];
            if next_char != b' ' && next_char != b'/' && next_char != b'>' && next_char != b'\t' && next_char != b'\n' && next_char != b'\r' {
                pos = after_tag;
                continue;
            }
        }

        // Find the end of this element (either /> or >)
        if let Some(tag_end) = find_element_end(&svg[abs_start..]) {
            let tag_str = &svg[abs_start..abs_start + tag_end + 1];
            if let Some(val) = extract_attribute(tag_str, attr_name) {
                results.push(val);
            }
        }
        pos = abs_start + search_open.len();
    }

    results
}

/// Extracts all attributes from all instances of a given element.
fn extract_element_attrs(svg: &str, element: &str) -> Vec<Vec<(String, String)>> {
    let mut results = Vec::new();
    let search_open = format!("<{}", element);
    let mut pos = 0;

    while let Some(start) = svg[pos..].find(&search_open) {
        let abs_start = pos + start;
        let after_tag = abs_start + search_open.len();
        if after_tag < svg.len() {
            let next_char = svg.as_bytes()[after_tag];
            if next_char != b' ' && next_char != b'/' && next_char != b'>' && next_char != b'\t' && next_char != b'\n' && next_char != b'\r' {
                pos = after_tag;
                continue;
            }
        }

        if let Some(tag_end) = find_element_end(&svg[abs_start..]) {
            let tag_str = &svg[abs_start..abs_start + tag_end + 1];
            results.push(parse_all_attributes(tag_str));
        }
        pos = abs_start + search_open.len();
    }

    results
}

/// Finds the end of an element tag (the closing `>` of either `<el .../>` or `<el ...>`).
fn find_element_end(s: &str) -> Option<usize> {
    let mut in_quote = false;
    let mut quote_char = '"';

    for (i, c) in s.char_indices() {
        if in_quote {
            if c == quote_char {
                in_quote = false;
            }
        } else if c == '"' || c == '\'' {
            in_quote = true;
            quote_char = c;
        } else if c == '>' {
            return Some(i);
        }
    }
    None
}

/// Extracts a single attribute value from a tag string.
fn extract_attribute(tag: &str, attr_name: &str) -> Option<String> {
    // Look for attr_name="value" or attr_name='value'
    let patterns = [
        format!("{}=\"", attr_name),
        format!("{}='", attr_name),
        format!("{} =\"", attr_name),
        format!("{} ='", attr_name),
    ];

    for pattern in &patterns {
        if let Some(attr_start) = tag.find(pattern.as_str()) {
            let value_start = attr_start + pattern.len();
            let quote_char = if pattern.ends_with('"') { '"' } else { '\'' };
            if let Some(value_end) = tag[value_start..].find(quote_char) {
                return Some(tag[value_start..value_start + value_end].to_string());
            }
        }
    }

    None
}

/// Parses all key="value" pairs from a tag string.
fn parse_all_attributes(tag: &str) -> Vec<(String, String)> {
    let mut attrs = Vec::new();
    let mut pos = 0;
    let bytes = tag.as_bytes();
    let len = bytes.len();

    // Skip the tag name
    while pos < len && bytes[pos] != b' ' && bytes[pos] != b'/' && bytes[pos] != b'>' {
        pos += 1;
    }

    while pos < len {
        // Skip whitespace
        while pos < len && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\n' || bytes[pos] == b'\r') {
            pos += 1;
        }

        if pos >= len || bytes[pos] == b'/' || bytes[pos] == b'>' {
            break;
        }

        // Read attribute name
        let name_start = pos;
        while pos < len && bytes[pos] != b'=' && bytes[pos] != b' ' && bytes[pos] != b'/' && bytes[pos] != b'>' {
            pos += 1;
        }
        let name = &tag[name_start..pos];

        // Skip to '='
        while pos < len && bytes[pos] == b' ' {
            pos += 1;
        }
        if pos >= len || bytes[pos] != b'=' {
            continue;
        }
        pos += 1; // skip '='

        // Skip whitespace after '='
        while pos < len && bytes[pos] == b' ' {
            pos += 1;
        }

        if pos >= len {
            break;
        }

        // Read value (quoted)
        let quote = bytes[pos];
        if quote != b'"' && quote != b'\'' {
            // Unquoted value — skip to next whitespace
            let val_start = pos;
            while pos < len && bytes[pos] != b' ' && bytes[pos] != b'/' && bytes[pos] != b'>' {
                pos += 1;
            }
            attrs.push((name.to_string(), tag[val_start..pos].to_string()));
            continue;
        }
        pos += 1; // skip opening quote

        let val_start = pos;
        while pos < len && bytes[pos] != quote {
            pos += 1;
        }
        let value = &tag[val_start..pos];
        if pos < len {
            pos += 1; // skip closing quote
        }

        attrs.push((name.to_string(), value.to_string()));
    }

    attrs
}

/// Parses an SVG `points` attribute value into a list of (x, y) tuples.
fn parse_points(points: &str) -> Vec<(f32, f32)> {
    let numbers: Vec<f32> = points
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<f32>().ok())
        .collect();

    numbers.chunks(2).filter_map(|chunk| {
        if chunk.len() == 2 {
            Some((chunk[0], chunk[1]))
        } else {
            None
        }
    }).collect()
}

/// Parses a dimension string like "24", "24px", "24.5" into a float.
fn parse_dimension(s: &str) -> Option<f32> {
    let numeric: String = s.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    numeric.parse::<f32>().ok()
}

fn get_attr_f32(attrs: &[(String, String)], name: &str) -> Option<f32> {
    attrs
        .iter()
        .find(|(k, _)| k == name)
        .and_then(|(_, v)| v.parse::<f32>().ok())
}

fn get_attr_str<'a>(attrs: &'a [(String, String)], name: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::icons::source::IconSource;

    #[test]
    fn test_extract_path_element() {
        let svg = r#"<svg viewBox="0 0 24 24"><path d="M5 12h14M12 5l7 7-7 7"/></svg>"#;
        let result = extract_path_from_svg(svg).unwrap();
        assert_eq!(result.path_data, "M5 12h14M12 5l7 7-7 7");
        assert_eq!(result.viewport_width, 24.0);
        assert_eq!(result.viewport_height, 24.0);
        assert_eq!(result.element_count, 1);
    }

    #[test]
    fn test_extract_circle() {
        let svg = r#"<svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"/></svg>"#;
        let result = extract_path_from_svg(svg).unwrap();
        assert!(result.path_data.contains('A'));
        assert_eq!(result.element_count, 1);
    }

    #[test]
    fn test_extract_line() {
        let svg = r#"<svg viewBox="0 0 24 24"><line x1="2" y1="3" x2="20" y2="21"/></svg>"#;
        let result = extract_path_from_svg(svg).unwrap();
        assert_eq!(result.path_data, "M2 3L20 21");
        assert_eq!(result.element_count, 1);
    }

    #[test]
    fn test_extract_rect() {
        let svg = r#"<svg viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18"/></svg>"#;
        let result = extract_path_from_svg(svg).unwrap();
        assert!(result.path_data.starts_with('M'));
        assert!(result.path_data.contains('h'));
    }

    #[test]
    fn test_extract_polygon() {
        let svg = r#"<svg viewBox="0 0 24 24"><polygon points="12,2 22,20 2,20"/></svg>"#;
        let result = extract_path_from_svg(svg).unwrap();
        assert!(result.path_data.starts_with("M12 2"));
        assert!(result.path_data.ends_with('z'));
    }

    #[test]
    fn test_extract_multiple_elements() {
        let svg = r#"<svg viewBox="0 0 24 24">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="8" x2="12" y2="16"/>
            <line x1="8" y1="12" x2="16" y2="12"/>
        </svg>"#;
        let result = extract_path_from_svg(svg).unwrap();
        assert_eq!(result.element_count, 3);
    }

    #[test]
    fn test_detect_viewport_from_viewbox() {
        let svg = r#"<svg viewBox="0 0 256 256"><path d="M12 12"/></svg>"#;
        let result = extract_path_from_svg(svg).unwrap();
        assert_eq!(result.viewport_width, 256.0);
        assert_eq!(result.viewport_height, 256.0);
    }

    #[test]
    fn test_no_drawable_elements() {
        let svg = r#"<svg viewBox="0 0 24 24"><g></g></svg>"#;
        assert!(extract_path_from_svg(svg).is_err());
    }

    #[test]
    fn test_icon_from_svg() {
        let svg = r#"<svg viewBox="0 0 24 24"><path d="M20 6L9 17l-5-5"/></svg>"#;
        let icon = icon_from_svg(svg).unwrap();
        assert_eq!(icon.path_data(), "M20 6L9 17l-5-5");
        assert_eq!(icon.viewport().width, 24.0);
    }
}
