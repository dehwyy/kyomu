use std::borrow::BorrowMut;

use tokio::io::Stdout;

use crate::core::ui::{components::{Component, input::Input}, Renderable};
use crate::boxed;

pub struct WelcomeScene {
  stage: usize,
  components: [Box<dyn Component>; 1],
}

impl WelcomeScene {

  pub fn new() -> Self {
    Self {
      stage: 0,
      components: [boxed!(Input::new())]
    }
  }
}

#[async_trait::async_trait]
impl Renderable for WelcomeScene {
  async fn render(&mut self, stdout: &mut Stdout) {
    if let Some(mut c) = self.components.get_mut(self.stage) {
      c.render(stdout).await;
    }
  }
}
