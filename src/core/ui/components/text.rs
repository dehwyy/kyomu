use std::process::exit;

use tokio::io::Stdout;

use crate::{
    build_padding,
    core::{
        event::EventReceiver,
        geom::align::Align,
        io::{
            ansi::{
                def as ansi,
                global::AnsiGlobal,
                sequence::{AnsiSequence, AnsiSequenceType},
            },
            out::{flags::OutputGroupFlags, group::OutputGroup, Output},
            text_decor::TextDecoration,
        },
        terminal::{Terminal, TerminalPosition},
    },
};

use super::{Component, ComponentInner, ComponentRenderOutput, ComponentSize, StaticComponent};

#[derive(Default, Clone)]
pub struct TextPart {
    text: String,
    decor: TextDecoration,
    ln: bool,
}

impl TextPart {
    pub fn new(s: impl AsRef<str>) -> Self {
        Self {
            text: s.as_ref().to_string(),
            decor: TextDecoration::default(),
            ln: false,
        }
    }

    pub fn newln(s: impl AsRef<str>) -> Self {
        Self {
            text: s.as_ref().to_string(),
            decor: TextDecoration::default(),
            ln: true,
        }
    }

    pub fn decor(mut self, decor: TextDecoration) -> Self {
        self.decor = decor;
        self
    }

    fn get_size(&self) -> (u16, u16) {
        (self.text.len() as u16, 1)
    }
}
pub struct TextBuilder {
    flags: OutputGroupFlags,
    parts: Vec<TextPart>,
    pos: TerminalPosition,
}

impl Default for TextBuilder {
    fn default() -> Self {
        Self {
            flags: OutputGroupFlags::empty(),
            parts: Vec::new(),
            pos: TerminalPosition::default(),
        }
    }
}

impl TextBuilder {
    pub fn new() -> Self {
        TextBuilder::default()
    }

    pub fn add_part(mut self, part: TextPart) -> Self {
        self.parts.push(part);
        self
    }

    pub fn flags(mut self, flags: OutputGroupFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn pos(mut self, c_pos: TerminalPosition) -> Self {
        self.pos = c_pos;
        self
    }

    pub fn build(self) -> Text {
        let (x, y) = self.pos;

        // move to base position.
        let mut reset_position_sequence = {
            let mut line = 1u16;
            move || {
                line += 1;
                TextPart::new(AnsiGlobal::MoveToCell((x, y + line)).compile())
            }
        };

        let mut parts = vec![reset_position_sequence()];

        for (i, part) in self.parts.iter().enumerate() {
            parts.push(part.clone());

            // TODO: do not call if `next` elem doesn't exist
            (part.ln && i < self.parts.len() - 1).then(|| parts.push(reset_position_sequence()));
        }

        Text {
            inner: ComponentInner::default(),
            flags: self.flags,
            parts,
        }
    }
}

pub struct Text {
    // Inner
    inner: ComponentInner,

    flags: OutputGroupFlags,
    parts: Vec<TextPart>,
}

impl Component for Text {
    fn get_size(&self) -> (u16, u16) {
        self.parts.iter().fold((0, 0), |acc, p| {
            let (w, h) = p.get_size();
            (acc.0 + w, acc.1.max(h))
        })
    }

    fn align(mut self, alignment: Align) -> Self {
        self.inner.pos = alignment.get_offset(Terminal::get_size(), self.get_size());
        self
    }
}

#[async_trait::async_trait]
impl StaticComponent for Text {
    async fn render(&mut self, stdout: &mut Stdout) {
        OutputGroup::new(
            self.flags,
            self.parts
                .iter()
                .map(|p| Output::from(p.decor).value(&p.text))
                .collect(),
        )
        .write(stdout)
        .await
        .unwrap();
    }
}
