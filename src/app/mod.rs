pub mod mods;
pub mod scenarios;
pub mod pastas;

mod settings;

// maybe u8 is better here
pub type MaxGameLength = u16;

/// `Scenario` may be either `Game` or `Settings` (something that can be selected and started)
pub trait Scenario {
    fn start(self) -> Self;
}

pub trait ScenarioWithResults: Scenario {
    fn get_result(self) -> i32;
    fn next(self);
}
