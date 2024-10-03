pub mod input;

use tokio::io::Stdout;

use crate::core::cell::Position;
use crate::core::geom::align::Align;

pub type ComponentSize = (u16, u16);

#[derive(Default)]
pub struct ComponentInner {
    pub pos: Position,
}

pub enum ComponentRenderOutput<RenderOut, DestroyOut> {
    Rendered(RenderOut),
    Destroyed(DestroyOut),
}

#[async_trait::async_trait]
pub trait Component<RenderOut, DestroyOut>: Send + Sync {
    async fn try_render(
        &mut self,
        stdout: &mut Stdout,
    ) -> ComponentRenderOutput<RenderOut, DestroyOut>;
    fn get_size(&self) -> ComponentSize;
    fn align(&mut self, alignment: Align);
}
