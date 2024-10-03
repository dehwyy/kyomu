pub mod components;
mod raw_ansi;

pub use raw_ansi::RawAnsi;

use tokio::io::{stdout, Stdout};

#[async_trait::async_trait]
pub trait Renderable: Send {
    async fn render(&mut self, stdout: &mut Stdout);
}

#[async_trait::async_trait]
pub trait Scene: Renderable + Send {
    async fn prerender_once(&mut self, stdout: &mut Stdout) {}
}

pub struct Ui {
    stdout: Stdout,
    scene: Option<Box<dyn Renderable>>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            stdout: stdout(),
            scene: None,
        }
    }

    pub async fn set_scene(&mut self, mut scene: impl Scene + 'static) {
        scene.prerender_once(&mut self.stdout).await;
        self.scene = Some(Box::new(scene));
    }

    pub async fn render(&mut self) {
        if let Some(scene) = &mut self.scene {
            scene.render(&mut self.stdout).await;
        }
    }
}
