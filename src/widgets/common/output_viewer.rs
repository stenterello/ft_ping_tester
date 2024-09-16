use crate::utils::thread::Thread;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        block::{BorderType, Title},
        Block, Paragraph, Widget, Wrap,
    },
};

#[derive(Debug, Clone)]
pub enum TextType {
    Standard(Vec<String>),
    Formatted(Vec<Vec<(bool, u8)>>),
}

#[derive(Debug)]
pub struct OutputViewer {
    thread: Thread,
    text_to_display: TextType,
    error_to_display: TextType,
}

impl OutputViewer {
    pub fn new(path: &str, name: &str) -> Self {
        OutputViewer {
            thread: Thread::new(path.into(), name.into()),
            text_to_display: TextType::Standard(Vec::default()),
            error_to_display: TextType::Standard(Vec::default()),
        }
    }

    pub fn start_process(&mut self, args: Vec<String>) -> () {
        self.thread.start(args);
    }

    pub fn get_exit_status(&self) -> (Option<i32>, Option<String>) {
        self.thread.get_exit()
    }

    pub fn get_output(&self) -> Vec<String> {
        self.thread.get_output()
    }

    pub fn get_error_output(&self) -> Vec<String> {
        self.thread.get_error_output()
    }

    pub fn is_running(&self) -> bool {
        self.thread.is_running()
    }

    pub fn get_error_message(&mut self) -> String {
        match self.get_error_output().len() {
            0 => match self.thread.get_exit() {
                (None, Some(err)) => err,
                _ => String::default(),
            },
            _ => self.get_error_output().join("\n"),
        }
    }

    pub fn set_text_to_display(&mut self, display: TextType) -> () {
        self.text_to_display = display;
    }

    pub fn set_error_to_display(&mut self, display: TextType) -> () {
        self.error_to_display = display;
    }

    pub fn clear_buffers(&mut self) {
        self.text_to_display = TextType::Standard(Vec::default());
        self.error_to_display = TextType::Standard(Vec::default());
        self.thread.clear_buffers();
    }

    fn retranslate(spans: &mut Vec<Span>) {
        let pattern: [u8; 4] = [b'p', b'i', b'n', b'g'];
        let mut idx: usize = 0;
        let mut saved_idx: Vec<usize> = Vec::default();
        let mut iter = spans.iter().enumerate();
        loop {
            match iter.next() {
                Some(c) => {
                    if c.1.content.as_bytes()[0] == pattern[idx] {
                        idx += 1;
                        if idx == pattern.len() {
                            saved_idx.push(c.0 - (pattern.len() - 1));
                            idx = 0;
                        }
                    } else {
                        idx = 0;
                    }
                }
                None => break,
            }
        }

        for index in saved_idx.iter().rev() {
            if *index > 2 && index < &spans.len() {
                if spans.get(index - 1).unwrap().content.as_bytes()[0] == b'_' {
                    if spans.get(index - 2).unwrap().content.as_bytes()[0] == b't' {
                        if spans.get(index - 3).unwrap().content.as_bytes()[0] == b'f' {}
                    }
                } else {
                    spans.insert(*index, Span::from("f").yellow());
                    spans.insert(*index + 1, Span::from("t").yellow());
                    spans.insert(*index + 2, Span::from("_").yellow());
                }
            } else {
                spans.insert(*index, Span::from("f").yellow());
                spans.insert(*index + 1, Span::from("t").yellow());
                spans.insert(*index + 2, Span::from("_").yellow());
            }
        }
    }
}

impl Widget for &OutputViewer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut t: String = String::from(" ");
        t.push_str((String::from(self.thread.name.clone()) + " ").as_str());
        let title = Title::from(t.as_str().bold().yellow());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        match &self.text_to_display {
            TextType::Standard(s) => {
                if !s.is_empty() {
                    let lines = s.join("\n");
                    Paragraph::new(lines.as_str())
                        .block(block.clone())
                        .wrap(Wrap { trim: true })
                        .style(
                            Style::default()
                                .bg(Color::Rgb(46, 52, 64))
                                .fg(Color::White)
                                .bold(),
                        )
                        .render(area, buf);
                }
            }
            TextType::Formatted(s) => {
                let mut lines: Vec<Line> = vec![];
                for string in s {
                    let mut spans: Vec<Span> = vec![];
                    for to_format in string {
                        match to_format {
                            (true, c) => spans.push(Span::styled(
                                char::from_u32(*c as u32).unwrap().to_string(),
                                Style::default(),
                            )),
                            (false, c) => spans.push(
                                Span::styled(
                                    char::from_u32(*c as u32).unwrap().to_string(),
                                    Style::default().fg(Color::Red),
                                )
                                .bold(),
                            ),
                        }
                    }
                    OutputViewer::retranslate(&mut spans);
                    lines.push(Line::from(spans));
                }

                Paragraph::new(Text::from(lines))
                    .block(block.clone())
                    .wrap(Wrap { trim: true })
                    .style(Style::default().bg(Color::Rgb(46, 52, 64)).fg(Color::White))
                    .render(area, buf);
            }
        }

        match &self.error_to_display {
            TextType::Standard(s) => {
                if !s.is_empty() {
                    let lines = s.join("\n");
                    Paragraph::new(lines.as_str())
                        .block(block.clone())
                        .wrap(Wrap { trim: true })
                        .style(Style::default().bg(Color::Rgb(46, 52, 64)).fg(Color::White))
                        .render(area, buf);
                }
            }

            TextType::Formatted(s) => {
                let mut lines: Vec<Line> = vec![];
                for string in s {
                    let mut spans: Vec<Span> = vec![];
                    for to_format in string {
                        match to_format {
                            (true, c) => spans.push(Span::styled(
                                char::from_u32(*c as u32).unwrap().to_string(),
                                Style::default(),
                            )),
                            (false, c) => spans.push(Span::styled(
                                char::from_u32(*c as u32).unwrap().to_string(),
                                Style::default().fg(Color::Red).bold(),
                            )),
                        }
                    }
                    OutputViewer::retranslate(&mut spans);
                    lines.push(Line::from(spans));
                }
                Paragraph::new(Text::from(lines))
                    .block(block)
                    .wrap(Wrap { trim: true })
                    .style(Style::default().bg(Color::Rgb(46, 52, 64)).fg(Color::White))
                    .render(area, buf);
            }
        }
    }
}
