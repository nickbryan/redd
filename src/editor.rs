use crate::{
    command_line::CommandLine,
    document::{Buffer, Document},
    io::{
        event::{CrosstermEventLoop, Event, Loop as EventLoop},
        CrosstermBackend,
    },
    ops::{buffer::Parser as BufferCommandParser, Command},
    status_bar::StatusBar,
    terminal::Terminal,
    ui::layout::Rect,
};
use anyhow::{Context, Result};
use std::{
    env,
    fmt::{self, Display, Formatter},
    io::{self, Stdout},
    time::Duration,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Command => write!(f, "COMMAND"),
        }
    }
}

pub struct Editor {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    event_loop: Box<dyn EventLoop>,
    should_quit: bool,
    buffers: Vec<Buffer>,
    active_buffer_idx: usize,
    mode: Mode,
    buffer_commands: BufferCommandParser,
    status_bar: StatusBar,
    command_line: CommandLine,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let args: Vec<String> = env::args().collect();

        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        let backend = CrosstermBackend::new(io::stdout());
        let event_loop = Box::new(CrosstermEventLoop::new(Duration::from_millis(250)));

        let terminal = Terminal::new(backend).context("unable to create Terminal")?;

        let document_viewport =
            Rect::new(terminal.viewport().width, terminal.viewport().height - 2);

        let status_bar = StatusBar::new(Rect::positioned(
            terminal.viewport().width,
            1,
            0,
            terminal.viewport().bottom() - 2,
        ));

        let command_line = CommandLine::new(Rect::positioned(
            terminal.viewport().width,
            1,
            0,
            terminal.viewport().bottom() - 1,
        ));

        Ok(Self {
            terminal,
            event_loop,
            should_quit: false,
            buffers: vec![Buffer::new(document, document_viewport)],
            active_buffer_idx: 0,
            mode: Mode::default(),
            buffer_commands: BufferCommandParser::default(),
            status_bar,
            command_line,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.event_loop.start();

        loop {
            self.refresh_screen().context("unable to refresh screen")?;

            if self.should_quit {
                break;
            }

            match self.event_loop.next()? {
                Event::Input(key) => match self.mode {
                    Mode::Normal | Mode::Insert => {
                        if let Some(command) =
                            self.buffer_commands.matched_command_for(key, self.mode)
                        {
                            self.process_command(command)
                                .context("unable to process command")?;

                            self.update_status_bar();
                        };
                    }
                    Mode::Command => {
                        if let Some(command) = self.command_line.matched_command_for(key) {
                            self.process_command(command)
                                .context("unable to process command")?;

                            self.update_status_bar();
                        };
                    }
                },
                Event::Tick => {}
                Event::Error(e) => return Err(e),
            };
        }

        Ok(())
    }

    fn update_status_bar(&mut self) {
        let active_buffer = &self.buffers[self.active_buffer_idx];

        self.status_bar.update(
            self.mode,
            active_buffer.lines_in_document(),
            active_buffer.cursor_position(),
            &active_buffer.document_name(),
        );
    }

    fn process_command(&mut self, command: Command) -> Result<()> {
        let actrive_buffer = &mut self.buffers[self.active_buffer_idx];

        if let Command::EnterMode(mode) = command {
            match mode {
                Mode::Command => {
                    self.command_line.start_prompt();
                }
                Mode::Insert => {
                    self.command_line.clear();
                    self.command_line.set_message(&format!("-- {} --", mode));
                }
                Mode::Normal => self.command_line.clear(),
            };

            self.mode = mode;

            return Ok(());
        }

        match command {
            Command::Quit => self.should_quit = true,
            _ => actrive_buffer
                .proccess_command(command)
                .context("unable to process command on active buffer")?,
        };

        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<()> {
        if self.should_quit {
            self.terminal.clear()?;
            self.terminal.flush()?;
            return Ok(());
        }

        let active_buffer = &self.buffers[self.active_buffer_idx];
        let status_bar = &self.status_bar;
        let command_line = &self.command_line;
        let mode = &self.mode;

        self.terminal.draw(|view| {
            view.render(active_buffer);
            view.render(status_bar);
            view.render(command_line);

            if let Mode::Command = mode {
                view.set_cursor_position(command_line.cursor_position());
            } else {
                view.set_cursor_position(active_buffer.cursor_position());
            }

            Ok(())
        })
    }
}
