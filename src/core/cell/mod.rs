pub mod color;

use color::Color;

pub type TerminalPosition = (u16, u16);

#[derive(Default)]
pub struct Cell {
  pub bg: Option<Color>,
  pub fg: Option<Color>,
  pub el: char,
  pub width: u8,
  pub pos: TerminalPosition
}

impl Cell {
  pub fn new_empty(pos: TerminalPosition) -> Self {
    Self {
      pos,
      ..Default::default()
    }
  }

  pub fn new_colored(bg_color: Color, pos: TerminalPosition) -> Self {
    Self {
      pos,
      bg: Some(bg_color),
      ..Default::default()
    }
  }

  pub fn new_content(el: char, fg: Option<Color>, bg: Option<Color>, pos: TerminalPosition) -> Self {
    Self {
      pos,
      el,
      bg,
      fg,
      width: 1
    }
  }
}
