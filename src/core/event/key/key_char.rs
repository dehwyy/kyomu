use super::Modifier;

#[derive(Eq, PartialEq)]
pub struct KeyChar {
  pub ch: char,
  pub modifiers: Vec<Modifier>
}

impl KeyChar {
  pub(super) fn new(ch: char, modifiers: Vec<Modifier>) -> Self {
    Self { ch, modifiers }
  }


}

impl KeyChar {
  pub fn build(ch: char) -> Self {
      Self::new(ch, vec!())
  }

  pub fn ctrl(mut self) -> Self {
    self.modifiers.push(Modifier::Ctrl);
    self
  }

  pub fn shift(mut self) -> Self {
    self.modifiers.push(Modifier::Shift);
    self
  }

  pub fn alt(mut self) -> Self {
    self.modifiers.push(Modifier::Alt);
    self
  }
}
