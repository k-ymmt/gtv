use crate::app::screen::{Screen, Style};
use crate::app::Result;

pub mod list;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn decompose(&self) -> (u16, u16, u16, u16) {
        (self.x, self.y, self.width, self.height)
    }
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
    fn draw(&mut self, screen: &mut Screen) -> Result<()>;
    fn set_frame(&mut self, frame: Rect);
    fn set_style(&mut self, style: Style);
    fn style(&self) -> &Style;
    fn frame(&self) -> Rect;
}