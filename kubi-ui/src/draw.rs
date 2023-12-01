use crate::{IfModified, text::TextRenderer};

use std::borrow::Cow;
use glam::{Vec2, Vec4, vec2};

#[derive(Clone, Debug, PartialEq)]
pub enum UiDrawCommand {
  ///Filled, colored rectangle
  Rectangle {
    ///Position in pixels
    position: Vec2,
    ///Size in pixels
    size: Vec2,
    ///Color (RGBA)
    color: Vec4,
  },
  Text {
    ///Position in pixels
    position: Vec2,
    ///Font size
    size: u8,
    ///Color (RGBA)
    color: Vec4,
    ///Text to draw
    text: Cow<'static, str>,
  },
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindTexture {
  FontTexture,
  //UserDefined(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiVertex {
  pub position: Vec2,
  pub color: Vec4,
  pub uv: Vec2,
}

#[derive(Default)]
pub struct UiDrawCall {
  pub vertices: Vec<UiVertex>,
  pub indices: Vec<u32>,
  pub bind_texture: Option<BindTexture>,
}

#[derive(Default)]
pub struct UiDrawPlan {
  pub calls: Vec<UiDrawCall>
}

impl UiDrawPlan {
  pub fn build(calls: &UiDrawCommands, tr: &mut TextRenderer) -> Self {
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
              uv: vec2(0.0, 0.0),
            },
            UiVertex {
              position: *position + Vec2::new(size.x, 0.0),
              color: *color,
              uv: vec2(1.0, 0.0),
            },
            UiVertex {
              position: *position + *size,
              color: *color,
              uv: vec2(1.0, 1.0),
            },
            UiVertex {
              position: *position + Vec2::new(0.0, size.y),
              color: *color,
              uv: vec2(0.0, 1.0),
            },
          ]);
        },
        UiDrawCommand::Text { position, size, color, text } => {
          todo!()
        }
      }
    }
    Self {
      calls: vec![call]
    }
  }
}

impl IfModified<UiDrawPlan> for (bool, &UiDrawPlan) {
  fn if_modified(&self) -> Option<&UiDrawPlan> {
    match self.0 {
      true => Some(self.1),
      false => None,
    }
  }
}
