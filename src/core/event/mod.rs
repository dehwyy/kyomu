pub mod key;

use crossterm::event::{poll as event_poll, Event as CrosstermEvent};
use key::{Key, KeyChar};
use tokio::time::Duration;

pub type EventReceiver = tokio::sync::broadcast::Receiver<Event>;

#[derive(Debug, Clone)]
pub enum Event {
    Key(Key),
    Quit,
}

const ZERO_TIMEOUT: Duration = Duration::from_secs(0);

impl Event {
    /// Returns `Some(event)` if any event is available.
    ///
    /// `Some(EventQuit)`if error occuried;
    ///
    /// `None` if no event is available.
    pub async fn read() -> Option<Self> {
        match event_poll(ZERO_TIMEOUT) {
            Ok(true) => {
                let Ok(ev) = crossterm::event::read() else {
                    return Some(Self::Quit);
                };

                Some(ev.into())
            }
            Ok(_) => None,
            Err(_) => Some(Self::Quit),
        }
    }
}

impl From<CrosstermEvent> for Event {
    fn from(event: CrosstermEvent) -> Self {
        match event {
            CrosstermEvent::Key(k) => {
                let key_event = Key::from(k);
                match &key_event {
                    Key::Char(key_char) => {
                        if KeyChar::build('c').ctrl() == *key_char {
                            return Self::Quit;
                        }

                        Self::Key(key_event)
                    }
                    _key_ev => Self::Key(key_event),
                }
            }
            _ => Self::Quit,
        }
    }
}
