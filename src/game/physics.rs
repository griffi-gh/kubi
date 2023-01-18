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
      let block_pos_f = block_pos.as_vec3a();
      if let Some(block) = world.try_get(block_pos) {
        match block.descriptor().collision {
          Some(super::blocks::CollisionType::Solid) => {
            let position_delta = self.position - block_pos_f;
            let distance_to_zero = position_delta.abs();
            let distance_to_one = (vec3a(1., 1., 1.) - position_delta).abs();

            // let mut max_distance = 0;
            // let mut max_distance_normal = 0;
            // distance_to_one.x
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
