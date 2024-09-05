use ratatui::{
    layout::{Constraint, Layout},
    Frame,
    crossterm::event::{KeyEvent, KeyCode},
};
use serde_json::Value;

use crate::traits::tui_widget_trait::TuiWidget;
use crate::widgets::message_widget::MessageWidget;
use crate::widgets::output_viewer::OutputViewer;

#[derive(Debug)]
pub struct ErrorHandling {
    ft_ping_output_viewer: OutputViewer,
    ping_output_viewer: OutputViewer,
    message_widget: MessageWidget,
    running: bool,
    to_run: bool,
    tests: Value,
}

impl TuiWidget for ErrorHandling {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match key_event.code {
            KeyCode::Char('q') => {},
            KeyCode::Up => {},
            KeyCode::Down => {},
            KeyCode::Enter => {},
            _ => {}
        };
    }
}

impl ErrorHandling {
    pub fn new(tests: Value) -> Self {
        ErrorHandling {
            ft_ping_output_viewer: OutputViewer::new("./ft_ping/ft_ping"),
            ping_output_viewer: OutputViewer::new("./inetutils-2.0/ping/ping"),
            message_widget: MessageWidget::default(),
            running: false,
            to_run: true,
            tests,
        }
    }

    pub fn run_processes(&mut self) {
        self.ft_ping_output_viewer.start_process();
        self.ping_output_viewer.start_process();
        self.running = true;
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        if self.running == false && self.to_run {
            self.run_processes();
            self.to_run = false;
        }

        let [upper_area, _] =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(frame.size());

        let [upper_left_area, upper_right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(upper_area);

        frame.render_widget(&self.ft_ping_output_viewer, upper_left_area);
        frame.render_widget(&self.ping_output_viewer, upper_right_area);
    }
}
