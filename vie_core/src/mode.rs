use crate::{
    backend::Key,
    command::{normal_mode, Command},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Descriptor {
    Insert,
    Normal,
    Command,
}

pub trait Mode {
    fn name() -> Descriptor;

    fn recieve_input(&mut self, key: Key);

    fn next_transition(&self) -> Option<Descriptor>;
}

#[derive(Default)]
pub struct NormalMode {
    input_buffer: String,
    next_transition: Option<Descriptor>,
}

impl Mode for NormalMode {
    fn name() -> Descriptor {
        Descriptor::Normal
    }

    fn recieve_input(&mut self, key: Key) {
        // We keep a buffer of recieved characters so we can pass them to the parser as a sequence
        // later on. Pressing Esc allows the user to clear the current input buffer's sequence.
        match key {
            Key::Char(ch) => self.input_buffer.push(ch),
            Key::Esc => self.input_buffer.clear(),
            _ => (),
        };

        // First try to see if we have a command directly mapped to the key press, if not, we will
        // try the input sequence stored in the input_buffer to see if we have a matching command
        // for that. If we match on an input sequence then we want to clear the input_buffer ready
        // for the next command.
        let command = normal_mode::command_for_key_press(key).map_or_else(
            || {
                let command = normal_mode::command_for_input_sequence(&self.input_buffer);

                if command.is_some() {
                    self.input_buffer.clear();
                }

                command
            },
            Some,
        );

        if let Some(Command::TransitionTo(descriptor)) = command {
            self.next_transition = Some(descriptor);
        }
    }

    fn next_transition(&self) -> Option<Descriptor> {
        self.next_transition
    }
}

#[derive(Default)]
pub struct InsertMode {}

impl Mode for InsertMode {
    fn name() -> Descriptor {
        Descriptor::Insert
    }

    fn recieve_input(&mut self, key: Key) {}

    fn next_transition(&self) -> Option<Descriptor> {
        None
    }
}
