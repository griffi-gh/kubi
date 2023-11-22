use std::num::NonZeroU16;
use glam::{Vec2, Vec4};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UiDrawCommand {
  ///Filled, colored rectangle
  Rectangle {
    ///Position in pixels
    position: Vec2,
    ///Size in pixels
    size: Vec2,
    ///Color (RGBA)
    color: Vec4,
  }
}

#[derive(Default)]
pub struct UiDrawCommands {
  pub commands: Vec<UiDrawCommand>,
}

// impl UiDrawCommands {
//   pub fn compare(&self, other: &Self) -> bool {
//     // if self.commands.len() != other.commands.len() { return false }
//     // self.commands.iter().zip(other.commands.iter()).all(|(a, b)| a == b)
//   }
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiVertex {
  pub position: Vec2,
  pub color: Vec4,
  //pub texture: Option<NonZeroU16>,
}

#[derive(Default)]
pub struct UiDrawCall {
  pub vertices: Vec<UiVertex>,
  pub indices: Vec<u32>,
}

#[derive(Default)]
pub struct UiDrawPlan {
  pub calls: Vec<UiDrawCall>
}

impl UiDrawPlan {
  pub fn build(calls: &UiDrawCommands) -> Self {
    let mut call = UiDrawCall::default();
    for command in &calls.commands {
      match command {
        UiDrawCommand::Rectangle { position, size, color } => {
          let idx = call.vertices.len() as u32;
          call.indices.extend([idx, idx + 1, idx + 2, idx, idx + 2, idx + 3]);
          call.vertices.extend([
            UiVertex {
              position: *position,
              color: *color,
            },
            UiVertex {
              position: *position + Vec2::new(size.x, 0.0),
              color: *color,
            },
            UiVertex {
              position: *position + *size,
              color: *color,
            },
            UiVertex {
              position: *position + Vec2::new(0.0, size.y),
              color: *color,
            },
          ]);
        }
      }
    }
    Self {
      calls: vec![call]
    }
  }
}
