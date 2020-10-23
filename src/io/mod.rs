use crate::ui::{buffer::Cell, layout::Rect};
use anyhow::Result;
use std::{io, time::Duration};

mod crossterm;
pub mod event;

pub use self::crossterm::CrosstermBackend;

pub trait Backend {
    fn clear(&mut self) -> Result<(), io::Error>;
    fn draw<'a, I: Iterator<Item = &'a Cell>>(&mut self, cells: I) -> Result<(), io::Error>;
    fn enable_raw_mode(&mut self) -> Result<(), io::Error>;
    fn enter_alterate_screen(&mut self) -> Result<(), io::Error>;
    fn disable_raw_mode(&mut self) -> Result<(), io::Error>;
    fn flush(&mut self) -> Result<(), io::Error>;
    fn leave_alterante_screen(&mut self) -> Result<(), io::Error>;
    fn hide_cursor(&mut self) -> Result<(), io::Error>;
    fn poll_events(&mut self, timeout: Duration) -> Result<bool, io::Error>;
    fn position_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error>;
    fn show_cursor(&mut self) -> Result<(), io::Error>;
    fn size(&self) -> Result<Rect, io::Error>;
}
