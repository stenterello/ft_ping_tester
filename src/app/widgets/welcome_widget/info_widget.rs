use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{block::BorderType, block::Title, Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct InfoWidget;

impl Widget for &InfoWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Infos ".bold().yellow());
        let block = Block::bordered()
            .style(Style::default().fg(Color::Yellow))
            .title(title)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = Text::from(vec![
            Line::from(" Made with Ratatui "),
            Line::from(""),
            Line::from(" https://docs.rs/ratatui/latest/ratatui/index.html "),
        ]);

        Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(Color::White))
            .render(area, buf);
    }
}