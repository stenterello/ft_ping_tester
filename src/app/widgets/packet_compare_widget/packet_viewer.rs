use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    widgets::{block::Title, Block, BorderType, Row, Table, Widget},
};

use crate::app::widgets::traits::thread_stringpuller::ViewerType;

#[derive(Debug, Default)]
pub struct PacketViewer {
    name: String,
}

impl PacketViewer {
    pub fn new(t: ViewerType) -> Self {
        Self {
            name: match t {
                ViewerType::FtPing => String::from("ft_ping packet"),
                ViewerType::Ping => String::from("ping packet"),
            },
        }
    }
}

impl Widget for &PacketViewer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(&*self.name);
        let area1 = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height / 2,
        };
        let area2 = Rect {
            x: area.x,
            y: area.height / 2,
            width: area.width,
            height: area.height / 2,
        };
        let block = Block::bordered()
            .title(title.clone().alignment(Alignment::Center))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        let block2 = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        let rows = vec![
            Row::new(vec!["type", "code", "ICMP checksum"]),
            Row::new(vec!["ID", "Sequence"]),
            Row::new(vec!["Payload"]),
        ];
        let widths = vec![
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ];
        Table::new(rows.clone(), widths.clone())
            .block(block)
            .render(area1, buf);
        Table::new(rows.clone(), widths.clone())
            .block(block2)
            .render(area2, buf);
    }
}
