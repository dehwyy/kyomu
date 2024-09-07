mod input;
pub mod terminal;
pub mod styled;

use std::io::{self, Stdout};
use crossterm::cursor::MoveUp;
use crossterm::ExecutableCommand;
use crossterm::style::PrintStyledContent;
use crossterm::terminal::{Clear, ClearType};

use dialoguer::{MultiSelect, Select};
use dialoguer::theme::ColorfulTheme;

use crate::core::io::input::InputValue;
use crate::core::io::styled::{StyledInput, StyledOutput};
use crate::input;

pub use crossterm::style::Color;
pub struct Terminal {
    stdout: Stdout,
    settings: terminal::TerminalSettings
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            settings: terminal::TerminalSettings::default()
        }
    }

    pub fn print(&mut self, p: StyledOutput) {
        self.stdout.execute(PrintStyledContent(p.to_styled())).unwrap();
    }

    pub fn println(&mut self, p: StyledOutput) {
        self.print(p.modify_text(|text|format!("{text}\n")));
    }

    pub fn input(&mut self, before: StyledOutput, after: Option<StyledInput>) -> io::Result<InputValue> {
        self.stdout.execute(PrintStyledContent(before.to_styled()))?;

        let input = InputValue::new(input!());

        if let Some(after) = after {
            self.clear_line().execute(PrintStyledContent(after.to_styled(input.clone())))?;
        }

        Ok(input)
    }

    /// Returns index of selected option
    pub fn select_one<ToStr: ToString>(&mut self, prompt: &str, options: &Vec<ToStr>, default_val: usize) -> usize {
        let select_item = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .items(options)
            .report(false)
            .default(default_val)
            .interact()
            .unwrap();

        // If `clear_line_after_action` is disabled
        if !self.settings.clear_line_after_action {
            // Print prompt once again, as after `selection` it will be cleared
            // Print without `\n`
            self.print(StyledOutput::new().with_text(prompt));

            // Print selected option (would be concatenated to the end of the prompt)
            self.println(
                StyledOutput::new()
                    .with_text(options[select_item].to_string())
                    .with_color(Color::Green)
            );
        }


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

        // if anything was selected AND `clear_line_after_action` is disabled
        if selected_items.len() != 0 && !self.settings.clear_line_after_action {
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
        }

        selected_items
    }

    /// Returns all selected options' indexes (at least one item must be selected)
    pub fn select_multiple_at_least_one<S: ToString>(&mut self, prompt: &str, options: &Vec<S>) -> Vec<usize> {
        let mut selected_items = vec!();
        while selected_items.len() == 0 {
            selected_items = self.select_multiple(prompt, options)
        }

        selected_items
    }

    pub fn update_settings(&mut self, settings: terminal::TerminalSettings) {
        self.settings = settings;
    }

    pub fn get_settings(&self) -> terminal::TerminalSettings {
        self.settings
    }

    fn clear_line(&mut self) -> &mut Stdout {
        self.stdout
            .execute(MoveUp(1))
            .expect("cannot move up by 1")
            .execute(Clear(ClearType::CurrentLine))
            .expect("cannot clear previous line")
    }
}
