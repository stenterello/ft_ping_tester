use super::{
    common::test_summary_widget::TestSummaryWidget,
    traits::{thread_stringpuller::ViewerType, tui_widget::TuiWidget},
};
use packet_viewer::PacketViewer;
use pnet::datalink::{interfaces, NetworkInterface};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Layout,
    Frame,
};
use serde_json::Value;
use std::io::Result;

mod packet_viewer;

#[derive(Debug, Default)]
enum State {
    #[default]
    Initial,
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
    interfaces: Vec<NetworkInterface>,
    ft_ping_viewer: PacketViewer,
    ping_viewer: PacketViewer,
}

impl PacketCompareWidget {
    pub fn new() -> Self {
        Self {
            interfaces: interfaces(),
            ft_ping_viewer: PacketViewer::new(ViewerType::FtPing),
            ping_viewer: PacketViewer::new(ViewerType::Ping),
            ..Default::default()
        }
    }

    pub fn reset_test_index(&mut self) -> () {
        self.tests_idx = usize::default();
    }
}

use std::fs::OpenOptions;
use std::io::prelude::*;
use ratatui::prelude::Constraint;

impl TuiWidget for PacketCompareWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        if key_event.code == KeyCode::Char('q') {
            self.upper_state = Some(crate::app::State::Welcome);
            self.state = State::Initial;
            self.reset_test_index();
            self.summary_widget.clear_results();
        } else {
            match self.state {
                State::Initial => {} // State::Interactive => match key_event.code {
                //     KeyCode::Char(' ') => {
                //         if !self.running && !self.to_run {
                //             self.to_run = true;
                //             self.ft_ping_output_viewer.clear_buffers();
                //             self.ping_output_viewer.clear_buffers();
                //         }
                //     }
                //     _ => {}
                // },
                // State::Batch => {}
                State::Summary => {
                    self.summary_widget.process_input(key_event);
                }
            };
        }
    }

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("ciao.txt")?;

        if let Err(e) = writeln!(file, "{}", self.interfaces.len()) {
            eprintln!("Couldn't write to file: {}", e);
        }

        for iface in &self.interfaces {
            if let Err(e) = writeln!(file, "{}", iface.name) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }

        let [left_area, right_area] = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(frame.size());

        frame.render_widget(&self.ft_ping_viewer, left_area);
        frame.render_widget(&self.ping_viewer, right_area);
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
