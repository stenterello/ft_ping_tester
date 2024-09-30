use crate::app::widgets::packet_compare_widget::packet_viewer::PacketField::*;
use crate::app::widgets::traits::thread_stringpuller::PingType;
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
use std::cell::RefCell;
use crate::app::utils::enums::TextType;
use crate::app::widgets::traits::viewer::Viewer;
use serde_json;

#[derive (Debug, Default, Clone)]
struct Packet {
    p_type: String,
    code: String,
    checksum: String,
    id: String,
    sequence: String,
    data: Vec<u8>
}

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
    packet: Option<Packet>,
}

impl Viewer for PacketViewer {
    fn set_text_to_display(&mut self, t: TextType) -> () {
        todo!()
    }

    fn set_error_to_display(&mut self, t: TextType) -> () {
        todo!()
    }
}

use std::fs::OpenOptions;
use std::io::prelude::*;
use serde_json::Value;

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

    pub fn add_packet(&mut self, s: String) -> () {
        let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open("ciao.txt")
                    .unwrap();
        if let Err(e) = writeln!(file, "string is {}", s) {
            eprintln!("Couldn't write to file: {}", e);
        }
        if let Ok(p) = serde_json::from_str::<Value>(&s) {
            self.packet = Some(Packet {
                p_type: p.get("type").unwrap().to_string(),
                code: p.get("code").unwrap().to_string(),
                checksum: p.get("checksum").unwrap().to_string(),
                id: p.get("id").unwrap().to_string(),
                sequence: p.get("sequence").unwrap().to_string(),
                data: vec![0],
                // data: p.get("data").unwrap().as_bytes().to_owned(),
            });
        }
    }

    fn generate_line(&self, rects: Vec<Rect>, l: LineEnum) -> () {
        let mut layout = self.layout.borrow_mut();
        match l {
            LineEnum::FirstLineLabel => {
                layout.type_label = rects[0];
                layout.code_label = rects[1];
                layout.checksum_label = rects[2];
            }
            LineEnum::FirstLineData => {
                layout.type_data = rects[0];
                layout.code_data = rects[1];
                layout.checksum_data = rects[2];
            }
            LineEnum::SecondLineLabel => {
                layout.id_label = rects[0];
                layout.sequence_label = rects[1];
            }
            LineEnum::SecondLineData => {
                layout.id_data = rects[0];
                layout.sequence_data = rects[1];
            }
            LineEnum::ThirdLineLabel => {
                layout.payload_label = rects[0];
            }
            LineEnum::ThirdLineData => {
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
            LineEnum::FirstLineLabel,
        );
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<3>(rows[1])),
            LineEnum::FirstLineData,
        );

        let constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<2>(rows[2])),
            LineEnum::SecondLineLabel,
        );
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<2>(rows[3])),
            LineEnum::SecondLineData,
        );

        let constraints = [Constraint::Percentage(100)];
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<1>(rows[4])),
            LineEnum::ThirdLineLabel,
        );
        self.generate_line(
            Vec::from(Layout::horizontal(constraints).areas::<1>(rows[5])),
            LineEnum::ThirdLineData,
        );
    }

    fn draw_paragraph(&self, f: PacketField, cell_block: Block, buf: &mut Buffer) -> () {
        let binding = match self.packet.clone() {
            Some(p) => p,
            None => Packet::default()
        };
        let (text, area) = match f {
            TypeLabel => ("type", self.layout.borrow().type_label),
            CodeLabel => ("code", self.layout.borrow().code_label),
            ChecksumLabel => ("checksum", self.layout.borrow().checksum_label),
            IdLabel => ("id", self.layout.borrow().id_label),
            SequenceLabel => ("sequence", self.layout.borrow().sequence_label),
            PayloadLabel => ("payload", self.layout.borrow().payload_label),
            TypeData => (binding.p_type.as_str(), self.layout.borrow().type_data),
            CodeData => (binding.code.as_str(), self.layout.borrow().code_data),
            ChecksumData => (binding.checksum.as_str(), self.layout.borrow().checksum_data),
            IdData => (binding.id.as_str(), self.layout.borrow().id_data),
            SequenceData => (binding.sequence.as_str(), self.layout.borrow().sequence_data),
            PayloadData => ("tmp", self.layout.borrow().payload_data),
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
        if let Some(_) = self.packet {
            self.draw_paragraph(TypeData, cell_block.clone(), buf);
            self.draw_paragraph(CodeData, cell_block.clone(), buf);
            self.draw_paragraph(ChecksumData, cell_block.clone(), buf);
            self.draw_paragraph(IdData, cell_block.clone(), buf);
            self.draw_paragraph(SequenceData, cell_block.clone(), buf);
            self.draw_paragraph(PayloadData, cell_block.clone(), buf);
        }
    }
}
