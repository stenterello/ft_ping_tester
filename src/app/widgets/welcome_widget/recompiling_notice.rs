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

use crate::app::{utils::thread::Thread, widgets::{common::default_style::DefaultStyle, traits::viewer::OutputType}};

#[derive(Debug)]
pub struct RecompilingNotice {
    thread: Thread,
    location: String,
    vertical_scroll: usize,
    widget_height: RefCell<usize>,
}

impl RecompilingNotice {
    pub fn new(path: &str) -> Self {
        RecompilingNotice {
            thread: Thread::new("".into(), "make".into()),
            location: String::from(path),
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
        if self
            .thread
            .get_output(OutputType::Stdout)
            .join("\n")
            .lines()
            .count()
            > (self.widget_height.borrow().to_owned() + self.vertical_scroll - 5)
        {
            self.vertical_scroll += 1;
        }
    }

    pub fn clear_output(&mut self) {
        self.thread.clear_buffers();
    }
}

impl Widget for &RecompilingNotice {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = if let true = self.thread.is_running() {
            DefaultStyle::block(Title::from(" Recompiling... ".bold().yellow()))
        } else if self.thread.get_exit().0 == Some(0) {
            DefaultStyle::block(Title::from(" Done! ".bold().green()))
        } else {
            DefaultStyle::block(Title::from(" Error! ".bold().red()))
        };

        let mut text = self.thread.get_output(OutputType::Stdout).join("\n");
        text.push_str(
            self.thread
                .get_output(OutputType::Stderr)
                .join("\n")
                .as_str(),
        );
        let mut h = self.widget_height.borrow_mut();
        *h = area.height as usize;

        Paragraph::new(text.clone())
            .block(block.clone())
            .wrap(Wrap { trim: true })
            .scroll((self.vertical_scroll as u16, 0))
            .style(DefaultStyle::style())
            .render(area, buf);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("^"))
            .end_symbol(Some("v"));

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(5)
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
