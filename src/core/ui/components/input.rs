use crate::core::geom::align::Align;

use super::Component;

#[derive(Default)]
pub struct Input {
  value: Option<String>,
  placeholder: Option<String>
}

impl Input {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn set_placeholder(&mut self, placeholder: String) -> &mut Self {
    self.placeholder = Some(placeholder);
    self
  }
}

impl Component for Input {
  fn render(&mut self, stdout: &mut tokio::io::Stdout) {
      todo!()
  }

  fn align(&mut self, alignment: Align) -> &mut Self {
   todo!()
  }
}
