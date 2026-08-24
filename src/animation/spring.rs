/// Configuration for spring physics animation.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpringConfig {
    /// Tension / stiffness of the spring (higher = faster, stiffer).
    pub stiffness: f32,
    /// Friction / damping of the spring (higher = less bounce/overshoot).
    pub damping: f32,
    /// Mass of the object (default 1.0).
    pub mass: f32,
    /// Threshold under which animation is considered settled.
    pub precision: f32,
}

impl SpringConfig {
    /// Smooth, fluid spring configuration without overshoot.
    pub const SMOOTH: Self = Self {
        stiffness: 120.0,
        damping: 14.0,
        mass: 1.0,
        precision: 0.001,
    };

    /// Snappy, fast spring configuration for immediate UI feedback.
    pub const SNAPPY: Self = Self {
        stiffness: 240.0,
        damping: 20.0,
        mass: 1.0,
        precision: 0.001,
    };

    /// Bouncy spring configuration with visible elastic overshoot.
    pub const BOUNCY: Self = Self {
        stiffness: 200.0,
        damping: 8.0,
        mass: 1.0,
        precision: 0.001,
    };

    /// Responsive, balanced default spring configuration.
    pub const DEFAULT: Self = Self::SMOOTH;

    /// Gentle spring configuration.
    pub const GENTLE: Self = Self {
        stiffness: 90.0,
        damping: 15.0,
        mass: 1.0,
        precision: 0.001,
    };

    /// Slow motion spring for dramatic morphing transitions.
    pub const SLO_MO: Self = Self {
        stiffness: 50.0,
        damping: 10.0,
        mass: 1.0,
        precision: 0.001,
    };
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Damped spring oscillator state solver.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    pub config: SpringConfig,
    pub value: f32,
    pub velocity: f32,
    pub target: f32,
}

impl Spring {
    /// Creates a new spring with initial value.
    pub fn new(initial_value: f32, config: SpringConfig) -> Self {
        Self {
            config,
            value: initial_value,
            velocity: 0.0,
            target: initial_value,
        }
    }

    /// Sets a new target value for the spring while preserving current position and velocity.
    pub fn set_target(&mut self, new_target: f32) {
        self.target = new_target;
    }

    /// Advances the spring physics simulation forward by delta time `dt` in seconds.
    /// Returns `true` if the spring is still moving, or `false` if it has settled.
    pub fn update(&mut self, dt: f32) -> bool {
        if self.is_settled() && (self.value - self.target).abs() < self.config.precision {
            self.value = self.target;
            self.velocity = 0.0;
            return false;
        }

        // Clamp dt to avoid numerical instability on frame spikes
        let dt = dt.min(0.064);
        let sub_steps = (dt / 0.004).ceil() as usize;
        let sub_dt = dt / sub_steps as f32;

        for _ in 0..sub_steps {
            let force = -self.config.stiffness * (self.value - self.target)
                - self.config.damping * self.velocity;
            let accel = force / self.config.mass;
            self.velocity += accel * sub_dt;
            self.value += self.velocity * sub_dt;
        }

        if self.is_settled() {
            self.value = self.target;
            self.velocity = 0.0;
            false
        } else {
            true
        }
    }

