pub mod ansiable;
pub mod def;
pub mod sequence;

pub trait Ansiable {
    fn to_ansi(self) -> Vec<String>;
}
