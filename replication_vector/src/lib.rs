#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use webgpu_vector_lib::VectorFrame;
use webgpu_vector_lib::{Color, Line, Polyline, StrokeStyle, Vec2, VectorCommand};

pub mod simulation;

use simulation::{ParentProbeReplay, ParentProbeState, deterministic_parent_probe_replay};

const CYAN: Color = Color {
    red: 0.2,
    green: 0.95,
    blue: 1.0,
    alpha: 1.0,
};

const GREEN: Color = Color {
    red: 0.45,
    green: 1.0,
    blue: 0.65,
    alpha: 1.0,
};

const AMBER: Color = Color {
    red: 1.0,
    green: 0.78,
    blue: 0.25,
    alpha: 1.0,
};

const BLUE: Color = Color {
    red: 0.5,
    green: 0.75,
    blue: 1.0,
    alpha: 1.0,
};

const PARENT_PROBE_OUTLINE: [[f32; 2]; 7] = [
    [-0.32, 0.0],
    [0.18, 0.24],
    [0.07, 0.06],
    [0.42, 0.0],
    [0.07, -0.06],
    [0.18, -0.24],
    [-0.32, 0.0],
];

const ASTEROID_OUTLINE: [[f32; 2]; 8] = [
    [-0.78, 0.28],
    [-0.68, 0.42],
    [-0.49, 0.46],
    [-0.38, 0.31],
    [-0.43, 0.12],
    [-0.63, 0.05],
    [-0.79, 0.14],
    [-0.78, 0.28],
];

const SHIELD_ARC: [[f32; 2]; 6] = [
    [-0.5, -0.3],
    [-0.4, -0.5],
    [-0.18, -0.62],
    [0.06, -0.61],
    [0.28, -0.48],
    [0.38, -0.3],
];

fn point([x, y]: [f32; 2]) -> Vec2 {
    Vec2 { x, y }
}

fn stroke(width: f32, color: Color, intensity: f32) -> StrokeStyle {
    StrokeStyle {
        width,
        color,
        intensity,
    }
}

fn line(start: [f32; 2], end: [f32; 2], style: StrokeStyle) -> VectorCommand {
    VectorCommand::Line(Line {
        start: point(start),
        end: point(end),
        style,
    })
}

fn polyline(points: &[[f32; 2]], style: StrokeStyle) -> VectorCommand {
    VectorCommand::Polyline(Polyline {
        points: points.iter().copied().map(point).collect(),
        style,
    })
}

fn transform_parent_probe_outline(state: ParentProbeState) -> Vec<[f32; 2]> {
    let (sin, cos) = state.heading_radians.sin_cos();
    PARENT_PROBE_OUTLINE
        .iter()
        .map(|[x, y]| {
            [
                x * cos - y * sin + state.position.x,
                x * sin + y * cos + state.position.y,
            ]
        })
        .collect()
}

fn polyline_from_vec(points: Vec<[f32; 2]>, style: StrokeStyle) -> VectorCommand {
    VectorCommand::Polyline(Polyline {
        points: points.into_iter().map(point).collect(),
        style,
    })
}

fn scene_for_parent_state(state: ParentProbeState) -> Vec<VectorCommand> {
    vec![
        polyline_from_vec(
            transform_parent_probe_outline(state),
            stroke(0.018, CYAN, 1.15),
        ),
        polyline(&ASTEROID_OUTLINE, stroke(0.014, AMBER, 0.95)),
        polyline(&SHIELD_ARC, stroke(0.012, BLUE, 0.85)),
        line([0.48, 0.1], [0.78, 0.2], stroke(0.01, GREEN, 1.25)),
    ]
}

pub fn first_replication_vector_scene() -> Vec<VectorCommand> {
    vec![
        polyline(&PARENT_PROBE_OUTLINE, stroke(0.018, CYAN, 1.15)),
        polyline(&ASTEROID_OUTLINE, stroke(0.014, AMBER, 0.95)),
        polyline(&SHIELD_ARC, stroke(0.012, BLUE, 0.85)),
        line([0.48, 0.1], [0.78, 0.2], stroke(0.01, GREEN, 1.25)),
    ]
}

pub fn deterministic_parent_probe_replay_scene() -> Vec<VectorCommand> {
    let replay = deterministic_parent_probe_replay();
    scene_for_parent_state(replay.final_state)
}

pub fn first_replication_vector_scene_command_count() -> usize {
    first_replication_vector_scene().len()
}

pub fn deterministic_parent_probe_replay_command_count() -> usize {
    deterministic_parent_probe_replay_scene().len()
}

