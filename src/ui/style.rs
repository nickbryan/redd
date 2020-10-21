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

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    foreground: Color,
    background: Color,
}

impl Style {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            foreground,
            background,
        }
    }

    pub fn foreground(&self) -> Color {
        self.foreground
    }

    pub fn background(&self) -> Color {
        self.background
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
