bitflags::bitflags! {
  #[derive(Clone, Copy)]
  pub struct OutputFlags: u8 {
    const BOLD = 1;
    const UNDERLINE = 1 << 1;
    const CLEAR_LINE = 1 << 2;
    const CLEAR_ALL = 1 << 3;
  }
}


impl OutputFlags {
  pub fn bold(&mut self) -> &mut Self {
    *self |= OutputFlags::BOLD;
    self
  }

  pub fn underline(&mut self) -> &mut Self {
    *self |= OutputFlags::UNDERLINE;
    self
  }

  pub fn clear_line(&mut self) -> &mut Self {
    *self |= OutputFlags::CLEAR_LINE;
    self
  }

  pub fn clear_all(&mut self) -> &mut Self {
    *self |= OutputFlags::CLEAR_ALL;
    self
  }
}
