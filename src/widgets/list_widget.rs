use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{
        block::{BorderType, Title},
        Block, List, ListState, StatefulWidget,
    },
};

#[derive(Debug, Default)]
pub struct ListWidget {
    title: String,
    state: ListState,
    items: Vec<String>,
}

impl ListWidget {
    pub fn new(title: String, items: Vec<String>) -> Self {
        let mut ret = ListWidget {
            state: ListState::default(),
            title,
            items,
        };

        ret.state.select_first();
        ret
    }

    pub fn get_state(&self) -> ListState {
        self.state.clone()
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
}

impl StatefulWidget for &ListWidget {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title = Title::from(self.title.clone().bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        StatefulWidget::render(
            List::new(self.items.clone())
                .highlight_symbol(">> ")
                .highlight_style(
                    Style::default()
                        .fg(Color::Rgb(46, 52, 64))
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::ITALIC)
                        .add_modifier(Modifier::BOLD),
                )
                .style(Style::default().bg(Color::Rgb(46, 52, 64)).fg(Color::White))
                .block(block),
            area,
            buf,
            state,
        );
    }
}
