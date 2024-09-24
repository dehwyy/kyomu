use std::fmt::Display;

use tokio::io::{Stdout, AsyncWriteExt, Error as WriteError};

use super::ansi::AnsiSequence;
use super::out_flags::OutputFlags;


use crate::core::cell::color::Color;


#[derive(Default)]
pub struct Output {
  flags: OutputFlags,
  fg_color: Color,
  bg_color: Option<Color>,
  new_line: bool
}


impl Output {
  pub fn new() -> Self {
    Self {
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

  // `false` by default
  pub fn new_line(mut self, new_line: bool) -> Self {
    self.new_line = new_line;
    self
  }


  pub async fn write<S>(&mut self, stdout: &mut Stdout, s: S) -> Result<(), WriteError>
  where S: Display
  {
    let mut ansi_sequence = AnsiSequence::new()
      .inject_flags(self.flags)
      .inject_fg_color(self.fg_color);

    if let Some(bg_color) = self.bg_color {
      ansi_sequence = ansi_sequence.inject_bg_color(bg_color);
    }

    let (ansi_start, ansi_end) = ansi_sequence.compile();


    let mut s = format!("{ansi_start}{s}{ansi_end}");

    if self.new_line {
      s += "\n";
    }

    stdout.write_all(s.as_bytes()).await?;

    stdout.flush().await?;

    Ok(())
  }
}
