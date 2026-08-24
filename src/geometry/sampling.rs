use crate::geometry::path::{IconPath, PathSegment, SubPath};
use crate::geometry::point::Point;

/// Default number of sample points per subpath.
pub const SAMPLES_PER_SUBPATH: usize = 64;

/// Represents a set of sampled 2D points representing an entire icon stroke path.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SampledIcon {
    /// List of sampled subpaths, each containing `SAMPLES_PER_SUBPATH` points.
    pub subpaths: Vec<Vec<Point>>,
}

impl SampledIcon {
    /// Samples an `IconPath` into a uniform `SampledIcon` with `num_samples` per subpath.
    pub fn sample(icon_path: &IconPath, num_samples: usize) -> Self {
        let sampled_subpaths: Vec<Vec<Point>> = icon_path
            .subpaths
            .iter()
            .map(|sp| sample_subpath(sp, num_samples))
            .collect();

        Self {
            subpaths: sampled_subpaths,
        }
    }

    /// Aligns two `SampledIcon` instances so they have the exact same number of subpaths and samples.
    /// Unmatched subpaths are padded with collapsed zero-length subpaths at nearest positions.
    pub fn align_with(self, other: Self, num_samples: usize) -> (Self, Self) {
        let max_subpaths = self.subpaths.len().max(other.subpaths.len()).max(1);

        let mut a_subpaths = self.subpaths;
        let mut b_subpaths = other.subpaths;

        // Ensure at least one subpath
        if a_subpaths.is_empty() {
            a_subpaths.push(vec![Point::ZERO; num_samples]);
        }
        if b_subpaths.is_empty() {
            b_subpaths.push(vec![Point::ZERO; num_samples]);
        }

        // Pad A if B has more subpaths
        while a_subpaths.len() < max_subpaths {
            let idx = a_subpaths.len();
            // Collapse to centroid of corresponding subpath in B, or last point
            let target_sp = &b_subpaths[idx.min(b_subpaths.len() - 1)];
            let center = centroid_of_points(target_sp);
            a_subpaths.push(vec![center; num_samples]);
        }

        // Pad B if A has more subpaths
        while b_subpaths.len() < max_subpaths {
            let idx = b_subpaths.len();
            let target_sp = &a_subpaths[idx.min(a_subpaths.len() - 1)];
            let center = centroid_of_points(target_sp);
            b_subpaths.push(vec![center; num_samples]);
        }

        (
            SampledIcon {
                subpaths: a_subpaths,
            },
            SampledIcon {
                subpaths: b_subpaths,
            },
        )
    }
}

/// Computes the centroid of a slice of points.
pub fn centroid_of_points(points: &[Point]) -> Point {
    if points.is_empty() {
        return Point::ZERO;
    }
    let sum = points
        .iter()
        .fold(Point::ZERO, |acc, &p| Point::new(acc.x + p.x, acc.y + p.y));
    Point::new(sum.x / points.len() as f32, sum.y / points.len() as f32)
}

/// Samples a single `SubPath` into `num_samples` equidistant points.
pub fn sample_subpath(subpath: &SubPath, num_samples: usize) -> Vec<Point> {
    if subpath.segments.is_empty() || num_samples == 0 {
        return vec![Point::ZERO; num_samples.max(1)];
    }

    // Step 1: Flatten segments into fine polyline
    let mut polyline = Vec::new();
    let mut current_pos = Point::ZERO;

    for seg in &subpath.segments {
        match seg {
            PathSegment::MoveTo(p) => {
                current_pos = *p;
                polyline.push(*p);
            }
            PathSegment::LineTo(p) => {
                polyline.push(*p);
                current_pos = *p;
            }
            PathSegment::CubicTo { ctrl1, ctrl2, end } => {
                // Subdivide cubic bezier into 16 linear steps
                let steps = 16;
                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let p = sample_cubic_bezier(current_pos, *ctrl1, *ctrl2, *end, t);
                    polyline.push(p);
                }
                current_pos = *end;
            }
            PathSegment::Close => {
                if let Some(&first) = polyline.first() {
                    if current_pos != first {
                        polyline.push(first);
                        current_pos = first;
                    }
                }
            }
        }
    }

    if polyline.is_empty() {
        return vec![Point::ZERO; num_samples];
    }

    if polyline.len() == 1 {
        return vec![polyline[0]; num_samples];
    }

    // Step 2: Compute cumulative arc lengths
    let mut distances = Vec::with_capacity(polyline.len());
    distances.push(0.0);
    let mut total_length = 0.0;

    for i in 1..polyline.len() {
        let dist = polyline[i - 1].distance(&polyline[i]);
        total_length += dist;
        distances.push(total_length);
    }

    if total_length <= 1e-6 {
        return vec![polyline[0]; num_samples];
    }

    // Step 3: Resample `num_samples` points at uniform distance intervals
    let mut sampled = Vec::with_capacity(num_samples);
    let step_dist = total_length / ((num_samples - 1) as f32);

    let mut current_idx = 0;

    for i in 0..num_samples {
        let target_dist = (i as f32) * step_dist;

        while current_idx < distances.len() - 2 && distances[current_idx + 1] < target_dist {
            current_idx += 1;
        }

        let d0 = distances[current_idx];
        let d1 = distances[current_idx + 1];
        let segment_len = d1 - d0;

        let t = if segment_len > 1e-6 {
            ((target_dist - d0) / segment_len).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let p = polyline[current_idx].lerp(&polyline[current_idx + 1], t);
        sampled.push(p);
    }

    sampled
}

