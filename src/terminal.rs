use anyhow::{Context, Result};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyEvent},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout, Write};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let size = terminal::size().context("unable to get terminal size")?;

        terminal::enable_raw_mode().context("unable to enable raw mode")?;

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            stdout: io::stdout(),
        })
    }

    pub fn clear(&mut self) -> Result<()> {
        crossterm::queue!(self.stdout, Clear(ClearType::All)).context("unable to clear screen")
    }

    pub fn clear_current_line(&mut self) -> Result<()> {
        crossterm::queue!(self.stdout, Clear(ClearType::CurrentLine))
            .context("unable to clear line")
    }

    pub fn enter_alternate_screen(&mut self) -> Result<()> {
        crossterm::queue!(self.stdout, EnterAlternateScreen)
            .context("unable to enter alternate screen")
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush().context("unable to flush stdout")
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        crossterm::queue!(self.stdout, Hide).context("unable to hide cursor")
    }

    pub fn leave_alternate_screen(&mut self) -> Result<()> {
        crossterm::queue!(self.stdout, LeaveAlternateScreen)
            .context("unable to leave alternate screen")
    }

    pub fn position_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        crossterm::queue!(self.stdout, MoveTo(x, y)).context("unable to position cursor")
    }

    pub fn process_events(&self) -> Result<Option<KeyEvent>> {
        match event::read().context("unable to read event")? {
            Event::Key(event) => Ok(Some(event)),
            _ => Ok(None),
        }
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        crossterm::queue!(self.stdout, Show).context("unable to show cursor")
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
