mod core;
mod app;

use app::scenarios::Scenarios;

use app::terminal::Terminal;
use app::terminal::event::Event;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Scenarios::welcome().menu();
    // Clear terminal bytes sequence
    // print!("\x1B[2J\x1B[1;1H");
    crossterm::terminal::enable_raw_mode().unwrap();

    let t = Terminal::new();

    loop {
        if let Some(ev) = t.read_next_event() {
            match ev {
                Event::Quit => std::process::exit(0),
                _ => {}
            }
       }
    }

    Ok(())
}