#[inline]
fn sample_cubic_bezier(p0: Point, p1: Point, p2: Point, p3: Point, t: f32) -> Point {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    Point::new(
        uuu * p0.x + 3.0 * uu * t * p1.x + 3.0 * u * tt * p2.x + ttt * p3.x,
        uuu * p0.y + 3.0 * uu * t * p1.y + 3.0 * u * tt * p2.y + ttt * p3.y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::path::IconPath;

    // --- centroid_of_points ---

    #[test]
    fn test_centroid_empty() {
        let pts: Vec<Point> = vec![];
        assert_eq!(centroid_of_points(&pts), Point::ZERO);
    }

    #[test]
    fn test_centroid_single_point() {
        let pts = vec![Point::new(5.0, 10.0)];
        let c = centroid_of_points(&pts);
        assert!((c.x - 5.0).abs() < 1e-6);
        assert!((c.y - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_centroid_symmetric() {
        let pts = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
        ];
        let c = centroid_of_points(&pts);
        assert!((c.x - 5.0).abs() < 1e-6);
        assert!((c.y - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_centroid_triangle() {
        let pts = vec![
            Point::new(0.0, 0.0),
            Point::new(6.0, 0.0),
            Point::new(3.0, 6.0),
        ];
        let c = centroid_of_points(&pts);
        assert!((c.x - 3.0).abs() < 1e-6);
        assert!((c.y - 2.0).abs() < 1e-6);
    }

    // --- sample_subpath ---

    #[test]
    fn test_sample_subpath_empty() {
        let sp = SubPath::new();
        let samples = sample_subpath(&sp, 10);
        assert_eq!(samples.len(), 10);
        // All should be ZERO
        for p in &samples {
            assert_eq!(*p, Point::ZERO);
        }
    }

    #[test]
    fn test_sample_subpath_zero_samples() {
        let sp = SubPath::new();
        let samples = sample_subpath(&sp, 0);
        assert_eq!(samples.len(), 1); // max(1) fallback
    }

    #[test]
    fn test_sample_subpath_single_point() {
        let mut sp = SubPath::new();
        sp.segments.push(PathSegment::MoveTo(Point::new(5.0, 5.0)));
        let samples = sample_subpath(&sp, 8);
        assert_eq!(samples.len(), 8);
        // All samples should be the single point
        for p in &samples {
            assert!((p.x - 5.0).abs() < 1e-6);
            assert!((p.y - 5.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_sample_subpath_line_endpoints() {
        let mut sp = SubPath::new();
        sp.segments.push(PathSegment::MoveTo(Point::new(0.0, 0.0)));
        sp.segments.push(PathSegment::LineTo(Point::new(10.0, 0.0)));
        let samples = sample_subpath(&sp, 11);
        assert_eq!(samples.len(), 11);
        // First point should be (0,0)
        assert!((samples[0].x).abs() < 1e-4);
        assert!((samples[0].y).abs() < 1e-4);
        // Last point should be (10,0)
        assert!((samples[10].x - 10.0).abs() < 1e-4);
        assert!((samples[10].y).abs() < 1e-4);
        // Middle point should be (5,0)
        assert!((samples[5].x - 5.0).abs() < 1e-4);
        assert!((samples[5].y).abs() < 1e-4);
    }

    #[test]
    fn test_sample_subpath_uniform_spacing() {
        let mut sp = SubPath::new();
        sp.segments.push(PathSegment::MoveTo(Point::new(0.0, 0.0)));
        sp.segments
            .push(PathSegment::LineTo(Point::new(100.0, 0.0)));
        let n = 21;
        let samples = sample_subpath(&sp, n);
        // Points should be uniformly spaced at 5.0 intervals
        for (i, sample) in samples.iter().enumerate().take(n) {
            let expected_x = (i as f32) * 5.0;
            assert!((sample.x - expected_x).abs() < 0.1);
            assert!((sample.y).abs() < 1e-4);
        }
    }

    #[test]
    fn test_sample_subpath_cubic_bezier() {
        let mut sp = SubPath::new();
        sp.segments.push(PathSegment::MoveTo(Point::new(0.0, 0.0)));
        sp.segments.push(PathSegment::CubicTo {
            ctrl1: Point::new(0.0, 10.0),
            ctrl2: Point::new(10.0, 10.0),
            end: Point::new(10.0, 0.0),
        });
        let samples = sample_subpath(&sp, 64);
        assert_eq!(samples.len(), 64);
        // First and last points should match start/end
        assert!((samples[0].x).abs() < 0.1);
        assert!((samples[0].y).abs() < 0.1);
        assert!((samples[63].x - 10.0).abs() < 0.1);
        assert!((samples[63].y).abs() < 0.1);
        // Middle samples should be above y=0 (the curve bulges up)
        assert!(samples[32].y > 0.0);
    }

    #[test]
    fn test_sample_subpath_closed() {
        let mut sp = SubPath::new();
        sp.segments.push(PathSegment::MoveTo(Point::new(0.0, 0.0)));
        sp.segments.push(PathSegment::LineTo(Point::new(10.0, 0.0)));
        sp.segments
            .push(PathSegment::LineTo(Point::new(10.0, 10.0)));
        sp.segments.push(PathSegment::Close);
        sp.is_closed = true;
        let samples = sample_subpath(&sp, 64);
        assert_eq!(samples.len(), 64);
        // First point should be (0,0)
        assert!((samples[0].x).abs() < 0.1);
        assert!((samples[0].y).abs() < 0.1);
    }

    #[test]
    fn test_sample_subpath_degenerate_zero_length() {
        // A line of zero length (same start and end point)
        let mut sp = SubPath::new();
        sp.segments.push(PathSegment::MoveTo(Point::new(5.0, 5.0)));
        sp.segments.push(PathSegment::LineTo(Point::new(5.0, 5.0)));
        let samples = sample_subpath(&sp, 10);
        assert_eq!(samples.len(), 10);
        // All should be at the same point
        for p in &samples {
            assert!((p.x - 5.0).abs() < 1e-4);
            assert!((p.y - 5.0).abs() < 1e-4);
        }
    }

    // --- SampledIcon::sample ---

    #[test]
    fn test_sampled_icon_sample_basic() {
        let icon_path = IconPath::parse("M0 0 L10 0 M0 5 L10 5").unwrap();
        let sampled = SampledIcon::sample(&icon_path, 32);
        assert_eq!(sampled.subpaths.len(), 2);
        assert_eq!(sampled.subpaths[0].len(), 32);
        assert_eq!(sampled.subpaths[1].len(), 32);
    }

    #[test]
    fn test_sampled_icon_sample_single_subpath() {
        let icon_path = IconPath::parse("M0 0 L24 24").unwrap();
        let sampled = SampledIcon::sample(&icon_path, 64);
        assert_eq!(sampled.subpaths.len(), 1);
        assert_eq!(sampled.subpaths[0].len(), 64);
    }

    // --- SampledIcon::align_with ---

    #[test]
    fn test_align_with_same_subpath_count() {
        let icon_a = IconPath::parse("M0 0 L10 0 M0 5 L10 5").unwrap();
        let icon_b = IconPath::parse("M0 0 L20 0 M0 10 L20 10").unwrap();
        let sampled_a = SampledIcon::sample(&icon_a, 32);
        let sampled_b = SampledIcon::sample(&icon_b, 32);
        let (aligned_a, aligned_b) = sampled_a.align_with(sampled_b, 32);
        assert_eq!(aligned_a.subpaths.len(), aligned_b.subpaths.len());
        assert_eq!(aligned_a.subpaths.len(), 2);
    }

    #[test]
    fn test_align_with_different_subpath_counts() {
        let icon_a = IconPath::parse("M0 0 L10 0").unwrap(); // 1 subpath
        let icon_b = IconPath::parse("M0 0 L10 0 M0 5 L10 5 M0 10 L10 10").unwrap(); // 3 subpaths
        let sampled_a = SampledIcon::sample(&icon_a, 16);
        let sampled_b = SampledIcon::sample(&icon_b, 16);
        let (aligned_a, aligned_b) = sampled_a.align_with(sampled_b, 16);
        assert_eq!(aligned_a.subpaths.len(), 3);
        assert_eq!(aligned_b.subpaths.len(), 3);
        // All subpaths should have 16 samples
        for sp in &aligned_a.subpaths {
            assert_eq!(sp.len(), 16);
        }
    }

    #[test]
    fn test_align_with_empty_a() {
        let sampled_a = SampledIcon { subpaths: vec![] };
        let icon_b = IconPath::parse("M0 0 L10 10").unwrap();
        let sampled_b = SampledIcon::sample(&icon_b, 8);
        let (aligned_a, aligned_b) = sampled_a.align_with(sampled_b, 8);
        assert_eq!(aligned_a.subpaths.len(), aligned_b.subpaths.len());
        assert!(!aligned_a.subpaths.is_empty());
    }

    #[test]
    fn test_align_with_both_empty() {
        let sampled_a = SampledIcon { subpaths: vec![] };
        let sampled_b = SampledIcon { subpaths: vec![] };
        let (aligned_a, aligned_b) = sampled_a.align_with(sampled_b, 8);
        // Both should get a default single subpath
        assert_eq!(aligned_a.subpaths.len(), 1);
        assert_eq!(aligned_b.subpaths.len(), 1);
        assert_eq!(aligned_a.subpaths[0].len(), 8);
    }

    #[test]
    fn test_align_padded_subpaths_are_at_centroid() {
        let icon_a = IconPath::parse("M0 0 L10 0").unwrap(); // 1 subpath
        let icon_b = IconPath::parse("M0 0 L10 0 M20 20 L30 20").unwrap(); // 2 subpaths
        let sampled_a = SampledIcon::sample(&icon_a, 8);
        let sampled_b = SampledIcon::sample(&icon_b, 8);
        let (aligned_a, _aligned_b) = sampled_a.align_with(sampled_b, 8);
        // The padded second subpath in A should be collapsed at centroid of B's second subpath
        let padded = &aligned_a.subpaths[1];
        // All points should be the same (collapsed)
        let first = padded[0];
        for p in padded {
            assert!((p.x - first.x).abs() < 1e-4);
            assert!((p.y - first.y).abs() < 1e-4);
        }
    }

    // --- sample_cubic_bezier (tested indirectly) ---

    #[test]
    fn test_cubic_bezier_endpoints() {
        // A straight-line cubic (all points collinear)
        let mut sp = SubPath::new();
        sp.segments.push(PathSegment::MoveTo(Point::new(0.0, 0.0)));
        sp.segments.push(PathSegment::CubicTo {
            ctrl1: Point::new(10.0, 0.0),
            ctrl2: Point::new(20.0, 0.0),
            end: Point::new(30.0, 0.0),
        });
        let samples = sample_subpath(&sp, 4);
        // Should be roughly linear
        assert!((samples[0].x - 0.0).abs() < 0.1);
        assert!((samples[3].x - 30.0).abs() < 0.1);
        // Middle should be roughly 10 and 20
        assert!((samples[1].x - 10.0).abs() < 1.0);
        assert!((samples[2].x - 20.0).abs() < 1.0);
    }

    // --- SAMPLES_PER_SUBPATH constant ---

    #[test]
    fn test_default_samples_constant() {
        assert_eq!(SAMPLES_PER_SUBPATH, 64);
    }

    // --- Integration: full icon sampling ---

    #[test]
    fn test_sample_real_icon_path() {
        let icon_path = IconPath::parse("M4 6h16M4 12h16M4 18h16").unwrap(); // menu icon
        let sampled = SampledIcon::sample(&icon_path, SAMPLES_PER_SUBPATH);
        assert_eq!(sampled.subpaths.len(), 3);
        for sp in &sampled.subpaths {
            assert_eq!(sp.len(), SAMPLES_PER_SUBPATH);
        }
    }

    #[test]
    fn test_sample_complex_icon() {
        let icon_path = IconPath::parse("M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z").unwrap();
        let sampled = SampledIcon::sample(&icon_path, 64);
        assert!(!sampled.subpaths.is_empty());
        assert_eq!(sampled.subpaths[0].len(), 64);
    }
}
