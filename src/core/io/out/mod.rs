pub mod flags;
pub mod group;

use std::fmt::Display;

use tokio::io::{AsyncWriteExt, Error as WriteError, Stdout};

use super::{
    ansi::sequence::{AnsiSequence, AnsiSequenceType},
    text_decor::TextDecoration,
};
use flags::OutputFlags;

use crate::core::{
    cell::color::{BgColor, Color, FgColor},
    io::ansi::global::AnsiGlobal,
};

pub struct Output {
    s: String,
    flags: OutputFlags,
    fg_color: FgColor,
    bg_color: Option<BgColor>,
}

impl Output {
    pub fn new(s: impl Display) -> Self {
        Self {
            s: s.to_string(),
            flags: OutputFlags::empty(),
            fg_color: FgColor::default(),
            bg_color: None,
        }
    }

    pub fn value(mut self, s: impl AsRef<str>) -> Self {
        self.s = s.as_ref().to_string();
        self
    }

    pub fn fg_color(mut self, color: Color) -> Self {
        self.fg_color = FgColor(color);
        self
    }

    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(BgColor(color));
        self
    }

    pub fn flags(mut self, flags: OutputFlags) -> Self {
        self.flags = flags;
        self
    }

    pub async fn write(&mut self, stdout: &mut Stdout) -> Result<(), WriteError> {
        let ansi_sequence = AnsiSequence::new(AnsiSequenceType::Graphic)
            .inject(self.flags)
            .inject(self.fg_color)
            .inject_maybe(self.bg_color);

        let s = {
            let styles = ansi_sequence.compile();
            let reset = AnsiGlobal::ResetStyle;

            format!("{styles}{}{reset}", self.s)
        };

        stdout.write_all(s.as_bytes()).await?;

        Ok(())
    }
}

impl From<TextDecoration> for Output {
    fn from(decor: TextDecoration) -> Self {
        let mut out = Self::new("").flags(decor.flags).fg_color(decor.fg_color);
        if let Some(bg_color) = decor.bg_color {
            out = out.bg_color(bg_color);
        }

        out
    }
}
