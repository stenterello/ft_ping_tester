use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::utils::thread::Thread;

#[derive(Debug)]
pub struct OutputViewer {
    thread: Thread,
}

impl OutputViewer {
    pub fn new(path: &str) -> Self {
        OutputViewer {
            thread: Thread::new(path.into()),
        }
    }

    pub fn start_process(&mut self) -> () {
        self.thread.start(vec!["localhost".into()]);
    }

    pub fn get_exit_status(&self) -> (Option<i32>, Option<String>) {
        self.thread.get_exit()
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
                _ => String::default()
            },
            _ => self.get_error_output().join("\n")
        }
    }
}

impl Widget for &OutputViewer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = self.thread.get_output().join("\n");
        let block = Block::bordered();
        Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: true })
            .style(Style::default().bg(Color::Rgb(46, 52, 64)).fg(Color::White))
            .render(area, buf);
    }
}
