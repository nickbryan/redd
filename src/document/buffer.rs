use crate::{
    document::Document,
    io::event::Key,
    ui::{
        layout::{Component, Position, Rect},
        style::Style,
        FrameBuffer,
    },
};
use anyhow::{Context, Result};

pub struct Buffer {
    document: Document,
    viewport: Rect,
    cursor_position: Position,
    offset: Position,
}

impl Buffer {
    pub fn new(document: Document, viewport: Rect) -> Self {
        Self {
            document,
            viewport,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }

    pub fn document_name(&self) -> String {
        self.document
            .file_name()
            .unwrap_or(&"[No Name]".to_string())
            .clone()
    }

    pub fn cursor_position(&self) -> Position {
        Position::new(
            self.cursor_position.x.saturating_sub(self.offset.x),
            self.cursor_position.y.saturating_sub(self.offset.y),
        )
    }

    pub fn lines_in_document(&self) -> usize {
        self.document.len()
    }

    pub fn proccess_keypress(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Ctrl('s') => self.document.save().context("unable to save document")?,
            Key::Char(ch) => {
                self.document
                    .insert(&self.cursor_position, ch)
                    .context("unable to insert character in document")?;

                self.move_cursor(Key::Right)
                    .context("unable to move cursor to the right")?;
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left)
                        .context("unable to move cursor to the left")?;
                    self.document.delete(&self.cursor_position);
                }
            }
            Key::Enter => {
                self.document.insert_newline(&self.cursor_position);
                self.move_cursor(Key::Down)
                    .context("unable to move to new line")?;
                self.move_cursor(Key::Home)
                    .context("unable to move to start of new line")?;
            }
            _ => {
                self.move_cursor(key).context("unable to move cursor")?;
            }
        };

        self.scroll()
    }

    fn move_cursor(&mut self, key: Key) -> Result<()> {
        use crate::document::Row;

        let terminal_height = self.viewport.height - 2;
        let Position { x, y } = self.cursor_position;
        let height = self.document.len();
        let width = self.document.row(y).map_or(0, Row::len);

        let (x, y) = match key {
            Key::Up => (x, y.saturating_sub(1)),
            Key::Down => {
                if y < height {
                    (x, y.saturating_add(1))
                } else {
                    (x, y)
                }
            }
            Key::Left => {
                if x > 0 {
                    (x - 1, y)
                } else if y > 0 {
                    self.document
                        .row(y)
                        .map_or((0, y - 1), |row| (row.len(), y - 1))
                } else {
                    (x, y)
                }
            }
            Key::Right => {
                if x < width {
                    (x + 1, y)
                } else if y < height {
                    (0, y + 1)
                } else {
                    (x, y)
                }
            }
            Key::PageUp => {
                if y > terminal_height {
                    (x, y - terminal_height)
                } else {
                    (x, 0)
                }
            }
            Key::PageDown => {
                if y.saturating_add(terminal_height) < height {
                    (x, y + terminal_height)
                } else {
                    (x, height)
                }
            }
            Key::Home => (0, y),
            Key::End => (width, y),
            _ => (x, y),
        };

        let new_width = self.document.row(y).map_or(0, Row::len);

        self.cursor_position = Position {
            x: if x > new_width { new_width } else { x },
            y,
        };

        Ok(())
    }

    pub fn scroll(&mut self) -> Result<()> {
        let Position { x, y } = self.cursor_position;
        let width = self.viewport.width;
        let height = self.viewport.height - 2;

        let offset = if y < self.offset.y {
            (self.offset.x, y)
        } else if y >= self.offset.y.saturating_add(height) {
            (self.offset.x, y.saturating_sub(height).saturating_add(1))
        } else {
            (self.offset.x, self.offset.y)
        };

        let offset = if x < self.offset.x {
            (x, offset.1)
        } else if x >= self.offset.x.saturating_add(width) {
            (x.saturating_add(width).saturating_add(1), offset.1)
        } else {
            (self.offset.x, offset.1)
        };

        self.offset = Position::from(offset);

        Ok(())
    }
}

impl Component for Buffer {
    fn render(&self, buffer: &mut FrameBuffer) {
        for terminal_row in 0..self.viewport.height {
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                let start = self.offset.x;
                let end = self.offset.x + self.viewport.width;
                let row = row.to_string(start, end);
                buffer.write_line(terminal_row, &row, &Style::default());
            } else {
                buffer.write_line(terminal_row, "~", &Style::default());
            }
        }
    }
}
