use crate::ops::Command;
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{all_consuming, value},
    IResult,
};

fn command_mode(input: &str) -> IResult<&str, Command> {
    value(Command::EnterMode(crate::editor::Mode::Command), char(':'))(input)
}

pub fn parse(input: &str) -> Option<Command> {
    if let Ok((_, command)) = all_consuming(command_mode)(input) {
        Some(command)
    } else {
        None
    }
}
