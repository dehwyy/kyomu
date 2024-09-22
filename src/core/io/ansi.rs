use std::collections::VecDeque;
use crate::core::cell::color::Color;

use super::ansidef as ansi;
use super::out_flags::OutputFlags;

macro_rules! escaped {
    ($s:expr) => {{
      format!(
        "\x1b[{}m",
        $s.into_iter()
          .map(|s| format!("{:X}", s)) // to Hex
          .collect::<Vec<_>>()
          .join(";")
      )
    }};
}



pub(super) struct AnsiSequence {
  before: Vec<u8>,
  after: VecDeque<u8>,
}

impl AnsiSequence {
  pub(super) fn new() -> Self {
    Self {
      before: Vec::new(),
      after: VecDeque::new(),
    }
  }

  // Push to `end_sequence`.
  pub fn push(&mut self, ansi_char: u8) {
    const v: u8 =0x0B;
    // v
    self.after.push_back(ansi_char);
  }

  // Add to `start_sequence`.
  pub fn add(&mut self, ansi_char: u8) {
    self.before.push(ansi_char);
  }

  pub fn add_pair(&mut self, start: u8, end: u8) {
    self.before.push(start);
    self.after.push_front(end);
  }

  pub fn inject_flags(mut self, flags: OutputFlags) -> Self {
    if flags.contains(OutputFlags::BOLD) {
      self.add(ansi::BOLD);
    }

    if flags.contains(OutputFlags::UNDERLINE) {
      self.add(ansi::UNDERLINE);
    }

    self
  }

  pub fn inject_fg_color(mut self, color: Color) -> Self {
    self.add(color.into());
    self
  }

  pub fn inject_bg_color(mut self, color: Color) -> Self {
    let color_fg: u8 = color.into();

    // foreground_color + 10 == background_color in ANSI
    self.add(color_fg + 10);
    self
  }

  /// Returns (`before`, `after`), which are Ansi Escape Sequences.
  pub fn compile(mut self) -> (String, String) {
    self.push(ansi::RESET);

    (escaped!(self.before), escaped!(self.after))
  }
}

impl Into<u8> for Color {
  fn into(self) -> u8 {
    match self {
      Self::White => ansi::WHITE,
      Self::Red => ansi::RED,
      Self::Green => ansi::GREEN,
      Self::Blue => ansi::BLUE,
      // TODO
      Self::Rgb(_) => 39,
    }
  }
}
