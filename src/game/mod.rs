pub mod mods;
pub mod pastas;
mod settings;

// maybe u8 is better here
pub type MaxGameLength = u16;

/// `Stage` may be either `Game` or `Settings` (something that can be selected and started)
pub trait Stage {
    fn enter(self);
}