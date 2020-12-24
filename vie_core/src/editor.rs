use crate::{backend::Backend, viewport::Viewport};
use anyhow::Result;

/// The main application state.
pub struct Editor<'a, B: Backend> {
    viewport: Viewport<'a, B>,
}

impl<'a, B: Backend> Editor<'a, B> {
    /// Create a new Editor.
    pub fn new(backend: &'a mut B) -> Result<Self> {
        use anyhow::Context;

        Ok(Self {
            viewport: Viewport::new(backend).context("unable to initialise Viewport")?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        Ok(())
    }
}
