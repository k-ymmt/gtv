use crate::app::view::{View, Rect};
use crate::app::screen::Screen;
use crate::app::{Result};

pub struct List {
    frame: Rect,
}

impl List {
    pub fn new() -> Self {
        List {
            frame: Rect::default()
        }
    }
}


impl View for List {
    fn draw(&self, screen: &mut Screen) -> Result<()> {
        screen.write(self.frame.x, self.frame.y, "foo".to_string())
    }

    fn frame(&self) -> Rect {
        self.frame
    }
}