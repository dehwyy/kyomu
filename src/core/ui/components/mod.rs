pub mod input;

use tokio::io::Stdout;

use crate::core::geom::align::Align;

pub type TerminalSize = (u16, u16);

pub struct ComponentInner {

}



pub trait Component {
  fn render(&mut self, stdout: &mut Stdout);
  fn align(&mut self, alignment: Align) -> &mut Self;
}
