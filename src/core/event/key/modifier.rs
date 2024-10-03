use crossterm::event::KeyModifiers as CrosstermKeyModifiers;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modifier {
    Ctrl,
    Shift,
    Alt,
    Unknown,
}

impl From<CrosstermKeyModifiers> for Modifier {
    fn from(crossterm_modifiers: CrosstermKeyModifiers) -> Self {
        match crossterm_modifiers {
            CrosstermKeyModifiers::CONTROL => Self::Ctrl,
            CrosstermKeyModifiers::SHIFT => Self::Shift,
            CrosstermKeyModifiers::ALT => Self::Alt,
            _ => Self::Unknown,
        }
    }
}

pub struct Modifiers(Vec<Modifier>);

impl From<CrosstermKeyModifiers> for Modifiers {
    fn from(crossterm_modifiers: CrosstermKeyModifiers) -> Self {
        let mut v = vec![];

        if crossterm_modifiers.contains(CrosstermKeyModifiers::CONTROL) {
            v.push(Modifier::Ctrl)
        }

        if crossterm_modifiers.contains(CrosstermKeyModifiers::SHIFT) {
            v.push(Modifier::Shift)
        }

        if crossterm_modifiers.contains(CrosstermKeyModifiers::ALT) {
            v.push(Modifier::Alt)
        }

        Self(v)
    }
}
