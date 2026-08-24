use crate::geometry::point::Point;
use crate::geometry::sampling::{centroid_of_points, SampledIcon};

/// Pre-aligned Procrustes morph pair containing spatial alignment data.
#[derive(Debug, Clone)]
pub struct ProcrustesMorphData {
    pub subpath_data: Vec<SubPathProcrustesData>,
}

#[derive(Debug, Clone)]
pub struct SubPathProcrustesData {
    pub centroid_a: Point,
    pub centroid_b: Point,
    pub rotation_angle: f32,
    pub centered_a: Vec<Point>,
    pub rotated_centered_b: Vec<Point>,
}

impl ProcrustesMorphData {
    /// Computes the 2D Procrustes alignment between `source` and `target` icon.
    pub fn compute(source: &SampledIcon, target: &SampledIcon) -> Self {
        let mut subpath_data = Vec::with_capacity(source.subpaths.len());

        for (sp_a, sp_b) in source.subpaths.iter().zip(target.subpaths.iter()) {
            let centroid_a = centroid_of_points(sp_a);
            let centroid_b = centroid_of_points(sp_b);

            let centered_a: Vec<Point> = sp_a.iter().map(|p| *p - centroid_a).collect();
            let centered_b: Vec<Point> = sp_b.iter().map(|p| *p - centroid_b).collect();

            // Compute 2D Cross-Covariance matrix H = A^T * B
            let mut h00 = 0.0f32;
            let mut h01 = 0.0f32;
            let mut h10 = 0.0f32;
            let mut h11 = 0.0f32;

            for (pa, pb) in centered_a.iter().zip(centered_b.iter()) {
                h00 += pa.x * pb.x;
                h01 += pa.x * pb.y;
                h10 += pa.y * pb.x;
                h11 += pa.y * pb.y;
            }

            // Optimal rotation angle theta using Procrustes analysis
            let rotation_angle = (h01 - h10).atan2(h00 + h11);

            // Rotate B back by -rotation_angle into A's coordinate space
            let rotated_centered_b: Vec<Point> = centered_b
                .iter()
                .map(|p| p.rotate(-rotation_angle))
                .collect();

            subpath_data.push(SubPathProcrustesData {
                centroid_a,
                centroid_b,
                rotation_angle,
                centered_a,
                rotated_centered_b,
            });
        }

        Self { subpath_data }
    }

