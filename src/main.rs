
mod core;
mod game;

use game::scenaries::Scenries;


fn main() {
    Scenries::welcome().menu();
    // Clear terminal bytes sequence
    // print!("\x1B[2J\x1B[1;1H");
}
