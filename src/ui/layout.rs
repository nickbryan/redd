use crate::ui::buffer::Buffer;

pub trait Component {
    fn render(&self, area: Rect, buffer: &mut Buffer);
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Rect {
    width: usize,
    height: usize,
    position: Position,
}

impl Rect {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            position: Position::default(),
        }
    }

    pub fn positioned(width: usize, height: usize, position: &Position) -> Self {
        Self {
            width,
            height,
            position: Position::from(*position),
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn x(&self) -> usize {
        self.position.x
    }

    pub fn y(&self) -> usize {
        self.position.y
    }

    pub fn area(&self) -> usize {
        self.width().saturating_mul(self.height())
    }

    pub fn left(&self) -> usize {
        self.position.x
    }

    pub fn right(&self) -> usize {
        self.position.x + self.width()
    }

    pub fn top(&self) -> usize {
        self.position.y
    }

    pub fn bottom(&self) -> usize {
        self.position.y + self.height()
    }

    pub fn contains(&self, position: &Position) -> bool {
        let Position { x, y } = *position;

        x >= self.left() && x < self.right() && y >= self.top() && y < self.bottom()
    }
}
