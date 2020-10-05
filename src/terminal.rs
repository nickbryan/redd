use anyhow::{Context, Result};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{self, Stdout, Write};
use tui::{backend::CrosstermBackend, layout::Rect};

pub struct Terminal {
    terminal: tui::Terminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let mut stdout = io::stdout();

        crossterm::terminal::enable_raw_mode().context("unable to enable raw mode")?;

        // We LeaveAlternateScreen in the Drop implementation to ensure that it is executed.
        crossterm::execute!(stdout, EnterAlternateScreen)
            .context("unable to enter alternate screen")?;

        let backend = CrosstermBackend::new(stdout);

        Ok(Self {
            terminal: tui::Terminal::new(backend)
                .context("unable to create underlying tui::Terminal")?,
        })
    }

    pub fn clear(&mut self) -> Result<()> {
        self.terminal.clear().context("unable to clear screen")
    }

    pub fn clear_current_line(&mut self) -> Result<()> {
        crossterm::queue!(io::stdout(), Clear(ClearType::CurrentLine))
            .context("unable to clear line")
    }

    pub fn flush(&mut self) -> Result<()> {
        self.terminal.flush().context("unable to flush output")
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        self.terminal.hide_cursor().context("unable to hide cursor")
    }

    pub fn position_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        self.terminal
            .set_cursor(x, y)
            .context("unable to position cursor")
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        self.terminal.show_cursor().context("unable to show cursor")
    }

    pub fn size(&mut self) -> Result<Rect> {
        self.terminal
            .size()
            .context("unable to get size of terminal")
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::queue!(io::stdout(), LeaveAlternateScreen)
            .expect("unable to leave alternate screen");

        crossterm::terminal::disable_raw_mode().expect("unable to disable raw mode");
    }
}
