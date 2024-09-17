mod info_widget;
mod intro_widget;
mod recompiling_notice;

use crate::app::widgets::common::commands_widget::CommandsWidget;
use crate::app::widgets::common::list_widget::ListWidget;
use crate::app::widgets::traits::tui_widget::TuiWidget;
use crate::app::State;
use info_widget::InfoWidget;
use intro_widget::IntroWidget;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    widgets::Clear,
    Frame,
};
use recompiling_notice::RecompilingNotice;

#[derive(Debug)]
pub struct WelcomeWidget {
    intro_widget: IntroWidget,
    select_test_widget: ListWidget,
    info_widget: InfoWidget,
    commands_widget: CommandsWidget,
    recompiling_notice: RecompilingNotice,
    pub recompiling: bool,
    to_clear: bool,
    state: Option<State>,
}

impl TuiWidget for WelcomeWidget {
    fn process_input(&mut self, key_event: KeyEvent) -> () {
        match key_event.code {
            KeyCode::Up => {
                if self.recompiling {
                    self.recompiling_notice.move_up();
                } else {
                    self.select_previous();
                }
            }
            KeyCode::Down => {
                if self.recompiling {
                    self.recompiling_notice.move_down();
                } else {
                    self.select_next();
                }
            }
            KeyCode::Enter => {
                self.state = Some(self.select_state());
            }
            _ => {}
        };
    }

    fn draw(&mut self, frame: &mut Frame) -> std::io::Result<()> {
        if self.to_clear() {
            frame.render_widget(Clear, frame.size());
            self.set_to_clear(false);
            return Ok(());
        }
        let [upper_area, lower_area, commands_area] = Layout::vertical([
            Constraint::Percentage(75),
            Constraint::Percentage(35),
            Constraint::Percentage(3),
        ])
        .areas(frame.size());
        frame.render_widget(&self.intro_widget, upper_area);

        let [lower_left_area, lower_right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(lower_area);
        let mut test = self.select_test_widget.get_state();
        frame.render_stateful_widget(&self.select_test_widget, lower_left_area, &mut test);

        frame.render_widget(&self.info_widget, lower_right_area);
        frame.render_widget(&self.commands_widget, commands_area);

        if self.recompiling {
            let center_h_area = Layout::horizontal([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .areas::<3>(frame.size())[1];
            let center_hv_area = Layout::vertical([
                Constraint::Percentage(35),
                Constraint::Percentage(30),
                Constraint::Percentage(35),
            ])
            .areas::<3>(center_h_area)[1];
            frame.render_widget(Clear, center_hv_area);
            frame.render_widget(&self.recompiling_notice, center_hv_area);
        }
        Ok(())
    }

    fn set_to_clear(&mut self, v: bool) -> () {
        self.to_clear = v;
    }

    fn to_clear(&self) -> bool {
        self.to_clear
    }
}

impl WelcomeWidget {
    pub fn new(path: String) -> Self {
        WelcomeWidget {
            select_test_widget: ListWidget::new(
                " Choose test: ".into(),
                vec![
                    "All tests".into(),
                    "Error handling tests".into(),
                    "Output tests".into(),
                    "Packets compliance tests".into(),
                    "Performance tests".into(),
                    "Recompile ft_ping".into(),
                ],
            ),
            recompiling_notice: RecompilingNotice::new(path),
            recompiling: bool::default(),
            intro_widget: IntroWidget::default(),
            info_widget: InfoWidget::default(),
            commands_widget: CommandsWidget::new(
                " ↑/↓: Move Up/Down | Enter: Select | Q: Exit ".to_string(),
            ),
            to_clear: true,
            state: None,
        }
    }

    pub fn select_previous(&mut self) {
        self.select_test_widget.select_previous();
    }

    pub fn select_next(&mut self) {
        self.select_test_widget.select_next();
    }

    pub fn recompile(&mut self, val: bool) {
        self.recompiling = val;
        if val {
            self.recompiling_notice.clear_output();
            self.recompiling_notice.start();
        }
    }

    pub fn select_state(&mut self) -> State {
        let index = self.select_test_widget.selected().unwrap();

        match index {
            0 => State::Welcome,
            1 => State::ErrorHandling,
            2 => State::OutputTests,
            3 => State::PacketTests,
            4 => State::PerformanceTests,
            5 => {
                self.recompile(true);
                State::Welcome
            }
            _ => State::Invalid,
        }
    }

    pub fn selected(&mut self) -> Option<State> {
        self.state.take()
    }
}
