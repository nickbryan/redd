use crate::event::Key;
use anyhow::{Context, Error, Result};
use crossterm::event;
use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

pub enum Event {
    Input(Key),
    Tick,
    Error(Error),
}

pub struct Events {
    rx: Receiver<Event>,
}

impl Events {
    pub fn listen(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            match event::poll(tick_rate) {
                Ok(true) => match event::read() {
                    Ok(event::Event::Key(key)) => tx.send(Event::Input(Key::from(key))).unwrap(),
                    Err(e) => {
                        tx.send(Event::Error(Error::from(e).context("unable to read event")))
                            .unwrap();

                        break;
                    }
                    _ => tx.send(Event::Tick).unwrap(),
                },
                Err(e) => {
                    tx.send(Event::Error(
                        Error::from(e).context("unable to poll for events"),
                    ))
                    .unwrap();

                    break;
                }
                _ => {}
            };
        });

        Events { rx }
    }

    pub fn next(&self) -> Result<Event> {
        self.rx
            .recv()
            .context("unable to recive from event channel")
    }
}
