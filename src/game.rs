use std::{io, num::ParseIntError};

use crate::tiles::Board;

#[derive(Clone, Copy)]
pub(crate) struct Click {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Clone)]
enum Status {
    Lose,
    None,
    Win,
}

#[derive(Clone)]
pub(crate) struct Game {
    pub(crate) board: Board,
    result: Status,
}

impl Game {
    pub(crate) fn default() -> Self {
        Self {
            board: Board::default(),
            result: Status::None,
        }
    }
}

// This function will be used to play the terminal version of the game
pub(crate) fn game_loop() {
    let mut game = Game::default();

    game.board.print_board();

    println!("Enter the first tile to click");

    let clicked_tile = get_valid_click();

    // println!("{}, {}", clicked_tile.x, clicked_tile.y);

    game.board.generate_mines(&clicked_tile, 10);

    loop {
        game.board.print_board();

        // game.board.reveal_solution();

        let new_click = get_valid_click();

        let issue = game.board.reveal_tile(new_click.x, new_click.y);

        if issue < 0 {
            game.result = Status::Lose;
        }
        if game.board.remaining_unrevealed_tiles() <= 10 {
            game.result = Status::Win;
        }

        if game.result != Status::None {
            if game.result == Status::Win {
                game.board.print_board();
                println!("Congrats");
            } else {
                game.board.reveal_solution();
                println!("Better Luck Next Time");
            }
            break;
        }
    }
}

fn get_valid_click() -> Click {
    loop {
        let mut click = String::new();

        io::stdin()
            .read_line(&mut click)
            .expect("Unable to parse line");

        let mut subs_iter = click.split_whitespace();

        let mut next_num = || -> Result<usize, ParseIntError> {
            // subs_iter.next().expect("Not a number")
            //     .parse().expect("Not a number");

            subs_iter.next().expect("Not parsable").parse()
        };

        // let y = match next_num() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("Please enter a valid number less than 7 and greater than 0");
        //         continue;
        //     }
        // };

        let Ok(y) = next_num() else {
            println!("Please enter a valid number!");
            continue;
        };

        let Ok(x) = next_num() else {
            println!("Please enter a valid number!");
            continue;
        };

        // let x = match next_num() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("Please enter a valid number less than 7 and greater than 0");
        //         continue;
        //     }
        // };

        if x > 7 || y > 7 {
            println!("Please enter a valid number less than 7 and greater than 0!");
            continue;
        }

        break Click { x, y };
    }
}

// impl Game {}
