use crossterm::event::KeyModifiers as CrosstermKeyModifiers;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modifier {
  Ctrl,
  Shift,
  Alt,
  Unknown
}

impl From<CrosstermKeyModifiers> for Modifier {
  fn from(crossterm_modifiers: CrosstermKeyModifiers) -> Self {

    match crossterm_modifiers {
      CrosstermKeyModifiers::CONTROL => Self::Ctrl,
      CrosstermKeyModifiers::SHIFT => Self::Shift,
      CrosstermKeyModifiers::ALT => Self::Alt,
      _ => Self::Unknown
    }
  }
}
