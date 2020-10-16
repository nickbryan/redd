use crate::{
    backend::Backend,
    editor::Position,
    ui::{
        buffer::Buffer,
        layout::{Component, Rect},
    },
};
use anyhow::{Context, Result};

pub struct View<'a, B: Backend> {
    cursor_position: (u16, u16),
    terminal: &'a mut Terminal<B>,
}

impl<'a, B: Backend> View<'a, B> {
    pub fn area(&self) -> Rect {
        self.terminal.viewport()
    }

    pub fn cursor_position(&self) -> (u16, u16) {
        self.cursor_position
    }

    pub fn render<C: Component>(&mut self, component: C, area: Rect) {
        component.render(area, self.terminal.current_buffer_mut());
    }

    pub fn set_cursor_position(&mut self, position: (u16, u16)) {
        self.cursor_position = position;
    }
}

pub struct Terminal<B: Backend> {
    backend: B,
    buffers: [Buffer; 2],
    current_buffer_idx: usize,
    viewport: Rect,
}

impl<B: Backend> Terminal<B> {
    pub fn new(mut backend: B) -> Result<Self> {
        backend
            .enable_raw_mode()
            .context("unable to enable raw mode")?;

        // We LeaveAlternateScreen in the Drop implementation to ensure that it is executed.
        backend
            .enter_alterate_screen()
            .context("unable to enter alternate screen")?;

        let viewport = backend.size().context("unable to initialise viewport")?;

        Ok(Self {
            backend,
            buffers: [Buffer::empty(viewport), Buffer::empty(viewport)],
            current_buffer_idx: 0,
            viewport,
        })
    }

    pub fn clear(&mut self) -> Result<()> {
        self.backend.clear().context("unable to clear screen")
    }

    pub fn clear_line(&mut self) -> Result<()> {
        self.backend.clear_line().context("unable to clear line")
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer_idx]
    }

    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut View<B>) -> Result<()>,
    {
        // TODO: remove this clear once we have buffer updating working
        self.clear()?;
        self.hide_cursor()?;
        self.position_cursor(&Position::default())?;

        let mut view = View {
            terminal: self,
            cursor_position: (0, 0),
        };

        f(&mut view)?;

        // TODO: try and remove the tuple in favour of a Position object
        let (x, y) = view.cursor_position();

        self.flush()?;

        self.position_cursor(&Position {
            x: x as usize,
            y: y as usize,
        })?;

        self.show_cursor()?;

        self.swap_buffers();

        self.backend.flush().context("unable to flush backend")
    }

    pub fn flush(&mut self) -> Result<()> {
        let previous_buffer = &self.buffers[1 - self.current_buffer_idx];
        let current_buffer = &self.buffers[self.current_buffer_idx];
        self.backend
            .draw(previous_buffer.diff(current_buffer).into_iter())
            .context("unable to draw buffer diff to terminal backend")
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        self.backend.hide_cursor().context("unable to hide cursor")
    }

    pub fn position_cursor(&mut self, position: &Position) -> Result<()> {
        self.backend
            .position_cursor(position.x as u16, position.y as u16)
            .context("unable to position cursor")
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        self.backend.show_cursor().context("unable to show cursor")
    }

    pub fn size(&self) -> Result<Rect> {
        let size = self
            .backend
            .size()
            .context("unable to get size of terminal")?;

        Ok(Rect::new(size.width(), size.height().saturating_sub(2)))
    }

    fn swap_buffers(&mut self) {
        self.buffers[1 - self.current_buffer_idx].reset();
        self.current_buffer_idx = 1 - self.current_buffer_idx;
    }

    pub fn viewport(&self) -> Rect {
        self.viewport
    }
}

impl<B: Backend> Drop for Terminal<B> {
    fn drop(&mut self) {
        self.backend
            .leave_alterante_screen()
            .expect("unable to leave alternate screen");

        self.backend
            .disable_raw_mode()
            .expect("unable to disable raw mode");
    }
}
