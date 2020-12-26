use crate::{
    backend::{EventLoop, Grid, Key},
    viewport::Viewport,
};
use anyhow::Result;

pub trait Mode {
    fn process(&mut self, key: Key);
}

pub struct NormalMode {}
impl Mode for NormalMode {
    fn process(&mut self, key: Key) {}
}

/// The main application state.
pub struct Editor<'a, E: EventLoop, G: Grid, M: Mode> {
    event_loop: E,
    viewport: Viewport<'a, G>,
    mode: M,
}

impl<'a, E: EventLoop, G: Grid, M: Mode> Editor<'a, E, G, M> {
    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.event_loop.read_event() {
                Event::Input(key) => self.mode.process(key),
                Event::Tick => {}
                Event::Error(e) => return Err(e),
            };
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
        })
    }
}
