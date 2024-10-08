mod utils;
mod widgets;

use crate::tui::Tui;
use ratatui::{
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    Frame,
};
use std::{io::Result, time::Duration};
use utils::config::config_extractor::{Config, ConfigExtractor};
use utils::config::test_config_extractor::TestConfigExtractor;
use widgets::error_handling::ErrorHandling;
use widgets::output_tests_widget::OutputTestsWidget;
use widgets::packet_compare_widget::PacketCompareWidget;
use widgets::traits::tui_widget::TuiWidget;
use widgets::welcome_widget::WelcomeWidget;

const CONF_FILE: &str = "./config.toml";

#[derive(Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Welcome,
    ErrorHandling,
    OutputTests,
    PacketTests,
    PerformanceTests,
    Exit,
}

#[derive(Debug)]
pub struct App {
    welcome_widget: WelcomeWidget,
    error_handling_widget: ErrorHandling,
    output_tests_widget: OutputTestsWidget,
    packet_compare_widget: PacketCompareWidget,
    state: State,
    about_to_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let config: Config = match ConfigExtractor::decode(CONF_FILE) {
            Ok(config) => config,
            Err(e) => return Err(e),
        };
        match TestConfigExtractor::decode(&config.locations.test_conf_path) {
            Ok(tests) => Ok(App {
                welcome_widget: WelcomeWidget::new(&config.locations.ft_ping_dir),
                error_handling_widget: ErrorHandling::new(
                    &config.locations,
                    tests["error_handling"].clone(),
                ),
                output_tests_widget: OutputTestsWidget::new(
                    &config.locations,
                    tests["output_tests"].clone(),
                ),
                packet_compare_widget: PacketCompareWidget::new(
                    &config.locations,
                    tests["packet_tests"].clone(),
                ),
                state: State::default(),
                about_to_quit: false,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn run(&mut self, tui: &mut Tui) -> Result<()> {
        let mut result: Result<()> = Ok(());
        loop {
            tui.terminal.draw(|frame| result = self.render(frame))?;
            if let Err(_) = result {
                return result;
            }
            self.handle_events()?;
            if self.about_to_quit == true {
                break;
            }
        }
        result
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_secs(0))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event)?;
                }
            };
        }
        Ok(())
    }

    fn update_state(&mut self) -> () {
        let captured_state: Option<State> = match self.state {
            State::Welcome => self.welcome_widget.state(),
            State::ErrorHandling => self.error_handling_widget.state(),
            State::OutputTests => self.output_tests_widget.state(),
            State::PacketTests => self.packet_compare_widget.state(),
            State::PerformanceTests => None,
            State::Exit => Some(State::Exit),
        };

        if let Some(state) = captured_state {
            self.state = state;
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.state {
            State::Welcome => self.welcome_widget.process_input(key_event),
            State::ErrorHandling => self.error_handling_widget.process_input(key_event),
            State::OutputTests => self.output_tests_widget.process_input(key_event),
            State::PacketTests => self.packet_compare_widget.process_input(key_event),
            State::PerformanceTests => todo!(),
            State::Exit => {}
        };

        self.update_state();
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) -> Result<()> {
        match self.state {
            State::Welcome => self.welcome_widget.draw(frame),
            State::ErrorHandling => self.error_handling_widget.draw(frame),
            State::OutputTests => self.output_tests_widget.draw(frame),
            State::PacketTests => self.packet_compare_widget.draw(frame),
            State::PerformanceTests => todo!(),
            State::Exit => {
                self.about_to_quit = true;
                Ok(())
            }
        }
    }
}
