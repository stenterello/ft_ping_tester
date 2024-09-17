use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::block::BorderType;
use ratatui::widgets::Block;
use ratatui::widgets::LineGauge;
use ratatui::widgets::Widget;

#[derive(Debug, Default)]
pub struct ProcessingWidget {
    ratio: f64,
}

impl ProcessingWidget {
    pub fn set_ratio(&mut self, v: f64) -> () {
        self.ratio = v;
    }
}

impl Widget for &ProcessingWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        LineGauge::default()
            .block(block)
            .filled_style(Style::default().fg(Color::Yellow).bg(Color::White))
            .ratio(self.ratio)
            .render(area, buf);
    }
}
