use crate::ui::buffer::Buffer;

pub trait Component {
    fn render(&self, area: Rect, buffer: &mut Buffer);
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct Rect {
    width: usize,
    height: usize,
}

impl Rect {
    pub fn new(width: usize, height: usize) -> Self {
        Rect { width, height }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn area(&self) -> usize {
        self.width.saturating_mul(self.height)
    }
}
