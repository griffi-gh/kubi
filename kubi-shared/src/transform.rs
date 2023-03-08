use shipyard::Component;
use glam::{Mat4, Mat3};

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Transform(pub Mat4);

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Transform2d(pub Mat3);