    /// Returns `true` if velocity and target displacement are within precision thresholds.
    pub fn is_settled(&self) -> bool {
        (self.value - self.target).abs() <= self.config.precision
            && self.velocity.abs() <= self.config.precision * 10.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- SpringConfig ---

    #[test]
    fn test_spring_config_presets_are_valid() {
        let configs = [
            SpringConfig::SMOOTH,
            SpringConfig::SNAPPY,
            SpringConfig::BOUNCY,
            SpringConfig::GENTLE,
            SpringConfig::SLO_MO,
            SpringConfig::DEFAULT,
        ];
        for config in &configs {
            assert!(config.stiffness > 0.0);
            assert!(config.damping > 0.0);
            assert!(config.mass > 0.0);
            assert!(config.precision > 0.0);
        }
    }

    #[test]
    fn test_spring_config_default_is_smooth() {
        assert_eq!(
            SpringConfig::DEFAULT.stiffness,
            SpringConfig::SMOOTH.stiffness
        );
        assert_eq!(SpringConfig::DEFAULT.damping, SpringConfig::SMOOTH.damping);
    }

    #[test]
    fn test_spring_config_default_trait() {
        let config: SpringConfig = Default::default();
        assert_eq!(config, SpringConfig::DEFAULT);
    }

    // --- Spring::new ---

    #[test]
    fn test_spring_new() {
        let s = Spring::new(0.5, SpringConfig::SMOOTH);
        assert_eq!(s.value, 0.5);
        assert_eq!(s.target, 0.5);
        assert_eq!(s.velocity, 0.0);
        assert_eq!(s.config, SpringConfig::SMOOTH);
    }

    #[test]
    fn test_spring_new_at_zero() {
        let s = Spring::new(0.0, SpringConfig::BOUNCY);
        assert_eq!(s.value, 0.0);
        assert_eq!(s.target, 0.0);
        assert_eq!(s.velocity, 0.0);
    }

    // --- Spring::set_target ---

    #[test]
    fn test_set_target() {
        let mut s = Spring::new(0.0, SpringConfig::SMOOTH);
        s.set_target(1.0);
        assert_eq!(s.target, 1.0);
        // Value and velocity should remain unchanged
        assert_eq!(s.value, 0.0);
        assert_eq!(s.velocity, 0.0);
    }

    #[test]
    fn test_set_target_preserves_velocity() {
        let mut s = Spring::new(0.0, SpringConfig::SMOOTH);
        s.set_target(1.0);
        s.update(0.016); // gain some velocity
        let vel_before = s.velocity;
        s.set_target(0.5);
        assert_eq!(s.velocity, vel_before);
    }

    // --- Spring::is_settled ---

    #[test]
    fn test_is_settled_at_rest() {
        let s = Spring::new(1.0, SpringConfig::SMOOTH);
        assert!(s.is_settled());
    }

    #[test]
    fn test_is_settled_not_at_target() {
        let mut s = Spring::new(0.0, SpringConfig::SMOOTH);
        s.set_target(1.0);
        assert!(!s.is_settled());
    }

    #[test]
    fn test_is_settled_with_velocity() {
        let mut s = Spring::new(1.0, SpringConfig::SMOOTH);
        s.velocity = 1.0; // Has velocity even though at target
        assert!(!s.is_settled());
    }

    // --- Spring::update ---

    #[test]
    fn test_update_returns_false_when_settled() {
        let mut s = Spring::new(1.0, SpringConfig::SMOOTH);
        let active = s.update(0.016);
        assert!(!active);
    }

    #[test]
    fn test_update_returns_true_when_animating() {
        let mut s = Spring::new(0.0, SpringConfig::SMOOTH);
        s.set_target(1.0);
        let active = s.update(0.016);
        assert!(active);
    }

    #[test]
    fn test_update_moves_toward_target() {
        let mut s = Spring::new(0.0, SpringConfig::SMOOTH);
        s.set_target(1.0);
        s.update(0.016);
        assert!(s.value > 0.0);
        assert!(s.value < 1.0);
    }

    #[test]
    fn test_update_converges_to_target() {
        let mut s = Spring::new(0.0, SpringConfig::SNAPPY);
        s.set_target(1.0);
        // Run for 2 seconds of simulated time
        for _ in 0..125 {
            s.update(0.016);
        }
        assert!((s.value - 1.0).abs() < 0.01);
        assert!(s.is_settled());
    }

    #[test]
    fn test_update_converges_all_configs() {
        let configs = [
            SpringConfig::SMOOTH,
            SpringConfig::SNAPPY,
            SpringConfig::BOUNCY,
            SpringConfig::GENTLE,
            SpringConfig::SLO_MO,
        ];
        for config in &configs {
            let mut s = Spring::new(0.0, *config);
            s.set_target(1.0);
            for _ in 0..500 {
                s.update(0.016);
            }
            assert!(
                (s.value - 1.0).abs() < 0.01,
                "Spring with config {:?} did not converge: value={}",
                config,
                s.value
            );
        }
    }

    #[test]
    fn test_update_clamps_large_dt() {
        let mut s = Spring::new(0.0, SpringConfig::SMOOTH);
        s.set_target(1.0);
        // Very large dt should not cause instability
        s.update(1.0); // 1 second, but clamped to 0.064
        assert!(!s.value.is_nan());
        assert!(!s.value.is_infinite());
        assert!(s.value > 0.0);
    }

    #[test]
    fn test_update_zero_dt() {
        let mut s = Spring::new(0.0, SpringConfig::SMOOTH);
        s.set_target(1.0);
        let val_before = s.value;
        s.update(0.0);
        // With dt=0, sub_steps = ceil(0/0.004) = 0, so no change
        // Actually ceil(0/0.004) = 0.0.ceil() = 0, loop doesn't execute
        assert_eq!(s.value, val_before);
    }

    #[test]
    fn test_bouncy_overshoots() {
        let mut s = Spring::new(0.0, SpringConfig::BOUNCY);
        s.set_target(1.0);
        let mut max_value = 0.0f32;
        for _ in 0..200 {
            s.update(0.016);
            max_value = max_value.max(s.value);
        }
        // BOUNCY should overshoot past 1.0
        assert!(max_value > 1.0);
    }

    #[test]
    fn test_smooth_less_overshoot_than_bouncy() {
        let mut smooth = Spring::new(0.0, SpringConfig::SMOOTH);
        smooth.set_target(1.0);
        let mut max_smooth = 0.0f32;
        for _ in 0..300 {
            smooth.update(0.016);
            max_smooth = max_smooth.max(smooth.value);
        }

        let mut bouncy = Spring::new(0.0, SpringConfig::BOUNCY);
        bouncy.set_target(1.0);
        let mut max_bouncy = 0.0f32;
        for _ in 0..300 {
            bouncy.update(0.016);
            max_bouncy = max_bouncy.max(bouncy.value);
        }

        // SMOOTH should overshoot less than BOUNCY
        assert!(max_smooth < max_bouncy);
    }

    #[test]
    fn test_update_reverse_direction() {
        let mut s = Spring::new(1.0, SpringConfig::SNAPPY);
        s.set_target(0.0);
        s.update(0.016);
        assert!(s.value < 1.0);
    }

    #[test]
    fn test_interruptible_mid_animation() {
        let mut s = Spring::new(0.0, SpringConfig::SMOOTH);
        s.set_target(1.0);
        // Animate partway
        for _ in 0..10 {
            s.update(0.016);
        }
        let mid_value = s.value;
        assert!(mid_value > 0.0 && mid_value < 1.0);

        // Interrupt: reverse target
        s.set_target(0.0);
        for _ in 0..300 {
            s.update(0.016);
        }
        assert!((s.value - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_spring_settles_snaps_to_target() {
        let mut s = Spring::new(0.0, SpringConfig::SNAPPY);
        s.set_target(1.0);
        // Run until settled
        let mut settled = false;
        for _ in 0..500 {
            if !s.update(0.016) {
                settled = true;
                break;
            }
        }
        assert!(settled);
        assert_eq!(s.value, 1.0);
        assert_eq!(s.velocity, 0.0);
    }
}
