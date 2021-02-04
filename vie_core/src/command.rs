use crate::{backend::Key, row::Row, ui::Position};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Mode {
    Execute,
    Insert,
    Normal,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    EnterMode(Mode),
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

pub trait Parser {
    fn display_name(&self) -> String;
    fn parse(&mut self, key: Key) -> Option<Command>;
}

pub enum Parsers {
    Execute(ExecuteParser),
    Insert(InsertParser),
    Normal(NormalParser),
}

impl Default for Parsers {
    fn default() -> Self {
        Self::Normal(NormalParser {
            input_buffer: String::new(),
        })
    }
}

impl Parser for Parsers {
    fn display_name(&self) -> String {
        match self {
            Self::Execute(parser) => parser.display_name(),
            Self::Insert(parser) => parser.display_name(),
            Self::Normal(parser) => parser.display_name(),
        }
    }

    fn parse(&mut self, key: Key) -> Option<Command> {
        let command = match self {
            Self::Execute(parser) => parser.parse(key),
            Self::Insert(parser) => parser.parse(key),
            Self::Normal(parser) => parser.parse(key),
        };

        if let Some(Command::EnterMode(ref mode)) = command {
            *self = match mode {
                Mode::Execute => Self::Execute(ExecuteParser {
                    row: Row::default(),
                    cursor_position: Position::default(),
                }),
                Mode::Insert => Self::Insert(InsertParser {}),
                Mode::Normal => Self::Normal(NormalParser {
                    input_buffer: String::new(),
                }),
            }
        }

        command
    }
}

pub struct ExecuteParser {
    row: Row,
    cursor_position: Position,
}

impl ExecuteParser {
    fn execute_command(&mut self, command: Command) -> Option<Command> {
        match command {
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

                if self.row.len() == 1 {
                    return Some(Command::EnterMode(Mode::Normal));
                }

                None
            }
            Command::DeleteCharBackward => {
                self.cursor_position.col = self.cursor_position.col.saturating_sub(1);
                self.row.delete(self.cursor_position.col);

                if self.row.len() == 1 {
                    return Some(Command::EnterMode(Mode::Normal));
                }

                None
            }
            _ => None,
        }
    }

    pub fn cursor_position(&self) -> Position {
        self.cursor_position
    }

    pub fn contents(&self) -> String {
        self.row.contents()
    }
}

impl Parser for ExecuteParser {
    fn display_name(&self) -> String {
        "Execute".into()
    }

    fn parse(&mut self, key: Key) -> Option<Command> {
        if let Key::Enter = key {
            return execute_mode::command_for_input(&self.row.contents());
        }

        match key {
            Key::Char(ch) => Some(Command::InsertChar(ch)),
            Key::Left => Some(Command::MoveCursorLeft(1)),
            Key::Right => Some(Command::MoveCursorRight(1)),
            Key::Backspace => Some(Command::DeleteCharBackward),
            Key::Delete => Some(Command::DeleteCharForward),
            Key::Home => Some(Command::MoveCursorLineStart),
            Key::End => Some(Command::MoveCursorLineEnd),
            Key::Esc => Some(Command::EnterMode(Mode::Normal)),
            _ => None,
        }
        .and_then(|command| self.execute_command(command))
    }
}

mod execute_mode {
    use super::Command;
    use nom::{
        branch::alt,
        character::complete::{anychar, char},
        combinator::{all_consuming, map, value},
        multi::many1,
        sequence::{pair, separated_pair},
        IResult,
    };

    fn quit(input: &str) -> IResult<&str, Command> {
        value(Command::Quit, all_consuming(char('q')))(input)
    }

    fn save(input: &str) -> IResult<&str, Command> {
        value(Command::Save, all_consuming(char('w')))(input)
    }

    fn save_as(input: &str) -> IResult<&str, Command> {
        map(
            separated_pair(char('w'), char(' '), many1(anychar)),
            |(_, name)| Command::SaveAs(name.into_iter().collect::<String>()),
        )(input)
    }

    pub fn command_for_input(input: &str) -> Option<Command> {
        if let Ok((_, (_, command))) =
            all_consuming(pair(char(':'), alt((quit, save, save_as))))(input)
        {
            return Some(command);
        }

        None
    }

    #[cfg(test)]
    mod tests {
        use super::{command_for_input, quit, save, save_as};
        use crate::command::Command;

