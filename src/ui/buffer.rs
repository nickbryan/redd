use crate::ui::layout::{Position, Rect};

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    position: Position,
    symbol: String,
}

impl Cell {
    pub fn new(x: usize, y: usize, symbol: &str) -> Self {
        Self {
            position: Position::new(x, y),
            symbol: symbol.into(),
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
}

pub struct Buffer {
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
                cells.push(Cell::new(x, y, symbol));
            }
        }

        Self { cells }
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

    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.reset();
        }
    }
}
