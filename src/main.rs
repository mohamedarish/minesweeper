#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_else_default,
    clippy::unwrap_used
)]

use game::game_loop;

pub mod game;
pub mod tiles;

fn main() {
    // println!("Hello World 🌏");

    game_loop();
}
