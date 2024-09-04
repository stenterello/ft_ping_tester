mod info_widget;
mod intro_widget;
mod recompiling_notice;

use crate::app::app::State;
use crate::widgets::list_widget::ListWidget;
use info_widget::InfoWidget;
use intro_widget::IntroWidget;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Clear,
    Frame,
};
use recompiling_notice::RecompilingNotice;

#[derive(Default, Debug)]
pub struct WelcomeWidget {
    intro_widget: IntroWidget,
    select_test_widget: ListWidget,
    info_widget: InfoWidget,
    recompiling_notice: RecompilingNotice,
    pub recompiling: bool,
}

impl WelcomeWidget {
    pub fn new() -> Self {
        WelcomeWidget {
            select_test_widget: ListWidget::new(
                " Choose test: ".into(),
                vec![
                    "All tests".into(),
                    "Error handling tests".into(),
                    "Output tests".into(),
                    "Performance tests".into(),
                    "Recompile ft_ping".into(),
                ],
            ),
            ..Default::default()
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        // Intro Widget
        let [upper_area, lower_area] =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(frame.size());
        frame.render_widget(&self.intro_widget, upper_area);

        // Select Test Widget
        let [lower_left_area, lower_right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(lower_area);
        let mut test = self.select_test_widget.get_state();
        frame.render_stateful_widget(&self.select_test_widget, lower_left_area, &mut test);

        // Info Widget
        frame.render_widget(&self.info_widget, lower_right_area);

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
    }

    pub fn select_previous(&mut self) {
        self.select_test_widget.select_previous();
    }

    pub fn select_next(&mut self) {
        self.select_test_widget.select_next();
    }

    pub fn recompile(&mut self) {
        self.recompiling = true;
        // recompile command
    }

    pub fn select_state(&mut self) -> State {
        let index = self.select_test_widget.selected().unwrap();

        match index {
            0 => State::Welcome,
            1 => State::ErrorHandling,
            4 => {
                self.recompile();
                State::Welcome
            }
            _ => State::Invalid,
        }
    }
}
