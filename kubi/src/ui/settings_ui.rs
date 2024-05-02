use hui::{
  element::{br::Break, container::Container, slider::Slider, text::Text, UiElementExt},
  layout::{Alignment, Direction},
  signal::Signal,
  frame_rect, size,
};
use shipyard::{NonSendSync, UniqueView, UniqueViewMut};
use winit::keyboard::KeyCode;
use crate::{hui_integration::UiState, input::RawKbmInputState, rendering::WindowSize, settings::GameSettings};

#[derive(Signal)]
enum SettingsSignal {
  SetRenderDistance(u8),
  SetEnableDynamicCrosshair(bool),
  SetEnableDebugChunkBorder(bool),
  SetMouseSensitivity(f32),
}

pub fn render_settings_ui(
  mut ui: NonSendSync<UniqueViewMut<UiState>>,
  size: UniqueView<WindowSize>,
  mut settings: UniqueViewMut<GameSettings>,
  kbd: UniqueView<RawKbmInputState>,
) {
  //f1 must be held down to open settings
  //TODO implement ModalManager instead of this
  if !kbd.keyboard_state.contains(KeyCode::F1 as u32) {
    return
  }

  Container::default()
    .with_size(size!(100%))
    .with_background((0., 0., 0., 0.5))
    .with_align(Alignment::Center)
    .with_children(|ui| {
      Container::default()
        .with_background(frame_rect! {
          color: (0.2, 0.2, 0.2),
          corner_radius: 8.
        })
        .with_size(size!(600, 300))
        .with_direction(Direction::Horizontal)
        .with_gap(10.)
        .with_padding(10.)
        .with_children(|ui| {
          Container::default()
            .with_size(size!(100%, auto))
            .with_align(Alignment::Center)
            .with_children(|ui| {
              Text::new("Settings")
                .with_text_size(32)
                .add_child(ui);
            })
            .add_child(ui);
          Break.add_child(ui);

          Text::new("Render Distance")
            .add_child(ui);
          Slider::new(settings.render_distance as f32 / 16.)
            .with_size(size!(300, auto))
            .on_change(|f| SettingsSignal::SetRenderDistance((f * 16.).round() as u8))
            .add_child(ui);
          Text::new(format!("{} Chunks", settings.render_distance))
            .add_child(ui);
          Break.add_child(ui);

          Text::new("Dynamic Crosshair")
            .add_child(ui);
          Slider::new(settings.dynamic_crosshair as u32 as f32)
            .with_size(size!(50, auto))
            .with_track_height(1.)
            .with_handle_size((25., 1.))
            .on_change(|f| SettingsSignal::SetEnableDynamicCrosshair(f >= 0.5))
            .add_child(ui);
          Text::new(if settings.dynamic_crosshair { "On" } else { "Off" })
            .add_child(ui);
          Break.add_child(ui);

          Text::new("Enable debug chunk border")
            .add_child(ui);
          Slider::new(settings.debug_draw_current_chunk_border as u32 as f32)
            .with_size(size!(50, (Slider::DEFAULT_HEIGHT)))
            .with_track_height(1.)
            .with_handle_size((25., 1.))
            .on_change(|f| SettingsSignal::SetEnableDebugChunkBorder(f >= 0.5))
            .add_child(ui);
          Text::new(if settings.debug_draw_current_chunk_border { "On" } else { "Off" })
            .add_child(ui);
          Break.add_child(ui);

          Text::new("Mouse Sensitivity")
            .add_child(ui);
          Slider::new(settings.mouse_sensitivity / 5.)
            .with_size(size!(300, (Slider::DEFAULT_HEIGHT)))
            .on_change(|f| SettingsSignal::SetMouseSensitivity(5. * f))
            .add_child(ui);
          Text::new(format!("{:.2}", settings.mouse_sensitivity))
            .add_child(ui);
        })
        .add_child(ui);
    })
    .add_root(&mut ui.hui, size.0.as_vec2());

  ui.hui.process_signals(|signal: SettingsSignal| match signal {
    SettingsSignal::SetRenderDistance(value) => settings.render_distance = value,
    SettingsSignal::SetEnableDynamicCrosshair(value) => settings.dynamic_crosshair = value,
    SettingsSignal::SetEnableDebugChunkBorder(value) => settings.debug_draw_current_chunk_border = value && cfg!(not(target_os = "android")),
    SettingsSignal::SetMouseSensitivity(value) => settings.mouse_sensitivity = value,
  });
}
