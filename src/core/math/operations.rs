use std::fmt::{Formatter, Display, self};

#[derive(Copy, Clone)]
pub enum MathOperation {
    Add,
    Subtract,
    Multiply
}

impl Display for MathOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl Default for MathOperation {
    fn default() -> Self {
        MathOperation::Add
    }
}

impl MathOperation {
    pub fn as_char(&self) -> char {
        match self {
            MathOperation::Add => '+',
            MathOperation::Subtract => '-',
            MathOperation::Multiply => '*',
        }
    }

    pub fn perform(&self, a: i64, b: i64) -> i64 {
        match self {
            MathOperation::Add => a + b,
            MathOperation::Subtract => a - b,
            MathOperation::Multiply => a * b
        }
    }
}