    /// Evaluates the polar-interpolated points at progress `t` (0.0 to 1.0).
    pub fn interpolate(&self, t: f32) -> SampledIcon {
        let t = t.clamp(0.0, 1.0);
        let mut interpolated_subpaths = Vec::with_capacity(self.subpath_data.len());

        for sp_data in &self.subpath_data {
            let current_centroid = sp_data.centroid_a.lerp(&sp_data.centroid_b, t);
            let current_rotation = sp_data.rotation_angle * t;

            let mut pts = Vec::with_capacity(sp_data.centered_a.len());

            for (pa, pb) in sp_data
                .centered_a
                .iter()
                .zip(sp_data.rotated_centered_b.iter())
            {
                let local_vec = pa.lerp(pb, t);
                let rotated_vec = local_vec.rotate(current_rotation);
                pts.push(current_centroid + rotated_vec);
            }

            interpolated_subpaths.push(pts);
        }

        SampledIcon {
            subpaths: interpolated_subpaths,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::path::IconPath;
    use crate::geometry::sampling::SampledIcon;
    use std::f32::consts::FRAC_PI_2;

    fn make_sampled_line(start: Point, end: Point, n: usize) -> Vec<Point> {
        (0..n)
            .map(|i| start.lerp(&end, i as f32 / (n - 1) as f32))
            .collect()
    }

    // --- ProcrustesMorphData::compute ---

    #[test]
    fn test_compute_identical_shapes() {
        let pts = make_sampled_line(Point::new(0.0, 0.0), Point::new(10.0, 0.0), 16);
        let source = SampledIcon {
            subpaths: vec![pts.clone()],
        };
        let target = SampledIcon {
            subpaths: vec![pts],
        };
        let data = ProcrustesMorphData::compute(&source, &target);
        assert_eq!(data.subpath_data.len(), 1);
        // Identical shapes should have near-zero rotation
        assert!((data.subpath_data[0].rotation_angle).abs() < 1e-4);
    }

    #[test]
    fn test_compute_rotated_shape() {
        // Source: horizontal line, Target: vertical line (90° rotation)
        let n = 16;
        let source_pts = make_sampled_line(Point::new(-5.0, 0.0), Point::new(5.0, 0.0), n);
        let target_pts = make_sampled_line(Point::new(0.0, -5.0), Point::new(0.0, 5.0), n);
        let source = SampledIcon {
            subpaths: vec![source_pts],
        };
        let target = SampledIcon {
            subpaths: vec![target_pts],
        };
        let data = ProcrustesMorphData::compute(&source, &target);
        // Rotation should be approximately PI/2 (90 degrees)
        assert!((data.subpath_data[0].rotation_angle.abs() - FRAC_PI_2).abs() < 0.1);
    }

    #[test]
    fn test_compute_multiple_subpaths() {
        let n = 8;
        let sp1 = make_sampled_line(Point::new(0.0, 0.0), Point::new(5.0, 0.0), n);
        let sp2 = make_sampled_line(Point::new(0.0, 5.0), Point::new(5.0, 5.0), n);
        let source = SampledIcon {
            subpaths: vec![sp1.clone(), sp2.clone()],
        };
        let target = SampledIcon {
            subpaths: vec![sp1, sp2],
        };
        let data = ProcrustesMorphData::compute(&source, &target);
        assert_eq!(data.subpath_data.len(), 2);
    }

    #[test]
    fn test_compute_centroids() {
        let source_pts = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
        ];
        let target_pts = vec![
            Point::new(20.0, 20.0),
            Point::new(30.0, 20.0),
            Point::new(30.0, 30.0),
            Point::new(20.0, 30.0),
        ];
        let source = SampledIcon {
            subpaths: vec![source_pts],
        };
        let target = SampledIcon {
            subpaths: vec![target_pts],
        };
        let data = ProcrustesMorphData::compute(&source, &target);
        // Source centroid should be (5, 5)
        assert!((data.subpath_data[0].centroid_a.x - 5.0).abs() < 1e-4);
        assert!((data.subpath_data[0].centroid_a.y - 5.0).abs() < 1e-4);
        // Target centroid should be (25, 25)
        assert!((data.subpath_data[0].centroid_b.x - 25.0).abs() < 1e-4);
        assert!((data.subpath_data[0].centroid_b.y - 25.0).abs() < 1e-4);
    }

    // --- ProcrustesMorphData::interpolate ---

    #[test]
    fn test_interpolate_at_zero_returns_source() {
        let n = 16;
        let source_pts = make_sampled_line(Point::new(0.0, 0.0), Point::new(10.0, 0.0), n);
        let target_pts = make_sampled_line(Point::new(20.0, 20.0), Point::new(30.0, 20.0), n);
        let source = SampledIcon {
            subpaths: vec![source_pts.clone()],
        };
        let target = SampledIcon {
            subpaths: vec![target_pts],
        };
        let data = ProcrustesMorphData::compute(&source, &target);
        let result = data.interpolate(0.0);

        // At t=0, should be very close to source
        for (i, p) in result.subpaths[0].iter().enumerate() {
            assert!((p.x - source_pts[i].x).abs() < 0.5);
            assert!((p.y - source_pts[i].y).abs() < 0.5);
        }
    }

    #[test]
    fn test_interpolate_at_one_returns_target() {
        let n = 16;
        let source_pts = make_sampled_line(Point::new(0.0, 0.0), Point::new(10.0, 0.0), n);
        let target_pts = make_sampled_line(Point::new(0.0, 0.0), Point::new(10.0, 0.0), n);
        let source = SampledIcon {
            subpaths: vec![source_pts],
        };
        let target = SampledIcon {
            subpaths: vec![target_pts.clone()],
        };
        let data = ProcrustesMorphData::compute(&source, &target);
        let result = data.interpolate(1.0);

        // At t=1 with identical shapes, result should match target
        for (i, p) in result.subpaths[0].iter().enumerate() {
            assert!((p.x - target_pts[i].x).abs() < 0.5);
            assert!((p.y - target_pts[i].y).abs() < 0.5);
        }
    }

    #[test]
    fn test_interpolate_midpoint_centroid() {
        let source_pts = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
        ];
        let target_pts = vec![
            Point::new(20.0, 0.0),
            Point::new(30.0, 0.0),
            Point::new(30.0, 10.0),
            Point::new(20.0, 10.0),
        ];
        let source = SampledIcon {
            subpaths: vec![source_pts],
        };
        let target = SampledIcon {
            subpaths: vec![target_pts],
        };
        let data = ProcrustesMorphData::compute(&source, &target);
        let result = data.interpolate(0.5);

        // Centroid at t=0.5 should be midpoint between (5,5) and (25,5) = (15,5)
        let mid_centroid = centroid_of_points(&result.subpaths[0]);
        assert!((mid_centroid.x - 15.0).abs() < 0.5);
        assert!((mid_centroid.y - 5.0).abs() < 0.5);
    }

