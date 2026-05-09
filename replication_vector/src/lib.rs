#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use webgpu_vector_lib::{Color, Line, StrokeStyle, Vec2, VectorCommand};

const CYAN: Color = Color {
    red: 0.2,
    green: 0.95,
    blue: 1.0,
    alpha: 1.0,
};

fn line(start: [f32; 2], end: [f32; 2]) -> VectorCommand {
    VectorCommand::Line(Line {
        start: Vec2 {
            x: start[0],
            y: start[1],
        },
        end: Vec2 {
            x: end[0],
            y: end[1],
        },
        style: StrokeStyle {
            width: 2.0,
            color: CYAN,
            intensity: 1.0,
        },
    })
}

pub fn bootstrap_scene() -> Vec<VectorCommand> {
    vec![
        line([-0.35, -0.2], [0.0, 0.32]),
        line([0.0, 0.32], [0.35, -0.2]),
        line([0.35, -0.2], [-0.35, -0.2]),
    ]
}

pub fn bootstrap_scene_command_count() -> usize {
    bootstrap_scene().len()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn replication_vector_bootstrap_command_count() -> u32 {
    bootstrap_scene_command_count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_scene_uses_velumin_vector_commands() {
        assert_eq!(bootstrap_scene_command_count(), 3);
        assert!(matches!(bootstrap_scene()[0], VectorCommand::Line(_)));
    }
}
