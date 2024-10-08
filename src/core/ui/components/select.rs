use std::{fmt::Display, process::exit};

use tokio::io::Stdout;

use crate::{
    build_padding,
    core::{
        cell::color::Color,
        event::{key::Key, Event, EventReceiver},
        geom::align::Align,
        io::{
            ansi::def as ansi,
            out::{flags::OutputGroupFlags, group::OutputGroup},
            text_decor::TextDecoration,
        },
        terminal::Terminal,
    },
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
    lpadding: u16,
    // To align text by center
    align_padding: u16,
    align_center: bool,

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
            align_center: false,

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

    pub fn align_center(mut self) -> Self {
        self.align_center = true;
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
    fn calculate_s_inner_padding(&self, s_len: usize) -> (u16, u16) {
        if !self.c.align_center {
            return (0, 0);
        }
        let delta = (self.c.longest_s_len - s_len) as u16;
        let lpadding = delta / 2;

        (lpadding, delta - lpadding)
    }

    fn calculate_outer_padding(
        &self,
        s_len: usize,
        is_focused: bool,
        is_selected: bool,
    ) -> (u16, u16) {
        // by default, align to longest string.
        let mut align_to = self.c.longest_s_len;

        // if `align_center` -> no `outer` r_align. Only `inner` alignment (to center).
        if self.c.align_center {
            align_to = s_len;
        }

        // `prefix` (len ==1) + `gap between words` (len == 1)
        const GAP: u16 = 2;

        let mut lpadding = self.c.lpadding;
        let mut rpadding = (align_to - s_len) as u16 + GAP;

        if is_selected {
            lpadding = lpadding.saturating_sub(GAP);
        }

        if is_focused {
            rpadding = rpadding.saturating_sub(GAP);
        }

        (lpadding, rpadding)
    }

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

        // const GAP: u16 = 2;
        // let mut lpadding = self.c.lpadding;
        // let mut align_padding = (self.c.longest_s_len - option.text.len()) as u16 + GAP;
        let mut prefix = String::new();
        let mut suffix = String::new();

        if is_selected {
            prefix = self.c.selected_prefix.clone();
        }

        if is_focused {
            suffix = self.c.focused_suffix.clone();
        }

        let (inner_lpadding, inner_rpadding) = self.calculate_s_inner_padding(option.text.len());
        let (lpadding, rpadding) =
            self.calculate_outer_padding(option.text.len(), is_focused, is_selected);

        builder
            .add_part(TextPart::new(format!(
                "{pad}{prefix} ",
                pad = build_padding!(lpadding)
            )))
            .add_part(
                TextPart::new(format!(
                    "{lpad}{s}{rpad}",
                    s = option.text,
                    lpad = build_padding!(inner_lpadding),
                    rpad = build_padding!(inner_rpadding)
                ))
                .decor(decor),
            )
            .add_part(TextPart::newln(format!(
                " {pad}{suffix}",
                pad = build_padding!(rpadding)
            )))
    }
}

impl Component for Select {
    fn get_size(&self) -> ComponentSize {
        let (l1, r1) = self.calculate_outer_padding(self.c.longest_s_len, true, true);
        let (l2, r2) = self.calculate_s_inner_padding(self.c.longest_s_len);

        let w = l1 + r1 + l2 + r2 + self.c.longest_s_len as u16 + self.c.lpadding;
        let h = self.c.options.len() as u16 + 1;

        (w, h)
    }

    fn align(mut self, alignment: Align) -> Self {
        self.inner.alignment = alignment;
        self.inner.pos = alignment.get_offset(Terminal::get_size(), self.get_size());
        self
    }
}

#[async_trait::async_trait]
impl DynamicComponent<(), Vec<usize>> for Select {
    async fn try_render(
        &mut self,
        rx: &mut EventReceiver,
        stdout: &mut Stdout,
    ) -> ComponentRenderOutput<(), Vec<usize>> {
        // Cursor::move_to(self.inner.pos, stdout).await;

        // let mut b = TextBuilder::new();
        // b.add_part(TextPart::new("debug!"))
        //     .build()
        //     .render(stdout)
        //     .await;

        // exit(1);

        let mut b = TextBuilder::new()
            .add_part(TextPart::newln(&self.c.placeholder).decor(self.c.placeholder_decor));

        for (i, choice) in self.c.options.iter().enumerate() {
            b = self.apply_option_style(i, choice, b);
        }

        b.pos(self.inner.pos).build().render(stdout).await;

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
