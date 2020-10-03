use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal, Result,
};

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    loop {
        match event::read()? {
            Event::Key(KeyEvent { code, modifiers }) => {
                if modifiers == KeyModifiers::CONTROL && code == KeyCode::Char('q').into() {
                    break;
                }

                if let KeyCode::Char(c) = code {
                    if c.is_control() {
                        println!("{:?} \r", c as u8);
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
