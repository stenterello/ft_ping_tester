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
use std::process::{Command, Stdio};
use sudo::RunningAs;
use crate::app::utils::config::config_extractor::Locations;
use crate::app::widgets::common::message_widget::MessageWidget;
use crate::app::widgets::common::processing_widget::ProcessingWidget;
use crate::app::widgets::common::thread_manager::ThreadManager;
use crate::app::widgets::packet_compare_widget::input_dialog::AuthenticationState;
use crate::app::widgets::traits::comparer::Comparer;
use crate::app::widgets::traits::thread_launcher::ThreadLauncher;
use crate::app::widgets::traits::thread_stringpuller::ThreadStringPuller;
use crate::app::widgets::traits::viewer::{OutputType, Viewer};

mod packet_viewer;
mod input_dialog;
const INTERCEPTOR_PATH: &str = "target/debug/";
const INTERCEPTOR_NAME: &str = "interceptor";

#[derive(Debug, Default, PartialEq)]
enum State {
    NoOp,
    RunningPing,
    RunningFtPing,
    #[default]
    PermissionCheck,
    _WaitingProcess,
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
    message_widget: MessageWidget,
    to_clear: bool,
    password_dialog: InputDialog,
    ft_ping_thread_mng: ThreadManager,
    ping_thread_mng: ThreadManager,
    interceptor: ThreadManager,
    ft_ping_viewer: PacketViewer,
    ping_viewer: PacketViewer,
    commands_widget: CommandsWidget,
    to_run: bool,
}

