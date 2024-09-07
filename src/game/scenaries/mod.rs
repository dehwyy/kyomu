use crate::core::io::Terminal;
use crate::core::io::{styled::{StyledInput, StyledOutput}, Color};

use crate::game::Stage;
use crate::game::mods::Standard;

pub struct Scenries {
  t: Terminal
}

impl Scenries {
  /// Start scenary
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
    ).unwrap();

    // Check saved presets
    let _ = name.to_string();

    Self { t }
  }

  /// End scenary
  pub fn menu(mut self) {
    // 1. Games
    // 2. Settings
    // 3. Exit
    let options = vec!["Play games", "Go to settings", "Exit"];

    match self.t.select_one("What do you want to do?: ", &options) {
        0 => self.games(),    // Play games
        1 => self.settings(), // Settings TODO
        _ => return,          // Exit on `2` or and invalid input
    };
  }

  fn games(mut self) -> Self {
    // 1. Standard
    // 2. Snake (PLANNING)
    let modes = vec!["Standard"];

    let t = match self.t.select_one("Play mode: ", &modes) {
        _ => Standard::init(self.t).enter()
    };

    Self { t }
  }

  fn settings(self) -> Self {
    todo!()
  }
}
