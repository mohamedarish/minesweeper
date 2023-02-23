use crate::generate_random_number;

#[derive(PartialEq, Clone, Copy)]
pub enum RevealStatus {
    NotRevealed,
    Flag,
    Revealed,
}

impl Default for RevealStatus {
    fn default() -> Self {
        Self::NotRevealed
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct Cell {
    pub is_mine: bool,
    pub status: RevealStatus,
    pub number: usize,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Board {
    pub rows: Vec<Vec<Cell>>,
    pub number_of_mines: usize,
}

#[derive(PartialEq, Clone)]
pub enum TileStatus {
    Number,
    Mine,
}

impl Board {
    pub fn default(width: usize, height: usize, number_of_mines: usize) -> Self {
        let board = vec![vec![Cell::default(); width]; height];
        Board {
            rows: board,
            number_of_mines,
        }
    }
}

impl Board {
    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    fn is_new_board(&self) -> bool {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.is_revealed(Position { x, y }) {
                    return false;
                }
            }
        }

        true
    }

    pub fn set_flag(&mut self, position: Position) {
        self.rows[position.y][position.x].status = RevealStatus::Flag;
    }

    pub fn unset_flag(&mut self, position: Position) {
        self.rows[position.y][position.x].status = RevealStatus::NotRevealed;
    }

    fn db_click(&mut self, position: Position) -> bool {
        self.dbc_rows(position)
    }

    fn dbc_rows(&mut self, position: Position) -> bool {
        if position.y == 0 {
            for y in 0..2 {
                let status = self.dbc_cols(position, y);

                if !status {
                    return false;
                }
            }
        } else if position.y == self.height() - 1 {
            for y in (position.y - 1)..self.height() {
                let status = self.dbc_cols(position, y);

                if !status {
                    return false;
                }
            }
        } else {
            for y in (position.y - 1)..(position.y + 2) {
                let status = self.dbc_cols(position, y);

                if !status {
                    return false;
                }
            }
        }

        true
    }

    fn dbc_cols(&mut self, position: Position, y: usize) -> bool {
        if position.x == 0 {
            for x in 0..2 {
                if !self.is_revealed(position) && !self.is_flag(position) {
                    self.reveal_tile(Position { x, y });

                    if self.rows[y][x].is_mine {
                        return false;
                    }
                }
            }
        } else if position.x == self.width() - 1 {
            for x in (position.x - 1)..self.width() {
                if !self.is_revealed(position) && !self.is_flag(position) {
                    self.reveal_tile(Position { x, y });
                }

                if self.rows[y][x].is_mine {
                    return false;
                }
            }
        } else {
            for x in (position.x - 1)..(position.x + 2) {
                if !self.is_revealed(position) && !self.is_flag(position) {
                    self.reveal_tile(Position { x, y });
                }

                if self.rows[y][x].is_mine {
                    return false;
                }
            }
        }
        true
    }

    pub fn reveal_tile(&mut self, click: Position) -> TileStatus {
        if self.is_new_board() {
            self.generate_mines(click);
        }

        if self.rows[click.y][click.x].is_mine {
            return TileStatus::Mine;
        }

        if self.rows[click.y][click.x].status == RevealStatus::Revealed {
            if self.nearby_flags(click) == self.tile_number(click) && !self.db_click(click) {
                return TileStatus::Mine;
            }
        } else {
            self.rows[click.y][click.x].status = RevealStatus::Revealed;

            if self.tile_number(click) == 0 {
                self.reveal_cols(click);
            }
        }

        TileStatus::Number
    }

    fn reveal_cols(&mut self, click: Position) {
        if click.x == 0 {
            for x in 0..2 {
                self.reveal_row(click, x);
            }
        } else if click.x == self.width() - 1 {
            for x in (click.x - 1)..(self.width()) {
                self.reveal_row(click, x);
            }
        } else {
            for x in (click.x - 1)..(click.x + 2) {
                self.reveal_row(click, x);
            }
        }
    }

    fn reveal_row(&mut self, click: Position, x: usize) {
        if click.y == 0 {
            for y in 0..2 {
                if self.is_revealed(Position { x, y }) {
                    continue;
                }

                self.rows[y][x].status = RevealStatus::Revealed;

                if self.tile_number(Position { x, y }) == 0 {
                    self.reveal_cols(Position { x, y });
                }
            }
        } else if click.y == self.height() - 1 {
            for y in (click.y - 1)..self.height() {
                if self.is_revealed(Position { x, y }) {
                    continue;
                }

                self.rows[y][x].status = RevealStatus::Revealed;

                if self.tile_number(Position { x, y }) == 0 {
                    self.reveal_cols(Position { x, y });
                }
            }
        } else {
            for y in (click.y - 1)..(click.y + 2) {
                if self.is_revealed(Position { x, y }) {
                    continue;
                }

                self.rows[y][x].status = RevealStatus::Revealed;

                if self.tile_number(Position { x, y }) == 0 {
                    self.reveal_cols(Position { x, y });
                }
            }
        }
    }

    fn nearby_flags(&self, position: Position) -> usize {
        self.nf_rows(position)
    }

    fn nf_rows(&self, position: Position) -> usize {
        let mut count = 0;
        if position.y == 0 {
            for y in 0..2 {
                count += self.nf_cols(position, y);
            }
        } else if position.y == self.height() - 1 {
            for y in (position.y - 1)..self.height() {
                count += self.nf_cols(position, y);
            }
        } else {
            for y in (position.y - 1)..(position.y + 2) {
                count += self.nf_cols(position, y);
            }
        }

        count
    }

    fn nf_cols(&self, position: Position, y: usize) -> usize {
        let mut count = 0;

        if position.x == 0 {
            for x in 0..2 {
                if self.is_flag(Position { x, y }) {
                    count += 1;
                }
            }
        } else if position.x == self.width() - 1 {
            for x in (position.x - 1)..self.width() {
                if self.is_flag(Position { x, y }) {
                    count += 1;
                }
            }
        } else {
            for x in (position.x - 1)..(position.x + 2) {
                if self.is_flag(Position { x, y }) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn tile_number(&self, position: Position) -> usize {
        self.rows[position.y][position.x].number
    }

    pub fn generate_mines(&mut self, initial_click: Position) {
        if self.number_of_mines >= self.height() * self.width() {
            panic!("Cannot generate more mines than the number of tiles!!");
        }

        for _ in 0..self.number_of_mines {
            self.gen_mine(initial_click);
        }

        self.set_tile_numbers();
    }

    fn nm_cols(&self, position: Position, y: usize) -> usize {
        let mut mines = 0;

        if position.x == 0 {
            for x in 0..2 {
                if self.rows[y][x].is_mine {
                    mines += 1;
                }
            }
        } else if position.x == self.width() - 1 {
            for x in (position.x - 1)..(self.width()) {
                if self.rows[y][x].is_mine {
                    mines += 1;
                }
            }
        } else {
            for x in (position.x - 1)..(position.x + 2) {
                if self.rows[y][x].is_mine {
                    mines += 1;
                }
            }
        }

        mines
    }

    fn nm_rows(&self, position: Position) -> usize {
        let mut mines = 0;

        if position.y == 0 {
            for y in 0..2 {
                mines += self.nm_cols(position, y);
            }
        } else if position.y == self.height() - 1 {
            for y in (position.y - 1)..(self.height()) {
                mines += self.nm_cols(position, y);
            }
        } else {
            for y in (position.y - 1)..(position.y + 2) {
                mines += self.nm_cols(position, y);
            }
        }

        mines
    }

    fn nearby_mines(&self, position: Position) -> usize {
        self.nm_rows(position)
    }

    fn set_tile_numbers(&mut self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.rows[y][x].is_mine {
                    continue;
                }

                self.rows[y][x].number = self.nearby_mines(Position { x, y });
            }
        }
    }

    pub fn is_revealed(&self, position: Position) -> bool {
        if self.rows[position.y][position.x].status == RevealStatus::Revealed {
            return true;
        }
        false
    }

    pub fn is_flag(&self, position: Position) -> bool {
        if self.rows[position.y][position.x].status == RevealStatus::Flag {
            return true;
        }
        false
    }

    fn gen_mine(&mut self, initial_click: Position) {
        loop {
            let x = generate_random_number(0, self.width());
            let y = generate_random_number(0, self.height());

            if x == initial_click.x && y == initial_click.y {
                continue;
            }

            if self.rows[y][x].is_mine {
                continue;
            }

            self.rows[y][x].is_mine = true;

            return;
        }
    }
}
