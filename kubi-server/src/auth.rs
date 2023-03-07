use shipyard::{UniqueView, UniqueViewMut};
use kubi_shared::networking::messages::{ClientToServerMessage, ServerToClientMessage, InitData};
use crate::{server::{ServerEvents, UdpServer}, config::ConfigTable, util::log_error};

pub fn authenticate_players(
  mut server: UniqueViewMut<UdpServer>,
  events: UniqueView<ServerEvents>,
  config: UniqueView<ConfigTable>
) {
  for event in &events.0 {
    if let ServerEvent::MessageReceived {
      from, 
      message: ClientToServerMessage::ClientHello {
        username,
        password
      } 
    } = event {
      log::info!("ClientHello from {} with username {} and password {:?}", from, username, password);

      // Handle password auth
      if let Some(server_password) = &config.server.password {
        if let Some(user_password) = &password {
          if server_password != user_password {
            server.0.send_message(*from, ServerToClientMessage::ServerFuckOff {
              reason: "Passwords don't match".into()
            }).map_err(log_error).ok();
            continue
          }
        } else {
          server.0.send_message(*from, ServerToClientMessage::ServerFuckOff {
            reason: "This server is password-protected".into()
          }).map_err(log_error).ok();
          continue
        }
      }

      //Spawn the user
      //TODO Spawn the user on server side

      //Approve the user
      server.0.send_message(*from, ServerToClientMessage::ServerHello {
        init: InitData {
          users: vec![] //TODO create init data
        }
      }).map_err(log_error).ok();
    }
  }
}
