use glam::{Vec2, Vec4};
use crate::{UiDirection, LayoutInfo, draw::UiDrawCommand, measure::{IsMeasurable, Response}, state::StateRepo, UiSize};
use super::UiElement;

#[derive(Default, Clone, Copy, Debug)]
pub struct ContainerBorders {
  pub top: Option<(Vec4, f32)>,
  pub bottom: Option<(Vec4, f32)>,
  pub left: Option<(Vec4, f32)>,
  pub right: Option<(Vec4, f32)>,
}

pub enum ContainerAlign {
  Begin,
  Center,
  End,
}

pub struct Container {
  pub min_size: (UiSize, UiSize),
  pub max_size: (UiSize, UiSize),
  pub direction: UiDirection,
  pub gap: f32,
  pub padding: f32,
  pub align: (ContainerAlign, ContainerAlign),
  pub background: Option<Vec4>,
  pub borders: ContainerBorders,
  pub clip: bool,
  pub elements: Vec<Box<dyn UiElement>>,
}

impl Default for Container {
  fn default() -> Self {
    Self {
      min_size: (UiSize::Auto, UiSize::Auto),
      max_size: (UiSize::Auto, UiSize::Auto),
      direction: UiDirection::Vertical,
      gap: 0.,
      padding: 0.,
      align: (ContainerAlign::Center, ContainerAlign::Begin),
      background: Default::default(),
      borders: Default::default(),
      clip: Default::default(),
      elements: Vec::new(),
    }
  }
}

impl UiElement for Container {
  fn measure(&self, state: &StateRepo, layout: &LayoutInfo) -> Response {
    let mut size = Vec2::ZERO;
    let mut leftover_gap = Vec2::ZERO;
    for element in &self.elements {
      let measure = element.measure(state, &LayoutInfo {
        position: layout.position + size,
        max_size: layout.max_size - size,
        direction: self.direction,
      });
      match self.direction {
        UiDirection::Horizontal => {
          size.x += measure.desired_size.x + self.gap;
          size.y = size.y.max(measure.desired_size.y);
          leftover_gap.x = self.gap;
        },
        UiDirection::Vertical => {
          size.x = size.x.max(measure.desired_size.x);
          size.y += measure.desired_size.y + self.gap;
          leftover_gap.y = self.gap;
        }
      }
    }
    size -= leftover_gap;
    Response { desired_size: size }
  }

  fn process(&self, measure: &Response, state: &mut StateRepo, layout: &LayoutInfo, draw: &mut Vec<UiDrawCommand>) {
    if let Some(color) = self.background {
      draw.push(UiDrawCommand::Rectangle {
        position: layout.position,
        size: measure.desired_size,
        color
      });

      //TODO draw borders
    }
  }
}
