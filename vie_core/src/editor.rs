use crate::{backend::Backend, viewport::Viewport};
use anyhow::Result;

/// The main application state.
pub struct Editor<B: Backend> {
    viewport: Viewport<B>,
}

impl<B: Backend> Editor<B> {
    /// Create a new Editor.
    pub fn new(backend: B) -> Result<Self> {
        use anyhow::Context;

        Ok(Self {
            viewport: Viewport::new(backend).context("unable to initialise Viewport")?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        Ok(())
    }
}
