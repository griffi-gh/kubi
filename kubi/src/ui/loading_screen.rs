use hui::{
  element::{container::Container, progress_bar::ProgressBar, text::Text, UiElement},
  layout::{Alignment, UiDirection, UiSize},
  rectangle::{Corners, Sides},
};
use shipyard::{UniqueView, UniqueViewMut, Workload, NonSendSync, IntoWorkload};
use winit::keyboard::KeyCode;
use crate::{
  hui_integration::UiState, input::RawKbmInputState, networking::ServerAddress, prefabs::UiFontPrefab, rendering::WindowSize, state::{GameState, NextState}, world::ChunkStorage
};

pub fn loading_screen_base(elements: Vec<Box<dyn UiElement>>, bg_alpha: f32) -> Container {
  Container {
    size: (UiSize::Fraction(1.), UiSize::Fraction(1.)),
    background: (0.1, 0.1, 0.1, bg_alpha).into(),
    align: Alignment::Center.into(),
    elements: vec![
      Box::new(Container {
        padding: Sides::all(10.),
        gap: 5.,
        background: (0.2, 0.2, 0.2).into(),
        corner_radius: Corners::all(8.),
        elements,
        ..Default::default()
      })
    ],
    ..Default::default()
  }
}

fn render_loading_ui(
  addr: Option<UniqueView<ServerAddress>>,
  world: UniqueView<ChunkStorage>,
  mut ui: NonSendSync<UniqueViewMut<UiState>>,
  font: UniqueView<UiFontPrefab>,
  size: UniqueView<WindowSize>
) {
  let loaded = world.chunks.iter().fold(0, |acc, (&_, chunk)| {
    acc + chunk.desired_state.matches_current(chunk.current_state) as usize
  });
  let total = world.chunks.len();
  let value = loaded as f32 / total as f32;
  let percentage = value * 100.;

  ui.hui.add(loading_screen_base(vec![
    Box::new(Text {
      text: match addr {
        Some(addr) => format!("Connected to {}\nDownloading world data...", addr.0).into(),
        _ => "Loading...".into(),
      },
      font: font.0,
      text_size: 16,
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
          font: font.0,
          text_size: 16,
          ..Default::default()
        })
      ],
      ..Default::default()
    }),
    // Box::new(Text {
    //   text: "--------------------------------------------------\nTip: You can press F to skip this loading screen".into(),
    //   font: font_handle,
    //   text_size: 16,
    //   color: (0.5, 0.5, 0.5, 1.).into(),
    //   ..Default::default()
    // })
  ], 1. - (value - 0.75).max(0.)), size.0.as_vec2());
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
