use crate::editor::Mode;

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
