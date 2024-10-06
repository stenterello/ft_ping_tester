use std::io::Write;
use std::process::{exit, Command, Stdio};
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
pub enum AuthenticationState {
    #[default]
    Editing,
    _Trying,
    _Error,
    Success,
}

#[derive (Debug, Default)]
pub struct InputDialog {
    state: AuthenticationState,
    title: String,
    input: String,
    commands_widget: CommandsWidget
}

use std::fs::OpenOptions;

impl InputDialog {
    pub fn new(title: &str) -> Self {
        Self {
            state: AuthenticationState::default(),
            title: String::from(title),
            input: String::default(),
            commands_widget: CommandsWidget::new(" Esc: Back | Enter: Confirm "),
        }
    }

    fn excalate(&mut self) -> () {
        let mut cmd = Command::new("sudo")
            .args(vec!["-S", "true"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        cmd.stdin.take().unwrap().write(self.input.as_bytes()).unwrap();

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("ciao.txt")
            .unwrap();

        if let Ok(exit) = cmd.wait() {
            if let Some(0) = exit.code() {
                if let Err(e) = writeln!(file, "Exit status ok, permission got") {
                    eprintln!("Couldn't write to file: {}", e);
                }
                self.state = AuthenticationState::Success;
            } else {
                if let Err(e) = writeln!(file, "Error") {
                    eprintln!("Couldn't write to file: {}", e);
                }
            }
        } else {
            if let Err(e) = writeln!(file, "Error") {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }

    pub fn authentication_state(&self) -> &AuthenticationState {
        &self.state
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
                    KeyCode::Enter => {
                        self.excalate();
                    },
                    _ => {}
                }
            },
            _ => {}
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

    fn set_to_clear(&mut self, _v: bool) -> () {
        todo!()
    }

    fn to_clear(&self) -> bool {
        todo!()
    }

    fn state(&mut self) -> Option<State> {
        todo!()
    }
}
