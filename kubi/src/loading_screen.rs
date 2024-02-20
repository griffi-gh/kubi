use hui::{
  element::{container::Container, progress_bar::ProgressBar, text::Text},
  layout::{Alignment, UiDirection, UiSize},
  rectangle::{Corners, Sides}
};
use shipyard::{UniqueView, UniqueViewMut, Workload, NonSendSync, IntoWorkload};
use winit::keyboard::KeyCode;
use crate::{
  world::ChunkStorage,
  state::{GameState, NextState},
  rendering::WindowSize,
  input::RawKbmInputState,
  hui_integration::UiState,
};

fn render_loading_ui(
  mut ui: NonSendSync<UniqueViewMut<UiState>>,
  world: UniqueView<ChunkStorage>,
  size: UniqueView<WindowSize>
) {
  let loaded = world.chunks.iter().fold(0, |acc, (&_, chunk)| {
    acc + chunk.desired_state.matches_current(chunk.current_state) as usize
  });
  let total = world.chunks.len();
  let value = loaded as f32 / total as f32;
  let percentage = value * 100.;
  ui.hui.add(
    Container {
      size: (UiSize::Fraction(1.), UiSize::Fraction(1.)),
      background: (0.1, 0.1, 0.1, 1. - (value - 0.75).max(0.)).into(),
      align: Alignment::Center.into(),
      elements: vec![
        Box::new(Container {
          padding: Sides::all(10.),
          gap: 10.,
          background: (0.2, 0.2, 0.2).into(),
          corner_radius: Corners::all(8.),
          elements: vec![
            Box::new(Text {
              text: "Loading...".into(),
              ..Default::default()
            }),
            Box::new(ProgressBar {
              value,
              size: (UiSize::Static(400.), UiSize::Auto),
              corner_radius: Corners::all(2.),
              ..Default::default()
            }),
            Box::new(Container {
              size: (UiSize::Static(400.), UiSize::Auto),
              align: (Alignment::End, Alignment::Begin).into(),
              direction: UiDirection::Horizontal,
              elements: vec![
                Box::new(Text {
                  text: format!("{loaded}/{total} ({percentage:.1}%)").into(),
                  ..Default::default()
                })
              ],
              ..Default::default()
            })
          ],
          ..Default::default()
        })
      ],
      ..Default::default()
    },
    size.0.as_vec2()
  );
}

fn switch_to_ingame_if_loaded(
  world: UniqueView<ChunkStorage>,
  mut state: UniqueViewMut<NextState>
) {
  if world.chunks.is_empty() {
    return
  }
  if world.chunks.iter().all(|(_, chunk)| {
    chunk.desired_state.matches_current(chunk.current_state)
  }) {
    log::info!("Finished loading chunks");
    state.0 = Some(GameState::InGame);
  }
}

fn override_loading(
  kbm_state: UniqueView<RawKbmInputState>,
  mut state: UniqueViewMut<NextState>
) {
  if kbm_state.keyboard_state.contains(KeyCode::KeyF as u32) {
    state.0 = Some(GameState::InGame);
  }
}

pub fn update_loading_screen() -> Workload {
  (
    render_loading_ui,
    override_loading,
    switch_to_ingame_if_loaded,
  ).into_sequential_workload()
}
