use super::{
    common::test_summary_widget::TestSummaryWidget,
    traits::{thread_stringpuller::ViewerType, tui_widget::TuiWidget},
};
use packet_viewer::PacketViewer;
use input_dialog::InputDialog;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Layout,
    Frame,
};
use serde_json::Value;
use std::error::Error;
use std::io::Result;
use sudo::RunningAs;

mod packet_viewer;
mod input_dialog;

#[derive(Debug, Default, PartialEq)]
enum State {
    Initial,
    #[default]
    PermissionCheck,
    Summary,
}

#[derive(Debug, Default)]
pub struct PacketCompareWidget {
    state: State,
    upper_state: Option<crate::app::State>,
    tests: Value,
    tests_idx: usize,
    summary_widget: TestSummaryWidget,
    to_clear: bool,
    password_dialog: InputDialog,
    ft_ping_viewer: PacketViewer,
    ping_viewer: PacketViewer,
    commands_widget: CommandsWidget,
}

impl PacketCompareWidget {
    pub fn new() -> Self {
        Self {
            // interfaces: interfaces(),
            ft_ping_viewer: PacketViewer::new(ViewerType::FtPing),
            ping_viewer: PacketViewer::new(ViewerType::Ping),
            state: if let RunningAs::Root = sudo::check() {
                State::Initial
            } else {
                State::default()
            },
            password_dialog: InputDialog::new("Insert password"),
            commands_widget: CommandsWidget::new(" ↑/↓: Move Up/Down | Enter: Select | Q: Back "),
            ..Default::default()
        }
    }

    pub fn reset_test_index(&mut self) -> () {
        self.tests_idx = usize::default();
    }
}

use ratatui::prelude::Constraint;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::prelude::*;
use ratatui::widgets::Clear;
use crate::app::widgets::common::commands_widget::CommandsWidget;

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
            };
        }
    }

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        let (commands_area, area) = Self::commands_area(&frame);
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(frame.size());

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
