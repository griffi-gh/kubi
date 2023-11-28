use crate::element::UiElement;

pub struct Interactable<T: UiElement> {
  pub element: T,
  pub hovered: Option<Box<dyn FnOnce()>>,
  pub clicked: Option<Box<dyn FnOnce()>>,
}

impl<T: UiElement> Interactable<T> {
  pub fn new(element: T) -> Self {
    Self {
      element,
      hovered: None,
      clicked: None,
    }
  }

  pub fn on_click(self, clicked: impl FnOnce() + 'static) -> Self {
    Self {
      clicked: Some(Box::new(clicked)),
      ..self
    }
  }

  pub fn on_hover(self, clicked: impl FnOnce() + 'static) -> Self {
    Self {
      clicked: Some(Box::new(clicked)),
      ..self
    }
  }
}

impl<T: UiElement> UiElement for Interactable<T> {
  fn measure(&self, state: &crate::state::StateRepo, layout: &crate::LayoutInfo) -> crate::measure::Response {
    self.element.measure(state, layout)
  }

  fn process(&self, measure: &crate::measure::Response, state: &mut crate::state::StateRepo, layout: &crate::LayoutInfo, draw: &mut Vec<crate::draw::UiDrawCommand>) {
    self.element.process(measure, state, layout, draw)
  }
}

pub trait IntoInteractable<T: UiElement>: UiElement {
  fn into_interactable(self) -> Interactable<T>;
}

impl<T: UiElement> IntoInteractable<T> for T {
  fn into_interactable(self) -> Interactable<Self> {
    Interactable::new(self)
  }
}
