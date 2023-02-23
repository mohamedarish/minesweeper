#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_else_default,
    clippy::unwrap_used
)]

// use minesweeper::game_loop;
// use stopwatch::Stopwatch;

use iced::{Application, Settings};
use minesweeper::game::Minesweeper;

fn main() {
    // println!("Hello, World! 🌏");

    // let mut sw = Stopwatch::new();

    // sw.start();

    // game_loop();

    // sw.stop();

    // println!("You took {} ms", sw.elapsed_ms());

    Minesweeper::run(Settings::default()).expect("Could not launch the GUI");
}
