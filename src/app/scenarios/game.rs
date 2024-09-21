use crate::core::io2::Terminal;
use crate::core::math::MathOperation;

use crate::app::Scenario;
use crate::app::mods::{standard, snake};

pub struct GameScenarios;

impl GameScenarios {
  pub fn standart(t: &mut Terminal) -> Box<dyn Scenario + '_> {
    use standard::{Difficulty, Standard};

    let difficulties = vec!("Easy", "Hard");

    let difficulty = match t.select_one("Difficulty: ", &difficulties, 0) {
          0 => Difficulty::Easy,
          1 => Difficulty::Hard,
          _ => Difficulty::Easy,
    };

    let operations = vec!(MathOperation::Add, MathOperation::Subtract);
    let selected_operations = t.select_multiple_at_least_one("Mathematical operations: ", &operations)
              .iter()
              .map(|idx| operations[*idx])
              .collect::<Vec<_>>();

    Box::new(Standard::new(difficulty, selected_operations, t))
  }

  pub fn snake(t: &mut Terminal) -> Box<dyn Scenario + '_> {
    use snake::SnakeGame;
    Box::new(SnakeGame::new(t))
  }
}
