use crate::animation::spring::{Spring, SpringConfig};
use crate::geometry::path::IconPath;
use crate::geometry::point::Point;
use crate::geometry::procrustes::ProcrustesMorphData;
use crate::geometry::sampling::{SampledIcon, SAMPLES_PER_SUBPATH};
use crate::icons::source::IconSource;

/// High-level vector drawing command suitable for any GUI rendering backend.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    MoveTo(Point),
    LineTo(Point),
    CubicTo {
        ctrl1: Point,
        ctrl2: Point,
        end: Point,
    },
    Close,
}

/// Core path morpher that interpolates between two SVG icon paths.
#[derive(Debug, Clone)]
pub struct PathMorpher {
    morph_data: ProcrustesMorphData,
    samples_per_subpath: usize,
}

impl PathMorpher {
    /// Creates a new `PathMorpher` by parsing two SVG path strings.
    pub fn new(from_svg_path: &str, to_svg_path: &str) -> Result<Self, String> {
        let from_icon = IconPath::parse(from_svg_path)?;
        let to_icon = IconPath::parse(to_svg_path)?;
        Ok(Self::from_icon_paths(&from_icon, &to_icon))
    }

    /// Creates a new `PathMorpher` from two `IconSource` implementations.
    ///
    /// This is the preferred constructor when working with external icon libraries,
    /// as it handles viewport normalization automatically.
    ///
    /// # Example
    ///
    /// ```rust
    /// use morpheusicons::prelude::*;
    ///
    /// // Mix built-in icons with external path data
    /// let external_icon = RawIcon::new("M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z");
    /// let morpher = PathMorpher::from_sources(&Icon::Play, &external_icon).unwrap();
    /// ```
    pub fn from_sources(
        from: &dyn IconSource,
        to: &dyn IconSource,
    ) -> Result<Self, String> {
        let from_path = from.to_icon_path(None)?;
        let to_path = to.to_icon_path(Some(&from.viewport()))?;
        Ok(Self::from_icon_paths(&from_path, &to_path))
    }

    /// Creates a new `PathMorpher` from parsed `IconPath` instances.
    pub fn from_icon_paths(from_icon: &IconPath, to_icon: &IconPath) -> Self {
        let n_samples = SAMPLES_PER_SUBPATH;
        let sampled_from = SampledIcon::sample(from_icon, n_samples);
        let sampled_to = SampledIcon::sample(to_icon, n_samples);

        let (aligned_from, aligned_to) = sampled_from.align_with(sampled_to, n_samples);
        let morph_data = ProcrustesMorphData::compute(&aligned_from, &aligned_to);

        Self {
            morph_data,
            samples_per_subpath: n_samples,
        }
    }

    /// Samples the morphed icon geometry at animation progress `t` (0.0 to 1.0).
    pub fn interpolate(&self, t: f32) -> SampledIcon {
        self.morph_data.interpolate(t)
    }

    /// Returns the number of sample points used per subpath.
    pub fn samples_per_subpath(&self) -> usize {
        self.samples_per_subpath
    }

    /// Converts interpolated sample points into an SVG `d="..."` path string at progress `t`.
    pub fn to_svg_path(&self, t: f32) -> String {
        let sampled = self.interpolate(t);
        sampled_icon_to_svg_path(&sampled)
    }

    /// Converts interpolated sample points into raw drawing commands at progress `t`.
    pub fn to_draw_commands(&self, t: f32) -> Vec<DrawCommand> {
        let sampled = self.interpolate(t);
        sampled_icon_to_draw_commands(&sampled)
    }
}

/// High-level morph controller driven by spring physics or manual progress.
#[derive(Debug, Clone)]
pub struct MorphController {
    morpher: PathMorpher,
    spring: Spring,
}

impl MorphController {
    /// Creates a new `MorphController` between two SVG path strings.
    pub fn new(
        from_svg_path: &str,
        to_svg_path: &str,
        spring_config: SpringConfig,
    ) -> Result<Self, String> {
        let morpher = PathMorpher::new(from_svg_path, to_svg_path)?;
        let spring = Spring::new(0.0, spring_config);
        Ok(Self { morpher, spring })
    }

