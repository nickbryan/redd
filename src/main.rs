mod document;
mod editor;
mod event;
mod row;
mod terminal;

use anyhow::Context;
use editor::Editor;
use std::process;

fn main() {
    if let Err(e) = match Editor::new().context("unable to initialise Editor") {
        Ok(mut editor) => editor
            .run()
            .context("an error occured while running the editor"),
        Err(e) => Err(e),
    } {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
