pub mod components;

use tokio::io::{stdout, Stdout};


pub type TerminalSize = (u16, u16);
pub type TerminalPosition = (u16, u16);



#[async_trait::async_trait]
pub trait Renderable: Send {
  async fn render(&mut self, stdout: &mut Stdout);
}

pub struct Ui {
  stdout: Stdout,
  scene: Option<Box<dyn Renderable>>
}

impl Ui {
  pub fn new() -> Self {
    Self {
      stdout: stdout(),
      scene: None
    }
  }

  pub fn set_scene(&mut self, scene: impl Renderable + 'static) {
    self.scene = Some(Box::new(scene));
  }

  pub async fn render(&mut self) {
    if let Some(scene) = &mut self.scene {
      scene.render(&mut self.stdout).await;
    }
  }
}
