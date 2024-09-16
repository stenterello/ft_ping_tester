use ratatui::crossterm::event::KeyEvent;

pub trait TuiWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> ();
}
