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
