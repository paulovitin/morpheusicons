use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

/// A 2D point represented as 32-bit floating point coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Creates a new 2D point.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Origin point (0, 0).
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Calculates Euclidean distance to another point.
    #[inline]
    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculates the squared Euclidean distance to another point.
    #[inline]
    pub fn distance_squared(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Calculates the magnitude (length) of the vector from origin.
    #[inline]
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Linearly interpolates between `self` and `other` by parameter `t` (0.0 to 1.0).
    #[inline]
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Rotates the point around the origin (0, 0) by an angle in radians.
    #[inline]
    pub fn rotate(&self, angle_rad: f32) -> Self {
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();
        Self {
            x: self.x * cos_a - self.y * sin_a,
            y: self.x * sin_a + self.y * cos_a,
        }
    }

    /// Dot product between self and another vector.
    #[inline]
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Point {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Point {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, PI};

    #[test]
    fn test_new_and_zero() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 4.0);
        assert_eq!(Point::ZERO, Point::new(0.0, 0.0));
    }

    #[test]
    fn test_distance() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);
        assert!((a.distance(&b) - 5.0).abs() < 1e-6);
        assert!((a.distance(&a)).abs() < 1e-6);
    }

    #[test]
    fn test_distance_squared() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(4.0, 6.0);
        assert!((a.distance_squared(&b) - 25.0).abs() < 1e-6);
    }

    #[test]
    fn test_length() {
        let p = Point::new(3.0, 4.0);
        assert!((p.length() - 5.0).abs() < 1e-6);
        assert!((Point::ZERO.length()).abs() < 1e-6);
    }

    #[test]
    fn test_lerp() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(10.0, 20.0);
        let mid = a.lerp(&b, 0.5);
        assert!((mid.x - 5.0).abs() < 1e-6);
        assert!((mid.y - 10.0).abs() < 1e-6);

        let start = a.lerp(&b, 0.0);
        assert_eq!(start, a);

        let end = a.lerp(&b, 1.0);
        assert!((end.x - b.x).abs() < 1e-6);
        assert!((end.y - b.y).abs() < 1e-6);
    }

    #[test]
    fn test_lerp_quarter() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(8.0, 4.0);
        let q = a.lerp(&b, 0.25);
        assert!((q.x - 2.0).abs() < 1e-6);
        assert!((q.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_rotate_90_degrees() {
        let p = Point::new(1.0, 0.0);
        let rotated = p.rotate(FRAC_PI_2);
        assert!((rotated.x - 0.0).abs() < 1e-5);
        assert!((rotated.y - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_rotate_180_degrees() {
        let p = Point::new(1.0, 0.0);
        let rotated = p.rotate(PI);
        assert!((rotated.x - (-1.0)).abs() < 1e-5);
        assert!((rotated.y - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_rotate_360_degrees() {
        let p = Point::new(3.0, 4.0);
        let rotated = p.rotate(2.0 * PI);
        assert!((rotated.x - p.x).abs() < 1e-4);
        assert!((rotated.y - p.y).abs() < 1e-4);
    }

    #[test]
    fn test_rotate_zero() {
        let p = Point::new(5.0, 7.0);
        let rotated = p.rotate(0.0);
        assert!((rotated.x - p.x).abs() < 1e-6);
        assert!((rotated.y - p.y).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product() {
        let a = Point::new(1.0, 0.0);
        let b = Point::new(0.0, 1.0);
        assert!((a.dot(&b)).abs() < 1e-6); // perpendicular

        let c = Point::new(2.0, 3.0);
        let d = Point::new(4.0, 5.0);
        assert!((c.dot(&d) - 23.0).abs() < 1e-6); // 2*4 + 3*5 = 23
    }

    #[test]
    fn test_add() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(3.0, 4.0);
        let c = a + b;
        assert_eq!(c, Point::new(4.0, 6.0));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Point::new(1.0, 2.0);
        a += Point::new(3.0, 4.0);
        assert_eq!(a, Point::new(4.0, 6.0));
    }

    #[test]
    fn test_sub() {
        let a = Point::new(5.0, 7.0);
        let b = Point::new(2.0, 3.0);
        let c = a - b;
        assert_eq!(c, Point::new(3.0, 4.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Point::new(5.0, 7.0);
        a -= Point::new(2.0, 3.0);
        assert_eq!(a, Point::new(3.0, 4.0));
    }

    #[test]
    fn test_mul_scalar() {
        let p = Point::new(2.0, 3.0);
        let scaled = p * 4.0;
        assert_eq!(scaled, Point::new(8.0, 12.0));
    }

    #[test]
    fn test_mul_zero() {
        let p = Point::new(2.0, 3.0);
        let scaled = p * 0.0;
        assert_eq!(scaled, Point::ZERO);
    }

    #[test]
    fn test_div_scalar() {
        let p = Point::new(10.0, 20.0);
        let divided = p / 2.0;
        assert_eq!(divided, Point::new(5.0, 10.0));
    }

    #[test]
    fn test_default() {
        let p: Point = Default::default();
        assert_eq!(p, Point::ZERO);
    }

    #[test]
    fn test_clone_and_copy() {
        let p = Point::new(1.0, 2.0);
        let q = p; // Copy
        let r = p.clone(); // Clone
        assert_eq!(p, q);
        assert_eq!(p, r);
    }

    #[test]
    fn test_negative_coordinates() {
        let a = Point::new(-3.0, -4.0);
        let b = Point::new(3.0, 4.0);
        assert!((a.distance(&b) - (6.0f32.powi(2) + 8.0f32.powi(2)).sqrt()).abs() < 1e-6);
        assert!((a.length() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_rotate_preserves_length() {
        let p = Point::new(3.0, 4.0);
        let original_len = p.length();
        for angle in [0.1, 0.5, 1.0, 2.0, 3.0, 5.0] {
            let rotated = p.rotate(angle);
            assert!((rotated.length() - original_len).abs() < 1e-4);
        }
    }
}
