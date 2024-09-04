use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{
        block::{BorderType, Title},
        Block, Paragraph, Widget,
    },
};

#[derive(Debug, Default)]
pub struct RecompilingNotice;

impl Widget for &RecompilingNotice {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Infos ".bold().yellow());
        let block = Block::bordered()
            .style(Style::default().fg(Color::Yellow))
            .title(title)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        Paragraph::new(Text::from("Recompiling..."))
            .block(block)
            .style(Style::default().fg(Color::White))
            .render(area, buf);
    }
}
