use kubi_udp::server::{Server, ServerConfig};
use kubi_shared::networking::messages::{ClientToServerMessage, ServerToClientMessage};

fn main() {
  let mut server: Server<ServerToClientMessage, ClientToServerMessage> = Server::bind(
    "0.0.0.0:1234".parse().unwrap(), 
    ServerConfig::default()
  ).unwrap();
  loop {
    if let Err(or) = server.update() {
      println!("Server error: {or:?}")
    }
  }
}
