use crate::{
    document::Document,
    event::{Event, Events, Key},
    row::Row,
    terminal::Terminal,
};
use anyhow::{Context, Result};
use std::{env, time::Duration};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let args: Vec<String> = env::args().collect();

        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Ok(Self {
            should_quit: false,
            terminal: Terminal::new().context("unable to create Terminal")?,
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
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
        let terminal_height = self.terminal.size()?.height as usize;
        let Position { x, y } = self.cursor_position;
        let height = self.document.len();
        let width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

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
            Key::PageUp => {
                if y > terminal_height {
                    (x, y - terminal_height)
                } else {
                    (x, 0)
                }
            }
            Key::PageDown => {
                if y.saturating_add(terminal_height) < height {
                    (x, y + terminal_height as usize)
                } else {
                    (x, height)
                }
            }
            Key::Home => (0, y),
            Key::End => (width, y),
            _ => (x, y),
        };

        let width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        self.cursor_position = Position {
            x: if x > width { width } else { x },
            y,
        };

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

        self.scroll().context("unable to scroll")
    }

    fn scroll(&mut self) -> Result<()> {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size()?.width as usize;
        let height = self.terminal.size()?.height as usize;

        if y < self.offset.y {
            self.offset.y = y;
        } else if y >= self.offset.y.saturating_add(height) {
            self.offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < self.offset.x {
            self.offset.x = x;
        } else if x >= self.offset.x.saturating_add(width) {
            self.offset.x = x.saturating_add(width).saturating_add(1);
        }

        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.position_cursor(&Position::default())?;

        if self.should_quit {
            self.terminal.clear()?;
            self.terminal.flush()?;
            return Ok(());
        }

        self.draw_rows()?;
        self.terminal.position_cursor(&Position {
            x: self.cursor_position.x.saturating_sub(self.offset.x),
            y: self.cursor_position.y.saturating_sub(self.offset.y),
        })?;

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

    fn draw_row(&self, row: &Row) -> Result<()> {
        let width = self.terminal.size()?.width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
        Ok(())
    }

    fn draw_rows(&mut self) -> Result<()> {
        let height = self.terminal.size()?.height;

        for terminal_row in 0..height - 1 {
            self.terminal.clear_current_line()?;

            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row).context("unable to draw row")?;
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message()?;
            } else {
                println!("~\r");
            }
        }

        Ok(())
    }
}
