use strum::EnumIter;
use shipyard::Unique;

#[derive(Unique, EnumIter)]
#[track(All)]
pub enum GameState {
  Connecting,
  LoadingWorld,
  InGame
}
