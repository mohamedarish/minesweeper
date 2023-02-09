use std::{io, num::ParseIntError};

use crate::tiles::Board;

pub(crate) struct Click {
    pub x: usize,
    pub y: usize,
}

struct Game {
    board: Board,
    quit_issued: bool,
}

impl Game {
    fn default(initial_click: &Click, number_of_mines: usize) -> Self {
        Self {
            board: Board::new(initial_click, number_of_mines),
            quit_issued: false,
        }
    }
}

// This function will be used to play the terminal version of the game
pub(crate) fn game_loop() {
    Board::default().print_board();

    println!("Enter the first tile to click");

    let clicked_tile = get_valid_click();

    // println!("{}, {}", clicked_tile.x, clicked_tile.y);

    let mut game = Game::default(&clicked_tile, 10);

    loop {
        game.board.print_board();

        game.quit_issued = true;

        if game.quit_issued {
            println!("GoodBye👋");
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

        let y = if let Ok(num) = next_num() {
            num
        } else {
            println!("Please enter a valid number!");
            continue;
        };

        let x = if let Ok(num) = next_num() {
            num
        } else {
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

impl Game {}
