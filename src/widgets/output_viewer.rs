use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::{BorderType, Title},
        Block, Paragraph, Widget, Wrap,
    },
};

use crate::utils::thread::Thread;

#[derive(Debug)]
pub struct OutputViewer<'a> {
    thread: Thread,
    to_display: Vec<Line<'a>>,
}

impl<'a> OutputViewer<'a> {
    pub fn new(path: &str, name: &str) -> Self {
        OutputViewer {
            thread: Thread::new(path.into(), name.into()),
            to_display: Vec::default(),
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

    pub fn set_text_to_display(&mut self, display: Vec<Line<'a>>) -> () {
        self.to_display = display;
    }

    pub fn clear_text_to_display(&mut self) {
        self.to_display = Vec::default();
    }
}

impl<'a> Widget for &OutputViewer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut t: String = String::from(" ");
        t.push_str((String::from(self.thread.name.clone()) + " ").as_str());
        let title = Title::from(t.as_str().bold().yellow());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);
        Paragraph::new(self.to_display.clone())
            .block(block)
            .wrap(Wrap { trim: true })
            .style(Style::default().bg(Color::Rgb(46, 52, 64)).fg(Color::White))
            .render(area, buf);
    }
}

impl<'a> Widget for &mut OutputViewer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut t: String = String::from(" ");
        t.push_str((String::from(self.thread.name.clone()) + " ").as_str());
        let title = Title::from(t.as_str().bold().yellow());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);
        Paragraph::new(self.to_display.clone())
            .block(block)
            .wrap(Wrap { trim: true })
            .style(Style::default().bg(Color::Rgb(46, 52, 64)).fg(Color::White))
            .render(area, buf);
    }
}
