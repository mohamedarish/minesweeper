use druid::{
    widget::{Flex, Label},
    Widget, WidgetExt,
};

use crate::{
    game::Game,
    tiles::{Board, Tile},
};

fn make_tile(number: i32) -> impl Widget<Game> {
    let mut character_to_display = String::new();

    match number.cmp(&0) {
        std::cmp::Ordering::Less => {
            if number >= -1 {
                character_to_display += &String::from("💣");
            } else {
                character_to_display += &String::from("🦒");
            }
        }
        std::cmp::Ordering::Equal => {
            character_to_display += &String::from("🚮");
        }
        std::cmp::Ordering::Greater => character_to_display += &String::from(&format!("{number}")),
    }

    Label::new(character_to_display.to_string()).padding(5.)
}

pub(crate) fn build_board(board: &Board) -> impl Widget<Game> {
    let mut game_board = Flex::column();

    for i in board.0 {
        let mut new_row = Flex::row();
        for j in i {
            let Tile(number) = j[0];

            new_row.add_child(make_tile(number));
        }

        game_board.add_child(new_row);
    }

    game_board
}
