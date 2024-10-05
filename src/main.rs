mod app;
mod core;
mod rt;

use app::scenes;
use core::event::Event;
use core::io::out::flags::OutputGroupFlags;
use core::terminal::Terminal;
use core::ui::components::text::{TextBuilder, TextPart};
use core::ui::components::Component;
use core::ui::Ui;
use rt::get_rt_config;

use futures::{select, FutureExt};
use std::process::exit;
use tokio::sync::broadcast;
use tokio::time::{interval, Instant};

use crossterm::{event::EventStream, terminal::enable_raw_mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // print!("\x1B[2J\x1B[1;1H");

    enable_raw_mode()?;

    // let mut event_reader = EventStream::new();

    let (tx, rx) = broadcast::channel::<Event>(32);
    let mut ui = Ui::new();

    ui.set_scene(scenes::WelcomeScene::new(tx.subscribe()))
        .await;

    let mut t = Terminal::new(rx, ui);

    let rt_config = get_rt_config();

    tokio::spawn(async move {
        let rt_config = rt_config.read().await;
        let frame_time = rt_config.get_frame_time();

        let mut interval = interval(frame_time);

        drop(rt_config);

        loop {
            tokio::join!(t.render(), interval.tick());
        }
    });

    loop {
        select! {
            ev = Event::read().fuse() => {
                if let Some(ev) = ev {
                    match ev {
                        Event::Quit => exit(0),
                        ev => tx.send(ev).unwrap()
                    };
                }
            }
        }
    }
}
