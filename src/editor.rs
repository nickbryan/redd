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
    io::{self, Stdout},
    time::Duration,
};

pub struct Editor {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    event_loop: Box<dyn EventLoop>,
    should_quit: bool,
    buffers: Vec<Buffer>,
    active_buffer_idx: usize,
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

        match key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => actrive_buffer
                .proccess_keypress(key)
                .context("unable to process keypress on active buffer")?,
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

        self.terminal.draw(|view| {
            let width = view.area().width;

            view.render(active_buffer);

            view.render(&StatusBar::new(
                Rect::positioned(width, 1, 0, view.area().height - 2),
                active_buffer.document_name(),
                active_buffer.lines_in_document(),
                active_buffer.cursor_position().y.saturating_add(1),
            ));

            view.set_cursor_position(active_buffer.cursor_position());

            Ok(())
        })
    }
}
