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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParentProbeTuningSliders {
    /// Designer-facing mass feel in `[0.0, 1.0]`.
    ///
    /// Higher values reduce thrust acceleration and top speed.
    pub weight: f32,
    /// Designer-facing drift persistence in `[0.0, 1.0]`.
    ///
    /// Higher values reduce linear and angular damping.
    pub inertia: f32,
    /// Designer-facing control response in `[0.0, 1.0]`.
    ///
    /// Higher values increase turn acceleration and angular speed.
    pub responsiveness: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParentProbeReplaySample {
    pub step_index: u32,
    pub elapsed_seconds: f32,
    pub input: ParentProbeMotionInput,
    pub state: ParentProbeState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParentProbeReplay {
    pub delta_seconds: f32,
    pub total_steps: u32,
    pub samples: Vec<ParentProbeReplaySample>,
    pub final_state: ParentProbeState,
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

impl Default for ParentProbeTuningSliders {
    fn default() -> Self {
        Self {
            weight: 0.5,
            inertia: 0.5,
            responsiveness: 0.5,
        }
    }
}

pub fn parent_probe_motion_config_from_tuning(
    sliders: ParentProbeTuningSliders,
) -> ParentProbeMotionConfig {
    let weight = unit_interval(sliders.weight);
    let inertia = unit_interval(sliders.inertia);
    let responsiveness = unit_interval(sliders.responsiveness);

    ParentProbeMotionConfig {
        thrust_acceleration: lerp(0.8, 0.2, weight),
        turn_acceleration: lerp(1.0, 3.0, responsiveness),
        linear_drag: lerp(0.35, 0.05, inertia),
        angular_drag: lerp(2.4, 0.6, inertia),
        max_speed: lerp(2.0, 1.0, weight),
        max_angular_speed: PI * lerp(0.55, 1.45, responsiveness),
    }
}

pub fn step_parent_probe_motion_with_tuning(
    state: ParentProbeState,
    input: ParentProbeMotionInput,
    sliders: ParentProbeTuningSliders,
    delta_seconds: f32,
) -> ParentProbeState {
    step_parent_probe_motion(
        state,
        input,
        parent_probe_motion_config_from_tuning(sliders),
        delta_seconds.clamp(0.0, 0.05),
    )
}

pub fn deterministic_parent_probe_replay() -> ParentProbeReplay {
    const DELTA_SECONDS: f32 = 1.0 / 30.0;
    const STEPS_PER_INPUT: u32 = 12;
    const SCRIPTED_INPUTS: [ParentProbeMotionInput; 4] = [
        ParentProbeMotionInput {
            thrust: 1.0,
            turn: 0.35,
        },
        ParentProbeMotionInput {
            thrust: 0.85,
            turn: -0.2,
        },
        ParentProbeMotionInput {
            thrust: 0.35,
            turn: 0.0,
        },
        ParentProbeMotionInput {
            thrust: 0.0,
            turn: 0.0,
        },
    ];

    let config = ParentProbeMotionConfig::default();
    let mut state = ParentProbeState::default();
    let mut samples = Vec::with_capacity(SCRIPTED_INPUTS.len() * STEPS_PER_INPUT as usize);
    let mut step_index = 0;

    for input in SCRIPTED_INPUTS {
        for _ in 0..STEPS_PER_INPUT {
            step_index += 1;
            state = step_parent_probe_motion(state, input, config, DELTA_SECONDS);
            samples.push(ParentProbeReplaySample {
                step_index,
                elapsed_seconds: step_index as f32 * DELTA_SECONDS,
                input,
                state,
            });
        }
    }

    ParentProbeReplay {
        delta_seconds: DELTA_SECONDS,
        total_steps: step_index,
        samples,
        final_state: state,
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

fn unit_interval(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

fn lerp(min: f32, max: f32, amount: f32) -> f32 {
    min + (max - min) * amount
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
    fn default_tuning_maps_to_existing_default_motion_config() {
        let config = parent_probe_motion_config_from_tuning(ParentProbeTuningSliders::default());

        assert_close(
            config.thrust_acceleration,
            ParentProbeMotionConfig::default().thrust_acceleration,
        );
        assert_close(
            config.turn_acceleration,
            ParentProbeMotionConfig::default().turn_acceleration,
        );
        assert_close(
            config.linear_drag,
            ParentProbeMotionConfig::default().linear_drag,
        );
        assert_close(
            config.angular_drag,
            ParentProbeMotionConfig::default().angular_drag,
        );
        assert_close(
            config.max_speed,
            ParentProbeMotionConfig::default().max_speed,
        );
        assert_close(
            config.max_angular_speed,
            ParentProbeMotionConfig::default().max_angular_speed,
        );
    }

    #[test]
    fn tuning_sliders_clamp_and_map_to_config_extremes() {
        let light_responsive = parent_probe_motion_config_from_tuning(ParentProbeTuningSliders {
            weight: -2.0,
            inertia: 0.0,
            responsiveness: 2.0,
        });
        let heavy_inert = parent_probe_motion_config_from_tuning(ParentProbeTuningSliders {
            weight: 1.0,
            inertia: 1.0,
            responsiveness: 0.0,
        });

        assert_close(light_responsive.thrust_acceleration, 0.8);
        assert_close(light_responsive.linear_drag, 0.35);
        assert_close(light_responsive.turn_acceleration, 3.0);
        assert_close(light_responsive.max_angular_speed, PI * 1.45);
        assert_close(heavy_inert.thrust_acceleration, 0.2);
        assert_close(heavy_inert.linear_drag, 0.05);
        assert_close(heavy_inert.angular_drag, 0.6);
        assert_close(heavy_inert.max_speed, 1.0);
    }

    #[test]
    fn tuning_step_bounds_large_browser_delta() {
        let state = step_parent_probe_motion_with_tuning(
            ParentProbeState::default(),
            ParentProbeMotionInput {
                thrust: 1.0,
                turn: 1.0,
            },
            ParentProbeTuningSliders::default(),
            2.0,
        );
        let bounded = step_parent_probe_motion(
            ParentProbeState::default(),
            ParentProbeMotionInput {
                thrust: 1.0,
                turn: 1.0,
            },
            ParentProbeMotionConfig::default(),
            0.05,
        );

        assert_eq!(state, bounded);
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

    #[test]
    fn deterministic_replay_records_final_state_from_motion_model() {
        let replay = deterministic_parent_probe_replay();

        assert_eq!(replay.total_steps, 48);
        assert_eq!(replay.samples.len(), replay.total_steps as usize);
        assert_eq!(replay.samples.last().unwrap().state, replay.final_state);
        assert!(replay.final_state.position.x > 0.1);
        assert!(replay.final_state.heading_radians > 0.0);
    }

    #[test]
    fn deterministic_replay_is_stable_across_runs() {
        let first = deterministic_parent_probe_replay();
        let second = deterministic_parent_probe_replay();

        assert_eq!(first, second);
    }
}
