use crossterm::event::KeyModifiers as CrosstermKeyModifiers;

// TODO: `Eq` and `PartialEq` difference?
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpecialKey {
  CTRL,
  SHIFT,
  ALT,
  UNKNOWN
}

pub struct KeyModifiers(Vec<SpecialKey>);

impl KeyModifiers {
  fn inner(&self) -> Vec<SpecialKey> {
    self.0.clone()
  }

  pub fn exact_match(&self, key_modifiers: &[SpecialKey]) -> bool {
    let inner = self.inner();

    for km in key_modifiers {
      if !inner.contains(km) {
        return false
      }
    }

    key_modifiers.len() == inner.len()
  }
}

impl From<CrosstermKeyModifiers> for KeyModifiers {
  fn from(crossterm_modifiers: CrosstermKeyModifiers) -> Self {
    let mut key_modifiers = vec!();

    for crossterm_km in crossterm_modifiers.iter() {
      key_modifiers.push(match crossterm_km {
        CrosstermKeyModifiers::CONTROL => SpecialKey::CTRL,
        CrosstermKeyModifiers::SHIFT => SpecialKey::SHIFT,
        CrosstermKeyModifiers::ALT => SpecialKey::ALT,
        _ => SpecialKey::UNKNOWN
      });
    }

    Self(key_modifiers)
  }
}
