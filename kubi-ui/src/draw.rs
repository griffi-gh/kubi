use glam::{Vec2, vec2, Vec4};

#[derive(Clone, Copy, Debug)]
pub enum UiDrawCall {
  ///Filled, colored rectangle
  Rectangle {
    ///Position in pixels
    position: Vec2,
    ///Size in pixels
    size: Vec2,
    ///Color (RGBA)
    color: Vec4,
  }
}

pub struct UiDrawCalls {
  pub calls: Vec<UiDrawCall>,
}

pub struct UiDrawPlan {

}
