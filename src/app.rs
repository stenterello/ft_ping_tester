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

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn  run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_inputs()?;
        }
        Ok(())
    }

    fn  render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn  handle_inputs(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Widget for &App {
    fn  render(self, area: Rect, buf: &mut Buffer) -> () {
        let title = Title::from(" ft_ping tester ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().into(),
            " Quit ".into(),
            "<Q> ".blue().into(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(instructions
                   .alignment(Alignment::Center)
                   .position(Position::Bottom)
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            " Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
