use crate::editor::Mode;
use crate::ui::{
    layout::{Component, Rect},
    style::{Color, Style},
    FrameBuffer,
};

pub struct StatusBar {
    file_name: String,
    lines: usize,
    current_line: usize,
    viewport: Rect,
    mode: Mode,
}

impl StatusBar {
    pub fn new(
        viewport: Rect,
        mut file_name: String,
        lines: usize,
        current_line: usize,
        mode: Mode,
    ) -> Self {
        file_name.truncate(20);

        Self {
            file_name,
            lines,
            current_line,
            viewport,
            mode,
        }
    }
}

impl Component for StatusBar {
    fn render(&self, buffer: &mut FrameBuffer) {
        let mut status = format!("Mode: [{}]    File: {}", self.mode, self.file_name);
        let line_indicator = format!("{}/{}", self.current_line, self.lines);

        let len = status.len() + line_indicator.len();

        if self.viewport.width > len {
            status.push_str(&" ".repeat(self.viewport.width - len));
        }

        status = format!("{}{}", status, line_indicator);
        status.truncate(self.viewport.width);

        buffer.write_line(
            self.viewport.top(),
            &status,
            &Style::new(Color::Rgb(63, 63, 63), Color::Rgb(239, 239, 239)),
        );
    }
}