    #[test]
    fn test_interpolate_clamps_t() {
        let n = 8;
        let pts = make_sampled_line(Point::new(0.0, 0.0), Point::new(10.0, 0.0), n);
        let source = SampledIcon {
            subpaths: vec![pts.clone()],
        };
        let target = SampledIcon {
            subpaths: vec![pts],
        };
        let data = ProcrustesMorphData::compute(&source, &target);

        // t < 0 should be clamped to 0
        let r_neg = data.interpolate(-1.0);
        let r_zero = data.interpolate(0.0);
        for (a, b) in r_neg.subpaths[0].iter().zip(r_zero.subpaths[0].iter()) {
            assert!((a.x - b.x).abs() < 1e-6);
            assert!((a.y - b.y).abs() < 1e-6);
        }

        // t > 1 should be clamped to 1
        let r_over = data.interpolate(2.0);
        let r_one = data.interpolate(1.0);
        for (a, b) in r_over.subpaths[0].iter().zip(r_one.subpaths[0].iter()) {
            assert!((a.x - b.x).abs() < 1e-6);
            assert!((a.y - b.y).abs() < 1e-6);
        }
    }

    #[test]
    fn test_interpolate_multiple_subpaths() {
        let n = 8;
        let sp1_a = make_sampled_line(Point::new(0.0, 0.0), Point::new(5.0, 0.0), n);
        let sp2_a = make_sampled_line(Point::new(0.0, 10.0), Point::new(5.0, 10.0), n);
        let sp1_b = make_sampled_line(Point::new(10.0, 0.0), Point::new(15.0, 0.0), n);
        let sp2_b = make_sampled_line(Point::new(10.0, 10.0), Point::new(15.0, 10.0), n);

        let source = SampledIcon {
            subpaths: vec![sp1_a, sp2_a],
        };
        let target = SampledIcon {
            subpaths: vec![sp1_b, sp2_b],
        };
        let data = ProcrustesMorphData::compute(&source, &target);
        let result = data.interpolate(0.5);
        assert_eq!(result.subpaths.len(), 2);
        assert_eq!(result.subpaths[0].len(), n);
        assert_eq!(result.subpaths[1].len(), n);
    }

    // --- Integration with real icons ---

    #[test]
    fn test_procrustes_with_parsed_icons() {
        let from = IconPath::parse("M4 6h16M4 12h16M4 18h16").unwrap(); // menu
        let to = IconPath::parse("M18 6L6 18M6 6l12 12").unwrap(); // X
        let n = 64;
        let sampled_from = SampledIcon::sample(&from, n);
        let sampled_to = SampledIcon::sample(&to, n);
        let (aligned_from, aligned_to) = sampled_from.align_with(sampled_to, n);
        let data = ProcrustesMorphData::compute(&aligned_from, &aligned_to);

        // Should produce valid interpolated results at various t values
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let result = data.interpolate(t);
            assert_eq!(result.subpaths.len(), aligned_from.subpaths.len());
            for sp in &result.subpaths {
                assert_eq!(sp.len(), n);
                // No NaN values
                for p in sp {
                    assert!(!p.x.is_nan());
                    assert!(!p.y.is_nan());
                }
            }
        }
    }

    #[test]
    fn test_interpolation_is_smooth() {
        let n = 16;
        let source_pts = make_sampled_line(Point::new(0.0, 0.0), Point::new(10.0, 0.0), n);
        let target_pts = make_sampled_line(Point::new(0.0, 10.0), Point::new(10.0, 10.0), n);
        let source = SampledIcon {
            subpaths: vec![source_pts],
        };
        let target = SampledIcon {
            subpaths: vec![target_pts],
        };
        let data = ProcrustesMorphData::compute(&source, &target);

        // Check that sequential t values produce smoothly changing results
        let r1 = data.interpolate(0.0);
        let r2 = data.interpolate(0.1);
        let r3 = data.interpolate(0.2);

        let c1 = centroid_of_points(&r1.subpaths[0]);
        let c2 = centroid_of_points(&r2.subpaths[0]);
        let c3 = centroid_of_points(&r3.subpaths[0]);

        // Centroids should be monotonically moving
        let d12 = c1.distance(&c2);
        let d23 = c2.distance(&c3);
        assert!(d12 > 0.0);
        assert!(d23 > 0.0);
        // Steps should be roughly similar size (smooth)
        assert!((d12 - d23).abs() < d12 * 0.5);
    }
}
