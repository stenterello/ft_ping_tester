use crate::app::widgets::packet_compare_widget::packet_viewer::LineEnum::{
    FirstLineData, FirstLineLabel, SecondLineData, SecondLineLabel, ThirdLineData, ThirdLineLabel,
};
use crate::app::widgets::packet_compare_widget::packet_viewer::PacketField::{
    ChecksumData, ChecksumLabel, CodeLabel, IdLabel, PayloadLabel, SequenceLabel, TypeLabel,
};
use crate::app::widgets::traits::thread_stringpuller::{PingType};
use ratatui::layout::Layout;
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Padding, Paragraph};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    widgets::{block::Title, Block, BorderType, Widget},
};
use std::cell::{RefCell};
use crate::app::utils::enums::TextType;
use crate::app::widgets::traits::viewer::Viewer;

enum LineEnum {
    FirstLineLabel,
    FirstLineData,
    SecondLineLabel,
    SecondLineData,
    ThirdLineLabel,
    ThirdLineData,
}

enum PacketField {
    TypeLabel,
    TypeData,
    CodeLabel,
    CodeData,
    ChecksumLabel,
    ChecksumData,
    IdLabel,
    IdData,
    SequenceLabel,
    SequenceData,
    PayloadLabel,
    PayloadData,
}

#[derive(Debug, Default)]
struct GridLayout {
    type_label: Rect,
    type_data: Rect,
    code_label: Rect,
    code_data: Rect,
    checksum_label: Rect,
    checksum_data: Rect,
    id_label: Rect,
    id_data: Rect,
    sequence_label: Rect,
    sequence_data: Rect,
    payload_label: Rect,
    payload_data: Rect,
}

#[derive(Debug, Default)]
pub struct PacketViewer {
    name: String,
    layout: RefCell<GridLayout>,
}

impl Viewer for PacketViewer {
    fn set_text_to_display(&mut self, t: TextType) -> () {
        todo!()
    }

    fn set_error_to_display(&mut self, t: TextType) -> () {
        todo!()
    }
}

impl PacketViewer {
    pub fn new(t: PingType) -> Self {
        Self {
            name: match t {
                PingType::FtPing => String::from("ft_ping packet"),
                PingType::Ping => String::from("ping packet"),
            },
            ..Default::default()
        }
    }

    fn generate_line(&self, rects: Vec<Rect>, l: LineEnum) -> () {
        let mut layout = self.layout.borrow_mut();
        match l {
            FirstLineLabel => {
                layout.type_label = rects[0];
                layout.code_label = rects[1];
                layout.checksum_label = rects[2];
            }
            FirstLineData => {
                layout.type_data = rects[0];
                layout.code_data = rects[1];
                layout.checksum_data = rects[2];
            }
            SecondLineLabel => {
                layout.id_label = rects[0];
                layout.sequence_label = rects[1];
            }
            SecondLineData => {
                layout.id_data = rects[0];
                layout.sequence_data = rects[1];
            }
            ThirdLineLabel => {
                layout.payload_label = rects[0];
            }
            ThirdLineData => {
                layout.payload_data = rects[0];
            }
        }
    }

    fn generate_layout(&self, area: Rect) -> () {
        let area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        };

        let rows = Layout::vertical([
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
        ])
        .areas::<6>(area);

        let constraints = [
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(50),
        ];
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<3>(rows[0])),
            FirstLineLabel,
        );
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<3>(rows[1])),
            FirstLineData,
        );

        let constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<2>(rows[2])),
            SecondLineLabel,
        );
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<2>(rows[3])),
            SecondLineData,
        );

        let constraints = [Constraint::Percentage(100)];
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<1>(rows[4])),
            ThirdLineLabel,
        );
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<1>(rows[5])),
            ThirdLineData,
        );
    }

    fn draw_paragraph(&self, f: PacketField, cell_block: Block, buf: &mut Buffer) -> () {
        let (text, area) = match f {
            TypeLabel => ("type", self.layout.borrow().type_label),
            CodeLabel => ("code", self.layout.borrow().code_label),
            ChecksumLabel => ("checksum", self.layout.borrow().checksum_label),
            IdLabel => ("id", self.layout.borrow().id_label),
            SequenceLabel => ("sequence", self.layout.borrow().sequence_label),
            PayloadLabel => ("payload", self.layout.borrow().payload_label),
            _ => ("", Rect::default()),
        };
        Paragraph::new(Line::from(Span::styled(
            text,
            Style::default().add_modifier(Modifier::BOLD),
        )))
        .block(cell_block)
        .centered()
        .render(area, buf);
    }
}

impl Widget for &PacketViewer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .title(Title::from(&*self.name).alignment(Alignment::Center))
            .style(Style::default().fg(Color::Yellow))
            .padding(Padding::new(1, 1, 1, 1))
            .border_type(BorderType::Rounded)
            .render(area, buf);

        self.generate_layout(area);

        let cell_block = Block::bordered()
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Double);

        self.draw_paragraph(TypeLabel, cell_block.clone(), buf);
        self.draw_paragraph(CodeLabel, cell_block.clone(), buf);
        self.draw_paragraph(ChecksumLabel, cell_block.clone(), buf);
        self.draw_paragraph(IdLabel, cell_block.clone(), buf);
        self.draw_paragraph(SequenceLabel, cell_block.clone(), buf);
        self.draw_paragraph(PayloadLabel, cell_block.clone(), buf);
    }
}
