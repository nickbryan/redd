use crate::{
    backend::{Canvas, Event, EventLoop},
    command::{Command, Mode, NormalMode},
    command_line::CommandLine,
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
    command_line: CommandLine,
    event_loop: E,
    mode: Mode,
    should_quit: bool,
    viewport: Viewport<'a, C>,
}

impl<'a, E: EventLoop, C: Canvas> Editor<'a, E, C> {
    /// Create a new Editor.
    pub fn new(event_loop: E, canvas: &'a mut C) -> Result<Self> {
        use anyhow::Context;

        let viewport = Viewport::new(canvas).context("unable to initialise Viewport")?;

        Ok(Self {
            command_line: CommandLine::new(Rect::positioned(
                viewport.area().width,
                1,
                0,
                viewport.area().bottom() - 1,
            )),
            event_loop,
            mode: Mode::default(),
            should_quit: false,
            viewport,
        })
    }

    pub fn run(&mut self) -> Result<(), EditorError> {
        while !self.should_quit {
            match self.event_loop.read_event()? {
                Event::Input(key) => {
                    if let Some(command) = match self.mode {
                        Mode::Execute(ref mut mode) => mode.handle(key),
                        Mode::Insert(ref mut mode) => mode.handle(key),
                        Mode::Normal(ref mut mode) => mode.handle(key),
                    } {
                        self.handle_command(command);
                    }
                }
                Event::Tick => (),
                Event::Error(e) => return Err(EditorError::from(e)),
            };

            self.render()?;
        }

        Ok(())
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Quit => self.should_quit = true,
            Command::EnterMode(mode) => self.mode = mode,
            _ => {
                if let Mode::Execute(ref mut mode) = self.mode {
                    if let Some(command) = self.command_line.execute_command(command) {
                        match command {
                            Command::ParseCommandLineInput(input) => {
                                let command = mode.parse(&input);
                                self.mode = Mode::Normal(NormalMode::default());
                                if let Some(command) = command {
                                    self.handle_command(command);
                                    return;
                                }
                            }
                            _ => {
                                self.handle_command(command);
                                return;
                            }
                        }
                    }
                }

                // Pass to buffer or whatever here.
            }
        }
    }

    fn render(&mut self) -> Result<(), EditorError> {
        let viewport_area = self.viewport.area();
        let mode = &self.mode;
        let command_line = &self.command_line;

        self.viewport
            .draw(|frame| {
                frame.render(StatusBar {
                    area: Rect::positioned(viewport_area.width, 1, 0, viewport_area.bottom() - 2),
                    mode: mode.to_string(),
                    line_count: 0,
                    cursor_position: Position::default(),
                    file_name: String::new(),
                });

                if let Mode::Execute(_) = mode {
                    frame.set_cursor_position(Position::new(
                        viewport_area
                            .position
                            .col
                            .saturating_add(command_line.cursor_position().col),
                        viewport_area.bottom() - 1,
                    ));
                    frame.render(command_line);
                }

                Ok(())
            })
            .map_err(|e| EditorError::Render(e))?;

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
