use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub enum ClientToServerMessage {
  Placeholder
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub enum ServerToClientMessage {
  Placeholder
}
