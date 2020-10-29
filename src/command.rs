use crate::editor::Mode;
use crate::io::event::Key;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Command {
    EnterMode(Mode),

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

    Quit,
}

type Map = HashMap<Command, Vec<Key>>;

pub struct Parser {
    buffer: Vec<Key>,
    normal_mode_map: Map,
    insert_mode_map: Map,
    command_mode_map: Map,
}

impl Parser {
    pub fn parse(&mut self, key: Key, mode: Mode) -> Option<Command> {
        self.buffer.push(key);

        if let Key::Esc = key {
            self.buffer.clear();
            return Some(Command::EnterMode(Mode::Normal));
        }

        if let Key::Char(':') = key {
            return Some(Command::EnterMode(Mode::Command(':')));
        }

        match mode {
            Mode::Normal => {
                for (command, sequence) in &self.normal_mode_map {
                    if self.buffer[..] == sequence[..] {
                        self.buffer.clear();
                        return Some(*command);
                    }
                }
            }
            Mode::Insert => {
                for (command, sequence) in &self.insert_mode_map {
                    if self.buffer[..] == sequence[..] {
                        self.buffer.clear();
                        return Some(*command);
                    }
                }

                if let Key::Char(ch) = key {
                    self.buffer.clear();
                    return Some(Command::InsertChar(ch));
                }
            }
            Mode::Command(_) => match key {
                Key::Char(ch) => return Some(Command::InsertChar(ch)),
                Key::Enter => {
                    for (command, sequence) in &self.command_mode_map {
                        if self.buffer[..] == sequence[..] {
                            self.buffer.clear();
                            return Some(*command);
                        }
                    }
                }
                _ => {}
            },
        }

        None
    }
}

impl Default for Parser {
    fn default() -> Self {
        let mut normal_mode_map = Map::new();

        normal_mode_map.insert(Command::MoveCursorUp, vec![Key::Char('k')]);
        normal_mode_map.insert(Command::MoveCursorDown, vec![Key::Char('j')]);
        normal_mode_map.insert(Command::MoveCursorLeft, vec![Key::Char('h')]);
        normal_mode_map.insert(Command::MoveCursorRight, vec![Key::Char('l')]);
        normal_mode_map.insert(Command::MoveCursorLineStart, vec![Key::Char('^')]);
        normal_mode_map.insert(Command::MoveCursorLineEnd, vec![Key::Char('$')]);
        normal_mode_map.insert(Command::DeleteCharForward, vec![Key::Char('x')]);
        normal_mode_map.insert(Command::EnterMode(Mode::Insert), vec![Key::Char('i')]);

        let mut insert_mode_map = Map::new();

        insert_mode_map.insert(Command::InsertLineBreak, vec![Key::Enter]);
        insert_mode_map.insert(Command::DeleteCharForward, vec![Key::Delete]);
        insert_mode_map.insert(Command::DeleteCharBackward, vec![Key::Backspace]);
        insert_mode_map.insert(Command::Save, vec![Key::Char(':'), Key::Char('w')]);

        let mut command_mode_map = Map::new();

        command_mode_map.insert(
            Command::Quit,
            vec![Key::Char(':'), Key::Char('q'), Key::Enter],
        );

        Self {
            buffer: Vec::new(),
            normal_mode_map,
            insert_mode_map,
            command_mode_map,
        }
    }
}
