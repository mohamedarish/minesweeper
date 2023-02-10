use druid::{widget::Label, Widget};

pub(crate) fn build_board() -> impl Widget<()> {
    Label::new("Minesweeper")
}