    /// Creates a new `MorphController` from two `IconSource` implementations.
    ///
    /// This is the preferred constructor when working with external icon libraries.
    /// Handles viewport normalization automatically.
    ///
    /// # Example
    ///
    /// ```rust
    /// use morpheusicons::prelude::*;
    ///
    /// // Morph between a built-in icon and a Lucide icon path
    /// let lucide_menu = RawIcon::new("M4 6h16M4 12h16M4 18h16");
    /// let controller = MorphController::from_sources(
    ///     &Icon::X,
    ///     &lucide_menu,
    ///     SpringConfig::BOUNCY,
    /// ).unwrap();
    ///
    /// // Or use raw path strings directly (they implement IconSource)
    /// let controller = MorphController::from_sources(
    ///     &"M4 6h16M4 12h16M4 18h16" as &dyn IconSource,
    ///     &"M18 6L6 18M6 6l12 12" as &dyn IconSource,
    ///     SpringConfig::GENTLE,
    /// ).unwrap();
    /// ```
    pub fn from_sources(
        from: &dyn IconSource,
        to: &dyn IconSource,
        spring_config: SpringConfig,
    ) -> Result<Self, String> {
        let morpher = PathMorpher::from_sources(from, to)?;
        let spring = Spring::new(0.0, spring_config);
        Ok(Self { morpher, spring })
    }

    /// Creates a `MorphController` from an existing `PathMorpher`.
    pub fn from_morpher(morpher: PathMorpher, spring_config: SpringConfig) -> Self {
        Self {
            morpher,
            spring: Spring::new(0.0, spring_config),
        }
    }

    /// Set animation target: `0.0` for source icon, `1.0` for target icon.
    pub fn set_target(&mut self, target: f32) {
        self.spring.set_target(target);
    }

    /// Switches target to `1.0` (morph to end).
    pub fn morph_to_end(&mut self) {
        self.set_target(1.0);
    }

    /// Switches target to `0.0` (morph to start).
    pub fn morph_to_start(&mut self) {
        self.set_target(0.0);
    }

    /// Toggles target between 0.0 and 1.0.
    pub fn toggle(&mut self) {
        if self.spring.target > 0.5 {
            self.morph_to_start();
        } else {
            self.morph_to_end();
        }
    }

    /// Updates spring animation by delta time `dt` seconds.
    /// Returns `true` if animation is active.
    pub fn update(&mut self, dt: f32) -> bool {
        self.spring.update(dt)
    }

    /// Returns current progress (0.0 to 1.0).
    pub fn progress(&self) -> f32 {
        self.spring.value
    }

    /// Sets explicit progress (bypassing spring solver).
    pub fn set_progress(&mut self, progress: f32) {
        self.spring.value = progress;
        self.spring.target = progress;
        self.spring.velocity = 0.0;
    }

    /// Returns the current SVG path string `d="..."` for rendering.
    pub fn current_svg_path(&self) -> String {
        self.morpher.to_svg_path(self.progress())
    }

    /// Returns current draw commands.
    pub fn current_draw_commands(&self) -> Vec<DrawCommand> {
        self.morpher.to_draw_commands(self.progress())
    }

    /// Access inner `PathMorpher`.
    pub fn morpher(&self) -> &PathMorpher {
        &self.morpher
    }
}

/// Checks if a subpath has collapsed into a single point (near-zero extent).
#[inline]
fn is_subpath_collapsed(subpath: &[Point]) -> bool {
    if subpath.len() <= 1 {
        return true;
    }
    let p0 = subpath[0];
    let max_dist_sq = subpath
        .iter()
        .map(|p| p.distance_squared(&p0))
        .fold(0.0f32, f32::max);
    max_dist_sq < 0.09 // Extent < 0.3 pixels
}

