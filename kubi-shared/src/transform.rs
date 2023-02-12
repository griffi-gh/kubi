use shipyard::Component;
use glam::{Mat4, Mat3};

#[derive(Component, Clone, Copy, Debug, Default)]
#[track(All)]
pub struct Transform(pub Mat4);

#[derive(Component, Clone, Copy, Debug, Default)]
#[track(All)]
pub struct Transform2d(pub Mat3);
