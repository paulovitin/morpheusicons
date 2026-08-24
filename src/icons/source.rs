use crate::geometry::path::IconPath;

/// Viewport dimensions for an icon coordinate system.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    /// Standard 24×24 viewport used by Lucide, Feather, Heroicons, Tabler, etc.
    pub const STANDARD_24: Self = Self {
        width: 24.0,
        height: 24.0,
    };

    /// 20×20 viewport used by some Heroicons mini variants.
    pub const STANDARD_20: Self = Self {
        width: 20.0,
        height: 20.0,
    };

    /// 16×16 viewport used by Octicons and some small icon sets.
    pub const STANDARD_16: Self = Self {
        width: 16.0,
        height: 16.0,
    };

    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Returns a uniform scale factor to normalize this viewport to a target viewport.
    pub fn scale_to(&self, target: &Viewport) -> f32 {
        let sx = target.width / self.width;
        let sy = target.height / self.height;
        sx.min(sy)
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::STANDARD_24
    }
}

/// Result of validating an icon source for morph compatibility.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the icon is valid for morphing.
    pub is_valid: bool,
    /// Warnings that don't prevent morphing but may affect quality.
    pub warnings: Vec<ValidationWarning>,
    /// Errors that prevent morphing.
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self {
            is_valid: true,
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn with_error(error: ValidationError) -> Self {
        Self {
            is_valid: false,
            warnings: Vec::new(),
            errors: vec![error],
        }
    }
}

/// Warnings that don't block morphing but may produce suboptimal animations.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationWarning {
    /// Icon has many subpaths which may produce complex animations.
    HighSubpathCount { count: usize },
    /// Icon coordinates extend beyond its declared viewport.
    CoordinatesOutOfViewport,
    /// Large difference in subpath count between morph source and target.
    SubpathCountMismatch { source: usize, target: usize },
}

/// Errors that prevent an icon from being used for morphing.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Path data string is empty.
    EmptyPathData,
    /// Path data could not be parsed as valid SVG path commands.
    InvalidPathData { reason: String },
    /// Viewport has zero or negative dimensions.
    InvalidViewport { width: f32, height: f32 },
}

impl core::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyPathData => write!(f, "path data is empty"),
            Self::InvalidPathData { reason } => write!(f, "invalid path data: {reason}"),
            Self::InvalidViewport { width, height } => {
                write!(f, "invalid viewport: {width}×{height}")
            }
        }
    }
}

impl core::fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::HighSubpathCount { count } => {
                write!(f, "high subpath count ({count}), animation may be complex")
            }
            Self::CoordinatesOutOfViewport => {
                write!(f, "some coordinates extend beyond the declared viewport")
            }
            Self::SubpathCountMismatch { source, target } => {
                write!(
                    f,
                    "subpath count mismatch: source has {source}, target has {target}"
                )
            }
        }
    }
}

/// Trait for any source of icon path data that can be used with MorpheusIcons.
///
/// Implement this trait to integrate any icon library (Lucide, Heroicons, Tabler,
/// Phosphor, or your own custom icons) with the morphing engine.
///
/// # Requirements for good morph results
///
/// - Icons should be **stroke-based** (not filled shapes)
/// - Path data must be valid SVG path commands (M, L, C, Q, A, H, V, S, T, Z)
/// - Both icons in a morph pair should share the same viewport size
///
/// # Example: Custom icon
///
/// ```rust
/// use morpheusicons::prelude::*;
///
/// struct MyIcon {
///     d: &'static str,
/// }
///
/// impl IconSource for MyIcon {
///     fn path_data(&self) -> &str {
///         self.d
///     }
///
///     fn viewport(&self) -> Viewport {
///         Viewport::STANDARD_24
///     }
/// }
///
/// let my_check = MyIcon { d: "M20 6L9 17l-5-5" };
/// let my_x = MyIcon { d: "M18 6L6 18M6 6l12 12" };
///
/// let controller = MorphController::from_sources(&my_check, &my_x, SpringConfig::BOUNCY).unwrap();
/// ```
pub trait IconSource {
    /// Returns the SVG path data string (`d="..."` content) for this icon.
    fn path_data(&self) -> &str;

    /// Returns the viewport (coordinate system) this icon is designed for.
    /// Defaults to 24×24 (the most common standard for stroke icon libraries).
    fn viewport(&self) -> Viewport {
        Viewport::STANDARD_24
    }

