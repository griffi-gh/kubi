use glam::{Vec2, DVec2};
use glium::glutin::event::{DeviceEvent, VirtualKeyCode, ElementState};
use hashbrown::HashSet;
use nohash_hasher::BuildNoHashHasher;
use shipyard::{AllStoragesView, Unique, View, IntoIter, UniqueViewMut, Workload, IntoWorkload, UniqueView};
use crate::events::InputDeviceEvent;

#[derive(Unique, Clone, Copy, Default, Debug)]
pub struct Inputs {
  pub movement: Vec2,
  pub look: Vec2,
  pub action_a: bool,
  pub action_b: bool,
}

#[derive(Unique, Clone, Default, Debug)]
pub struct RawInputState {
  pub keyboard_state: HashSet<VirtualKeyCode, BuildNoHashHasher<u32>>,
  pub mouse_delta: DVec2
}

pub fn process_events(
  device_events: View<InputDeviceEvent>,
  mut input_state: UniqueViewMut<RawInputState>,
) {
  input_state.mouse_delta = DVec2::ZERO;
  for event in device_events.iter() {
    match event.event {
      DeviceEvent::MouseMotion { delta } => {
        input_state.mouse_delta = DVec2::from(delta);
      },
      DeviceEvent::Key(input) => {
        if let Some(keycode) = input.virtual_keycode {
          match input.state {
            ElementState::Pressed  => input_state.keyboard_state.insert(keycode),
            ElementState::Released => input_state.keyboard_state.remove(&keycode),
          };
        }
      },
      DeviceEvent::Button { button: _, state: _ } => {
        //log::debug!("Button {button} {state:?}");
      },
      _ => ()
    }
  }
}

pub fn update_input_states (
  raw_inputs: UniqueView<RawInputState>,
  mut inputs: UniqueViewMut<Inputs>,
) {
  inputs.movement = Vec2::new(
    raw_inputs.keyboard_state.contains(&VirtualKeyCode::D) as u32 as f32 -
    raw_inputs.keyboard_state.contains(&VirtualKeyCode::A) as u32 as f32,
    raw_inputs.keyboard_state.contains(&VirtualKeyCode::W) as u32 as f32 -
    raw_inputs.keyboard_state.contains(&VirtualKeyCode::S) as u32 as f32
  ).normalize_or_zero();
  inputs.look = raw_inputs.mouse_delta.as_vec2();
}

pub fn init_input (
  storages: AllStoragesView
) {
  storages.add_unique(Inputs::default());
  storages.add_unique(RawInputState::default());
}

pub fn process_inputs() -> Workload {
  (
    process_events, 
    update_input_states
  ).into_workload()
}
