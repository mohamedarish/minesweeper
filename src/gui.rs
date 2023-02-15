use druid::{
    widget::{Flex, Label},
    Color, LocalizedString, Widget, WidgetExt,
};

use crate::{
    game::{Click, Game},
    tiles::Tile,
};

fn make_tile(position: Click) -> impl Widget<Game> {
    let text = LocalizedString::new("hello-counter").with_arg("count", move |data: &Game, _env| {
        let Tile(record) = data.board.0[position.y][position.x][1];

        // println!("{record}");

        format!("{record}").into()
    });

    Label::new(text)
        .padding(5.)
        .on_click(move |_ctx, data: &mut Game, _env| {
            data.board.reveal_tile(position.x, position.y);
        })
        .border(Color::rgb(99., 00., 00.), 1.)
}

pub(crate) fn build_board() -> impl Widget<Game> {
    let mut game_board = Flex::column();

    for i in 0..8 {
        let mut new_row = Flex::row();
        for j in 0..8 {
            new_row.add_child(make_tile(Click { x: i, y: j }));
        }

        game_board.add_child(new_row);
    }

    game_board
}
