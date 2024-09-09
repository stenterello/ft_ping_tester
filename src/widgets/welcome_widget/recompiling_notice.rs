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
use std::cell::RefCell;

use crate::utils::thread::Thread;

#[derive(Debug)]
pub struct RecompilingNotice {
    thread: Thread,
    location: String,
    vertical_scroll: usize,
    widget_height: RefCell<usize>,
}

impl RecompilingNotice {
    pub fn new(path: String) -> Self {
        RecompilingNotice {
            thread: Thread::new("".into(), "make".into()),
            location: path,
            vertical_scroll: usize::default(),
            widget_height: RefCell::new(usize::default()),
        }
    }

    pub fn start(&mut self) {
        self.thread.start(vec!["-C".into(), self.location.clone()]);
    }

    pub fn move_up(&mut self) {
        if self.vertical_scroll > 0 {
            self.vertical_scroll -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.thread.get_output().join("\n").lines().count()
            > (self.widget_height.borrow().to_owned() + self.vertical_scroll - 5)
        {
            self.vertical_scroll += 1;
        }
    }

    pub fn clean_output(&mut self) {
        self.thread.clean_output();
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
        } else if self.thread.get_exit().0 == Some(0) {
            block = block.title(done_title);
        } else {
            block = block.title(error_title);
        }

        let mut text = self.thread.get_output().join("\n");
        text.push_str(self.thread.get_error_output().join("\n").as_str());
        let mut h = self.widget_height.borrow_mut();
        *h = area.height as usize;

        Paragraph::new(text.clone())
            .block(block.clone())
            .wrap(Wrap { trim: true })
            .scroll((self.vertical_scroll as u16, 0))
            .style(Style::default().fg(Color::White))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("^"))
            .end_symbol(Some("v"));

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(text.lines().count())
            .viewport_content_length(*h)
            .position(self.vertical_scroll);

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
