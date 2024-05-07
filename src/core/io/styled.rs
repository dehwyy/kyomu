use crossterm::style::{Color, StyledContent, Stylize};
use crate::core::io::input::InputValue;

#[derive(Default)]
pub struct StyledOutput {
    text: String,
    color: Option<Color>,
    bg: Option<Color>,
}

impl StyledOutput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn with_text(mut self, text: impl AsRef<str>) -> Self {
        self.text = text.as_ref().to_string();
        self
    }

    pub fn modify_text(mut self, text_callback: impl FnOnce(String) -> String) -> Self {
        self.text = text_callback(self.text);
        self
    }

    /// Consumes `self` and returns `StyledContent`
    pub(super) fn to_styled(self) -> StyledContent<String> {

        let mut styled_text = self.text.stylize();

        if let Some(color) = self.color {
            styled_text = styled_text.with(color);
        }
        if let Some(bg) = self.bg {
            styled_text = styled_text.on(bg);
        }

        styled_text
    }
}

pub struct StyledInput<'a> {
    callback: Box<dyn FnOnce(InputValue) -> StyledOutput + 'a>
}

impl<'a> StyledInput<'a> {
    pub fn new(callback: impl FnOnce(InputValue) -> StyledOutput + 'a) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }

    pub(super) fn to_styled(self, input: InputValue) -> StyledContent<String> {
        (self.callback)(input).to_styled()
    }
}