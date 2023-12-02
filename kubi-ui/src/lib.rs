use glam::Vec2;

pub mod element;
pub mod event;
pub mod draw;
pub mod measure;
pub mod state;
pub mod text;
pub mod interaction;

use element::UiElement;
use state::StateRepo;
use draw::{UiDrawCommands, UiDrawPlan};
use text::{TextRenderer, FontTextureInfo, FontHandle};

// pub struct ElementContext<'a> {
//   pub state: &'a mut StateRepo,
//   pub draw: &'a mut UiDrawCommands,
//   pub text: &'a mut TextRenderer,
// }
pub trait IfModified<T> {
  fn if_modified(&self) -> Option<&T>;
}

pub struct KubiUi {
  //mouse_position: Vec2,
  stateful_state: StateRepo,
  //event_queue: VecDeque<UiEvent>,
  prev_draw_commands: UiDrawCommands,
  draw_commands: UiDrawCommands,
  draw_plan: UiDrawPlan,
  draw_plan_modified: bool,
  text_renderer: TextRenderer,
}

impl KubiUi {
  pub fn new() -> Self {
    KubiUi {
      //mouse_position: Vec2::ZERO,
      stateful_state: StateRepo::default(),
      //event_queue: VecDeque::new(),
      // root_elements: Vec::new(),
      prev_draw_commands: UiDrawCommands::default(),
      draw_commands: UiDrawCommands::default(),
      draw_plan: UiDrawPlan::default(),
      draw_plan_modified: false,
      // ftm: FontTextureManager::default(),
      text_renderer: TextRenderer::new(),
    }
  }

  pub fn add_font_from_bytes(&mut self, font: &[u8]) -> FontHandle {
    self.text_renderer.add_font_from_bytes(font)
  }

  pub fn add<T: UiElement>(&mut self, element: T, max_size: Vec2) {
    let layout = LayoutInfo {
      position: Vec2::ZERO,
      max_size,
      direction: UiDirection::Vertical,
    };
    let measure = element.measure(&self.stateful_state, &layout);
    element.process(&measure, &mut self.stateful_state, &layout, &mut self.draw_commands);
  }

  pub fn begin(&mut self) {
    std::mem::swap(&mut self.prev_draw_commands, &mut self.draw_commands);
    self.draw_plan_modified = false;
    self.draw_commands.commands.clear();
    self.text_renderer.reset_frame();
  }

  pub fn end(&mut self) {
    if self.draw_commands.commands == self.prev_draw_commands.commands {
      return
    }
    self.draw_plan = UiDrawPlan::build(&self.draw_commands, &mut self.text_renderer);
    self.draw_plan_modified = true;
  }

  pub fn draw_plan(&self) -> (bool, &UiDrawPlan) {
    (self.draw_plan_modified, &self.draw_plan)
  }

  pub fn font_texture(&self) -> FontTextureInfo {
    self.text_renderer.font_texture()
  }
}

impl Default for KubiUi {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum UiSize {
  #[default]
  Auto,
  Percentage(f32),
  Pixels(f32),
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UiDirection {
  #[default]
  Vertical,
  Horizontal,
}

pub struct LayoutInfo {
  ///Not availabe during measuring step
  pub position: Vec2,
  pub max_size: Vec2,
  pub direction: UiDirection,
}

pub struct ElementList(Vec<Box<dyn UiElement>>);

impl ElementList {
  pub fn add(&mut self, element: impl UiElement + 'static) {
    self.0.push(Box::new(element));
  }
}
pub fn elements(f: impl FnOnce(&mut ElementList)) -> Vec<Box<dyn UiElement>> {
  let mut elements = ElementList(Vec::new());
  f(&mut elements);
  elements.0
}
