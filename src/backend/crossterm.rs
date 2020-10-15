use crate::{backend::Backend, ui::layout::Rect};
use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

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

    fn clear_line(&mut self) -> Result<(), io::Error> {
        crossterm::queue!(self.buffer, Clear(ClearType::CurrentLine))
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
