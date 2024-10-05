use std::fmt::Display;

use tokio::io::Stdout;

use crate::core::{
    cell::color::Color,
    event::{key::Key, Event, EventReceiver},
    geom::align::Align,
    io::{
        ansi::def as ansi,
        out::{flags::OutputGroupFlags, group::OutputGroup},
        text_decor::TextDecoration,
    },
    terminal::Terminal,
};

use super::{
    text::{TextBuilder, TextPart},
    Component, ComponentInner, ComponentRenderOutput, ComponentSize, DynamicComponent,
    StaticComponent,
};

pub struct SelectChoice {
    pub text: String,
    pub decor: TextDecoration,
}

impl SelectChoice {
    pub fn new(text: impl Display) -> Self {
        Self {
            text: text.to_string(),
            decor: TextDecoration::default(),
        }
    }

    pub fn decor(mut self, decor: TextDecoration) -> Self {
        self.decor = decor;
        self
    }
}

#[derive(Default)]
pub struct SelectBuilder {
    placeholder: String,
    placeholder_decor: TextDecoration,

    options: Vec<SelectChoice>,

    selected_index: usize,
    selected_decoration: TextDecoration,

    multiple_choice: bool,
}

impl SelectBuilder {
    pub fn new() -> Self {
        Self {
            selected_decoration: TextDecoration::new()
                .bg_color(Color::White)
                .fg_color(Color::Black),
            ..Default::default()
        }
    }

    pub fn placeholder(mut self, placeholder: impl Display) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn placeholder_decor(mut self, placeholder_decor: TextDecoration) -> Self {
        self.placeholder_decor = placeholder_decor;
        self
    }

    pub fn add_option(mut self, option: SelectChoice) -> Self {
        self.options.push(option);
        self
    }

    pub fn selected_option_decor(mut self, decor: TextDecoration) -> Self {
        self.selected_decoration = decor;
        self
    }

    pub fn default_selected_index(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    pub fn multiple_choice(mut self, multiple_choice: bool) -> Self {
        self.multiple_choice = multiple_choice;
        self
    }

    pub fn build(self) -> Select {
        Select {
            inner: ComponentInner::default(),
            c: self,
        }
    }
}

pub struct Select {
    inner: ComponentInner,
    c: SelectBuilder,
}

impl Component for Select {
    fn get_size(&self) -> ComponentSize {
        todo!()
    }

    fn align(&mut self, alignment: Align) {
        self.inner.pos = alignment.get_offset(Terminal::get_size(), self.get_size());
    }
}

#[async_trait::async_trait]
impl DynamicComponent<(), usize> for Select {
    async fn try_render(
        &mut self,
        rx: &mut EventReceiver,
        stdout: &mut Stdout,
    ) -> ComponentRenderOutput<(), usize> {
        let mut b = TextBuilder::new()
            .add_part(TextPart::newln(&self.c.placeholder).decor(self.c.placeholder_decor));

        for (i, choice) in self.c.options.iter().enumerate() {
            let decor = match self.c.selected_index == i {
                true => self.c.selected_decoration,
                false => choice.decor,
            };

            b = b.add_part(
                TextPart::newln(format!("{tab}{}", choice.text, tab = ansi::TAB)).decor(decor),
            );
        }

        b.build().render(stdout).await;

        while let Ok(ev) = rx.try_recv() {
            if let Event::Key(key) = ev {
                match key {
                    Key::Up => {
                        if self.c.selected_index != 0 {
                            self.c.selected_index -= 1;
                        }
                    }
                    Key::Down => {
                        if self.c.selected_index < self.c.options.len() - 1 {
                            self.c.selected_index += 1;
                        }
                    }
                    Key::Enter(_) => {
                        return ComponentRenderOutput::Destroyed(self.c.selected_index);
                    }
                    _ => {}
                };
            }
        }

        ComponentRenderOutput::Rendered(())
    }
}
