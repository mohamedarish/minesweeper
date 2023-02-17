#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_else_default,
    clippy::unwrap_used
)]

use minesweeper::game_loop;

fn main() {
    // println!("Hello, World! 🌏");

    // let mut game = Minesweeper::default(8, 8, 10);

    // game.reveal_solution();

    game_loop();
}
