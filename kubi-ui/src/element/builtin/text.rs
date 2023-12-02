use std::borrow::Cow;
use glam::{vec2, Vec4};
use crate::{
  LayoutInfo,
  UiSize,
  element::UiElement,
  state::StateRepo,
  measure::Response,
  draw::{UiDrawCommand, UiDrawCommands}, text::FontHandle
};

pub struct Text {
  pub text: Cow<'static, str>,
  pub size: (UiSize, UiSize),
  pub color: Vec4,
  pub font: FontHandle,
  pub text_size: u8,
}

impl Default for Text {
  fn default() -> Self {
    Self {
      text: "".into(),
      size: (UiSize::Auto, UiSize::Auto),
      color: Vec4::new(1., 1., 1., 1.),
      font: FontHandle(0),
      text_size: 16,
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
          UiSize::Auto => self.text_size as f32,
          UiSize::Percentage(percentage) => layout.max_size.y * percentage,
          UiSize::Pixels(pixels) => pixels,
        },
      ),
      hints: Default::default(),
      user_data: None
    }
  }

  fn process(&self, _measure: &Response, _state: &mut StateRepo, layout: &LayoutInfo, draw: &mut UiDrawCommands) {
    draw.add(UiDrawCommand::Text {
      text: self.text.clone(),
      position: layout.position,
      size: self.text_size,
      color: self.color,
      font: self.font
    });
  }
}
