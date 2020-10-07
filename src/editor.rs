use crate::{
    event::{Event, Events, Key},
    terminal::Terminal,
};
use anyhow::{Context, Result};
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            should_quit: false,
            terminal: Terminal::new().context("unable to create Terminal")?,
            cursor_position: Position { x: 0, y: 0 },
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let events = Events::listen(Duration::from_millis(250));

        loop {
            self.refresh_screen().context("unable to refresh screen")?;

            if self.should_quit {
                break;
            }

            match events.next()? {
                Event::Input(key) => self
                    .proccess_keypress(key)
                    .context("unable to process key press")?,
                Event::Tick => { /* We can do stuff here while waiting for input */ }
                Event::Error(e) => return Err(e),
            };
        }

        Ok(())
    }

    fn move_cursor(&mut self, key: Key) -> Result<()> {
        let Position { x, y } = self.cursor_position;
        let size = self.terminal.size()?;
        let width = size.width.saturating_sub(1) as usize;
        let height = size.height.saturating_sub(1) as usize;

        let (x, y) = match key {
            Key::Up => (x, y.saturating_sub(1)),
            Key::Down => {
                if y < height {
                    (x, y.saturating_add(1))
                } else {
                    (x, y)
                }
            }
            Key::Left => (x.saturating_sub(1), y),
            Key::Right => {
                if x < width {
                    (x.saturating_add(1), y)
                } else {
                    (x, y)
                }
            }
            Key::PageUp => (x, 0),
            Key::PageDown => (x, height),
            Key::Home => (0, y),
            Key::End => (width, y),
            _ => (x, y),
        };

        self.cursor_position = Position { x, y };
        Ok(())
    }
    fn proccess_keypress(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(key).context("unable to move cursor")?,
            _ => {}
        };
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.position_cursor(&Position { x: 0, y: 0 })?;

        if self.should_quit {
            self.terminal.clear()?;
            self.terminal.flush()?;
            return Ok(());
        }

        self.draw_rows()?;
        self.terminal.position_cursor(&self.cursor_position)?;

        self.terminal.show_cursor()?;
        self.terminal.flush()
    }

    fn draw_welcome_message(&mut self) -> Result<()> {
        let mut welcome_message = format!("Redd editor -- version {}", VERSION);
        let width = self.terminal.size()?.width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
        Ok(())
    }

    fn draw_rows(&mut self) -> Result<()> {
        let height = self.terminal.size()?.height;

        for row in 0..height - 1 {
            self.terminal.clear_current_line()?;

            if row == height / 3 {
                self.draw_welcome_message()?;
            } else {
                println!("~\r");
            }
        }

        Ok(())
    }
}
