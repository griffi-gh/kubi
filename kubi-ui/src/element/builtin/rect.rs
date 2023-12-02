use glam::{vec2, Vec4};
use crate::{
  LayoutInfo,
  UiSize,
  element::UiElement,
  state::StateRepo,
  measure::Response,
  draw::{UiDrawCommand, UiDrawCommands}
};

pub struct Rect {
  pub size: (UiSize, UiSize),
  pub color: Option<Vec4>,
}

impl Default for Rect {
  fn default() -> Self {
    Self {
      size: (UiSize::Pixels(10.), UiSize::Pixels(10.)),
      color: Some(Vec4::new(0., 0., 0., 0.5)),
    }
  }
}

impl UiElement for Rect {
  fn measure(&self, _state: &StateRepo, layout: &LayoutInfo) -> Response {
    Response {
      size: vec2(
        match self.size.0 {
          UiSize::Auto => layout.max_size.x,
          UiSize::Percentage(percentage) => layout.max_size.x * percentage,
          UiSize::Pixels(pixels) => pixels,
        },
        match self.size.1 {
          UiSize::Auto => layout.max_size.y,
          UiSize::Percentage(percentage) => layout.max_size.y * percentage,
          UiSize::Pixels(pixels) => pixels,
        },
      ),
      hints: Default::default(),
      user_data: None
    }
  }

  fn process(&self, measure: &Response, _state: &mut StateRepo, layout: &LayoutInfo, draw: &mut UiDrawCommands) {
    if let Some(color) = self.color {
      draw.add(UiDrawCommand::Rectangle {
        position: layout.position,
        size: measure.size,
        color,
      });
    }
  }
}
