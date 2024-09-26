use super::{
    common::test_summary_widget::TestSummaryWidget,
    traits::{thread_stringpuller::PingType, tui_widget::TuiWidget},
};
use packet_viewer::PacketViewer;
use input_dialog::InputDialog;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Layout,
    Frame,
};
use ratatui::widgets::Clear;
use ratatui::prelude::Constraint;
use crate::app::widgets::common::commands_widget::CommandsWidget;
use serde_json::Value;
use std::io::Result;
use sudo::RunningAs;
use crate::app::utils::config::config_extractor::Locations;
use crate::app::utils::thread::Thread;
use crate::app::widgets::common::thread_manager::ThreadManager;

mod packet_viewer;
mod input_dialog;

#[derive(Debug, Default, PartialEq)]
enum State {
    Initial,
    #[default]
    PermissionCheck,
    WaitingProcess,
    PresentingResults,
    Summary,
}

#[derive(Debug)]
pub struct PacketCompareWidget {
    state: State,
    upper_state: Option<crate::app::State>,
    tests: Value,
    tests_idx: usize,
    summary_widget: TestSummaryWidget,
    to_clear: bool,
    password_dialog: InputDialog,
    ft_ping_thread_mng: ThreadManager,
    ping_thread_mng: ThreadManager,
    ft_ping_viewer: PacketViewer,
    ping_viewer: PacketViewer,
    commands_widget: CommandsWidget,
}

impl PacketCompareWidget {
    pub fn new(locations: &Locations, tests: Value) -> Self {
        Self {
            ft_ping_viewer: PacketViewer::new(PingType::FtPing),
            ping_viewer: PacketViewer::new(PingType::Ping),
            state: if let RunningAs::Root = sudo::check() {
                State::Initial
            } else {
                State::default()
            },
            password_dialog: InputDialog::new("Insert password"),
            commands_widget: CommandsWidget::new(" ↑/↓: Move Up/Down | Enter: Select | Q: Back "),
            ft_ping_thread_mng: ThreadManager::new(&locations.ft_ping_dir, &locations.ft_ping_name),
            ping_thread_mng: ThreadManager::new(&locations.ping_dir, &locations.ping_name),
            upper_state: None,
            tests,
            tests_idx: usize::default(),
            summary_widget: TestSummaryWidget::default(),
            to_clear: false,
        }
    }

    pub fn reset_test_index(&mut self) -> () {
        self.tests_idx = usize::default();
    }
}

impl TuiWidget for PacketCompareWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        if key_event.code == KeyCode::Char('q') && self.state != State::PermissionCheck {
            self.upper_state = Some(crate::app::State::Welcome);
            self.state = State::Initial;
            self.reset_test_index();
            self.summary_widget.clear_results();
        } else {
            match self.state {
                State::Initial => {}
                State::PermissionCheck => {
                    match key_event.code {
                        KeyCode::Esc => {
                            self.upper_state = Some(crate::app::State::Welcome);
                            self.state = State::Initial;
                            self.reset_test_index();
                            self.summary_widget.clear_results();
                        }
                        _ => self.password_dialog.process_input(key_event)
                    }
                }
                State::Summary => {
                    self.summary_widget.process_input(key_event);
                }
                _ => {}
            };
        }
    }

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        let (commands_area, area) = Self::commands_area(&frame);
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(area);

        frame.render_widget(&self.ft_ping_viewer, left_area);
        frame.render_widget(&self.ping_viewer, right_area);
        frame.render_widget(Clear, commands_area);
        frame.render_widget(&self.commands_widget, commands_area);

        if let State::PermissionCheck = &self.state {
            self.password_dialog.draw(frame)?;
        }

        Ok(())
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
