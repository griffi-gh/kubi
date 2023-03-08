use shipyard::{UniqueView, NonSendSync};
use uflow::{server::Event as ServerEvent, SendMode};
use kubi_shared::networking::messages::{
  ClientToServerMessage,
  ServerToClientMessage,
  InitData,
  C_CLIENT_HELLO
};
use crate::{
  server::{ServerEvents, UdpServer, IsMessageOfType}, 
  config::ConfigTable
};

pub fn authenticate_players(
  server: NonSendSync<UniqueView<UdpServer>>,
  events: UniqueView<ServerEvents>,
  config: UniqueView<ConfigTable>
) {
  for event in &events.0 {
    // if let ServerEvent::MessageReceived {
    //   from, 
    //   message: ClientToServerMessage::ClientHello {
    //     username,
    //     password
    //   } 
    // } = event {
    
    let ServerEvent::Receive(client_addr, data) = event else{
      continue
    };
    let Some(client) = server.0.client(client_addr) else {
      log::error!("Client doesn't exist");
      continue
    };
    if !event.is_message_of_type::<C_CLIENT_HELLO>() {
      continue
    }
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
          let res = postcard::to_allocvec(&ServerToClientMessage::ServerFuckOff {
            reason: "Passwords don't match".into()
          }).unwrap().into_boxed_slice();
          client.borrow_mut().send(
            res, 0, SendMode::Reliable
          );
          continue
        }
      } else {
        let res = postcard::to_allocvec(&ServerToClientMessage::ServerFuckOff {
          reason: "This server is password protected".into()
        }).unwrap().into_boxed_slice();
        client.borrow_mut().send(
          res, 0, SendMode::Reliable
        );
        continue
      }
    }

    //Spawn the user
    //TODO Spawn the user on server side

    //Approve the user
    let res = postcard::to_allocvec(&ServerToClientMessage::ServerHello {
      init: InitData {
        users: vec![] //TODO create init data
      }
    }).unwrap().into_boxed_slice();
    client.borrow_mut().send(
      res, 0, SendMode::Reliable
    );

    log::info!("{username} joined the game!")
  }
}
