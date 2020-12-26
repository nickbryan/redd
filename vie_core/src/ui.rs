/// A part of the ui that can be rendered to the screen. This allows for widgeits/components to be
/// able to be responsible for their own drawing to the current frame buffer.
pub trait Component {
    fn render(&self, buffer: &mut frame::Buffer);
}

/// A position in ui space.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    /// Create a new Position.
    pub fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }
}

/// Rect represents an area/container in the ui.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Rect {
    pub width: usize,
    pub height: usize,
    pub position: Position,
}

impl Rect {
    /// Create a new Rect with default Position (0, 0).
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            position: Position::default(),
        }
    }

    /// Create a new Rect with a set Position.
    pub fn positioned(width: usize, height: usize, col: usize, row: usize) -> Self {
        Self {
            width,
            height,
            position: Position::new(col, row),
        }
    }

    /// Returns the area of the Rect.
    pub fn area(&self) -> usize {
        self.width.saturating_mul(self.height)
    }

    /// Returns the leftmost possible value of the Rect.
    pub fn left(&self) -> usize {
        self.position.col
    }

    /// Returns the rightmost possible value of the Rect.
    pub fn right(&self) -> usize {
        self.position.col + self.width
    }

    /// Returns the topmost possible value of the Rect.
    pub fn top(&self) -> usize {
        self.position.row
    }

    /// Returns the bottommost possible value of the Rect.
    pub fn bottom(&self) -> usize {
        self.position.row + self.height
    }

    /// Check if the given position is within the Rect, taking the Rect's Position into
    /// consideration.
    pub fn contains(&self, position: &Position) -> bool {
        let Position { col, row } = *position;

        col >= self.left() && col < self.right() && row >= self.top() && row < self.bottom()
    }
}

#[cfg(test)]
pub(crate) mod testutil {
    use super::{frame, Component, Style};

    pub(crate) struct MockComponent {
        lines: Vec<String>,
    }

    impl MockComponent {
        pub(crate) fn new() -> Self {
            Self { lines: Vec::new() }
        }

        pub(crate) fn add_line(&mut self, text: &str) {
            self.lines.push(text.into());
        }
    }

    impl Component for MockComponent {
        fn render(&self, buffer: &mut frame::Buffer) {
            for (i, line) in self.lines.iter().enumerate() {
                buffer.write_line(i, line, &Style::default());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Position, Rect};

    #[test]
    fn new_sets_default_position() {
        let r = Rect::new(0, 0);
        assert_eq!(r.position.col, 0);
        assert_eq!(r.position.row, 0);
    }

    #[test]
    fn positioned_sets_position() {
        let r = Rect::positioned(0, 0, 10, 20);
        assert_eq!(r.position.col, 10);
        assert_eq!(r.position.row, 20);
    }

    #[test]
    fn area_is_calculated() {
        assert_eq!(Rect::new(10, 10).area(), 100);
    }

    #[test]
    fn left_returns_leftmost_possible_value() {
        assert_eq!(Rect::positioned(5, 10, 20, 25).left(), 20);
    }

    #[test]
    fn right_returns_rightmost_possible_value() {
        assert_eq!(Rect::positioned(5, 10, 20, 25).right(), 25);
    }

    #[test]
    fn top_returns_topmost_possible_value() {
        assert_eq!(Rect::positioned(5, 10, 20, 25).top(), 25);
    }

    #[test]
    fn bottom_returns_bottommost_possible_value() {
        assert_eq!(Rect::positioned(5, 10, 20, 25).bottom(), 35);
    }

    #[test]
    fn contains_returns_true_if_position_contained() {
        let r = Rect::new(10, 10);
        assert!(r.contains(&Position::new(5, 5)));
    }

    #[test]
    fn contains_returns_false_if_position_not_contained() {
        let r = Rect::positioned(10, 10, 10, 10);
        assert_eq!(r.contains(&Position::new(5, 5)), false);
    }
}

/// Colors supported by the editor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    AnsiValue(u8),
}

/// Style encapsulates the foreground and background color of a cell.
#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub foreground: Color,
    pub background: Color,
}