/// Converts a `SampledIcon` into a clean SVG `d="..."` path string using smooth Catmull-Rom cubic Bezier curves.
pub fn sampled_icon_to_svg_path(sampled: &SampledIcon) -> String {
    let mut buf = String::with_capacity(512);

    for subpath in &sampled.subpaths {
        if is_subpath_collapsed(subpath) {
            continue;
        }

        // Write MoveTo start
        let p0 = subpath[0];
        buf.push_str(&format!("M{:.2} {:.2}", p0.x, p0.y));

        if subpath.len() <= 2 {
            for pt in subpath.iter().skip(1) {
                buf.push_str(&format!("L{:.2} {:.2}", pt.x, pt.y));
            }
            continue;
        }

        // Fit smooth Catmull-Rom cubic Beziers through points
        let len = subpath.len();
        for i in 0..len - 1 {
            let p_prev = if i == 0 { subpath[0] } else { subpath[i - 1] };
            let p_curr = subpath[i];
            let p_next = subpath[i + 1];
            let p_next2 = if i + 2 < len {
                subpath[i + 2]
            } else {
                subpath[len - 1]
            };

            let c1 = p_curr + (p_next - p_prev) * (1.0 / 6.0);
            let c2 = p_next - (p_next2 - p_curr) * (1.0 / 6.0);

            buf.push_str(&format!(
                "C{:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
                c1.x, c1.y, c2.x, c2.y, p_next.x, p_next.y
            ));
        }
    }

    buf
}

