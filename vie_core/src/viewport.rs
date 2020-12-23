use crate::{
    backend::Backend,
    ui::{frame, Component, Position, Rect},
};
use anyhow::Result;

/// The next frame to be rendered to the screen. Used in the draw callback to allow the caller to
/// set the cursor position and render components to the current buffer.
pub struct Frame<'a> {
    cursor_position: Position,
    current_buffer: &'a mut frame::Buffer,
}

impl<'a> Frame<'a> {
    /// Create a new frame for the current buffer, setting the cursor_position back to default.
    fn new(current_buffer: &'a mut frame::Buffer) -> Self {
        Self {
            cursor_position: Position::default(),
            current_buffer,
        }
    }

    /// Render the given component into the current buffer.
    pub fn render<C: Component>(&mut self, component: C) {
        component.render(self.current_buffer);
    }

    /// Set the cursor position for the final frame render.
    pub fn set_cursor_position(&mut self, position: Position) {
        self.cursor_position = position;
    }
}

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
    pub fn new(backend: B) -> Result<Self> {
        use anyhow::Context;

        let area = backend.size().context("unable to set Viewport area")?;

        Ok(Self {
            area,
            backend,
            buffers: [frame::Buffer::empty(area), frame::Buffer::empty(area)],
            current_buffer_idx: 0,
        })
    }

    /// The area represented by the viewport.
    pub fn area(&self) -> Rect {
        self.area
    }

    /// Draw the current buffer to the screen. This wil call the given callback allowing the caller
    /// to define render order and cursor position. Buffer swapping and diff is handled here to
    /// ensure that only the required screen cells are updated.
    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Frame) -> Result<()>,
    {
        use anyhow::Context;

        self.backend
            .hide_cursor()
            .context("unable to hide cursor pre draw")?;

        let mut frame = Frame::new(&mut self.buffers[self.current_buffer_idx]);

        f(&mut frame)?;

        let next_cursor_pos = frame.cursor_position;

        let previous_buffer = &self.buffers[1 - self.current_buffer_idx];
        let current_buffer = &self.buffers[self.current_buffer_idx];
        let changes = previous_buffer.diff(current_buffer);

        self.backend
            .draw(changes.into_iter())
            .context("unable to draw buffer diff")?;

        self.backend
            .position_cursor(next_cursor_pos.row, next_cursor_pos.col)
            .context("unable to set cursor position for next frame render")?;

        self.backend
            .show_cursor()
            .context("unable to show cursor post draw")?;

        self.swap_buffers();

        self.backend.flush().context("unable to flush backend")
    }

    fn swap_buffers(&mut self) {
        self.buffers[1 - self.current_buffer_idx].reset();
        self.current_buffer_idx = 1 - self.current_buffer_idx;
    }
}
