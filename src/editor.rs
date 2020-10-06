use crate::{
    event::{Event, Events, Key},
    terminal::Terminal,
};
use anyhow::{Context, Result};
use std::{cmp, time::Duration};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
                Event::Error(e) => return Err(e),
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

    fn draw_welcome_message(&mut self) -> Result<()> {
        let mut welcome_message = format!("Redd editor -- version {}", VERSION);
        let width = self.terminal.size()?.width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
        Ok(())
    }

    fn draw_rows(&mut self) -> Result<()> {
        let height = self.terminal.size()?.height;

        for row in 0..height - 1 {
            self.terminal.clear_current_line()?;

            if row == height / 3 {
                self.draw_welcome_message()?;
            } else {
                println!("~\r");
            }
        }

        Ok(())
    }
}
