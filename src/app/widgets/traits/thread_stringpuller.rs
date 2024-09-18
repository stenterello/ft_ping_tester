use super::thread_launcher::ThreadLauncher;
use super::tui_widget::TuiWidget;
use crate::app::utils::enums::{TestResult, TextType};
use crate::app::widgets::common::commands_widget::CommandsWidget;
use crate::app::widgets::common::message_widget::MessageWidget;
use crate::app::widgets::common::output_viewer::OutputViewer;
use crate::app::widgets::common::processing_widget::ProcessingWidget;
use crate::app::widgets::common::test_summary_widget::TestSummaryWidget;
use crate::app::widgets::traits::comparer::Comparer;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Clear;
use ratatui::Frame;
use serde_json::Value;
use std::io::{Error, ErrorKind, Result};

pub enum Viewer {
    FtPing,
    Ping,
}

pub trait ThreadStringPuller: Comparer + TuiWidget {
    fn get_actual_test(&self) -> Option<&Value>;
    fn tests(&self) -> &Value;
    fn tests_idx(&self) -> usize;
    fn summary_widget(&mut self) -> &mut TestSummaryWidget;
    fn message_widget(&mut self) -> &mut MessageWidget;
    fn processing_widget(&mut self) -> &mut ProcessingWidget;
    fn output_viewer(&mut self, v: Viewer) -> &mut OutputViewer;
    fn running(&self) -> bool;
    fn set_running(&mut self, v: bool) -> ();
    fn to_run(&self) -> bool;
    fn set_to_run(&mut self, v: bool) -> ();
    fn increment_test_index(&mut self) -> ();
    fn set_finished(&mut self) -> ();

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
                self.output_viewer(Viewer::FtPing)
                    .start_process(arguments_vec.clone());
                self.output_viewer(Viewer::Ping)
                    .start_process(arguments_vec);
                self.set_running(true);
                self.increment_test_index();
            }
            None => {
                self.set_finished();
            }
        }
    }

    fn check_thread_exit_status(&mut self, output_viewer: Viewer) -> Result<()> {
        let viewer = match output_viewer {
            Viewer::FtPing => self.output_viewer(Viewer::FtPing),
            Viewer::Ping => self.output_viewer(Viewer::Ping),
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
        if let Err(e) = self.check_thread_exit_status(Viewer::FtPing) {
            return Err(e);
        }

        if let Err(e) = self.check_thread_exit_status(Viewer::Ping) {
            return Err(e);
        }

        if !self.output_viewer(Viewer::FtPing).is_running()
            && !self.output_viewer(Viewer::Ping).is_running()
        {
            self.set_running(false);
        }

        Ok(())
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
            self.output_viewer(Viewer::FtPing).take_output(),
            self.output_viewer(Viewer::Ping).take_output(),
        );

        let (mut ft_ping_error_text, mut ping_error_text): (Vec<String>, Vec<String>) = (
            self.output_viewer(Viewer::FtPing).take_error_output(),
            self.output_viewer(Viewer::Ping).take_error_output(),
        );

        let (mut ft_useful_error_text, qqq) =
            <Self as Comparer>::remove_path(&mut ft_ping_error_text);
        let (ping_useful_error_text, www) = <Self as Comparer>::remove_path(&mut ping_error_text);

        let _ = TextType::Formatted(self.compare_output(&mut ft_ping_text, &ping_text));
        let _ = TextType::Formatted(
            self.compare_output(&mut ft_useful_error_text, &ping_useful_error_text),
        );
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
            self.output_viewer(Viewer::FtPing).get_output(),
            self.output_viewer(Viewer::Ping).get_output(),
        );

        let (mut ft_ping_error_text, mut ping_error_text): (Vec<String>, Vec<String>) = (
            self.output_viewer(Viewer::FtPing).get_error_output(),
            self.output_viewer(Viewer::Ping).get_error_output(),
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

        self.output_viewer(Viewer::FtPing)
            .set_text_to_display(ft_ping_formatted);
        self.output_viewer(Viewer::Ping)
            .set_text_to_display(TextType::Standard(ping_text));
        self.output_viewer(Viewer::FtPing)
            .set_error_to_display(ft_ping_error_formatted);
        self.output_viewer(Viewer::Ping)
            .set_error_to_display(TextType::Standard(ping_useful_error_text.to_owned()));

        if self.to_clear() {
            frame.render_widget(Clear, upper_left_area);
            frame.render_widget(Clear, upper_right_area);
            self.set_to_clear(false);
        } else {
            frame.render_widget(&*self.output_viewer(Viewer::FtPing), upper_left_area);
            frame.render_widget(&*self.output_viewer(Viewer::Ping), upper_right_area);
        }
        frame.render_widget(&*self.message_widget(), status_area);
        frame.render_widget(&*self.commands_widget(), commands_area);
        Ok(())
    }
}
