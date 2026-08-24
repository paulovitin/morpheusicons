//! Validation utilities for checking icon compatibility with the morphing engine.
//!
//! Use these functions to verify that external icon data (from Lucide, Heroicons, Tabler,
//! Phosphor, or any custom SVG source) meets the requirements for smooth morph animations.
//!
//! # Criteria for morph-compatible icons
//!
//! | Criterion | Required | Reason |
//! |-----------|----------|--------|
//! | Stroke-based geometry | ✅ | Fill-based icons collapse rather than morph |
//! | Valid SVG path commands | ✅ | Parser must understand the geometry |
//! | Consistent viewport | ✅ | Both icons must share coordinate space |
//! | Similar subpath count | ⚠️ recommended | Large mismatches cause expand/collapse artifacts |
//! | Coordinates within viewport | ⚠️ recommended | Out-of-bounds points may clip during animation |
//!
//! # Example
//!
//! ```rust
//! use morpheusicons::icons::validate::*;
//! use morpheusicons::icons::source::Viewport;
//!
//! // Check a raw path string from an external icon library
//! let lucide_home = "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2zM9 22V12h6v10";
//! let result = check_path_data(lucide_home);
//! assert!(result.is_ok());
//!
//! // Check if two icons are compatible for morphing
//! let lucide_menu = "M4 6h16M4 12h16M4 18h16";
//! let lucide_x = "M18 6L6 18M6 6l12 12";
//! let compat = check_morph_compatibility(lucide_menu, lucide_x, &Viewport::STANDARD_24);
//! assert!(compat.is_compatible);
//! ```

use crate::geometry::path::IconPath;
use crate::icons::source::{
    validate_icon_source, validate_morph_pair, RawIcon, ValidationError,
    ValidationResult, ValidationWarning, Viewport,
};

/// Quick check: is this path data string parseable and non-empty?
/// Returns `Ok(subpath_count)` or `Err(reason)`.
pub fn check_path_data(path_data: &str) -> Result<usize, String> {
    if path_data.trim().is_empty() {
        return Err("path data is empty".to_string());
    }
    let icon_path = IconPath::parse(path_data)?;
    Ok(icon_path.subpaths.len())
}

/// Detailed compatibility report for a morph pair.
#[derive(Debug, Clone)]
pub struct MorphCompatibility {
    /// Whether the pair can be morphed (both are valid).
    pub is_compatible: bool,
    /// Quality score from 0.0 (poor) to 1.0 (excellent) based on geometric similarity.
    pub quality_score: f32,
    /// Warnings about potential animation quality issues.
    pub warnings: Vec<ValidationWarning>,
    /// Errors that prevent morphing.
    pub errors: Vec<ValidationError>,
    /// Number of subpaths in the source icon.
    pub source_subpath_count: usize,
    /// Number of subpaths in the target icon.
    pub target_subpath_count: usize,
}

/// Checks whether two path data strings are compatible for morphing within the given viewport.
pub fn check_morph_compatibility(
    source_path: &str,
    target_path: &str,
    viewport: &Viewport,
) -> MorphCompatibility {
    let src = RawIcon::with_viewport(source_path.to_string(), *viewport);
    let tgt = RawIcon::with_viewport(target_path.to_string(), *viewport);

    let pair_result = validate_morph_pair(&src, &tgt);

    let src_count = IconPath::parse(source_path)
        .map(|p| p.subpaths.len())
        .unwrap_or(0);
    let tgt_count = IconPath::parse(target_path)
        .map(|p| p.subpaths.len())
        .unwrap_or(0);

    let quality_score = if pair_result.is_valid {
        compute_quality_score(src_count, tgt_count, &pair_result.warnings)
    } else {
        0.0
    };

    MorphCompatibility {
        is_compatible: pair_result.is_valid,
        quality_score,
        warnings: pair_result.warnings,
        errors: pair_result.errors,
        source_subpath_count: src_count,
        target_subpath_count: tgt_count,
    }
}

/// Validates a single icon path for use with MorpheusIcons.
/// Shorthand for `validate_icon_source` with the standard 24×24 viewport.
pub fn check_icon(path_data: &str) -> ValidationResult {
    validate_icon_source(path_data, &Viewport::STANDARD_24)
}

