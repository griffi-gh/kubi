use shipyard::{AllStoragesView, UniqueViewMut};
use std::{env, net::SocketAddr};
use crate::{
  networking::{GameType, ServerAddress},
  state::{GameState, NextState}
};

pub fn initialize_from_args(
  all_storages: AllStoragesView,
) {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    let address = args[1].parse::<SocketAddr>().expect("invalid address");
    all_storages.add_unique(GameType::Muliplayer);
    all_storages.add_unique(ServerAddress(address));
    all_storages.borrow::<UniqueViewMut<NextState>>().unwrap().0 = Some(GameState::Connecting);
  } else {
    all_storages.add_unique(GameType::Singleplayer);
    all_storages.borrow::<UniqueViewMut<NextState>>().unwrap().0 = Some(GameState::LoadingWorld);
  }
}
