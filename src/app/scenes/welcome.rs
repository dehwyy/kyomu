use std::borrow::BorrowMut;

use tokio::{io::Stdout, sync::broadcast};

use crate::core::event::{Event, EventReceiver};
use crate::core::ui::components::ComponentRenderOutput;
use crate::core::ui::{
    components::{input::Input, Component},
    Renderable,
};

struct WelcomeComponents {
    input: Input,
}

impl WelcomeComponents {
    fn new(rx: EventReceiver) -> Self {
        Self {
            input: Input::new(rx).set_placeholder("Enter your name:"),
        }
    }
}

pub struct WelcomeScene {
    stage: usize,
    components: WelcomeComponents,
}

impl WelcomeScene {
    pub fn new(rx: EventReceiver) -> Self {
        Self {
            stage: 1,
            components: WelcomeComponents::new(rx),
        }
    }

    async fn render_stage_1(&mut self, stdout: &mut Stdout) {
        match self.components.input.try_render(stdout).await {
            ComponentRenderOutput::Destroyed(input_value) => {
                self.stage += 1;
                println!("Hello, {input_value}!");
            }
            ComponentRenderOutput::Rendered(_) => {}
        }
    }
}

#[async_trait::async_trait]
impl Renderable for WelcomeScene {
    async fn render(&mut self, stdout: &mut Stdout) {
        match self.stage {
            1 => self.render_stage_1(stdout).await,
            _ => {}
        };
        //   if let Some(mut c) = self.components.get_mut(self.stage) {
        //     c.render(stdout).await;
        //   }
        // }
    }
}
