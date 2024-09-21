use std::collections::VecDeque;
use crate::core::cell::color::Color;

use super::out_flags::OutputFlags;

macro_rules! ansi_pair_def {
    ($start_var:ident, $end_var:ident, $start:literal, $end:literal) => {
        const $start_var: u8 = $start;
        const $end_var: u8 = $end;
    };
}
macro_rules! escaped {
    ($s:expr) => {{
      format!("\x1b[{}m", $s.into_iter().collect::<Vec<_>>().join(";"))
    }};
}

// Style
ansi_pair_def!(BOLD_START, BOLD_END, 1, 22);
ansi_pair_def!(UNDERLINE_START, UNDERLINE_END, 4, 24);

// Color
ansi_pair_def!(RED_START, RED_END, 31, 39);
ansi_pair_def!(GREEN_START, GREEN_END, 32, 39);
ansi_pair_def!(BLUE_START, BLUE_END, 34, 39);
ansi_pair_def!(WHITE_START, WHITE_END, 37, 39);

pub(super) struct AnsiSequence {
  before: VecDeque<String>,
  after: VecDeque<String>,
}

impl AnsiSequence {
  pub(super) fn new() -> Self {
    Self {
      before: VecDeque::new(),
      after: VecDeque::new(),
    }
  }

  pub fn add_pair(&mut self, start: u8, end: u8) {
    self.before.push_front(start.to_string());
    self.after.push_back(end.to_string());
  }

  pub fn add(&mut self, text: u8) {
    self.before.push_back(text.to_string());
  }

  pub fn inject_flags(&mut self, flags: OutputFlags) -> &mut Self {
    if flags.contains(OutputFlags::BOLD) {
      self.add_pair(BOLD_START, BOLD_END);
    }

    if flags.contains(OutputFlags::UNDERLINE) {
      self.add_pair(UNDERLINE_START, UNDERLINE_END);
    }

    self
  }

  pub fn inject_fg_color(&mut self, color: Color) -> &mut Self {
    let (color_start, color_end) = Self::get_byte_sequence_by_color(color);
    self.add_pair(color_start, color_end);

    self
  }

  pub fn inject_bg_color(&mut self, color: Color) -> &mut Self {
    let (color_start, color_end) = Self::get_byte_sequence_by_color(color);
    self.add_pair(color_end + 10, color_start + 10);

    self
  }

  /// Returns (`before`, `after`), which are Ansi Escape Sequences.
  pub fn get(self) -> (String, String) {
    (escaped!(self.before), escaped!(self.after))
  }

  // TODO: maybe this function should be in `color.rs`?
  fn get_byte_sequence_by_color(color: Color) -> (u8, u8) {
    match color {
      Color::White => (WHITE_START, WHITE_END),
      Color::Red => (RED_START, RED_END),
      Color::Green => (GREEN_START, GREEN_END),
      Color::Blue => (BLUE_START, BLUE_END),
      Color::Rgb(_) => (39, 39),
    }
  }
}
