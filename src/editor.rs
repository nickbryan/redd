use crate::terminal::Terminal;
use anyhow::{Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
        loop {
            self.refresh_screen().context("unable to refresh screen")?;

            if self.should_quit {
                break;
            }

            if let Some(key_event) = self
                .terminal
                .process_events()
                .context("unable to process events")?
            {
                self.proccess_keypress(key_event);
            }
        }

        Ok(())
    }

    fn proccess_keypress(&mut self, event: KeyEvent) {
        match event {
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if c == 'q' {
                    self.should_quit = true;
                }
            }
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
