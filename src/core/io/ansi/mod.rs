pub mod def;
pub mod ansiable;
pub mod sequence;

use super::out::flags::OutputFlags;

pub trait Ansiable {
  fn to_ansi(self) -> Vec<String>;
}
