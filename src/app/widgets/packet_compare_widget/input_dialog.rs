use ratatui::buffer::Buffer;
use ratatui::crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};
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
        let mut center_dialog = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)]).areas::<3>(frame.size())[1];
        center_dialog = Layout::vertical([Constraint::Percentage(40), Constraint::Percentage(20), Constraint::Percentage(40)]).areas::<3>(center_dialog)[1];
        frame.render_widget(Clear, center_dialog);
        let input = Paragraph::new(self.input_line.value())
            .block(Block::default().borders(Borders::ALL).title("Input"));
        frame.render_widget(input, center_dialog);
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