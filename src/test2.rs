use crate::io::event::Key;
use std::collections::HashMap;

pub trait Command {
    fn execute(&mut self);
}

pub trait ParameterisedCommand {
    fn execute(&mut self, param: &str);
}

pub trait InputCommand {
    fn execute(&mut self, ch: char);
}

enum ExecuteMode {
    Instant,
    OnEnter,
}

pub struct Buffer {
    prompt_symbol: char,
    execute_mode: ExecuteMode,
    input_command: Option<Box<dyn InputCommand>>,
    commands: HashMap<String, Box<dyn Command>>,
    parameterised_commands: HashMap<String, Box<dyn ParameterisedCommand>>,
    key_commands: HashMap<Key, Box<dyn Command>>,
    command_buffer: String,
    input_buffer: String,
    capture_input: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            prompt_symbol: ':',
            execute_mode: ExecuteMode::Instant,
            input_command: None,
            commands: HashMap::new(),
            parameterised_commands: HashMap::new(),
            key_commands: HashMap::new(),
            command_buffer: String::new(),
            input_buffer: String::new(),
            capture_input: false,
        }
    }

    pub fn new_input_captured(input_command: Box<dyn InputCommand>) -> Self {
        let mut buffer = Self::new();
        buffer.input_command = Some(input_command);
        buffer
    }

    pub fn add_binding(&mut self, binding: &str, command: Box<dyn Command>) {
        self.commands.insert(String::from(binding), command);
    }

    pub fn add_parameterised_binding(
        &mut self,
        binding: &str,
        command: Box<dyn ParameterisedCommand>,
    ) {
        self.parameterised_commands
            .insert(String::from(binding), command);
    }

    pub fn add_key_binding(&mut self, key: Key, command: Box<dyn Command>) {
        // TODO: prevent Char etc from being passed here
        self.key_commands.insert(key, command);
    }

    pub fn execute(&mut self, key: Key) {
        if let Some(command) = self.key_commands.get(&key) {
            command.execute();
            return;
        }

        match key {
            Key::Char(' ') => {
                if let ExecuteMode::OnEnter = self.execute_mode {
                    if self
                        .parameterised_commands
                        .contains_key(&self.command_buffer)
                    {
                        self.capture_input = true;
                    }
                }

                if let Some(input_command) = self.input_command {
                    input_command.execute(' ');
                }
            }
            Key::Char(ch) => {
                if let Some(input_command) = self.input_command {
                    input_command.execute(ch);
                }

                if let ExecuteMode::Instant = self.execute_mode {
                    if let Some(command) = self.commands.get(&self.command_buffer) {
                        command.execute();
                    }

                    return;
                }

                if ch == self.prompt_symbol {
                    self.execute_mode = ExecuteMode::OnEnter;
                }

                if self.capture_input {
                    self.input_buffer.push(ch);
                    return;
                }

                self.command_buffer.push(ch);
            }
            Key::Esc => {
                if let ExecuteMode::OnEnter = self.execute_mode {
                    self.execute_mode = ExecuteMode::Instant;

                    self.command_buffer.clear();

                    if self.capture_input {
                        self.input_buffer.clear();
                        self.capture_input = false;
                    }
                    // TODO: return error here to state promot was aborted
                }
            }
            Key::Enter => {
                if let ExecuteMode::OnEnter = self.execute_mode {
                    if self.capture_input {
                        self.parameterised_commands
                            .get(&self.command_buffer)
                            .unwrap()
                            .execute(&self.input_buffer);
                    } else {
                        if let Some(command) = self.commands.get(&self.command_buffer) {
                            command.execute();
                        }
                    }
                }
            }
            _ => {}
        };
    }
}
