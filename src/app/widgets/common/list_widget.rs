use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::text::Text;
use ratatui::widgets::ListItem;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{
        block::Title,
        List, ListState, StatefulWidget,
    },
};

use super::default_style::DefaultStyle;

#[derive(Debug, Default, Clone)]
pub enum ListAlignment {
    #[default]
    Standard,
    Centered,
}

#[derive(Debug, Default, Clone)]
pub struct ListWidget {
    title: String,
    state: ListState,
    items: Vec<String>,
    alignment: ListAlignment,
}

impl ListWidget {
    pub fn new(title: String, items: Vec<String>) -> Self {
        let mut ret = ListWidget {
            state: ListState::default(),
            title,
            items,
            alignment: ListAlignment::default(),
        };

        ret.state.select_first();
        ret
    }

    pub fn get_state(&self) -> ListState {
        self.state.clone()
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn select_next(&mut self) {
        if self.state.selected().unwrap() != self.items.len() - 1 {
            self.state.select_next();
        }
    }

    pub fn selected(&mut self) -> Option<usize> {
        self.state.selected()
    }

    pub fn with_alignment(mut self, alignment: ListAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
        let [area] = Layout::horizontal([horizontal])
            .flex(Flex::Center)
            .areas(area);
        let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
        area
    }
}

impl StatefulWidget for &ListWidget {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = DefaultStyle::block(Title::from(self.title.clone().bold()));

        let correct_area: Rect = match self.alignment {
            ListAlignment::Standard => area,
            ListAlignment::Centered => {
                ListWidget::center(area, Constraint::Percentage(50), Constraint::Percentage(50))
            }
        };

        let items: Vec<ListItem> = match self.alignment {
            ListAlignment::Standard => self
                .items
                .iter()
                .map(|s| ListItem::from(Text::from(s.clone())))
                .collect(),
            ListAlignment::Centered => self
                .items
                .iter()
                .map(|s| ListItem::from(Text::from(s.clone()).alignment(Alignment::Center)))
                .collect(),
        };

        StatefulWidget::render(
            List::new(items)
                .highlight_symbol(match self.alignment {
                    ListAlignment::Standard => ">> ",
                    ListAlignment::Centered => "",
                })
                .highlight_style(
                    Style::default()
                        .fg(Color::Rgb(46, 52, 64))
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::ITALIC)
                        .add_modifier(Modifier::BOLD),
                )
                .style(DefaultStyle::style())
                .block(block),
            correct_area,
            buf,
            state,
        );
    }
}
