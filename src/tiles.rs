#[derive(Clone, Copy, Debug)]
struct Tile(i32);

struct Board([[[Tile; 2]; 8]; 8]);

impl Default for Board {
    fn default() -> Self {
        Self([[[Tile(-2), Tile(0)]; 8]; 8])
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

    fn get_random_number(start_range: i32, end_range: i32) -> i32 {
        todo!("")
    }

    fn generate_board(self, number_of_mines: usize) {
        for i in 0..number_of_mines {}
    }
}
