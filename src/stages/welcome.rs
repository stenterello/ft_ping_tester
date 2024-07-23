


use ratatui::{
    widgets::{
        Widget,
        Paragraph,
        Block,
        block::{Position, Title},
    },
    text::{Line, Text},
    symbols::border,
    layout::{Alignment, Rect},
    buffer::Buffer,
    style::Stylize,
    Frame,
};

#[derive(Debug, Default)]
pub struct WelcomeWidget;

impl Widget for &WelcomeWidget {
    fn  render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" ft_ping_tester ".bold());
        let description = Text::from(vec![Line::from(vec![
            "This is the start".into(),
        ])]);
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);

        Paragraph::new(description)
            .block(block)
            .render(area, buf);
    }
}
