use glam::Vec2;

pub enum UiEvent {
  MouseMove(Vec2),
  MouseDown(Vec2),
  MouseUp(Vec2),
  KeyDown(u32),
  KeyUp(u32),
  TextInput(char),
}
