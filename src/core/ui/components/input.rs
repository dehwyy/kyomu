use std::fmt::Display;

use tokio::io::{AsyncWriteExt, Stdout};

use crate::core::{
    cell::color::{Color, Rgb},
    event::key::Key,
    geom::align::Align,
    io::out::{
        flags::{self, OutputFlags, OutputGroupFlags},
        group::OutputGroup,
        Output,
    },
};

use crate::core::event::{Event, EventReceiver};
use crate::core::terminal::{Terminal, TerminalSize};

use super::{Component, ComponentInner, ComponentRenderOutput, ComponentSize};

pub struct Input {
    rx: EventReceiver,
    value: String,
    placeholder: String,
    inner: ComponentInner,
}

impl Input {
    pub fn new(rx: EventReceiver) -> Self {
        Self {
            rx,
            value: String::from(" "),
            placeholder: String::from("Input"),
            inner: ComponentInner {
                ..Default::default()
            },
        }
    }

    fn get_placeholder(&self) -> String {
        self.placeholder.clone()
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }

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

    pub fn set_placeholder(mut self, placeholder: impl Display) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn set_value(mut self, value: impl Display) -> Self {
        self.value = value.to_string();
        self
    }
}

#[async_trait::async_trait]
impl Component<(), String> for Input {
    async fn try_render(&mut self, stdout: &mut Stdout) -> ComponentRenderOutput<(), String> {
        while let Ok(new_event) = self.rx.try_recv() {
            if let Event::Key(key) = new_event {
                match key {
                    Key::Backspace(_) => {
                        let val_len = self.get_value().len();
                        if val_len > 0 {
                            // self.set_value(self.value[0..val_len - 1].to_string());
                            self.value.remove(self.get_value().len() - 1);
                        }
                    }
                    Key::Char(c) => {
                        // self.set_value(format!("{}{}", self.get_value(), c.ch));
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
            Output::new(self.get_placeholder())
                .flags(OutputFlags::UNDERLINE | OutputFlags::BOLD)
                .fg_color(Color::Rgb(Rgb::new(230, 100, 240)))
                .bg_color(Color::Blue),
            Output::new(self.get_value()).fg_color(Color::Rgb(Rgb::new(230, 100, 240))),
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
