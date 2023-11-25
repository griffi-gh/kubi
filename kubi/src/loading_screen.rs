use kubi_ui::element::progress_bar::ProgressBar;
use shipyard::{UniqueView, UniqueViewMut, Workload, NonSendSync, IntoWorkload};
use winit::keyboard::KeyCode;
use crate::{
  world::ChunkStorage,
  state::{GameState, NextState},
  rendering::WindowSize,
  input::RawKbmInputState,
  guiv2_integration::UiState,
};

fn render_progressbar(
  mut ui: NonSendSync<UniqueViewMut<UiState>>,
  world: UniqueView<ChunkStorage>,
  size: UniqueView<WindowSize>
) {
  let value = {
    let loaded = world.chunks.iter().fold(0, |acc, (&_, chunk)| {
      acc + chunk.desired_state.matches_current(chunk.current_state) as usize
    });
    let total = world.chunks.len();
    loaded as f32 / total as f32
  };
  ui.kui.add(
    ProgressBar { value, ..Default::default() },
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
    render_progressbar,
    override_loading,
    switch_to_ingame_if_loaded,
  ).into_sequential_workload()
}
