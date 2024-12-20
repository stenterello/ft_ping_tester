use crate::app::utils::config::config_extractor::Locations;
use crate::app::widgets::common::choose_test_method::ChooseTestMethod;
use crate::app::widgets::common::commands_widget::CommandsWidget;
use crate::app::widgets::common::message_widget::MessageWidget;
use crate::app::widgets::common::output_viewer::OutputViewer;
use crate::app::widgets::common::test_summary_widget::TestSummaryWidget;
use crate::app::widgets::traits::comparer::Comparer;
use crate::app::widgets::traits::thread_stringpuller::{
    ThreadStringPuller, ThreadStringPullerWidget,
};
use crate::app::widgets::traits::tui_widget::TuiWidget;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    Frame,
};
use serde_json::Value;
use std::io::Result;
use crate::app::widgets::common::thread_manager::ThreadManager;
use crate::app::widgets::traits::runner::Runner;
use super::common::processing_widget::ProcessingWidget;
use super::traits::thread_stringpuller::PingType;
use super::traits::viewer::Viewer;

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
    ft_ping_thread_mng: ThreadManager,
    ping_thread_mng: ThreadManager,
    ft_ping_output_viewer: OutputViewer,
    ping_output_viewer: OutputViewer,
    message_widget: MessageWidget,
    commands_widget: CommandsWidget,
    summary_widget: TestSummaryWidget,
    processing_widget: ProcessingWidget,
    to_run: bool,
    tests: Value,
    tests_idx: usize,
    to_clear: bool,
    state: State,
    upper_state: Option<crate::app::State>,
}

impl ErrorHandling {
    pub fn new(locations: &Locations, tests: Value) -> Self {
        ErrorHandling {
            choose_method_widget: ChooseTestMethod::new(vec![
                "Interactive".to_string(),
                "Immediate".to_string(),
            ]),
            ft_ping_thread_mng: ThreadManager::new(
                &locations.ft_ping_dir,
                &locations.ft_ping_name
            ),
            ft_ping_output_viewer: OutputViewer::new(&locations.ft_ping_name),
            ping_thread_mng: ThreadManager::new(&locations.ping_dir, &locations.ping_name),
            ping_output_viewer: OutputViewer::new(&locations.ping_name),
            message_widget: MessageWidget::default(),
            commands_widget: CommandsWidget::new(" Q: Back | Space: Next test "),
            summary_widget: TestSummaryWidget::default(),
            processing_widget: ProcessingWidget::default(),
            to_run: true,
            tests,
            tests_idx: usize::default(),
            to_clear: false,
            state: State::default(),
            upper_state: None,
        }
    }

    pub fn reset_test_index(&mut self) -> () {
        self.tests_idx = usize::default();
    }
}

impl TuiWidget for ErrorHandling {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        if key_event.code == KeyCode::Char('q') {
            self.upper_state = Some(crate::app::State::Welcome);
            self.state = State::ChooseMethod;
            self.reset_test_index();
            self.summary_widget.clear_results();
        } else {
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
                        if !self.running() && !self.to_run {
                            self.to_run = true;
                            self.clear_buffers();
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

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        match self.state {
            State::ChooseMethod => self.choose_method_widget.draw(frame),
            State::Interactive => self.draw_interactive_mode(frame),
            State::Batch => {
                let ret = self.batch_mode();
                frame.render_widget(&self.processing_widget, frame.size());
                if self.tests_idx == self.tests.as_array().unwrap().len() - 1 {
                    self.state = State::Summary;
                }
                ret
            }
            State::Summary => {
                frame.render_widget(&self.summary_widget, frame.size());
                Ok(())
            }
        }
    }

    fn set_to_clear(&mut self, v: bool) -> () {
        self.to_clear = v;
    }

    fn to_clear(&self) -> &bool {
        &self.to_clear
    }

    fn state(&mut self) -> Option<crate::app::State> {
        self.upper_state.take()
    }
}

impl Comparer for ErrorHandling {
    fn set_errors(&mut self, val: bool) -> () {
        self.message_widget.set_errors(val);
    }
}

impl ThreadStringPuller for ErrorHandling {
    fn get_actual_test(&self) -> Option<&Value> {
        self.tests.get(self.tests_idx)
    }

    fn tests(&self) -> &Value {
        &self.tests
    }

    fn tests_idx(&self) -> usize {
        self.tests_idx
    }

    fn summary_widget(&mut self) -> &mut TestSummaryWidget {
        &mut self.summary_widget
    }

    fn message_widget(&mut self) -> &mut MessageWidget {
        &mut self.message_widget
    }

    fn processing_widget(&mut self) -> &mut ProcessingWidget {
        &mut self.processing_widget
    }

    fn viewer_mut(&mut self, v: PingType) -> &mut impl Viewer {
        match v {
            PingType::FtPing => &mut self.ft_ping_output_viewer,
            PingType::Ping => &mut self.ping_output_viewer,
        }
    }

    fn running(&self) -> bool {
        self.ft_ping_thread_mng.is_running() || self.ping_thread_mng.is_running()
    }

    fn to_run(&self) -> bool {
        self.to_run
    }

    fn set_to_run(&mut self, v: bool) -> () {
        self.to_run = v;
    }

    fn increment_test_index(&mut self) -> () {
        self.tests_idx += 1;
    }

    fn set_finished(&mut self) -> () {
        self.state = State::Summary;
    }

    fn thread_mng_mut(&mut self, t: PingType) -> &mut ThreadManager {
        match t {
            PingType::FtPing => &mut self.ft_ping_thread_mng,
            PingType::Ping => &mut self.ping_thread_mng
        }
    }

    fn thread_mng(&self, t: PingType) -> &ThreadManager {
        match t {
            PingType::FtPing => &self.ft_ping_thread_mng,
            PingType::Ping => &self.ping_thread_mng
        }
    }

    fn clear_buffers(&mut self) -> () {
        self.ft_ping_output_viewer.clear_buffers();
        self.ping_output_viewer.clear_buffers();
        self.ft_ping_thread_mng.thread_mut().clear_buffers();
        self.ping_thread_mng.thread_mut().clear_buffers();
    }
}

impl ThreadStringPullerWidget for ErrorHandling {
    fn commands_widget(&mut self) -> &mut CommandsWidget {
        &mut self.commands_widget
    }

    fn render_viewer(&mut self, frame: &mut Frame, t: PingType, area: ratatui::prelude::Rect) {
        match t {
            PingType::FtPing => frame.render_widget(&self.ft_ping_output_viewer, area),
            PingType::Ping => frame.render_widget(&self.ping_output_viewer, area),
        }
    }
}

