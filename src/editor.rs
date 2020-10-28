use crate::{
    document::{Buffer, Document},
    io::{
        event::{CrosstermEventLoop, Event, Key, Loop as EventLoop},
        CrosstermBackend,
    },
    terminal::Terminal,
    ui::{layout::Rect, status_bar::StatusBar},
};
use anyhow::{Context, Result};
use std::{
    env,
    fmt::{self, Display, Formatter},
    io::{self, Stdout},
    time::Duration,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "Normal"),
            Self::Insert => write!(f, "Insert"),
        }
    }
}
pub struct Editor {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    event_loop: Box<dyn EventLoop>,
    should_quit: bool,
    buffers: Vec<Buffer>,
    active_buffer_idx: usize,
    mode: Mode,
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

        let terminal = Terminal::new(backend).context("unable to create Terminal")?;

        let document_viewport =
            Rect::new(terminal.viewport().width, terminal.viewport().height - 2);

        Ok(Self {
            terminal,
            event_loop,
            should_quit: false,
            buffers: vec![Buffer::new(document, document_viewport)],
            active_buffer_idx: 0,
            mode: Mode::default(),
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

    fn proccess_keypress(&mut self, key: Key) -> Result<()> {
        let actrive_buffer = &mut self.buffers[self.active_buffer_idx];

        match self.mode {
            Mode::Insert => {
                match key {
                    Key::Esc => self.mode = Mode::Normal,
                    _ => {
                        if let Some(new_mode) = actrive_buffer
                            .proccess_keypress(key, self.mode)
                            .context("unable to process keypress on active buffer")?
                        {
                            self.mode = new_mode;
                        }
                    }
                };
            }
            Mode::Normal => {
                match key {
                    Key::Char('q') => self.should_quit = true,
                    _ => {
                        if let Some(new_mode) = actrive_buffer
                            .proccess_keypress(key, self.mode)
                            .context("unable to process keypress on active buffer")?
                        {
                            self.mode = new_mode;
                        }
                    }
                };
            }
        };

        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<()> {
        if self.should_quit {
            self.terminal.clear()?;
            self.terminal.flush()?;
            return Ok(());
        }

        let active_buffer = &self.buffers[self.active_buffer_idx];
        let mode = self.mode;

        self.terminal.draw(|view| {
            let width = view.area().width;

            view.render(active_buffer);

            view.render(&StatusBar::new(
                Rect::positioned(width, 1, 0, view.area().height - 2),
                active_buffer.document_name(),
                active_buffer.lines_in_document(),
                active_buffer.cursor_position().y.saturating_add(1),
                mode,
            ));

            view.set_cursor_position(active_buffer.cursor_position());

            Ok(())
        })
    }
}
