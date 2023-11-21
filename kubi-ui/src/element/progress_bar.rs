use glam::{vec2, Vec4};
use crate::{
  UiSize, LayoutInfo,
  draw::UiDrawCall,
  measure::Response,
  state::StateRepo
};
use super::UiElement;

struct ProgressBar {
  size: (UiSize, UiSize),
  value: f32,
  color_foreground: Vec4,
  color_background: Vec4,
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

  fn draw(&self, measure: &Response, state: &mut StateRepo, layout: &LayoutInfo, draw: &mut Vec<UiDrawCall>) {
    draw.push(UiDrawCall::Rectangle {
      position: layout.position,
      size: measure.desired_size,
      color: self.color_background
    });

    draw.push(UiDrawCall::Rectangle {
      position: layout.position,
      size: measure.desired_size * vec2(self.value, 1.0),
      color: self.color_foreground
    });
  }
}
