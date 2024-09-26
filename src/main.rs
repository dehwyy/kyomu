mod core;
mod app;
mod rt;

use core::event::Event;
use core::terminal::Terminal;
use core::ui::Ui;
use app::scenes;
use rt::config::RuntimeConfig;

use tokio::time::{Instant, interval};
use tokio::sync::broadcast;
use std::process::exit;
use futures::{FutureExt, select};

use crossterm::{event::EventStream, terminal::enable_raw_mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // print!("\x1B[2J\x1B[1;1H");

    enable_raw_mode()?;

    let mut event_reader = EventStream::new();

    let (tx, rx) = broadcast::channel::<Event>(32);
    let mut ui = Ui::new();

    ui.set_scene(scenes::WelcomeScene::new(tx.subscribe()));

    let mut t = Terminal::new(rx, ui);

    // TODO: config update
    let rt_config = RuntimeConfig::new();

    tokio::spawn(async move {
        let mut _frames_rendered = 0u32;
        let _start = Instant::now();

        let mut interval = interval(rt_config.get_frame_time());

        loop {
            if rt_config.get_flags().is_debug() {
                println!(
                    "rendering frame {_frames_rendered}, time passed {}, sleep time {}",
                    _start.elapsed().as_millis(),
                    rt_config.get_frame_time().as_millis()
                );
            }

            tokio::join!(
                t.render(),
                interval.tick()
            );

            _frames_rendered += 1
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
