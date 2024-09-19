use crate::app::widgets::common::commands_widget::CommandsWidget;
use crate::app::widgets::common::list_widget::{ListAlignment, ListWidget};
use crate::app::widgets::traits::tui_widget::TuiWidget;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::Frame;
use std::io::Result;

#[derive(Debug, Default, Clone)]
pub struct ChooseTestMethod {
    select_box: ListWidget,
    selected: Option<usize>,
    commands_widget: CommandsWidget,
}

impl ChooseTestMethod {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            select_box: ListWidget::new(" Test Method ".to_string(), items)
                .with_alignment(ListAlignment::Centered),
            selected: None,
            commands_widget: CommandsWidget::new(" ↑/↓: Move Up/Down | Enter: Select | Q: Back "),
        }
    }

    pub fn selected(&mut self) -> Option<usize> {
        self.selected.take()
    }
}

impl TuiWidget for ChooseTestMethod {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match key_event.code {
            KeyCode::Up => self.select_box.select_previous(),
            KeyCode::Down => self.select_box.select_next(),
            KeyCode::Enter => self.selected = self.select_box.selected(),
            _ => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        let (commands_area, area) = Self::commands_area(frame);
        let center_h_area = Layout::horizontal([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .areas::<3>(area)[1];
        let center_hv_area = Layout::vertical([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .areas::<3>(center_h_area)[1];
        let mut t = self.select_box.get_state();
        frame.render_stateful_widget(&self.select_box, center_hv_area, &mut t);
        frame.render_widget(
            &self.commands_widget,
            commands_area,
        );
        Ok(())
    }

    fn set_to_clear(&mut self, _v: bool) -> () {}
    fn to_clear(&self) -> bool {
        false
    }
}
