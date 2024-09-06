use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    Frame,
};
use serde_json::Value;
use std::io::{Error, ErrorKind, Result};

use crate::traits::tui_widget_trait::TuiWidget;
use crate::utils::config_extractor::Locations;
use crate::widgets::message_widget::MessageWidget;
use crate::widgets::output_viewer::OutputViewer;

enum OutputViewerWidget {
    FtPing,
    Ping,
}

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
            KeyCode::Char('q') => {}
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::Enter => {}
            _ => {}
        };
    }
}

impl ErrorHandling {
    pub fn new(locations: Locations, tests: Value) -> Self {
        ErrorHandling {
            ft_ping_output_viewer: OutputViewer::new(&locations.ft_ping_dir),
            ping_output_viewer: OutputViewer::new(&locations.ping_dir),
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

    fn check_thread_exit_status(&mut self, output_viewer: OutputViewerWidget) -> Result<()> {
        let widget = match output_viewer {
            OutputViewerWidget::FtPing => &mut self.ft_ping_output_viewer,
            OutputViewerWidget::Ping => &mut self.ping_output_viewer,
        };

        if widget.is_running() {
            match widget.get_exit_status() {
                Some(n) => {
                    return Err(Error::new(
                        ErrorKind::Interrupted,
                        format!(
                            "Exit code: {} : {}",
                            n,
                            widget.get_error_output().join("\n")
                        ),
                    ));
                },
                None => {}
            }
        }
        Ok(())
    }

    fn check_treads(&mut self) -> Result<()> {
        println!("Eccomi");
        match self.check_thread_exit_status(OutputViewerWidget::FtPing) {
            Ok(_) => {}
            Err(e) => { println!("return error"); return Err(e) },
        }

        match self.check_thread_exit_status(OutputViewerWidget::Ping) {
            Ok(_) => {}
            Err(e) => { println!("return error"); return Err(e) },
        }

        if !self.ft_ping_output_viewer.is_running() && !self.ping_output_viewer.is_running() {
            self.running = false;
        }

        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        if self.running == false && self.to_run {
            self.run_processes();
            self.to_run = false;
        } else if self.running {
            match self.check_treads() {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        let [upper_area, _] =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(frame.size());

        let [upper_left_area, upper_right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(upper_area);

        frame.render_widget(&self.ft_ping_output_viewer, upper_left_area);
        frame.render_widget(&self.ping_output_viewer, upper_right_area);
        Ok(())
    }
}
