use crate::{
    board::{Position, TileStatus},
    game::{GameStatus, Minesweeper},
};
use iced::{
    alignment,
    theme::Button::{Destructive, Primary},
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
                let res = self.board.reveal_tile(Position { x, y });

                if res == TileStatus::Mine {
                    self.result = GameStatus::Loss;
                }
            }
            ClickType::Right(x, y) => {
                self.board.set_flag(Position { x, y });
            }
            ClickType::Double(_, _) => todo!(),
        }
        self.result_decided();
        self.stop_timer();
        println!("{:?}", self.result);
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let mut col = Column::new();

        let mut timer_row = Row::new();

        timer_row = timer_row.push(Text::new(format!("{}", self.timer.elapsed_ms())));

        col = col.push(timer_row);

        for y in 0..self.board.height() {
            let mut row = Row::new();
            for x in 0..self.board.width() {
                let button_theme;

                let button_text = match self.board.rows[y][x].status {
                    crate::board::RevealStatus::NotRevealed => {
                        button_theme = Primary;
                        Text::new(" ")
                    }
                    crate::board::RevealStatus::Flag => {
                        button_theme = Primary;
                        Text::new("F")
                    }
                    crate::board::RevealStatus::Revealed => {
                        if self.board.rows[y][x].is_mine {
                            button_theme = Destructive;
                            Text::new("💣")
                        } else {
                            button_theme = Primary;
                            Text::new(format!("{}", self.board.tile_number(Position { x, y })))
                        }
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

        match self.result {
            GameStatus::Victory => {
                let mut col = Column::new().push(
                    Text::new("V for victory")
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .vertical_alignment(alignment::Vertical::Bottom)
                        .size(20.)
                        .height(400.)
                        .width(800.),
                );

                col = col.push(
                    Text::new(format!(
                        "{}:{}:{}",
                        self.timer
                            .elapsed_ms()
                            .checked_div(3600000)
                            .expect("Cannot divide"),
                        self.timer
                            .elapsed_ms()
                            .checked_div(60000)
                            .expect("Cannot divide"),
                        self.timer
                            .elapsed_ms()
                            .checked_div(100)
                            .expect("Cannot divide")
                    ))
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Top)
                    .width(800.)
                    .height(400.),
                );
                container(col)
                    .center_x()
                    .center_y()
                    .width(800.)
                    .height(800.)
                    .into()
            }
            GameStatus::Loss => {
                let mut col = Column::new().push(
                    Text::new("try again next time")
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .vertical_alignment(alignment::Vertical::Bottom)
                        .size(20.)
                        .height(400.)
                        .width(800.),
                );

                col = col.push(
                    Text::new(format!("{} ms", self.timer.elapsed_ms()))
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .vertical_alignment(alignment::Vertical::Top)
                        .height(400.)
                        .width(800.),
                );

                container(col)
                    .center_x()
                    .center_y()
                    .width(800.)
                    .height(800.)
                    .into()
            }
            GameStatus::NotDecided => container(col)
                .width(Length::Fixed(800.))
                .height(Length::Fixed(800.))
                .center_x()
                .center_y()
                .into(),
        }
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
