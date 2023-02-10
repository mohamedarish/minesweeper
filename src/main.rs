#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_or_else_default,
    clippy::unwrap_used
)]

use druid::{AppLauncher, WindowDesc};
use game::game_loop;
use gui::build_board;

pub mod game;
pub mod gui;
pub mod tiles;

fn main() {
    // println!("Hello World 🌏");

    // game_loop();

    let window = WindowDesc::new(build_board())
        .window_size((600., 600.))
        .title("Minesweeper 💣");

    AppLauncher::with_window(window)
        .launch(())
        .expect("Failed to launch application");

    // for terminal version of the gaem
    game_loop();
}
