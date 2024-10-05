use tokio::io::{AsyncWriteExt, Stdout};

use crate::core::io::ansi::sequence::{AnsiSequence, AnsiSequenceType};

use super::Renderable;

bitflags::bitflags! {
  #[derive(Default, Clone, Copy)]
  pub struct RenderFlags: u8 {
    const CLEAR_SCREEN = 1;
    const CURSOR_HOME = 1 << 1;
  }
}

impl RenderFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_screen(&mut self) -> &mut Self {
        *self |= RenderFlags::CLEAR_SCREEN;
        self
    }

    pub fn cursor_home(&mut self) -> &mut Self {
        *self |= RenderFlags::CURSOR_HOME;
        self
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
