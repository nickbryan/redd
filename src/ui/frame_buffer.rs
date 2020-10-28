use crate::{
    ui::layout::{Position, Rect},
    ui::style::Style,
};
use anyhow::Result;
use std::fmt::{Display, Formatter, Result as FmtResult};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    position: Position,
    symbol: String,
    style: Style,
}

impl Cell {
    pub fn new(x: usize, y: usize, symbol: &str, style: Style) -> Self {
        Self {
            position: Position::new(x, y),
            symbol: symbol.into(),
            style,
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn reset(&mut self) {
        self.symbol = " ".into();
    }

    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn style(&self) -> &Style {
        &self.style
    }
}

#[derive(Debug, Clone)]
pub struct OutOfBoundsError;

impl Display for OutOfBoundsError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "trying to access index out of bounds")
    }
}

pub struct FrameBuffer {
    area: Rect,
    cells: Vec<Cell>,
}

impl FrameBuffer {
    pub fn empty(area: Rect) -> Self {
        FrameBuffer::filled(area, " ")
    }

    pub fn filled(area: Rect, symbol: &str) -> Self {
        let size = area.area();
        let mut cells = Vec::with_capacity(size);

        for y in 0..area.height {
            for x in 0..area.width {
                cells.push(Cell::new(x, y, symbol, Style::default()));
            }
        }

        Self { cells, area }
    }

    pub fn diff<'a>(&self, other: &'a FrameBuffer) -> Vec<&'a Cell> {
        let front_buffer = &self.cells;
        let back_buffer = &other.cells;

        let mut updates = vec![];
        for (i, (front, back)) in back_buffer.iter().zip(front_buffer.iter()).enumerate() {
            if front != back {
                updates.push(&back_buffer[i]);
            }
        }

        updates
    }

    fn index_of(&self, position: &Position) -> Result<usize, OutOfBoundsError> {
        if self.area.contains(position) {
            Ok((position.y - self.area.position.y) * self.area.width
                + (position.x - self.area.position.x))
        } else {
            Err(OutOfBoundsError)
        }
    }

    pub fn reset(&mut self) {
        for cell in &mut self.cells {
            cell.reset();
        }
    }

    pub fn write_line(&mut self, line_number: usize, string: &str, style: &Style) {
        let index = self.index_of(&Position::new(0, line_number)).unwrap();

        for (i, grapheme) in string[..].graphemes(true).enumerate() {
            let cell_idx = index + i;
            self.cells[cell_idx] = Cell::new(
                self.cells[cell_idx].position.x,
                self.cells[cell_idx].position.y,
                &grapheme,
                style.clone(),
            );
        }

        for i in index + string[..].graphemes(true).count()..index + self.area.width {
            self.cells[i].reset();
        }
    }
}
