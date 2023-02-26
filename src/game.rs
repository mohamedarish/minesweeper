use stopwatch::Stopwatch;

use crate::board::{Board, Cell, Position};

#[derive(Clone, PartialEq, Debug)]
pub enum GameStatus {
    NotDecided,
    Victory,
    Loss,
}

#[derive(Clone)]
pub struct Minesweeper {
    pub board: Board,
    pub result: GameStatus,
    pub timer: Stopwatch,
}

impl Minesweeper {
    pub fn default(height: usize, width: usize, number_of_mines: usize) -> Self {
        let board = vec![vec![Cell::default(); width]; height];
        Self {
            board: Board {
                rows: board,
                number_of_mines,
            },
            result: GameStatus::NotDecided,
            timer: Stopwatch::start_new(),
        }
    }
}

impl Minesweeper {
    pub fn reveal_solution(&self) {
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                if self.board.rows[y][x].is_mine {
                    print!("💣\t");
                } else {
                    print!("{}\t", self.board.tile_number(Position { x, y }));
                }
            }
            println!();
        }
    }

    pub fn print_board(&self) {
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                if self.board.is_revealed(Position { x, y }) {
                    if self.board.rows[y][x].is_mine {
                        print!("💣\t");
                    } else {
                        print!("{}\t", self.board.tile_number(Position { x, y }));
                    }
                } else {
                    print!("🟧\t");
                }
            }
            println!();
        }
    }

    fn remaining_unrevealed_tiles(&self) -> usize {
        let mut tiles = 0;

        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                if !self.board.is_revealed(Position { x, y }) {
                    tiles += 1;
                }
            }
        }

        tiles
    }

    pub fn result_decided(&mut self) {
        if self.remaining_unrevealed_tiles() == self.board.number_of_mines {
            self.result = GameStatus::Victory;
        }
    }

    pub fn stop_timer(&mut self) {
        if self.result != GameStatus::NotDecided {
            self.timer.stop();
        }
    }
}
