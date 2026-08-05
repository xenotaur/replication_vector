#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use webgpu_vector_lib::VectorFrame;
use webgpu_vector_lib::{Color, Line, Polyline, StrokeStyle, Vec2, VectorCommand};

pub mod simulation;

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

pub fn first_replication_vector_scene() -> Vec<VectorCommand> {
    vec![
        polyline(
            &[
                [-0.32, 0.0],
                [0.18, 0.24],
                [0.07, 0.06],
                [0.42, 0.0],
                [0.07, -0.06],
                [0.18, -0.24],
                [-0.32, 0.0],
            ],
            stroke(0.018, CYAN, 1.15),
        ),
        polyline(
            &[
                [-0.78, 0.28],
                [-0.68, 0.42],
                [-0.49, 0.46],
                [-0.38, 0.31],
                [-0.43, 0.12],
                [-0.63, 0.05],
                [-0.79, 0.14],
                [-0.78, 0.28],
            ],
            stroke(0.014, AMBER, 0.95),
        ),
        polyline(
            &[
                [-0.5, -0.3],
                [-0.4, -0.5],
                [-0.18, -0.62],
                [0.06, -0.61],
                [0.28, -0.48],
                [0.38, -0.3],
            ],
            stroke(0.012, BLUE, 0.85),
        ),
        line([0.48, 0.1], [0.78, 0.2], stroke(0.01, GREEN, 1.25)),
    ]
}

pub fn first_replication_vector_scene_command_count() -> usize {
    first_replication_vector_scene().len()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn replication_vector_scene_command_count() -> u32 {
    first_replication_vector_scene_command_count() as u32
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn replication_vector_first_scene_frame() -> Result<VectorFrame, JsValue> {
    let mut frame = VectorFrame::new();

    for command in first_replication_vector_scene() {
        match command {
            VectorCommand::Line(line) => push_frame_line(&mut frame, &line)?,
            VectorCommand::Polyline(polyline) => push_frame_polyline(&mut frame, &polyline)?,
        }
    }

    Ok(frame)
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
}
