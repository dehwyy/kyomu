use std::fmt::{Display, self};

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
}

impl Display for InputValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

