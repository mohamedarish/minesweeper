#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_else_default,
    clippy::unwrap_used
)]

use iced::{Application, Settings};
use minesweeper::game::Game;
use stopwatch::Stopwatch;

fn main() {
    // println!("Hello World 🌏");

    // for terminal version of the gaem

    let sw = Stopwatch::start_new();

    // game_loop();

    Game::run(Settings::default()).expect("Working");

    println!("Finished in {} ms", sw.elapsed_ms());
}
