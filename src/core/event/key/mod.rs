mod modifier;

pub use modifier::Modifier;
use crossterm::event::KeyEvent as CrosstermKeyEvent;

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
    // Unimplemented. keys
    Null,
}

pub struct KeyChar {
  ch: Option<char>,
  modifiers: Vec<Modifier>
}

impl From<CrosstermKeyEvent> for Key {
  fn from(key_ev: CrosstermKeyEvent) -> Self {
    use crossterm::event::KeyCode as KC;
    match key_ev.code {
      KC::Left => Self::Left,
      KC::Right => Self::Right,
      KC::Up => Self::Up,
      KC::Down => Self::Down,

      KC::Backspace => Self::Backspace,
      KC::Enter => Self::Enter,
      KC::Tab => Self::Tab,
      KC::BackTab => Self::ShiftTab,


      _ => Self::Null
    }
  }
}

// pub struct KeyModifiers(Vec<SpecialKey>);

// impl KeyModifiers {
//   fn inner(&self) -> Vec<SpecialKey> {
//     self.0.clone()
//   }

//   pub fn exact_match(&self, key_modifiers: &[SpecialKey]) -> bool {
//     let inner = self.inner();

//     for km in key_modifiers {
//       if !inner.contains(km) {
//         return false
//       }
//     }

//     key_modifiers.len() == inner.len()
//   }
// }

// impl From<CrosstermKeyModifiers> for KeyModifiers {
//   fn from(crossterm_modifiers: CrosstermKeyModifiers) -> Self {
//     let mut key_modifiers = vec!();

//     for crossterm_km in crossterm_modifiers.iter() {
//       key_modifiers.push(match crossterm_km {
//         CrosstermKeyModifiers::CONTROL => SpecialKey::CTRL,
//         CrosstermKeyModifiers::SHIFT => SpecialKey::SHIFT,
//         CrosstermKeyModifiers::ALT => SpecialKey::ALT,
//         _ => SpecialKey::UNKNOWN
//       });
//     }

//     Self(key_modifiers)
//   }
// }
