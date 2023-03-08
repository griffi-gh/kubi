use shipyard::Component;

#[derive(Component)]
pub struct Entity;

#[derive(Component)]
pub struct Health {
  pub current: u8,
  pub max: u8,
}
impl Health {
  fn new(health: u8) -> Self {
    Self {
      current: health,
      max: health
    }
  }
}
