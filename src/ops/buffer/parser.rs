use crate::{editor::Mode, io::event::Key, ops::Command};

enum InputBuffer {
    Inactive,
    KeyPress(Key),
    CharacterSequence(String),
}

pub struct Parser {
    input_buffer: InputBuffer,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            input_buffer: InputBuffer::Inactive,
        }
    }
}

impl Parser {
    pub fn receive_input(&mut self, key: Key) {
        match key {
            Key::Char(ch) => {
                match self.input_buffer {
                    InputBuffer::Inactive => {
                        self.input_buffer = InputBuffer::CharacterSequence(String::from(ch))
                    }
                    InputBuffer::CharacterSequence(ref mut sequence) => sequence.push(ch),
                    InputBuffer::KeyPress(_) => {
                        /* TODO: throw a error here */
                        panic!("this should not happen");
                    }
                };
            }
            // TODO: handle error cases here
            _ => self.input_buffer = InputBuffer::KeyPress(key),
        }
    }

    pub fn matched_command(&mut self, mode: Mode) -> Option<Command> {
        match self.input_buffer {
            InputBuffer::Inactive => None,
            InputBuffer::KeyPress(ref key) => {
                if let Some(command) = match mode {
                    Mode::Normal => normal_mode_command_for_key_press(key),
                    Mode::Insert => insert_mode_command_for_key_press(key),
                    Mode::Command => None,
                } {
                    self.input_buffer = InputBuffer::Inactive;
                    Some(command)
                } else {
                    None
                }
            }
            InputBuffer::CharacterSequence(ref sequence) => {
                if let Some(command) = match mode {
                    Mode::Normal => normal_mode_command_for_input_sequence(sequence),
                    Mode::Insert => insert_mode_command_for_input_sequence(sequence),
                    Mode::Command => None,
                } {
                    self.input_buffer = InputBuffer::Inactive;
                    Some(command)
                } else {
                    None
                }
            }
        }
    }
}

fn normal_mode_command_for_key_press(key: &Key) -> Option<Command> {
    None
}

fn insert_mode_command_for_key_press(key: &Key) -> Option<Command> {
    None
}

fn normal_mode_command_for_input_sequence(sequence: &str) -> Option<Command> {
    if sequence == ":" {
        return Some(Command::EnterMode(Mode::Command));
    }
    None
}

fn insert_mode_command_for_input_sequence(sequence: &str) -> Option<Command> {
    None
}
