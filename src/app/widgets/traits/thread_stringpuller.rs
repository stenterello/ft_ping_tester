use super::thread_launcher::ThreadLauncher;
use super::tui_widget::TuiWidget;
use super::viewer::Viewer;
use crate::app::utils::enums::{TestResult, TextType};
use crate::app::widgets::common::commands_widget::CommandsWidget;
use crate::app::widgets::common::message_widget::MessageWidget;
use crate::app::widgets::common::processing_widget::ProcessingWidget;
use crate::app::widgets::common::test_summary_widget::TestSummaryWidget;
use crate::app::widgets::traits::comparer::Comparer;
use crate::app::widgets::common::thread_manager::ThreadManager;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Clear;
use ratatui::Frame;
use serde_json::Value;
use std::io::{Error, ErrorKind, Result};

pub enum PingType {
    FtPing,
    Ping,
}

#[derive(PartialEq)]
pub enum ExitResult {
    Correct(i32),
    Error(i32, String),
    None,
}

pub trait ThreadStringPuller: Comparer + TuiWidget {
    fn get_actual_test(&self) -> Option<&Value>;
    fn tests(&self) -> &Value;
    fn tests_idx(&self) -> usize;
    fn summary_widget(&mut self) -> &mut TestSummaryWidget;
    fn message_widget(&mut self) -> &mut MessageWidget;
    fn processing_widget(&mut self) -> &mut ProcessingWidget;
    fn thread_mng_mut(&mut self, v: PingType) -> &mut ThreadManager;
    fn thread_mng(&self, v: PingType) -> &ThreadManager;
    fn viewer(&self, v: PingType) -> &impl Viewer;
    fn viewer_mut(&mut self, v: PingType) -> &mut impl Viewer;
    fn running(&self) -> bool;
    fn to_run(&self) -> bool;
    fn set_to_run(&mut self, v: bool) -> ();
    fn increment_test_index(&mut self) -> ();
    fn set_finished(&mut self) -> ();
    fn clear_buffers(&mut self) -> ();
    fn run_processes(&mut self) {
        match self.get_actual_test() {
            Some(test) => {
                let arguments_vec = match test.as_array() {
                    Some(s) => s
                        .iter()
                        .filter_map(|val| val.as_str().map(|s| s.to_string()))
                        .collect(),
                    None => Vec::new(),
                };

                self.summary_widget().add_test(arguments_vec.join(" "));
                self.message_widget().set_arguments(arguments_vec.join(" "));
                self.thread_mng_mut(PingType::FtPing).start_process(arguments_vec.clone());
                self.thread_mng_mut(PingType::Ping).start_process(arguments_vec);
                self.message_widget().set_running(true);
                self.increment_test_index();
            }
            None => {
                self.set_finished();
            }
        }
    }

