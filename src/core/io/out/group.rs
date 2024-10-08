use crate::core::io::ansi::sequence::{AnsiSequence, AnsiSequenceType};
use tokio::io::{AsyncWriteExt, Error as WriteError, Stdout};

use super::{flags::OutputGroupFlags, Output};

pub struct OutputGroup {
    flags: OutputGroupFlags,
    elements: Vec<Output>,
}

impl OutputGroup {
    pub fn new(flags: OutputGroupFlags, elements: Vec<Output>) -> Self {
        Self { elements, flags }
    }

    pub async fn write(self, stdout: &mut Stdout) -> Result<(), WriteError> {
        let s = AnsiSequence::new(AnsiSequenceType::NotChainable)
            .inject(self.flags)
            .compile();

        stdout.write_all(s.as_bytes()).await?;

        for mut el in self.elements {
            el.write(stdout).await?;
        }

        stdout.flush().await?;

        Ok(())
    }
}
