mod app;
mod logger;

#[macro_use]
extern crate bitflags;

use crate::app::{App};
use crate::app::screen::{TermionScreen, Style, Fg, Bg, Color, Modifier};
use crate::app::view::list::{List, ListItem};
use crate::app::view::{View, Rect};

fn main() {
    let mut list = List::new();
    list.set_items(vec![
        ListItem::new("foo".to_string()),
        ListItem::new("hoge".to_string()),
        ListItem::new("bar".to_string()),
        ListItem::new("foo".to_string()),
        ListItem::new("foo".to_string()),
        ListItem::new("foo".to_string()),
    ]);
    list.set_style(Style {
        fg: Fg::new(Color::White),
        bg: Bg::new(Color::Black),
        modifier: Modifier::empty()
    });
    list.set_highlight(Style {
        fg: Fg::new(Color::Black),
        bg: Bg::new(Color::LightWhite),
        modifier: Modifier::empty()
    });
    let mut app = App::new(TermionScreen::new().unwrap(), list);
    app.run().unwrap();
}
