use crate::{backend::Backend, editor::Position, ui::layout::Rect};
use anyhow::{Context, Result};

pub struct Terminal<B: Backend> {
    backend: B,
}

impl<B: Backend> Terminal<B> {
    pub fn new(mut backend: B) -> Result<Self> {
        backend
            .enable_raw_mode()
            .context("unable to enable raw mode")?;

        // We LeaveAlternateScreen in the Drop implementation to ensure that it is executed.
        backend
            .enter_alterate_screen()
            .context("unable to enter alternate screen")?;

        Ok(Self { backend })
    }

    pub fn clear(&mut self) -> Result<()> {
        self.backend.clear().context("unable to clear screen")
    }

    pub fn clear_line(&mut self) -> Result<()> {
        self.backend.clear_line().context("unable to clear line")
    }

    pub fn flush(&mut self) -> Result<()> {
        self.backend.flush().context("unable to flush output")
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        self.backend.hide_cursor().context("unable to hide cursor")
    }

    pub fn position_cursor(&mut self, position: &Position) -> Result<()> {
        self.backend
            .position_cursor(position.x as u16, position.y as u16)
            .context("unable to position cursor")
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        self.backend.show_cursor().context("unable to show cursor")
    }

    pub fn size(&self) -> Result<Rect> {
        let size = self
            .backend
            .size()
            .context("unable to get size of terminal")?;

        Ok(Rect::new(size.width(), size.height().saturating_sub(2)))
    }
}

impl<B: Backend> Drop for Terminal<B> {
    fn drop(&mut self) {
        self.backend
            .leave_alterante_screen()
            .expect("unable to leave alternate screen");

        self.backend
            .disable_raw_mode()
            .expect("unable to disable raw mode");
    }
}
