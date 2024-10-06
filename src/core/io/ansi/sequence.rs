use std::collections::VecDeque;
use std::fmt::Display;

use crate::escaped;

use super::def as ansi;
use super::Ansiable;

pub enum AnsiSequenceType {
    Graphic,
    AbsoluteMove,
    NotChainable,
}

pub struct AnsiSequence {
    t: AnsiSequenceType,
    start: Vec<String>,
    end: VecDeque<String>,
}

impl AnsiSequence {
    pub fn new(t: AnsiSequenceType) -> Self {
        Self {
            t,
            start: vec![],
            end: VecDeque::new(),
        }
    }

    // Push to `end_sequence`.
    fn push(&mut self, ansi_char: impl Display) {
        self.end.push_back(ansi_char.to_string());
    }

    fn add_vec(&mut self, ansi_chars: Vec<impl Display>) {
        self.start
            .extend(ansi_chars.into_iter().map(|s| s.to_string()));
    }

    pub fn inject_maybe(mut self, maybe_ansiable: Option<impl Ansiable>) -> Self {
        if let Some(ansiable) = maybe_ansiable {
            self.add_vec(ansiable.to_ansi());
        }

        self
    }

    pub fn inject(mut self, ansiable: impl Ansiable) -> Self {
        self.add_vec(ansiable.to_ansi());
        self
    }

    /// Returns (`start`, `end`), which are Ansi Escape Sequences.
    pub fn compile(mut self) -> (String, String) {
        match self.t {
            AnsiSequenceType::Graphic => {
                self.push(ansi::RESET);

                (
                    escaped!(self.start, ansi::END_GRAPHIC),
                    escaped!(self.end, ansi::END_GRAPHIC),
                )
            }
            AnsiSequenceType::NotChainable => (
                self.start
                    .into_iter()
                    .map(|s| escaped!(vec!(s)))
                    .collect::<Vec<_>>()
                    .join(""),
                String::new(),
            ),
            AnsiSequenceType::AbsoluteMove => {
                (escaped!(self.start, ansi::END_ABSOLUTE_MOVE), String::new())
            }
        }
    }
}
