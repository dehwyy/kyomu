use crate::core::cell::color::{BgColor, Color, FgColor};
use crate::core::io::out::flags::{OutputFlags, OutputGroupFlags};

use super::def as ansi;
use super::Ansiable;

macro_rules! impl_color_ansiable {
    ($s:ident, $is_bg:literal) => {
        impl Ansiable for $s {
            fn to_ansi(self) -> Vec<String> {
                // `fg` + 10 == 'bg'
                let with_delta = move |ansi: &str| {
                    ansi.parse::<u8>().unwrap() + $is_bg.then_some(10u8).unwrap_or(0)
                };

                match self.0 {
                    Color::Rgb(r, g, b) => ansi::rgb(with_delta(ansi::RGB), r, g, b),
                    common_color => {
                        let color = match common_color {
                            Color::White => ansi::WHITE,
                            Color::Red => ansi::RED,
                            Color::Green => ansi::GREEN,
                            Color::Blue => ansi::BLUE,
                            Color::Magenta => ansi::MAGENTA,
                            Color::Cyan => ansi::CYAN,
                            Color::Black => ansi::BLACK,
                            Color::Yellow => ansi::YELLOW,
                            _ => ansi::WHITE,
                        };

                        vec![(with_delta(color)).to_string()]
                    }
                }
            }
        }
    };
}

impl_color_ansiable!(FgColor, false);
impl_color_ansiable!(BgColor, true);

impl Ansiable for Vec<String> {
    fn to_ansi(self) -> Vec<String> {
        self
    }
}

impl Ansiable for OutputFlags {
    fn to_ansi(self) -> Vec<String> {
        let mut v = vec![];

        if self.contains(OutputFlags::BOLD) {
            v.push(ansi::BOLD);
        }

        if self.contains(OutputFlags::DIM) {
            v.push(ansi::DIM);
        }

        if self.contains(OutputFlags::ITALIC) {
            v.push(ansi::ITALIC);
        }

        if self.contains(OutputFlags::UNDERLINE) {
            v.push(ansi::UNDERLINE);
        }

        if self.contains(OutputFlags::BLINKING) {
            v.push(ansi::BLINKING);
        }

        if self.contains(OutputFlags::INVERTED) {
            v.push(ansi::INVERTED);
        }

        if self.contains(OutputFlags::HIDDEN) {
            v.push(ansi::HIDDEN);
        }

        if self.contains(OutputFlags::STRIKETHROUGH) {
            v.push(ansi::STRIKETHROUGH);
        }

        v.iter().map(|s| s.to_string()).collect()
    }
}

impl Ansiable for OutputGroupFlags {
    fn to_ansi(self) -> Vec<String> {
        let mut v = vec![];

        if self.contains(OutputGroupFlags::CLEAR_LINE) {
            v.extend([ansi::CARET_RESET, ansi::CLEAR_LINE]);
        }

        if self.contains(OutputGroupFlags::NEW_LINE) {
            v.push(ansi::NEW_LINE);
        }

        v.iter().map(|s| s.to_string()).collect()
    }
}
