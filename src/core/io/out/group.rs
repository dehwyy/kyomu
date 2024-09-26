use tokio::io::{Stdout, AsyncWriteExt, Error as WriteError};
use crate::core::io::ansi::sequence::{AnsiSequence, AnsiSequenceType};

use super::{flags::OutputGroupFlags, Output};

pub struct OutputGroup {
  flags: OutputGroupFlags,
  elements: Vec<Output>
}

impl OutputGroup {
  pub fn new(flags: OutputGroupFlags, elements: Vec<Output> ) -> Self {
    Self { elements, flags }
  }

  pub async fn write(self, stdout: &mut Stdout) -> Result<(), WriteError> {

    let (begin, end) = AnsiSequence::new(AnsiSequenceType::NotChainable).inject(self.flags).compile();
    let s = format!("{begin}{end}");

    // println!("{}", s.escape_debug());

    stdout.write_all(s.as_bytes()).await?;

    for mut el in self.elements {
      el.write(stdout).await?;
    }

    stdout.flush().await?;


    Ok(())
  }
}
