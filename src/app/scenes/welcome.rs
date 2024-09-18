use tokio::io::Stdout;

use crate::core::ui::Renderable;

pub struct WelcomeScene;

impl WelcomeScene {
  pub fn new() -> Self {
    Self
  }
}

impl Renderable for WelcomeScene {
  fn render(&self, _out: &mut Stdout) {
    todo!()
  }
}
