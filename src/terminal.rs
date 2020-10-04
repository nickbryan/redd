use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyEvent},
    terminal::{self, Clear, ClearType},
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
    pub fn clear(&mut self) {
        crossterm::queue!(self.stdout, Clear(ClearType::All)).unwrap();
    }

    pub fn clear_current_line(&mut self) {
        crossterm::queue!(self.stdout, Clear(ClearType::CurrentLine)).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn hide_cursor(&mut self) {
        crossterm::queue!(self.stdout, Hide).unwrap();
    }

    pub fn position_cursor(&mut self, x: u16, y: u16) {
        crossterm::queue!(self.stdout, MoveTo(x, y)).unwrap();
    }

    pub fn process_events(&self) -> Option<KeyEvent> {
        match event::read().unwrap() {
            Event::Key(event) => {
                return Some(event);
            }
            _ => return None,
        }
    }

    pub fn show_cursor(&mut self) {
        crossterm::queue!(self.stdout, Show).unwrap();
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}

impl Default for Terminal {
    fn default() -> Self {
        // TODO: handle unwrapping in this function.
        let size = terminal::size().unwrap();

        terminal::enable_raw_mode().unwrap();

        Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            stdout: io::stdout(),
        }
    }
}