    /// Validates that this icon is suitable for morphing.
    /// Returns a `ValidationResult` with any warnings or errors.
    fn validate(&self) -> ValidationResult {
        validate_icon_source(self.path_data(), &self.viewport())
    }

    /// Parses the path data into an `IconPath`, optionally normalizing to a target viewport.
    fn to_icon_path(&self, target_viewport: Option<&Viewport>) -> Result<IconPath, String> {
        let path_data = self.path_data();
        if path_data.trim().is_empty() {
            return Err("path data is empty".to_string());
        }

        let icon_path = IconPath::parse(path_data)?;

        match target_viewport {
            Some(target) => {
                let source_vp = self.viewport();
                if (source_vp.width - target.width).abs() < 0.01
                    && (source_vp.height - target.height).abs() < 0.01
                {
                    Ok(icon_path)
                } else {
                    Ok(icon_path.scale(source_vp.scale_to(target)))
                }
            }
            None => Ok(icon_path),
        }
    }
}

/// Validates icon path data for morph compatibility.
pub fn validate_icon_source(path_data: &str, viewport: &Viewport) -> ValidationResult {
    if path_data.trim().is_empty() {
        return ValidationResult::with_error(ValidationError::EmptyPathData);
    }

    if viewport.width <= 0.0 || viewport.height <= 0.0 {
        return ValidationResult::with_error(ValidationError::InvalidViewport {
            width: viewport.width,
            height: viewport.height,
        });
    }

    match IconPath::parse(path_data) {
        Ok(icon_path) => {
            let mut warnings = Vec::new();

            // Check subpath count
            let subpath_count = icon_path.subpaths.len();
            if subpath_count > 8 {
                warnings.push(ValidationWarning::HighSubpathCount {
                    count: subpath_count,
                });
            }

            // Check if coordinates are within viewport bounds (with some tolerance)
            let tolerance = 2.0;
            let mut out_of_bounds = false;
            for subpath in &icon_path.subpaths {
                for seg in &subpath.segments {
                    let points = segment_points(seg);
                    for p in points {
                        if p.x < -tolerance
                            || p.y < -tolerance
                            || p.x > viewport.width + tolerance
                            || p.y > viewport.height + tolerance
                        {
                            out_of_bounds = true;
                            break;
                        }
                    }
                    if out_of_bounds {
                        break;
                    }
                }
                if out_of_bounds {
                    break;
                }
            }
            if out_of_bounds {
                warnings.push(ValidationWarning::CoordinatesOutOfViewport);
            }

            ValidationResult {
                is_valid: true,
                warnings,
                errors: Vec::new(),
            }
        }
        Err(reason) => ValidationResult::with_error(ValidationError::InvalidPathData { reason }),
    }
}

/// Validates that two icon sources are compatible for morphing together.
pub fn validate_morph_pair(
    source: &dyn IconSource,
    target: &dyn IconSource,
) -> ValidationResult {
    let source_result = source.validate();
    if !source_result.is_valid {
        return source_result;
    }

    let target_result = target.validate();
    if !target_result.is_valid {
        return target_result;
    }

    let mut warnings = source_result.warnings;
    warnings.extend(target_result.warnings);

    // Check subpath count difference
    if let (Ok(src_path), Ok(tgt_path)) = (
        IconPath::parse(source.path_data()),
        IconPath::parse(target.path_data()),
    ) {
        let src_count = src_path.subpaths.len();
        let tgt_count = tgt_path.subpaths.len();
        if src_count.abs_diff(tgt_count) > 4 {
            warnings.push(ValidationWarning::SubpathCountMismatch {
                source: src_count,
                target: tgt_count,
            });
        }
    }

    ValidationResult {
        is_valid: true,
        warnings,
        errors: Vec::new(),
    }
}

/// A simple wrapper that implements `IconSource` for any raw path data string.
/// Use this when you have SVG path data from an external icon library.
///
/// # Example
///
/// ```rust
/// use morpheusicons::prelude::*;
///
/// // Using a Lucide icon's path data directly
/// let home = RawIcon::new("M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2zM9 22V12h6v10");
///
/// // Using a Tabler icon with explicit viewport
/// let tabler_icon = RawIcon::with_viewport(
///     "M12 3l8 4.5v9l-8 4.5l-8-4.5v-9l8-4.5",
///     Viewport::STANDARD_24,
/// );
/// ```
#[derive(Debug, Clone)]
pub struct RawIcon {
    path_data: String,
    viewport: Viewport,
}

