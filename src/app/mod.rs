extern crate termion;

pub mod screen;

use std::io;
use crate::app::screen::Screen;
use std::io::stdin;
use self::termion::input::TermRead;
use self::termion::event::Key;

#[derive(Debug)]
pub enum AppError {
    Screen(io::Error)
}

pub struct App<S: Screen> {
    screen: S
}

impl<S: Screen> App<S> {
    pub fn new(screen: S) -> Self {
        App {
            screen
        }
    }

    pub fn run(&self) -> Result<(), AppError> {
        let stdin = stdin();
        for key in stdin.keys() {
            let key = key.map_err(|err| AppError::Screen(err))?;

            match key {
                Key::Esc => return Ok(()),
                _ => continue,
            };
        }

        Ok(())
    }
}
