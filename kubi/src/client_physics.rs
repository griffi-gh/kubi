//TODO client-side physics
//TODO move this to shared
use glam::{vec3, IVec3, Mat4, Vec3};
use shipyard::{track, AllStoragesView, Component, IntoIter, Unique, UniqueView, View, ViewMut};
use kubi_shared::{block::{Block, CollisionType}, transform::Transform};
use crate::{delta_time::DeltaTime, world::ChunkStorage};

#[derive(Unique)]
pub struct GlobalClPhysicsConfig {
  pub gravity: Vec3,
  ///XXX: currenly unused:
  pub iterations: usize,
}

impl Default for GlobalClPhysicsConfig {
  fn default() -> Self {
    Self {
      gravity: Vec3::new(0., -9.8, 0.),
      iterations: 10,
    }
  }
}

//TODO: actors should be represented by a vertical line, not a point.
//XXX: maybe a capsule? (or configurable hull?)
#[derive(Component)]
pub struct ClPhysicsActor {
  pub offset: Vec3,
  pub forces: Vec3,
  pub velocity: Vec3,
  pub terminal_velocity: f32,
  //TODO: this should be configurable per block
  pub friction_agains_ground: f32,
  flag_ground: bool,
  flag_collision: bool,
}

impl ClPhysicsActor {
  pub fn apply_force(&mut self, force: Vec3) {
    self.forces += force;
  }

  pub fn on_ground(&self) -> bool {
    self.flag_ground
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
      flag_ground: false,
      flag_collision: false,
    }
  }
}

trait BlockCollisionExt {
  fn collision_type(&self) -> CollisionType;
  fn is_solid(&self) -> bool {
    self.collision_type() == CollisionType::Solid
  }
}

impl BlockCollisionExt for Option<Block> {
  fn collision_type(&self) -> CollisionType {
    self.unwrap_or(Block::Air).descriptor().collision
  }
}

impl BlockCollisionExt for Block {
  fn collision_type(&self) -> CollisionType {
    self.descriptor().collision
  }
}

pub fn init_client_physics(
  storages: AllStoragesView,
) {
  storages.add_unique(GlobalClPhysicsConfig::default());
}

pub fn update_client_physics_late(
  mut actors: ViewMut<ClPhysicsActor>,
  mut transforms: ViewMut<Transform, track::All>,
  conf: UniqueView<GlobalClPhysicsConfig>,
  world: UniqueView<ChunkStorage>,
  dt: UniqueView<DeltaTime>,
) {
  for (mut actor, mut transform) in (&mut actors, &mut transforms).iter() {
    //apply forces
    let actor_forces = actor.forces;
    actor.velocity += (actor_forces + conf.gravity) * dt.0.as_secs_f32();
    actor.forces = Vec3::ZERO;

    //get position
    let (scale, rotation, mut actor_position) = transform.0.to_scale_rotation_translation();
    actor_position -= actor.offset;

    //get grid-aligned pos and blocks
    let actor_block_pos = actor_position.floor().as_ivec3();
    let actor_block = world.get_block(actor_block_pos);
    let actor_block_below = world.get_block(actor_block_pos + IVec3::NEG_Y);

    //update flags
    actor.flag_collision = actor_block.is_solid();
    actor.flag_ground = actor.flag_collision || actor_block_below.is_solid();

    //push actor back out of the block
    if actor.flag_collision {
      //first, compute restitution, based on position inside the block
      // let block_center = actor_block_pos.as_f32() + Vec3::ONE * 0.5;
      // let to_block_center = actor_position - block_center;

      //then, based on normal:
      //push the actor back
      //actor_position += normal * 0.5;
      //cancel out velocity in the direction of the normal
      // let dot = actor.velocity.dot(normal);
      // if dot > 0. {
      //   //actor.velocity -= normal * dot;
      //   actor.velocity = Vec3::ZERO;
      // }

      //HACK: for now, just stop the vertical velocity if on ground altogether,
      //as we don't have proper collision velocity resolution yet (we need to compute dot product or sth)
      if actor.flag_ground {
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
