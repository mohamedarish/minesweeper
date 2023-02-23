use iced::{
    widget::{container, Button, Column, Row, Text},
    Length, Sandbox,
};

use crate::{board::Position, game::Minesweeper};

#[derive(Debug, Clone)]
pub enum ClickType {
    Right(usize, usize),
    Left(usize, usize),
    Double(usize, usize),
}

impl Sandbox for Minesweeper {
    type Message = ClickType;

    fn new() -> Self {
        Self::default(8, 8, 10)
    }

    fn title(&self) -> String {
        String::from("minesweeper")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ClickType::Left(x, y) => {
                self.board.reveal_tile(Position { x, y });
            }
            ClickType::Right(x, y) => {
                self.board.set_flag(Position { x, y });
            }
            ClickType::Double(_, _) => todo!(),
        }
        self.result_decided();
        println!("{:?}", self.result);
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut col = Column::new();

        for y in 0..self.board.height() {
            let mut row = Row::new();
            for x in 0..self.board.width() {
                row = row.push(
                    Button::new(Text::new(format!(
                        "{}",
                        self.board.tile_number(Position { x, y })
                    )))
                    .padding(25)
                    .on_press(ClickType::Left(x, y)),
                );
            }
            col = col.push(row);
        }

        container(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::default()
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn run(settings: iced::Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }
}
