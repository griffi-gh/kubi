use shipyard::Component;
use glam::{Mat4, Vec2};

#[derive(Component, Clone, Copy, Debug, Default)]
#[track(All)]
pub struct Transform(pub Mat4);