impl PacketCompareWidget {
    pub fn new(locations: &Locations, tests: Value) -> Self {
        let running = if let RunningAs::Root = sudo::check() {
            true
        } else if let true = Self::has_permissions() {
            true
        } else {
            false
        };
        Self {
            ft_ping_viewer: PacketViewer::new(PingType::FtPing),
            ping_viewer: PacketViewer::new(PingType::Ping),
            state: if running {
                State::NoOp
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
            message_widget: if running {
                MessageWidget::new()
            } else {MessageWidget::default()},
            to_clear: false,
            to_run: running,
            interceptor: ThreadManager::new(INTERCEPTOR_PATH, INTERCEPTOR_NAME)
        }
    }

    fn has_permissions() -> bool {
        let cmd = Command::new("sudo")
            .args(vec!["-n", "true"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        if let Some(0) = cmd.status.code() {
            true
        } else { false }
    }

    fn handle_state(&mut self) -> () {
        match &self.state {
            State::RunningPing => {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open("ciao.txt")
                    .unwrap();
                if !self.ping_thread_mng.is_running() && !self.interceptor.is_running() {
                    self.state = State::RunningFtPing;
                    if let Err(e) = writeln!(file, "Finished with {}", self.ping_thread_mng.get_exit_status().0.unwrap()) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                } else {
                    // if let Err(e) = writeln!(file, "ping running?: {}", self.ping_thread_mng.is_running()) {
                    //     eprintln!("Couldn't write to file: {}", e);
                    // }
                    // if let Err(e) = writeln!(file, "interceptor running?: {}", self.interceptor.is_running()) {
                    //     eprintln!("Couldn't write to file: {}", e);
                    // }
                    for i in self.interceptor.take_output(OutputType::Stdout) {
                        self.ping_viewer.add_packet(i.clone());
                        if let Err(e) = writeln!(file, "interceptor text: {}", i) {
                            eprintln!("Couldn't write to file: {}", e);
                        }
                    }
                }
            },
            State::RunningFtPing => {
                if !self.ft_ping_thread_mng.is_running() && !self.interceptor.is_running() {
                    self.state = State::PresentingResults;
                }
            }
            _ => {}
        }
    }

    pub fn reset_test_index(&mut self) -> () {
        self.tests_idx = usize::default();
    }
}

use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::app::widgets::packet_compare_widget::State::RunningPing;

impl TuiWidget for PacketCompareWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        if key_event.code == KeyCode::Char('q') && self.state != State::PermissionCheck {
            self.upper_state = Some(crate::app::State::Welcome);
            self.state = State::NoOp;
            self.reset_test_index();
            self.summary_widget.clear_results();
        } else {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("ciao.txt")
                .unwrap();
            match self.state {
                State::NoOp => {
                    self.state = State::RunningPing;
                    match key_event.code {
                        KeyCode::Char(' ') => {
                            match self.get_actual_test() {
                                Some(test) => {
                                    let arguments_vec = match test.as_array() {
                                        Some(s) => s
                                            .iter()
                                            .filter_map(|val| val.as_str().map(|s| s.to_string()))
                                            .collect(),
                                        None => Vec::new(),
                                    };

                                    if let Err(e) = writeln!(file, "Starting...") {
                                        eprintln!("Couldn't write to file: {}", e);
                                    }
                                    self.summary_widget().add_test(arguments_vec.join(" "));
                                    self.message_widget().set_arguments(arguments_vec.join(" "));
                                    self.interceptor.thread_mut().start(vec!["2".to_string()]);
                                    self.thread_mng_mut(PingType::Ping).start_process(arguments_vec);
                                    self.message_widget().set_running(true);
                                    self.state = RunningPing;
                                }
                                None => {
                                    self.set_finished();
                                }
                            }
                        }
                        _ => {}
                    }
                }
                State::PermissionCheck => {
                    match key_event.code {
                        KeyCode::Esc => {
                            self.upper_state = Some(crate::app::State::Welcome);
                            self.state = RunningPing;
                            self.reset_test_index();
                            self.summary_widget.clear_results();
                        }
                        _ => {
                            self.password_dialog.process_input(key_event);
                            match self.password_dialog.authentication_state() {
                                AuthenticationState::Success => {
                                    self.state = RunningPing;
                                    self.message_widget.set_running(true);
                                    self.to_run = true;
                                },
                                _ => {}
                            }
                        }
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

        self.handle_state();

        let (commands_area, area) = Self::commands_area(&frame);
        let [upper_area, status_area] =
            Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)])
                .areas(area);
        let [left_area, right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(upper_area);

        frame.render_widget(&self.ft_ping_viewer, left_area);
        frame.render_widget(&self.ping_viewer, right_area);
        frame.render_widget(&self.message_widget, status_area);
        frame.render_widget(Clear, commands_area);
        frame.render_widget(&self.commands_widget, commands_area);

        // let mut file = OpenOptions::new()
        //     .write(true)
        //     .append(true)
        //     .open("ciao.txt")
        //     .unwrap();
        match &self.state {
            State::PermissionCheck => self.password_dialog.draw(frame)?,
            State::RunningPing => {
                // if let Err(e) = writeln!(file, "quidentro") {
                //     eprintln!("Couldn't write to file: {}", e);
                // }
            }
            _ => {}
        }
        // if let Err(e) = writeln!(file, "qui fuori") {
        //     eprintln!("Couldn't write to file: {}", e);
        // }

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

impl Comparer for PacketCompareWidget {
    fn set_errors(&mut self, val: bool) -> () {
        self.message_widget.set_errors(val);
    }
}

impl ThreadStringPuller for PacketCompareWidget {
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
        todo!()
    }

    fn thread_mng_mut(&mut self, v: PingType) -> &mut ThreadManager {
        match v {
            PingType::FtPing => &mut self.ft_ping_thread_mng,
            PingType::Ping => &mut self.ping_thread_mng
        }
    }

    fn thread_mng(&self, v: PingType) -> &ThreadManager {
        match v {
            PingType::FtPing => &self.ft_ping_thread_mng,
            PingType::Ping => &self.ping_thread_mng
        }
    }

    fn viewer_mut(&mut self, v: PingType) -> &mut impl Viewer {
        match v {
            PingType::FtPing => &mut self.ft_ping_viewer,
            PingType::Ping => &mut self.ping_viewer
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

    fn clear_buffers(&mut self) -> () {
        todo!()
    }
}
