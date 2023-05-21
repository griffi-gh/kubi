use gilrs::{Gilrs, GamepadId, Button, Event, Axis};
use glam::{Vec2, DVec2, vec2};
use glium::glutin::event::{DeviceEvent, VirtualKeyCode, ElementState};
use hashbrown::HashSet;
use nohash_hasher::BuildNoHashHasher;
use shipyard::{AllStoragesView, Unique, View, IntoIter, UniqueViewMut, Workload, IntoWorkload, UniqueView, NonSendSync};
use crate::events::InputDeviceEvent;

#[derive(Unique, Clone, Copy, Default, Debug)]
pub struct Inputs {
  pub movement: Vec2,
  pub look: Vec2,
  pub action_a: bool,
  pub action_b: bool,
}

#[derive(Unique, Clone, Copy, Default, Debug)]
pub struct PrevInputs(pub Inputs);

#[derive(Unique, Clone, Default, Debug)]
pub struct RawKbmInputState {
  pub keyboard_state: HashSet<VirtualKeyCode, BuildNoHashHasher<u32>>,
  pub button_state: [bool; 32],
  pub mouse_delta: DVec2
}

#[derive(Unique)]
pub struct GilrsWrapper(Option<Gilrs>);

#[derive(Unique, Default, Clone, Copy)]
pub struct ActiveGamepad(Option<GamepadId>);

//maybe we should manage gamepad state ourselves just like keyboard?
//at least for the sake of consitency

fn process_events(
  device_events: View<InputDeviceEvent>,
  mut input_state: UniqueViewMut<RawKbmInputState>,
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
      DeviceEvent::Button { button, state } => {
        if button < 32 {
          input_state.button_state[button as usize] = matches!(state, ElementState::Pressed);
        }
      },
      _ => ()
    }
  }
}

fn process_gilrs_events(
  mut gilrs: NonSendSync<UniqueViewMut<GilrsWrapper>>,
  mut active_gamepad: UniqueViewMut<ActiveGamepad>
) {
  if let Some(gilrs) = &mut gilrs.0 {
    while let Some(Event { id, event: _, time: _ }) = gilrs.next_event() {
      active_gamepad.0 = Some(id);
    }
  }
}

fn input_start(
  mut inputs: UniqueViewMut<Inputs>,
  mut prev_inputs: UniqueViewMut<PrevInputs>,
) {
  prev_inputs.0 = *inputs;
  *inputs = Inputs::default();
}

fn update_input_state (
  raw_inputs: UniqueView<RawKbmInputState>,
  mut inputs: UniqueViewMut<Inputs>,
) {
  inputs.movement += Vec2::new(
    raw_inputs.keyboard_state.contains(&VirtualKeyCode::D) as u32 as f32 -
    raw_inputs.keyboard_state.contains(&VirtualKeyCode::A) as u32 as f32,
    raw_inputs.keyboard_state.contains(&VirtualKeyCode::W) as u32 as f32 -
    raw_inputs.keyboard_state.contains(&VirtualKeyCode::S) as u32 as f32
  );
  inputs.look += raw_inputs.mouse_delta.as_vec2();
  inputs.action_a |= raw_inputs.button_state[1];
  inputs.action_b |= raw_inputs.button_state[3];
}

fn update_input_state_gamepad (
  gilrs: NonSendSync<UniqueView<GilrsWrapper>>,
  active_gamepad: UniqueView<ActiveGamepad>,
  mut inputs: UniqueViewMut<Inputs>,
) {
  if let Some(gilrs) = &gilrs.0 {
    if let Some(gamepad) = active_gamepad.0.map(|id| gilrs.gamepad(id)) {
      let left_stick = vec2(gamepad.value(Axis::LeftStickX), gamepad.value(Axis::LeftStickY));
      let right_stick = vec2(gamepad.value(Axis::RightStickX), -gamepad.value(Axis::RightStickY));
      inputs.movement += left_stick;
      inputs.look += right_stick;
      inputs.action_a |= gamepad.is_pressed(Button::South);
      inputs.action_b |= gamepad.is_pressed(Button::East);
    }
  }
}

fn input_end(
  mut inputs: UniqueViewMut<Inputs>,
) {
  if inputs.movement.length() >= 1. {
    inputs.movement = inputs.movement.normalize();
  }
}

pub fn init_input (
  storages: AllStoragesView
) {
  storages.add_unique_non_send_sync(GilrsWrapper(
    Gilrs::new().map_err(|x| {
      log::error!("Failed to initialize Gilrs");
      x
    }).ok()
  ));
  storages.add_unique(ActiveGamepad::default());
  storages.add_unique(Inputs::default());
  storages.add_unique(PrevInputs::default());
  storages.add_unique(RawKbmInputState::default());
}

pub fn process_inputs() -> Workload {
  (
    process_events,
    process_gilrs_events,
    input_start,
    update_input_state,
    update_input_state_gamepad,
    input_end,
  ).into_sequential_workload()
}
