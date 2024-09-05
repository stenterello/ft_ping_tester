use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub trait TuiWidget {
  fn process_input(&mut self, key_event: KeyEvent) -> ();
}
