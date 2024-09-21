use std::fmt::Display;

use tokio::io::{Stdout, AsyncWriteExt, Error as WriteError};


use super::out_flags::OutputFlags;
use crate::{colored, core::cell::color::{self, Color}};



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


  // TODO: flags...
  pub async fn write<S>(&mut self, stdout: &mut Stdout, s: S) -> Result<(), WriteError>
  where S: Display
  {
    let (ansi_start, ansi_end) = self.flags.get_ansi_sequence();

    let s = format!("{ansi_start}{s}{ansi_end}");

    stdout.write_all(s.as_bytes()).await?;

    stdout.flush().await?;

    Ok(())
  }
}
