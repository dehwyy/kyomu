pub mod mods;
pub mod pastas;

// maybe u8 is better here
pub type MaxGameLength = u16;

pub trait Game {
    fn start(self);
}