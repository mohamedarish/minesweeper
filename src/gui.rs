use crate::{board::Position, game::Minesweeper};
use iced::{
    alignment, theme,
    widget::{container, Button, Column, Row, Text},
    Length, Sandbox,
};

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
                let button_theme;

                let button_text = match self.board.rows[y][x].status {
                    crate::board::RevealStatus::NotRevealed => {
                        button_theme = theme::Button::Secondary;
                        Text::new(" ")
                    }
                    crate::board::RevealStatus::Flag => {
                        button_theme = theme::Button::Secondary;
                        Text::new("F")
                    }
                    crate::board::RevealStatus::Revealed => {
                        button_theme = theme::Button::Primary;
                        Text::new(format!("{}", self.board.tile_number(Position { x, y })))
                    }
                };

                row = row.push(
                    Button::new(
                        button_text
                            .horizontal_alignment(alignment::Horizontal::Center)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .on_press(ClickType::Left(x, y))
                    .style(button_theme)
                    .height(80.)
                    .width(80.),
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
