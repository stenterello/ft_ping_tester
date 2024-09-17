use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Widget},
};

#[derive(Debug, Default, Clone)]
pub struct CommandsWidget {
    commands_string: String
}

impl CommandsWidget {
    pub fn new(commands_string: String) -> Self {
        Self {
            commands_string
        }
    }
}

impl Widget for &CommandsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.commands_string.clone())
            .black()
            .on_yellow()
            .render(area, buf);
    }
}
