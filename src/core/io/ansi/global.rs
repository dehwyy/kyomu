use std::fmt::{self, Display, Formatter};

use crate::{
    core::{io::ansi::def as ansi, terminal::TerminalPosition},
    esc,
};

pub enum AnsiGlobal {
    ResetStyle,
    MoveToCell(TerminalPosition),
}

impl Display for AnsiGlobal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let v = match self {
            AnsiGlobal::ResetStyle => esc!(ansi::RESET, ansi::END_GRAPHIC),
            AnsiGlobal::MoveToCell(pos) => esc!(ansi::absolute_move(*pos), ansi::END_ABSOLUTE_MOVE),
        };

        write!(f, "{v}")
    }
}

impl AnsiGlobal {
    pub fn compile(self) -> String {
        self.to_string()
    }
}
