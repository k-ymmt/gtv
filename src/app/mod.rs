extern crate crossbeam;
extern crate signal_hook;
extern crate termion;

pub mod event;
pub mod screen;
pub mod view;

use self::termion::event::Key;
use self::termion::input::TermRead;
use crate::app::event::{watch_input_event, watch_resize_event, Event};
use crate::app::screen::{Screen, Style};
use crate::app::view::{Rect, View};
use crate::logger::Logger;
use crossbeam::Receiver;
use std::io::stdin;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::{io, thread};

type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    IOError(io::Error),
    OutOfBounds(usize),
    Unknown(String),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IOError(err)
    }
}

pub struct App<V: View, S: Screen> {
    screen: S,
    view: V,
}

impl<V: View, S: Screen> App<V, S> {
    pub fn new(screen: S, root: V) -> Self {
        App { screen, view: root }
    }

    pub fn run(&mut self) -> Result<()> {
        let (w, h) = self.screen.size()?;

        let (resize_sender, resize_receiver) = crossbeam::bounded(0);
        let running = Arc::new(AtomicBool::new(true));
        watch_resize_event(resize_sender, running.clone());

        let (input_sender, input_receiver) = crossbeam::bounded(0);
        watch_input_event(input_sender);

        self.view.set_frame(Rect {
            width: w,
            height: h,
            ..Rect::default()
        });
        self.view.draw(&mut self.screen)?;

        self.screen.flush()?;

        loop {
            if let Some(event) = self.poll_event(&input_receiver, &resize_receiver) {
                match event {
                    Event::Key(key) => match key {
                        Key::Esc => break,
                        _ => {
                            if let Err(err) = self.view.receive_event(Event::Key(key)) {
                                Logger::log(format!("{:?}", err));
                            }
                        }
                    },
                    Event::Resize => {
                        let (w, h) = self.screen.size()?;
                        self.view.set_frame(Rect {
                            width: w,
                            height: h,
                            ..self.view.frame()
                        });
                    }
                    _ => continue,
                }

                self.view.draw(&mut self.screen)?;
                self.screen.flush()?;
            }
        }

        Ok(())
    }

    #[inline]
    fn poll_event(
        &self,
        input_receiver: &Receiver<Option<Event>>,
        resize_receiver: &Receiver<()>,
    ) -> Option<Event> {
        select! {
            recv(input_receiver) -> event => event.ok()?,
            recv(resize_receiver) -> _ => Some(Event::Resize),
            default => None
        }
    }
}
