use std::cmp;

use rand::Rng;

use crate::game::Click;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Tile(i32);

#[derive(Clone, Copy)]
pub(crate) struct Board([[[Tile; 2]; 8]; 8]);

impl Default for Board {
    fn default() -> Self {
        Self([[[Tile(-2), Tile(0)]; 8]; 8])
    }
}

impl Board {
    pub(crate) fn generate_mines(&mut self, initial_click: &Click, number_of_mines: usize) {
        self.generate_board(initial_click, number_of_mines);

        self.reveal_tile(initial_click.x, initial_click.y);
    }

    fn reveal_nearby_columns(&mut self, x: usize, y: usize) {
        if x > 0 && x < 7 {
            for i in (x - 1)..(x + 2) {
                self.reveal_nearby_rows(x, y, i);
            }
        } else if x == 0 {
            for i in 0..(x + 2) {
                self.reveal_nearby_rows(x, y, i);
            }
        } else if x == 7 {
            for i in (x - 1)..8 {
                self.reveal_nearby_rows(x, y, i);
            }
        }
    }

    fn reveal_nearby_rows(&mut self, x: usize, y: usize, i: usize) {
        if y > 0 && y < 7 {
            for j in (y - 1)..(y + 2) {
                if i != x || j != y {
                    self.reveal_tile(i, j);
                }
            }
        } else if y == 0 {
            for j in 0..(y + 2) {
                if i != x || j != y {
                    self.reveal_tile(i, j);
                }
            }
        } else if y == 7 {
            for j in (y - 1)..8 {
                if i != x || j != y {
                    self.reveal_tile(i, j);
                }
            }
        }
    }

    // fn reveal_nearby_tiles(&mut self, x: usize, y: usize) {
    //     if x > 0 && x < 7 {
    //         for i in (x - 1)..(x + 2) {
    //             if y > 0 && y < 7 {
    //                 for j in (y - 1)..(y + 2) {
    //                     if i != x || j != y {
    //                         self.reveal_tile(i, j);
    //                     }
    //                 }
    //             } else if y == 0 {
    //                 for j in 0..(y + 2) {
    //                     if i != x || j != y {
    //                         self.reveal_tile(i, j);
    //                     }
    //                 }
    //             } else if y == 7 {
    //                 for j in (y - 1)..8 {
    //                     if i != x || j != y {
    //                         self.reveal_tile(i, j);
    //                     }
    //                 }
    //             }
    //         }
    //     } else if x == 0 {
    //         for i in 0..(x + 2) {
    //             if y > 0 && y < 7 {
    //                 for j in (y - 1)..(y + 2) {
    //                     if i != x || j != y {
    //                         self.reveal_tile(i, j);
    //                     }
    //                 }
    //             } else if y == 0 {
    //                 for j in 0..(y + 2) {
    //                     if i != x || j != y {
    //                         self.reveal_tile(i, j);
    //                     }
    //                 }
    //             } else if y == 7 {
    //                 for j in (y - 1)..8 {
    //                     if i != x || j != y {
    //                         self.reveal_tile(i, j);
    //                     }
    //                 }
    //             }
    //         }
    //     } else if x == 7 {
    //         for i in (x - 1)..8 {
    //             if y > 0 && y < 7 {
    //                 for j in (y - 1)..(y + 2) {
    //                     if i != x || j != y {
    //                         self.reveal_tile(i, j);
    //                     }
    //                 }
    //             } else if y == 0 {
    //                 for j in 0..(y + 2) {
    //                     if i != x || j != y {
    //                         self.reveal_tile(i, j);
    //                     }
    //                 }
    //             } else if y == 7 {
    //                 for j in (y - 1)..8 {
    //                     if i != x || j != y {
    //                         self.reveal_tile(i, j);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    pub(crate) fn reveal_tile(&mut self, x: usize, y: usize) -> i32 {
        let Tile(number) = self.0[y][x][1];

        let Tile(revealed) = self.0[y][x][0];

        if revealed > -2 {
            return 1;
        }

        self.0[y][x][0] = self.0[y][x][1];

