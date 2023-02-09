use shipyard::AllStoragesView;
use std::{env, net::SocketAddr};
use crate::{
  networking::{GameType, ServerAddress},
  state::GameState
};

pub fn initialize_from_args(
  all_storages: AllStoragesView,
) {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    let address = args[1].parse::<SocketAddr>().expect("invalid address");
    all_storages.add_unique(GameType::Muliplayer);
    all_storages.add_unique(GameState::Connecting);
    all_storages.add_unique(ServerAddress(address));
  } else {
    all_storages.add_unique(GameType::Singleplayer);
    all_storages.add_unique(GameState::LoadingWorld);
  }
}
