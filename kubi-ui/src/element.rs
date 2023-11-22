use std::any::Any;
use crate::{
  LayoutInfo,
  draw::UiDrawCommand,
  measure::Response,
  state::StateRepo
};

#[cfg(feature = "builtin_elements")] pub mod container;
#[cfg(feature = "builtin_elements")] pub mod spacer;
#[cfg(feature = "builtin_elements")] pub mod progress_bar;

pub trait UiElement {
  fn name(&self) -> &'static str { "UiElement" }
  fn state_id(&self) -> Option<u64> { None }
  fn is_stateful(&self) -> bool { self.state_id().is_some() }
  fn is_stateless(&self) -> bool { self.state_id().is_none() }
  fn init_state(&self) -> Option<Box<dyn Any>> { None }
  fn measure(&self, state: &StateRepo, layout: &LayoutInfo) -> Response;
  fn process(&self, measure: &Response, state: &mut StateRepo, layout: &LayoutInfo, draw: &mut Vec<UiDrawCommand>);
}
