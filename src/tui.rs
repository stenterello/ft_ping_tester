use std::io::{Result, Stdout, stdout};
use ratatui::{
    crossterm::{
        terminal::{
            enable_raw_mode,
            disable_raw_mode,
            EnterAlternateScreen,
            LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    backend::CrosstermBackend,
    Terminal,
};

pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn  new() -> Result<Tui> {
        let tui = Tui {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
        };

        Ok(tui)
    }

    pub fn  enter(&mut self) -> Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn  restore(&mut self) -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}