pub fn deterministic_parent_probe_replay_metadata_json() -> String {
    let replay = deterministic_parent_probe_replay();
    replay_metadata_json(&replay)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn replication_vector_scene_command_count() -> u32 {
    first_replication_vector_scene_command_count() as u32
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn replication_vector_replay_command_count() -> u32 {
    deterministic_parent_probe_replay_command_count() as u32
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn replication_vector_replay_metadata_json() -> String {
    deterministic_parent_probe_replay_metadata_json()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn replication_vector_first_scene_frame() -> Result<VectorFrame, JsValue> {
    vector_frame_from_commands(first_replication_vector_scene())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn replication_vector_replay_frame() -> Result<VectorFrame, JsValue> {
    vector_frame_from_commands(deterministic_parent_probe_replay_scene())
}

#[cfg(target_arch = "wasm32")]
fn vector_frame_from_commands(commands: Vec<VectorCommand>) -> Result<VectorFrame, JsValue> {
    let mut frame = VectorFrame::new();

    for command in commands {
        match command {
            VectorCommand::Line(line) => push_frame_line(&mut frame, &line)?,
            VectorCommand::Polyline(polyline) => push_frame_polyline(&mut frame, &polyline)?,
        }
    }

    Ok(frame)
}

fn replay_metadata_json(replay: &ParentProbeReplay) -> String {
    let final_state = replay.final_state;
    format!(
        concat!(
            "{{",
            "\"kind\":\"deterministic-parent-probe-replay\",",
            "\"deltaSeconds\":{:.8},",
            "\"totalSteps\":{},",
            "\"capturedStep\":{},",
            "\"elapsedSeconds\":{:.8},",
            "\"finalState\":{{",
            "\"position\":{{\"x\":{:.8},\"y\":{:.8}}},",
            "\"velocity\":{{\"x\":{:.8},\"y\":{:.8}}},",
            "\"headingRadians\":{:.8},",
            "\"angularVelocityRadiansPerSecond\":{:.8}",
            "}}",
            "}}"
        ),
        replay.delta_seconds,
        replay.total_steps,
        replay.total_steps,
        replay.total_steps as f32 * replay.delta_seconds,
        final_state.position.x,
        final_state.position.y,
        final_state.velocity.x,
        final_state.velocity.y,
        final_state.heading_radians,
        final_state.angular_velocity_radians_per_second,
    )
}

#[cfg(target_arch = "wasm32")]
fn push_frame_line(frame: &mut VectorFrame, line: &Line) -> Result<(), JsValue> {
    let style = line.style;
    frame.js_line(
        line.start.x,
        line.start.y,
        line.end.x,
        line.end.y,
        style.color.red,
        style.color.green,
        style.color.blue,
        style.color.alpha,
        style.width,
        style.intensity,
    )
}

#[cfg(target_arch = "wasm32")]
fn push_frame_polyline(frame: &mut VectorFrame, polyline: &Polyline) -> Result<(), JsValue> {
    let points: Vec<f32> = polyline
        .points
        .iter()
        .flat_map(|point| [point.x, point.y])
        .collect();
    let style = polyline.style;

    frame.js_polyline(
        &points,
        style.color.red,
        style.color.green,
        style.color.blue,
        style.color.alpha,
        style.width,
        style.intensity,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_scene_uses_velumin_vector_commands_for_required_primitives() {
        let scene = first_replication_vector_scene();

        assert_eq!(scene.len(), 4);
        assert!(matches!(scene[0], VectorCommand::Polyline(_)));
        assert!(matches!(scene[1], VectorCommand::Polyline(_)));
        assert!(matches!(scene[2], VectorCommand::Polyline(_)));
        assert!(matches!(scene[3], VectorCommand::Line(_)));
    }

    #[test]
    fn outlines_are_closed_and_shield_arc_stays_open() {
        let scene = first_replication_vector_scene();

        let VectorCommand::Polyline(parent_probe) = &scene[0] else {
            panic!("parent probe should be a polyline outline");
        };
        assert_eq!(parent_probe.points.first(), parent_probe.points.last());

        let VectorCommand::Polyline(asteroid) = &scene[1] else {
            panic!("asteroid should be a polyline outline");
        };
        assert_eq!(asteroid.points.first(), asteroid.points.last());

        let VectorCommand::Polyline(shield_arc) = &scene[2] else {
            panic!("shield should be an arc polyline");
        };
        assert_ne!(shield_arc.points.first(), shield_arc.points.last());
        assert!(shield_arc.points.len() >= 4);
    }

    #[test]
    fn replay_scene_uses_replayed_parent_pose_and_preserves_primitives() {
        let first_scene = first_replication_vector_scene();
        let replay_scene = deterministic_parent_probe_replay_scene();

        assert_eq!(replay_scene.len(), first_scene.len());
        assert!(matches!(replay_scene[0], VectorCommand::Polyline(_)));
        assert!(matches!(replay_scene[1], VectorCommand::Polyline(_)));
        assert!(matches!(replay_scene[2], VectorCommand::Polyline(_)));
        assert!(matches!(replay_scene[3], VectorCommand::Line(_)));

        let VectorCommand::Polyline(first_parent) = &first_scene[0] else {
            panic!("first scene parent probe should be a polyline outline");
        };
        let VectorCommand::Polyline(replay_parent) = &replay_scene[0] else {
            panic!("replay parent probe should be a polyline outline");
        };
        assert_eq!(replay_parent.points.first(), replay_parent.points.last());
        assert_ne!(replay_parent.points, first_parent.points);
    }

    #[test]
    fn replay_metadata_reports_fixed_sequence_context() {
        let metadata = deterministic_parent_probe_replay_metadata_json();

        assert!(metadata.contains("\"kind\":\"deterministic-parent-probe-replay\""));
        assert!(metadata.contains("\"totalSteps\":48"));
        assert!(metadata.contains("\"capturedStep\":48"));
        assert!(metadata.contains("\"finalState\""));
    }
}
