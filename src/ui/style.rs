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

pub struct Style {
    foreground: Option<Color>,
    background: Option<Color>,
}

impl Style {
    pub fn new(foreground: Option<Color>, background: Option<Color>) -> Self {
        Self {
            foreground,
            background,
        }
    }

    pub fn foreground(&self) -> Option<&Color> {
        self.foreground.as_ref()
    }

    pub fn background(&self) -> Option<&Color> {
        self.background.as_ref()
    }
}
