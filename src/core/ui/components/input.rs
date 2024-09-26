use tokio::io::Stdout;

use crate::core::{cell::color::{Color, Rgb}, event::key::Key, geom::align::Align, io::out::{flags::{self, OutputFlags, OutputGroupFlags}, group::OutputGroup, Output}};

use crate::core::terminal::{Terminal, TerminalSize};
use crate::core::event::{Event, EventReceiver};

use super::{Component, ComponentInner};

pub struct Input {
  rx: EventReceiver,
  value: String,
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
      value: String::new(),
      placeholder: None
    }
  }

  fn get_placeholder(&self) -> String {
    self.placeholder.clone().unwrap_or("Input".to_string())
  }

  fn get_value(&self) -> String {
    self.value.clone()
  }

  pub fn set_placeholder(&mut self, placeholder: String) -> &mut Self {
    self.placeholder = Some(placeholder);
    self
  }

  pub fn set_value<F: Fn(String) -> String>(&mut self, callback: F) -> &mut Self {
    self.value = callback(self.value.clone());
    self
  }
}

#[async_trait::async_trait]
impl Component for Input {
  async fn render(&mut self, stdout: &mut Stdout) {
    while let Ok(new_event) = self.rx.try_recv() {
      if let Event::Key(key) = new_event {
        match key {
          Key::Backspace => {
            let val_len = self.get_value().len();
            if val_len > 0 {
              self.value.remove(self.get_value().len() - 1);
            }
          },
          Key::Char(c) => {
            self.value.push(c.ch);
          },
          _ev => {}
        };
      }
    }

    print!("\r\x1b[2K");

    OutputGroup::new(
      *OutputGroupFlags::empty().new_lined().clear_line(),
      vec!(
        Output::new(self.get_placeholder())
          .flags(OutputFlags::UNDERLINE | OutputFlags::BOLD | OutputFlags::STRIKETHROUGH)
          .fg_color(Color::Rgb(Rgb::new(200, 40, 120))),

        Output::new(": "),

        Output::new(self.get_value())
          .fg_color(Color::Red)
      ),
    ).write(stdout).await;
  }

  // TODO
  /// Ends componenet lifecycle and returns input value.
  fn destroy(self) {
    self.value;
  }

  fn align(&mut self, alignment: Align) {
    self.inner.pos = alignment.get_offset(Terminal::get_size(), self.inner.size);
  }
}
