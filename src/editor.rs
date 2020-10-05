use crate::{
    event::{Event, Events, Key},
    terminal::Terminal,
};
use anyhow::{Context, Result};
use std::time::Duration;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            should_quit: false,
            terminal: Terminal::new().context("unable to create Terminal")?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let events = Events::listen(Duration::from_millis(250));

        loop {
            self.refresh_screen().context("unable to refresh screen")?;

            if self.should_quit {
                break;
            }

            match events.next()? {
                Event::Input(key) => self.proccess_keypress(key),
                Event::Tick => { /* We can do stuff here while waiting for input */ }
            }
        }

        Ok(())
    }

    fn proccess_keypress(&mut self, key: Key) {
        match key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn refresh_screen(&mut self) -> Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.position_cursor(0, 0)?;

        if self.should_quit {
            self.terminal.clear()?;
            self.terminal.flush()?;
            return Ok(());
        }

        self.draw_rows()?;
        self.terminal.position_cursor(1, 0)?;

        self.terminal.show_cursor()?;
        self.terminal.flush()
    }

    fn draw_rows(&mut self) -> Result<()> {
        for _ in 0..self.terminal.size()?.height {
            self.terminal.clear_current_line()?;
            println!("~\r");
        }

        Ok(())
    }
}
