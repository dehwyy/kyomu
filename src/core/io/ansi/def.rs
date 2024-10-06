macro_rules! ansidef_variable {
    ($var:ident, $val:literal) => {
        pub const $var: &str = $val;
    };
}

ansidef_variable!(ANSI_DELIMITER, ";");

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
pub fn absolute_move(coords: (u16, u16)) -> String {
    format!("{}{ANSI_DELIMITER}{}", coords.1, coords.0)
}

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
pub fn rgb(start: u8, r: u8, g: u8, b: u8) -> Vec<String> {
    vec![start, 2, r, g, b]
        .into_iter()
        .map(|v| v.to_string())
        .collect()
}

// AnsiEndChar
ansidef_variable!(END_GRAPHIC, "m");
ansidef_variable!(END_ABSOLUTE_MOVE, "H");
