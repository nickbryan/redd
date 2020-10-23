use crate::{
    io::Backend,
    ui::{buffer::Cell, layout::Rect, style::Color},
};
use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Color as CrosstermColor, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    time::Duration,
};

pub struct CrosstermBackend<W: Write> {
    buffer: W,
}

impl<W: Write> CrosstermBackend<W> {
    pub fn new(buffer: W) -> Self {
        Self { buffer }
    }
}

impl<W: Write> Write for CrosstermBackend<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

impl<W: Write> Backend for CrosstermBackend<W> {
    fn clear(&mut self) -> Result<(), io::Error> {
        crossterm::queue!(self.buffer, Clear(ClearType::All))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn draw<'a, I>(&mut self, cells: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = &'a Cell>,
    {
        let mut prev_bg = Color::Reset;
        let mut prev_fg = Color::Reset;

        for cell in cells {
            self.position_cursor(cell.position().x as u16, cell.position().y as u16)?;

            if cell.style().background() != prev_bg {
                crossterm::queue!(
                    self.buffer,
                    SetBackgroundColor(CrosstermColor::from(cell.style().background()))
                )
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

                prev_bg = cell.style().background();
            }

            if cell.style().foreground() != prev_fg {
                crossterm::queue!(
                    self.buffer,
                    SetForegroundColor(CrosstermColor::from(cell.style().foreground()))
                )
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

                prev_fg = cell.style().foreground();
            }

            crossterm::queue!(self.buffer, Print(cell.symbol()))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        }

        crossterm::queue!(
            self.buffer,
            SetBackgroundColor(CrosstermColor::from(Color::Reset)),
            SetForegroundColor(CrosstermColor::from(Color::Reset)),
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn enable_raw_mode(&mut self) -> Result<(), io::Error> {
        crossterm::terminal::enable_raw_mode()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn enter_alterate_screen(&mut self) -> Result<(), io::Error> {
        crossterm::queue!(self.buffer, EnterAlternateScreen)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn disable_raw_mode(&mut self) -> Result<(), io::Error> {
        crossterm::terminal::disable_raw_mode()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn leave_alterante_screen(&mut self) -> Result<(), io::Error> {
        crossterm::queue!(self.buffer, LeaveAlternateScreen)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.buffer.flush()
    }

    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        crossterm::queue!(self.buffer, Hide)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn poll_events(&mut self, timeout: Duration) -> Result<bool, io::Error> {
        crossterm::event::poll(timeout)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn position_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error> {
        crossterm::queue!(self.buffer, MoveTo(x, y))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn show_cursor(&mut self) -> Result<(), io::Error> {
        crossterm::queue!(self.buffer, Show)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }

    fn size(&self) -> Result<Rect, io::Error> {
        let (width, height) = crossterm::terminal::size()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        Ok(Rect::new(usize::from(width), usize::from(height)))
    }
}

impl From<Color> for CrosstermColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Reset => CrosstermColor::Reset,
            Color::Black => CrosstermColor::Black,
            Color::Red => CrosstermColor::DarkRed,
            Color::Green => CrosstermColor::DarkGreen,
            Color::Yellow => CrosstermColor::DarkYellow,
            Color::Blue => CrosstermColor::DarkBlue,
            Color::Magenta => CrosstermColor::DarkMagenta,
            Color::Cyan => CrosstermColor::DarkCyan,
            Color::Gray => CrosstermColor::Grey,
            Color::DarkGray => CrosstermColor::DarkGrey,
            Color::LightRed => CrosstermColor::Red,
            Color::LightGreen => CrosstermColor::Green,
            Color::LightBlue => CrosstermColor::Blue,
            Color::LightYellow => CrosstermColor::Yellow,
            Color::LightMagenta => CrosstermColor::Magenta,
            Color::LightCyan => CrosstermColor::Cyan,
            Color::White => CrosstermColor::White,
            Color::AnsiValue(v) => CrosstermColor::AnsiValue(v),
            Color::Rgb(r, g, b) => CrosstermColor::Rgb { r, g, b },
        }
    }
}
