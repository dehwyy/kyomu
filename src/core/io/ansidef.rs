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
ansidef_variable!(RESET, 0x0);
// Style
ansidef_variable!(BOLD, 0x1);
ansidef_variable!(DIM, 0x2);
ansidef_variable!(ITALIC, 0x3);
ansidef_variable!(UNDERLINE, 0x4);
ansidef_variable!(BLINKING, 0x5);
ansidef_variable!(INVERTED, 0x7);
ansidef_variable!(HIDDEN, 0x8);
ansidef_variable!(STRIKETHROUGH, 0x9);

// Color
ansidef_variable!(BLACK, 0x30);
ansidef_variable!(RED, 0x31);
ansidef_variable!(GREEN, 0x32);
ansidef_variable!(YELLOW, 0x33);
ansidef_variable!(BLUE, 0x34);
ansidef_variable!(MAGENTA, 0x35);
ansidef_variable!(CYAN, 0x36);
ansidef_variable!(WHITE, 0x37);
