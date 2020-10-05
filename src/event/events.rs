use crate::event::Key;
use anyhow::{Context, Result};
use crossterm::event;
use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

pub enum Event {
    Input(Key),
    Tick,
}
pub struct Events {
    reciever: Receiver<Event>,
}

impl Events {
    pub fn listen(tick_rate: Duration) -> Self {
        let (sender, reciever) = mpsc::channel();

        thread::spawn(move || loop {
            // TODO: try and handle the errors here
            if event::poll(tick_rate).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    sender.send(Event::Input(Key::from(key))).unwrap();
                }
            }

            sender.send(Event::Tick).unwrap();
        });

        Events { reciever }
    }

    pub fn next(&self) -> Result<Event> {
        self.reciever
            .recv()
            .context("unable to recive from event channel")
    }
}
