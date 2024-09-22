use ratatui::buffer::Buffer;
use ratatui::crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use tui_input::Input;
use crate::app::State;
use crate::app::widgets::traits::tui_widget::TuiWidget;

#[derive (Debug, Default)]
enum AuthenticationState {
    #[default]
    Editing,
    Trying,
    Error,
}

#[derive (Debug, Default)]
pub struct InputDialog {
    state: AuthenticationState,
    input_line: Input,
    input: String,
}

impl TuiWidget for InputDialog {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match &self.state {
            AuthenticationState::Editing => {},
            AuthenticationState::Trying => {},
            AuthenticationState::Error => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame) -> std::io::Result<()> {
        let input = Paragraph::new(self.input_line.value())
            .block(Block::default().borders(Borders::ALL).title("Input"));
        frame.render_widget(input, frame.size());
        Ok(())
    }

    fn set_to_clear(&mut self, v: bool) -> () {
        todo!()
    }

    fn to_clear(&self) -> bool {
        todo!()
    }

    fn state(&mut self) -> Option<State> {
        todo!()
    }

    fn commands_area(frame: &Frame) -> (Rect, Rect) {
        todo!()
    }
}