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
    text::{Text, TextBuilder, TextPart},
    Component, ComponentInner, ComponentRenderOutput, ComponentSize, DynamicComponent,
    StaticComponent,
};

pub struct SelectOption {
    pub text: String,
    pub decor: TextDecoration,
}

impl SelectOption {
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

pub struct SelectBuilder {
    // Data
    placeholder: String,
    focused: usize,
    selected: Vec<usize>,
    multiple: bool,

    // Theme
    options: Vec<SelectOption>,
    placeholder_decor: TextDecoration,
    selected_decoration: TextDecoration,
    selected_prefix: String,
    focused_suffix: String,
    focused_decoration: TextDecoration,
    lpadding: u8,
    // To align text by center
    align_padding: u8,

    // Computed
    longest_s_len: usize,
}

impl Default for SelectBuilder {
    fn default() -> Self {
        Self {
            placeholder: String::new(),
            focused: 0,
            selected: vec![],
            multiple: false,

            options: Vec::new(),
            placeholder_decor: TextDecoration::default(),
            selected_prefix: "✅".to_string(),
            selected_decoration: TextDecoration::new()
                .bg_color(Color::Blue)
                .fg_color(Color::Red),

            focused_suffix: "<".to_string(),
            focused_decoration: TextDecoration::new()
                .bg_color(Color::White)
                .fg_color(Color::Black),
            lpadding: 4,
            align_padding: 0,

            longest_s_len: 0,
        }
    }
}

impl SelectBuilder {
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

    pub fn add_option(mut self, option: SelectOption) -> Self {
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
            c: Self {
                selected,
                longest_s_len: self
                    .options
                    .iter()
                    .map(|opt| opt.text.len())
                    .max()
                    .unwrap_or(0),
                ..self
            },
        }
    }
}

pub struct Select {
    inner: ComponentInner,
    c: SelectBuilder,
}

impl Select {
    fn apply_option_style(
        &self,
        idx: usize,
        option: &SelectOption,
        builder: TextBuilder,
    ) -> TextBuilder {
        let is_focused = self.c.focused == idx;
        let is_selected = self.c.selected.contains(&idx);

        let decor = match (is_focused, is_selected) {
            (true, _) => self.c.focused_decoration,
            (_, true) => self.c.selected_decoration,
            _ => option.decor,
        };

        const GAP: u8 = 2;

        let mut lpadding = self.c.lpadding;
        let mut align_padding = self.c.longest_s_len - option.text.len() + GAP as usize;
        let mut prefix = String::new();
        let mut suffix = String::new();

        if is_selected {
            // Add prefix & Align with padding
            prefix = self.c.selected_prefix.clone();
            lpadding -= GAP;
        }

        if is_focused {
            suffix = self.c.focused_suffix.clone();
            align_padding -= GAP as usize;
        }

        let lpadding_with_space: String = (0..lpadding).map(|_| ' ').collect();
        let align_padding_with_space: String = (0..align_padding).map(|_| ' ').collect();

        builder
            .add_part(TextPart::new(format!("{lpadding_with_space}{prefix} ")))
            .add_part(TextPart::new(&option.text).decor(decor))
            .add_part(TextPart::newln(format!(
                " {align_padding_with_space}{suffix}"
            )))
    }
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
            b = self.apply_option_style(i, choice, b);
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
