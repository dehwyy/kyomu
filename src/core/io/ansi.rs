use std::collections::VecDeque;
use std::fmt::Display;
use crate::core::cell::color::Color;

use super::ansidef as ansi;
use super::out_flags::OutputFlags;

macro_rules! escaped {
    ($s:expr) => {{
      format!(
        "\x1b[{}m",
        $s.into_iter()
          .collect::<Vec<_>>()
          .join(";")
      )
    }};
}



pub(super) struct AnsiSequence {
  before: Vec<String>,
  after: VecDeque<String>,
}

impl AnsiSequence {
  pub(super) fn new() -> Self {
    Self {
      before: Vec::new(),
      after: VecDeque::new(),
    }
  }

  // Push to `end_sequence`.
  pub fn push(&mut self, ansi_char: impl Display) {
    self.after.push_back(ansi_char.to_string());
  }

  // Add to `start_sequence`.
  pub fn add(&mut self, ansi_char: impl Display) {
    self.before.push(ansi_char.to_string());
  }

  pub fn add_vec<D: Display>(&mut self, ansi_chars: Vec<D>) {
    self.before.extend(
      ansi_chars.into_iter().map(|s| s.to_string())
    );
  }

  pub fn add_pair<D: Display>(&mut self, start: D, end: D) {
    self.before.push(start.to_string());
    self.after.push_front(end.to_string());
  }

  pub fn inject_flags(mut self, flags: OutputFlags) -> Self {
    if flags.contains(OutputFlags::BOLD) {
      self.add(ansi::BOLD);
    }

    if flags.contains(OutputFlags::DIM) {
      self.add(ansi::DIM);
    }

    if flags.contains(OutputFlags::ITALIC) {
      self.add(ansi::ITALIC);
    }

    if flags.contains(OutputFlags::UNDERLINE) {
      self.add(ansi::UNDERLINE);
    }

    if flags.contains(OutputFlags::BLINKING) {
      self.add(ansi::BLINKING);
    }

    if flags.contains(OutputFlags::INVERTED) {
      self.add(ansi::INVERTED);
    }

    if flags.contains(OutputFlags::HIDDEN) {
      self.add(ansi::HIDDEN);
    }

    if flags.contains(OutputFlags::STRIKETHROUGH) {
      self.add(ansi::STRIKETHROUGH);
    }

    self
  }

  pub fn inject_fg_color(mut self, color: Color) -> Self {
    self.add_vec(color.to_ansi(false));
    self
  }

  pub fn inject_bg_color(mut self, color: Color) -> Self {
    self.add_vec(color.to_ansi(true));
    self
  }

  /// Returns (`before`, `after`), which are Ansi Escape Sequences.
  pub fn compile(mut self) -> (String, String) {
    self.push(ansi::RESET);

    (escaped!(self.before), escaped!(self.after))
  }
}

impl Color {
  fn to_ansi(self, for_bg: bool) -> Vec<String> {
    let bg_delta = for_bg.then_some(10u8).unwrap_or(0);

    match self {
      Self::Rgb(rgb) => ansi::rgb(
        (ansi::RGB + bg_delta, rgb.get_r(), rgb.get_g(), rgb.get_b()),
      ),

      common_color => {
        let color = match common_color {
          Self::White => ansi::WHITE,
          Self::Red => ansi::RED,
          Self::Green => ansi::GREEN,
          Self::Blue => ansi::BLUE,
          _ => ansi::WHITE
        };

        vec!((color + bg_delta).to_string())
      }
    }
  }
}
