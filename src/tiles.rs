use rand::Rng;

use crate::game::Click;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Tile(i32);

struct Board([[[Tile; 2]; 8]; 8]);

impl Default for Board {
    fn default() -> Self {
        Self([[[Tile(-2), Tile(0)]; 8]; 8])
    }
}

impl Board {
    fn new(initial_click: &Click, number_of_mines: usize) -> Self {
        let new_board = Board::default();

        todo!("Use generate new board to create the new board")
    }

    fn generate_board(mut self, initial_click: &Click, number_of_mines: usize) {
        for i in 0..number_of_mines {
            loop {
                let new_mine_x = get_random_number(0, 8);
                let new_mine_y = get_random_number(0, 8);

                if self.0[new_mine_y][new_mine_x][1] == Tile(-1)
                    || (new_mine_x == initial_click.x && new_mine_y == initial_click.y)
                {
                    continue;
                }

                self.0[new_mine_y][new_mine_x][1] = Tile(-1);
            }
        }
    }

    fn set_surrounding_numbers(mut self, x_pos: usize, y_pos: usize) {
        todo!("Create a function to set surrounding tiles");
    }
}

impl Board {
    fn print_board(self) {
        for row in self.0 {
            for tile in row {
                println!("{:?}", tile[0]);
            }
        }
    }

    fn reveal_solution(self) {
        for row in self.0 {
            for tile in row {
                println!("{:?}", tile[1]);
            }
        }
    }
}

fn get_random_number(start_range: i32, end_range: i32) -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(start_range..end_range)
        .try_into()
        .expect("Cannot convert to usize")
}
