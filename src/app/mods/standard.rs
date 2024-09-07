use std::ops::Range as StdRange;
use crossterm::style::Color;
use eval::eval;

use crate::core::random::Random;
use crate::core::io::{styled::{StyledInput, StyledOutput}, Terminal};
use crate::core::math::operations::MathOperation;

use crate::app::{Scenario, ScenarioWithResults, MaxGameLength};
use crate::app::pastas::{PUTIN, PUTIN_2, PEREKUR, BILLY_HERRINGTON};

type Range = StdRange<i64>;

pub enum Difficulty {
    Easy,
    Hard
}

/// Difficulty
pub struct DifficultyInfo {
    range: Range,
    operations: Vec<MathOperation>,
    stages: MaxGameLength
}

impl DifficultyInfo {
    pub fn new(difficulty: Difficulty, operations: Vec<MathOperation>) -> Self {
        match difficulty {
            Difficulty::Easy => Self::create(10..100, operations, 5),
            Difficulty::Hard => Self::create(100..1000, operations, 5),
        }
    }

    fn create(range: Range, operations: Vec<MathOperation>, stages: MaxGameLength) -> Self {
        Self { range, operations, stages }
    }
}

#[derive(Clone)]
pub struct Seed<const N: usize, const OP: usize> {
    nums: [i64; N],
    operations: [MathOperation; OP],
}

impl<const N: usize, const OP: usize> Seed<N, OP> {
    /// Unified `range`. Random peak from `Vec` of `MathOperations`.
    pub fn new(range: Range, available_operations: Vec<MathOperation>) -> Self {
        let nums: [i64; N] = Random::generate(range);
        let operations: [MathOperation; OP] = Random::peak(available_operations);

        Self {
            nums, operations
        }
    }

    pub fn to_string(&self) -> String {
        let mut nums = self.nums.iter();
        let mut s = format!("{}", nums.next().expect("cannot access index 0 in `nums` array"));

        for (operation, num) in self.operations.iter().zip(nums) {
            s += &format!(" {} {num}", operation.as_char())
        }

        s
    }

    pub fn get_result(&self) -> i64 {
        eval(&self.to_string()).expect("cannot eval result").as_i64().expect("cannot cast eval value to i64")
    }
}

pub struct Standard<'a> {
    difficulty_info: DifficultyInfo,
    t: &'a mut Terminal
}

impl<'a> Standard<'a> {
    pub fn new(difficulty: Difficulty, operations: Vec<MathOperation>, t: &'a mut Terminal) -> Self {
        Self {
            difficulty_info: DifficultyInfo::new(difficulty, operations),
            t
        }
    }

    fn print(&mut self) -> bool {
        let seed: Seed<2, 1> = Seed::new(
            self.difficulty_info.range.clone(),
            self.difficulty_info.operations.clone()
        );

        let eq = seed.to_string();
        let result = seed.get_result();

        let v = self.t.input(
            StyledOutput::new()
                .with_text(format!("{eq}: "))
                .with_color(Color::Cyan),
            Some(StyledInput::new(move |input| {
                let mut st = StyledOutput::new()
                    .with_text(format!("{eq}: {input}\n"));

                let v = input.to_i64().unwrap();

                st = match v == result {
                    true => st.with_color(Color::Green),
                    false => st.with_color(Color::Red)
                };

                st
            }))
        ).expect("cannot print!");

        v.to_i64().unwrap() == result
    }
}

impl Scenario for Standard<'_> {
    fn start(mut self) -> Self {

        // well, it means `right` or `correct`
        let mut 正 = 0;

        for _ in 0..self.difficulty_info.stages {
           if self.print() {
               正 += 1;
           }
        }

        // ALl answers are right AND funny pastas are enabled -> use them : use common words
        // `Type` may seem a bit weird, but `Random::peak` fn uses `const generic` to manipulate the number of returned values
        let outro_message: [&str; 1] = match 正 == self.difficulty_info.stages && self.t.get_settings().funny_pastas {
            true => Random::peak(vec!(PUTIN, PUTIN_2, PEREKUR, BILLY_HERRINGTON)),
            false => Random::peak(vec!("Good job!", "Nice!", "Not bad!")),
        };

        self.t.println(
            StyledOutput::new()
                .with_text(format!("{}\n", outro_message[0]))
                .with_color(Color::Magenta)
        );

        self
    }
}

impl ScenarioWithResults for Standard<'_> {
    fn get_result(self) -> i32{
        // TODO
        1i32
    }
    fn next(self) {}
}
