pub mod mods;
pub mod scenaries;
pub mod pastas;

mod settings;

// maybe u8 is better here
pub type MaxGameLength = u16;

/// `Scenary` may be either `Game` or `Settings` (something that can be selected and started)
pub trait Scenary {
    fn start(self) -> Self;
}

pub trait ScenaryWithResults: Scenary {
    fn get_result(self) -> i32;
    fn next(self);
}
