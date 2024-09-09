use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        block::{Block, BorderType, Title},
        Paragraph, Widget, Wrap,
    },
};

#[derive(Debug, Default)]
pub struct MessageWidget {
    running_test: bool,
    arguments: Vec<String>,
}

impl MessageWidget {
    pub fn new() -> Self {
        MessageWidget {
            running_test: false,
            arguments: Vec::default(),
        }
    }

    pub fn set_running(&mut self, value: bool) -> () {
        self.running_test = value;
    }

    pub fn set_arguments(&mut self, args: Vec<String>) -> () {
        self.arguments = args;
    }
}

impl Widget for &MessageWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Status ".bold().yellow());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Left))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        let message = if self.running_test {
            let mut tmp = String::from("Running {ping}");
            for arg in &self.arguments {
                tmp.push_str(" ");
                tmp.push_str(arg.as_str());
            }
            tmp.push_str("...");
            tmp
        } else {
            let mut tmp = String::from("Last run: |{ping}");
            for arg in &self.arguments {
                tmp.push_str(" ");
                tmp.push_str(arg.as_str());
            }
            tmp.push_str("|.");
            tmp
        };

        Paragraph::new(message)
            .block(block)
            .wrap(Wrap { trim: true })
            .style(Style::default().bg(Color::Rgb(46, 52, 64)).fg(Color::White))
            .render(area, buf);
    }
}
