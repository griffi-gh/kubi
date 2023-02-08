use shipyard::{Unique, UniqueView};

#[derive(Unique, PartialEq, Eq)]
#[track(All)]
pub enum GameState {
  Connecting,
  LoadingWorld,
  InGame
}

pub fn is_ingame(
  state: UniqueView<GameState>
) -> bool {
  *state == GameState::InGame
}

pub fn is_loading(
  state: UniqueView<GameState>
) -> bool {
  matches!(*state, GameState::LoadingWorld)
}

pub fn is_ingame_or_loading(
  state: UniqueView<GameState>
) -> bool {
  matches!(*state, GameState::InGame | GameState::LoadingWorld)
}
