use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Color},
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
