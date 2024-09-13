pub mod mods;
pub mod scenarios;
pub mod pastas;
pub mod terminal;

mod settings;

// maybe u8 is better here
pub type MaxGameLength = u16;

pub type GameResults = std::collections::HashMap<String, String>;

/// `Scenario` may be either `Game` or `Settings` (something that can be selected and started)
pub trait Scenario {
    fn start(&mut self) -> GameResults;
}
