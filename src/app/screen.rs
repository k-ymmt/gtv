use super::termion::color;
use super::termion::color::Color as TColor;
use super::termion::style;
use crate::app::Result;
use std::fmt;
use std::io::{stdout, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use termion::terminal_size;

bitflags! {
    pub struct Modifier: u16 {
        const bold = 0b0000_0001;
        const italic = 0b0000_0010;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,
}

trait ToTermion<T> {
    fn to_termion(&self) -> T;
}

#[derive(Copy, Clone, Debug)]
pub struct Fg(Color);

impl Fg {
    pub fn new(color: Color) -> Self {
        Fg(color)
    }
}

impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Color::Reset => color::Reset.write_fg(f),
            Color::Black => color::Black.write_fg(f),
            Color::Red => color::Red.write_fg(f),
            Color::Green => color::Green.write_fg(f),
            Color::Yellow => color::Yellow.write_fg(f),
            Color::Blue => color::Blue.write_fg(f),
            Color::Magenta => color::Magenta.write_fg(f),
            Color::Cyan => color::Cyan.write_fg(f),
            Color::White => color::White.write_fg(f),
            Color::LightBlack => color::LightBlack.write_fg(f),
            Color::LightRed => color::LightRed.write_fg(f),
            Color::LightGreen => color::LightGreen.write_fg(f),
            Color::LightYellow => color::LightYellow.write_fg(f),
            Color::LightBlue => color::LightBlue.write_fg(f),
            Color::LightMagenta => color::LightMagenta.write_fg(f),
            Color::LightCyan => color::LightCyan.write_fg(f),
            Color::LightWhite => color::LightWhite.write_fg(f),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Bg(Color);

impl Bg {
    pub fn new(color: Color) -> Self {
        Bg(color)
    }
}

impl fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Color::Reset => color::Reset.write_bg(f),
            Color::Black => color::Black.write_bg(f),
            Color::Red => color::Red.write_bg(f),
            Color::Green => color::Green.write_bg(f),
            Color::Yellow => color::Yellow.write_bg(f),
            Color::Blue => color::Blue.write_bg(f),
            Color::Magenta => color::Magenta.write_bg(f),
            Color::Cyan => color::Cyan.write_bg(f),
            Color::White => color::White.write_bg(f),
            Color::LightBlack => color::LightBlack.write_bg(f),
            Color::LightRed => color::LightRed.write_bg(f),
            Color::LightGreen => color::LightGreen.write_bg(f),
            Color::LightYellow => color::LightYellow.write_bg(f),
            Color::LightBlue => color::LightBlue.write_bg(f),
            Color::LightMagenta => color::LightMagenta.write_bg(f),
            Color::LightCyan => color::LightCyan.write_bg(f),
            Color::LightWhite => color::LightWhite.write_bg(f),
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Style {
    pub fg: Fg,
    pub bg: Bg,
    pub modifier: Modifier,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg: Fg(Color::White),
            bg: Bg(Color::Black),
            modifier: Modifier::empty(),
        }
    }
}

pub trait Screen {
    fn size(&self) -> Result<(u16, u16)>;
    fn write(&mut self, x: u16, y: u16, text: String, style: Style) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
}

pub struct TermionScreen {
    screen: AlternateScreen<RawTerminal<Stdout>>,
}

impl TermionScreen {
    pub fn new() -> Result<Self> {
        let stdout = stdout().into_raw_mode()?;

        let mut screen = AlternateScreen::from(stdout);
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )?;
        screen.flush()?;
        Ok(TermionScreen { screen })
    }
}

impl Screen for TermionScreen {
    fn size(&self) -> Result<(u16, u16)> {
        let s = terminal_size()?;
        Ok(s)
    }

    fn write(&mut self, x: u16, y: u16, text: String, style: Style) -> Result<()> {
        if style.modifier.contains(Modifier::bold) {
            write!(self.screen, "{}", style::Bold)?;
        } else {
            write!(self.screen, "{}", style::NoBold)?;
        }
        if style.modifier.contains(Modifier::italic) {
            write!(self.screen, "{}", style::Italic)?;
        } else {
            write!(self.screen, "{}", style::NoItalic)?;
        }
        write!(
            self.screen,
            "{}{}{}{}",
            termion::cursor::Goto(x, y),
            style.fg,
            style.bg,
            text
        )?;
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        self.screen.flush()?;
        Ok(())
    }
}

impl Drop for TermionScreen {
    fn drop(&mut self) {
        write!(self.screen, "{}", termion::cursor::Show).unwrap();
    }
}
