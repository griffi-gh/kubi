use glam::{vec2, Vec4, vec4};
use crate::{
  UiSize, LayoutInfo,
  draw::UiDrawCommand,
  measure::Response,
  state::StateRepo
};
use super::UiElement;

pub struct ProgressBar {
  pub size: (UiSize, UiSize),
  pub value: f32,
  pub color_foreground: Vec4,
  pub color_background: Vec4,
}

impl Default for ProgressBar {
  fn default() -> Self {
    Self {
      size: (UiSize::Auto, UiSize::Auto),
      value: 0.,
      color_foreground: vec4(0.0, 0.0, 1.0, 1.0),
      color_background: vec4(0.0, 0.0, 0.0, 1.0),
    }
  }
}

const BAR_HEIGHT: f32 = 20.0;

impl UiElement for ProgressBar {
  fn name(&self) -> &'static str { "Progress bar" }

  fn measure(&self, _: &StateRepo, layout: &LayoutInfo) -> Response {
    Response {
      desired_size: vec2(
        match self.size.0 {
          UiSize::Auto => layout.max_size.x.max(300.),
          UiSize::Percentage(p) => layout.max_size.x * p,
          UiSize::Pixels(p) => p,
        },
        match self.size.1 {
          UiSize::Auto => BAR_HEIGHT,
          UiSize::Percentage(p) => layout.max_size.y * p,
          UiSize::Pixels(p) => p,
        }
      )
    }
  }

  fn process(&self, measure: &Response, state: &mut StateRepo, layout: &LayoutInfo, draw: &mut Vec<UiDrawCommand>) {
    let value = self.value.clamp(0., 1.);
    if value < 1. {
      draw.push(UiDrawCommand::Rectangle {
        position: layout.position,
        size: measure.desired_size,
        color: self.color_background
      });
    }
    if value > 0. {
      draw.push(UiDrawCommand::Rectangle {
        position: layout.position,
        size: measure.desired_size * vec2(value, 1.0),
        color: self.color_foreground
      });
    }
  }
}
