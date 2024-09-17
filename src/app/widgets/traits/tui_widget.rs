use ratatui::crossterm::event::KeyEvent;
use ratatui::Frame;
use std::io::Result;

pub trait TuiWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> ();
    fn draw(&mut self, frame: &mut Frame) -> Result<()>;
    fn set_to_clear(&mut self, v: bool) -> ();
    fn to_clear(&self) -> bool;
}
