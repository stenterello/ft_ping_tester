use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    Frame,
};
use serde_json::Value;
use std::io::{Error, ErrorKind, Result};

use crate::app::comparer::Comparer;
use crate::traits::tui_widget_trait::TuiWidget;
use crate::utils::config_extractor::Locations;
use crate::widgets::message_widget::MessageWidget;
use crate::widgets::output_viewer::OutputViewer;

// use std::fs::OpenOptions;
// use std::io::prelude::*;

enum Viewer {
    FtPing,
    Ping,
}

#[derive(Debug)]
pub struct ErrorHandling<'a> {
    ft_ping_output_viewer: OutputViewer<'a>,
    ping_output_viewer: OutputViewer<'a>,
    message_widget: MessageWidget,
    running: bool,
    to_run: bool,
    tests: Value,
    tests_idx: usize,
}

impl<'a> TuiWidget for ErrorHandling<'a> {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match key_event.code {
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::Enter => {}
            _ => {}
        };
    }
}

impl<'a> ErrorHandling<'a> {
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

        let [upper_left_area, upper_right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(upper_area);

        self.ft_ping_output_viewer.clear_text_to_display();
        self.ping_output_viewer.clear_text_to_display();

        let mut ft_ping_text = self.ft_ping_output_viewer.get_output();
        let ping_text = self.ping_output_viewer.get_output();

        if ft_ping_text.len() > 0 || ping_text.len() > 0 {
            // let mut file = OpenOptions::new()
            //     .write(true)
            //     .append(true)
            //     .open("ciao.txt")
            //     .unwrap();

            // if let Err(e) = writeln!(file, "ft_ping_len {}, ping_len {}", ft_ping_text.len(), ping_text.len()) {
            //     eprintln!("Couldn't write to file: {}", e);
            // }
            if !Comparer::compare_output(&mut ft_ping_text, &ping_text) {
                self.ft_ping_output_viewer
                    .set_text_to_display(vec!["Error".into()].into());
            } else {
                self.ft_ping_output_viewer
                    .set_text_to_display(vec![ft_ping_text.join("\n").into()].into());
            }
        } else {
            let mut ft_ping_error_text = self.ft_ping_output_viewer.get_error_output();
            let ping_error_text = self.ping_output_viewer.get_error_output();

            if ft_ping_error_text.len() > 0 || ping_error_text.len() > 0 {
                // let mut file = OpenOptions::new()
                //     .write(true)
                //     .append(true)
                //     .open("ciao.txt")
                //     .unwrap();

                // if let Err(e) = writeln!(file, "ft_ping_len {}, ping_len {}", ft_ping_text.len(), ping_text.len()) {
                //     eprintln!("Couldn't write to file: {}", e);
                // }
                if !Comparer::compare_output(&mut ft_ping_error_text, &ping_error_text) {
                    self.ping_output_viewer.set_text_to_display(vec!["Error".into()].into());
                } else {
                    self.ping_output_viewer
                        .set_text_to_display(vec![ping_error_text.join("\n").into()].into());
                }
            }
        }

        frame.render_widget(&self.ft_ping_output_viewer, upper_left_area);
        frame.render_widget(&self.ping_output_viewer, upper_right_area);
        frame.render_widget(&self.message_widget, lower_area);
        Ok(())
    }
}
