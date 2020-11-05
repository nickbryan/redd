use crate::{
    document::Row,
    io::event::Key,
    ops::{command_line, Command},
    ui::{
        layout::{Component, Position, Rect},
        style::Style,
        FrameBuffer,
    },
};

const PROMPT_SYMBOL: &str = ":";

pub struct CommandLine {
    row: Row,
    viewport: Rect,
    cursor_position: Position,
}

impl Default for CommandLine {
    fn default() -> Self {
        Self {
            row: Row::default(),
            viewport: Rect::default(),
            cursor_position: Position::default(),
        }
    }
}

impl CommandLine {
    pub fn new(viewport: Rect) -> Self {
        Self {
            viewport,
            ..Self::default()
        }
    }

    pub fn cursor_position(&self) -> Position {
        Position::new(
            self.viewport
                .position
                .x
                .saturating_add(self.cursor_position.x),
            self.viewport
                .position
                .y
                .saturating_add(self.cursor_position.y),
        )
    }

    pub fn matched_command_for(&mut self, key: Key) -> Option<Command> {
        if let Key::Enter = key {
            return command_line::command_for_input(&self.row.contents());
        }

        if let Some(command) = command_line::command_for_key(key) {
            return self.execute_command(command);
        }

        None
    }

    fn execute_command(&mut self, command: Command) -> Option<Command> {
        match command {
            Command::EnterMode(_) => return Some(command),
            Command::InsertChar(ch) => {
                self.row.insert(self.cursor_position.x, ch);
                self.cursor_position.x = self.cursor_position.x.saturating_add(1);
            }
            Command::MoveCursorLeft(n) => {
                self.cursor_position.x = self.cursor_position.x.saturating_sub(n)
            }
            Command::MoveCursorRight(n) => {
                self.cursor_position.x = self.cursor_position.x.saturating_add(n)
            }
            Command::MoveCursorLineStart => self.cursor_position.x = 1,
            Command::MoveCursorLineEnd => self.cursor_position.x = self.row.len(),
            Command::DeleteCharForward => self.row.delete(self.cursor_position.x),
            Command::DeleteCharBackward => {
                self.cursor_position.x = self.cursor_position.x.saturating_sub(1);
                self.row.delete(self.cursor_position.x);
            }
            _ => {}
        };

        None
    }

    pub fn start_prompt(&mut self) {
        self.row = Row::from(PROMPT_SYMBOL);
        self.cursor_position.x = self.row.len();
    }

    pub fn clear(&mut self) {
        self.row = Row::default();
    }

    pub fn set_message(&mut self, message: &str) {
        self.row = Row::from(message);
    }
}

impl Component for CommandLine {
    fn render(&self, buffer: &mut FrameBuffer) {
        buffer.write_line(self.viewport.top(), &self.row.contents(), &Style::default());
    }
}
