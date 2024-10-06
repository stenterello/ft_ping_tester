use ratatui::{
    buffer::Buffer,
    layout::{Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{block::Title, Paragraph, Widget},
};

use super::super::common::default_style::DefaultStyle;

#[derive(Debug, Default)]
pub struct InfoWidget;

impl Widget for &InfoWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Infos ".bold().yellow());

        let text = Text::from(vec![
            Line::from(" Made with Ratatui "),
            Line::from(""),
            Line::from(" https://docs.rs/ratatui/latest/ratatui/index.html "),
        ]);

        Paragraph::new(text)
            .block(DefaultStyle::block(title))
            .style(DefaultStyle::style())
            .render(area, buf);
    }
}
