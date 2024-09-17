use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::{Block, BorderType, Title},
        Paragraph, Widget, Wrap,
    },
};

#[derive(Debug, Default)]
pub struct MessageWidget {
    running_test: bool,
    arguments: String,
    has_errors: bool,
}

impl MessageWidget {
    pub fn new() -> Self {
        MessageWidget {
            running_test: false,
            arguments: String::default(),
            has_errors: false,
        }
    }

    pub fn arguments(&self) -> &str {
        &self.arguments
    }
    
    pub fn errors(&self) -> bool {
        self.has_errors
    }

    pub fn set_running(&mut self, value: bool) -> () {
        self.running_test = value;
    }

    pub fn set_arguments(&mut self, args: String) -> () {
        self.arguments = args;
    }

    pub fn set_errors(&mut self, val: bool) -> () {
        self.has_errors = val;
    }
}

impl Widget for &MessageWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Status ".bold().yellow());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Left))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        let message: Vec<Line> = {
            let mut ret: Vec<Line> = Vec::default();
            let mut tmp = Line::from(format!(
                "{} {{ping}} ",
                if self.running_test {
                    "Running"
                } else {
                    "Last run"
                }
            ));
            tmp.push_span(Span::from(self.arguments.as_str()));
            tmp.push_span(Span::from("."));

            ret.push(tmp);
            ret.push(Line::default());

            if !self.running_test {
                ret.push(Line::from(Span::from(format!(
                    "Test Result: {}",
                    if self.has_errors {
                        "🔴 ERROR!"
                    } else {
                        "🟢 CORRECT!"
                    }
                ))));
            }
            ret
        };

        Paragraph::new(message)
            .block(block)
            .wrap(Wrap { trim: true })
            .style(Style::default().bg(Color::Rgb(46, 52, 64)).fg(Color::White))
            .render(area, buf);
    }
}
