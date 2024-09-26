mod modifier;
mod key_char;

pub use key_char::KeyChar;

use modifier::Modifier;
use crossterm::event::KeyEvent as CrosstermKeyEvent;

#[derive(Debug, Clone)]
pub enum Key {
    // Arrows.
    Left,
    Right,
    Up,
    Down,
    // Special keys.
    Backspace,
    Enter,
    Tab,
    ShiftTab,
    Delete,
    Esc,
    // F key.
    F(u8),
    // Char.
    Char(KeyChar),
    // Unimplemented.
    Null,
}



impl From<CrosstermKeyEvent> for Key {
  fn from(key_ev: CrosstermKeyEvent) -> Self {
    use crossterm::event::KeyCode as KC;

    // println!("key_ev: {:?}", key_ev);
    match key_ev.code {
      KC::Left => Self::Left,
      KC::Right => Self::Right,
      KC::Up => Self::Up,
      KC::Down => Self::Down,

      KC::Backspace => Self::Backspace,
      KC::Enter => Self::Enter,
      KC::Tab => Self::Tab,
      KC::BackTab => Self::ShiftTab,

      KC::Delete => Self::Delete,
      KC::Esc => Self::Esc,

      KC::F(n) => Self::F(n),
      KC::Char(ch) => Self::Char(
        KeyChar::new(
          ch,
          key_ev.modifiers.iter().map(|m| m.into()).collect()
        )
      ),

      _ => Self::Null
    }
  }
}
