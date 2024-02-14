//TODO client-side physics
//TODO move this to shared
use glam::{vec3, IVec3, Mat4, Vec3};
use shipyard::{track, AllStoragesView, Component, IntoIter, Unique, UniqueView, View, ViewMut};
use kubi_shared::{block::{Block, CollisionType}, transform::Transform};
use crate::{delta_time::DeltaTime, world::ChunkStorage};

#[derive(Unique)]
pub struct GlobalClPhysicsConfig {
  pub gravity: Vec3,
}

#[derive(Component)]
pub struct ClPhysicsActor {
  pub offset: Vec3,
  pub forces: Vec3,
  pub velocity: Vec3,
  pub terminal_velocity: f32,
  //TODO: this should be configurable per block
  pub friction_agains_ground: f32,
  on_ground_flag: bool,
}

impl ClPhysicsActor {
  pub fn apply_force(&mut self, force: Vec3) {
    self.forces += force;
  }

  pub fn on_ground(&self) -> bool {
    self.on_ground_flag
  }
}

impl Default for ClPhysicsActor {
  fn default() -> Self {
    Self {
      //HACK: for player
      offset: vec3(0., 1.5, 0.),
      forces: Vec3::ZERO,
      velocity: Vec3::ZERO,
      terminal_velocity: 40.,
      friction_agains_ground: 0.5,
      on_ground_flag: false,
    }
  }
}

pub fn init_client_physics(
  storages: AllStoragesView,
) {
  storages.add_unique(GlobalClPhysicsConfig {
    gravity: Vec3::new(0., -1.0, 0.),
  });
}

pub fn update_client_physics_late(
  mut actors: ViewMut<ClPhysicsActor>,
  mut transforms: ViewMut<Transform, track::All>,
  phy_conf: UniqueView<GlobalClPhysicsConfig>,
  world: UniqueView<ChunkStorage>,
  dt: UniqueView<DeltaTime>,
) {
  for (mut actor, mut transform) in (&mut actors, &mut transforms).iter() {
    //apply forces
    let actor_forces = actor.forces;
    actor.velocity += actor_forces + phy_conf.gravity;
    actor.forces = Vec3::ZERO;

    let (scale, rotation, mut actor_position) = transform.0.to_scale_rotation_translation();
    actor_position -= actor.offset;
    let actor_block_pos = actor_position.floor().as_ivec3();
    let actor_block = world.get_block(actor_block_pos);
    let actor_block_below = world.get_block(actor_block_pos + IVec3::NEG_Y);
    actor.on_ground_flag =
      actor_block_below.map_or_else(|| false, |x| x.descriptor().collision == CollisionType::Solid) ||
      actor_block.map_or_else(|| false, |x| x.descriptor().collision == CollisionType::Solid);
    //push actor back out of the block
    if actor_block.is_some() {
      //first, compute the normal (assuming actor is a point)
      //must be accurate!
      let mut normal = Vec3::ZERO;
      for i in 0..3 {
        let mut offset = Vec3::ZERO;
        offset[i] = 0.5;
        let block_pos = actor_block_pos + offset.as_ivec3();
        let block = world.get_block(block_pos).unwrap_or(Block::Air);
        if block.descriptor().collision == CollisionType::Solid {
          normal[i] = 1.;
        }
      }
      //then, based on normal:
      //push the actor back
      actor_position += normal * 0.5;
      //cancel out velocity in the direction of the normal
      // let dot = actor.velocity.dot(normal);
      // if dot > 0. {
      //   //actor.velocity -= normal * dot;
      //   actor.velocity = Vec3::ZERO;
      // }
      if actor.on_ground_flag {
        actor.velocity.y = 0.;
      }
    }
    //Apply velocity
    actor_position += actor.velocity * dt.0.as_secs_f32();
    actor_position += actor.offset;
    transform.0 = Mat4::from_scale_rotation_translation(scale, rotation, actor_position);
  }

  // for (_, mut transform) in (&controllers, &mut transforms).iter() {
  //   let (scale, rotation, mut translation) = transform.0.to_scale_rotation_translation();
  //   translation.y -= dt.0.as_secs_f32() * 100.;
  //   transform.0 = Mat4::from_scale_rotation_translation(scale, rotation, translation);
  // }
}
