use std::{io, process, time::Duration};
use vie_core::Editor;
use vie_tui::CrosstermBackend;

fn main() {
    use anyhow::Context;

    if let Err(e) = CrosstermBackend::new(io::stdout(), Duration::from_millis(250))
        .context("unable to create CrosstermBackend")
        .and_then(|mut backend| {
            Editor::new(&mut backend)
                .context("unable to initialise Editor")
                .and_then(|mut editor| {
                    editor
                        .run()
                        .context("an error occured while running the Editor")
                })
        })
    {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
