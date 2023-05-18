use shipyard::Component;

pub type ClientId = u16;

#[derive(Component, Clone, Copy, Debug)]
pub struct Client(pub ClientId);
