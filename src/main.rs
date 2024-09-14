mod core;
mod app;

use core::ui::Ui;

use tokio::time::{Instant, Duration, sleep};
use tokio::sync::broadcast;
use futures::{StreamExt, FutureExt, select};

use crossterm::{event::EventStream, terminal::enable_raw_mode};

use app::terminal::Terminal;
use app::terminal::event::Event;


const FPS: u64 = 60;
const FRAME_TIME: Duration = Duration::from_millis(1_000 / FPS);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Clear terminal bytes sequence
    // print!("\x1B[2J\x1B[1;1H");

    enable_raw_mode()?;

    let mut event_reader = EventStream::new();
    let t = Terminal::new();

    let (tx, rx) = broadcast::channel::<Event>(32);


    let mut ui = Ui::new(rx);
    tokio::spawn(async move {
        loop {
            tokio::join!(
                ui.render(),
                sleep(FRAME_TIME)
            );
        }
    });

    loop {
        // proccess all events -> render -> sleep (frame time)
        select! {
            ev = event_reader.next().fuse() => {
                if let Some(Ok(ev)) = ev {
                    sleep(Duration::from_millis(10)).await;
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
