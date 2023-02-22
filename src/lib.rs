pub mod board;
pub mod game;

use std::{io, num::ParseIntError};

use board::Position;
use rand::Rng;

use crate::{
    board::TileStatus,
    game::{GameStatus, Minesweeper},
};

pub fn generate_random_number(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(start..end)
}

fn get_valid_click(start_range: usize, end_range: usize) -> Position {
    loop {
        let mut click = String::new();

        io::stdin()
            .read_line(&mut click)
            .expect("Unable to parse line");

        let mut subs_iter = click.split_whitespace();

        let mut next_num =
            || -> Result<usize, ParseIntError> { subs_iter.next().expect("Not parsable").parse() };

        let Ok(y) = next_num() else {
            println!("Please enter a valid number!");
            continue;
        };

        let Ok(x) = next_num() else {
            println!("Please enter a valid number!");
            continue;
        };

        if x >= start_range || y >= end_range {
            println!(
                "Please enter a valid number less than {end_range} and greater than {start_range}!"
            );
            continue;
        }

        break Position { x, y };
    }
}

fn get_number(start: usize, end: usize) -> usize {
    loop {
        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("Cannot read line");

        match num.trim().parse() {
            Ok(num) => {
                if start <= num && num < end {
                    return num;
                }

                println!("Enter a number less than {end} and more than {start}");

                continue;
            }
            Err(_) => continue,
        }
    }
}

pub fn game_loop() {
    // game.reveal_solution();
    println!("Enter the height and width of the board: ");

    let wh = get_valid_click(99, 99);

    println!("Enter the number of mines: ");

    let mines = get_number(1, wh.x * wh.y);

    let mut game = Minesweeper::default(wh.y, wh.x, mines);

    loop {
        game.print_board();

        println!("Enter the y, x cordinate of the tile to be revealed: ");

        let click = get_valid_click(wh.x, wh.y);

        let res = game.board.reveal_tile(click);

        if res == TileStatus::Mine {
            game.result = GameStatus::Loss;
        }

        game.result_decided();

        match game.result {
            GameStatus::Victory => {
                println!("Congrats!!");
                break;
            }
            GameStatus::Loss => {
                println!("Try again next time!!");
                break;
            }
            GameStatus::NotDecided => continue,
        }
    }
    game.reveal_solution();
}
