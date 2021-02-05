use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{Color as CrosstermColor, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Error as IoError, Write},
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};
use vie_core::{frame, Canvas, Color as VieColor, Event, EventLoop, Key as VieKey, Rect};

/// Newtype to allow mapping VieColor to CrosstermColor.
struct Color(VieColor);

/// Newtype to allow mapping crossterm::event::KeyEvent to VieKey.
struct Key(VieKey);

/// Convert crossterm errors to std::io::Error.
fn crossterm_to_io_error(e: crossterm::ErrorKind) -> IoError {
    match e {
        crossterm::ErrorKind::IoError(e) => e,
        crossterm::ErrorKind::Utf8Error(e) => {
            IoError::new(io::ErrorKind::InvalidData, format!("{}", e))
        }
        _ => IoError::new(io::ErrorKind::Other, format!("{}", e)),
    }
}

/// EventLoop implementation for Crossterm.
pub struct CrosstermEventLoop {
    rx: Option<Receiver<Event>>,
    tick_rate: Duration,
}

impl CrosstermEventLoop {
    /// Creates a new CrosstermEventLoop and starts listening for events.
    pub fn new(tick_rate: Duration) -> Result<Self, IoError> {
        let mut backend = Self {
            rx: None,
            tick_rate,
        };

        backend.listen_for_events();

        Ok(backend)
    }

    /// Polls crossterm for events. If an input event is found it will be pushed onto the rx channel.
    /// If no event is found after the given tick_rate, a Tick event will be pushed onto the channel.
    /// If an error occures an Error event will be pushed onto the channel.
    /// This allows the caller to block on the read_event method and recieve a steady tick rate or
    /// input event respectively.
    fn listen_for_events(&mut self) {
        use crossterm::event as ctevent;

        let (tx, rx) = mpsc::channel();
        let tick_rate = self.tick_rate;

        thread::spawn(move || loop {
            match ctevent::poll(tick_rate) {
                Ok(true) => match ctevent::read() {
                    Ok(ctevent::Event::Key(key)) => {
                        tx.send(Event::Input(Key::from(key).0)).unwrap()
                    }
                    Err(e) => {
                        tx.send(Event::Error(crossterm_to_io_error(e))).unwrap();

                        break;
                    }
                    Ok(ctevent::Event::Mouse(_)) | Ok(ctevent::Event::Resize(_, _)) => (),
                },
                Ok(false) => tx.send(Event::Tick).unwrap(),
                Err(e) => {
                    tx.send(Event::Error(crossterm_to_io_error(e))).unwrap();

                    break;
                }
            };
        });

        self.rx = Some(rx);
    }
}

impl EventLoop for CrosstermEventLoop {
    fn read_event(&mut self) -> Result<Event, IoError> {
        use anyhow::Context;

        match self.rx.as_ref() {
            Some(rx) => rx
                .recv()
                .context("unable to recieve from event loop channel")
                .map_err(|e| IoError::new(io::ErrorKind::BrokenPipe, format!("{}", e))),
            None => panic!("trying to read from event channel that has not been initialised"),
        }
    }
}

/// Canvas implementation for crossterm.
pub struct CrosstermCanvas<W: Write> {
    out: W,
}

impl<W: Write> CrosstermCanvas<W> {
    /// Creates a new CrosstermCanvas.
    pub fn new(mut out: W) -> Result<Self, IoError> {
        crossterm::terminal::enable_raw_mode().map_err(crossterm_to_io_error)?;
        crossterm::execute!(out, EnterAlternateScreen).map_err(crossterm_to_io_error)?;

        Ok(Self { out })
    }
}

impl<W: Write> Drop for CrosstermCanvas<W> {
    /// Ensures that we LeaveAlternateScreen and disable_raw_mode before the application ends to
    /// return the user terminal back to normal.
    fn drop(&mut self) {
        crossterm::execute!(self.out, LeaveAlternateScreen)
            .expect("unable to leave alternate screen");
        crossterm::terminal::disable_raw_mode().expect("unable to disable raw mode");
    }
}

impl<W: Write> Canvas for CrosstermCanvas<W> {
    fn clear(&mut self) -> Result<(), IoError> {
        crossterm::queue!(self.out, Clear(ClearType::All)).map_err(crossterm_to_io_error)?;
        Ok(())
    }

    fn draw<'a, I>(&mut self, cells: I) -> Result<(), IoError>
    where
        I: Iterator<Item = &'a frame::Cell>,
    {
        let mut prev_background = Color(VieColor::Reset);
        let mut prev_foreground = Color(VieColor::Reset);

        for cell in cells {
            self.position_cursor(cell.position().row, cell.position().col)?;

            if cell.style().background != prev_background.0 {
                crossterm::queue!(
                    self.out,
                    SetBackgroundColor(CrosstermColor::from(Color(cell.style().background)))
                )
                .map_err(crossterm_to_io_error)?;

                prev_background = Color(cell.style().background);
            }

            if cell.style().foreground != prev_foreground.0 {
                crossterm::queue!(
                    self.out,
                    SetForegroundColor(CrosstermColor::from(Color(cell.style().foreground)))
                )
                .map_err(crossterm_to_io_error)?;

                prev_foreground = Color(cell.style().foreground);
            }

            crossterm::queue!(self.out, Print(cell.symbol())).map_err(crossterm_to_io_error)?;
        }

        crossterm::queue!(
            self.out,
            SetBackgroundColor(CrosstermColor::from(Color(VieColor::Reset))),
            SetForegroundColor(CrosstermColor::from(Color(VieColor::Reset))),
        )
        .map_err(crossterm_to_io_error)?;

        Ok(())
    }

