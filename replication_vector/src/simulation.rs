use std::f32::consts::PI;

const TAU: f32 = 2.0 * PI;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SimVec2 {
    pub x: f32,
    pub y: f32,
}

impl SimVec2 {
    pub const ZERO: SimVec2 = SimVec2 { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn length_squared(self) -> f32 {
        self.x.mul_add(self.x, self.y * self.y)
    }

    fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    fn clamp_length(self, max_length: f32) -> Self {
        if max_length <= 0.0 {
            return Self::ZERO;
        }

        let length = self.length();
        if length <= max_length || length <= f32::EPSILON {
            self
        } else {
            self * (max_length / length)
        }
    }
}

impl std::ops::Add for SimVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for SimVec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Mul<f32> for SimVec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParentProbeState {
    pub position: SimVec2,
    pub velocity: SimVec2,
    pub heading_radians: f32,
    pub angular_velocity_radians_per_second: f32,
}

impl Default for ParentProbeState {
    fn default() -> Self {
        Self {
            position: SimVec2::ZERO,
            velocity: SimVec2::ZERO,
            heading_radians: 0.0,
            angular_velocity_radians_per_second: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ParentProbeMotionInput {
    /// Normalized forward thrust in `[0.0, 1.0]`.
    ///
    /// Negative values are clamped to zero for this first simulation slice, so
    /// the parent probe has no reverse thrust until a later tuning item chooses
    /// otherwise.
    pub thrust: f32,
    /// Normalized turn input in `[-1.0, 1.0]`.
    pub turn: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParentProbeMotionConfig {
    pub thrust_acceleration: f32,
    pub turn_acceleration: f32,
    pub linear_drag: f32,
    pub angular_drag: f32,
    pub max_speed: f32,
    pub max_angular_speed: f32,
}

impl Default for ParentProbeMotionConfig {
    fn default() -> Self {
        Self {
            thrust_acceleration: 0.5,
            turn_acceleration: 2.0,
            linear_drag: 0.2,
            angular_drag: 1.5,
            max_speed: 1.5,
            max_angular_speed: PI,
        }
    }
}

pub fn step_parent_probe_motion(
    state: ParentProbeState,
    input: ParentProbeMotionInput,
    config: ParentProbeMotionConfig,
    delta_seconds: f32,
) -> ParentProbeState {
    let dt = delta_seconds.max(0.0);
    let turn = input.turn.clamp(-1.0, 1.0);
    let thrust = input.thrust.clamp(0.0, 1.0);

    let mut angular_velocity =
        state.angular_velocity_radians_per_second + turn * config.turn_acceleration * dt;
    angular_velocity = apply_drag(angular_velocity, config.angular_drag, dt);
    angular_velocity = clamp_abs(angular_velocity, config.max_angular_speed);

    let heading = normalize_heading(state.heading_radians + angular_velocity * dt);
    let forward = SimVec2::new(heading.cos(), heading.sin());

    let mut velocity = state.velocity + forward * (thrust * config.thrust_acceleration * dt);
    velocity = velocity * drag_factor(config.linear_drag, dt);
    velocity = velocity.clamp_length(config.max_speed);

    let position = state.position + velocity * dt;

    ParentProbeState {
        position,
        velocity,
        heading_radians: heading,
        angular_velocity_radians_per_second: angular_velocity,
    }
}

fn apply_drag(value: f32, drag: f32, delta_seconds: f32) -> f32 {
    value * drag_factor(drag, delta_seconds)
}

fn drag_factor(drag: f32, delta_seconds: f32) -> f32 {
    (1.0 - drag.max(0.0) * delta_seconds).clamp(0.0, 1.0)
}

fn clamp_abs(value: f32, max_abs: f32) -> f32 {
    let max_abs = max_abs.max(0.0);
    value.clamp(-max_abs, max_abs)
}

pub fn normalize_heading(heading_radians: f32) -> f32 {
    (heading_radians + PI).rem_euclid(TAU) - PI
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.0001;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() <= EPSILON,
            "expected {actual} to be within {EPSILON} of {expected}",
        );
    }

    fn config_without_drag() -> ParentProbeMotionConfig {
        ParentProbeMotionConfig {
            linear_drag: 0.0,
            angular_drag: 0.0,
            ..ParentProbeMotionConfig::default()
        }
    }

    #[test]
    fn zero_input_keeps_resting_probe_stable() {
        let state = step_parent_probe_motion(
            ParentProbeState::default(),
            ParentProbeMotionInput::default(),
            ParentProbeMotionConfig::default(),
            1.0,
        );

        assert_eq!(state, ParentProbeState::default());
    }

    #[test]
    fn positive_thrust_accelerates_along_heading() {
        let state = step_parent_probe_motion(
            ParentProbeState::default(),
            ParentProbeMotionInput {
                thrust: 1.0,
                turn: 0.0,
            },
            config_without_drag(),
            1.0,
        );

        assert_close(state.velocity.x, 0.5);
        assert_close(state.velocity.y, 0.0);
        assert_close(state.position.x, 0.5);
        assert_close(state.position.y, 0.0);
    }

    #[test]
    fn negative_thrust_is_clamped_to_no_reverse_thrust() {
        let state = step_parent_probe_motion(
            ParentProbeState::default(),
            ParentProbeMotionInput {
                thrust: -1.0,
                turn: 0.0,
            },
            config_without_drag(),
            1.0,
        );

        assert_eq!(state, ParentProbeState::default());
    }

    #[test]
    fn turn_input_changes_angular_velocity_and_heading_over_time() {
        let state = step_parent_probe_motion(
            ParentProbeState::default(),
            ParentProbeMotionInput {
                thrust: 0.0,
                turn: 1.0,
            },
            config_without_drag(),
            0.5,
        );

        assert_close(state.angular_velocity_radians_per_second, 1.0);
        assert_close(state.heading_radians, 0.5);
    }

    #[test]
    fn drag_reduces_existing_linear_and_angular_velocity() {
        let state = step_parent_probe_motion(
            ParentProbeState {
                velocity: SimVec2::new(1.0, 0.0),
                angular_velocity_radians_per_second: 1.0,
                ..ParentProbeState::default()
            },
            ParentProbeMotionInput::default(),
            ParentProbeMotionConfig {
                linear_drag: 0.25,
                angular_drag: 0.5,
                ..ParentProbeMotionConfig::default()
            },
            1.0,
        );

        assert_close(state.velocity.x, 0.75);
        assert_close(state.angular_velocity_radians_per_second, 0.5);
        assert_close(state.position.x, 0.75);
    }

    #[test]
    fn max_linear_and_angular_speed_are_enforced() {
        let state = step_parent_probe_motion(
            ParentProbeState {
                velocity: SimVec2::new(4.0, 3.0),
                angular_velocity_radians_per_second: 10.0,
                ..ParentProbeState::default()
            },
            ParentProbeMotionInput {
                thrust: 1.0,
                turn: 1.0,
            },
            ParentProbeMotionConfig {
                linear_drag: 0.0,
                angular_drag: 0.0,
                max_speed: 2.0,
                max_angular_speed: 1.0,
                ..ParentProbeMotionConfig::default()
            },
            1.0,
        );

        assert_close(state.velocity.length(), 2.0);
        assert_close(state.angular_velocity_radians_per_second, 1.0);
    }

    #[test]
    fn heading_is_normalized_to_stable_range() {
        let state = step_parent_probe_motion(
            ParentProbeState {
                heading_radians: PI - 0.1,
                angular_velocity_radians_per_second: 1.0,
                ..ParentProbeState::default()
            },
            ParentProbeMotionInput::default(),
            config_without_drag(),
            1.0,
        );

        assert!(state.heading_radians >= -PI);
        assert!(state.heading_radians < PI);
        assert_close(state.heading_radians, -2.2415926);
    }

    #[test]
    fn repeated_fixed_step_updates_are_deterministic() {
        let inputs = [
            ParentProbeMotionInput {
                thrust: 1.0,
                turn: 0.25,
            },
            ParentProbeMotionInput {
                thrust: 0.0,
                turn: -0.5,
            },
            ParentProbeMotionInput {
                thrust: 0.75,
                turn: 0.0,
            },
        ];
        let config = ParentProbeMotionConfig::default();

        let first = inputs
            .iter()
            .cycle()
            .take(18)
            .fold(ParentProbeState::default(), |state, input| {
                step_parent_probe_motion(state, *input, config, 1.0 / 30.0)
            });
        let second = inputs
            .iter()
            .cycle()
            .take(18)
            .fold(ParentProbeState::default(), |state, input| {
                step_parent_probe_motion(state, *input, config, 1.0 / 30.0)
            });

        assert_eq!(first, second);
    }
}
