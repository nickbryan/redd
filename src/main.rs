mod editor;

use crossterm::Result;
use editor::Editor;

fn main() -> Result<()> {
    Editor::default().run()
}
