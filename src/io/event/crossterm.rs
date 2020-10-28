use crate::io::event::{Event, Key, Loop as EventLoop};
use anyhow::{Context, Error, Result};
use crossterm::event::{self as ctevent, KeyCode, KeyEvent, KeyModifiers};
use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

pub struct Loop {
    rx: Option<Receiver<Event>>,
    tick_rate: Duration,
}

impl Loop {
    pub fn new(tick_rate: Duration) -> Self {
        Self {
            rx: None,
            tick_rate,
        }
    }
}

impl EventLoop for Loop {
    fn start(&mut self) {
        let (tx, rx) = mpsc::channel();
        let tick_rate = self.tick_rate;

        thread::spawn(move || loop {
            match ctevent::poll(tick_rate) {
                Ok(true) => match ctevent::read() {
                    Ok(ctevent::Event::Key(key)) => tx.send(Event::Input(Key::from(key))).unwrap(),
                    Err(e) => {
                        tx.send(Event::Error(Error::from(e).context("unable to read event")))
                            .unwrap();

                        break;
                    }
                    Ok(ctevent::Event::Mouse(_)) | Ok(ctevent::Event::Resize(_, _)) => {}
                },
                Ok(false) => tx.send(Event::Tick).unwrap(),
                Err(e) => {
                    tx.send(Event::Error(
                        Error::from(e).context("unable to poll for events"),
                    ))
                    .unwrap();

                    break;
                }
            };
        });

        self.rx = Some(rx);
    }

    fn next(&self) -> Result<Event> {
        match self.rx.as_ref() {
            Some(rx) => rx
                .recv()
                .context("trying to read from event loop that has not been started yet"),
            None => panic!("trying to read from event loop that has not been started yet"),
        }
    }
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        match event {
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Enter,
            } => Key::Enter,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Tab,
            } => Key::Tab,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Backspace,
            } => Key::Backspace,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Esc,
            } => Key::Esc,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Left,
            } => Key::Left,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Right,
            } => Key::Right,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Down,
            } => Key::Down,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Up,
            } => Key::Up,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Insert,
            } => Key::Insert,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Delete,
            } => Key::Delete,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Home,
            } => Key::Home,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::End,
            } => Key::End,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::PageUp,
            } => Key::PageUp,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::PageDown,
            } => Key::PageDown,
            KeyEvent {
                modifiers: KeyModifiers::NONE,
                code: KeyCode::Char(ch),
            } => Key::Char(ch),
            KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Char(ch),
            } => Key::Ctrl(ch),
            _ => Key::Unknown,
        }
    }
}
