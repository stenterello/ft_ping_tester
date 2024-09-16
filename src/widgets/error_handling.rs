use crate::traits::comparer::Comparer;
use crate::traits::tui_widget::TuiWidget;
use crate::utils::config::config_extractor::Locations;
use crate::widgets::common::choose_test_method::ChooseTestMethod;
use crate::widgets::common::commands_widget::CommandsWidget;
use crate::widgets::common::message_widget::MessageWidget;
use crate::widgets::common::output_viewer::{OutputViewer, TextType};
use crate::widgets::common::test_summary_widget::{TestResult, TestSummaryWidget};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    widgets::Clear,
    Frame,
};
use serde_json::Value;
use std::io::{Error, ErrorKind, Result};

enum Viewer {
    FtPing,
    Ping,
}

#[derive(Debug, Default)]
enum State {
    #[default]
    ChooseMethod,
    Interactive,
    Batch,
    Summary,
}

#[derive(Debug)]
pub struct ErrorHandling {
    choose_method_widget: ChooseTestMethod,
    ft_ping_output_viewer: OutputViewer,
    ping_output_viewer: OutputViewer,
    message_widget: MessageWidget,
    commands_widget: CommandsWidget,
    summary_widget: TestSummaryWidget,
    running: bool,
    finished: bool,
    to_run: bool,
    tests: Value,
    tests_idx: usize,
    to_clear: bool,
    state: State,
}

impl TuiWidget for ErrorHandling {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match self.state {
            State::ChooseMethod => {
                self.choose_method_widget.process_input(key_event);
                if let Some(state) = self.choose_method_widget.selected() {
                    match state {
                        0 => self.state = State::Interactive,
                        1 => self.state = State::Batch,
                        _ => {}
                    }
                }
            }
            State::Interactive => match key_event.code {
                KeyCode::Char(' ') => {
                    if !self.running && !self.to_run {
                        self.to_run = true;
                        self.ft_ping_output_viewer.clear_buffers();
                        self.ping_output_viewer.clear_buffers();
                    }
                }
                _ => {}
            },
            State::Batch => {}
            State::Summary => {
                self.summary_widget.process_input(key_event);
            }
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
            choose_method_widget: ChooseTestMethod::new(vec![
                "Interactive".to_string(),
                "Immediate".to_string(),
            ]),
            ft_ping_output_viewer: OutputViewer::new(
                &locations.ft_ping_dir,
                &locations.ft_ping_name,
            ),
            ping_output_viewer: OutputViewer::new(&locations.ping_dir, &locations.ping_name),
            message_widget: MessageWidget::default(),
            commands_widget: CommandsWidget::new(" Q: Back | Space: Next test ".to_string()),
            summary_widget: TestSummaryWidget::default(),
            running: false,
            finished: false,
            to_run: true,
            tests,
            tests_idx: usize::default(),
            to_clear: false,
            state: State::default(),
        }
    }

    pub fn run_processes(&mut self) {
        match self.tests.get(self.tests_idx) {
            Some(test) => {
                let arguments_vec = match test.as_array() {
                    Some(s) => s
                        .iter()
                        .filter_map(|val| val.as_str().map(|s| s.to_string()))
                        .collect(),
                    None => Vec::new(),
                };

                self.summary_widget.add_test(arguments_vec.join(" "));
                self.message_widget.set_arguments(arguments_vec.join(" "));
                self.ft_ping_output_viewer
                    .start_process(arguments_vec.clone());
                self.ping_output_viewer.start_process(arguments_vec);
                self.running = true;
                self.tests_idx += 1;
            }
            None => {
                self.finished = true;
            }
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

    fn draw_interactive_mode(&mut self, frame: &mut Frame) -> Result<()> {
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

        let (mut ft_useful_error_text, ft_unnecessary_path) =
            ErrorHandling::remove_path(&mut ft_ping_error_text);
        let (ping_useful_error_text, ping_unnecessary_path) =
            ErrorHandling::remove_path(&mut ping_error_text);

        let ft_ping_formatted =
            TextType::Formatted(self.compare_output(&mut ft_ping_text, &ping_text));
        let mut ft_ping_error_formatted = TextType::Formatted(
            self.compare_output(&mut ft_useful_error_text, &ping_useful_error_text),
        );

        self.summary_widget
            .set_result(match !self.message_widget.errors() {
                true => TestResult::Correct,
                false => TestResult::Incorrect,
            });

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
                        }
                        None => {
                            first_line.insert(last_index + 1, (true, 58));
                            first_line.insert(last_index + 2, (true, 32));
                            break;
                        }
                    }
                }
            };
            ft_useful_error_text
                .get_mut(0)
                .unwrap()
                .insert_str(0, (ft_unnecessary_path + ": ").as_str());
        }
        if !ping_unnecessary_path.is_empty() {
            ping_useful_error_text
                .get_mut(0)
                .unwrap()
                .insert_str(0, (ping_unnecessary_path + ": ").as_str());
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

    pub fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        match self.state {
            State::ChooseMethod => self.choose_method_widget.draw(frame),
            State::Interactive => self.draw_interactive_mode(frame),
            State::Batch => Ok(()),
            State::Summary => {
                frame.render_widget(&self.summary_widget, frame.size());
                Ok(())
            }
        }
    }

    pub fn reset_test_index(&mut self) -> () {
        self.tests_idx = usize::default();
    }
}
