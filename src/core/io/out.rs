use std::fmt::Display;

use tokio::io::{Stdout, AsyncWriteExt, Error as WriteError};

use super::ansi::AnsiSequence;
use super::out_flags::OutputFlags;


use crate::core::cell::color::Color;


pub struct Output {
  flags: OutputFlags,
  color: Color,
}


// static YELLOW: String = colored!(33);

impl Output {
  pub fn new() -> Self {
    Self {
      color: Color::default(),
      flags: OutputFlags::empty(),
    }
  }

  pub fn color(mut self, color: Color) -> Self {
    self.color = color;
    self
  }

  pub fn flags(mut self,  flags: OutputFlags) -> Self {
    self.flags = flags;
    self
  }


  pub async fn write<S>(&mut self, stdout: &mut Stdout, s: S) -> Result<(), WriteError>
  where S: Display
  {
    let (ansi_start, ansi_end)  = AnsiSequence::new()
      .inject_flags(self.flags)
      .inject_fg_color(self.color)
      .compile();

    let s = format!("{ansi_start}{s}{ansi_end}");

    stdout.write_all(s.as_bytes()).await?;

    stdout.flush().await?;

    Ok(())
  }
}
