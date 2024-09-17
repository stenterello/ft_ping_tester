use ratatui::crossterm::event::KeyEvent;
use ratatui::Frame;
use std::io::Result;

pub trait TuiWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> ();
    fn draw(&mut self, frame: &mut Frame) -> Result<()>;
}