    fn check_thread_exit_status(&mut self, output_viewer: PingType) -> Result<()> {
        let viewer = match output_viewer {
            PingType::FtPing => self.thread_mng_mut(PingType::FtPing),
            PingType::Ping => self.thread_mng_mut(PingType::Ping),
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
        if !self.thread_mng(PingType::FtPing).is_running() && !self.thread_mng(PingType::Ping).is_running() {
            self.message_widget().set_running(false);
        }

        if let Err(e) = self.check_thread_exit_status(PingType::FtPing) {
            return Err(e);
        }

        if let Err(e) = self.check_thread_exit_status(PingType::Ping) {
            return Err(e);
        }

        Ok(())
    }

    fn retrieve_exit_status(&mut self, t: PingType) -> ExitResult {
        match self.thread_mng_mut(t).get_exit_status() {
            (Some(code), None) => ExitResult::Correct(code),
            (Some(code), Some(err)) => ExitResult::Error(code, err),
            (None, None) => ExitResult::None,
            _ => ExitResult::None,
        }
    }

    fn batch_mode(&mut self) -> Result<()> {
        let tests_len = self.tests().as_array().unwrap().len();
        let ratio = tests_len as f64 / 100f64 * (self.tests_idx() + 1) as f64 / 10f64;
        self.processing_widget().set_ratio(ratio);

        if !self.running() && self.tests_idx() != tests_len - 1 {
            self.run_processes();
            self.set_to_run(false);
        } else if self.running() {
            match self.check_treads() {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        let (mut ft_ping_text, ping_text): (Vec<String>, Vec<String>) = (
            self.thread_mng_mut(PingType::FtPing)
                .take_output(super::viewer::OutputType::Stdout),
            self.thread_mng_mut(PingType::Ping)
                .take_output(super::viewer::OutputType::Stdout),
        );

        let (mut ft_ping_error_text, mut ping_error_text): (Vec<String>, Vec<String>) = (
            self.thread_mng_mut(PingType::FtPing)
                .take_output(super::viewer::OutputType::Stderr),
            self.thread_mng_mut(PingType::Ping)
                .take_output(super::viewer::OutputType::Stderr),
        );

        let (mut ft_useful_error_text, _) =
            <Self as Comparer>::remove_path(&mut ft_ping_error_text);
        let (ping_useful_error_text, _) = <Self as Comparer>::remove_path(&mut ping_error_text);

        let _ = TextType::Formatted(self.compare_output(&mut ft_ping_text, &ping_text));
        let _ = TextType::Formatted(
            self.compare_output(&mut ft_useful_error_text, &ping_useful_error_text),
        );

        let ft_exit = self.retrieve_exit_status(PingType::FtPing);
        let ping_exit = self.retrieve_exit_status(PingType::Ping);

        if ft_exit != ping_exit {
            self.message_widget().set_errors(true);
        }

        let res: TestResult = match !self.message_widget().errors() {
            true => TestResult::Correct,
            false => TestResult::Incorrect,
        };
        self.summary_widget().set_result(res);
        Ok(())
    }
}

pub trait ThreadStringPullerWidget: ThreadStringPuller {
    fn commands_widget(&mut self) -> &mut CommandsWidget;

    fn draw_interactive_mode(&mut self, frame: &mut Frame) -> Result<()> {
        if !self.running() && self.to_run() {
            self.run_processes();
            self.set_to_run(false);
            self.set_to_clear(true);
        } else if self.running() {
            self.set_to_clear(false);
            match self.check_treads() {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        let (commands_area, area) = Self::commands_area(frame);

        let [upper_area, status_area] =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(area);

        let [upper_left_area, upper_right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(upper_area);

        let (mut ft_ping_text, ping_text): (Vec<String>, Vec<String>) = (
            self.thread_mng(PingType::FtPing)
                .get_output(super::viewer::OutputType::Stdout),
            self.thread_mng(PingType::Ping)
                .get_output(super::viewer::OutputType::Stdout),
        );

        let (mut ft_ping_error_text, mut ping_error_text): (Vec<String>, Vec<String>) = (
            self.thread_mng(PingType::FtPing)
                .get_output(super::viewer::OutputType::Stderr),
            self.thread_mng(PingType::Ping)
                .get_output(super::viewer::OutputType::Stderr),
        );

        let (mut ft_useful_error_text, ft_unnecessary_path) =
            <Self as Comparer>::remove_path(&mut ft_ping_error_text);
        let (ping_useful_error_text, ping_unnecessary_path) =
            <Self as Comparer>::remove_path(&mut ping_error_text);

        let ft_ping_formatted =
            TextType::Formatted(self.compare_output(&mut ft_ping_text, &ping_text));
        let mut ft_ping_error_formatted = TextType::Formatted(
            self.compare_output(&mut ft_useful_error_text, &ping_useful_error_text),
        );

        let ft_exit = self.retrieve_exit_status(PingType::FtPing);
        let ping_exit = self.retrieve_exit_status(PingType::Ping);

        self.message_widget().set_codes(ft_exit, ping_exit);

        let res: TestResult = match !self.message_widget().errors() {
            true => TestResult::Correct,
            false => TestResult::Incorrect,
        };
        self.summary_widget().set_result(res);

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

        self.viewer_mut(PingType::FtPing)
            .set_text_to_display(ft_ping_formatted);
        self.viewer_mut(PingType::Ping)
            .set_text_to_display(TextType::Standard(ping_text));
        self.viewer_mut(PingType::FtPing)
            .set_error_to_display(ft_ping_error_formatted);
        self.viewer_mut(PingType::Ping)
            .set_error_to_display(TextType::Standard(ping_useful_error_text.to_owned()));

        if self.to_clear() {
            frame.render_widget(Clear, upper_left_area);
            frame.render_widget(Clear, upper_right_area);
            self.set_to_clear(false);
        } else {
            self.render_viewer(frame, PingType::FtPing, upper_left_area);
            self.render_viewer(frame, PingType::Ping, upper_right_area);
        }

        frame.render_widget(&*self.message_widget(), status_area);
        frame.render_widget(&*self.commands_widget(), commands_area);
        Ok(())
    }

    fn render_viewer(&mut self, frame: &mut Frame, t: PingType, area: Rect);
}
