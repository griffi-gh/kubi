use hui::element::text::Text;
use kubi_shared::networking::state::ClientJoinState;
use shipyard::{IntoWorkload, NonSendSync, UniqueView, UniqueViewMut, Workload};
use crate::{
  hui_integration::UiState,
  loading_screen::loading_screen_base,
  networking::ServerAddress,
  rendering::WindowSize,
  state::{GameState, NextState}
};

fn render_connecting_ui(
  addr: UniqueView<ServerAddress>,
  mut ui: NonSendSync<UniqueViewMut<UiState>>,
  size: UniqueView<WindowSize>,
) {
  let font_handle = ui.fonts[0];
  ui.hui.add(
    loading_screen_base(vec![
      Box::new(Text {
        text: format!(
          "Connecting to {}...",
          addr.0,
        ).into(),
        font: font_handle,
        text_size: 16,
        ..Default::default()
      }),
    ], 1.),
    size.0.as_vec2(),
  );
}

fn switch_to_loading_if_connected(
  mut next_state: UniqueViewMut<NextState>,
  client_state: UniqueView<ClientJoinState>,
) {
  if *client_state == ClientJoinState::Joined {
    next_state.0 = Some(GameState::LoadingWorld);
  }
}

pub fn update_connecting_screen() -> Workload {
  (
    render_connecting_ui,
    switch_to_loading_if_connected,
  ).into_workload()
}
