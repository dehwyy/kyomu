use std::fmt::Display;

use tokio::io::{AsyncWriteExt, Stdout};

use crate::{
    core::{
        cell::color::{Color, Rgb},
        event::key::{Key, KeyChar},
        geom::align::Align,
        io::{
            ansi,
            out::{
                flags::{self, OutputFlags, OutputGroupFlags},
                group::OutputGroup,
                Output,
            },
            text_decor::TextDecoration,
        },
    },
    rt::get_rt_config,
};

use crate::core::event::{Event, EventReceiver};
use crate::core::terminal::{Terminal, TerminalSize};

use super::{
    text::{TextBuilder, TextPart},
    Component, ComponentInner, ComponentRenderOutput, ComponentSize, DynamicComponent,
    StaticComponent,
};

#[derive(Default)]
pub struct InputBuilder {
    placeholder: String,
    placeholder_decor: TextDecoration,

    value: String,
    value_decor: TextDecoration,
}

impl InputBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn placeholder(mut self, placeholder: impl Display) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn placeholder_decor(mut self, placeholder_decor: TextDecoration) -> Self {
        self.placeholder_decor = placeholder_decor;
        self
    }

    pub fn default_value(mut self, value: impl Display) -> Self {
        self.value = value.to_string();
        self
    }

    pub fn input_value_decor(mut self, value_decor: TextDecoration) -> Self {
        self.value_decor = value_decor;
        self
    }

    pub fn build(self) -> Input {
        Input {
            inner: ComponentInner {
                ..Default::default()
            },
            placeholder: self.placeholder,
            placeholder_decor: self.placeholder_decor,
            value: self.value,
            value_decor: self.value_decor,
        }
    }
}

pub struct Input {
    // Inner
    inner: ComponentInner,

    // Appearance
    placeholder_decor: TextDecoration,
    value_decor: TextDecoration,

    // Data
    value: String,
    placeholder: String,
}

impl Input {
    fn get_extra_label(&self) -> String {
        format!(" {:2?}:", self.get_input_size())
    }

    fn get_input_size(&self) -> ComponentSize {
        ((self.placeholder.len() + self.value.len()) as u16, 1)
    }

    async fn destroy(&mut self, stdout: &mut Stdout) -> String {
        self.value.trim().to_string()
    }
}

impl Component for Input {
    fn get_size(&self) -> ComponentSize {
        let (w, h) = self.get_input_size();
        (w + self.get_extra_label().len() as u16, h)
    }

    fn align(mut self, alignment: Align) -> Self {
        self.inner.pos = alignment.get_offset(Terminal::get_size(), self.get_size());
        self
    }
}

#[async_trait::async_trait]
impl DynamicComponent<(), String> for Input {
    async fn try_render(
        &mut self,
        rx: &mut EventReceiver,
        stdout: &mut Stdout,
    ) -> ComponentRenderOutput<(), String> {
        let mut text_builder = TextBuilder::new()
            .add_part(TextPart::new(&self.placeholder).decor(self.placeholder_decor));

        let is_debug = get_rt_config().read().await.get_flags().is_debug();
        if is_debug {
            text_builder = text_builder.add_part(
                TextPart::new(&self.get_extra_label())
                    .decor(TextDecoration::new().flags(OutputFlags::new().italic())),
            );
        }

        text_builder
            .add_part(TextPart::new(&self.value).decor(self.value_decor))
            .build_with_align(self.inner.pos)
            .render(stdout)
            .await;

        while let Ok(new_event) = rx.try_recv() {
            if let Event::Key(key) = new_event {
                match key {
                    // Backspace
                    Key::Backspace(_) => {
                        let val_len = self.value.len();
                        if val_len > 0 {
                            self.value.remove(self.value.len() - 1);
                        }
                    }
                    Key::Char(c) => {
                        // CTRL + W
                        if c == KeyChar::build('w').ctrl() {
                            self.value = String::new();
                        } else {
                            self.value.push(c.ch);
                        }
                    }
                    // Enter -> Destroy component
                    Key::Enter(_) => {
                        return ComponentRenderOutput::Destroyed(self.destroy(stdout).await);
                    }
                    // TODO: handle
                    _ev => {}
                };
            }
        }

        ComponentRenderOutput::Rendered(())
    }
}