        #[test]
        fn test_command_for_input() {
            let tests = vec![
                (":q", Command::Quit),
                (":w", Command::Save),
                (":w some_file.txt", Command::SaveAs("some_file.txt".into())),
            ];

            for (input, command) in tests.into_iter() {
                assert_eq!(command_for_input(input), Some(command));
            }
        }

        #[test]
        fn test_quit() {
            assert!(quit("w").is_err());
            assert_eq!(quit("q"), Ok(("", Command::Quit)));
        }

        #[test]
        fn test_save() {
            assert!(save("q").is_err());
            assert_eq!(save("w"), Ok(("", Command::Save)));
        }

        #[test]
        fn test_save_as() {
            assert!(save_as("w").is_err());
            assert_eq!(
                save_as("w test.txt"),
                Ok(("", Command::SaveAs("test.txt".into())))
            );
        }
    }
}

pub struct InsertParser {}

impl Parser for InsertParser {
    fn display_name(&self) -> String {
        "Insert".into()
    }

    fn parse(&mut self, key: Key) -> Option<Command> {
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
}

pub struct NormalParser {
    input_buffer: String,
}

impl Parser for NormalParser {
    fn display_name(&self) -> String {
        "Normal".into()
    }

    fn parse(&mut self, key: Key) -> Option<Command> {
        if let Key::Char(ch) = key {
            self.input_buffer.push(ch);
        }

        if let Key::Esc = key {
            self.input_buffer.clear();
        }

        match key {
            Key::Home => Some(Command::MoveCursorLineStart),
            Key::End => Some(Command::MoveCursorLineEnd),
            Key::PageUp => Some(Command::MoveCursorPageUp),
            Key::PageDown => Some(Command::MoveCursorPageDown),
            Key::Insert => Some(Command::EnterMode(Mode::Insert)),
            Key::Enter => Some(Command::MoveCursorDown(1)),
            _ => None,
        }
        .map_or_else(
            || {
                let command = normal_mode::command_for_input(&self.input_buffer);
                self.input_buffer.clear();
                command
            },
            Some,
        )
    }
}

mod normal_mode {
    use super::{Command, Mode};
    use nom::{
        branch::alt,
        character::complete::{char, digit0, one_of},
        combinator::{all_consuming, map, recognize, value},
        sequence::pair,
        IResult,
    };
    pub fn command_for_input(input: &str) -> Option<Command> {
        if let Ok((_, command)) =
            all_consuming(alt((command_mode, insert_mode, movement_action)))(input)
        {
            return Some(command);
        }

        None
    }

    fn command_mode(input: &str) -> IResult<&str, Command> {
        value(Command::EnterMode(Mode::Execute), char(':'))(input)
    }

    fn insert_mode(input: &str) -> IResult<&str, Command> {
        value(Command::EnterMode(Mode::Insert), char('i'))(input)
    }

    fn non_zero_digit(input: &str) -> IResult<&str, char> {
        one_of("123456789")(input)
    }

    fn multiplier(input: &str) -> IResult<&str, &str> {
        recognize(pair(non_zero_digit, digit0))(input)
    }

    fn movement_key(input: &str) -> IResult<&str, char> {
        alt((char('h'), char('j'), char('k'), char('l')))(input)
    }

    fn single_move_action(input: &str) -> IResult<&str, Command> {
        map(movement_key, |c| match c {
            'h' => Command::MoveCursorLeft(1),
            'j' => Command::MoveCursorDown(1),
            'k' => Command::MoveCursorUp(1),
            'l' => Command::MoveCursorRight(1),
            _ => unreachable!(),
        })(input)
    }

    fn multi_move_action(input: &str) -> IResult<&str, Command> {
        map(pair(multiplier, movement_key), |(m, c)| match c {
            'h' => Command::MoveCursorLeft(m.parse::<usize>().unwrap()),
            'j' => Command::MoveCursorDown(m.parse::<usize>().unwrap()),
            'k' => Command::MoveCursorUp(m.parse::<usize>().unwrap()),
            'l' => Command::MoveCursorRight(m.parse::<usize>().unwrap()),
            _ => unreachable!(),
        })(input)
    }

    fn movement_action(input: &str) -> IResult<&str, Command> {
        alt((single_move_action, multi_move_action))(input)
    }
}
