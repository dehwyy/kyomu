mod key_char;
mod modifier;

pub use key_char::KeyChar;

use crossterm::event::{KeyEvent as CrosstermKeyEvent, KeyEventKind as CrosstermKeyEventKind};
use modifier::Modifier;

#[derive(Debug, Clone)]
pub enum Key {
    // Arrows.
    Left,
    Right,
    Up,
    Down,
    // Special keys.
    Backspace(Vec<Modifier>),
    Enter(Vec<Modifier>),
    Tab,
    ShiftTab,
    Delete,
    Esc,
    Space,
    // F key.
    F(u8),
    // Char.
    Char(KeyChar),
    // Unimplemented.
    Null,
}

impl Key {
    pub fn vim(mut self) -> Self {
        match self {
            Self::Char(c) => match c.ch {
                'k' => Self::Up,
                'j' => Self::Down,
                _ => Self::Char(c),
            },
            _ => self,
        }
    }
}

impl From<CrosstermKeyEvent> for Key {
    fn from(key_ev: CrosstermKeyEvent) -> Self {
        use crossterm::event::KeyCode as KC;

        // println!("key_ev: {:?}", key_ev);

        if key_ev.kind == CrosstermKeyEventKind::Release {
            return Self::Null;
        }

        let modifiers = key_ev
            .modifiers
            .iter()
            .map(|m| m.into())
            .collect::<Vec<Modifier>>();

        match key_ev.code {
            KC::Left => Self::Left,
            KC::Right => Self::Right,
            KC::Up => Self::Up,
            KC::Down => Self::Down,

            KC::Backspace => Self::Backspace(modifiers),
            KC::Enter => Self::Enter(modifiers),
            KC::Tab => Self::Tab,
            KC::BackTab => Self::ShiftTab,

            KC::Delete => Self::Delete,
            KC::Esc => Self::Esc,

            KC::F(n) => Self::F(n),
            KC::Char(ch) => {
                if ch == ' ' {
                    return Self::Space;
                }
                Self::Char(KeyChar::new(ch, modifiers))
            }

            _ => Self::Null,
        }
    }
}
