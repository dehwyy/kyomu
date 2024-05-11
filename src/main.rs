
mod core;
mod game;

use core::io::{Terminal, styled::{StyledOutput, StyledInput}, Color};
use core::math::operations::MathOperation;

use game::Game;
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

    let name = input_value.to_string();

    let modes = vec!["Standard"];
    let play_mode = t.select_one("Play mode: ", &modes);


    let game = match play_mode {
        _ => standard(t),
    };

    game.start();

    // Clear terminal bytes sequence
    // print!("\x1B[2J\x1B[1;1H");
}

fn standard(mut t: Terminal) -> impl Game {
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
