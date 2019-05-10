mod app;
mod logger;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate crossbeam;

extern crate chrono;

use crate::app::screen::{Bg, Color, Fg, Modifier, Style, TermionScreen};
use crate::app::view::list::{List, ListItem};
use crate::app::view::{Rect, View};
use crate::app::{App, AppError};
use crate::logger::Logger;

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
        modifier: Modifier::empty(),
    });
    list.set_highlight(Style {
        fg: Fg::new(Color::Black),
        bg: Bg::new(Color::LightWhite),
        modifier: Modifier::empty(),
    });

    let mut app = App::new(TermionScreen::new().unwrap(), list);
    Logger::log("foo".to_string());
    let r = app.run();
    if let Err(err) = r {
        Logger::log(format!("{:?}", err))
    }
}
