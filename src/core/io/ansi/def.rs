macro_rules! ansidef_variable {
    ($var:ident, $val:literal) => {
        pub const $var: &str = $val;
    };
}

macro_rules! ansidef_function {
    ($fn_name:ident, $var_args_name:ident,  $fn_args:tt, $fn:expr) => {
        pub fn $fn_name($var_args_name: $fn_args) -> Vec<String> {
            $fn($var_args_name)
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        }
    };
    ($var:ident,  $fn_args:tt, $fn:expr) => {
        ansidef_function!($var, args, $fn_args, $fn);
    };
}

// Global
ansidef_variable!(RESET, "0");
ansidef_variable!(TAB, "\t");
ansidef_variable!(NEW_LINE, "\r\n");
ansidef_variable!(CARET_RESET, "\r");
ansidef_variable!(CLEAR_LINE, "2K");
ansidef_variable!(CLEAR_SCREEN, "1J");
ansidef_variable!(CURSOR_HIDE, "?25l");
ansidef_variable!(CURSOR_SHOW, "?25h");
ansidef_variable!(CURSOR_HOME, "H");
ansidef_function!(absolute_move, coords, (u16, u16), |(x, y)| vec!(y, x));

// Style
ansidef_variable!(BOLD, "1");
ansidef_variable!(DIM, "2");
ansidef_variable!(ITALIC, "3");
ansidef_variable!(UNDERLINE, "4");
ansidef_variable!(BLINKING, "5"); // Switches from `Normal` to `Dim` color.
ansidef_variable!(INVERTED, "7"); // Switches background and foreground colors.
ansidef_variable!(HIDDEN, "8");
ansidef_variable!(STRIKETHROUGH, "9");

// Color
ansidef_variable!(BLACK, "30");
ansidef_variable!(RED, "31");
ansidef_variable!(GREEN, "32");
ansidef_variable!(YELLOW, "33");
ansidef_variable!(BLUE, "34");
ansidef_variable!(MAGENTA, "35");
ansidef_variable!(CYAN, "36");
ansidef_variable!(WHITE, "37");
ansidef_variable!(RGB, "38");
ansidef_function!(rgb, (u8, u8, u8, u8), |(start, r, g, b)| vec!(
    start, 2, r, g, b
));

// AnsiEndChar
ansidef_variable!(END_GRAPHIC, "m");
ansidef_variable!(END_ABSOLUTE_MOVE, "H");
