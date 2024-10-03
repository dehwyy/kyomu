use tokio::io::{AsyncWriteExt, Stdout};

use crate::core::io::ansi::sequence::{AnsiSequence, AnsiSequenceType};

use super::Renderable;

bitflags::bitflags! {
  #[derive(Clone, Copy)]
  pub struct RenderFlags: u8 {
    const CLEAR_SCREEN = 1;
    const CURSOR_HOME = 1 << 1;
  }
}

#[async_trait::async_trait]
impl Renderable for RenderFlags {
    async fn render(&mut self, stdout: &mut Stdout) {
        let (s, _) = AnsiSequence::new(AnsiSequenceType::NotChainable)
            .inject(*self)
            .compile();

        stdout.write_all(s.as_bytes()).await.unwrap();
    }
}
