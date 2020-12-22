use crate::ui::{frame, Rect};
use anyhow::Result;
use std::io::Error as IoError;

/// Key presses accepted by the editor.
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

/// Events are dispatched from the backend to allow the application to handle input.
#[derive(Debug)]
pub enum Event {
    /// Input was recieved from the backend.
    Input(Key),

    /// No input recieved, do something else for now.
    Tick,

    /// An error occurred reading events.
    Error(IoError),
}

/// Backend is an interface to the ui. It could be the terminal or web ui.
pub trait Backend {
    /// Clear the ui.
    fn clear(&mut self) -> Result<(), IoError>;

    /// Draw the given cells in the ui's current buffer.
    fn draw<'a, I: Iterator<Item = &'a frame::Cell>>(&mut self, cells: I) -> Result<(), IoError>;

    /// Flush the ui's current buffer.
    fn flush(&mut self) -> Result<(), IoError>;

    /// Hide the cursor.
    fn hide_cursor(&mut self) -> Result<(), IoError>;

    /// Position the cursor at the given row and column.
    fn position_cursor(&mut self, row: usize, col: usize) -> Result<(), IoError>;

    /// Read and wait for the next event.
    fn read_event(&self) -> Result<Event>;

    /// Show the cursor.
    fn show_cursor(&mut self) -> Result<(), IoError>;

    /// Get the size of the ui.
    fn size(&self) -> Result<Rect, IoError>;
}
