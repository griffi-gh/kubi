use shipyard::{AllStoragesView, Unique, UniqueViewMut};

pub enum ChatMessage {
  PlayerMessage {
    username: String,
    message: String,
  },
  PlayerJoin {
    username: String,
  },
  PlayerLeave {
    username: String,
  },
  System(String),
}

#[derive(Unique, Default)]
pub struct ChatManager {
  pub messages: Vec<ChatMessage>,
}

impl ChatManager {
  pub fn add_message(&mut self, message: ChatMessage) {
    self.messages.push(message);
  }

  pub fn add_chat_message(&mut self, username: String, message: String,) {
    self.messages.push(ChatMessage::PlayerMessage { username, message });
  }

  pub fn add_player_join(&mut self, username: String) {
    self.messages.push(ChatMessage::PlayerJoin { username });
  }

  pub fn add_player_leave(&mut self, username: String) {
    self.messages.push(ChatMessage::PlayerLeave { username });
  }

  pub fn add_system_message(&mut self, message: String) {
    self.messages.push(ChatMessage::System(message));
  }

  pub fn get_messages(&self) -> &[ChatMessage] {
    &self.messages[..]
  }
}

pub fn init_chat_manager(
  storages: AllStoragesView,
) {
  let mut chat_manager = ChatManager::default();
  chat_manager.add_system_message("Welcome to Kubi!".to_string());
  storages.add_unique(chat_manager);
}
