use tokio::io::{AsyncWriteExt, Stdout};

use super::{flags::OutputGroupFlags, Output};

pub struct OutputGroup {
  flags: OutputGroupFlags,
  elements: Vec<Output>
}

impl OutputGroup {
  pub fn new(flags: OutputGroupFlags, elements: Vec<Output> ) -> Self {
    Self { elements, flags }
  }

  pub async fn write(self, stdout: &mut Stdout) {
    for mut el in self.elements {
      el.write(stdout).await.unwrap();
    }
  }
}
