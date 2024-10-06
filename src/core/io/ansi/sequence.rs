use std::fmt::Display;

use crate::esc;

use super::def as ansi;
use super::Ansiable;

pub enum AnsiSequenceType {
    Graphic,
    AbsoluteMove,
    NotChainable,
}

pub struct AnsiSequence {
    t: AnsiSequenceType,
    items: Vec<String>,
}

impl AnsiSequence {
    pub fn new(t: AnsiSequenceType) -> Self {
        Self { t, items: vec![] }
    }

    fn join_items(self, end: &str) -> String {
        esc!(self.items.join(ansi::ANSI_DELIMITER), end)
    }

    fn escape_each_item(self) -> String {
        self.items
            .into_iter()
            .map(|item| esc!(item))
            .collect::<Vec<_>>()
            .join("")
    }

    fn add_vec(&mut self, ansi_chars: Vec<impl Display>) {
        self.items
            .extend(ansi_chars.into_iter().map(|s| s.to_string()));
    }

    pub fn inject_maybe(self, maybe_ansiable: Option<impl Ansiable>) -> Self {
        if let Some(ansiable) = maybe_ansiable {
            return self.inject(ansiable);
        }

        self
    }

    pub fn inject(mut self, ansiable: impl Ansiable) -> Self {
        self.add_vec(ansiable.to_ansi());
        self
    }

    /// Returns (`items`, `end`), which are Ansi Escape Sequences.
    pub fn compile(self) -> String {
        match self.t {
            AnsiSequenceType::Graphic => self.join_items(ansi::END_GRAPHIC),
            AnsiSequenceType::AbsoluteMove => self.join_items(ansi::END_ABSOLUTE_MOVE),
            AnsiSequenceType::NotChainable => self.escape_each_item(),
        }
    }
}
