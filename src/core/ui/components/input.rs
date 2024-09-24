use tokio::io::{Stdout, AsyncWriteExt};

use crate::{app::terminal::{event::{Event, EventReceiver}, Terminal}, core::{cell::color::{Color, Rgb}, geom::align::Align, io::{out::Output, out_flags::OutputFlags}, ui::TerminalSize}};

use super::{Component, ComponentInner};

pub struct Input {
  rx: EventReceiver,
  value: Option<String>,
  placeholder: Option<String>,
  inner: ComponentInner
}

impl Input {
  pub fn new(rx: EventReceiver) -> Self {
    Self {
      inner: ComponentInner {
        ..Default::default()
      },
      rx,
      value: None,
      placeholder: None
    }
  }

  pub fn set_placeholder(&mut self, placeholder: String) -> &mut Self {
    self.placeholder = Some(placeholder);
    self
  }

  pub fn set_value<F: Fn(&Option<String>) -> String>(&mut self, callback: F) -> &mut Self {
    self.value = Some(callback(&self.value));
    self
  }
}

#[async_trait::async_trait]
impl Component for Input {
  async fn render(&mut self, stdout: &mut Stdout) {
    let placeholder_formatted = self.placeholder.as_ref()
      .map(|s| format!("{s}: "))
      .unwrap_or(String::from(""));

    let val = self.value.clone().unwrap_or_default();

    Output::new()
      .flags(OutputFlags::UNDERLINE | OutputFlags::BOLD)
      .write(stdout, format!("hello private\n")).await.unwrap();
  }

  // TODO
  /// Ends componenet lifecycle and returns input value.
  fn destroy(self) {
    self.value.unwrap_or_default();
  }

  fn align(&mut self, alignment: Align) {
    self.inner.pos = alignment.get_offset(Terminal::get_size(), self.inner.size);
  }
}
