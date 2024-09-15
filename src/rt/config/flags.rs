use bitflags::bitflags;


bitflags! {
    #[derive(Clone, Copy)]
    pub struct RuntimeFlags: u8 {
        const DEBUG = 1;
    }
}

impl RuntimeFlags {
    pub(super) fn initialize(args: Vec<String>) -> Self {
        println!("{args:?}");
        let mut flags = Self::empty();

        for arg in args {
            match arg.as_str() {
                "debug" => flags |= Self::DEBUG,
                _ => {}
            }
        }

        flags
    }

    pub fn is_debug(&self) -> bool {
        self.contains(Self::DEBUG)
    }
}
