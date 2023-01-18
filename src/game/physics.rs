use glam::{Vec3A, vec3a};
use crate::game::World;

const GRAVITY: Vec3A = vec3a(0., -1., 0.);

pub struct BasicPhysicsActor {
  pub height: f32,
  pub gravity: Vec3A,
  pub position: Vec3A,
  pub velocity: Vec3A,
}
impl BasicPhysicsActor {
  pub fn new(height: f32) -> Self {
    Self {
      height,
      gravity: GRAVITY,
      position: vec3a(0., 0., 0.),
      velocity: vec3a(0., 0., 0.),
    }
  }
  pub fn update(&mut self, world: &World, dt: f32) {
    self.velocity += GRAVITY;
    self.position += self.velocity;
    loop {
      let block_pos = self.position.floor().as_ivec3();
      if let Some(block) = world.try_get(block_pos) {
        match block.descriptor().collision {
          Some(super::blocks::CollisionType::Solid) => {
            //todo compute restitution here
          }
          _ => break
        }
      } else {
        break
      }
    }
  }
}
