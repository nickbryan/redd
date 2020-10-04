use crate::terminal::Terminal;
use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    Result,
};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) -> Result<()> {
        loop {
            self.refresh_screen();

            if self.should_quit {
                println!("Thanks for using the Redd editor!\r");
                return Ok(());
            }

            if let Some(key_event) = self.terminal.process_events() {
                self.proccess_keypress(key_event);
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

    fn refresh_screen(&mut self) {
        // TODO: convert unwrap to errors
        self.terminal.clear();
        self.terminal.position_cursor(0, 0);

        if self.should_quit {
            self.terminal.flush();
            return;
        }

        self.draw_rows();
        self.terminal.position_cursor(1, 0);

        self.terminal.flush();
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default(),
        }
    }
}
