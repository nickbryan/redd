use crate::{
    ui::layout::{Position, Rect},
    ui::style::Style,
};
use anyhow::Result;
use std::fmt::{Display, Formatter, Result as FmtResult};

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

pub struct Buffer {
    area: Rect,
    cells: Vec<Cell>,
}

impl Buffer {
    pub fn empty(area: Rect) -> Self {
        Buffer::filled(area, " ")
    }

    pub fn filled(area: Rect, symbol: &str) -> Self {
        let size = area.area();
        let mut cells = Vec::with_capacity(size);

        for y in 0..area.height() {
            for x in 0..area.width() {
                cells.push(Cell::new(x, y, symbol, Style::default()));
            }
        }

        Self { cells, area }
    }

    pub fn diff<'a>(&self, other: &'a Buffer) -> Vec<&'a Cell> {
        let previous_buffer = &self.cells;
        let next_buffer = &other.cells;

        let mut updates = vec![];
        for (i, (current, previous)) in next_buffer.iter().zip(previous_buffer.iter()).enumerate() {
            if current != previous {
                updates.push(&next_buffer[i]);
            }
        }

        updates
    }

    fn index_of(&self, position: &Position) -> Result<usize, OutOfBoundsError> {
        if !self.area.contains(position) {
            Err(OutOfBoundsError)
        } else {
            Ok((position.y - self.area.y()) * self.area.width() + (position.x - self.area.x()))
        }
    }

    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.reset();
        }
    }

    pub fn write_line(&mut self, line_number: usize, string: String, style: Style) {
        let index = self.index_of(&Position::new(0, line_number)).unwrap();

        let mut string_index = 0;
        for i in index..index + string.len() {
            self.cells[i] = Cell::new(
                self.cells[i].position.x,
                self.cells[i].position.y,
                // TODO: this panics when using characters like – en dash (i assume it takes up
                // more space, thought the graphemes were handling this though)
                &string.chars().nth(string_index).unwrap().to_string(),
                style.clone(),
            );

            string_index += 1;
        }

        for i in index + string.len()..index + self.area.width() {
            self.cells[i].reset();
        }
    }
}
