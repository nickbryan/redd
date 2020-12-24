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
    fn read_event(&mut self) -> Result<Event>;

    /// Show the cursor.
    fn show_cursor(&mut self) -> Result<(), IoError>;

    /// Get the size of the ui.
    fn size(&self) -> Result<Rect, IoError>;
}

#[cfg(test)]
pub(crate) mod testutil {
    use super::{Backend, Event, Key};
    use crate::ui::{frame, Rect};
    use anyhow::Result;
    use std::{collections::VecDeque, io::Error as IoError};

    /// Provides the ability to assert output captured by the MockBackend.
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) enum CapturedOut {
        Clear,
        Draw(String),
        Flush,
        HideCursor,
        PositionCursor { row: usize, col: usize },
        ShowCursor,
    }

    /// A mocked version of Backend. Events can be queued to later be read from read_event. All
    /// output that would usually be passed to the underlying Write will be captured to be asserted
    /// later.
    pub(crate) struct MockBackend {
        captured_out: Vec<CapturedOut>,
        size: Rect,
        events: VecDeque<Event>,
    }

    impl MockBackend {
        pub(crate) fn captured_out(&self) -> &[CapturedOut] {
            self.captured_out.as_slice()
        }
    }

    impl Backend for MockBackend {
        fn clear(&mut self) -> Result<(), IoError> {
            self.captured_out.push(CapturedOut::Clear);
            Ok(())
        }

        fn draw<'a, I>(&mut self, cells: I) -> Result<(), IoError>
        where
            I: Iterator<Item = &'a frame::Cell>,
        {
            let mut output = String::new();

            cells
                .into_iter()
                .for_each(|cell| output.push_str(cell.symbol()));

            self.captured_out.push(CapturedOut::Draw(output));
            Ok(())
        }

        fn flush(&mut self) -> Result<(), IoError> {
            self.captured_out.push(CapturedOut::Flush);
            Ok(())
        }

        fn hide_cursor(&mut self) -> Result<(), IoError> {
            self.captured_out.push(CapturedOut::HideCursor);
            Ok(())
        }

        fn position_cursor(&mut self, row: usize, col: usize) -> Result<(), IoError> {
            self.captured_out
                .push(CapturedOut::PositionCursor { col, row });
            Ok(())
        }

        fn read_event(&mut self) -> Result<Event> {
            match self.events.pop_front() {
                Some(e) => Ok(e),
                None => Ok(Event::Input(Key::Unknown)),
            }
        }

        fn show_cursor(&mut self) -> Result<(), IoError> {
            self.captured_out.push(CapturedOut::ShowCursor);
            Ok(())
        }

        fn size(&self) -> Result<Rect, IoError> {
            Ok(self.size)
        }
    }

    /// Provides an interface for easily building a MockBackend.
    pub(crate) struct MockBackendBuilder {
        events: VecDeque<Event>,
        size: Rect,
    }

    impl MockBackendBuilder {
        pub(crate) fn new() -> Self {
            Self::sized(1440, 900)
        }

        pub(crate) fn sized(cols: usize, rows: usize) -> Self {
            Self {
                events: VecDeque::new(),
                size: Rect::new(cols, rows),
            }
        }

        pub(crate) fn add_key_press(&mut self, key: Key) {
            self.events.push_back(Event::Input(key));
        }

        pub(crate) fn add_input_string(&mut self, input: &str) {
            input
                .chars()
                .for_each(|ch| self.events.push_back(Event::Input(Key::Char(ch))));
        }

        pub(crate) fn build(self) -> MockBackend {
            MockBackend {
                captured_out: Vec::new(),
                events: self.events,
                size: self.size,
            }
        }
    }
}
