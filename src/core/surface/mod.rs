use tokio::io::{AsyncWriteExt, Stdout};

use super::{io::ansi::global::AnsiGlobal, terminal::Terminal};

async fn w(s: impl std::fmt::Display, stdout: &mut Stdout) {
    stdout.write_all(s.to_string().as_bytes()).await.unwrap();
}
pub struct Surface;

impl Surface {
    pub async fn clear(stdout: &mut Stdout) {
        w(AnsiGlobal::MoveToCell(Terminal::get_size()), stdout).await;
        w(AnsiGlobal::ClearScreen, stdout).await
    }

    pub async fn hide_cursor(stdout: &mut Stdout) {
        w(AnsiGlobal::CursorHide, stdout).await
    }

    pub async fn show_cursor(stdout: &mut Stdout) {
        w(AnsiGlobal::CursorShow, stdout).await
    }
}