/// Validates a single icon path with a custom viewport.
pub fn check_icon_with_viewport(path_data: &str, viewport: &Viewport) -> ValidationResult {
    validate_icon_source(path_data, viewport)
}

/// Estimates morph animation quality based on structural properties.
///
/// Returns a score from 0.0 to 1.0:
/// - 1.0: Ideal pair (same subpath count, no warnings)
/// - 0.7-0.9: Good pair (minor differences)
/// - 0.4-0.7: Acceptable pair (noticeable artifacts possible)
/// - < 0.4: Poor pair (significant visual artifacts expected)
fn compute_quality_score(
    source_subpaths: usize,
    target_subpaths: usize,
    warnings: &[ValidationWarning],
) -> f32 {
    let mut score: f32 = 1.0;

    // Penalize subpath count difference
    let diff = source_subpaths.abs_diff(target_subpaths) as f32;
    score -= (diff * 0.1).min(0.4);

    // Penalize high subpath counts (more complex = harder to animate smoothly)
    let max_count = source_subpaths.max(target_subpaths) as f32;
    if max_count > 6.0 {
        score -= ((max_count - 6.0) * 0.05).min(0.2);
    }

    // Penalize warnings
    for warning in warnings {
        match warning {
            ValidationWarning::HighSubpathCount { .. } => score -= 0.1,
            ValidationWarning::CoordinatesOutOfViewport => score -= 0.05,
            ValidationWarning::SubpathCountMismatch { .. } => score -= 0.15,
        }
    }

    score.clamp(0.0, 1.0)
}

/// Known icon library metadata for documentation and detection purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnownIconLibrary {
    /// Lucide (fork of Feather) — stroke-based, 24×24
    Lucide,
    /// Feather Icons — stroke-based, 24×24
    Feather,
    /// Heroicons Outline — stroke-based, 24×24
    HeroiconsOutline,
    /// Tabler Icons — stroke-based, 24×24
    Tabler,
    /// Phosphor Regular/Light/Thin — stroke-based, 256×256
    Phosphor,
}

impl KnownIconLibrary {
    /// Returns the default viewport for this icon library.
    pub const fn viewport(&self) -> Viewport {
        match self {
            Self::Lucide | Self::Feather | Self::HeroiconsOutline | Self::Tabler => {
                Viewport::STANDARD_24
            }
            Self::Phosphor => Viewport::new(256.0, 256.0),
        }
    }

    /// Returns whether this library's icons are stroke-based (and thus morph-compatible).
    pub const fn is_stroke_based(&self) -> bool {
        true // All known supported libraries are stroke-based
    }

    /// Returns a human-readable description of the library.
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Lucide => "Lucide Icons — beautiful & consistent stroke icons (lucide.dev)",
            Self::Feather => "Feather Icons — simply beautiful open source icons",
            Self::HeroiconsOutline => "Heroicons Outline — hand-crafted SVG icons by Tailwind CSS",
            Self::Tabler => "Tabler Icons — free and open source stroke icons",
            Self::Phosphor => "Phosphor Regular — flexible icon family (256×256 viewport)",
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PATH: &str = "M4 6h16M4 12h16M4 18h16";
    const VALID_PATH_2: &str = "M18 6L6 18M6 6l12 12";
    const COMPLEX_PATH: &str = "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z";
    const MANY_SUBPATHS: &str = "M0 0L1 1M2 2L3 3M4 4L5 5M6 6L7 7M8 8L9 9M10 10L11 11M12 12L13 13M14 14L15 15M16 16L17 17";

    // --- check_path_data ---

