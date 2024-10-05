pub mod input;
pub mod select;
pub mod text;

use tokio::io::Stdout;

use crate::core::cell::Position;
use crate::core::event::EventReceiver;
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

pub trait Component: Send + Sync {
    fn get_size(&self) -> ComponentSize;
    fn align(&mut self, alignment: Align);
}

#[async_trait::async_trait]
pub trait StaticComponent: Component {
    async fn render(&mut self, stdout: &mut Stdout);
}

#[async_trait::async_trait]
pub trait DynamicComponent<RenderOut, DestroyOut>: Component {
    async fn try_render(
        &mut self,
        rx: &mut EventReceiver,
        stdout: &mut Stdout,
    ) -> ComponentRenderOutput<RenderOut, DestroyOut>;
}
