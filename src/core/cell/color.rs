#[derive(Copy, Clone)]
pub struct Rgb(u8, u8, u8);

#[derive(Default)]
pub enum Color {
  #[default]
  White,
  Red,
  Green,
  Blue,
  Rgb(Rgb)
}

impl Color {
  pub fn to_rgb(&self) -> Rgb {
    match self {
      Self::White => Rgb(255, 255, 255),
      Self::Red => Rgb(255, 0, 0),
      Self::Green => Rgb(0, 255, 0),
      Self::Blue => Rgb(0, 0, 255),
      Self::Rgb(rgb_color) => *rgb_color
    }
  }
}
