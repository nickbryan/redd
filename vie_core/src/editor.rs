use crate::{
    backend::{Canvas, Event, EventLoop},
    command::{Command, Parser, ParserMode},
    ui::{frame, Color, Component, Position, Rect, Style},
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
pub struct Editor<'a, E: EventLoop, C: Canvas> {
    event_loop: E,
    mode: ParserMode,
    should_quit: bool,
    viewport: Viewport<'a, C>,
}

impl<'a, E: EventLoop, C: Canvas> Editor<'a, E, C> {
    /// Create a new Editor.
    pub fn new(event_loop: E, canvas: &'a mut C) -> Result<Self> {
        use anyhow::Context;

        Ok(Self {
            event_loop,
            mode: ParserMode::default(),
            should_quit: false,
            viewport: Viewport::new(canvas).context("unable to initialise Viewport")?,
        })
    }

    pub fn run(&mut self) -> Result<(), EditorError> {
        while !self.should_quit {
            match self.event_loop.read_event()? {
                Event::Input(key) => {
                    if let Some(command) = self.mode.parse(key) {
                        match command {
                            Command::Quit => self.should_quit = true,
                            _ => (),
                        }
                    }
                }
                Event::Tick => (),
                Event::Error(e) => return Err(EditorError::from(e)),
            };

            let viewport_area = self.viewport.area();
            let mode = self.mode.to_string();
            self.viewport
                .draw(|frame| {
                    frame.render(StatusBar {
                        area: Rect::positioned(
                            viewport_area.width,
                            1,
                            0,
                            viewport_area.bottom() - 2,
                        ),
                        mode,
                        line_count: 0,
                        cursor_position: Position::default(),
                        file_name: String::new(),
                    });

                    Ok(())
                })
                .map_err(|e| EditorError::Render(e))?;
        }

        Ok(())
    }
}

// TODO: Status bar should be a part of the document as it is reporting on
// the status of the document. Each document view should have its own status
// bar as the last row.
struct StatusBar {
    pub area: Rect,
    pub mode: String,
    pub line_count: usize,
    pub cursor_position: Position,
    pub file_name: String,
}

impl Component for StatusBar {
    fn render(&self, buffer: &mut frame::Buffer) {
        let mut status = format!("Mode: [{}]    File: {}", self.mode, self.file_name);
        let line_indicator = format!(
            "L: {}/{} C: {}",
            self.cursor_position.row,
            self.line_count,
            self.cursor_position.col + 1
        );

        let len = status.len() + line_indicator.len();

        if self.area.width > len {
            status.push_str(&" ".repeat(self.area.width - len));
        }

        status = format!("{}{}", status, line_indicator);
        status.truncate(self.area.width);

        buffer.write_line(
            self.area.top(),
            &status,
            &Style::new(Color::Rgb(63, 63, 63), Color::Rgb(239, 239, 239)),
        );
    }
}