    #[test]
    fn test_check_path_data_valid() {
        let result = check_path_data(VALID_PATH);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3); // menu icon has 3 subpaths
    }

    #[test]
    fn test_check_path_data_single_subpath() {
        let result = check_path_data("M0 0 L10 10");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_check_path_data_empty() {
        let result = check_path_data("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_check_path_data_whitespace_only() {
        let result = check_path_data("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_check_path_data_invalid() {
        let result = check_path_data("INVALID PATH DATA");
        assert!(result.is_err());
    }

    #[test]
    fn test_check_path_data_complex() {
        let result = check_path_data(COMPLEX_PATH);
        assert!(result.is_ok());
        assert!(result.unwrap() >= 1);
    }

    // --- check_icon ---

    #[test]
    fn test_check_icon_valid() {
        let result = check_icon(VALID_PATH);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_check_icon_empty() {
        let result = check_icon("");
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
        assert!(matches!(result.errors[0], ValidationError::EmptyPathData));
    }

    #[test]
    fn test_check_icon_invalid_path() {
        let result = check_icon("XXXXX");
        assert!(!result.is_valid);
        assert!(matches!(
            result.errors[0],
            ValidationError::InvalidPathData { .. }
        ));
    }

    #[test]
    fn test_check_icon_high_subpath_count_warning() {
        let result = check_icon(MANY_SUBPATHS);
        assert!(result.is_valid);
        let has_warning = result.warnings.iter().any(|w| {
            matches!(w, ValidationWarning::HighSubpathCount { count } if *count > 8)
        });
        assert!(has_warning);
    }

    #[test]
    fn test_check_icon_within_viewport() {
        // All coords within 24x24
        let result = check_icon("M2 2 L22 22");
        assert!(result.is_valid);
        let has_oob = result
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::CoordinatesOutOfViewport));
        assert!(!has_oob);
    }

    #[test]
    fn test_check_icon_out_of_viewport() {
        // Coords way outside 24x24 viewport
        let result = check_icon("M0 0 L100 100");
        assert!(result.is_valid); // Still valid, just a warning
        let has_oob = result
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::CoordinatesOutOfViewport));
        assert!(has_oob);
    }

    // --- check_icon_with_viewport ---

    #[test]
    fn test_check_icon_with_custom_viewport() {
        let viewport = Viewport::new(256.0, 256.0);
        let result = check_icon_with_viewport("M0 0 L100 100", &viewport);
        assert!(result.is_valid);
        // 100x100 is within 256x256, so no OOB warning
        let has_oob = result
            .warnings
            .iter()
            .any(|w| matches!(w, ValidationWarning::CoordinatesOutOfViewport));
        assert!(!has_oob);
    }

    #[test]
    fn test_check_icon_with_invalid_viewport() {
        let viewport = Viewport::new(0.0, 24.0);
        let result = check_icon_with_viewport(VALID_PATH, &viewport);
        assert!(!result.is_valid);
        assert!(matches!(
            result.errors[0],
            ValidationError::InvalidViewport { .. }
        ));
    }

    #[test]
    fn test_check_icon_with_negative_viewport() {
        let viewport = Viewport::new(-10.0, 24.0);
        let result = check_icon_with_viewport(VALID_PATH, &viewport);
        assert!(!result.is_valid);
    }

    // --- check_morph_compatibility ---

    #[test]
    fn test_morph_compatibility_valid_pair() {
        let compat = check_morph_compatibility(VALID_PATH, VALID_PATH_2, &Viewport::STANDARD_24);
        assert!(compat.is_compatible);
        assert!(compat.quality_score > 0.0);
        assert!(compat.errors.is_empty());
    }

    #[test]
    fn test_morph_compatibility_same_icon() {
        let compat = check_morph_compatibility(VALID_PATH, VALID_PATH, &Viewport::STANDARD_24);
        assert!(compat.is_compatible);
        assert!(compat.quality_score >= 0.9); // Same icon = high quality
    }

    #[test]
    fn test_morph_compatibility_invalid_source() {
        let compat = check_morph_compatibility("INVALID", VALID_PATH, &Viewport::STANDARD_24);
        assert!(!compat.is_compatible);
        assert_eq!(compat.quality_score, 0.0);
    }

    #[test]
    fn test_morph_compatibility_invalid_target() {
        let compat = check_morph_compatibility(VALID_PATH, "INVALID", &Viewport::STANDARD_24);
        assert!(!compat.is_compatible);
        assert_eq!(compat.quality_score, 0.0);
    }

    #[test]
    fn test_morph_compatibility_empty_source() {
        let compat = check_morph_compatibility("", VALID_PATH, &Viewport::STANDARD_24);
        assert!(!compat.is_compatible);
    }

    #[test]
    fn test_morph_compatibility_subpath_counts() {
        let compat = check_morph_compatibility(VALID_PATH, VALID_PATH_2, &Viewport::STANDARD_24);
        assert_eq!(compat.source_subpath_count, 3); // menu has 3
        assert_eq!(compat.target_subpath_count, 2); // X has 2
    }

    #[test]
    fn test_morph_compatibility_large_subpath_mismatch_warning() {
        // 9 subpaths vs 1 subpath - diff > 4
        let compat =
            check_morph_compatibility(MANY_SUBPATHS, "M0 0 L10 10", &Viewport::STANDARD_24);
        assert!(compat.is_compatible);
        let has_mismatch = compat.warnings.iter().any(|w| {
            matches!(w, ValidationWarning::SubpathCountMismatch { .. })
        });
        assert!(has_mismatch);
    }

    #[test]
    fn test_morph_compatibility_quality_score_range() {
        let compat = check_morph_compatibility(VALID_PATH, VALID_PATH_2, &Viewport::STANDARD_24);
        assert!(compat.quality_score >= 0.0);
        assert!(compat.quality_score <= 1.0);
    }

    #[test]
    fn test_morph_compatibility_similar_subpaths_higher_quality() {
        // Same subpath count should have higher quality than different
        let same_count =
            check_morph_compatibility(VALID_PATH_2, "M0 0L5 5M10 10L15 15", &Viewport::STANDARD_24);
        let diff_count =
            check_morph_compatibility(VALID_PATH, VALID_PATH_2, &Viewport::STANDARD_24);
        // same_count: 2 vs 2, diff_count: 3 vs 2
        assert!(same_count.quality_score >= diff_count.quality_score);
    }

    // --- KnownIconLibrary ---

    #[test]
    fn test_known_icon_library_viewports() {
        assert_eq!(KnownIconLibrary::Lucide.viewport(), Viewport::STANDARD_24);
        assert_eq!(KnownIconLibrary::Feather.viewport(), Viewport::STANDARD_24);
        assert_eq!(
            KnownIconLibrary::HeroiconsOutline.viewport(),
            Viewport::STANDARD_24
        );
        assert_eq!(KnownIconLibrary::Tabler.viewport(), Viewport::STANDARD_24);
        assert_eq!(
            KnownIconLibrary::Phosphor.viewport(),
            Viewport::new(256.0, 256.0)
        );
    }

    #[test]
    fn test_known_icon_library_is_stroke_based() {
        assert!(KnownIconLibrary::Lucide.is_stroke_based());
        assert!(KnownIconLibrary::Feather.is_stroke_based());
        assert!(KnownIconLibrary::HeroiconsOutline.is_stroke_based());
        assert!(KnownIconLibrary::Tabler.is_stroke_based());
        assert!(KnownIconLibrary::Phosphor.is_stroke_based());
    }

    #[test]
    fn test_known_icon_library_description() {
        assert!(!KnownIconLibrary::Lucide.description().is_empty());
        assert!(!KnownIconLibrary::Feather.description().is_empty());
        assert!(!KnownIconLibrary::HeroiconsOutline.description().is_empty());
        assert!(!KnownIconLibrary::Tabler.description().is_empty());
        assert!(!KnownIconLibrary::Phosphor.description().is_empty());
    }

    #[test]
    fn test_known_icon_library_equality() {
        assert_eq!(KnownIconLibrary::Lucide, KnownIconLibrary::Lucide);
        assert_ne!(KnownIconLibrary::Lucide, KnownIconLibrary::Feather);
    }

    // --- compute_quality_score (tested indirectly) ---

    #[test]
    fn test_quality_score_perfect_pair() {
        // Same subpath count, no warnings
        let compat =
            check_morph_compatibility("M0 0L10 10", "M5 5L15 15", &Viewport::STANDARD_24);
        assert!(compat.quality_score >= 0.9);
    }

    #[test]
    fn test_quality_score_degraded_by_mismatch() {
        // Large difference in subpath count
        let compat =
            check_morph_compatibility(MANY_SUBPATHS, "M0 0 L10 10", &Viewport::STANDARD_24);
        assert!(compat.quality_score < 0.7);
    }
}
