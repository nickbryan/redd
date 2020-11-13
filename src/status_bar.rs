use crate::{
    editor::Mode,
    ui::layout::{Component, Position, Rect},
    ui::style::{Color, Style},
    ui::FrameBuffer,
};

#[derive(Default)]
pub struct StatusBar {
    viewport: Rect,
    mode: Mode,
    line_count: usize,
    cursor_position: Position,
    file_name: String,
}

impl StatusBar {
    pub fn new(viewport: Rect) -> Self {
        Self {
            viewport,
            ..Self::default()
        }
    }

    pub fn update(
        &mut self,
        mode: Mode,
        line_count: usize,
        cursor_position: Position,
        file_name: &str,
    ) {
        self.mode = mode;
        self.line_count = line_count;
        self.cursor_position = cursor_position;
        self.file_name = file_name.into();
    }
}

impl Component for StatusBar {
    fn render(&self, buffer: &mut FrameBuffer) {
        let mut status = format!("Mode: [{}]    File: {}", self.mode, self.file_name);
        let line_indicator = format!(
            "L: {}/{} C: {}",
            self.cursor_position.y,
            self.line_count,
            self.cursor_position.x + 1
        );

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
