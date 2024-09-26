pub mod flags;
pub mod group;

use std::fmt::Display;

use tokio::io::{Stdout, AsyncWriteExt, Error as WriteError};

use super::ansi::AnsiSequence;
use flags::OutputFlags;


use crate::{boxed, core::cell::color::Color};


pub struct Output {
  s: String,
  flags: OutputFlags,
  fg_color: Color,
  bg_color: Option<Color>,
  new_line: bool
}

impl Default for Output {
  fn default() -> Self {
      Self {
        s: String::new(),
        flags: OutputFlags::default(),
        fg_color: Color::default(),
        bg_color: None,
        new_line: false
      }
  }
}


impl Output {
  pub fn new(s: impl Display) -> Self {
    Self {
      s: s.to_string(),
      ..Default::default()
    }
  }

  pub fn fg_color(mut self, color: Color) -> Self {
    self.fg_color = color;
    self
  }

  pub fn bg_color(mut self, color: Color) -> Self {
    self.bg_color = Some(color);
    self
  }

  pub fn flags(mut self,  flags: OutputFlags) -> Self {
    self.flags = flags;
    self
  }

  pub fn new_lined(mut self) -> Self {
    self.new_line = true;
    self
  }


  pub async fn write(&mut self, stdout: &mut Stdout) -> Result<(), WriteError> {
    let mut ansi_sequence = AnsiSequence::new()
      .inject_flags(self.flags)
      .inject_fg_color(self.fg_color);

    if let Some(bg_color) = self.bg_color {
      ansi_sequence = ansi_sequence.inject_bg_color(bg_color);
    }

    if self.new_line {
      ansi_sequence = ansi_sequence.new_lined();
    }

    let (ansi_start, ansi_end) = ansi_sequence.compile();


    let s = format!("{ansi_start}{s}{ansi_end}", s=self.s);

    stdout.write_all(s.as_bytes()).await?;

    stdout.flush().await?;

    Ok(())
  }
}
