use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Margin, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        block::{BorderType, Title},
        Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget,
        Wrap,
    },
};

use crate::utils::thread::Thread;

#[derive(Debug)]
pub struct RecompilingNotice {
    thread: Thread,
    location: String,
    vertical_scroll: usize,
}

impl RecompilingNotice {
    pub fn new(path: String) -> Self {
        RecompilingNotice {
            thread: Thread::new("make".into()),
            location: path,
            vertical_scroll: usize::default(),
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
        let error_title = Title::from(" Error! ".bold().red());
        let mut block = Block::bordered()
            .style(Style::default().fg(Color::Yellow))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        if self.thread.is_running() {
            block = block.title(recompiling_title);
        } else if self.thread.get_exit_status() == 0 {
            block = block.title(done_title);
        } else {
            block = block.title(error_title);
        }

        let text = self.thread.get_output().join("\n");

        let mut p = self.vertical_scroll;

        if text.clone().lines().count() > area.height as usize {
            p += text.clone().lines().count() - area.height as usize;
        }

        Paragraph::new(text.clone())
            .block(block.clone())
            .wrap(Wrap { trim: true })
            .scroll((p as u16, 0))
            .style(Style::default().fg(Color::White))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("^"))
            .end_symbol(Some("v"));

        let mut scrollbar_state = ScrollbarState::new(text.lines().count()).position(p);

        scrollbar.render(
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            buf,
            &mut scrollbar_state,
        );
    }
}
