mod utils;
mod widgets;

use crate::tui::Tui;
use ratatui::{
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    Frame,
};
use std::{
    io::{Error, ErrorKind, Result},
    time::Duration,
};
use utils::config::config_extractor::{ConfigExtractor, ConfigValues};
use utils::config::test_config_extractor::TestConfigExtractor;
use widgets::error_handling::ErrorHandling;
use widgets::output_tests_widget::OutputTestsWidget;
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
    Invalid,
}

#[derive(Debug)]
pub struct App {
    welcome_widget: WelcomeWidget,
    error_handling_widget: ErrorHandling,
    output_tests_widget: OutputTestsWidget,
    state: State,
    about_to_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = ConfigExtractor::decode(CONF_FILE.into());
        if !config.valid {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid paths in config.toml",
            ));
        }
        let config: ConfigValues = config.config.unwrap();
        let tests = TestConfigExtractor::decode(config.locations.test_conf_path.clone());
        Ok(App {
            welcome_widget: WelcomeWidget::new(config.locations.ft_ping_dir.clone()),
            error_handling_widget: ErrorHandling::new(
                config.locations.clone(),
                tests["error_handling"].clone(),
            ),
            output_tests_widget: OutputTestsWidget::new(
                config.locations,
                tests["output_tests"].clone(),
            ),
            state: State::default(),
            about_to_quit: false,
        })
    }

    pub fn run(&mut self, tui: &mut Tui) -> Result<()> {
        let mut error: Option<Result<()>> = None;
        loop {
            tui.terminal.draw(|frame| match self.render(frame) {
                Ok(_) => {}
                Err(e) => error = Some(Err(e)),
            })?;
            match error {
                Some(e) => {
                    return e;
                }
                None => {}
            }
            self.handle_events()?;
            if self.about_to_quit == true {
                break;
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_secs(0))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)?;
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn update_state(&mut self) -> () {
        match self.state {
            State::Welcome => {
                if let Some(state) = self.welcome_widget.state() {
                    self.state = state;
                }
            }
            State::ErrorHandling => {
                if let Some(state) = self.error_handling_widget.state() {
                    self.state = state;
                }
            }
            State::OutputTests => {
                if let Some(state) = self.output_tests_widget.state() {
                    self.state = state;
                }
            }
            State::PacketTests | State::PerformanceTests => todo!(),
            State::Invalid | State::Exit => {}
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.state {
            State::Welcome => self.welcome_widget.process_input(key_event),
            State::ErrorHandling => self.error_handling_widget.process_input(key_event),
            State::OutputTests => self.output_tests_widget.process_input(key_event),
            State::PacketTests | State::PerformanceTests => todo!(),
            State::Invalid | State::Exit => {}
        };

        self.update_state();
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) -> Result<()> {
        match self.state {
            State::Welcome => self.welcome_widget.draw(frame),
            State::ErrorHandling => self.error_handling_widget.draw(frame),
            State::OutputTests => self.output_tests_widget.draw(frame),
            State::PacketTests | State::PerformanceTests | State::Invalid => todo!(),
            State::Exit => {
                self.exit();
                Ok(())
            }
        }
    }

    fn exit(&mut self) {
        self.about_to_quit = true;
    }
}
