use std::{io, process, time::Duration};
use vie_core::Editor;
use vie_tui::{CrosstermEventLoop, CrosstermGrid};

fn main() {
    use anyhow::Context;

    if let Err(e) = CrosstermEventLoop::new(Duration::from_millis(250))
        .context("unable to create CrosstermEventLoop")
        .and_then(|event_loop| {
            CrosstermGrid::new(io::stdout())
                .context("unable to create CrosstermGrid")
                .and_then(|mut grid| {
                    Editor::new(event_loop, &mut grid)
                        .context("unable to initialise Editor")
                        .and_then(|mut editor| {
                            editor
                                .run()
                                .context("an error occured while running the Editor")
                        })
                })
        })
    {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
