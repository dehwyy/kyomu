#[macro_export]
macro_rules! input {
    () => {{
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    }};
}

#[macro_export]
macro_rules! boxed {
    ($($x:expr),*) => {{
        Box::new(($($x),*))
    }}
}

#[macro_export]
macro_rules! escaped {
    ($s:expr, $end_char:expr) => {{
        let join_char = match $end_char.len() {
            0 => "",
            _ => ";",
        };

        format!(
            "\x1b[{}{}",
            $s.into_iter().collect::<Vec<_>>().join(join_char),
            $end_char
        )
    }};

    ($s:expr) => {{
        escaped!($s, "")
    }};
}

#[macro_export]
macro_rules! esc {
    ($s:expr, $end:expr) => {{
        format!("\x1b[{}{}", $s, $end)
    }};

    ($s:expr) => {{
        esc!($s, "")
    }};
}