impl Style {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            foreground: Color::Reset,
            background: Color::Reset,
        }
    }
}

pub mod frame {
    use super::{Position, Rect, Style};
    use anyhow::Result;
    use thiserror::Error;
    use unicode_segmentation::UnicodeSegmentation;

    /// A single cell within the frame (viewport). Each cell has a position, symbol (the shown
    /// character) and style.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Cell {
        position: Position,
        symbol: String,
        style: Style,
    }

    impl Cell {
        /// Create a new Cell.
        pub fn new(col: usize, row: usize, symbol: &str, style: Style) -> Self {
            Self {
                position: Position::new(col, row),
                symbol: symbol.into(),
                style,
            }
        }

        /// Returns the Position of the Cell.
        pub fn position(&self) -> &Position {
            &self.position
        }

        /// Reset the Cell's symbol to an empty space.
        pub fn reset(&mut self) {
            self.symbol = " ".into();
        }

        /// Returns the Cell's symbol.
        pub fn symbol(&self) -> &String {
            &self.symbol
        }

        /// Returns the Cell's style.
        pub fn style(&self) -> &Style {
            &self.style
        }
    }

    /// Raised by the Buffer when trying to access a cell that is out of bounds.
    #[derive(Error, Debug)]
    #[error("trying to access index out of bounds")]
    pub struct OutOfBoundsError;

    /// A mapping of Cells for a given area.
    ///
    /// All drawing within the editor will be mapped to a buffer. The buffer can then be diffed
    /// with another buffer to detect changes that occured within the last draw loop. This allows
    /// for more efficient rendering as we only need to update changed cells and not the entire
    /// screen.
    pub struct Buffer {
        area: Rect,
        cells: Vec<Cell>,
    }

    impl Buffer {
        /// Create a Buffer with all Cells having the symbol " ".
        pub fn empty(area: Rect) -> Self {
            Self::filled(area, " ")
        }

        /// Create a Buffer with all Cells set to the given symbol.
        pub fn filled(area: Rect, symbol: &str) -> Self {
            let size = area.area();
            let mut cells = Vec::with_capacity(size);

            for row in 0..area.height {
                for col in 0..area.width {
                    cells.push(Cell::new(col, row, symbol, Style::default()));
                }
            }

            Self { cells, area }
        }

        /// Diff the current Buffer with the other Buffer to get a list of changed Cells.
        pub fn diff<'a>(&self, other: &'a Buffer) -> Vec<&'a Cell> {
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
                Ok((position.row - self.area.position.row) * self.area.width
                    + (position.col - self.area.position.col))
            } else {
                Err(OutOfBoundsError)
            }
        }

        /// Reset the Buffer to it's empty state.
        pub fn reset(&mut self) {
            for cell in &mut self.cells {
                cell.reset();
            }
        }

        /// Write a line into the Buffer with the given style. This will overwrite any Cells
        /// currently set in the Buffer's given line. If the string does not fill the line it, the
        /// rest of the line will be cleared.
        pub fn write_line(&mut self, line_number: usize, string: &str, style: &Style) {
            let index = self.index_of(&Position::new(0, line_number)).unwrap();

            for (i, grapheme) in string[..].graphemes(true).enumerate() {
                let cell_idx = index + i;
                self.cells[cell_idx] = Cell::new(
                    self.cells[cell_idx].position.col,
                    self.cells[cell_idx].position.row,
                    &grapheme,
                    style.clone(),
                );
            }

            for i in index + string[..].graphemes(true).count()..index + self.area.width {
                self.cells[i].reset();
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::{Rect, Style};
        use super::{Buffer, Cell};

        fn assert_diff(diff: Vec<&Cell>, expected: Vec<Cell>) {
            for (a, b) in diff.iter().zip(expected.iter()) {
                assert_eq!(*a, b);
            }
        }

        #[test]
        fn empty_buffers_have_no_diff() {
            let front = Buffer::empty(Rect::new(5, 5));
            let back = Buffer::empty(Rect::new(5, 5));

            assert!(front.diff(&back).is_empty());
        }

        #[test]
        fn identical_filled_buffers_have_no_diff() {
            let front = Buffer::filled(Rect::new(5, 5), "A");
            let back = Buffer::filled(Rect::new(5, 5), "A");

            assert!(front.diff(&back).is_empty());
        }

        #[test]
        fn different_filled_buffers_have_full_diff() {
            let front = Buffer::empty(Rect::new(2, 2));
            let back = Buffer::filled(Rect::new(2, 2), "O");

            let diff = front.diff(&back);

            let expected_diff = vec![
                Cell::new(0, 0, "O", Style::default()),
                Cell::new(1, 0, "O", Style::default()),
                Cell::new(0, 1, "O", Style::default()),
                Cell::new(1, 1, "O", Style::default()),
            ];
            assert_diff(diff, expected_diff);
        }

        #[test]
        fn write_full_line_has_a_full_line_diff() {
            let front = Buffer::empty(Rect::new(5, 2));
            let mut back = Buffer::empty(Rect::new(5, 2));

            back.write_line(0, "hello", &Style::default());
            let diff = front.diff(&back);

            let expected_diff = vec![
                Cell::new(0, 0, "h", Style::default()),
                Cell::new(1, 0, "e", Style::default()),
                Cell::new(2, 0, "l", Style::default()),
                Cell::new(3, 0, "l", Style::default()),
                Cell::new(4, 0, "o", Style::default()),
                Cell::new(0, 1, " ", Style::default()),
                Cell::new(1, 1, " ", Style::default()),
                Cell::new(2, 1, " ", Style::default()),
                Cell::new(3, 1, " ", Style::default()),
                Cell::new(4, 1, " ", Style::default()),
            ];
            assert_diff(diff, expected_diff);
        }

        #[test]
        fn write_full_lines_has_a_full_diff() {
            let front = Buffer::empty(Rect::new(5, 2));
            let mut back = Buffer::empty(Rect::new(5, 2));

            back.write_line(0, "hello", &Style::default());
            back.write_line(1, "world", &Style::default());
            let diff = front.diff(&back);

            let expected_diff = vec![
                Cell::new(0, 0, "h", Style::default()),
                Cell::new(1, 0, "e", Style::default()),
                Cell::new(2, 0, "l", Style::default()),
                Cell::new(3, 0, "l", Style::default()),
                Cell::new(4, 0, "o", Style::default()),
                Cell::new(0, 1, "w", Style::default()),
                Cell::new(1, 1, "o", Style::default()),
                Cell::new(2, 1, "r", Style::default()),
                Cell::new(3, 1, "l", Style::default()),
                Cell::new(4, 1, "d", Style::default()),
            ];
            assert_diff(diff, expected_diff);
        }

        #[test]
        fn write_partial_line_clears_remainder_of_line() {
            let front = Buffer::empty(Rect::new(10, 1));
            let mut back = Buffer::filled(Rect::new(10, 1), "B");

            back.write_line(0, "hello", &Style::default());
            let diff = front.diff(&back);

            let expected_diff = vec![
                Cell::new(0, 0, "h", Style::default()),
                Cell::new(1, 0, "e", Style::default()),
                Cell::new(2, 0, "l", Style::default()),
                Cell::new(3, 0, "l", Style::default()),
                Cell::new(4, 0, "o", Style::default()),
                // Remaining blanked out cells will not show in diff
                // as they match the empty front buffer.
            ];
            assert_diff(diff, expected_diff);
        }

        #[test]
        fn reset_clears_the_buffer() {
            let front = Buffer::empty(Rect::new(10, 10));
            let mut back = Buffer::filled(Rect::new(10, 10), "B");

            back.reset();

            assert!(front.diff(&back).is_empty());
        }
    }
}
