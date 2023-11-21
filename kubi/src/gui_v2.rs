use glam::{Vec2, vec2, Vec4};
use shipyard::Unique;

pub enum UiSize {
  Auto,
  Percentage(f32),
  Pixels(f32),
}

struct LayoutInfo {
  position: Vec2,
  max_preferred_size: Vec2,
}

struct Response {
  size: Vec2
}

pub trait UiElement {
  fn process(&self, layout: &LayoutInfo, draw: &mut Vec<UiDrawCall>) -> Response;
  fn measure(&self, layout: &LayoutInfo) -> Option<Response> { None }
}

pub enum LayoutDirection {
  Horizontal,
  Vertical
}

pub struct LayoutBox {
  pub direction: LayoutDirection,
  pub gap: f32,
  pub elements: Vec<Box<dyn UiElement>>,
}

struct ProgressBar {
  size: (UiSize, UiSize),
  value: f32,
  color_foreground: Vec4,
  color_background: Vec4,
}

const BAR_HEIGHT: f32 = 20.0;

impl UiElement for ProgressBar {
  fn measure(&self, layout: &LayoutInfo) -> Option<Response> {
    let width = match self.size.0 {
      UiSize::Auto => layout.max_preferred_size.x,
      UiSize::Percentage(p) => layout.max_preferred_size.x * p,
      UiSize::Pixels(p) => p,
    };
    let height = match self.size.1 {
      UiSize::Auto => BAR_HEIGHT,
      UiSize::Percentage(p) => layout.max_preferred_size.y * p,
      UiSize::Pixels(p) => p,
    };
    let size = Vec2::new(width, height);
    Some(Response { size })
  }

  fn process(&self, layout: &LayoutInfo, draw: &mut Vec<UiDrawCall>) -> Response {
    let measure = self.measure(layout).unwrap();

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

enum UiDrawCall {
  Rectangle {
    ///Position in pixels
    position: Vec2,
    ///Size in pixels
    size: Vec2,
    ///Color (RGBA)
    color: Vec4,
  }
}

#[derive(Unique)]
struct UiDrawCalls {
  pub calls: Vec<UiDrawCall>,
}
