use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    widgets::Clear,
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
    to_clear: bool,
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
            to_clear: false,
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
            self.to_clear = true;
        } else if self.running {
            self.to_clear = false;
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

        let (mut ft_ping_error_text, mut ping_error_text): (Vec<String>, Vec<String>) = (
            self.ft_ping_output_viewer.get_error_output(),
            self.ping_output_viewer.get_error_output(),
        );

        let (mut ft_useful_error_text, ft_unnecessary_path) = ErrorHandling::remove_path(&mut ft_ping_error_text);
        let (ping_useful_error_text, ping_unnecessary_path) = ErrorHandling::remove_path(&mut ping_error_text);

        let ft_ping_formatted =
            TextType::Formatted(self.compare_output(&mut ft_ping_text, &ping_text));
        let mut ft_ping_error_formatted =
            TextType::Formatted(self.compare_output(&mut ft_useful_error_text, &ping_useful_error_text));

        if !ft_unnecessary_path.is_empty() {
            if let TextType::Formatted(ref mut vec) = ft_ping_error_formatted {
                let vec: &mut Vec<Vec<(bool, u8)>> = vec;
                let first_line: &mut Vec<(bool, u8)> = vec.get_mut(0).unwrap();
                let mut iter = ft_unnecessary_path.as_bytes().iter().enumerate();
                let mut last_index: usize = 0;
                loop {
                    match iter.next() {
                        Some(c) => {
                            first_line.insert(c.0, (true, *c.1));
                            last_index = c.0;
                        },
                        None => {
                            first_line.insert(last_index + 1, (true, 58));
                            first_line.insert(last_index + 2, (true, 32));
                            break;
                        },
                    }
                }

            };
            ft_useful_error_text.get_mut(0).unwrap()
                .insert_str(
                    0,
                    (ft_unnecessary_path + ": ").as_str()
                );
        }
        if !ping_unnecessary_path.is_empty() {
            ping_useful_error_text.get_mut(0).unwrap()
                .insert_str(
                    0,
                    (ping_unnecessary_path + ": ").as_str()
                );
        }

        self.ft_ping_output_viewer
            .set_text_to_display(ft_ping_formatted);
        self.ping_output_viewer
            .set_text_to_display(TextType::Standard(ping_text));
        self.ft_ping_output_viewer
            .set_error_to_display(ft_ping_error_formatted);
        self.ping_output_viewer
            .set_error_to_display(TextType::Standard(ping_useful_error_text.to_owned()));

        if self.to_clear {
            frame.render_widget(Clear, upper_left_area);
            frame.render_widget(Clear, upper_right_area);
        } else {
            frame.render_widget(&self.ft_ping_output_viewer, upper_left_area);
            frame.render_widget(&self.ping_output_viewer, upper_right_area);
        }
        frame.render_widget(&self.message_widget, status_area);
        frame.render_widget(&self.commands_widget, commands_area);

        Ok(())
    }

    pub fn reset_test_index(&mut self) -> () {
        self.tests_idx = usize::default();
    }
}
