use std::any::Any;
use crate::{UiDirection, LayoutInfo, draw::UiDrawCall, measure::{IsMeasurable, Response}, state::StateRepo};
use super::UiElement;

pub struct LayoutBox {
  pub direction: UiDirection,
  pub gap: f32,
  pub elements: Vec<Box<dyn UiElement>>,
}

impl UiElement for LayoutBox {
  fn is_measurable(&self) -> IsMeasurable {
    IsMeasurable::Maybe
  }

  fn measure(&self, state: StateRepo, layout: &LayoutInfo) -> Option<Response> {
    for element in &self.elements {
      if element.is_measurable() == IsMeasurable::No {
        return None
      }
      element.measure(None, layout);
    }
    todo!()
  }

  fn process(&self, _state: Option<&mut Box<dyn Any>>, layout: &LayoutInfo, draw: &mut Vec<UiDrawCall>) -> Response {
    todo!()
  }
}
