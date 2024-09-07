pub mod game;

use game::GameScenarios;

use crate::core::io::Terminal;
use crate::core::io::{styled::{StyledInput, StyledOutput}, Color};

use crate::app::{Scenario, ScenarioWithResults};

pub struct Scenarios {
  name: String,
  t: Terminal
}

impl Scenarios {
  /// Start scenario
  pub fn welcome() -> Self {
    // create `terminal` instance`
    let mut t = Terminal::new();

    // `Welcome! ...`
    t.println(
      StyledOutput::new()
        .with_text("Welcome to Kyomu!")
        .with_color(Color::Blue)
    );

    // "Name? ..."
    let name = t.input(
      StyledOutput::new()
          .with_text("What's your name?: ")
          .with_color(Color::Cyan),
      Some(StyledInput::new(|input| {
          StyledOutput::new().with_text(format!("Hello, {}!\n", input.to_string().trim()))
      }))
    ).unwrap().to_string();

    Self { t, name }
  }

  /// End scenario
  pub fn menu(mut self) {
    // 0. Games
    // 1. Settings
    // 2. Exit
    let options = vec!["Play games", "Go to settings", "Exit"];

    match self.t.select_one("What do you want to do?: ", &options) {
        0 => self.games(),    // Play games
        1 => self.settings(), // Settings TODO
        _ => return,          // Exit on `2` or and invalid input
    };
  }

  fn games(mut self) {
    // 1. Standard
    // 2. Snake (PLANNING)
    let modes = vec!["Standard"];

    let game_scenario = match self.t.select_one("Play mode: ", &modes) {
        _ => GameScenarios::standart(&mut self.t)
    };

    let _ = game_scenario.start().get_result();

    // after game end -> return to menu
    self.menu()
  }

  fn settings(mut self) {
    // 0.1. filter??
    // 0.2 main color?
    // 0. displayed name
    // 1. funny pastas
    // 2. exit
    let displayed_name = format!("Displayed name: {} (change)", &self.name);
    let options: Vec<&str> = vec![&displayed_name, "Funny pastas", "Exit"];

    match self.t.select_one("Settings: ", &options) {
        0 => {},
        1 => {},
        _ => return
    }

    self.menu();
  }
}
