use hui::{element::{container::Container, text::Text, UiElementExt}, layout::Alignment, size};
use shipyard::{NonSendSync, UniqueView, UniqueViewMut};
use crate::{chat::{ChatManager, ChatMessage}, hui_integration::UiState, rendering::WindowSize};

pub fn render_chat(
  mut hui: NonSendSync<UniqueViewMut<UiState>>,
  size: UniqueView<WindowSize>,
  chat: UniqueView<ChatManager>,
) {
  let messages = chat.get_messages();
  if messages.is_empty() { return }
  Container::default()
    .with_size(size!(100%, 100%))
    .with_align((Alignment::Begin, Alignment::End))
    .with_children(|ui| {
      for message in messages.iter().rev().take(10).rev() {
        let text = match message {
          ChatMessage::PlayerMessage { username, message } => {
            format!("{}: {}", username, message)
          }
          ChatMessage::PlayerJoin { username } => {
            format!("{} joined the game", username)
          }
          ChatMessage::PlayerLeave { username } => {
            format!("{} left the game", username)
          }
          ChatMessage::System(message) => message.clone(),
        };
        Container::default()
          .with_background((0., 0., 0., 0.5))
          .with_children(|ui| {
            Text::new(text)
              .add_child(ui)
          })
          .add_child(ui);
      }
    })
    .add_root(&mut hui.hui, size.0.as_vec2());
}
