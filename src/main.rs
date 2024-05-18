
mod core;
mod game;

use core::io::{Terminal, styled::{StyledOutput, StyledInput}, Color};
use core::math::operations::MathOperation;

use game::Stage;
use game::mods::standard::{Standard, Difficulty};


fn main() {
    let mut t = Terminal::new();

    t.println(
        StyledOutput::new()
            .with_text("Welcome to Kyomu!")
            .with_color(Color::Blue)
    );

    let input_value = t.input(
        StyledOutput::new()
            .with_text("What's your name?: ")
            .with_color(Color::Cyan),
        Some(StyledInput::new(|input| {
            StyledOutput::new().with_text(format!("Hello, {}!\n", input.to_string().trim()))
        }))
    ).unwrap();

    let _ = input_value.to_string();

    let paths = vec!["Play games", "Go to settings", "Exit"];
    let stage= match t.select_one("What do you want to do?: ", &paths) {
        0 => {}, // Play games
        1 => {}, // Settings
        _ => return, // Exit on `2` or and invalid input
    };

    let modes = vec!["Standard"];
    let play_mode = t.select_one("Play mode: ", &modes);


    let game = match play_mode {
        _ => standard(t),
    };

    game.enter();

    // Clear terminal bytes sequence
    // print!("\x1B[2J\x1B[1;1H");
}

fn standard(mut t: Terminal) -> impl Stage {
    let difficulties = vec!("Easy", "Hard");

    let difficulty = match t.select_one("Difficulty: ", &difficulties) {
        0 => Difficulty::Easy,
        1 => Difficulty::Hard,
        _ => Difficulty::Easy,
    };

    let operations = vec!(MathOperation::Add, MathOperation::Subtract);
    let selected_operations = t.select_multiple_at_least_one("Mathematical operations: ", &operations)
            .iter()
            .map(|idx| operations[*idx])
            .collect::<Vec<_>>();


    Standard::new(difficulty, selected_operations, t)
}
