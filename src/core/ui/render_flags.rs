use tokio::io::{AsyncWriteExt, Stdout};

use crate::core::io::ansi::sequence::{AnsiSequence, AnsiSequenceType};

use super::Renderable;

bitflags::bitflags! {
  #[derive(Default, Clone, Copy)]
  pub struct RenderFlags: u8 {
    const CLEAR_SCREEN = 1;
    const CURSOR_HOME = 1 << 1;
    const CURSOR_HIDE = 1 << 2;
    const CURSOR_SHOW = 1 << 3;
  }
}

impl RenderFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_screen(mut self) -> Self {
        self |= RenderFlags::CLEAR_SCREEN;
        self
    }

    pub fn cursor_home(mut self) -> Self {
        self |= RenderFlags::CURSOR_HOME;
        self
    }

    pub fn cursor_hide(mut self) -> Self {
        self |= RenderFlags::CURSOR_HIDE;
        self
    }

    pub fn cursor_show(mut self) -> Self {
        self |= RenderFlags::CURSOR_SHOW;
        self
    }
}

#[async_trait::async_trait]
impl Renderable for RenderFlags {
    async fn render(&mut self, stdout: &mut Stdout) {
        let s = AnsiSequence::new(AnsiSequenceType::NotChainable)
            .inject(*self)
            .compile();

        stdout.write_all(s.as_bytes()).await.unwrap();
    }
}
