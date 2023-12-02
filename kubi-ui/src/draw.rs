use crate::{IfModified, text::{TextRenderer, FontHandle}};

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
    ///Font handle to use
    font: FontHandle,
  },
}

#[derive(Default)]
pub struct UiDrawCommands {
  pub commands: Vec<UiDrawCommand>,
}

impl UiDrawCommands {
  pub fn add(&mut self, command: UiDrawCommand) {
    self.commands.push(command);
  }
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

struct CallSwapper {
  calls: Vec<UiDrawCall>,
  call: UiDrawCall,
}

impl CallSwapper {
  pub fn new() -> Self {
    Self {
      calls: vec![],
      call: UiDrawCall::default(),
    }
  }

  pub fn current(&self) -> &UiDrawCall {
    &self.call
  }

  pub fn current_mut(&mut self) -> &mut UiDrawCall {
    &mut self.call
  }

  pub fn swap(&mut self) {
    self.calls.push(std::mem::replace(&mut self.call, UiDrawCall::default()));
  }

  pub fn finish(mut self) -> Vec<UiDrawCall> {
    self.calls.push(self.call);
    self.calls
  }
}

impl UiDrawPlan {
  pub fn build(draw_commands: &UiDrawCommands, tr: &mut TextRenderer) -> Self {
    let mut swapper = CallSwapper::new();
    let mut prev_command = None;
    for command in &draw_commands.commands {

      let do_swap = if let Some(prev_command) = prev_command {
        std::mem::discriminant(prev_command) != std::mem::discriminant(command)
      } else {
        false
      };

      if do_swap {
        swapper.swap();
      }

      if do_swap || prev_command.is_none() {
        match command {
          UiDrawCommand::Rectangle { .. } => (),
          UiDrawCommand::Text { .. } => {
            swapper.current_mut().bind_texture = Some(BindTexture::FontTexture);
          }
        }
      }

      match command {
        UiDrawCommand::Rectangle { position, size, color } => {
          let vidx = swapper.current().vertices.len() as u32;
          swapper.current_mut().indices.extend([vidx, vidx + 1, vidx + 2, vidx, vidx + 2, vidx + 3]);
          swapper.current_mut().vertices.extend([
            UiVertex {
              position: *position,
              color: *color,
              uv: vec2(0.0, 0.0),
            },
            UiVertex {
              position: *position + vec2(size.x, 0.0),
              color: *color,
              uv: vec2(1.0, 0.0),
            },
            UiVertex {
              position: *position + *size,
              color: *color,
              uv: vec2(1.0, 1.0),
            },
            UiVertex {
              position: *position + vec2(0.0, size.y),
              color: *color,
              uv: vec2(0.0, 1.0),
            },
          ]);
        },
        UiDrawCommand::Text { position, size, color, text, font } => {
          let mut rpos_x = 0.;
          for char in text.chars() {
            let vidx = swapper.current().vertices.len() as u32;
            let glyph = tr.glyph(*font, char, *size);
            rpos_x += glyph.metrics.advance_width;//glyph.metrics.advance_width;
            swapper.current_mut().indices.extend([vidx, vidx + 1, vidx + 2, vidx, vidx + 2, vidx + 3]);
            let p0x = glyph.position.x as f32 / 1024.;
            let p1x = (glyph.position.x + glyph.size.x as i32) as f32 / 1024.;
            let p0y = glyph.position.y as f32 / 1024.;
            let p1y = (glyph.position.y + glyph.size.y as i32) as f32 / 1024.;
            swapper.current_mut().vertices.extend([
              UiVertex {
                position: *position + vec2(rpos_x, 0.0),
                color: *color,
                uv: vec2(p0x, p0y),
              },
              UiVertex {
                position: *position + vec2(rpos_x + glyph.metrics.width as f32, 0.0),
                color: *color,
                uv: vec2(p1x, p0y),
              },
              UiVertex {
                position: *position + vec2(rpos_x + glyph.metrics.width as f32, glyph.metrics.height as f32),
                color: *color,
                uv: vec2(p1x, p1y),
              },
              UiVertex {
                position: *position + vec2(rpos_x, glyph.metrics.height as f32),
                color: *color,
                uv: vec2(p0x, p1y),
              },
            ]);
          }
        }
      }
      prev_command = Some(command);
    }
    Self {
      calls: swapper.finish()
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
