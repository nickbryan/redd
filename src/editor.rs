use crate::{
    command::{Command, Parser},
    document::{Buffer, Document},
    io::{
        event::{CrosstermEventLoop, Event, Loop as EventLoop},
        CrosstermBackend,
    },
    terminal::Terminal,
    ui::layout::{Component, Rect},
    ui::style::{Color, Style},
    ui::FrameBuffer,
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
    Command(char),
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Normal
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "Normal"),
            Self::Insert => write!(f, "Insert"),
            Self::Command(_) => write!(f, "Command"),
        }
    }
}

#[derive(Default)]
struct StatusBar {
    viewport: Rect,
    mode: Mode,
    line_count: usize,
    current_line: usize,
    file_name: String,
}

impl StatusBar {
    pub fn new(viewport: Rect) -> Self {
        Self {
            viewport,
            ..Self::default()
        }
    }

    pub fn update(&mut self, mode: Mode, line_count: usize, current_line: usize, file_name: &str) {
        self.mode = mode;
        self.line_count = line_count;
        self.current_line = current_line;
        self.file_name = file_name.into();
    }
}

impl Component for StatusBar {
    fn render(&self, buffer: &mut FrameBuffer) {
        let mut status = format!("Mode: [{}]    File: {}", self.mode, self.file_name);
        let line_indicator = format!("{}/{}", self.current_line, self.line_count);

        let len = status.len() + line_indicator.len();

        if self.viewport.width > len {
            status.push_str(&" ".repeat(self.viewport.width - len));
        }

        status = format!("{}{}", status, line_indicator);
        status.truncate(self.viewport.width);

        buffer.write_line(
            self.viewport.top(),
            &status,
            &Style::new(Color::Rgb(63, 63, 63), Color::Rgb(239, 239, 239)),
        );
    }
}

#[derive(Debug, Default)]
pub struct MessageBar {
    viewport: Rect,
    message: String,
}

impl MessageBar {
    pub fn new(viewport: Rect) -> Self {
        Self {
            viewport,
            ..Self::default()
        }
    }

    pub fn clear(&mut self) {
        self.message.clear();
    }

    pub fn push_char(&mut self, ch: char) {
        self.message.push(ch);
    }
}

impl Component for MessageBar {
    fn render(&self, buffer: &mut FrameBuffer) {
        buffer.write_line(self.viewport.top(), &self.message, &Style::default());
    }
}

pub struct Editor {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    event_loop: Box<dyn EventLoop>,
    should_quit: bool,
    buffers: Vec<Buffer>,
    active_buffer_idx: usize,
    mode: Mode,
    command_parser: Parser,
    status_bar: StatusBar,
    message_bar: MessageBar,
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
            terminal.viewport().height,
            0,
            terminal.viewport().bottom() - 2,
        ));

        let message_bar = MessageBar::new(Rect::positioned(
            terminal.viewport().width,
            terminal.viewport().height,
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
            command_parser: Parser::default(),
            status_bar,
            message_bar,
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
                Event::Input(key) => {
                    if let Some(command) = self.command_parser.parse(key, self.mode) {
                        self.proccess_command(command)
                            .context("unable to process command")?;
                    };

                    self.update_status_bar();
                }
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
            active_buffer.cursor_position().y,
            &active_buffer.document_name(),
        );
    }

    fn proccess_command(&mut self, command: Command) -> Result<()> {
        let actrive_buffer = &mut self.buffers[self.active_buffer_idx];

        match self.mode {
            Mode::Command(_) => match command {
                Command::InsertChar(ch) => self.message_bar.push_char(ch),
                Command::Quit => self.should_quit = true,
                Command::EnterMode(mode) => {
                    if let Mode::Command(starting_ch) = mode {
                        self.message_bar.push_char(starting_ch);
                    }

                    self.mode = mode;
                }
                _ => {}
            },
            _ => match command {
                Command::EnterMode(mode) => {
                    if let Mode::Command(starting_ch) = mode {
                        self.message_bar.push_char(starting_ch);
                    }

                    self.mode = mode;
                }
                _ => actrive_buffer
                    .proccess_command(command)
                    .context("unable to process command on active buffer")?,
            },
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
        let message_bar = &self.message_bar;

        self.terminal.draw(|view| {
            view.render(active_buffer);
            view.render(status_bar);
            view.render(message_bar);

            view.set_cursor_position(active_buffer.cursor_position());

            Ok(())
        })
    }
}
