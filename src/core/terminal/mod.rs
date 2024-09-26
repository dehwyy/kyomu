use tokio::sync::broadcast;

use crate::core::ui::Ui;

use super::event::EventReceiver;

pub type TerminalSize = (u16, u16);
pub type TerminalPosition = (u16, u16);

pub struct Terminal {
  rx: EventReceiver,
  ui: Ui,
}

impl Terminal {
  pub fn new(rx: EventReceiver, ui: Ui) -> Self {
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
