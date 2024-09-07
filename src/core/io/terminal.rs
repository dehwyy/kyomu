#[derive(Copy, Clone)]
pub struct TerminalSettings {
  pub clear_line_after_action: bool,
  pub funny_pastas: bool
}

impl Default for TerminalSettings {
  fn default() -> Self {
    Self {
      clear_line_after_action: true,
      funny_pastas: true
    }
  }
}

impl TerminalSettings {
  pub fn new() -> Self { Self::default() }

  pub fn set_clear_line_after_action(mut self, value: bool) -> Self {
    self.clear_line_after_action = value;
    self
  }

  pub fn set_funny_pastas(mut self, value: bool) -> Self {
    self.funny_pastas = value;
    self
  }
}
