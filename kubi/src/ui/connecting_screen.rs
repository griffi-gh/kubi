use hui::element::text::Text;
use kubi_shared::networking::state::ClientJoinState;
use shipyard::{IntoWorkload, NonSendSync, UniqueView, UniqueViewMut, Workload};
use crate::{
  hui_integration::UiState,
  loading_screen::loading_screen_base,
  networking::{ConnectionRejectionReason, ServerAddress},
  prefabs::UiFontPrefab,
  rendering::WindowSize,
  state::{GameState, NextState}
};

fn render_connecting_ui(
  addr: UniqueView<ServerAddress>,
  rejection: Option<UniqueView<ConnectionRejectionReason>>,
  join_state: UniqueView<ClientJoinState>,
  mut ui: NonSendSync<UniqueViewMut<UiState>>,
  font: UniqueView<UiFontPrefab>,
  size: UniqueView<WindowSize>,
) {
  ui.hui.add(
    loading_screen_base(vec![
      Box::new(Text {
        text: match (rejection, *join_state) {
          (Some(err), _) => format!("Connection rejected by {}\n\n{}", addr.0, err.reason).into(),
          (_, ClientJoinState::Disconnected) => format!("Lost connection to {}", addr.0).into(),
          _ => format!("Connecting to {}...", addr.0).into(),
        },
        font: font.0,
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
