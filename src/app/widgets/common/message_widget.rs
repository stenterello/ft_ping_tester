use crate::app::widgets::traits::thread_stringpuller::ExitResult;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{
        block::{Block, BorderType, Title},
        Row, Table, Widget,
    },
};

#[derive(Debug, Default)]
pub struct MessageWidget {
    running_test: bool,
    arguments: String,
    has_errors: bool,
    codes: (i32, i32),
}

impl MessageWidget {
    pub fn new() -> Self {
        MessageWidget {
            running_test: false,
            arguments: String::default(),
            has_errors: false,
            codes: (-1, -1),
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

    pub fn set_codes(&mut self, ft_exit: ExitResult, ping_exit: ExitResult) -> () {
        match ft_exit {
            ExitResult::Correct(code) | ExitResult::Error(code, _) => self.codes.0 = code,
            ExitResult::None => self.codes.0 = 127,
        };
        match ping_exit {
            ExitResult::Correct(code) | ExitResult::Error(code, _) => self.codes.1 = code,
            ExitResult::None => self.codes.0 = 127,
        };

        if self.codes.0 != self.codes.1 {
            self.set_errors(true);
        }
    }
}

impl Widget for &MessageWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Status ".bold().yellow());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Left))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        Table::new(
            [
                Row::new(vec![
                    "ft_ping".to_string(),
                    self.codes.0.to_string(),
                    if self.errors() {
                        "🔴".to_string()
                    } else {
                        "🟢".to_string()
                    },
                ])
                .style(Style::default().white()),
                Row::new(vec!["ping".to_string(), self.codes.1.to_string()])
                    .style(Style::default().white()),
            ],
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ],
        )
        .white()
        .block(block)
        .style(Style::default().white())
        .header(
            Row::new(vec![
                "Exec".to_string(),
                "Exit code".to_string(),
                "Result".to_string(),
            ])
            .style(Style::default().white().bold()),
        )
        .footer(if self.codes.0 != 127 {
            Row::new(vec![
                "Arguments: ".to_string(),
                self.arguments().to_string(),
            ])
        } else {
            Row::new(vec![
                Span::from("Arguments: ".to_string()),
                Span::from(self.arguments().to_string()),
                Span::from("Segfault on ft_ping".to_string()).bold().red(),
            ])
        })
        .render(area, buf);
    }
}
