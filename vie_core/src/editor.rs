use crate::{
    backend::{Canvas, Event, EventLoop},
    mode::{Descriptor, Mode, NormalMode},
    viewport::Viewport,
};
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("there was an issue communicating with the underlying backend")]
    Io(#[from] std::io::Error),
    #[error("there was an issue drawing to the viewport")]
    Render(#[source] anyhow::Error),
}

/// The main application state.
pub struct Editor<'a, E: EventLoop, C: Canvas, M: Mode> {
    event_loop: E,
    viewport: Viewport<'a, C>,
    mode: M,
    should_quit: bool,
}

impl<'a, E: EventLoop, C: Canvas, M: Mode> Editor<'a, E, C, M> {
    pub fn run(&mut self) -> Result<(), EditorError> {
        while !self.should_quit {
            match self.event_loop.read_event()? {
                Event::Input(key) => {
                    if let crate::backend::Key::Char('q') = key {
                        self.should_quit = true;
                    }
                    self.mode.recieve_input(key);
                }
                Event::Tick => (),
                Event::Error(e) => return Err(EditorError::from(e)),
            };

            if let Some(mode) = self.mode.next_transition() {
                match mode {
                    Descriptor::Insert => (),
                    Descriptor::Normal => (),
                    Descriptor::Command => (),
                };
            }

            self.viewport
                .draw(|frame| Ok(()))
                .map_err(|e| EditorError::Render(e))?;
        }

        Ok(())
    }
}

impl<'a, E: EventLoop, C: Canvas> Editor<'a, E, C, NormalMode> {
    /// Create a new Editor.
    pub fn new(event_loop: E, canvas: &'a mut C) -> Result<Self> {
        use anyhow::Context;

        Ok(Self {
            event_loop,
            viewport: Viewport::new(canvas).context("unable to initialise Viewport")?,
            mode: NormalMode::default(),
            should_quit: false,
        })
    }
}
