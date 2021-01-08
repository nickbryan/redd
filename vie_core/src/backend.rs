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

/// EventLoop handles the dispatching of input within the application. When no input is ready, the
/// Tick Event should be triggered to allow the application to do other work.
pub trait EventLoop {
    /// Read and wait for the next event.
    fn read_event(&mut self) -> Result<Event, IoError>;
}

/// Canvas is an interface to the ui. It could be the terminal or web ui.
pub trait Canvas {
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

    /// Show the cursor.
    fn show_cursor(&mut self) -> Result<(), IoError>;

    /// Get the size of the ui.
    fn size(&self) -> Result<Rect, IoError>;
}

#[cfg(test)]
pub(crate) mod testutil {
    use super::{Canvas, Event, EventLoop, Key};
    use crate::ui::{frame, Rect};
    use anyhow::Result;
    use std::{collections::VecDeque, io::Error as IoError};

    pub(crate) struct MockEventLoop {
        events: VecDeque<Event>,
    }

    impl EventLoop for MockEventLoop {
        fn read_event(&mut self) -> Result<Event, IoError> {
            match self.events.pop_front() {
                Some(e) => Ok(e),
                None => Ok(Event::Input(Key::Unknown)),
            }
        }
    }

    /// Provides an interface for easily building a MockEventLoop.
    pub(crate) struct MockEventLoopBuilder {
        events: VecDeque<Event>,
    }

    impl MockEventLoopBuilder {
        pub(crate) fn new() -> Self {
            Self {
                events: VecDeque::new(),
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

        pub(crate) fn build(self) -> MockEventLoop {
            MockEventLoop {
                events: self.events,
            }
        }
    }

    /// Provides the ability to assert output captured by the MockCanvas.
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) enum CapturedOut {
        Clear,
        Draw(String),
        Flush,
        HideCursor,
        PositionCursor { col: usize, row: usize },
        ShowCursor,
    }

    /// A mocked version of Canvas. Events can be queued to later be read from read_event. All
    /// output that would usually be passed to the underlying Write will be captured to be asserted
    /// later.
    pub(crate) struct MockCanvas {
        captured_out: Vec<CapturedOut>,
        size: Rect,
    }

    impl MockCanvas {
        pub(crate) fn new(cols: usize, rows: usize) -> Self {
            Self {
                captured_out: Vec::new(),
                size: Rect::new(cols, rows),
            }
        }

        pub(crate) fn captured_out(&self) -> &[CapturedOut] {
            self.captured_out.as_slice()
        }
    }

    impl Canvas for MockCanvas {
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

        fn show_cursor(&mut self) -> Result<(), IoError> {
            self.captured_out.push(CapturedOut::ShowCursor);
            Ok(())
        }

        fn size(&self) -> Result<Rect, IoError> {
            Ok(self.size)
        }
    }
}
