use std::borrow::Cow;
use glam::{vec2, Vec4};
use crate::{
  LayoutInfo,
  UiSize,
  element::UiElement,
  state::StateRepo,
  measure::Response,
  draw::UiDrawCommand, text::FontHandle
};

pub struct Text {
  pub text: Cow<'static, str>,
  pub size: (UiSize, UiSize),
  pub color: Vec4,
  pub font: FontHandle
}

impl Default for Text {
  fn default() -> Self {
    Self {
      text: "".into(),
      size: (UiSize::Percentage(1.), UiSize::Pixels(32.)),
      color: Vec4::new(1., 1., 1., 1.),
      font: FontHandle(0)
    }
  }
}

impl UiElement for Text {
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

  fn process(&self, _measure: &Response, _state: &mut StateRepo, layout: &LayoutInfo, draw: &mut Vec<UiDrawCommand>) {
    draw.push(UiDrawCommand::Text {
      text: self.text.clone(),
      position: layout.position,
      size: 32,
      color: self.color,
      font: self.font
    });
  }
}
