use crate::core::io::Terminal;
use crate::core::math::MathOperation;

use crate::game::ScenaryWithResults;

use crate::game::mods::standard;

pub struct PrepareGameScenries;

impl PrepareGameScenries {
  pub fn standart(t: &mut Terminal) -> impl ScenaryWithResults + '_ {
    use standard::{Difficulty, Standard};

    let difficulties = vec!("Easy", "Hard");

    let difficulty = match t.select_one("Difficulty: ", &difficulties) {
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
