use hui::{color, element::{br::Break, container::Container, slider::Slider, text::Text, UiElementExt}, layout::{Alignment, Direction}, signal::Signal, size};
use shipyard::{NonSendSync, UniqueView, UniqueViewMut};
use winit::keyboard::KeyCode;
use crate::{hui_integration::UiState, input::RawKbmInputState, rendering::WindowSize, settings::GameSettings};

#[derive(Signal)]
enum SettingsSignal {
  SetRenderDistance(f32),
}

pub fn render_settings_ui(
  mut ui: NonSendSync<UniqueViewMut<UiState>>,
  size: UniqueView<WindowSize>,
  mut settings: UniqueViewMut<GameSettings>,
  kbd: UniqueView<RawKbmInputState>,
) {
  //f1 must be held down to open settings
  if !kbd.keyboard_state.contains(KeyCode::F1 as u32) {
    return
  }
  Container::default()
    .with_size(size!(100%))
    .with_align(Alignment::Center)
    .with_children(|ui| {
      Container::default()
        .with_background(color::BLACK)
        .with_size(size!(50%, 50%))
        .with_direction(Direction::Horizontal)
        .with_gap(10.)
        .with_padding(10.)
        .with_children(|ui| {
          Text::new("Settings")
            .add_child(ui);
          Break.add_child(ui);
          Text::new("Render Distance")
            .add_child(ui);
          Slider::new(settings.render_distance as f32 / 16.)
            .with_size(size!(300, (Slider::DEFAULT_HEIGHT)))
            .on_change(SettingsSignal::SetRenderDistance)
            .add_child(ui);
          Break.add_child(ui);
        })
        .add_child(ui);
    })
    .add_root(&mut ui.hui, size.0.as_vec2());

  ui.hui.process_signals(|signal: SettingsSignal| match signal {
    SettingsSignal::SetRenderDistance(value) => {
      settings.render_distance = (value * 16.).round() as u8;
    }
  });
}
