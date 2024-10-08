use crate::core::{
    terminal::{TerminalPosition, TerminalSize},
    ui::components::ComponentSize,
};

#[derive(Default, Clone, Copy)]
pub enum Align {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,

    MiddleLeft,
    MiddleCenter,
    MiddleRight,

    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Align {
    pub fn get_offset(
        &self,
        term_size: TerminalSize,
        (width, height): ComponentSize,
    ) -> TerminalPosition {
        let (term_width, term_height) = term_size;

        match self {
            Align::TopLeft => (0, 0),
            Align::TopCenter => ((term_width - width) / 2, 0),
            Align::TopRight => (term_width - width, 0),

            Align::MiddleLeft => (0, (term_height - height) / 2),
            Align::MiddleCenter => ((term_width - width) / 2, (term_height - height) / 2),
            Align::MiddleRight => (term_width - width, (term_height - height) / 2),

            Align::BottomLeft => (0, term_height - height),
            Align::BottomCenter => ((term_width - width) / 2, term_height - height),
            Align::BottomRight => (term_width - width, term_height - height),
        }
    }
}
