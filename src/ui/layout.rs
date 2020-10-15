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
}
