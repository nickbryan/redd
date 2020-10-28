use crate::{
    io::Backend as BaseBackend,
    ui::{layout::Rect, style::Color, FrameBufferCell},
};
use anyhow::{Error, Result};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Color as CrosstermColor, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    time::Duration,
};

pub struct Backend<W: Write> {
    buffer: W,
}

impl<W: Write> Backend<W> {
    pub fn new(buffer: W) -> Self {
        Self { buffer }
    }
}

impl<W: Write> Write for Backend<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer.flush()
    }
}

impl<W: Write> BaseBackend for Backend<W> {
    fn clear(&mut self) -> Result<(), Error> {
        crossterm::queue!(self.buffer, Clear(ClearType::All))?;
        Ok(())
    }

    fn draw<'a, I>(&mut self, cells: I) -> Result<(), Error>
    where
        I: Iterator<Item = &'a FrameBufferCell>,
    {
        let mut prev_background = Color::Reset;
        let mut prev_foreground = Color::Reset;

        for cell in cells {
            self.position_cursor(cell.position().x, cell.position().y)?;

            if cell.style().background() != prev_background {
                crossterm::queue!(
                    self.buffer,
                    SetBackgroundColor(CrosstermColor::from(cell.style().background()))
                )?;

                prev_background = cell.style().background();
            }

            if cell.style().foreground() != prev_foreground {
                crossterm::queue!(
                    self.buffer,
                    SetForegroundColor(CrosstermColor::from(cell.style().foreground()))
                )?;

                prev_foreground = cell.style().foreground();
            }

            crossterm::queue!(self.buffer, Print(cell.symbol()))?;
        }

        crossterm::queue!(
            self.buffer,
            SetBackgroundColor(CrosstermColor::from(Color::Reset)),
            SetForegroundColor(CrosstermColor::from(Color::Reset)),
        )?;

        Ok(())
    }

    fn enable_raw_mode(&mut self) -> Result<(), Error> {
        crossterm::terminal::enable_raw_mode()?;
        Ok(())
    }

    fn enter_alterate_screen(&mut self) -> Result<(), Error> {
        crossterm::queue!(self.buffer, EnterAlternateScreen)?;
        Ok(())
    }

    fn disable_raw_mode(&mut self) -> Result<(), Error> {
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    fn leave_alterante_screen(&mut self) -> Result<(), Error> {
        crossterm::queue!(self.buffer, LeaveAlternateScreen)?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.buffer.flush()?;
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Error> {
        crossterm::queue!(self.buffer, Hide)?;
        Ok(())
    }

    fn poll_events(&mut self, timeout: Duration) -> Result<bool, Error> {
        let polled = crossterm::event::poll(timeout)?;
        Ok(polled)
    }

    fn position_cursor(&mut self, x: usize, y: usize) -> Result<(), Error> {
        use std::convert::TryFrom;

        let x = u16::try_from(x)?;
        let y = u16::try_from(y)?;

        crossterm::queue!(self.buffer, MoveTo(x, y))?;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), Error> {
        crossterm::queue!(self.buffer, Show)?;
        Ok(())
    }

    fn size(&self) -> Result<Rect, Error> {
        let (width, height) = crossterm::terminal::size()?;

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
