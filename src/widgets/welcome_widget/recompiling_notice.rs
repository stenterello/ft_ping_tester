use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        block::{BorderType, Title},
        Block, Paragraph, Widget, Wrap,
    },
};

use crate::utils::thread::Thread;

#[derive(Debug)]
pub struct RecompilingNotice {
    thread: Thread,
    location: String,
}

impl RecompilingNotice {
    pub fn new(path: String) -> Self {
        RecompilingNotice {
            thread: Thread::new("make".into()),
            location: path,
        }
    }

    pub fn start(&mut self) {
        self.thread.start(vec!["-C".into(), self.location.clone()]);
    }
}

impl Widget for &RecompilingNotice {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let recompiling_title = Title::from(" Recompiling... ".bold().yellow());
        let done_title = Title::from(" Done! ".bold().green());
        let mut block = Block::bordered()
            .style(Style::default().fg(Color::Yellow))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        if self.thread.is_running() {
            block = block.title(recompiling_title);
        } else {
            block = block.title(done_title);
        }

        let text = self.thread.get_output().join("\n");

        Paragraph::new(text)
            .block(block.clone())
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .render(area, buf);
    }
}
