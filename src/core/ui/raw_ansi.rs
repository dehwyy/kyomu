use tokio::io::{AsyncWriteExt, Stdout};

use crate::escaped;

use super::Renderable;

pub struct RawAnsi {
    ansi: Vec<String>,
}

impl RawAnsi {
    pub fn new(ansi: &[&'static str]) -> Self {
        Self {
            ansi: ansi.into_iter().map(|s| escaped!(vec!(*s))).collect(),
        }
    }
}

#[async_trait::async_trait]
impl Renderable for RawAnsi {
    async fn render(&mut self, stdout: &mut Stdout) {
        for s in &self.ansi {
            stdout.write_all(s.as_bytes()).await.unwrap();
        }

        stdout.flush().await.unwrap();
    }
}
