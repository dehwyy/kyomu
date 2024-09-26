use std::borrow::BorrowMut;

use tokio::{io::Stdout, sync::broadcast};

use crate::core::ui::{components::{input::Input, Component}, Renderable};
use crate::core::event::Event;

use crate::boxed;

pub struct WelcomeScene {
  stage: usize,
  components: [Box<dyn Component>; 1],
}

impl WelcomeScene {

  pub fn new(rx: broadcast::Receiver<Event>) -> Self {
    Self {
      stage: 0,
      components: [boxed!(Input::new(rx))]
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
