use shipyard::Unique;

#[derive(Unique)]
pub enum GameState {
  Connecting,
  LoadingWorld,
  InGame,  
}

fn insert_default_state(
  
) {

}
