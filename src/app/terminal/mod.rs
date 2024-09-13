pub mod config;
pub mod event;
pub mod key;

use std::io::{stdout, Stdout};
use config::Config;
use event::Event;

pub struct Terminal {
  pub config: Config,
  stdout: Stdout
}

impl Terminal {
  pub fn new() -> Self {
    Self {
      config: Config::new(),
      stdout: stdout()
    }
  }

  pub fn read_next_event(&self) -> Option<Event> {
    Event::read()
  }
}
