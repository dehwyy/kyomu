mod core;
mod app;
mod rt;

use core::ui::Ui;
use rt::config::RuntimeConfig;

use tokio::time::{Instant, interval};
use tokio::sync::broadcast;
use futures::{StreamExt, FutureExt, select};

use crossterm::{event::EventStream, terminal::enable_raw_mode};

use app::terminal::Terminal;
use app::terminal::event::Event;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Clear terminal bytes sequence
    // print!("\x1B[2J\x1B[1;1H");

    enable_raw_mode()?;

    let mut event_reader = EventStream::new();
    let t = Terminal::new();

    let (tx, rx) = broadcast::channel::<Event>(32);


    // TODO: config update
    let rt_config = RuntimeConfig::new();

    let mut ui = Ui::new(rx);
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
                ui.render(),
                interval.tick()
            );

            _frames_rendered += 1
        }
    });

    loop {
        // proccess all events -> render -> sleep (frame time)
        select! {
            ev = event_reader.next().fuse() => {
                if let Some(Ok(ev)) = ev {
                    let ev: Event = ev.into();
                    match ev {
                        Event::Quit => std::process::exit(0),
                        e => tx.send(e).unwrap(),
                    };
                } else {
                    std::process::exit(1);
                }
            }
        };
    }
}