/// Converts a `SampledIcon` into `DrawCommand` primitives.
pub fn sampled_icon_to_draw_commands(sampled: &SampledIcon) -> Vec<DrawCommand> {
    let mut commands = Vec::new();

    for subpath in &sampled.subpaths {
        if is_subpath_collapsed(subpath) {
            continue;
        }

        commands.push(DrawCommand::MoveTo(subpath[0]));

        if subpath.len() <= 2 {
            for pt in subpath.iter().skip(1) {
                commands.push(DrawCommand::LineTo(*pt));
            }
            continue;
        }

        let len = subpath.len();
        for i in 0..len - 1 {
            let p_prev = if i == 0 { subpath[0] } else { subpath[i - 1] };
            let p_curr = subpath[i];
            let p_next = subpath[i + 1];
            let p_next2 = if i + 2 < len {
                subpath[i + 2]
            } else {
                subpath[len - 1]
            };

            let c1 = p_curr + (p_next - p_prev) * (1.0 / 6.0);
            let c2 = p_next - (p_next2 - p_curr) * (1.0 / 6.0);

            commands.push(DrawCommand::CubicTo {
                ctrl1: c1,
                ctrl2: c2,
                end: p_next,
            });
        }
    }

    commands
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::sampling::SAMPLES_PER_SUBPATH;
    use crate::icons::source::RawIcon;

    const MENU_PATH: &str = "M4 6h16M4 12h16M4 18h16";
    const X_PATH: &str = "M18 6L6 18M6 6l12 12";
    const PLAY_PATH: &str = "M5 3l14 9-14 9V3z";
    const PAUSE_PATH: &str = "M6 4h4v16H6zM14 4h4v16h-4z";

    // --- PathMorpher::new ---

    #[test]
    fn test_path_morpher_new_valid() {
        let morpher = PathMorpher::new(MENU_PATH, X_PATH);
        assert!(morpher.is_ok());
    }

    #[test]
    fn test_path_morpher_new_invalid_source() {
        let morpher = PathMorpher::new("INVALID", X_PATH);
        assert!(morpher.is_err());
    }

    #[test]
    fn test_path_morpher_new_invalid_target() {
        let morpher = PathMorpher::new(MENU_PATH, "XXXXX");
        assert!(morpher.is_err());
    }

    #[test]
    fn test_path_morpher_samples_per_subpath() {
        let morpher = PathMorpher::new(MENU_PATH, X_PATH).unwrap();
        assert_eq!(morpher.samples_per_subpath(), SAMPLES_PER_SUBPATH);
    }

    // --- PathMorpher::from_sources ---

    #[test]
    fn test_path_morpher_from_sources_str() {
        let from: &str = MENU_PATH;
        let to: &str = X_PATH;
        let morpher = PathMorpher::from_sources(&from, &to);
        assert!(morpher.is_ok());
    }

    #[test]
    fn test_path_morpher_from_sources_raw_icon() {
        let from = RawIcon::new(PLAY_PATH);
        let to = RawIcon::new(PAUSE_PATH);
        let morpher = PathMorpher::from_sources(&from, &to);
        assert!(morpher.is_ok());
    }

    #[test]
    fn test_path_morpher_from_sources_empty_fails() {
        let from = RawIcon::new("");
        let to = RawIcon::new(MENU_PATH);
        let morpher = PathMorpher::from_sources(&from, &to);
        assert!(morpher.is_err());
    }

    // --- PathMorpher::interpolate ---

    #[test]
    fn test_interpolate_returns_correct_structure() {
        let morpher = PathMorpher::new(MENU_PATH, X_PATH).unwrap();
        let sampled = morpher.interpolate(0.5);
        assert!(!sampled.subpaths.is_empty());
        for sp in &sampled.subpaths {
            assert_eq!(sp.len(), SAMPLES_PER_SUBPATH);
        }
    }

    #[test]
    fn test_interpolate_no_nan() {
        let morpher = PathMorpher::new(PLAY_PATH, PAUSE_PATH).unwrap();
        for t in [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0] {
            let sampled = morpher.interpolate(t);
            for sp in &sampled.subpaths {
                for p in sp {
                    assert!(!p.x.is_nan(), "NaN at t={t}");
                    assert!(!p.y.is_nan(), "NaN at t={t}");
                }
            }
        }
    }

    // --- PathMorpher::to_svg_path ---

    #[test]
    fn test_to_svg_path_at_zero() {
        let morpher = PathMorpher::new(MENU_PATH, X_PATH).unwrap();
        let svg = morpher.to_svg_path(0.0);
        assert!(!svg.is_empty());
        assert!(svg.starts_with('M'));
    }

    #[test]
    fn test_to_svg_path_at_one() {
        let morpher = PathMorpher::new(MENU_PATH, X_PATH).unwrap();
        let svg = morpher.to_svg_path(1.0);
        assert!(!svg.is_empty());
        assert!(svg.starts_with('M'));
    }

    #[test]
    fn test_to_svg_path_contains_cubic_commands() {
        let morpher = PathMorpher::new(MENU_PATH, X_PATH).unwrap();
        let svg = morpher.to_svg_path(0.5);
        // Catmull-Rom produces cubic beziers
        assert!(svg.contains('C'));
    }

    #[test]
    fn test_to_svg_path_different_at_different_t() {
        let morpher = PathMorpher::new(MENU_PATH, X_PATH).unwrap();
        let svg_0 = morpher.to_svg_path(0.0);
        let svg_1 = morpher.to_svg_path(1.0);
        assert_ne!(svg_0, svg_1);
    }

    // --- PathMorpher::to_draw_commands ---

    #[test]
    fn test_to_draw_commands_at_zero() {
        let morpher = PathMorpher::new(MENU_PATH, X_PATH).unwrap();
        let cmds = morpher.to_draw_commands(0.0);
        assert!(!cmds.is_empty());
        assert!(matches!(cmds[0], DrawCommand::MoveTo(_)));
    }

    #[test]
    fn test_to_draw_commands_contains_cubics() {
        let morpher = PathMorpher::new(MENU_PATH, X_PATH).unwrap();
        let cmds = morpher.to_draw_commands(0.5);
        let has_cubic = cmds.iter().any(|c| matches!(c, DrawCommand::CubicTo { .. }));
        assert!(has_cubic);
    }

    #[test]
    fn test_to_draw_commands_no_close() {
        // sampled_icon_to_draw_commands doesn't emit Close
        let morpher = PathMorpher::new(MENU_PATH, X_PATH).unwrap();
        let cmds = morpher.to_draw_commands(0.5);
        let has_close = cmds.iter().any(|c| matches!(c, DrawCommand::Close));
        assert!(!has_close);
    }

    // --- MorphController ---

    #[test]
    fn test_morph_controller_new() {
        let ctrl = MorphController::new(MENU_PATH, X_PATH, SpringConfig::SMOOTH);
        assert!(ctrl.is_ok());
        let ctrl = ctrl.unwrap();
        assert_eq!(ctrl.progress(), 0.0);
    }

    #[test]
    fn test_morph_controller_new_invalid() {
        let ctrl = MorphController::new("INVALID", X_PATH, SpringConfig::SMOOTH);
        assert!(ctrl.is_err());
    }

    #[test]
    fn test_morph_controller_from_sources() {
        let from = RawIcon::new(PLAY_PATH);
        let to = RawIcon::new(PAUSE_PATH);
        let ctrl = MorphController::from_sources(&from, &to, SpringConfig::BOUNCY);
        assert!(ctrl.is_ok());
    }

    #[test]
    fn test_morph_controller_from_morpher() {
        let morpher = PathMorpher::new(MENU_PATH, X_PATH).unwrap();
        let ctrl = MorphController::from_morpher(morpher, SpringConfig::SNAPPY);
        assert_eq!(ctrl.progress(), 0.0);
    }

    #[test]
    fn test_morph_controller_morph_to_end() {
        let mut ctrl = MorphController::new(MENU_PATH, X_PATH, SpringConfig::SNAPPY).unwrap();
        ctrl.morph_to_end();
        // Animate
        for _ in 0..200 {
            ctrl.update(0.016);
        }
        assert!((ctrl.progress() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_morph_controller_morph_to_start() {
        let mut ctrl = MorphController::new(MENU_PATH, X_PATH, SpringConfig::SNAPPY).unwrap();
        ctrl.set_progress(1.0);
        ctrl.morph_to_start();
        for _ in 0..200 {
            ctrl.update(0.016);
        }
        assert!((ctrl.progress() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_morph_controller_toggle() {
        let mut ctrl = MorphController::new(MENU_PATH, X_PATH, SpringConfig::SNAPPY).unwrap();
        // Initially at 0, toggle should go to 1
        ctrl.toggle();
        for _ in 0..200 {
            ctrl.update(0.016);
        }
        assert!((ctrl.progress() - 1.0).abs() < 0.01);

        // Toggle again, should go back to 0
        ctrl.toggle();
        for _ in 0..200 {
            ctrl.update(0.016);
        }
        assert!((ctrl.progress() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_morph_controller_set_progress() {
        let mut ctrl = MorphController::new(MENU_PATH, X_PATH, SpringConfig::SMOOTH).unwrap();
        ctrl.set_progress(0.7);
        assert!((ctrl.progress() - 0.7).abs() < 1e-6);
    }

    #[test]
    fn test_morph_controller_current_svg_path() {
        let ctrl = MorphController::new(MENU_PATH, X_PATH, SpringConfig::SMOOTH).unwrap();
        let svg = ctrl.current_svg_path();
        assert!(!svg.is_empty());
        assert!(svg.starts_with('M'));
    }

    #[test]
    fn test_morph_controller_current_draw_commands() {
        let ctrl = MorphController::new(MENU_PATH, X_PATH, SpringConfig::SMOOTH).unwrap();
        let cmds = ctrl.current_draw_commands();
        assert!(!cmds.is_empty());
    }

    #[test]
    fn test_morph_controller_morpher_access() {
        let ctrl = MorphController::new(MENU_PATH, X_PATH, SpringConfig::SMOOTH).unwrap();
        let morpher = ctrl.morpher();
        assert_eq!(morpher.samples_per_subpath(), SAMPLES_PER_SUBPATH);
    }

    #[test]
    fn test_morph_controller_update_returns_false_when_done() {
        let mut ctrl = MorphController::new(MENU_PATH, X_PATH, SpringConfig::SNAPPY).unwrap();
        // Don't move target, already at rest
        let active = ctrl.update(0.016);
        assert!(!active);
    }

    // --- sampled_icon_to_svg_path ---

    #[test]
    fn test_sampled_icon_to_svg_path_collapsed_skipped() {
        // A subpath with all points at the same location should be skipped
        let sampled = SampledIcon {
            subpaths: vec![vec![Point::new(5.0, 5.0); 64]],
        };
        let svg = sampled_icon_to_svg_path(&sampled);
        assert!(svg.is_empty());
    }

    #[test]
    fn test_sampled_icon_to_svg_path_two_points() {
        let sampled = SampledIcon {
            subpaths: vec![vec![Point::new(0.0, 0.0), Point::new(10.0, 10.0)]],
        };
        let svg = sampled_icon_to_svg_path(&sampled);
        assert!(svg.contains('M'));
        assert!(svg.contains('L'));
    }

    #[test]
    fn test_sampled_icon_to_svg_path_many_points() {
        let pts: Vec<Point> = (0..64)
            .map(|i| Point::new(i as f32, (i as f32).sin() * 5.0 + 12.0))
            .collect();
        let sampled = SampledIcon {
            subpaths: vec![pts],
        };
        let svg = sampled_icon_to_svg_path(&sampled);
        assert!(svg.starts_with('M'));
        assert!(svg.contains('C'));
    }

    // --- sampled_icon_to_draw_commands ---

    #[test]
    fn test_sampled_icon_to_draw_commands_collapsed_skipped() {
        let sampled = SampledIcon {
            subpaths: vec![vec![Point::new(5.0, 5.0); 64]],
        };
        let cmds = sampled_icon_to_draw_commands(&sampled);
        assert!(cmds.is_empty());
    }

    #[test]
    fn test_sampled_icon_to_draw_commands_two_points() {
        let sampled = SampledIcon {
            subpaths: vec![vec![Point::new(0.0, 0.0), Point::new(10.0, 10.0)]],
        };
        let cmds = sampled_icon_to_draw_commands(&sampled);
        assert_eq!(cmds.len(), 2); // MoveTo + LineTo
        assert!(matches!(cmds[0], DrawCommand::MoveTo(_)));
        assert!(matches!(cmds[1], DrawCommand::LineTo(_)));
    }

    #[test]
    fn test_sampled_icon_to_draw_commands_many_points() {
        let pts: Vec<Point> = (0..10)
            .map(|i| Point::new(i as f32 * 3.0, i as f32 * 2.0))
            .collect();
        let sampled = SampledIcon {
            subpaths: vec![pts],
        };
        let cmds = sampled_icon_to_draw_commands(&sampled);
        // 1 MoveTo + 9 CubicTo (for 10 points, 9 segments)
        assert_eq!(cmds.len(), 10); // MoveTo + 9 CubicTo
        assert!(matches!(cmds[0], DrawCommand::MoveTo(_)));
        for cmd in &cmds[1..] {
            assert!(matches!(cmd, DrawCommand::CubicTo { .. }));
        }
    }

    // --- is_subpath_collapsed ---

    #[test]
    fn test_is_subpath_collapsed_single_point() {
        assert!(is_subpath_collapsed(&[Point::new(1.0, 1.0)]));
    }

    #[test]
    fn test_is_subpath_collapsed_empty() {
        assert!(is_subpath_collapsed(&[]));
    }

    #[test]
    fn test_is_subpath_collapsed_all_same() {
        let pts = vec![Point::new(5.0, 5.0); 100];
        assert!(is_subpath_collapsed(&pts));
    }

    #[test]
    fn test_is_subpath_collapsed_tiny_extent() {
        let pts = vec![
            Point::new(5.0, 5.0),
            Point::new(5.1, 5.1),
            Point::new(5.05, 5.0),
        ];
        assert!(is_subpath_collapsed(&pts));
    }

    #[test]
    fn test_is_subpath_not_collapsed() {
        let pts = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
        ];
        assert!(!is_subpath_collapsed(&pts));
    }

    // --- DrawCommand PartialEq ---

    #[test]
    fn test_draw_command_equality() {
        let a = DrawCommand::MoveTo(Point::new(1.0, 2.0));
        let b = DrawCommand::MoveTo(Point::new(1.0, 2.0));
        assert_eq!(a, b);

        let c = DrawCommand::CubicTo {
            ctrl1: Point::new(1.0, 2.0),
            ctrl2: Point::new(3.0, 4.0),
            end: Point::new(5.0, 6.0),
        };
        let d = DrawCommand::CubicTo {
            ctrl1: Point::new(1.0, 2.0),
            ctrl2: Point::new(3.0, 4.0),
            end: Point::new(5.0, 6.0),
        };
        assert_eq!(c, d);
        assert_ne!(a, DrawCommand::Close);
    }
}
