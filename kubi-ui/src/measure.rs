use glam::Vec2;

#[derive(Default)]
#[non_exhaustive]
pub struct Hints {
  pub inner_content_size: Option<Vec2>,
  pub inner_content_size_cache: Option<Vec<Vec2>>,
}

#[derive(Default)]
pub struct Response {
  pub size: Vec2,
  pub hints: Hints,
  pub user_data: Option<Box<dyn std::any::Any>>,
}
