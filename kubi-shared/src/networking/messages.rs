use rkyv::{Archive, Deserialize, Serialize};
use bytecheck::CheckBytes;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive_attr(derive(CheckBytes, Debug))]
pub enum ClientToServerMessage {
  Placeholder
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive_attr(derive(CheckBytes, Debug))]
pub enum ServerToClientMessage {
  Placeholder
}
