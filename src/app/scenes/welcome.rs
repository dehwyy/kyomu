use crate::core::geom::align::Align;
use crate::core::geom::stretch::Stretch;
use crate::core::surface::Surface;
use crate::core::ui::components::select::{Select, SelectBuilder, SelectOption};

use tokio::io::Stdout;
use tokio::sync::Mutex;

use crate::core::cell::color::{Color, Rgb};
use crate::core::event::{Event, EventReceiver};
use crate::core::io::out::flags::{OutputFlags, OutputGroupFlags};
use crate::core::io::text_decor::TextDecoration;
use crate::core::ui::components::text::{Text, TextBuilder, TextPart};
use crate::core::ui::components::{ComponentRenderOutput, DynamicComponent, StaticComponent};
use crate::core::ui::Scene;
use crate::core::ui::{
    components::{
        input::{Input, InputBuilder},
        Component,
    },
    Renderable,
};

struct WelcomeComponents {
    input: Input,
    select_mode: Select,
}

impl WelcomeComponents {
    fn new() -> Self {
        Self {
            input: InputBuilder::new()
                .placeholder("Enter your name:")
                .placeholder_decor(TextDecoration::new().fg_color(Color::Blue))
                .default_value(" ")
                .build()
                .align(Align::BottomRight),
            select_mode: SelectBuilder::new()
                .placeholder("Select mode:")
                .placeholder_decor(TextDecoration::new().fg_color(Color::Cyan))
                .add_option(
                    SelectOption::new("ImproveItYaDaun")
                        .decor(TextDecoration::new().fg_color(Color::Green)),
                )
                .add_option(
                    SelectOption::new("SnakeGame")
                        .decor(TextDecoration::new().fg_color(Color::Yellow)),
                )
                .add_option(
                    SelectOption::new("TicTacToeIdiVZhopy zhivotnoe")
                        .decor(TextDecoration::new().fg_color(Color::Red)),
                )
                .multiple()
                .align_center()
                .build()
                .align(Align::MiddleCenter),
        }
    }
}

pub struct WelcomeScene {
    stage: usize,
    rx: EventReceiver,
    components: WelcomeComponents,
}

impl WelcomeScene {
    pub fn new(rx: EventReceiver) -> Self {
        Self {
            rx,
            stage: 1,
            components: WelcomeComponents::new(),
        }
    }

    async fn render_stage_1(&mut self, stdout: &mut Stdout) {
        Surface::clear(stdout).await;

        match self.components.input.try_render(&mut self.rx, stdout).await {
            ComponentRenderOutput::Destroyed(input_value) => {
                self.stage += 1;
                TextBuilder::new()
                    .add_part(
                        TextPart::new("Hello my friend").decor(
                            TextDecoration::new()
                                .fg_color(Color::Rgb(255, 255, 0))
                                .flags(OutputFlags::new().bold().italic().underline()),
                        ),
                    )
                    .add_part(TextPart::new(", "))
                    .add_part(
                        TextPart::new(&format!("{input_value}!"))
                            .decor(TextDecoration::new().fg_color(Color::Yellow)),
                    )
                    .build()
                    .render(stdout)
                    .await;
            }
            ComponentRenderOutput::Rendered(_) => {}
        }
    }

    async fn render_stage_2(&mut self, stdout: &mut Stdout) {
        Surface::clear(stdout).await;
        Surface::hide_cursor(stdout).await;

        match self
            .components
            .select_mode
            .try_render(&mut self.rx, stdout)
            .await
        {
            ComponentRenderOutput::Destroyed(idx) => {
                self.stage += 1;
                TextBuilder::new()
                    .add_part(
                        TextPart::new("Mode: ")
                            .decor(TextDecoration::new().fg_color(Color::Yellow)),
                    )
                    .add_part(
                        TextPart::new(&format!("{:?}", idx))
                            .decor(TextDecoration::new().fg_color(Color::Green)),
                    )
                    .build()
                    .render(stdout)
                    .await;
            }
            ComponentRenderOutput::Rendered(_) => {}
        }
    }

    async fn render_stage_3(&mut self, stdout: &mut Stdout) {
        // clear_screen(stdout).await;
    }
}

#[async_trait::async_trait]
impl Renderable for WelcomeScene {
    async fn render(&mut self, stdout: &mut Stdout) {
        match self.stage {
            1 => self.render_stage_1(stdout).await,
            2 => self.render_stage_2(stdout).await,
            3 => self.render_stage_3(stdout).await,
            _ => {}
        };
    }
}

#[async_trait::async_trait]
impl Scene for WelcomeScene {
    async fn prerender_once(&mut self, stdout: &mut Stdout) {}
}
