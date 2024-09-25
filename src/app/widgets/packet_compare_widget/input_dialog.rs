use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, BorderType, Clear, Padding, Paragraph};
use ratatui::widgets::block::Title;
use crate::app::State;
use crate::app::widgets::common::commands_widget::CommandsWidget;
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
    title: String,
    input: String,
    commands_widget: CommandsWidget
}

impl InputDialog {
    pub fn new(title: &str) -> Self {
        Self {
            state: AuthenticationState::default(),
            title: String::from(title),
            input: String::default(),
            commands_widget: CommandsWidget::new(" Esc: Back | Enter: Confirm "),
        }
    }
}

impl TuiWidget for InputDialog {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match &self.state {
            AuthenticationState::Editing => {
                match key_event.code {
                    KeyCode::Char(c) => {
                        if !c.is_control() || c.is_ascii_whitespace() {
                           self.input.push(c);
                        }
                    },
                    KeyCode::Backspace => {
                        self.input.pop();
                    },
                    _ => {}
                }
            },
            AuthenticationState::Trying => {},
            AuthenticationState::Error => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame) -> std::io::Result<()> {
        let mut center_dialog = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)]).areas::<3>(frame.size())[1];
        center_dialog = Layout::vertical([Constraint::Percentage(42), Constraint::Percentage(15), Constraint::Percentage(43)]).areas::<3>(center_dialog)[1];
        frame.render_widget(Clear, center_dialog);

        let (commands_area, _) = Self::commands_area(&frame);

        let block = Block::bordered()
            .title(Title::from(self.title.as_str()).alignment(Alignment::Center))
            .style(Style::default().fg(Color::Yellow))
            .padding(Padding::new(1, 1, 1, 1))
            .border_type(BorderType::Double).border_type(BorderType::Rounded);

        let mut secret: String = String::default();
        for _ in 0..self.input.len() {
            secret.push('*');
        }
        let input = Paragraph::new(secret)
            .alignment(Alignment::Center)
            .block(block);
        frame.render_widget(input, center_dialog);
        frame.render_widget(Clear, commands_area);
        frame.render_widget(&self.commands_widget, commands_area);

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
}