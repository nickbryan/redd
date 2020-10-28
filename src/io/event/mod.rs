use anyhow::{Error, Result};

mod crossterm;
pub use self::crossterm::Loop as CrosstermEventLoop;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Key {
    Enter,
    Tab,
    Backspace,
    Esc,
    Left,
    Right,
    Up,
    Down,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Char(char),
    Ctrl(char),
    Unknown,
}

#[derive(Debug)]
pub enum Event {
    Input(Key),
    Tick,
    Error(Error),
}

pub trait Loop {
    fn start(&mut self);
    fn next(&self) -> Result<Event>;
}
