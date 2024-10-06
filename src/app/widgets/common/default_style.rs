use ratatui::widgets::block::Block;
use ratatui::{
    widgets::block::{BorderType, Title},
    layout::Alignment,
    style::{Color, Style},
};

pub struct DefaultStyle;

impl DefaultStyle {
    pub fn block(title: Title) -> Block {
        Block::bordered()
            .style(Style::default().fg(Color::Yellow))
            .title(title)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
    }

    pub fn style() -> Style {
        Style::default()
            .bg(Color::Rgb(40, 44, 52))
            .fg(Color::White)
    }
}
