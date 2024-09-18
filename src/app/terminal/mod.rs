pub mod config;
pub mod event;
pub mod key;

use std::io::{stdout, Stdout};
use tokio::sync::broadcast;

use config::Config;
use event::Event;

use crate::core::ui::Ui;

pub struct Terminal {
  pub config: Config,
  rx: broadcast::Receiver<Event>,
  ui: Ui
}

impl Terminal {
  pub fn new(rx: broadcast::Receiver<Event>, ui: Ui) -> Self {
    Self {
      config: Config::new(),
      rx,
      ui
    }
  }

  pub async fn render(&mut self) {
    self.ui.render().await;
  }
}
