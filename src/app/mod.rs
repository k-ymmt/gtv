extern crate termion;

pub mod screen;
pub mod view;

use std::io;
use crate::app::screen::Screen;
use std::io::stdin;
use self::termion::input::TermRead;
use self::termion::event::Key;
use crate::app::view::View;

type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Screen(io::Error),
}

pub trait ConvertAppErr<T> {
    fn to_app_err(self) -> Result<T>;
}

impl<T> ConvertAppErr<T> for io::Result<T> {
    #[inline]
    fn to_app_err(self) -> Result<T> {
        match self {
            Ok(t) => Ok(t),
            Err(err) => Err(AppError::Screen(err)),
        }
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
        self.view.draw(&mut self.screen)?;
        self.screen.flush()?;
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
