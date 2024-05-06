use std::fmt::Display;
use std::io::{Stdout, Result, self};

use dialoguer::{Select, MultiSelect, theme::ColorfulTheme};

use crossterm::cursor::MoveUp;
use crossterm::ExecutableCommand;
use crossterm::style::{PrintStyledContent, StyledContent, Stylize};
use crossterm::terminal::{Clear, ClearType};

pub use crossterm::style::Color;

use crate::input;

#[derive(Clone)]
pub struct InputValue {
    value: String
}

impl InputValue {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn to_i64(self) -> Option<i64> {
        match self.value.to_string().parse() {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }

    pub fn to_string(self) -> String {
        self.value
    }
}

impl Display for InputValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}


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
    fn to_styled(self) -> StyledContent<String> {

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

    fn to_styled(self, input: InputValue) -> StyledContent<String> {
        (self.callback)(input).to_styled()
    }
}

pub struct Terminal {
    stdout: Stdout
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout()
        }
    }

    pub fn print(&mut self, p: StyledOutput) {
        self.stdout.execute(PrintStyledContent(p.to_styled())).unwrap();
    }

    pub fn println(&mut self, p: StyledOutput) {
        self.print(p.modify_text(|text|format!("{text}\n")));
    }

    pub fn input(&mut self, before: StyledOutput, after: Option<StyledInput>) -> Result<InputValue> {
        self.stdout.execute(PrintStyledContent(before.to_styled()))?;

        let input = InputValue::new(input!());

        if let Some(after) = after {
                self.clear_line().execute(PrintStyledContent(after.to_styled(input.clone())))?;
        }

        Ok(input)
    }

    /// Returns index of selected option
    pub fn select_one<ToStr: ToString>(&mut self, prompt: &str, options: &Vec<ToStr>) -> usize {
        let select_item = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(options)
            .report(false)
            .default(0)
            .interact()
            .unwrap();

        // Print prompt once again, as after `selection` it will be cleared
        // Print without `\n`
        self.print(StyledOutput::new().with_text(prompt));

        // Print selected option (would be concatenated to the end of the prompt)
        self.println(
            StyledOutput::new()
                .with_text(options[select_item].to_string())
                .with_color(Color::Green)
        );

        select_item
    }

    /// Returns all selected options' indexes
    pub fn select_multiple<ToStr: ToString>(&mut self, prompt: &str, options: &Vec<ToStr>) -> Vec<usize> {
        let selected_items = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(options)
            .report(false)
            .interact()
            .unwrap();

        // this block works the same as in `select_one` fn
        self.print(StyledOutput::new().with_text(prompt));
        self.println(
            StyledOutput::new()
                .with_color(Color::Green)
                // Example: {options: ["a", "b", "c"]; selected: [0, 2]} -> "a, c"
                .with_text(
                    selected_items.iter()
                        .map(|idx| options[*idx].to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
        );


        selected_items
    }

    fn clear_line(&mut self) -> &mut Stdout {
        self.stdout
            .execute(MoveUp(1))
            .expect("cannot move up by 1")
            .execute(Clear(ClearType::CurrentLine))
            .expect("cannot clear previous line")
    }
}