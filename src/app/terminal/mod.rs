pub mod event;
pub mod key;

use std::io::{stdout, Stdout};
use tokio::sync::broadcast;

use event::Event;

use crate::core::ui::Ui;

pub type TerminalSize = (u16, u16);

pub struct Terminal {
  rx: broadcast::Receiver<Event>,
  ui: Ui,
}

impl Terminal {
  pub fn new(rx: broadcast::Receiver<Event>, ui: Ui) -> Self {
    Self {
      rx,
      ui,
    }
  }

  pub async fn render(&mut self) {
    self.ui.render().await;
  }

  pub fn get_size() -> TerminalSize {
    crossterm::terminal::size().unwrap_or((1, 1))
  }
}
