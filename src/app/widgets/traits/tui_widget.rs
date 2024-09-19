use ratatui::crossterm::event::KeyEvent;
use ratatui::Frame;
use std::io::Result;
use ratatui::layout::Rect;
use crate::app::State;

pub trait TuiWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> ();
    fn draw(&mut self, frame: &mut Frame) -> Result<()>;
    fn set_to_clear(&mut self, v: bool) -> ();
    fn to_clear(&self) -> bool;
    fn state(&mut self) -> Option<State> {
        None
    }
    fn commands_area(frame: &Frame) -> (Rect, Rect) {
        (
            Rect {
                x: frame.size().x,
                y: frame.size().y + frame.size().height - 1,
                width: frame.size().width,
                height: 1
            },
            Rect {
                x: frame.size().x,
                y: frame.size().y,
                width: frame.size().width,
                height: frame.size().height - 1
            }
        )
    }
}
