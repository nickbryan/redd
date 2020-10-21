use crate::ui::{
    buffer::Buffer,
    layout::{Component, Rect},
    style::{Color, Style},
};

pub struct StatusBar {
    file_name: String,
    lines: usize,
    current_line: usize,
}

impl StatusBar {
    pub fn new(mut file_name: String, lines: usize, current_line: usize) -> Self {
        file_name.truncate(20);

        Self {
            file_name,
            lines,
            current_line,
        }
    }
}

impl Component for StatusBar {
    fn render(&self, area: Rect, buffer: &mut Buffer) {
        let mut status = format!("File: {}", self.file_name);
        let line_indicator = format!("{}/{}", self.current_line, self.lines);

        let len = status.len() + line_indicator.len();

        if area.width() > len {
            status.push_str(&" ".repeat(area.width() - len));
        }

        status = format!("{}{}", status, line_indicator);
        status.truncate(area.width());

        buffer.write_line(
            area.y(),
            status,
            Style::new(Color::Rgb(63, 63, 63), Color::Rgb(239, 239, 239)),
        );
    }
}
