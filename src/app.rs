use crate::tui;
use std::io;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    style::Stylize,
    layout::{Alignment, Rect},
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

use color_eyre::{Result, eyre::{bail, WrapErr}};
use crate::stages;

#[derive(Debug, Default)]
pub struct App {
    welcome_widget: stages::welcome::WelcomeWidget,
    exit: bool,
}

impl App {
    pub fn  run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events().wrap_err("handle_events_failed");
        }
        Ok(())
    }

    fn  render(&self, frame: &mut Frame) {
        frame.render_widget(&self.welcome_widget, frame.size());
    }

    fn  handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event)
                    .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}"))
            }
            _ => Ok(())
        };
        Ok(())
    }

    fn  handle_key_events(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }

    fn  exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn  render(self, area: Rect, buf: &mut Buffer) -> () {
        let title = Title::from(" ft_ping tester ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(instructions
                   .alignment(Alignment::Center)
                   .position(Position::Bottom)
            )
            .border_set(border::THICK);
    }
}