        match number.cmp(&0) {
            cmp::Ordering::Less => {
                return -1;
            }
            cmp::Ordering::Equal => {
                self.reveal_nearby_columns(x, y);
            }
            cmp::Ordering::Greater => {}
        };
        0
    }

    fn generate_board(&mut self, initial_click: &Click, number_of_mines: usize) {
        for _ in 0..number_of_mines {
            self.add_new_mine(initial_click);
            // println!("Succesfully added a mine");
        }
    }

    pub(crate) fn remaining_unrevealed_tiles(&self) -> i32 {
        let mut remaining_tiles = 0;

        for i in self.0 {
            for j in i {
                let Tile(num) = j[0];

                if num < -1 {
                    remaining_tiles += 1;
                }
            }
        }

        remaining_tiles
    }

    fn add_new_mine(&mut self, initial_click: &Click) {
        loop {
            let new_mine_x = get_random_number(0, 8);
            let new_mine_y = get_random_number(0, 8);

            // println!("{new_mine_x}, {new_mine_y}");

            if self.0[new_mine_y][new_mine_x][1] == Tile(-1)
                || (new_mine_x == initial_click.x && new_mine_y == initial_click.y)
            {
                continue;
            }

            self.set_as_mine(new_mine_x, new_mine_y);

            // self.reveal_solution();

            return;
        }
    }

    fn set_as_mine(&mut self, x_cord: usize, y_cord: usize) {
        self.set_surrounding_numbers(x_cord, y_cord);
        self.0[y_cord][x_cord][1] = Tile(-1);
    }

    fn set_surrounding_numbers(&mut self, x_pos: usize, y_pos: usize) {
        if x_pos > 0 && x_pos < 7 {
            for x in (x_pos - 1)..(x_pos + 2) {
                if y_pos > 0 && y_pos < 7 {
                    for y in (y_pos - 1)..(y_pos + 2) {
                        self.increment_tile(x, y);
                    }
                } else if y_pos == 0 {
                    for y in 0..(y_pos + 2) {
                        self.increment_tile(x, y);
                    }
                } else if y_pos == 7 {
                    for y in (y_pos - 1)..8 {
                        self.increment_tile(x, y);
                    }
                }
            }
        } else if x_pos == 0 {
            for x in (x_pos)..(x_pos + 2) {
                if y_pos > 0 && y_pos < 7 {
                    for y in (y_pos - 1)..(y_pos + 2) {
                        self.increment_tile(x, y);
                    }
                } else if y_pos == 0 {
                    for y in 0..(y_pos + 2) {
                        self.increment_tile(x, y);
                    }
                } else if y_pos == 7 {
                    for y in (y_pos - 1)..8 {
                        self.increment_tile(x, y);
                    }
                }
            }
        } else if x_pos == 7 {
            for x in (x_pos - 1)..8 {
                if y_pos > 0 && y_pos < 7 {
                    for y in (y_pos - 1)..(y_pos + 2) {
                        self.increment_tile(x, y);
                    }
                } else if y_pos == 0 {
                    for y in 0..(y_pos + 2) {
                        self.increment_tile(x, y);
                    }
                } else if y_pos == 7 {
                    for y in (y_pos - 1)..8 {
                        self.increment_tile(x, y);
                    }
                }
            }
        }
    }

    fn increment_tile(&mut self, x: usize, y: usize) {
        let Tile(old_number) = self.0[y][x][1];

        if old_number >= 0 {
            self.0[y][x][1] = Tile(old_number + 1);
        }
    }
}

impl Board {
    pub(crate) fn print_board(self) {
        for row in self.0 {
            for tile in row {
                let Tile(number) = tile[0];

                match number.cmp(&0) {
                    cmp::Ordering::Less => {
                        if number < -1 {
                            print!("⬜️\t");
                        } else {
                            print!("💣\t");
                        }
                    }
                    cmp::Ordering::Equal => print!(" \t"),
                    cmp::Ordering::Greater => print!("{number}\t"),
                }
            }
            println!();
        }
    }

    pub(crate) fn reveal_solution(self) {
        for row in self.0 {
            for tile in row {
                let Tile(number) = tile[1];

                match number.cmp(&0) {
                    cmp::Ordering::Less => print!("💣\t"),
                    cmp::Ordering::Equal => print!(" \t"),
                    cmp::Ordering::Greater => print!("{number}\t"),
                }
            }
            println!();
        }
    }
}

fn get_random_number(start_range: i32, end_range: i32) -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(start_range..end_range)
        .try_into()
        .expect("Cannot convert to usize")
}
