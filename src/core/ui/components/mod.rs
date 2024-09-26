pub mod input;


use tokio::io::Stdout;

use crate::core::cell::Position;
use crate::core::geom::align::Align;

pub type ComponentSize = (u16, u16);

#[derive(Default)]
pub struct ComponentInner {
  pub pos: Position,
  pub size: ComponentSize,
}



#[async_trait::async_trait]
pub trait Component: Send + Sync{
  async fn render(&mut self, stdout: &mut Stdout);
  fn destroy(self);
  fn align(&mut self, alignment: Align);
}
