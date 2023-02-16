use iced::{
    theme,
    widget::{container, Button, Column, Row, Text},
    Sandbox,
};

use crate::{game::Game, tiles::Tile};

#[derive(Debug, Clone, Copy)]
pub enum CType {
    Right,
    Left,
    Double,
}

impl Sandbox for Game {
    type Message = CType;

    fn new() -> Self {
        Game::default()
    }

    fn title(&self) -> String {
        String::from("minesweeper")
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut column = Column::new();

        for i in 0..8 {
            let mut row = Row::new();
            for j in 0..8 {
                let Tile(number) = self.board.0[j][i][1];

                let button_text = Text::new(format!("{number}"));

                let new_button = Button::new(button_text)
                    .style(theme::Button::Secondary)
                    .padding(30);

                row = row.push(new_button);
            }

            column = column.push(row.align_items(iced::Alignment::Center));
        }

        container(column.align_items(iced::Alignment::Center))
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
