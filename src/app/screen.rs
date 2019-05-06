use std::io::{stdin, stdout, Stdout, Write};
use termion::screen::AlternateScreen;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::app::AppError;

pub trait Screen {}

pub struct TermionScreen {
    screen: AlternateScreen<RawTerminal<Stdout>>
}

impl TermionScreen {
    pub fn new() -> Result<Self, AppError> {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().map_err(|err| AppError::Screen(err))?;

        let mut screen = AlternateScreen::from(stdout);
        write!(screen, "{}{}", termion::cursor::Goto(1, 1), termion::cursor::Hide).map_err(|err| AppError::Screen(err));
        screen.flush();
        Ok(TermionScreen {
            screen
        })
    }
}

impl Screen for TermionScreen {}

impl Drop for TermionScreen {
    fn drop(&mut self) {
        write!(self.screen, "{}", termion::cursor::Show).unwrap();
    }
}
