use super::termion::event::Key;
use crate::app::event::Event;
use crate::app::screen::{Screen, Style};
use crate::app::view::{Rect, View};
use crate::app::{AppError, Result};
use std::ops::Range;

#[derive(Clone)]
pub struct ListItem {
    text: String,
}

impl ListItem {
    pub fn new(text: String) -> Self {
        ListItem { text }
    }
}

pub struct List {
    frame: Rect,
    items: Vec<ListItem>,
    style: Style,
    highlight_style: Style,
    visible_range: Range<usize>,
    current_line: usize,
}

impl List {
    pub fn new() -> Self {
        List {
            frame: Rect::default(),
            items: Vec::new(),
            style: Style::default(),
            highlight_style: Style::default(),
            visible_range: 0..1,
            current_line: 0,
        }
    }

    pub fn set_items(&mut self, items: Vec<ListItem>) {
        self.items = items;
    }

    pub fn set_highlight(&mut self, style: Style) {
        self.highlight_style = style;
    }

    pub fn set_current_line(&mut self, line: usize) -> Result<()> {
        let limit = self.items.len() - 1;
        if line > limit {
            return Err(AppError::OutOfBounds(line));
        }

        if self.current_line == line {
            return Ok(());
        }

        self.current_line = line;

        Ok(())
    }

    fn update_visible_range(&mut self) {
        let line = self.current_line;
        let limit = self.items.len() - 1;
        let h = self.frame.height as usize - 1;
        if h > limit {
            self.visible_range = 0..limit + 1;
        } else {
            let r = &self.visible_range;
            if line <= r.start {
                self.visible_range = line..line + h
            } else if line >= r.end {
                self.visible_range = line - h..line
            }
        }
    }
}

impl View for List {
    fn draw(&mut self, screen: &mut Screen) -> Result<()> {
        let r = &self.visible_range;
        let mut i = self.frame.y as usize;
        let current_index = self.current_line - r.start;
        for item in self.items[r.start..r.end].to_vec() {
            let t = item.text.clone();

            let s = if current_index == i - 1 {
                self.highlight_style
            } else {
                self.style
            };
            let empty_string = " ".repeat(self.frame.width as usize);
            screen.write(self.frame.x, i as u16, empty_string, s)?;
            screen.write(self.frame.x, i as u16, t, s)?;
            i += 1;
        }

        Ok(())
    }

    fn set_frame(&mut self, frame: Rect) {
        self.frame = frame;
        self.update_visible_range();
    }

    fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn frame(&self) -> Rect {
        self.frame
    }

    fn receive_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key) => match key {
                Key::Char('j') => {
                    if self.current_line > self.items.len() - 1 {
                        return Ok(());
                    }
                    self.set_current_line(self.current_line + 1)?;
                }
                Key::Char('k') => {
                    if self.current_line == 0 {
                        return Ok(());
                    }
                    self.set_current_line(self.current_line - 1)?;
                }
                _ => {}
            },
            _ => {}
        };

        Ok(())
    }
}
