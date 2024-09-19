pub mod color;

use color::Color;

pub type TerminalPosition = (u16, u16);

#[derive(Default)]
pub struct CellInner {
  pub bg_color: Option<Color>,
  pub el_color: Option<Color>,
  pub el: char,
  pub el_width: u8,
  pub pos: TerminalPosition
}

enum Cell {
  Empty(TerminalPosition),
  Colored(Color, TerminalPosition),
  // char, color, bg_color, position
  Content(char, Option<Color>, Option<Color>, TerminalPosition)
}

impl Into<CellInner> for Cell {
  fn into(self) -> CellInner {
      match self {
        Self::Empty(pos) => CellInner {
          pos,
          el: ' ',
          ..Default::default()
        },
        Self::Colored(color, pos) => CellInner {
          pos,
          bg_color: Some(color),
          el_width: 2,
          el: ' ',
          ..Default::default()
        },
        Self::Content(el, el_color, bg_color, pos) => CellInner {
          pos,
          el,
          el_color,
          bg_color,
          el_width: 1
        }
      }
  }
}
