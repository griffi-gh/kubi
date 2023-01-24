use glium::glutin::event::VirtualKeyCode;
use shipyard::{AllStoragesView, Unique};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Action {
  Move(f32, f32),
  Look(f32, f32),
}

#[derive(Unique)]
pub struct InputSystem<A: 'static> {
  pub keyboard_keymap: HashMap<VirtualKeyCode, A, BuildNoHashHasher<u32>>,
  mouse_map: Option<fn(f32, f32) -> A>,
  keyboard_state: HashMap<VirtualKeyCode, bool, BuildNoHashHasher<u32>>,
  mouse_delta: (f32, f32),
  mouse_position: (f32, f32),
}
impl<A> InputSystem<A> {
  pub fn new() -> Self { 
    Self {
      keyboard_keymap: HashMap::with_hasher(BuildNoHashHasher::default()),
      mouse_map: None,
      keyboard_state: HashMap::with_hasher(BuildNoHashHasher::default()),
      mouse_delta: (0., 0.),
      mouse_position: (0., 0.),
    }
  }
  pub fn map_to_mouse(&mut self, function: fn(f32, f32) -> A) {
    self.mouse_map = Some(function);
  }
}

pub fn setup_input_system(
  storages: AllStoragesView
) {
  let mut input = InputSystem::new();
  input.keyboard_keymap.extend([
    (VirtualKeyCode::W, Action::Move(0., 1.)),
    (VirtualKeyCode::A, Action::Move(-1., 0.)),
    (VirtualKeyCode::S, Action::Move(0., -1.)),
    (VirtualKeyCode::D, Action::Move(1., 0.))
  ]);
  input.map_to_mouse(Action::Look);
  storages.add_unique(input);
}
