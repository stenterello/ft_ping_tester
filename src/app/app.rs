use crate::traits::tui_widget_trait::TuiWidget;
use crate::tui::Tui;
use crate::utils::config_extractor::{ConfigExtractor, ConfigValues};
use crate::utils::test_config_extractor::TestConfigExtractor;
use crate::widgets::error_handling::ErrorHandling;
use crate::widgets::welcome_widget::WelcomeWidget;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    Frame,
};
use std::{
    io::{Error, ErrorKind, Result},
    time::Duration,
};

const CONF_FILE: &str = "./conf.toml";

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Welcome,
    ErrorHandling,
    Invalid,
}

#[derive(Debug)]
pub struct App {
    welcome_widget: WelcomeWidget,
    error_handling_widget: ErrorHandling,
    state: State,
    about_to_quit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = ConfigExtractor::decode(CONF_FILE.into());
        if !config.valid {
            return Err(Error::new(ErrorKind::Other, "Invalid paths in conf.toml"));
        }
        let config: ConfigValues = config.config.unwrap();
        let tests = TestConfigExtractor::decode(config.locations.test_conf_path.into());
        Ok(App {
            welcome_widget: WelcomeWidget::new(config.locations.ft_ping_dir.into()),
            error_handling_widget: ErrorHandling::new(tests["error_handling"].clone()),
            state: State::default(),
            about_to_quit: false,
        })
    }

    pub fn run(&mut self, tui: &mut Tui) -> Result<()> {
        loop {
            tui.terminal.draw(|frame| self.render(frame))?;
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

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => {
                if self.welcome_widget.recompiling {
                    self.welcome_widget.recompile(false);
                } else {
                    self.exit();
                }
            }
            KeyCode::Enter => self.select(),
            _ => {
                match self.state {
                    State::Welcome => self.welcome_widget.process_input(key_event),
                    State::ErrorHandling => self.error_handling_widget.process_input(key_event),
                    State::Invalid => {}
                };
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        match self.state {
            State::Welcome => self.welcome_widget.draw(frame),
            State::ErrorHandling => self.error_handling_widget.draw(frame),
            _ => {}
        };
    }

    fn select(&mut self) {
        if self.welcome_widget.recompiling {
            return;
        }

        match &self.state {
            State::Welcome => self.state = self.welcome_widget.select_state() as State,
            _ => {}
        };
    }

    fn exit(&mut self) {
        self.about_to_quit = true;
    }
}
