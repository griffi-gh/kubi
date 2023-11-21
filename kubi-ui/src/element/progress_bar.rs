use glam::{vec2, Vec2, Vec4};
use crate::{
  UiSize, LayoutInfo,
  draw::UiDrawCall,
  measure::{Response, IsMeasurable},
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

  fn is_measurable(&self) -> IsMeasurable { IsMeasurable::Yes }

  fn measure(&self, _: &StateRepo, layout: &LayoutInfo) -> Option<Response> {
    Some(Response {
      size: Vec2::new(
        match self.size.0 {
          UiSize::Auto => layout.max_size.x,
          UiSize::Percentage(p) => layout.max_size.x * p,
          UiSize::Pixels(p) => p,
        },
        match self.size.1 {
          UiSize::Auto => BAR_HEIGHT,
          UiSize::Percentage(p) => layout.max_size.y * p,
          UiSize::Pixels(p) => p,
        }
      )
    })
  }

  fn process(&self, state: &mut StateRepo, layout: &LayoutInfo, draw: &mut Vec<UiDrawCall>) -> Response {
    let measure = self.measure(&state, layout).unwrap();

    draw.push(UiDrawCall::Rectangle {
      position: layout.position,
      size: measure.size,
      color: self.color_background
    });

    draw.push(UiDrawCall::Rectangle {
      position: layout.position,
      size: measure.size * vec2(self.value, 1.0),
      color: self.color_foreground
    });

    measure
  }
}
