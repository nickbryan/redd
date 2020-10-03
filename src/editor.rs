use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal, Result,
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;

        loop {
            if self.should_quit {
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
}

impl Default for Editor {
    fn default() -> Self {
        Self { should_quit: false }
    }
}
