use shipyard::{UniqueView, NonSendSync, EntitiesViewMut, ViewMut, UniqueViewMut, AllStoragesView};
use uflow::{server::Event as ServerEvent, SendMode};
use kubi_shared::{
  networking::{
    messages::{
      ClientToServerMessage,
      ServerToClientMessage,
      InitData,
      C_CLIENT_HELLO
    }, 
    client::{Client, ClientId}, channels::CHANNEL_AUTH
  }, 
  player::{Player, PLAYER_HEALTH}, 
  transform::Transform, entity::{Entity, Health}
};
use crate::{
  config::ConfigTable, 
  server::{ServerEvents, UdpServer, IsMessageOfType}, 
  client::{ClientAddress, ClientAddressMap}
};
pub use kubi_shared::networking::client::ClientIdMap;

pub fn authenticate_players(
  storages: AllStoragesView,
) {
  let mut client_entity_map = storages.borrow::<UniqueViewMut<ClientIdMap>>().unwrap();
  let mut client_addr_map = storages.borrow::<UniqueViewMut<ClientAddressMap>>().unwrap();
  let server = storages.borrow::<NonSendSync<UniqueView<UdpServer>>>().unwrap();
  let events = storages.borrow::<UniqueView<ServerEvents>>().unwrap();
  let config = storages.borrow::<UniqueView<ConfigTable>>().unwrap();
  
  for event in &events.0 {
    let ServerEvent::Receive(client_addr, data) = event else{
      continue
    };
    if !event.is_message_of_type::<C_CLIENT_HELLO>() {
      continue
    }
    let Some(client) = server.0.client(client_addr) else {
      log::error!("Client doesn't exist");
      continue
    };
    let Ok(parsed_message) = postcard::from_bytes(data) else {
      log::error!("Malformed message");
      continue
    };
    let ClientToServerMessage::ClientHello { username, password } = parsed_message else {
      unreachable!()
    };

    log::info!("ClientHello; username={} password={:?}", username, password);

    // Handle password auth
    if let Some(server_password) = &config.server.password {
      if let Some(user_password) = &password {
        if server_password != user_password {
          client.borrow_mut().send(
            postcard::to_allocvec(&ServerToClientMessage::ServerFuckOff {
              reason: "Incorrect password".into()
            }).unwrap().into_boxed_slice(), 
            CHANNEL_AUTH, 
            SendMode::Reliable
          );
          continue
        }
      } else {
        client.borrow_mut().send(
          postcard::to_allocvec(&ServerToClientMessage::ServerFuckOff {
            reason: "This server is password protected".into()
          }).unwrap().into_boxed_slice(), 
          CHANNEL_AUTH, 
          SendMode::Reliable
        );
        continue
      }
    }

    //Find the player ID
    let max_clients = config.server.max_clients as ClientId;
    let Some(client_id) = (0..max_clients).into_iter().find(|id| {
      !client_entity_map.0.contains_key(id) 
    }) else {
      client.borrow_mut().send(
        postcard::to_allocvec(&ServerToClientMessage::ServerFuckOff {
          reason: "Can't find a free spot for you!".into()
        }).unwrap().into_boxed_slice(), 
        CHANNEL_AUTH, 
        SendMode::Reliable
      );
      continue
    };

    //Spawn the user
    let entity_id = {
      storages.borrow::<EntitiesViewMut>().unwrap().add_entity((
        &mut storages.borrow::<ViewMut<Entity>>().unwrap(),
        &mut storages.borrow::<ViewMut<Player>>().unwrap(),
        &mut storages.borrow::<ViewMut<Health>>().unwrap(),
        &mut storages.borrow::<ViewMut<Client>>().unwrap(),
        &mut storages.borrow::<ViewMut<ClientAddress>>().unwrap(),
        &mut storages.borrow::<ViewMut<Transform>>().unwrap(),
      ), (
        Entity,
        Player,
        Health::new(PLAYER_HEALTH),
        Client(client_id),
        ClientAddress(*client_addr),
        Transform::default(),
      ))
    };

    //Add the user to the ClientIdMap and ClientAddressMap
    client_entity_map.0.insert(client_id, entity_id);
    client_addr_map.0.insert(*client_addr, entity_id);

    //Approve the user
    client.borrow_mut().send(
      postcard::to_allocvec(&ServerToClientMessage::ServerHello {
        init: InitData {
          users: vec![] //TODO create init data
        }
      }).unwrap().into_boxed_slice(), 
      CHANNEL_AUTH, 
      SendMode::Reliable
    );

    log::info!("{username}({client_id}) joined the game!")
  }
}
