pub mod out_flags;
pub mod out;

#[macro_export]
macro_rules! ansidef {
    ($start_var:ident, $end_var:ident, $start:literal, $end:literal) => {
        const $start_var: &str = $start;
        const $end_var: &str = $end;
    };
}

#[macro_export]
macro_rules! escaped {
    ($s:expr) => {{
      format!("\x1b[{}m", $s.into_iter().collect::<Vec<_>>().join(";"))
    }};
}
