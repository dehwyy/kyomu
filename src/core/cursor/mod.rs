use tokio::io::{AsyncWriteExt, Stdout};

use super::{
    io::ansi::{
        def::absolute_move,
        sequence::{AnsiSequence, AnsiSequenceType},
    },
    terminal::TerminalPosition,
};

pub struct Cursor;

impl Cursor {
    pub async fn move_to(coords: TerminalPosition, stdout: &mut Stdout) {
        let (s, _) = AnsiSequence::new(AnsiSequenceType::AbsoluteMove)
            .inject(absolute_move(coords))
            .compile();

        stdout.write_all(s.as_bytes()).await.unwrap();
    }

    pub fn ansi_compile_move_to(coords: TerminalPosition) -> String {
        let (s, _) = AnsiSequence::new(AnsiSequenceType::AbsoluteMove)
            .inject(absolute_move(coords))
            .compile();

        s
    }
}
