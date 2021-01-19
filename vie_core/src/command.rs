use crate::mode::Descriptor;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    TransitionTo(Descriptor),

    InsertChar(char),
    InsertLineBreak,
    DeleteCharForward,
    DeleteCharBackward,

    MoveCursorUp(usize),
    MoveCursorDown(usize),
    MoveCursorLeft(usize),
    MoveCursorRight(usize),
    MoveCursorLineStart,
    MoveCursorLineEnd,
    MoveCursorPageUp,
    MoveCursorPageDown,

    Save,
    SaveAs(String),

    Quit,
}

pub mod normal_mode {
    use super::Command;
    use crate::{backend::Key, mode::Descriptor};

    pub fn command_for_key_press(key: Key) -> Option<Command> {
        match key {
            Key::Home => Some(Command::MoveCursorLineStart),
            Key::End => Some(Command::MoveCursorLineEnd),
            Key::PageUp => Some(Command::MoveCursorPageUp),
            Key::PageDown => Some(Command::MoveCursorPageDown),
            Key::Insert => Some(Command::TransitionTo(Descriptor::Insert)),
            Key::Enter => Some(Command::MoveCursorDown(1)),
            _ => None,
        }
    }

    pub fn command_for_input_sequence(input: &str) -> Option<Command> {
        None
    }
}
