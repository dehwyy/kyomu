macro_rules! ansidef {
  ($var:ident, $val:literal) => {
    pub(super) const $var: u8 = $val;
  }
}

// Global
ansidef!(RESET, 0);

// Style
ansidef!(BOLD, 1);
ansidef!(UNDERLINE, 4);

// Color
ansidef!(RED, 31);
ansidef!(GREEN, 32);
ansidef!(BLUE, 34);
ansidef!(WHITE, 37);
