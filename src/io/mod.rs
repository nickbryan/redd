use crate::ui::{layout::Rect, FrameBufferCell};
use anyhow::{Error, Result};
use std::time::Duration;

mod crossterm;
pub mod event;

pub use self::crossterm::Backend as CrosstermBackend;

pub trait Backend {
    fn clear(&mut self) -> Result<(), Error>;
    fn draw<'a, I: Iterator<Item = &'a FrameBufferCell>>(&mut self, cells: I) -> Result<(), Error>;
    fn enable_raw_mode(&mut self) -> Result<(), Error>;
    fn enter_alterate_screen(&mut self) -> Result<(), Error>;
    fn disable_raw_mode(&mut self) -> Result<(), Error>;
    fn flush(&mut self) -> Result<(), Error>;
    fn leave_alterante_screen(&mut self) -> Result<(), Error>;
    fn hide_cursor(&mut self) -> Result<(), Error>;
    fn poll_events(&mut self, timeout: Duration) -> Result<bool, Error>;
    fn position_cursor(&mut self, x: usize, y: usize) -> Result<(), Error>;
    fn show_cursor(&mut self) -> Result<(), Error>;
    fn size(&self) -> Result<Rect, Error>;
}
