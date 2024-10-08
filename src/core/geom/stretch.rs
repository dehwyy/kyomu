use crate::core::{terminal::TerminalSize, ui::components::ComponentSize};

#[derive(Default, Clone, Copy)]
pub enum Stretch {
    #[default]
    None,
    ByWidth,
    ByHeight,
}

impl Stretch {
    pub fn get_new_size(
        self,
        term_size: TerminalSize,
        (width, height): ComponentSize,
    ) -> ComponentSize {
        match self {
            Stretch::None => (width, height),
            Stretch::ByWidth => (term_size.0, height),
            Stretch::ByHeight => (width, term_size.1),
        }
    }
}
