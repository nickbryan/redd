use crate::{
    backend::Canvas,
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
/// interactions with the backend and drawing.
pub struct Viewport<'a, C: Canvas> {
    area: Rect,
    canvas: &'a mut C,
    buffers: [frame::Buffer; 2],
    current_buffer_idx: usize,
}

impl<'a, C: Canvas> Viewport<'a, C> {
    /// Create a new Viewport for the provided Canvas.
    pub fn new(canvas: &'a mut C) -> Result<Self> {
        use anyhow::Context;

        let area = canvas.size().context("unable to set Viewport area")?;

        Ok(Self {
            area,
            canvas,
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

        self.canvas
            .hide_cursor()
            .context("unable to hide cursor pre draw")?;

        let mut frame = Frame::new(&mut self.buffers[self.current_buffer_idx]);

        f(&mut frame)?;

        let next_cursor_pos = frame.cursor_position;

        let previous_buffer = &self.buffers[1 - self.current_buffer_idx];
        let current_buffer = &self.buffers[self.current_buffer_idx];
        let changes = previous_buffer.diff(current_buffer);

        self.canvas
            .draw(changes.into_iter())
            .context("unable to draw buffer diff")?;

        self.canvas
            .position_cursor(next_cursor_pos.row, next_cursor_pos.col)
            .context("unable to set cursor position for next frame render")?;

        self.canvas
            .show_cursor()
            .context("unable to show cursor post draw")?;

        self.swap_buffers();

        self.canvas.flush().context("unable to flush canvas")
    }

    fn swap_buffers(&mut self) {
        self.buffers[1 - self.current_buffer_idx].reset();
        self.current_buffer_idx = 1 - self.current_buffer_idx;
    }
}

impl<'a, G: Canvas> Drop for Viewport<'a, G> {
    /// When the Viewport goes out of scope (application has ended) we want to ensure that the
    /// screen is cleared and flushed to leave the user with a clean terminal.
    fn drop(&mut self) {
        self.canvas.clear().unwrap();
        self.canvas.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Viewport;
    use crate::{
        backend::testutil::{CapturedOut, MockCanvas},
        ui::{testutil::MockComponent, Position},
    };
    use anyhow::Result;

    #[test]
    fn cursor_position_can_be_updated_through_frame() {
        let mut canvas = MockCanvas::new(10, 10);

        {
            let mut viewport = Viewport::new(&mut canvas).unwrap();
            viewport
                .draw(|frame| -> Result<()> {
                    frame.set_cursor_position(Position::new(9, 9));
                    Ok(())
                })
                .unwrap();
        }

        assert!(canvas
            .captured_out()
            .contains(&CapturedOut::PositionCursor { col: 9, row: 9 }));
    }

    #[test]
    fn backend_interaction_order() {
        let mut canvas = MockCanvas::new(10, 10);

        {
            let mut viewport = Viewport::new(&mut canvas).unwrap();
            viewport.draw(|_| -> Result<()> { Ok(()) }).unwrap();
        }

        assert_eq!(
            &[
                CapturedOut::HideCursor, // Hidden during drawing to prevent flicker.
                CapturedOut::Draw(String::new()),
                CapturedOut::PositionCursor { col: 0, row: 0 },
                CapturedOut::ShowCursor,
                CapturedOut::Flush,
                CapturedOut::Clear, // Cleared on drop.
                CapturedOut::Flush,
            ],
            canvas.captured_out()
        );
    }

    #[test]
    fn component_can_be_drawn_to_frame() {
        let mut canvas = MockCanvas::new(10, 10);

        {
            let mut viewport = Viewport::new(&mut canvas).unwrap();
            viewport
                .draw(|frame| -> Result<()> {
                    let mut component = MockComponent::new();
                    // If we used a " " instead of "-", our diff would not render the space due to the
                    // buffer defaulting to a full buffer of empty spaces.
                    // The draw assertion would look like &CapturedOut::Draw("HelloWorld!".into()).
                    component.add_line("Hello-World!");
                    frame.render(component);
                    Ok(())
                })
                .unwrap();
        }

        assert!(canvas
            .captured_out()
            .contains(&CapturedOut::Draw("Hello-World!".into())));
    }

    #[test]
    fn only_changes_are_drawn_to_the_canvas() {
        let mut canvas = MockCanvas::new(10, 10);

        {
            let mut viewport = Viewport::new(&mut canvas).unwrap();
            viewport
                .draw(|frame| -> Result<()> {
                    let mut component = MockComponent::new();
                    component.add_line("Hello World!");
                    frame.render(component);
                    Ok(())
                })
                .unwrap();

            viewport
                .draw(|frame| -> Result<()> {
                    let mut component = MockComponent::new();
                    component.add_line("Hello Girl");
                    frame.render(component);
                    Ok(())
                })
                .unwrap();
        }

        assert!(canvas
            .captured_out()
            .contains(&CapturedOut::Draw("HelloWorld!".into())));

        assert!(canvas
            .captured_out()
            .contains(&CapturedOut::Draw("Gi  ".into())));
    }
}
