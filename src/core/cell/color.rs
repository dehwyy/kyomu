#[derive(Copy, Clone)]
pub struct Rgb(u8, u8, u8);

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn get_r(self) -> u8 {
        self.0
    }

    pub fn get_g(self) -> u8 {
        self.1
    }

    pub fn get_b(self) -> u8 {
        self.2
    }
}

#[derive(Default, Copy, Clone)]
pub enum Color {
    #[default]
    White,
    Black,
    Red,
    Green,
    Yellow,
    Magenta,
    Cyan,
    Blue,
    Rgb(u8, u8, u8),
}

#[derive(Default, Copy, Clone)]
pub struct FgColor(pub Color);
#[derive(Copy, Clone)]
pub struct BgColor(pub Color);

impl Color {
    pub fn to_rgb(&self) -> Rgb {
        match self {
            Self::White => Rgb(255, 255, 255),
            Self::Red => Rgb(255, 0, 0),
            Self::Green => Rgb(0, 255, 0),
            Self::Blue => Rgb(0, 0, 255),
            Self::Yellow => Rgb(255, 255, 0),
            Self::Cyan => Rgb(0, 255, 255),
            Self::Magenta => Rgb(255, 0, 255),
            Self::Black => Rgb(0, 0, 0),
            Self::Rgb(r, g, b) => Rgb(*r, *g, *b),
        }
    }
}
