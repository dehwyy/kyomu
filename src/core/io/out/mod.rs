pub mod flags;
pub mod group;

use std::fmt::Display;

use tokio::io::{Stdout, AsyncWriteExt, Error as WriteError};

use super::ansi::sequence::{AnsiSequence, AnsiSequenceType};
use flags::OutputFlags;


use crate::core::cell::color::{BgColor, Color, FgColor};


pub struct Output {
  s: String,
  flags: OutputFlags,
  fg_color: FgColor,
  bg_color: Option<BgColor>,
}


impl Output {
  pub fn new(s: impl Display) -> Self {
    Self {
      s: s.to_string(),
      flags: OutputFlags::empty(),
      fg_color: FgColor::default(),
      bg_color: None,
    }
  }

  pub fn fg_color(mut self, color: Color) -> Self {
    self.fg_color = FgColor(color);
    self
  }

  pub fn bg_color(mut self, color: Color) -> Self {
    self.bg_color = Some(BgColor(color));
    self
  }

  pub fn flags(mut self,  flags: OutputFlags) -> Self {
    self.flags = flags;
    self
  }

  pub async fn write(&mut self, stdout: &mut Stdout) -> Result<(), WriteError> {
    let ansi_sequence = AnsiSequence::new(AnsiSequenceType::Graphic)
      .inject(self.flags)
      .inject(self.fg_color)
      .inject_maybe(self.bg_color);

    let s = {
      let (begin, end) = ansi_sequence.compile();

      format!("{begin}{}{end}", self.s)
    };

    stdout.write_all(s.as_bytes()).await?;

    Ok(())
  }
}
