bitflags::bitflags! {
  #[derive(Default, Clone, Copy)]
  pub struct OutputFlags: u8 {
    const BOLD = 1;
    const DIM = 1 << 1;
    const ITALIC = 1 << 2;
    const UNDERLINE = 1 << 3;
    const BLINKING = 1 << 4;
    const INVERTED = 1 << 5;
    const HIDDEN = 1 << 6;
    const STRIKETHROUGH = 1 << 7;
  }
}

impl OutputFlags {
    pub fn new() -> Self {
        Self::empty()
    }

    pub fn bold(mut self) -> Self {
        self |= OutputFlags::BOLD;
        self
    }

    pub fn dim(mut self) -> Self {
        self |= OutputFlags::DIM;
        self
    }

    pub fn italic(mut self) -> Self {
        self |= OutputFlags::ITALIC;
        self
    }

    pub fn underline(mut self) -> Self {
        self |= OutputFlags::UNDERLINE;
        self
    }

    pub fn blinking(mut self) -> Self {
        self |= OutputFlags::BLINKING;
        self
    }

    pub fn inverted(mut self) -> Self {
        self |= OutputFlags::INVERTED;
        self
    }

    pub fn hidden(mut self) -> Self {
        self |= OutputFlags::HIDDEN;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self |= OutputFlags::STRIKETHROUGH;
        self
    }
}

bitflags::bitflags! {
  #[derive(Default, Clone, Copy)]
  pub struct OutputGroupFlags: u8 {
    const NEW_LINE = 1;
    const CLEAR_LINE = 1 << 1;
  }
}

impl OutputGroupFlags {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_line(mut self) -> Self {
        self |= OutputGroupFlags::NEW_LINE;
        self
    }

    pub fn clear_line(mut self) -> Self {
        self |= OutputGroupFlags::CLEAR_LINE;
        self
    }
}
