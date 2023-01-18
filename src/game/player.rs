use crate::game::camera::Camera;
use crate::game::physics::BasicPhysicsActor;

pub struct MainPlayer {
    pub camera: Camera,
    pub actor: BasicPhysicsActor,
}
