use shipyard::{UniqueView, UniqueViewMut, Workload, IntoWorkload};
use crate::{
  world::ChunkStorage, 
  state::GameState
};

pub fn insert_progressbar() {

}


pub fn switch_to_ingame_if_loaded(
  world: UniqueView<ChunkStorage>,
  mut state: UniqueViewMut<GameState>
) {
  if world.chunks.is_empty() {
    return
  }
  if world.chunks.iter().all(|(_, chunk)| {
    chunk.desired_state.matches(chunk.current_state)
  }) {
    *state = GameState::InGame;
  }
}

pub fn update_loading_screen() -> Workload {
  (
    insert_progressbar,
    switch_to_ingame_if_loaded
  ).into_workload()
}
