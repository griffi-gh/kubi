use shipyard::{Unique, AllStoragesView, UniqueView, UniqueViewMut, Workload, IntoWorkload, EntitiesViewMut, Component, ViewMut, SystemModificator, View, IntoIter, WorkloadModificator};
use glium::glutin::event_loop::ControlFlow;
use std::net::SocketAddr;
use uflow::{client::{Client, Config as ClientConfig, Event as ClientEvent}, EndpointConfig};
use kubi_shared::networking::{
  messages::{ClientToServerMessage, ServerToClientMessage, S_SERVER_HELLO},
  state::ClientJoinState, 
  channels::CHANNEL_AUTH,
};
use crate::{
  events::EventComponent, 
  control_flow::SetControlFlow, 
  world::tasks::ChunkTaskManager, 
  state::is_ingame_or_loading
};

mod world;

use world::{
  inject_network_responses_into_manager_queue,
  send_block_place_events,
  recv_block_place_events,
};

#[derive(Unique, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
  Singleplayer,
  Muliplayer
}

#[derive(Unique, Clone, Copy, PartialEq, Eq)]
pub struct ServerAddress(pub SocketAddr);

#[derive(Unique)]
pub struct UdpClient(pub Client);

#[derive(Component)]
pub struct NetworkEvent(pub ClientEvent);

impl NetworkEvent {
  ///Checks if postcard-encoded message has a type
  pub fn is_message_of_type<const T: u8>(&self) -> bool {
    let ClientEvent::Receive(data) = &self.0 else { return false };
    if data.len() == 0 { return false }
    data[0] == T
  }
}

#[derive(Component)]
pub struct NetworkMessageEvent(pub ServerToClientMessage);

fn connect_client(
  storages: AllStoragesView
) {
  log::info!("Creating client");
  let address = storages.borrow::<UniqueView<ServerAddress>>().unwrap();
  let client = Client::connect(address.0, ClientConfig {
    endpoint_config: EndpointConfig {
      active_timeout_ms: 10000,
      keepalive: true,
      keepalive_interval_ms: 1000,
      ..Default::default()
    },
  }).expect("Client connection failed");
  storages.add_unique(UdpClient(client));
  storages.add_unique(ClientJoinState::Disconnected);
}

fn poll_client(
  mut client: UniqueViewMut<UdpClient>,
  mut entities: EntitiesViewMut,
  mut events: ViewMut<EventComponent>,
  mut network_events: ViewMut<NetworkEvent>,
) {
  entities.bulk_add_entity((
    &mut events,
    &mut network_events,
  ), client.0.step().map(|event| {
    (EventComponent, NetworkEvent(event))
  }));
}

fn flush_client(
  mut client: UniqueViewMut<UdpClient>,
) {
  client.0.flush();
}

fn set_client_join_state_to_connected(
  mut join_state: UniqueViewMut<ClientJoinState>
) {
  log::info!("Setting ClientJoinState");
  *join_state = ClientJoinState::Connected;
}

fn say_hello(
  mut client: UniqueViewMut<UdpClient>,
) {
  log::info!("Authenticating");
  client.0.send(
    postcard::to_allocvec(
      &ClientToServerMessage::ClientHello {
        username: "Sbeve".into(),
        password: None
      }
    ).unwrap().into_boxed_slice(),
    CHANNEL_AUTH,
    uflow::SendMode::Reliable
  );
}

fn check_server_hello_response(
  network_events: View<NetworkEvent>,
  mut join_state: UniqueViewMut<ClientJoinState>
) {
  for event in network_events.iter() {
    let ClientEvent::Receive(data) = &event.0 else {
      continue
    };
    if !event.is_message_of_type::<S_SERVER_HELLO>() {
      continue
    }
    let Ok(parsed_message) = postcard::from_bytes(data) else {
      log::error!("Malformed message");
      continue
    };
    let ServerToClientMessage::ServerHello { init: _ } = parsed_message else {
      unreachable!()
    };
    //TODO handle init data
    *join_state = ClientJoinState::Joined;
    log::info!("Joined the server!");
    return;
  }
}

fn handle_disconnect(
  network_events: View<NetworkEvent>,
  mut join_state: UniqueViewMut<ClientJoinState>
) {
  for event in network_events.iter() {
    if matches!(event.0, ClientEvent::Disconnect) {
      log::warn!("Disconnected from server");
      *join_state = ClientJoinState::Disconnected;
      return;
    }
  }
}

pub fn update_networking() -> Workload {
  (
    connect_client.run_if_missing_unique::<UdpClient>(),
    poll_client,
    (
      set_client_join_state_to_connected,
      say_hello,
    ).into_sequential_workload().run_if(if_just_connected),
    (
      check_server_hello_response,
      handle_disconnect,
    ).into_sequential_workload().run_if(is_join_state::<{ClientJoinState::Connected as u8}>),
    (
      recv_block_place_events
    ).run_if(is_join_state::<{ClientJoinState::Joined as u8}>).run_if(is_ingame_or_loading),
    inject_network_responses_into_manager_queue.run_if(is_ingame_or_loading).skip_if_missing_unique::<ChunkTaskManager>(),
  ).into_sequential_workload()
}

pub fn update_networking_late() -> Workload {
  (
    send_block_place_events.run_if(is_join_state::<{ClientJoinState::Joined as u8}>),
    flush_client,
  ).into_sequential_workload()
}

pub fn disconnect_on_exit(
  control_flow: UniqueView<SetControlFlow>,
  mut client: UniqueViewMut<UdpClient>,
) {
  if let Some(ControlFlow::ExitWithCode(_)) = control_flow.0 {
    if client.0.is_active() {
      client.0.flush();
      client.0.disconnect();
      while client.0.is_active() { client.0.step().for_each(|_|()); }
      log::info!("Client disconnected");
    } else {
      log::info!("Client inactive")
    }
    // if let Err(error) = client.0. {
    //   log::error!("failed to disconnect: {}", error);
    // } else {
    //   log::info!("Client disconnected");
    // }
  }
}

// conditions

fn if_just_connected(
  network_events: View<NetworkEvent>,
) -> bool {
  network_events.iter().any(|event| matches!(&event.0, ClientEvent::Connect))
}

fn is_join_state<const STATE: u8>(
  join_state: UniqueView<ClientJoinState>
) -> bool {
  (*join_state as u8) == STATE
}

pub fn is_multiplayer(
  game_type: UniqueView<GameType>
) -> bool {
  *game_type == GameType::Muliplayer
}

pub fn is_singleplayer(
  game_type: UniqueView<GameType>
) -> bool {
  *game_type == GameType::Singleplayer
}
