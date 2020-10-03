use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, Clear, ClearType},
    Result,
};
use std::io::{self, Write};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;

        loop {
            self.refresh_screen();

            if self.should_quit {
                println!("Thanks for using the Redd editor!");
                return Ok(());
            }

            match event::read()? {
                Event::Key(event) => {
                    self.proccess_keypress(event);
                }
                _ => {}
            }
        }
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

    fn refresh_screen(&self) {
        let mut stdout = io::stdout();
        // TODO: convert unwrap to errors
        crossterm::queue!(stdout, Clear(ClearType::All), cursor::MoveTo(1, 1)).unwrap();
        stdout.flush().unwrap();
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self { should_quit: false }
    }
}
