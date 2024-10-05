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

    multiple: bool,
    selected: Vec<usize>,
    selected_decoration: TextDecoration,

    focused: usize,
    focused_decoration: TextDecoration,
}

impl SelectBuilder {
    pub fn new() -> Self {
        Self {
            selected_decoration: TextDecoration::new()
                .bg_color(Color::Blue)
                .fg_color(Color::Red),
            focused_decoration: TextDecoration::new()
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

    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    pub fn default_selected(mut self, selected: Vec<usize>) -> Self {
        self.selected = selected;
        self
    }

    pub fn default_focused(mut self, focused: usize) -> Self {
        self.focused = focused;
        self
    }

    pub fn selected_option_decor(mut self, decor: TextDecoration) -> Self {
        self.selected_decoration = decor;
        self
    }

    pub fn focused_option_decor(mut self, decor: TextDecoration) -> Self {
        self.focused_decoration = decor;
        self
    }

    pub fn build(self) -> Select {
        let selected = match self.multiple {
            // Multiple. At least 1 element.
            true => match self.selected.is_empty() {
                true => vec![self.focused],
                false => self.selected,
            },
            // Single. At most 1 element.
            false => vec![*self.selected.first().unwrap_or(&self.focused)],
        };
        Select {
            inner: ComponentInner::default(),
            c: Self { selected, ..self },
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
impl DynamicComponent<(), Vec<usize>> for Select {
    async fn try_render(
        &mut self,
        rx: &mut EventReceiver,
        stdout: &mut Stdout,
    ) -> ComponentRenderOutput<(), Vec<usize>> {
        let mut b = TextBuilder::new()
            .add_part(TextPart::newln(&self.c.placeholder).decor(self.c.placeholder_decor));

        for (i, choice) in self.c.options.iter().enumerate() {
            let is_focused = self.c.focused == i;
            let is_selected = self.c.selected.contains(&i);

            let mut s = String::from(&choice.text);
            let mut padding = 4u8;

            let decor = if is_focused {
                s += " <";
                self.c.focused_decoration
            } else if is_selected {
                self.c.selected_decoration
            } else {
                choice.decor
            };

            if is_selected {
                s = format!("✅ {s}");
                padding -= 3;

                if !is_focused {
                    s = format!("{s}  ");
                }
            }

            let padding = (0..padding).map(|_| ' ').collect::<String>();

            b = b.add_part(TextPart::newln(format!("{padding}{s}")).decor(decor));
        }

        b.build().render(stdout).await;

        while let Ok(ev) = rx.try_recv() {
            if let Event::Key(key) = ev {
                match key.vim() {
                    Key::Up => {
                        if self.c.focused != 0 {
                            self.c.focused -= 1;
                        }
                    }
                    Key::Down => {
                        if self.c.focused < self.c.options.len() - 1 {
                            self.c.focused += 1;
                        }
                    }
                    Key::Space => {
                        if self.c.multiple {
                            // Is not included -> push
                            if !self.c.selected.contains(&self.c.focused) {
                                self.c.selected.push(self.c.focused);
                            }
                            // Included -> remove
                            else {
                                self.c.selected.retain(|i| *i != self.c.focused);
                            }
                        } else {
                            self.c.selected = vec![self.c.focused];
                        }
                    }
                    Key::Enter(_) => {
                        return ComponentRenderOutput::Destroyed(self.c.selected.clone());
                    }
                    _ => {}
                };
            }
        }

        ComponentRenderOutput::Rendered(())
    }
}
