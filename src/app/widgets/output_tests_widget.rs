use crate::app::utils::config::config_extractor::Locations;
use crate::app::widgets::common::commands_widget::CommandsWidget;
use crate::app::widgets::common::message_widget::MessageWidget;
use crate::app::widgets::common::output_viewer::OutputViewer;
use crate::app::widgets::common::test_summary_widget::TestSummaryWidget;
use crate::app::widgets::traits::comparer::Comparer;
use crate::app::widgets::traits::thread_stringpuller::{
    ThreadStringPuller, ThreadStringPullerWidget,
};
use crate::app::widgets::traits::tui_widget::TuiWidget;
use crate::app::widgets::traits::viewer::Viewer;

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use serde_json::Value;

use super::common::processing_widget::ProcessingWidget;
use super::traits::thread_stringpuller::ViewerType;

#[derive(Debug, Default)]
enum State {
    #[default]
    Interactive,
    Summary,
}

#[derive(Debug)]
pub struct OutputTestsWidget {
    ft_ping_output_viewer: OutputViewer,
    ping_output_viewer: OutputViewer,
    message_widget: MessageWidget,
    commands_widget: CommandsWidget,
    summary_widget: TestSummaryWidget,
    processing_widget: ProcessingWidget,
    running: bool,
    to_run: bool,
    tests: Value,
    tests_idx: usize,
    to_clear: bool,
    state: State,
    upper_state: Option<crate::app::State>,
}

impl TuiWidget for OutputTestsWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match self.state {
            State::Interactive => match key_event.code {
                KeyCode::Char('q') => {
                    self.upper_state = Some(crate::app::State::Welcome);
                    self.reset_test_index();
                    self.summary_widget.clear_results();
                }
                KeyCode::Char(' ') => {
                    if !self.running && !self.to_run {
                        self.to_run = true;
                        self.ft_ping_output_viewer.clear_buffers();
                        self.ping_output_viewer.clear_buffers();
                    }
                }
                _ => {}
            },
            State::Summary => {
                self.summary_widget.process_input(key_event);
            }
        };
    }

    fn draw(&mut self, frame: &mut Frame) -> std::io::Result<()> {
        match self.state {
            State::Interactive => self.draw_interactive_mode(frame),
            State::Summary => {
                frame.render_widget(&self.summary_widget, frame.size());
                Ok(())
            }
        }
    }

    fn set_to_clear(&mut self, v: bool) -> () {
        self.to_clear = v;
    }

    fn to_clear(&self) -> bool {
        self.to_clear
    }

    fn state(&mut self) -> Option<crate::app::State> {
        self.upper_state.take()
    }
}

impl Comparer for OutputTestsWidget {
    fn set_errors(&mut self, val: bool) -> () {
        self.message_widget.set_errors(val);
    }
}

impl ThreadStringPuller for OutputTestsWidget {
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

    fn viewer_mut(&mut self, v: ViewerType) -> &mut impl Viewer {
        match v {
            ViewerType::FtPing => &mut self.ft_ping_output_viewer,
            ViewerType::Ping => &mut self.ping_output_viewer,
        }
    }

    fn viewer(&self, v: ViewerType) -> &impl Viewer {
        match v {
            ViewerType::FtPing => &self.ft_ping_output_viewer,
            ViewerType::Ping => &self.ping_output_viewer,
        }
    }

    fn running(&self) -> bool {
        self.running
    }

    fn set_running(&mut self, v: bool) -> () {
        self.running = v;
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
}

impl ThreadStringPullerWidget for OutputTestsWidget {
    fn commands_widget(&mut self) -> &mut CommandsWidget {
        &mut self.commands_widget
    }

    fn render_viewer(&mut self, frame: &mut Frame, t: ViewerType, area: ratatui::prelude::Rect) {
        match t {
            ViewerType::FtPing => frame.render_widget(&self.ft_ping_output_viewer, area),
            ViewerType::Ping => frame.render_widget(&self.ping_output_viewer, area),
        }
    }
}

impl OutputTestsWidget {
    pub fn new(locations: &Locations, tests: Value) -> Self {
        OutputTestsWidget {
            ft_ping_output_viewer: OutputViewer::new(
                &locations.ft_ping_dir,
                &locations.ft_ping_name,
            ),
            ping_output_viewer: OutputViewer::new(&locations.ping_dir, &locations.ping_name),
            message_widget: MessageWidget::default(),
            commands_widget: CommandsWidget::new(" Q: Back | Space: Next test "),
            summary_widget: TestSummaryWidget::default(),
            processing_widget: ProcessingWidget::default(),
            running: false,
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
