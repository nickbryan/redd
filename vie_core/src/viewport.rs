use crate::{
    backend::Backend,
    ui::{frame, Position, Rect},
};
use anyhow::Result;

/// The area of the screen that we can draw to. The Viewport is responsible for handling
/// interactions with the backed and drawing.
pub struct Viewport<B: Backend> {
    area: Rect,
    backend: B,
    buffers: [frame::Buffer; 2],
    current_buffer_idx: usize,
}

impl<B: Backend> Viewport<B> {
    /// Create a new Viewport for the provided Backend.
    pub fn new(mut backend: B) -> Result<Self> {
        use anyhow::Context;

        let area = backend.size().context("unable to set Viewport area")?;

        Ok(Self {
            area,
            backend,
            buffers: [frame::Buffer::empty(area), frame::Buffer::empty(area)],
            current_buffer_idx: 0,
        })
    }
}