    fn flush(&mut self) -> Result<(), IoError> {
        self.out.flush()
    }

    fn hide_cursor(&mut self) -> Result<(), IoError> {
        crossterm::queue!(self.out, Hide).map_err(crossterm_to_io_error)?;
        Ok(())
    }

    fn position_cursor(&mut self, row: usize, col: usize) -> Result<(), IoError> {
        use std::convert::TryFrom;

        let x =
            u16::try_from(col).map_err(|e| IoError::new(io::ErrorKind::Other, format!("{}", e)))?;
        let y =
            u16::try_from(row).map_err(|e| IoError::new(io::ErrorKind::Other, format!("{}", e)))?;

        crossterm::queue!(self.out, MoveTo(x, y)).map_err(crossterm_to_io_error)?;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), IoError> {
        crossterm::queue!(self.out, Show).map_err(crossterm_to_io_error)?;
        Ok(())
    }

    fn size(&self) -> Result<Rect, IoError> {
        let (width, height) = crossterm::terminal::size().map_err(crossterm_to_io_error)?;
        Ok(Rect::new(usize::from(width), usize::from(height)))
    }
}

impl From<Color> for CrosstermColor {
    fn from(color: Color) -> Self {
        match color.0 {
            VieColor::Reset => CrosstermColor::Reset,
            VieColor::Black => CrosstermColor::Black,
            VieColor::Red => CrosstermColor::DarkRed,
            VieColor::Green => CrosstermColor::DarkGreen,
            VieColor::Yellow => CrosstermColor::DarkYellow,
            VieColor::Blue => CrosstermColor::DarkBlue,
            VieColor::Magenta => CrosstermColor::DarkMagenta,
            VieColor::Cyan => CrosstermColor::DarkCyan,
            VieColor::Gray => CrosstermColor::Grey,
            VieColor::DarkGray => CrosstermColor::DarkGrey,
            VieColor::LightRed => CrosstermColor::Red,
            VieColor::LightGreen => CrosstermColor::Green,
            VieColor::LightBlue => CrosstermColor::Blue,
            VieColor::LightYellow => CrosstermColor::Yellow,
            VieColor::LightMagenta => CrosstermColor::Magenta,
            VieColor::LightCyan => CrosstermColor::Cyan,
            VieColor::White => CrosstermColor::White,
            VieColor::AnsiValue(v) => CrosstermColor::AnsiValue(v),
            VieColor::Rgb(r, g, b) => CrosstermColor::Rgb { r, g, b },
        }
    }
}

impl From<crossterm::event::KeyEvent> for Key {
    fn from(event: crossterm::event::KeyEvent) -> Self {
        use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

        match event {
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Enter,
            } => Key(VieKey::Enter),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Tab,
            } => Key(VieKey::Tab),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Backspace,
            } => Key(VieKey::Backspace),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Esc,
            } => Key(VieKey::Esc),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Left,
            } => Key(VieKey::Left),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Right,
            } => Key(VieKey::Right),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Down,
            } => Key(VieKey::Down),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Up,
            } => Key(VieKey::Up),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Insert,
            } => Key(VieKey::Insert),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Delete,
            } => Key(VieKey::Delete),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Home,
            } => Key(VieKey::Home),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::End,
            } => Key(VieKey::End),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::PageUp,
            } => Key(VieKey::PageUp),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::PageDown,
            } => Key(VieKey::PageDown),
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Char(ch),
            } => Key(VieKey::Char(ch)),
            KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Char(ch),
            } => Key(VieKey::Ctrl(ch)),
            _ => Key(VieKey::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::crossterm_to_io_error;
    use super::CrosstermCanvas;

    #[test]
    fn crossterm_ioerror_to_std_ioerror_maps_correctly() {
        use crossterm::ErrorKind as CtErrorKind;
        use std::io::{Error as IoError, ErrorKind as IoErrorKind};

        let crossterm_error = CtErrorKind::IoError(IoError::new(IoErrorKind::Other, "test"));

        let e = crossterm_to_io_error(crossterm_error);

        assert_eq!(e.kind(), IoErrorKind::Other);
        assert_eq!("test", format!("{}", e));
    }

    #[test]
    fn crossterm_utf8error_to_std_ioerror_maps_correctly() {
        use crossterm::ErrorKind as CtErrorKind;
        use std::io::ErrorKind as IoErrorKind;

        let crossterm_error = CtErrorKind::Utf8Error(String::from_utf8(vec![0, 159]).unwrap_err());

        let e = crossterm_to_io_error(crossterm_error);

        assert_eq!(e.kind(), IoErrorKind::InvalidData);
        assert_eq!(
            "invalid utf-8 sequence of 1 bytes from index 1",
            format!("{}", e)
        );
    }

    #[test]
    fn crossterm_other_to_std_ioerror_maps_correctly() {
        use crossterm::ErrorKind as CtErrorKind;
        use std::io::ErrorKind as IoErrorKind;

        let crossterm_error = CtErrorKind::ResizingTerminalFailure(String::new());

        let e = crossterm_to_io_error(crossterm_error);

        assert_eq!(e.kind(), IoErrorKind::Other);
        assert_eq!("Cannot resize the terminal", format!("{}", e));
    }

    #[test]
    fn crossterm_backend_enters_and_leaves_alternate_screen() {
        let mut out: Vec<u8> = Vec::new();

        let backend = CrosstermCanvas::new(&mut out);
        drop(backend);

        assert_eq!(
            "\u{1b}[?1049h\u{1b}[?1049l",
            String::from_utf8(out).unwrap()
        );
    }
}
