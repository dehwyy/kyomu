use crossterm::event::KeyCode;
use tokio::{io::{AsyncWriteExt, Stdout}, sync::broadcast};

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

  fn get_placeholder(&self) -> String {
    self.placeholder.clone().unwrap_or("Input".to_string())
  }

  fn get_value(&self) -> String {
    self.value.clone().unwrap_or_default()
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
    while let Ok(new_event) = self.rx.try_recv() {
      if let Event::Key(key) = new_event {
        if let KeyCode::Char(c) = key {
          self.set_value(|v| format!("{}{c}", v.clone().unwrap_or_default()));
        }
      }
    }

    Output::new()
      .flags(OutputFlags::UNDERLINE | OutputFlags::BOLD | OutputFlags::STRIKETHROUGH)
      .fg_color(Color::Rgb(Rgb::new(200, 40, 120)))
      // .new_lined()
      .write(stdout, self.get_placeholder()).await.unwrap();

    Output::new()
      .write(stdout, ": ").await.unwrap();

    Output::new()
      .fg_color(Color::Red)
      .new_lined()
      .write(stdout, self.get_value()).await.unwrap();
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
