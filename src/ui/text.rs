use crate::{
    document::Document,
    ui::layout::{Component, Position, Rect},
    ui::style::Style,
    ui::FrameBuffer,
};

pub struct DocumentView<'a> {
    document: &'a Document,
    offset: Position,
}

impl<'a> DocumentView<'a> {
    pub fn new(document: &'a Document, offset: Position) -> Self {
        Self { document, offset }
    }
}

impl<'a> Component for DocumentView<'a> {
    fn render(&self, area: Rect, buffer: &mut FrameBuffer) {
        for terminal_row in 0..area.height {
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                let start = self.offset.x;
                let end = self.offset.x + area.width;
                let row = row.to_string(start, end);
                buffer.write_line(terminal_row, &row, &Style::default());
            } else {
                buffer.write_line(terminal_row, "~", &Style::default());
            }
        }
    }
}
