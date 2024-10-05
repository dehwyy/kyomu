use tokio::io::Stdout;

use crate::core::{
    event::EventReceiver,
    geom::align::Align,
    io::{
        ansi,
        out::{flags::OutputGroupFlags, group::OutputGroup, Output},
        text_decor::TextDecoration,
    },
    terminal::Terminal,
};

use super::{Component, ComponentInner, ComponentRenderOutput, StaticComponent};

#[derive(Default)]
pub struct TextPart {
    text: String,
    decor: TextDecoration,
}

impl TextPart {
    pub fn new(s: impl AsRef<str>) -> Self {
        Self {
            text: s.as_ref().to_string(),
            decor: TextDecoration::default(),
        }
    }

    pub fn newln(s: impl AsRef<str>) -> Self {
        Self::new(format!("{s}{ln}", s = s.as_ref(), ln = ansi::def::NEW_LINE))
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
}

impl Default for TextBuilder {
    fn default() -> Self {
        Self {
            flags: OutputGroupFlags::empty(),
            parts: Vec::new(),
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

    pub fn build(self) -> Text {
        Text {
            inner: ComponentInner::default(),
            flags: self.flags,
            parts: self.parts,
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

    fn align(&mut self, alignment: Align) {
        self.inner.pos = alignment.get_offset(Terminal::get_size(), self.get_size());
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
