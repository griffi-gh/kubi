use shipyard::{UniqueView, UniqueViewMut, Unique, AllStoragesView};
use glium::glutin::{event::VirtualKeyCode, event_loop::ControlFlow};
use crate::input::RawKbmInputState;

#[derive(Unique)]
pub struct SetControlFlow(pub Option<ControlFlow>);

pub fn exit_on_esc(
  raw_inputs: UniqueView<RawKbmInputState>,
  mut control_flow: UniqueViewMut<SetControlFlow>
) {
  if raw_inputs.keyboard_state.contains(&VirtualKeyCode::Escape) {
    control_flow.0 = Some(ControlFlow::Exit);
  }
}

pub fn insert_control_flow_unique(
  storages: AllStoragesView
) {
  storages.add_unique(SetControlFlow(None))
}
