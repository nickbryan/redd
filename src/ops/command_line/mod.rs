use crate::{io::event::Key, ops::Command};
use nom::{
    branch::alt,
    character::complete::{anychar, char},
    combinator::{all_consuming, map, value},
    multi::many1,
    sequence::{pair, separated_pair},
    IResult,
};

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

pub fn quit(input: &str) -> IResult<&str, Command> {
    value(Command::Quit, all_consuming(char('q')))(input)
}

pub fn save(input: &str) -> IResult<&str, Command> {
    value(Command::Save, all_consuming(char('w')))(input)
}

pub fn save_as(input: &str) -> IResult<&str, Command> {
    map(
        separated_pair(char('w'), char(' '), many1(anychar)),
        |(_, name)| Command::SaveAs(name.into_iter().collect::<String>()),
    )(input)
}

pub fn command_for_input(input: &str) -> Option<Command> {
    if let Ok((_, (_, command))) = all_consuming(pair(char(':'), alt((quit, save, save_as))))(input)
    {
        return Some(command);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

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
