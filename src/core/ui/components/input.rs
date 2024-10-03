use std::fmt::Display;

use tokio::io::{AsyncWriteExt, Stdout};

use crate::core::{
    cell::color::{Color, Rgb},
    event::key::Key,
    geom::align::Align,
    io::{
        out::{
            flags::{self, OutputFlags, OutputGroupFlags},
            group::OutputGroup,
            Output,
        },
        text_decor::TextDecoration,
    },
};

use crate::core::event::{Event, EventReceiver};
use crate::core::terminal::{Terminal, TerminalSize};

use super::{Component, ComponentInner, ComponentRenderOutput, ComponentSize};

pub struct InputBuilder {
    placeholder: String,
    placeholder_decor: TextDecoration,

    value: String,
    value_decor: TextDecoration,
}

impl Default for InputBuilder {
    fn default() -> Self {
        Self {
            placeholder: String::from("Input"),
            placeholder_decor: TextDecoration::build()
                .flags(OutputFlags::UNDERLINE | OutputFlags::BOLD)
                .fg_color(Color::Rgb(Rgb::new(230, 100, 240))),

            value: String::from(" "),
            value_decor: TextDecoration::build().fg_color(Color::Blue),
        }
    }
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

    pub fn build(self, rx: EventReceiver) -> Input {
        Input {
            rx,
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
    rx: EventReceiver,
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
        stdout.write_all(b"\n").await.unwrap();
        stdout.flush().await.unwrap();

        self.value.trim().to_string()
    }
}

#[async_trait::async_trait]
impl Component<(), String> for Input {
    async fn try_render(&mut self, stdout: &mut Stdout) -> ComponentRenderOutput<(), String> {
        while let Ok(new_event) = self.rx.try_recv() {
            if let Event::Key(key) = new_event {
                match key {
                    Key::Backspace(_) => {
                        let val_len = self.value.len();
                        if val_len > 0 {
                            self.value.remove(self.value.len() - 1);
                        }
                    }
                    Key::Char(c) => {
                        self.value.push(c.ch);
                    }
                    // Destroy component
                    Key::Enter(_) => {
                        return ComponentRenderOutput::Destroyed(self.destroy(stdout).await);
                    }
                    _ev => {}
                };
            }
        }

        let mut output_group = Vec::from([
            Output::from(self.placeholder_decor).value(&self.placeholder),
            Output::from(self.value_decor).value(&self.value),
        ]);

        // TODO: debug mode
        let is_debug = false;
        if is_debug {
            output_group.push(Output::new(self.get_extra_label()));
        }

        OutputGroup::new(*OutputGroupFlags::empty().clear_line(), output_group)
            .write(stdout)
            .await
            .unwrap();

        ComponentRenderOutput::Rendered(())
    }

    fn get_size(&self) -> ComponentSize {
        let (w, h) = self.get_input_size();
        (w + self.get_extra_label().len() as u16, h)
    }

    fn align(&mut self, alignment: Align) {
        self.inner.pos = alignment.get_offset(Terminal::get_size(), self.get_size());
    }
}
