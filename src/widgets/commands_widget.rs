use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Widget},
};

const COMMANDS_INFO: &str = " q: Go back | space: Next test ";

#[derive(Debug, Default)]
pub struct CommandsWidget;

impl Widget for &CommandsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(COMMANDS_INFO)
            .black()
            .on_yellow()
            .render(area, buf);
    }
}
