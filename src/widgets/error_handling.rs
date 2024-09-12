use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    Frame,
};
use serde_json::Value;
use std::io::{Error, ErrorKind, Result};

use crate::traits::comparer::Comparer;
use crate::traits::tui_widget::TuiWidget;
use crate::utils::config_extractor::Locations;
use crate::widgets::commands_widget::CommandsWidget;
use crate::widgets::message_widget::MessageWidget;
use crate::widgets::output_viewer::{OutputViewer, TextType};

// use std::fs::OpenOptions;
// use std::io::prelude::*;

enum Viewer {
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
    tests_idx: usize,
    commands_widget: CommandsWidget,
}

impl TuiWidget for ErrorHandling {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match key_event.code {
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::Enter => {}
            KeyCode::Char(' ') => {
                if !self.running && !self.to_run {
                    self.to_run = true;
                    self.ft_ping_output_viewer.clear_buffers();
                    self.ping_output_viewer.clear_buffers();
                }
            }
            _ => {}
        };
    }
}

impl Comparer for ErrorHandling {
    fn set_errors(&mut self, val: bool) -> () {
        self.message_widget.set_errors(val);
    }
}

impl ErrorHandling {
    pub fn new(locations: Locations, tests: Value) -> Self {
        ErrorHandling {
            ft_ping_output_viewer: OutputViewer::new(
                &locations.ft_ping_dir,
                &locations.ft_ping_name,
            ),
            ping_output_viewer: OutputViewer::new(&locations.ping_dir, &locations.ping_name),
            message_widget: MessageWidget::default(),
            running: false,
            to_run: true,
            tests,
            tests_idx: usize::default(),
            commands_widget: CommandsWidget::default(),
        }
    }

    pub fn run_processes(&mut self) {
        match self.tests.get(self.tests_idx) {
            Some(test) => {
                let mut str_vector: Vec<String> = Vec::default();
                for val in test.clone().as_array().unwrap() {
                    str_vector.push(String::from(val.as_str().unwrap()));
                }
                self.message_widget.set_arguments(str_vector.clone());
                self.ft_ping_output_viewer.start_process(str_vector.clone());
                self.ping_output_viewer.start_process(str_vector);
                self.running = true;
                self.tests_idx += 1;
            }
            None => {}
        }
    }

    fn check_thread_exit_status(&mut self, output_viewer: Viewer) -> Result<()> {
        let viewer = match output_viewer {
            Viewer::FtPing => &mut self.ft_ping_output_viewer,
            Viewer::Ping => &mut self.ping_output_viewer,
        };

        match viewer.get_exit_status() {
            (None, Some(err)) => {
                return Err(Error::new(
                    ErrorKind::Interrupted,
                    format!("Error: {}", err),
                ));
            }
            _ => {}
        }

        Ok(())
    }

    fn check_treads(&mut self) -> Result<()> {
        match self.check_thread_exit_status(Viewer::FtPing) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        match self.check_thread_exit_status(Viewer::Ping) {
            Ok(_) => {}
            Err(e) => return Err(e),
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

        let [upper_area, lower_area] =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(frame.size());

        let [status_area, commands_area] =
            Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)])
                .areas(lower_area);

        let [upper_left_area, upper_right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(upper_area);

        let (mut ft_ping_text, ping_text): (Vec<String>, Vec<String>) = (
            self.ft_ping_output_viewer.get_output(),
            self.ping_output_viewer.get_output(),
        );

        let (mut ft_ping_error_text, ping_error_text): (Vec<String>, Vec<String>) = (
            self.ft_ping_output_viewer.get_error_output(),
            self.ping_output_viewer.get_error_output(),
        );

        let ft_ping_formatted =
            TextType::Formatted(self.compare_output(&mut ft_ping_text, &ping_text));
        let ft_ping_error_formatted =
            TextType::Formatted(self.compare_output(&mut ft_ping_error_text, &ping_error_text));

        self.ft_ping_output_viewer
            .set_text_to_display(ft_ping_formatted);
        self.ping_output_viewer
            .set_text_to_display(TextType::Standard(ping_text));
        self.ft_ping_output_viewer
            .set_error_to_display(ft_ping_error_formatted);
        self.ping_output_viewer
            .set_error_to_display(TextType::Standard(ping_error_text));

        frame.render_widget(&self.ft_ping_output_viewer, upper_left_area);
        frame.render_widget(&self.ping_output_viewer, upper_right_area);
        frame.render_widget(&self.message_widget, status_area);
        frame.render_widget(&self.commands_widget, commands_area);

        Ok(())
    }
}
