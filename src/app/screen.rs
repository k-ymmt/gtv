use std::io::{stdout, Stdout, Write};
use termion::screen::AlternateScreen;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;
use crate::app::{Result, ConvertAppErr};

pub trait Screen {
    fn size(&self) -> Result<(u16, u16)>;
    fn write(&mut self, x: u16, y: u16, text: String) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
}

pub struct TermionScreen {
    screen: AlternateScreen<RawTerminal<Stdout>>,
}

impl TermionScreen {
    pub fn new() -> Result<Self> {
        let stdout = stdout().into_raw_mode().to_app_err()?;

        let mut screen = AlternateScreen::from(stdout);
        write!(screen, "{}{}", termion::cursor::Goto(1, 1), termion::cursor::Hide).to_app_err()?;
        screen.flush().to_app_err()?;
        Ok(TermionScreen {
            screen,
        })
    }
}

impl Screen for TermionScreen {
    fn size(&self) -> Result<(u16, u16)> {
        Ok(terminal_size().to_app_err()?)
    }

    fn write(&mut self, x: u16, y: u16, text: String) -> Result<()> {
        write!(self.screen, "{}{}", termion::cursor::Goto(x, y), text).to_app_err()
    }

    fn flush(&mut self) -> Result<()> {
        self.screen.flush().to_app_err()
    }
}

impl Drop for TermionScreen {
    fn drop(&mut self) {
        write!(self.screen, "{}", termion::cursor::Show).unwrap();
    }
}
