use crate::traits::tui_widget::TuiWidget;
use crate::utils::enums::TestResult;
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
        self.test_results.last_mut().unwrap().1 = result;
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
}

impl Widget for &TestSummaryWidget {
    fn render(self, area: Rect, buf: &mut Buffer) -> () {
        let title = Title::from(" Test Summary ".bold().yellow());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

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
            .style(Style::default().bg(Color::Rgb(46, 52, 64)).fg(Color::White))
            .render(area, buf);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("^"))
            .end_symbol(Some("v"));

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(self.test_results.len())
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
