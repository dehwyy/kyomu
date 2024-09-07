
mod core;
mod app;

use app::scenarios::Scenarios;


fn main() {
    Scenarios::welcome().menu();
    // Clear terminal bytes sequence
    // print!("\x1B[2J\x1B[1;1H");
}
