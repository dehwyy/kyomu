use crate::core::io::ansi::def as ansi;

use tokio::{io::Stdout, sync::broadcast};

use crate::core::cell::color::Color;
use crate::core::event::{Event, EventReceiver};
use crate::core::io::out::flags::OutputFlags;
use crate::core::io::text_decor::TextDecoration;
use crate::core::ui::components::text::{Text, TextBuilder, TextPart};
use crate::core::ui::components::ComponentRenderOutput;
use crate::core::ui::{
    components::{
        input::{Input, InputBuilder},
        Component,
    },
    Renderable,
};
use crate::core::ui::{RawAnsi, Scene};

struct WelcomeComponents {
    input: Input,
}

impl WelcomeComponents {
    fn new(rx: EventReceiver) -> Self {
        Self {
            input: InputBuilder::new().build(rx),
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
                TextBuilder::new()
                    .add_part(
                        TextPart::new("Hello").decor(
                            TextDecoration::new()
                                .fg_color(Color::Red)
                                .flags(OutputFlags::STRIKETHROUGH),
                        ),
                    )
                    .add_part(TextPart::new(", "))
                    .add_part(
                        TextPart::new(&format!("{input_value}!"))
                            .decor(TextDecoration::new().fg_color(Color::Green)),
                    )
                    .build()
                    .try_render(stdout)
                    .await;
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

#[async_trait::async_trait]
impl Scene for WelcomeScene {
    async fn prerender_once(&mut self, stdout: &mut Stdout) {
        RawAnsi::new(&[ansi::CLEAR_SCREEN, ansi::CURSOR_HOME])
            .render(stdout)
            .await;
    }
}
