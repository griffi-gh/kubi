use kubi_udp::{
  server::{Server, ServerConfig, ServerEvent}, 
  client::{Client, ClientConfig, ClientEvent},
};
use std::{thread, time::Duration};

const TEST_ADDR: &str = "127.0.0.1:22342";

type CtsMessage = u32;
type StcMessage = u64;

const CTS_MSG: CtsMessage = 0xbeef_face;
const STC_MSG: StcMessage = 0xdead_beef_cafe_face;

#[test]
fn test_connection() {
  //Init logging
  kubi_logging::init();

  //Create server and client
  let mut server: Server<StcMessage, CtsMessage> = Server::bind(
    TEST_ADDR.parse().expect("Invalid TEST_ADDR"), 
    ServerConfig::default()
  ).expect("Failed to create server");
  let mut client: Client<CtsMessage, StcMessage> = Client::new(
    TEST_ADDR.parse().unwrap(), 
    ClientConfig::default()
  ).expect("Failed to create client");

  //Start server update thread
  let server_handle = thread::spawn(move || {
    let mut message_received = false;
    loop {
      server.update().unwrap();
      let events: Vec<_> = server.process_events().collect();
      for event in events {
        match event {
          ServerEvent::Connected(id) => {
            assert_eq!(id.get(), 1, "Unexpected client id");
            server.send_message(id, STC_MSG).unwrap();
          },
          ServerEvent::Disconnected(id) => {
            assert!(message_received, "Client {id} disconnected from the server before sending the message")
          },
          ServerEvent::MessageReceived { from, message } => {
            assert_eq!(message, CTS_MSG, "Received message not equal");
            message_received = true;
            break;
          },
          _ => ()
        }
      }
    }
  });
  
  //Wait a bit
  thread::sleep(Duration::from_secs(1));

  //Connect client
  client.connect().expect("Client connect failed");
  
  //Start updating the client
  let client_handle = thread::spawn(move || {
    let mut message_received = false;
    loop {
      client.update().unwrap();
      let events: Vec<_> = client.process_events().collect();
      for event in events {
        match event {
          ClientEvent::Connected(id) => {
            assert_eq!(id.get(), 1, "Unexpected client id");
            client.send_message(CTS_MSG).unwrap();
          },
          ClientEvent::Disconnected(reason) => {
            assert!(message_received, "Client lost connection to the server before sending the message with reason: {reason:?}")
          },
          ClientEvent::MessageReceived(data) => {
            assert_eq!(data, STC_MSG, "Received message not equal");
            message_received = true;
            break;
          },
          _ => ()
        }
      }
    }
  });

  server_handle.join().unwrap();
  client_handle.join().unwrap();
}
