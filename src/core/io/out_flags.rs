use std::collections::VecDeque;

use crate::{ansidef, escaped};


bitflags::bitflags! {
  pub struct OutputFlags: u8 {
    const BOLD = 1;
    const UNDERLINE = 1 << 1;
    const CLEAR_LINE = 1 << 2;
    const CLEAR_ALL = 1 << 3;
  }
}


ansidef!(BOLD_START, BOLD_END, "1", "22");
ansidef!(UNDERLINE_START, UNDERLINE_END, "4", "24");

impl OutputFlags {

  /// Returns (ansi_start, ansi_end)
  pub(super) fn get_ansi_sequence(&self) -> (String, String) {
    let mut ansi_start = Vec::new();
    let mut ansi_end = VecDeque::new();

    if self.contains(OutputFlags::BOLD) {
      ansi_start.push(BOLD_START);
      ansi_end.push_back(BOLD_END);
    }

    if self.contains(OutputFlags::UNDERLINE) {
      ansi_start.push(UNDERLINE_START);
      ansi_end.push_back(UNDERLINE_END);
    }

    // if self.contains(OutputFlags::CLEAR_LINE) {
    //   ansi_end.push_front(CLEAR_LINE.to_string())
    // }

    // if self.contains(OutputFlags::CLEAR_ALL) {
    //   ansi_end.push_front(CLEAR_ALL.to_string());
    // }

    (escaped!(ansi_start), escaped!(ansi_end))
  }

  pub fn bold(&mut self) -> &mut Self {
    *self |= OutputFlags::BOLD;
    self
  }

  pub fn underline(&mut self) -> &mut Self {
    *self |= OutputFlags::UNDERLINE;
    self
  }

  pub fn clear_line(&mut self) -> &mut Self {
    *self |= OutputFlags::CLEAR_LINE;
    self
  }

  pub fn clear_all(&mut self) -> &mut Self {
    *self |= OutputFlags::CLEAR_ALL;
    self
  }
}
