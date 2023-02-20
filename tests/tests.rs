use minesweeper::{
    board::{Board, Position},
    generate_random_number,
};

#[test]
fn board_generation_test() {
    let mut board = Board::default(8, 8, 10);
    board.reveal_tile(Position { x: 0, y: 0 });

    let mut number_of_mines = 0;

    for i in 0..8 {
        for j in 0..8 {
            if board.rows[i][j].is_mine {
                number_of_mines += 1;
            }
        }
    }

    assert_eq!(number_of_mines, 10);
}

#[test]
fn board_size_test() {
    let board = Board::default(8, 8, 10);

    let height = board.height();
    let width = board.width();

    assert_eq!([height, width], [8, 8]);
}

#[test]
fn reveal_test_corner() {
    let mut board = Board::default(8, 8, 63);

    let mut clone_1 = board.clone();
    let mut clone_2 = clone_1.clone();
    let mut clone_3 = clone_2.clone();

    clone_1.reveal_tile(Position { x: 0, y: 0 });
    clone_2.reveal_tile(Position { x: 7, y: 0 });
    clone_3.reveal_tile(Position { x: 0, y: 7 });
    board.reveal_tile(Position { x: 7, y: 7 });

    let number_0_0 = clone_1.rows[0][0].number;
    let number_7_0 = clone_2.rows[0][7].number;
    let number_0_7 = clone_3.rows[7][0].number;
    let number_7_7 = board.rows[7][7].number;

    assert_eq!([number_0_0, number_0_7, number_7_0, number_7_7], [3; 4]);
}

#[test]
fn reveal_test_edge() {
    let mut board = Board::default(8, 8, 63);

    let x = generate_random_number(0, 8);
    let mut y = 1;
    if x > 0 && x < 7 {
        y = 0;
    }

    let mut clone_0 = board.clone();

    board.reveal_tile(Position { x, y });
    clone_0.reveal_tile(Position { x: y, y: x });

    let number_0 = board.rows[y][x].number;
    let number_1 = board.rows[y][x].number;

    assert_eq!([number_0, number_1], [5; 2]);
}

#[test]
fn reveal_test_middle() {
    let mut board = Board::default(8, 8, 63);

    let x = generate_random_number(1, 7);
    let y = generate_random_number(1, 7);

    board.reveal_tile(Position { x, y });

    println!("{} {}", y, x);

    let number = board.rows[y][x].number;

    assert_eq!(number, 8);
}
