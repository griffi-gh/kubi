use std::collections::VecDeque;
use glam::Vec2;

pub mod element;
pub mod event;
pub mod draw;
pub mod backend;
pub mod measure;
pub mod state;

use state::StateRepo;

pub struct KubiUi {
  mouse_position: Vec2,
  stateful_state: StateRepo,
  event_queue: VecDeque<event::UiEvent>,
}

impl KubiUi {
  pub fn new() -> Self {
    KubiUi {
      mouse_position: Vec2::ZERO,
      stateful_state: StateRepo::default(),
      event_queue: VecDeque::new(),
    }
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
