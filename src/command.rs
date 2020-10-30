use crate::editor::Mode;
use crate::io::event::Key;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command {
    EnterMode(Mode),

    CommandLineBeginInput(char),
    CommandLineUpdateInput(String),
    CommandLineInputAborted,
    CommandLineExecute(Box<Self>),
    CommandLineInputNotRecognised(String),

    InsertChar(char),
    InsertLineBreak,
    DeleteCharForward,
    DeleteCharBackward,

    MoveCursorUp,
    MoveCursorDown,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorLineStart,
    MoveCursorLineEnd,
    MoveCursorPageUp,
    MoveCursorPageDown,

    Save,
    SaveAs(String),

    Quit,
}

impl Command {
    pub fn with_input(&mut self, input: &str) {
        match self {
            Self::SaveAs(_) => {
                *self = Self::SaveAs(input.into());
            }
            _ => { /* TODO: handle error case */ }
        };
    }
}

#[derive(Debug, Clone)]
struct Binding {
    command: Command,
    sequence: String,
    input_capture_allowed: bool,
}

impl Binding {
    pub fn new(sequence: &str, command: Command) -> Self {
        Self {
            sequence: sequence.into(),
            command,
            input_capture_allowed: false,
        }
    }

    pub fn new_with_input(sequence: &str, command: Command) -> Self {
        let mut binding = Self::new(sequence, command);
        binding.input_capture_allowed = true;
        binding
    }
}

pub struct Parser {
    buffer: String,
    normal_bindings: Vec<Binding>,
    command_bindings: Vec<Binding>,
    active_binding: Option<Binding>,
}

// TODO: return a Result that has an error to determine if input was required or not or the command
// TODO: replcae with TryFrom?
// TODO: try clean this up as much as possible
impl Parser {
    pub fn parse(&mut self, key: Key, mode: Mode) -> Option<Command> {
        match mode {
            Mode::Command => match key {
                Key::Char(' ') => {
                    for binding in &self.command_bindings {
                        if binding.sequence == self.buffer && binding.input_capture_allowed {
                            self.active_binding = Some(binding.clone());
                        }
                    }

                    self.buffer.push(' ');
                    Some(Command::CommandLineUpdateInput(self.buffer.clone()))
                }
                Key::Char(ch) => {
                    self.buffer.push(ch);
                    Some(Command::CommandLineUpdateInput(self.buffer.clone()))
                }
                Key::Esc => {
                    self.buffer.clear();
                    self.active_binding = None;
                    Some(Command::CommandLineInputAborted)
                }
                Key::Enter => {
                    if let Some(binding) = self.active_binding.take() {
                        let mut command = Box::new(binding.command);

                        let input: String = self.buffer[..]
                            .graphemes(true)
                            .skip(binding.sequence[..].graphemes(true).count() + 1) // 1 for the space
                            .collect();

                        command.with_input(&input);
                        self.buffer.clear();
                        return Some(Command::CommandLineExecute(command));
                    }

                    for binding in &self.command_bindings {
                        if binding.sequence == self.buffer {
                            self.buffer.clear();
                            return Some(Command::CommandLineExecute(Box::new(
                                binding.command.clone(),
                            )));
                        }
                    }

                    let result = Some(Command::CommandLineInputNotRecognised(self.buffer.clone()));
                    self.buffer.clear();
                    result
                }
                _ => None,
            },
            Mode::Insert => match key {
                Key::Char(ch) => Some(Command::InsertChar(ch)),
                Key::Enter => Some(Command::InsertLineBreak),
                Key::Backspace => Some(Command::DeleteCharBackward),
                Key::Delete => Some(Command::DeleteCharForward),
                Key::Esc => {
                    self.buffer.clear();
                    Some(Command::EnterMode(Mode::Normal))
                }
                _ => None,
            },
            Mode::Normal => match key {
                Key::Char(':') => {
                    self.buffer.push(':');
                    Some(Command::CommandLineBeginInput(':'))
                }
                Key::Char(ch) => {
                    self.buffer.push(ch);

                    for binding in &self.normal_bindings {
                        if binding.sequence == self.buffer {
                            self.buffer.clear();
                            return Some(binding.command.clone());
                        }
                    }

                    None
                }
                _ => None,
            },
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        let normal_bindings = vec![
            Binding::new("k".into(), Command::MoveCursorUp),
            Binding::new("j".into(), Command::MoveCursorDown),
            Binding::new("h".into(), Command::MoveCursorLeft),
            Binding::new("l".into(), Command::MoveCursorRight),
            Binding::new("^".into(), Command::MoveCursorLineStart),
            Binding::new("$".into(), Command::MoveCursorLineEnd),
            Binding::new("x".into(), Command::DeleteCharForward),
            Binding::new("i".into(), Command::EnterMode(Mode::Insert)),
        ];

        let command_bindings = vec![
            Binding::new(":q".into(), Command::Quit),
            Binding::new(":w".into(), Command::Save),
            Binding::new_with_input(":w".into(), Command::SaveAs(String::new())),
        ];

        Self {
            buffer: String::new(),
            normal_bindings,
            command_bindings,
            active_binding: None,
        }
    }
}
