#[cfg(test)]
mod tests {
    use minesweeper::{
        game::Click,
        get_random_number,
        tiles::{Board, Tile},
    };

    #[test]
    fn edge_test_00() {
        let mut board = Board::default();

        board.generate_mines(&Click { x: 0, y: 0 }, 63);

        let Tile(number) = board.0[0][0][1];

        assert_ne!(number, -1);
    }

    #[test]
    fn edge_test_x0() {
        let mut board = Board::default();

        board.generate_mines(&Click { x: 7, y: 0 }, 63);

        let Tile(number) = board.0[0][7][1];

        assert_ne!(number, -1);
    }

    #[test]
    fn edge_test_0y() {
        let mut board = Board::default();

        board.generate_mines(&Click { x: 0, y: 7 }, 63);

        let Tile(number) = board.0[7][0][1];

        assert_ne!(number, -1);
    }

    #[test]
    fn edge_test_xy() {
        let mut board = Board::default();

        board.generate_mines(&Click { x: 7, y: 7 }, 63);

        let Tile(number) = board.0[7][7][1];

        assert_ne!(number, -1);
    }

    #[test]
    fn non_edge_test() {
        let mut board = Board::default();

        let x = get_random_number(1, 7);
        let y = get_random_number(1, 7);

        board.generate_mines(&Click { x, y }, 63);

        let Tile(number) = board.0[y][x][1];

        assert_eq!(number, 8);
    }

    #[test]
    fn board_generation_test() {
        let mut board = Board::default();

        let x = get_random_number(0, 8);
        let y = get_random_number(0, 8);

        board.generate_mines(&Click { x, y }, 10);

        board.reveal_tile(x, y);

        let mut number_of_mines = 0;

        for i in 0..8 {
            for j in 0..8 {
                if board.0[j][i][1] == Tile(-1) {
                    number_of_mines += 1;
                }
            }
        }

        assert_eq!(number_of_mines, 10);
    }
}
