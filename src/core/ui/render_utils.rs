use tokio::io::Stdout;

use crate::core::io::ansi::def::absolute_move;

use super::{render_flags::RenderFlags, Renderable};

pub async fn clear_screen(stdout: &mut Stdout) {
    RenderFlags::new()
        .clear_screen()
        .cursor_home()
        .render(stdout)
        .await;
}

pub async fn disable_cursor(stdout: &mut Stdout) {
    RenderFlags::new().cursor_hide().render(stdout).await;
}

pub async fn enable_cursor(stdout: &mut Stdout) {
    RenderFlags::new().cursor_show().render(stdout).await;
}

pub async fn move_to(stdout: &mut Stdout, coords: (u16, u16)) {
    absolute_move(coords);
    todo!()
}
