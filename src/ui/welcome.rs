use crate::ui::{
    buffer::Buffer,
    layout::{Component, Rect},
    style::Style,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Screen {}

impl Component for Screen {
    fn render(&self, area: Rect, buffer: &mut Buffer) {
        for terminal_row in 0..area.height {
            if terminal_row == area.height / 3 {
                let mut welcome_message = format!("Vie editor -- version {}", VERSION);
                let len = welcome_message.len();
                let padding = area.width.saturating_sub(len) / 2;
                let spaces = " ".repeat(padding.saturating_sub(1));
                welcome_message = format!("~{}{}", spaces, welcome_message);
                welcome_message.truncate(area.width);
                buffer.write_line(terminal_row, &welcome_message, &Style::default());
            } else {
                buffer.write_line(terminal_row, "~", &Style::default());
            }
        }
    }
}
