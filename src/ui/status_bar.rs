use crate::ui::{
    buffer::Buffer,
    layout::{Component, Rect},
    style::{Color, Style},
};

pub struct StatusBar {}

impl StatusBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for StatusBar {
    fn render(&self, area: Rect, buffer: &mut Buffer) {
        buffer.write_line(
            area.y(),
            " ".repeat(area.width()).into(),
            Style::new(Color::Reset, Color::Rgb(239, 239, 239)),
        );
    }
}
