use crate::app::screen::Screen;
use crate::app::Result;

pub mod list;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            x: 1,
            y: 1,
            width: 1,
            height: 1,
        }
    }
}

pub trait View {
    fn draw(&self, screen: &mut Screen) -> Result<()>;
    fn frame(&self) -> Rect;
}