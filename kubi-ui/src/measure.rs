use glam::Vec2;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum IsMeasurable {
  #[default]
  No,
  Maybe,
  Yes
}

pub struct Response {
  pub desired_size: Vec2
}
