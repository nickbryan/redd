use std::{io, process, time::Duration};
use vie_core::Editor;
use vie_tui::{CrosstermCanvas, CrosstermEventLoop};

fn main() {
    use anyhow::Context;

    if let Err(e) = CrosstermEventLoop::new(Duration::from_millis(250))
        .context("unable to create CrosstermEventLoop")
        .and_then(|event_loop| {
            CrosstermCanvas::new(io::stdout())
                .context("unable to create CrosstermCanvas")
                .and_then(|mut canvas| {
                    Editor::new(event_loop, &mut canvas)
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
