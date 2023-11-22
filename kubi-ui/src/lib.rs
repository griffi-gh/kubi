use std::collections::VecDeque;
use glam::Vec2;

pub mod element;
pub mod event;
pub mod draw;
pub mod backend;
pub mod measure;
pub mod state;

use element::UiElement;
use state::StateRepo;
use event::UiEvent;
use draw::{UiDrawCommands, UiDrawPlan};

pub struct KubiUi {
  mouse_position: Vec2,
  stateful_state: StateRepo,
  event_queue: VecDeque<UiEvent>,
  prev_draw_commands: UiDrawCommands,
  draw_commands: UiDrawCommands,
  draw_plan: UiDrawPlan,
  draw_plan_modified: bool,
}

impl KubiUi {
  pub fn new() -> Self {
    KubiUi {
      mouse_position: Vec2::ZERO,
      stateful_state: StateRepo::default(),
      event_queue: VecDeque::new(),
      // root_elements: Vec::new(),
      prev_draw_commands: UiDrawCommands::default(),
      draw_commands: UiDrawCommands::default(),
      draw_plan: UiDrawPlan::default(),
      draw_plan_modified: false,
    }
  }

  pub fn begin(&mut self) {
    std::mem::swap(&mut self.prev_draw_commands, &mut self.draw_commands);
    self.draw_plan_modified = false;
    self.draw_commands.commands.clear();
  }

  pub fn end(&mut self) {
    if self.draw_commands.commands == self.prev_draw_commands.commands {
      return
    }
    self.draw_plan = UiDrawPlan::build(&self.draw_commands);
    self.draw_plan_modified = true;
  }

  pub fn draw_plan(&self) -> (bool, &UiDrawPlan) {
    (self.draw_plan_modified, &self.draw_plan)
  }
}

impl Default for KubiUi {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Default)]
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

struct LayoutInfo {
  ///Not availabe during measuring step
  position: Vec2,
  max_size: Vec2,
  direction: UiDirection,
}
