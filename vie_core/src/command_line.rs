use crate::{
    command::{Command, Mode, NormalMode},
    row::Row,
    ui::{frame, Component, Position, Rect, Style},
};

#[derive(Debug)]
pub struct CommandLine {
    area: Rect,
    row: Row,
    cursor_position: Position,
}

impl CommandLine {
    pub fn new(area: Rect) -> Self {
        let mut command_line = CommandLine {
            area,
            row: Row::default(),
            cursor_position: Position::default(),
        };
        command_line.reset();
        command_line
    }

    pub fn execute_command(&mut self, command: Command) -> Option<Command> {
        match command {
            Command::EndCommandLineInput => {
                let command = Some(Command::ParseCommandLineInput(self.row.contents()));
                self.reset();
                command
            }
            Command::AbortCommandLineInput => {
                self.reset();
                Some(Command::EnterMode(Mode::Normal(NormalMode::default())))
            }
            Command::InsertChar(ch) => {
                self.row.insert(self.cursor_position.col, ch);
                self.cursor_position.col = self.cursor_position.col.saturating_add(1);
                None
            }
            Command::MoveCursorLeft(n) => {
                if self.cursor_position.col > 1 {
                    self.cursor_position.col = self.cursor_position.col.saturating_sub(n)
                }
                None
            }
            Command::MoveCursorRight(n) => {
                if self.cursor_position.col != self.row.len() {
                    self.cursor_position.col = self.cursor_position.col.saturating_add(n)
                }
                None
            }
            Command::MoveCursorLineStart => {
                self.cursor_position.col = 1;
                None
            }
            Command::MoveCursorLineEnd => {
                self.cursor_position.col = self.row.len();
                None
            }
            Command::DeleteCharForward => {
                self.row.delete(self.cursor_position.col);

                if self.row.len() <= 1 {
                    self.reset();
                    return Some(Command::EnterMode(Mode::Normal(NormalMode::default())));
                }

                None
            }
            Command::DeleteCharBackward => {
                self.cursor_position.col = self.cursor_position.col.saturating_sub(1);
                self.row.delete(self.cursor_position.col);

                if self.row.len() <= 1 {
                    self.reset();
                    return Some(Command::EnterMode(Mode::Normal(NormalMode::default())));
                }

                None
            }
            _ => None,
        }
    }

    pub fn cursor_position(&self) -> Position {
        Position::new(
            self.area
                .position
                .col
                .saturating_add(self.cursor_position.col),
            self.area
                .position
                .row
                .saturating_add(self.cursor_position.row),
        )
    }

    pub fn command_string(&self) -> String {
        self.row.contents()
    }

    fn reset(&mut self) {
        self.row = Row::from(":");
        self.cursor_position.col = self.row.len();
    }
}

impl Component for &CommandLine {
    fn render(&self, buffer: &mut frame::Buffer) {
        buffer.write_line(self.area.top(), &self.row.contents(), &Style::default());
    }
}
