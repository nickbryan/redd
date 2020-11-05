use crate::{io::event::Key, ops::Command};

pub fn command_for_key(key: Key) -> Option<Command> {
    match key {
        Key::Char(ch) => Some(Command::InsertChar(ch)),
        Key::Left => Some(Command::MoveCursorLeft(1)),
        Key::Right => Some(Command::MoveCursorRight(1)),
        Key::Backspace => Some(Command::DeleteCharBackward),
        Key::Delete => Some(Command::DeleteCharForward),
        Key::Home => Some(Command::MoveCursorLineStart),
        Key::End => Some(Command::MoveCursorLineEnd),
        Key::Esc => Some(Command::EnterMode(crate::editor::Mode::Normal)),
        _ => None,
    }
}

pub fn command_for_input(input: &str) -> Option<Command> {
    if input == ":q" {
        return Some(Command::Quit);
    }

    None
}
