use crate::{editor::Mode, ops::Command};
use nom::{
    branch::alt,
    character::complete::{char, digit0, one_of},
    combinator::{all_consuming, map, recognize, value},
    sequence::pair,
    IResult,
};

fn command_mode(input: &str) -> IResult<&str, Command> {
    value(Command::EnterMode(crate::editor::Mode::Command), char(':'))(input)
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

pub fn parse(input: &str) -> Option<Command> {
    if let Ok((_, command)) =
        all_consuming(alt((command_mode, insert_mode, movement_action)))(input)
    {
        return Some(command);
    }

    None
}
