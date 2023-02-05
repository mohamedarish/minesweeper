#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_else_default,
    clippy::unwrap_used
)]

pub mod board;
pub mod game;
pub mod gui;
pub mod terminal;

fn main() {
    println!("Hello World 🌏");
}
