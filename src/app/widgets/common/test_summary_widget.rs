use crate::app::utils::enums::TestResult;
use crate::app::widgets::traits::tui_widget::TuiWidget;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Margin};
use ratatui::prelude::{Color, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::block::Title;
use ratatui::widgets::{
    Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};
use std::cell::RefCell;

use super::default_style::DefaultStyle;

#[derive(Debug, Default)]
pub struct TestSummaryWidget {
    test_results: Vec<(String, TestResult)>,
    vertical_scroll: usize,
    widget_height: RefCell<usize>,
}

impl TestSummaryWidget {
    pub fn add_test(&mut self, args: String) -> () {
        self.test_results.push((args, TestResult::default()));
    }

    pub fn set_result(&mut self, result: TestResult) -> () {
        match self.test_results.last_mut() {
            Some(r) => r.1 = result,
            None => {}
        }
    }

    fn move_up(&mut self) {
        if self.vertical_scroll > 0 {
            self.vertical_scroll -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.vertical_scroll + self.widget_height.borrow().to_owned()
            < self.test_results.len() + 5
        {
            self.vertical_scroll += 1;
        }
    }

    pub fn clear_results(&mut self) -> () {
        self.test_results.clear();
    }
}

impl TuiWidget for TestSummaryWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match key_event.code {
            KeyCode::Up => {
                self.move_up();
            }
            KeyCode::Down => {
                self.move_down();
            }
            _ => {}
        }
    }

    fn draw(&mut self, _frame: &mut ratatui::Frame) -> std::io::Result<()> {
        Ok(())
    }

    fn set_to_clear(&mut self, _v: bool) -> () {}
    fn to_clear(&self) -> bool {
        false
    }
}

impl Widget for &TestSummaryWidget {
    fn render(self, area: Rect, buf: &mut Buffer) -> () {
        let block = DefaultStyle::block(Title::from(" Test Summary ".bold().yellow()));

        let mut line_vec: Vec<Line> = Vec::default();
        for test in &self.test_results {
            match test.1 {
                TestResult::Correct => line_vec.push(Line::from(String::from(&test.0) + ": ✅")),
                TestResult::Incorrect => line_vec.push(Line::from(String::from(&test.0) + ": ❌")),
                TestResult::Unknown => line_vec.push(Line::from(String::from(&test.0) + ": 🛠️")),
            }
        }

        let mut h = self.widget_height.borrow_mut();
        *h = area.height as usize;

        Paragraph::new(Text::from(line_vec))
            .block(block)
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
