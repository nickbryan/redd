use crate::{editor::Mode, io::event::Key, ops::Command};

pub struct Parser {
    input_buffer: String,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            input_buffer: String::new(),
        }
    }
}

impl Parser {
    pub fn matched_command_for(&mut self, key: Key, mode: Mode) -> Option<Command> {
        match mode {
            Mode::Normal => {
                if let Key::Char(ch) = key {
                    self.input_buffer.push(ch);
                }

                if let Key::Esc = key {
                    self.input_buffer.clear();
                }

                normal_mode_command_for_key_press(key).map_or_else(
                    || {
                        let command = normal_mode_command_for_input_sequence(&self.input_buffer);
                        self.input_buffer.clear();
                        command
                    },
                    Some,
                )
            }
            Mode::Insert => insert_mode_command_for_key_press(key),
            Mode::Command => None,
        }
    }
}

fn normal_mode_command_for_key_press(key: Key) -> Option<Command> {
    match key {
        Key::Home => Some(Command::MoveCursorLineStart),
        Key::End => Some(Command::MoveCursorLineEnd),
        Key::PageUp => Some(Command::MoveCursorPageUp),
        Key::PageDown => Some(Command::MoveCursorPageDown),
        Key::Insert => Some(Command::EnterMode(Mode::Insert)),
        Key::Enter => Some(Command::MoveCursorDown(1)),
        _ => None,
    }
}

fn insert_mode_command_for_key_press(key: Key) -> Option<Command> {
    match key {
        Key::Up => Some(Command::MoveCursorUp(1)),
        Key::Down => Some(Command::MoveCursorDown(1)),
        Key::Left => Some(Command::MoveCursorLeft(1)),
        Key::Right => Some(Command::MoveCursorRight(1)),
        Key::Home => Some(Command::MoveCursorLineStart),
        Key::End => Some(Command::MoveCursorLineEnd),
        Key::PageUp => Some(Command::MoveCursorPageUp),
        Key::PageDown => Some(Command::MoveCursorPageDown),
        Key::Delete => Some(Command::DeleteCharForward),
        Key::Backspace => Some(Command::DeleteCharBackward),
        Key::Enter => Some(Command::InsertLineBreak),
        Key::Char(ch) => Some(Command::InsertChar(ch)),
        Key::Esc => Some(Command::EnterMode(Mode::Normal)),
        _ => None,
    }
}

fn normal_mode_command_for_input_sequence(sequence: &str) -> Option<Command> {
    super::normal::parse(sequence)
}
