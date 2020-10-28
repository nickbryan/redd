use crate::{
    document::Document,
    io::{
        event::{CrosstermEventLoop, Event, Key, Loop as EventLoop},
        CrosstermBackend,
    },
    terminal::Terminal,
    ui::{
        layout::{Position, Rect},
        status_bar::StatusBar,
        text::DocumentView,
        welcome::Screen as WelcomeScreen,
    },
};
use anyhow::{Context, Result};
use std::{
    env,
    io::{self, Stdout},
    time::Duration,
};

pub struct Editor {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    event_loop: Box<dyn EventLoop>,
    cursor_position: Position,
    document: Document,
    offset: Position,
    should_quit: bool,
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

        let backend = CrosstermBackend::new(io::stdout());
        let event_loop = Box::new(CrosstermEventLoop::new(Duration::from_millis(250)));

        Ok(Self {
            terminal: Terminal::new(backend).context("unable to create Terminal")?,
            event_loop,
            cursor_position: Position::default(),
            document,
            offset: Position::default(),
            should_quit: false,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.event_loop.start();

        loop {
            self.refresh_screen().context("unable to refresh screen")?;

            if self.should_quit {
                break;
            }

            match self.event_loop.next()? {
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
        use crate::document::Row;

        let terminal_height = self.terminal.viewport().height - 2;
        let Position { x, y } = self.cursor_position;
        let height = self.document.len();
        let width = self.document.row(y).map_or(0, Row::len);

        let (x, y) = match key {
            Key::Up => (x, y.saturating_sub(1)),
            Key::Down => {
                if y < height {
                    (x, y.saturating_add(1))
                } else {
                    (x, y)
                }
            }
            Key::Left => {
                if x > 0 {
                    (x - 1, y)
                } else if y > 0 {
                    self.document
                        .row(y)
                        .map_or((0, y - 1), |row| (row.len(), y - 1))
                } else {
                    (x, y)
                }
            }
            Key::Right => {
                if x < width {
                    (x + 1, y)
                } else if y < height {
                    (0, y + 1)
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
                    (x, y + terminal_height)
                } else {
                    (x, height)
                }
            }
            Key::Home => (0, y),
            Key::End => (width, y),
            _ => (x, y),
        };

        let new_width = self.document.row(y).map_or(0, Row::len);

        self.cursor_position = Position {
            x: if x > new_width { new_width } else { x },
            y,
        };

        Ok(())
    }

    fn proccess_keypress(&mut self, key: Key) -> Result<()> {
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Ctrl('s') => self.document.save().context("unable to save document")?,
            Key::Char(ch) => {
                self.document
                    .insert(&self.cursor_position, ch)
                    .context("unable to insert character in document")?;

                self.move_cursor(Key::Right)
                    .context("unable to move cursor to the right")?;
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left)
                        .context("unable to move cursor to the left")?;
                    self.document.delete(&self.cursor_position);
                }
            }
            Key::Enter => {
                self.document.insert_newline(&self.cursor_position);
                self.move_cursor(Key::Down)
                    .context("unable to move to new line")?;
                self.move_cursor(Key::Home)
                    .context("unable to move to start of new line")?;
            }
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
        let width = self.terminal.viewport().width;
        let height = self.terminal.viewport().height - 2;

        let offset = if y < self.offset.y {
            (self.offset.x, y)
        } else if y >= self.offset.y.saturating_add(height) {
            (self.offset.x, y.saturating_sub(height).saturating_add(1))
        } else {
            (self.offset.x, self.offset.y)
        };

        let offset = if x < self.offset.x {
            (x, offset.1)
        } else if x >= self.offset.x.saturating_add(width) {
            (x.saturating_add(width).saturating_add(1), offset.1)
        } else {
            (self.offset.x, offset.1)
        };

        self.offset = Position::from(offset);

        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<()> {
        if self.should_quit {
            self.terminal.clear()?;
            self.terminal.flush()?;
            return Ok(());
        }

        let document = &self.document;
        let offset = &self.offset;
        let cursor_position = &self.cursor_position;

        self.terminal.draw(|view| {
            let width = view.area().width;
            let height = view.area().height - 2;

            if document.is_empty() {
                view.render(&WelcomeScreen {}, view.area());
            } else {
                view.render(
                    &DocumentView::new(document, *offset),
                    Rect::new(width, height),
                );

                let file_name = document
                    .file_name()
                    .unwrap_or(&"[No Name]".to_string())
                    .clone();

                view.render(
                    &StatusBar::new(
                        file_name,
                        document.len(),
                        cursor_position.y.saturating_add(1),
                    ),
                    Rect::positioned(width, 1, 0, view.area().height - 2),
                );
            }

            view.set_cursor_position(Position::new(
                cursor_position.x.saturating_sub(offset.x),
                cursor_position.y.saturating_sub(offset.y),
            ));

            Ok(())
        })
    }
}
