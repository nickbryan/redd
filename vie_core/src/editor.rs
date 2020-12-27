use crate::{
    backend::{Event, EventLoop, Grid, Key},
    viewport::Viewport,
};
use anyhow::Result;
use thiserror::Error;

pub trait Mode {
    fn process(&mut self, key: Key);
}

pub struct NormalMode {}
impl Mode for NormalMode {
    fn process(&mut self, key: Key) {}
}

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("there was an issue communicating with the underlying backend")]
    Io(#[from] std::io::Error),
    #[error("there was an issue drawing to the viewport")]
    Render(#[source] anyhow::Error),
}

/// The main application state.
pub struct Editor<'a, E: EventLoop, G: Grid, M: Mode> {
    event_loop: E,
    viewport: Viewport<'a, G>,
    mode: M,
    should_quit: bool,
}

impl<'a, E: EventLoop, G: Grid, M: Mode> Editor<'a, E, G, M> {
    pub fn run(&mut self) -> Result<(), EditorError> {
        while !self.should_quit {
            match self.event_loop.read_event()? {
                Event::Input(key) => self.mode.process(key),
                Event::Tick => {}
                Event::Error(e) => return Err(EditorError::from(e)),
            };

            self.viewport
                .draw(|frame| Ok(()))
                .map_err(|e| EditorError::Render(e))?;
        }

        Ok(())
    }
}

impl<'a, E: EventLoop, G: Grid> Editor<'a, E, G, NormalMode> {
    /// Create a new Editor.
    pub fn new(event_loop: E, grid: &'a mut G) -> Result<Self> {
        use anyhow::Context;

        Ok(Self {
            event_loop,
            viewport: Viewport::new(grid).context("unable to initialise Viewport")?,
            mode: NormalMode {},
            should_quit: false,
        })
    }
}