impl RawIcon {
    /// Creates a new `RawIcon` with the standard 24×24 viewport.
    pub fn new(path_data: impl Into<String>) -> Self {
        Self {
            path_data: path_data.into(),
            viewport: Viewport::STANDARD_24,
        }
    }

    /// Creates a new `RawIcon` with a custom viewport.
    pub fn with_viewport(path_data: impl Into<String>, viewport: Viewport) -> Self {
        Self {
            path_data: path_data.into(),
            viewport,
        }
    }
}

impl IconSource for RawIcon {
    fn path_data(&self) -> &str {
        &self.path_data
    }

    fn viewport(&self) -> Viewport {
        self.viewport
    }
}

/// Implement `IconSource` for string references for maximum ergonomics.
/// Assumes standard 24×24 viewport.
impl IconSource for &str {
    fn path_data(&self) -> &str {
        self
    }

    fn viewport(&self) -> Viewport {
        Viewport::STANDARD_24
    }
}

impl IconSource for String {
    fn path_data(&self) -> &str {
        self.as_str()
    }

    fn viewport(&self) -> Viewport {
        Viewport::STANDARD_24
    }
}

// Helper: extract points from a path segment for validation bounds checking.
fn segment_points(seg: &crate::geometry::path::PathSegment) -> Vec<crate::geometry::point::Point> {
    use crate::geometry::path::PathSegment;
    match seg {
        PathSegment::MoveTo(p) | PathSegment::LineTo(p) => vec![*p],
        PathSegment::CubicTo { ctrl1, ctrl2, end } => vec![*ctrl1, *ctrl2, *end],
        PathSegment::Close => vec![],
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // --- Viewport ---

    #[test]
    fn test_viewport_standard_24() {
        let v = Viewport::STANDARD_24;
        assert_eq!(v.width, 24.0);
        assert_eq!(v.height, 24.0);
    }

    #[test]
    fn test_viewport_standard_20() {
        let v = Viewport::STANDARD_20;
        assert_eq!(v.width, 20.0);
        assert_eq!(v.height, 20.0);
    }

    #[test]
    fn test_viewport_standard_16() {
        let v = Viewport::STANDARD_16;
        assert_eq!(v.width, 16.0);
        assert_eq!(v.height, 16.0);
    }

    #[test]
    fn test_viewport_new() {
        let v = Viewport::new(100.0, 200.0);
        assert_eq!(v.width, 100.0);
        assert_eq!(v.height, 200.0);
    }

    #[test]
    fn test_viewport_default_is_24() {
        let v: Viewport = Default::default();
        assert_eq!(v, Viewport::STANDARD_24);
    }

    #[test]
    fn test_viewport_scale_to_same() {
        let v = Viewport::STANDARD_24;
        let scale = v.scale_to(&Viewport::STANDARD_24);
        assert!((scale - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_viewport_scale_to_double() {
        let v = Viewport::new(12.0, 12.0);
        let scale = v.scale_to(&Viewport::STANDARD_24);
        assert!((scale - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_viewport_scale_to_half() {
        let v = Viewport::new(48.0, 48.0);
        let scale = v.scale_to(&Viewport::STANDARD_24);
        assert!((scale - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_viewport_scale_to_non_uniform() {
        // 24x48 -> 24x24: sx=1.0, sy=0.5 → min = 0.5
        let v = Viewport::new(24.0, 48.0);
        let scale = v.scale_to(&Viewport::STANDARD_24);
        assert!((scale - 0.5).abs() < 1e-6);
    }

    // --- RawIcon ---

    #[test]
    fn test_raw_icon_new() {
        let icon = RawIcon::new("M0 0 L10 10");
        assert_eq!(icon.path_data(), "M0 0 L10 10");
        assert_eq!(icon.viewport(), Viewport::STANDARD_24);
    }

    #[test]
    fn test_raw_icon_with_viewport() {
        let icon = RawIcon::with_viewport("M0 0 L100 100", Viewport::new(256.0, 256.0));
        assert_eq!(icon.path_data(), "M0 0 L100 100");
        assert_eq!(icon.viewport(), Viewport::new(256.0, 256.0));
    }

    #[test]
    fn test_raw_icon_from_string() {
        let path = String::from("M5 5 L10 10");
        let icon = RawIcon::new(path);
        assert_eq!(icon.path_data(), "M5 5 L10 10");
    }

    // --- IconSource for &str ---

    #[test]
    fn test_icon_source_str_path_data() {
        let s: &str = "M0 0 L10 10";
        assert_eq!(s.path_data(), "M0 0 L10 10");
    }

    #[test]
    fn test_icon_source_str_viewport() {
        let s: &str = "M0 0 L10 10";
        assert_eq!(s.viewport(), Viewport::STANDARD_24);
    }

    // --- IconSource for String ---

    #[test]
    fn test_icon_source_string_path_data() {
        let s = String::from("M0 0 L10 10");
        assert_eq!(s.path_data(), "M0 0 L10 10");
    }

    #[test]
    fn test_icon_source_string_viewport() {
        let s = String::from("M0 0 L10 10");
        assert_eq!(s.viewport(), Viewport::STANDARD_24);
    }

    // --- IconSource::validate ---

    #[test]
    fn test_icon_source_validate_valid() {
        let icon = RawIcon::new("M4 6h16M4 12h16M4 18h16");
        let result = icon.validate();
        assert!(result.is_valid);
    }

    #[test]
    fn test_icon_source_validate_empty() {
        let icon = RawIcon::new("");
        let result = icon.validate();
        assert!(!result.is_valid);
        assert!(matches!(result.errors[0], ValidationError::EmptyPathData));
    }

    #[test]
    fn test_icon_source_validate_invalid() {
        let icon = RawIcon::new("NOT_VALID");
        let result = icon.validate();
        assert!(!result.is_valid);
        assert!(matches!(
            result.errors[0],
            ValidationError::InvalidPathData { .. }
        ));
    }

    // --- IconSource::to_icon_path ---

    #[test]
    fn test_to_icon_path_no_target() {
        let icon = RawIcon::new("M0 0 L10 10");
        let path = icon.to_icon_path(None).unwrap();
        assert_eq!(path.subpaths.len(), 1);
    }

    #[test]
    fn test_to_icon_path_same_viewport() {
        let icon = RawIcon::new("M0 0 L10 10");
        let path = icon.to_icon_path(Some(&Viewport::STANDARD_24)).unwrap();
        // Same viewport, no scaling
        assert_eq!(path.subpaths.len(), 1);
    }

    #[test]
    fn test_to_icon_path_different_viewport_scales() {
        let icon = RawIcon::with_viewport("M0 0 L10 10", Viewport::new(48.0, 48.0));
        let path = icon.to_icon_path(Some(&Viewport::STANDARD_24)).unwrap();
        // 48→24 = scale 0.5, so L10 10 → L5 5
        let sp = &path.subpaths[0];
        if let crate::geometry::path::PathSegment::LineTo(p) = &sp.segments[1] {
            assert!((p.x - 5.0).abs() < 1e-4);
            assert!((p.y - 5.0).abs() < 1e-4);
        } else {
            panic!("Expected LineTo");
        }
    }

    #[test]
    fn test_to_icon_path_empty_fails() {
        let icon = RawIcon::new("");
        let result = icon.to_icon_path(None);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_icon_path_whitespace_only_fails() {
        let icon = RawIcon::new("   ");
        let result = icon.to_icon_path(None);
        assert!(result.is_err());
    }

    // --- validate_icon_source ---

    #[test]
    fn test_validate_icon_source_valid() {
        let result = validate_icon_source("M4 6h16M4 12h16M4 18h16", &Viewport::STANDARD_24);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_icon_source_empty() {
        let result = validate_icon_source("", &Viewport::STANDARD_24);
        assert!(!result.is_valid);
        assert!(matches!(result.errors[0], ValidationError::EmptyPathData));
    }

    #[test]
    fn test_validate_icon_source_invalid_viewport() {
        let result = validate_icon_source("M0 0 L10 10", &Viewport::new(0.0, 24.0));
        assert!(!result.is_valid);
        assert!(matches!(
            result.errors[0],
            ValidationError::InvalidViewport { .. }
        ));
    }

    #[test]
    fn test_validate_icon_source_negative_viewport() {
        let result = validate_icon_source("M0 0 L10 10", &Viewport::new(-5.0, -5.0));
        assert!(!result.is_valid);
    }

    #[test]
    fn test_validate_icon_source_out_of_bounds() {
        let result = validate_icon_source("M0 0 L50 50", &Viewport::STANDARD_24);
        assert!(result.is_valid);
        let has_oob = result
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::CoordinatesOutOfViewport));
        assert!(has_oob);
    }

    #[test]
    fn test_validate_icon_source_within_tolerance() {
        // Coords slightly outside viewport but within 2.0 tolerance
        let result = validate_icon_source("M-1 -1 L25 25", &Viewport::STANDARD_24);
        assert!(result.is_valid);
        // -1 > -2.0 tolerance and 25 < 26.0 tolerance → no warning
        let has_oob = result
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::CoordinatesOutOfViewport));
        assert!(!has_oob);
    }

    #[test]
    fn test_validate_icon_source_high_subpath_count() {
        // > 8 subpaths triggers warning
        let path = "M0 0L1 1M2 2L3 3M4 4L5 5M6 6L7 7M8 8L9 9M10 10L11 11M12 12L13 13M14 14L15 15M16 16L17 17";
        let result = validate_icon_source(path, &Viewport::STANDARD_24);
        assert!(result.is_valid);
        let has_high_count = result
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::HighSubpathCount { .. }));
        assert!(has_high_count);
    }

    // --- validate_morph_pair ---

    #[test]
    fn test_validate_morph_pair_valid() {
        let src = RawIcon::new("M4 6h16M4 12h16M4 18h16");
        let tgt = RawIcon::new("M18 6L6 18M6 6l12 12");
        let result = validate_morph_pair(&src, &tgt);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_morph_pair_invalid_source() {
        let src = RawIcon::new("");
        let tgt = RawIcon::new("M0 0 L10 10");
        let result = validate_morph_pair(&src, &tgt);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_validate_morph_pair_invalid_target() {
        let src = RawIcon::new("M0 0 L10 10");
        let tgt = RawIcon::new("INVALID");
        let result = validate_morph_pair(&src, &tgt);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_validate_morph_pair_subpath_mismatch_warning() {
        // Source has 9 subpaths, target has 1 → diff > 4
        let src_path = "M0 0L1 1M2 2L3 3M4 4L5 5M6 6L7 7M8 8L9 9M10 10L11 11M12 12L13 13M14 14L15 15M16 16L17 17";
        let src = RawIcon::new(src_path);
        let tgt = RawIcon::new("M0 0 L10 10");
        let result = validate_morph_pair(&src, &tgt);
        assert!(result.is_valid);
        let has_mismatch = result
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::SubpathCountMismatch { .. }));
        assert!(has_mismatch);
    }

    #[test]
    fn test_validate_morph_pair_no_mismatch_small_diff() {
        // 3 vs 2 subpaths: diff = 1, which is <= 4
        let src = RawIcon::new("M4 6h16M4 12h16M4 18h16"); // 3
        let tgt = RawIcon::new("M18 6L6 18M6 6l12 12"); // 2
        let result = validate_morph_pair(&src, &tgt);
        let has_mismatch = result
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::SubpathCountMismatch { .. }));
        assert!(!has_mismatch);
    }

    // --- ValidationResult ---

    #[test]
    fn test_validation_result_ok() {
        let r = ValidationResult::ok();
        assert!(r.is_valid);
        assert!(r.warnings.is_empty());
        assert!(r.errors.is_empty());
    }

    #[test]
    fn test_validation_result_with_error() {
        let r = ValidationResult::with_error(ValidationError::EmptyPathData);
        assert!(!r.is_valid);
        assert_eq!(r.errors.len(), 1);
    }

    // --- ValidationError Display ---

    #[test]
    fn test_validation_error_display() {
        let e = ValidationError::EmptyPathData;
        assert_eq!(format!("{e}"), "path data is empty");

        let e = ValidationError::InvalidPathData {
            reason: "bad command".to_string(),
        };
        assert!(format!("{e}").contains("bad command"));

        let e = ValidationError::InvalidViewport {
            width: 0.0,
            height: 24.0,
        };
        assert!(format!("{e}").contains("0"));
    }

    // --- ValidationWarning Display ---

    #[test]
    fn test_validation_warning_display() {
        let w = ValidationWarning::HighSubpathCount { count: 12 };
        assert!(format!("{w}").contains("12"));

        let w = ValidationWarning::CoordinatesOutOfViewport;
        assert!(!format!("{w}").is_empty());

        let w = ValidationWarning::SubpathCountMismatch {
            source: 3,
            target: 9,
        };
        assert!(format!("{w}").contains("3"));
        assert!(format!("{w}").contains("9"));
    }

    // --- segment_points helper (tested indirectly through validate_icon_source) ---

    #[test]
    fn test_validate_with_cubic_out_of_bounds() {
        // Control points outside viewport should trigger warning
        let result =
            validate_icon_source("M0 0 C100 100 200 200 10 10", &Viewport::STANDARD_24);
        assert!(result.is_valid);
        let has_oob = result
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::CoordinatesOutOfViewport));
        assert!(has_oob);
    }
}
