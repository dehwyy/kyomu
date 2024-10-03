use super::out::flags::OutputFlags;
use crate::core::cell::color::Color;

#[derive(Default, Clone, Copy)]
pub struct TextDecoration {
    pub(super) flags: OutputFlags,
    pub(super) fg_color: Color,
    pub(super) bg_color: Option<Color>,
}

impl TextDecoration {
    pub fn build() -> Self {
        Self::default()
    }

    pub fn flags(mut self, flags: OutputFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn fg_color(mut self, color: Color) -> Self {
        self.fg_color = color;
        self
    }

    pub fn bg_color(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }
}
