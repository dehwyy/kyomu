use super::key::{SpecialKey, KeyModifiers};

#[derive(Debug)]
pub enum Event {
  Key(String),
  Quit
}

const ZERO_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(0);

impl Event {
  pub fn read() -> Option<Self> {
    match crossterm::event::poll(ZERO_TIMEOUT) {
      Ok(true) => {
        let Ok(ev) = crossterm::event::read() else {
          return Some(Self::Quit);
        };

        Some(ev.into())
      },
      Ok(_) => None,
      Err(_) => Some(Self::Quit)
    }
  }
}

impl From<crossterm::event::Event> for Event {
  // TODO
  fn from(event: crossterm::event::Event) -> Self {
    use crossterm::event::{Event as E, KeyCode};

    println!("{event:?}");

    match event {
      E::Key(k) => {
        let key_modifiers = KeyModifiers::from(k.modifiers);

        match k.code {
          KeyCode::Char(ch) => {
            if key_modifiers.exact_match(&[SpecialKey::CTRL]) && ch == 'c' {
              return Self::Quit
            }
          },

          _ => {}
        };


        Self::Key(format!("{:?}", k.code))
      },

      _ => Self::Quit
    }

  }
}
