macro_rules! ansidef_variable {
  ($var:ident, $val:literal) => {
    pub(super) const $var: u8 = $val;
  }
}


macro_rules! ansidef_function {
    ($var:ident, $fn_args:tt, $fn:expr) => {
        pub fn $var(args: $fn_args) -> Vec<String> {
          $fn(args).into_iter().map(|s| s.to_string()).collect::<Vec<_>>()
        }
    };
}

// Global
ansidef_variable!(RESET, 0);
// Style
ansidef_variable!(BOLD, 1);
ansidef_variable!(DIM, 2);
ansidef_variable!(ITALIC, 3);
ansidef_variable!(UNDERLINE, 4);
ansidef_variable!(BLINKING, 5);
ansidef_variable!(INVERTED, 7);
ansidef_variable!(HIDDEN, 8);
ansidef_variable!(STRIKETHROUGH, 9);

// Color
ansidef_variable!(BLACK, 30);
ansidef_variable!(RED, 31);
ansidef_variable!(GREEN, 32);
ansidef_variable!(YELLOW, 33);
ansidef_variable!(BLUE, 34);
ansidef_variable!(MAGENTA, 35);
ansidef_variable!(CYAN, 36);
ansidef_variable!(WHITE, 37);

ansidef_variable!(RGB, 38);
ansidef_function!(rgb, (u8, u8, u8, u8), |(start, r, g, b)| vec!(start, 2, r, g, b));
