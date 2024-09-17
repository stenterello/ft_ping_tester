use crate::traits::comparer::Comparer;
use crate::traits::thread_stringpuller::{ThreadStringPuller, ThreadStringPullerWidget, Viewer};
use crate::traits::tui_widget::TuiWidget;
use crate::utils::config::config_extractor::Locations;
use crate::widgets::common::commands_widget::CommandsWidget;
use crate::widgets::common::message_widget::MessageWidget;
use crate::widgets::common::output_viewer::OutputViewer;
use crate::widgets::common::test_summary_widget::TestSummaryWidget;

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use serde_json::Value;

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
    running: bool,
    finished: bool,
    to_run: bool,
    tests: Value,
    tests_idx: usize,
    to_clear: bool,
    state: State,
}

impl TuiWidget for OutputTestsWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match self.state {
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

    fn summary_widget(&mut self) -> &mut TestSummaryWidget {
        &mut self.summary_widget
    }

    fn message_widget(&mut self) -> &mut MessageWidget {
        &mut self.message_widget
    }

    fn output_viewer(&mut self, v: Viewer) -> &mut OutputViewer {
        match v {
            Viewer::FtPing => &mut self.ft_ping_output_viewer,
            Viewer::Ping => &mut self.ping_output_viewer,
        }
    }

    fn set_running(&mut self, v: bool) -> () {
        self.running = v;
    }

    fn increment_test_index(&mut self) -> () {
        self.tests_idx += 1;
    }

    fn set_finished(&mut self) -> () {
        self.finished = true;
    }
}

impl ThreadStringPullerWidget for OutputTestsWidget {
    fn commands_widget(&mut self) -> &mut CommandsWidget {
        &mut self.commands_widget
    }

    fn running(&self) -> bool {
        self.running
    }

    fn to_run(&self) -> bool {
        self.to_run
    }

    fn set_to_run(&mut self, v: bool) -> () {
        self.to_run = v;
    }
}

impl OutputTestsWidget {
    pub fn new(locations: Locations, tests: Value) -> Self {
        OutputTestsWidget {
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

    pub fn reset_test_index(&mut self) -> () {
        self.tests_idx = usize::default();
    }
}
