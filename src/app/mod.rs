extern crate termion;

pub mod screen;
pub mod view;

use std::io;
use crate::app::screen::{Screen, Style};
use std::io::stdin;
use self::termion::input::TermRead;
use self::termion::event::Key;
use crate::app::view::{View, Rect};

type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    IOError(io::Error),
    OutOfBounds(usize),
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
        App {
            screen,
            view: root,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let (w, h) = self.screen.size()?;
        self.view.set_frame(Rect {
            width: w,
            height: h,
            ..Rect::default()
        });
        self.view.draw(&mut self.screen)?;

        self.screen.flush()?;
        let stdin = stdin();
        for key in stdin.keys() {
            let key = key.map_err(|err| AppError::IOError(err))?;

            match key {
                Key::Esc => return Ok(()),
                _ => continue,
            };
        }

        Ok(())
    }
}
