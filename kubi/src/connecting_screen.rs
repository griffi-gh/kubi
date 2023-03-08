use kubi_shared::networking::state::ClientJoinState;
use shipyard::{UniqueViewMut, UniqueView};
use crate::state::{NextState, GameState};

pub fn switch_to_loading_if_connected(
  mut next_state: UniqueViewMut<NextState>,
  client_state: UniqueView<ClientJoinState>,
) { 
  if *client_state == ClientJoinState::Joined {
    next_state.0 = Some(GameState::LoadingWorld);
  }
}
