use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{
        block::{Block, BorderType, Title},
        Paragraph, Widget, Wrap,
    },
};

#[derive(Debug, Default)]
pub struct IntroWidget;

impl Widget for &IntroWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" ft_ping_tester ".bold().yellow());
        let description = Text::from(vec![
            Line::from(""),
            Line::from(" This is a simple ft_ping 42 project tester. I'm m0nt4lb4n0 and I divided the testing process into \
            four macrocathegories:"),
            Line::from(""),
            Line::from(" Error handling ".bold().yellow()),
            Line::from(" This part focuses on verifying the compliance with ping (inetutils-v2.0) behaviour in handling different errors."),
            Line::from(""),
            Line::from(" Output tester ".bold().yellow()),
            Line::from(" This part focuses on comparing the outputs from different command and options with your ft_ping and ping."),
            Line::from(""),
            Line::from(" Packets' compliance ".bold().yellow()),
            Line::from(" This part focuses on comparing the effective packets sent with the ICMP protocol."),
            Line::from(""),
            Line::from(" Performance tester ".bold().yellow()),
            Line::from(" This part focuses on comparing some specific traits from your implementation of ft_ping and ping, such as number of packets sent in a simple flood call."),
            Line::from(""),
            Line::from(""),
            Line::from(" Choose below the option to run all the tester consequencially or run only a specific part."),
        ]);

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .style(Style::default().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        Paragraph::new(description)
            .block(block)
            .wrap(Wrap { trim: true })
            .style(Style::default().bg(Color::Rgb(40, 44, 52)).fg(Color::White))
            .render(area.clone(), buf);
    }
}
