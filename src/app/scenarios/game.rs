use crate::core::io::Terminal;
use crate::core::math::MathOperation;

use crate::app::ScenarioWithResults;

use crate::app::mods::standard;

pub struct GameScenarios;

impl GameScenarios {
  pub fn standart(t: &mut Terminal) -> impl ScenarioWithResults + '_ {
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

    Standard::new(difficulty, selected_operations, t)
  }
}
